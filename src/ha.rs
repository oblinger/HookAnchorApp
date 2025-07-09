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
    
    let args: Vec<String> = env::args().collect();
    
    // If arguments are provided, run in command-line mode (no GUI)
    if args.len() > 1 {
        // CLI mode can create its own ApplicationState
        hookanchor::cmd::run_command_line_mode(args);
        Ok(())
    } else {
        // No arguments, run GUI mode
        let app_state = ApplicationState::new();
        hookanchor::ui::run_gui_with_prompt("", app_state)
    }
}