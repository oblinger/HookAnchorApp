//! Top-level application coordinator for HookAnchor
//!
//! This module handles global application state and dispatches to either
//! GUI mode (popup) or CLI mode (command-line processing).

use std::env;
use hookanchor::ApplicationState;

/// Main application entry point
/// 
/// Determines whether to run in GUI mode (no arguments) or CLI mode (with arguments)
fn main() -> Result<(), eframe::Error> {
    // Log startup with build timestamp to verify we're running the new binary
    hookanchor::utils::debug_log("STARTUP", "HookAnchor starting - Build: 21:33 PM July 7, 2025");
    
    // Initialize the global error queue for user error display
    hookanchor::error_display::init_error_queue();
    
    // Start command server if needed (smart management with PID tracking)
    if let Err(e) = hookanchor::server_management::start_server_if_needed() {
        hookanchor::utils::debug_log("STARTUP", &format!("Failed to start command server: {}", e));
        // Continue - commands will show error dialogs when server is needed
    }
    
    let args: Vec<String> = env::args().collect();
    
    // If arguments are provided, run in command-line mode (no GUI)
    if args.len() > 1 {
        // CLI mode can create its own ApplicationState
        hookanchor::cmd::run_command_line_mode(args);
        
        // Note: We don't shutdown the server here since CLI commands
        // may spawn background processes that need the server
        Ok(())
    } else {
        // No arguments, run GUI mode
        let app_state = ApplicationState::new();
        let result = hookanchor::ui::run_gui_with_prompt("", app_state);
        
        // Note: Server remains running as persistent daemon
        
        result
    }
}