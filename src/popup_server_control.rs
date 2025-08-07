//! Control socket for popup server
//! 
//! This module implements a control socket that allows external commands
//! to control the popup window visibility.

use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Write};
use eframe::egui;

/// Socket path for popup control
const POPUP_SOCKET: &str = "/tmp/hookanchor_popup.sock";

/// Check if the current process owns the popup control socket
/// Returns true if this instance is the primary popup server
pub fn is_primary_popup_instance() -> bool {
    // Check if the socket exists and if we can connect to it
    // If we can't connect, we're likely the owner (or there's no owner)
    match std::os::unix::net::UnixStream::connect(POPUP_SOCKET) {
        Ok(_stream) => {
            // Successfully connected - there's another popup that owns the socket
            // We are NOT the primary instance
            false
        }
        Err(_) => {
            // Can't connect - either we own it or no one does
            // Check if the socket file exists
            if std::path::Path::new(POPUP_SOCKET).exists() {
                // Socket exists but can't connect - we likely own it
                true
            } else {
                // No socket exists - we could be primary but haven't started listener yet
                // This is a startup race condition - assume we're primary
                true
            }
        }
    }
}

/// Commands that can be sent to the popup
#[derive(Debug, Clone)]
pub enum PopupCommand {
    Show,
    Hide,
    Ping,
}

/// Shared state for popup control
pub struct PopupControl {
    /// Pending command to process
    pub pending_command: Arc<Mutex<Option<PopupCommand>>>,
}

