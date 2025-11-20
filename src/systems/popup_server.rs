//! Control socket for popup server
//!
//! This module implements a control socket that allows external commands
//! to control the popup window visibility.

use crate::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Write};
use eframe::egui;
use std::os::unix::fs::PermissionsExt;

/// Get the secure popup socket path in user's config directory
fn get_popup_socket_path() -> std::path::PathBuf {
    crate::core::get_config_dir().join("popup.sock")
}

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
        let pending_command = Arc::clone(&self.pending_command);
        let ctx_handle = Arc::clone(&self.ctx_handle);

        thread::spawn(move || {
            let socket_path = get_popup_socket_path();
            detailed_log("POPUP_SERVER", &format!("Starting control socket at: {}", socket_path.display()));

            // Clean up any stale socket file from previous crashed instances
            if socket_path.exists() {
                if let Err(e) = std::fs::remove_file(&socket_path) {
                    log_error(&format!("Failed to remove stale popup socket: {}", e));
                } else {
                    detailed_log("POPUP_SERVER", "Cleaned up stale socket file");
                }
            }

            match UnixListener::bind(&socket_path) {
                Ok(listener) => {
                    // Set restrictive permissions (0600 - owner read/write only)
                    if let Ok(metadata) = std::fs::metadata(&socket_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o600);
                        if let Err(e) = std::fs::set_permissions(&socket_path, perms) {
                            log_error(&format!("Failed to set socket permissions: {}", e));
                        } else {
                            detailed_log("POPUP_SERVER", "Socket permissions set to 0600 (secure)");
                        }
                    }

                    detailed_log("POPUP_SERVER", "Control socket listening");

                    for stream in listener.incoming() {
                        match stream {
                            Ok(mut stream) => {
                                handle_client(&mut stream, &pending_command, &ctx_handle);
                            }
                            Err(e) => {
                                log_error(&format!("Failed to accept connection: {}", e));
                            }
                        }
                    }
                }
                Err(e) => {
                    log_error(&format!("Failed to bind control socket: {}", e));
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
                        log("⏱️ POPUP_SERVER: Processing 'show' in update loop");
                        // Show the window and focus it
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                        ctx.request_repaint();
                        log("⏱️ POPUP_SERVER: Viewport commands sent");
                    }
                    PopupCommand::Hide => {
                        detailed_log("POPUP_SERVER", "Processing hide command");
                        
                        // Hide the window using viewport command
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                        ctx.request_repaint();
                    }
                    PopupCommand::Ping => {
                        // Just a connectivity check
                        detailed_log("POPUP_SERVER", "Ping received");
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
            
            detailed_log("POPUP_SERVER", &format!("Received command: {}", command_str));
            
            let (command, mut response) = match command_str {
                "show" => {
                    detailed_log("POPUP_SERVER", "Received 'show' command");
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
                    detailed_log("POPUP_SERVER", "Delete command received, exiting");
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
                            detailed_log("POPUP_SERVER", "Requested repaint for show command");
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
            log_error(&format!("Failed to read from client: {}", e));
        }
    }
}