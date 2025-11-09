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
    // In the supervisor architecture, the popup_server that's spawned by the supervisor
    // is always the primary instance. The supervisor ensures only one popup_server
    // is running at a time.
    // 
    // The old logic of trying to connect to our own socket was flawed - we would
    // successfully connect to ourselves and incorrectly think we were a duplicate!
    //
    // Since the supervisor manages the lifecycle, we should always consider ourselves
    // primary if we're running.
    true
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
    /// Context handle for triggering repaints
    pub ctx_handle: Arc<Mutex<Option<egui::Context>>>,
}

impl PopupControl {
    /// Create a new popup control instance
    pub fn new() -> Self {
        Self {
            pending_command: Arc::new(Mutex::new(None)),
            ctx_handle: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Start the control socket listener
    pub fn start_listener(&self) {
        // Clean up any existing socket
        let _ = std::fs::remove_file(POPUP_SOCKET);
        
        let pending_command = Arc::clone(&self.pending_command);
        let ctx_handle = Arc::clone(&self.ctx_handle);
        
        thread::spawn(move || {
            crate::utils::detailed_log("POPUP_SERVER", &format!("Starting control socket at: {}", POPUP_SOCKET));

            // Clean up any stale socket file from previous crashed instances
            if std::path::Path::new(POPUP_SOCKET).exists() {
                if let Err(e) = std::fs::remove_file(POPUP_SOCKET) {
                    crate::utils::log_error(&format!("Failed to remove stale popup socket: {}", e));
                } else {
                    crate::utils::detailed_log("POPUP_SERVER", "Cleaned up stale socket file");
                }
            }

            match UnixListener::bind(POPUP_SOCKET) {
                Ok(listener) => {
                    crate::utils::detailed_log("POPUP_SERVER", "Control socket listening");
                    
                    for stream in listener.incoming() {
                        match stream {
                            Ok(mut stream) => {
                                handle_client(&mut stream, &pending_command, &ctx_handle);
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
        // Store the context for use by the socket handler
        if let Ok(mut ctx_lock) = self.ctx_handle.lock() {
            if ctx_lock.is_none() {
                *ctx_lock = Some(ctx.clone());
            }
        }
        if let Ok(mut pending) = self.pending_command.lock() {
            if let Some(command) = pending.take() {
                match &command {
                    PopupCommand::Show => {
                        crate::utils::log("⏱️ POPUP_SERVER: Processing 'show' in update loop");
                        // Show the window and focus it
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                        ctx.request_repaint();
                        crate::utils::log("⏱️ POPUP_SERVER: Viewport commands sent");
                    }
                    PopupCommand::Hide => {
                        crate::utils::detailed_log("POPUP_SERVER", "Processing hide command");
                        
                        // Hide the window using viewport command
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                        ctx.request_repaint();
                    }
                    PopupCommand::Ping => {
                        // Just a connectivity check
                        crate::utils::detailed_log("POPUP_SERVER", "Ping received");
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
fn handle_client(stream: &mut UnixStream, pending_command: &Arc<Mutex<Option<PopupCommand>>>, ctx_handle: &Arc<Mutex<Option<egui::Context>>>) {
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer) {
        Ok(size) => {
            let message = String::from_utf8_lossy(&buffer[..size]);
            let command_str = message.trim();
            
            crate::utils::detailed_log("POPUP_SERVER", &format!("Received command: {}", command_str));
            
            let (command, mut response) = match command_str {
                "show" => {
                    crate::utils::log("⏱️ POPUP_SERVER: Received 'show' command");
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
                    crate::utils::detailed_log("POPUP_SERVER", "Delete command received, exiting");
                    let _ = stream.write_all(b"OK: Exiting");
                    std::process::exit(0);
                }
                _ => {
                    (None, String::from("ERROR: Unknown command"))
                }
            };
            
            // Set pending command
            if let Some(cmd) = command.clone() {
                // Special handling for show command - need to wake up the UI
                if matches!(cmd, PopupCommand::Show) {
                    // Request a repaint to wake up the hidden window
                    if let Ok(ctx_lock) = ctx_handle.lock() {
                        if let Some(ref ctx) = *ctx_lock {
                            ctx.request_repaint();
                            crate::utils::detailed_log("POPUP_SERVER", "Requested repaint for show command");
                        }
                    }
                }
                
                if let Ok(mut pending) = pending_command.lock() {
                    *pending = Some(cmd);
                }
            }
            
            // For show command, we no longer need verification since viewport commands handle it
            if matches!(command, Some(PopupCommand::Show)) {
                // The viewport commands in process_commands() will handle the actual show
                response = String::from("OK: Show command processed");
            }
            
            // Placeholder to keep line numbers - removed verification code
            
            // Send response
            let _ = stream.write_all(response.as_bytes());
        }
        Err(e) => {
            crate::utils::log_error(&format!("Failed to read from client: {}", e));
        }
    }
}