impl PopupControl {
    /// Create a new popup control instance
    pub fn new() -> Self {
        Self {
            pending_command: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Start the control socket listener
    pub fn start_listener(&self) {
        // Clean up any existing socket
        let _ = std::fs::remove_file(POPUP_SOCKET);
        
        let pending_command = Arc::clone(&self.pending_command);
        
        thread::spawn(move || {
            crate::utils::debug_log("POPUP_SERVER", &format!("Starting control socket at: {}", POPUP_SOCKET));
            
            match UnixListener::bind(POPUP_SOCKET) {
                Ok(listener) => {
                    crate::utils::debug_log("POPUP_SERVER", "Control socket listening");
                    
                    for stream in listener.incoming() {
                        match stream {
                            Ok(mut stream) => {
                                handle_client(&mut stream, &pending_command);
                            }
                            Err(e) => {
                                crate::utils::log_error(&format!("Failed to accept connection: {}", e));
                            }
                        }
                    }
                }
                Err(e) => {
                    crate::utils::log_error(&format!("Failed to bind control socket: {}", e));
                }
            }
        });
    }
    
    /// Process any pending commands
    /// Returns the command that was processed, if any
    pub fn process_commands(&self, ctx: &egui::Context) -> Option<PopupCommand> {
        if let Ok(mut pending) = self.pending_command.lock() {
            if let Some(command) = pending.take() {
                match &command {
                    PopupCommand::Show => {
                        crate::utils::debug_log("POPUP_SERVER", "Processing show command");
                        // Just trigger the show - verification will happen in the socket handler
                        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                        ctx.request_repaint();
                    }
                    PopupCommand::Hide => {
                        crate::utils::debug_log("POPUP_SERVER", "Processing hide command");
                        
                        // Use AppleScript to hide the window
                        #[cfg(target_os = "macos")]
                        {
                            let pid = std::process::id();
                            std::thread::spawn(move || {
                                let script = format!(
                                    "tell application \"System Events\"\n\
                                     if exists (first process whose unix id is {}) then\n\
                                       tell (first process whose unix id is {})\n\
                                         set visible to false\n\
                                       end tell\n\
                                     end if\n\
                                     end tell",
                                    pid, pid
                                );
                                
                                let _ = std::process::Command::new("osascript")
                                    .arg("-e")
                                    .arg(&script)
                                    .output();
                                
                                crate::utils::log("Window hidden via AppleScript");
                            });
                        }
                    }
                    PopupCommand::Ping => {
                        // Just a connectivity check
                        crate::utils::debug_log("POPUP_SERVER", "Ping received");
                    }
                }
                Some(command)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Handle a client connection
fn handle_client(stream: &mut UnixStream, pending_command: &Arc<Mutex<Option<PopupCommand>>>) {
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer) {
        Ok(size) => {
            let message = String::from_utf8_lossy(&buffer[..size]);
            let command_str = message.trim();
            
            crate::utils::debug_log("POPUP_SERVER", &format!("Received command: {}", command_str));
            
            let (command, mut response) = match command_str {
                "show" => {
                    (Some(PopupCommand::Show), String::from("Processing show command..."))
                }
                "hide" => {
                    (Some(PopupCommand::Hide), String::from("OK: Window hidden"))
                }
                "ping" => {
                    (Some(PopupCommand::Ping), String::from("OK: Popup server running"))
                }
                "status" => {
                    (None, String::from("OK: Popup server active"))
                }
                "delete" => {
                    // Exit the process
                    crate::utils::debug_log("POPUP_SERVER", "Delete command received, exiting");
                    let _ = stream.write_all(b"OK: Exiting");
                    std::process::exit(0);
                }
                _ => {
                    (None, String::from("ERROR: Unknown command"))
                }
            };
            
            // Set pending command
            if let Some(cmd) = command.clone() {
                if let Ok(mut pending) = pending_command.lock() {
                    *pending = Some(cmd);
                }
            }
            
            // For show command, actually do the show operation with retries
            if matches!(command, Some(PopupCommand::Show)) {
                let pid = std::process::id();
                let max_retries = 3; // TODO: Read from config popup_server_retries
                
                for attempt in 1..=max_retries {
                    crate::utils::log(&format!("Show attempt {}/{}", attempt, max_retries));
                    
                    // On first attempt, skip the show script - the Focus command might be enough
                    // On retries, use AppleScript to force visibility
                    if attempt > 1 {
                        // AppleScript to show window
                        let show_script = format!(
                            "tell application \"System Events\"\n\
                             if exists (first process whose unix id is {}) then\n\
                               tell (first process whose unix id is {})\n\
                                 set visible to true\n\
                                 set frontmost to true\n\
                                 if (count of windows) > 0 then\n\
                                   tell window 1\n\
                                     perform action \"AXRaise\"\n\
                                   end tell\n\
                                 end if\n\
                               end tell\n\
                             end if\n\
                             end tell",
                            pid, pid
                        );
                        
                        // Execute show command
                        let _ = std::process::Command::new("osascript")
                            .arg("-e")
                            .arg(&show_script)
                            .output();
                    }
                    
                    // On first attempt, no wait needed since we do show+verify in one script
                    // On retries, wait for window to appear
                    if attempt > 1 {
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }
                    
                    // On first attempt, combine show and verify in one script for speed
                    let verify_script = if attempt == 1 {
                        // Fast path: try to make visible and verify in one script
                        format!(
                            "tell application \"System Events\"\n\
                             if exists (first process whose unix id is {}) then\n\
                               tell (first process whose unix id is {})\n\
                                 set visible to true\n\
                                 set frontmost to true\n\
                                 if (count of windows) > 0 then\n\
                                   tell window 1\n\
                                     perform action \"AXRaise\"\n\
                                     set p to position\n\
                                     set s to size\n\
                                   end tell\n\
                                   if visible and frontmost then\n\
                                     return \"SUCCESS: Window visible and frontmost at \" & (item 1 of p) & \",\" & (item 2 of p) & \" size \" & (item 1 of s) & \"x\" & (item 2 of s)\n\
                                   else\n\
                                     return \"FAILED: visible=\" & visible & \" frontmost=\" & frontmost\n\
                                   end if\n\
                                 else\n\
                                   return \"FAILED: No windows\"\n\
                                 end if\n\
                               end tell\n\
                             else\n\
                               return \"FAILED: Process not found\"\n\
                             end if\n\
                             end tell",
                            pid, pid
                        )
                    } else {
                        // Slower path: just verify after separate show command
                        format!(
                            "tell application \"System Events\"\n\
                             if exists (first process whose unix id is {}) then\n\
                               tell (first process whose unix id is {})\n\
                                 if (count of windows) > 0 then\n\
                                   set isVisible to visible\n\
                                   set isFrontmost to frontmost\n\
                                   tell window 1\n\
                                     set p to position\n\
                                     set s to size\n\
                                   end tell\n\
                                   if isVisible and isFrontmost then\n\
                                     return \"SUCCESS: Window visible and frontmost at \" & (item 1 of p) & \",\" & (item 2 of p) & \" size \" & (item 1 of s) & \"x\" & (item 2 of s)\n\
                                   else\n\
                                     return \"FAILED: visible=\" & isVisible & \" frontmost=\" & isFrontmost\n\
                                   end if\n\
                                 else\n\
                                   return \"FAILED: No windows\"\n\
                                 end if\n\
                               end tell\n\
                             else\n\
                               return \"FAILED: Process not found\"\n\
                             end if\n\
                             end tell",
                            pid, pid
                        )
                    };
                    
                    if let Ok(output) = std::process::Command::new("osascript")
                        .arg("-e")
                        .arg(&verify_script)
                        .output()
                    {
                        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        crate::utils::log(&format!("Attempt {} result: {}", attempt, result));
                        
                        if result.starts_with("SUCCESS") {
                            response = result;
                            break; // Success!
                        } else if attempt == max_retries {
                            // Last attempt failed
                            response = format!("FAILED after {} attempts: {}", max_retries, result);
                            crate::utils::log_error("All show attempts failed - may need to rebuild server");
                        } else {
                            // Wait before retry
                            std::thread::sleep(std::time::Duration::from_millis(500));
                        }
                    }
                }
            }
            
            // Send response
            let _ = stream.write_all(response.as_bytes());
        }
        Err(e) => {
            crate::utils::log_error(&format!("Failed to read from client: {}", e));
        }
    }
}