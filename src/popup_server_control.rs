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
                        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    }
                    PopupCommand::Hide => {
                        crate::utils::debug_log("POPUP_SERVER", "Processing hide command");
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
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
            
            let (command, response) = match command_str {
                "show" => {
                    (Some(PopupCommand::Show), "OK: Window shown")
                }
                "hide" => {
                    (Some(PopupCommand::Hide), "OK: Window hidden")
                }
                "ping" => {
                    (Some(PopupCommand::Ping), "OK: Popup server running")
                }
                "status" => {
                    (None, "OK: Popup server active")
                }
                "delete" => {
                    // Exit the process
                    crate::utils::debug_log("POPUP_SERVER", "Delete command received, exiting");
                    let _ = stream.write_all(b"OK: Exiting");
                    std::process::exit(0);
                }
                _ => {
                    (None, "ERROR: Unknown command")
                }
            };
            
            // Set pending command
            if let Some(cmd) = command {
                if let Ok(mut pending) = pending_command.lock() {
                    *pending = Some(cmd);
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