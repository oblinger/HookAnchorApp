//! Top-level application coordinator for the Anchor Selector
//!
//! This module handles global application state and dispatches to either
//! GUI mode (popup) or CLI mode (command-line processing).

use std::env;
use anchor_selector::ApplicationState;

/// Main application entry point
/// 
/// Determines whether to run in GUI mode (no arguments) or CLI mode (with arguments)
fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    
    // If arguments are provided, run in command-line mode (no GUI)
    if args.len() > 1 {
        // CLI mode can create its own ApplicationState
        anchor_selector::cmd::run_command_line_mode(args);
        Ok(())
    } else {
        // No arguments, run GUI mode
        let app_state = ApplicationState::new();
        anchor_selector::ui::run_gui_with_prompt("", app_state)
    }
}