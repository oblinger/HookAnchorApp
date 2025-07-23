//! Top-level application coordinator for HookAnchor
//!
//! This module handles global application state and dispatches to either
//! GUI mode (popup) or CLI mode (command-line processing).

use std::env;
use hookanchor::ApplicationState;
use chrono::{Local, TimeZone};

/// Main application entry point
/// 
/// Determines whether to run in GUI mode (no arguments) or CLI mode (with arguments)
fn main() -> Result<(), eframe::Error> {
    // Visual separator for new app launch in logs
    hookanchor::utils::debug_log("STARTUP", "════════════════════════════════════════════════════════════════");
    
    // Load state to get build time
    let state = hookanchor::core::state::load_state();
    let build_time_str = if let Some(build_time) = state.build_time {
        let dt = Local.timestamp_opt(build_time, 0).single()
            .unwrap_or_else(|| Local::now());
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        "Unknown".to_string()
    };
    
    // Log startup with actual build timestamp
    hookanchor::utils::debug_log("STARTUP", &format!("HookAnchor starting - Build: {}", build_time_str));
    
    // Initialize the global error queue for user error display
    hookanchor::error_display::init_error_queue();
    
    // Check if this is first run and run setup if needed
    if let Ok(true) = hookanchor::setup_assistant::check_and_run_setup() {
        // Setup was run, exit so user can configure Karabiner
        // They'll launch HookAnchor again after setup
        return Ok(());
    }
    
    let args: Vec<String> = env::args().collect();
    
    // If arguments are provided, run in command-line mode (no GUI)
    if args.len() > 1 {
        // CLI mode needs server - start it here
        if let Err(e) = hookanchor::server_management::start_server_if_needed() {
            hookanchor::utils::debug_log("STARTUP", &format!("Failed to start command server: {}", e));
            // Continue - commands will show error dialogs when server is needed
        }
        
        // CLI mode can create its own ApplicationState
        hookanchor::cmd::run_command_line_mode(args);
        
        // Note: We don't shutdown the server here since CLI commands
        // may spawn background processes that need the server
        Ok(())
    } else {
        // GUI mode - skip server startup and ApplicationState creation for speed
        // Server will be started on-demand when actually needed for command execution
        let result = hookanchor::ui::run_gui_with_prompt("", ApplicationState::minimal());
        
        result
    }
}