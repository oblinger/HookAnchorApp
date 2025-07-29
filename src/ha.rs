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
    // Initialize global binary path for consistent process spawning
    hookanchor::init_binary_path();
    
    // Visual separator for new app launch in logs
    hookanchor::utils::debug_log("", "════════════════════════════════════════════════════════════════");
    
    // Load state to get build time
    let state = hookanchor::core::state::load_state();
    let build_time_str = if let Some(build_time) = state.build_time {
        let dt = Local.timestamp_opt(build_time, 0).single()
            .unwrap_or_else(|| Local::now());
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        "Unknown".to_string()
    };
    
    // Log startup with version and build timestamp
    let version = env!("CARGO_PKG_VERSION");
    hookanchor::utils::debug_log("", &format!("HookAnchor v{} starting - Build: {}", version, build_time_str));
    
    // Initialize the global error queue for user error display
    hookanchor::error_display::init_error_queue();
    
    // Check if this is first run and run setup if needed
    if let Ok(true) = hookanchor::setup_assistant::check_and_run_setup() {
        // Setup was run, exit so user can configure Karabiner
        // They'll launch HookAnchor again after setup
        return Ok(());
    }
    
    let args: Vec<String> = env::args().collect();
    
    // IMPORTANT: macOS does NOT pass URLs via command line arguments when handling URL schemes!
    // macOS uses Apple Events to pass URLs to app bundles, not command line arguments.
    // When a URL like "hook://cnnp" is opened, macOS launches the app with no arguments (args.len() == 1)
    // and sends the URL via Apple Events to the running application.
    // URL handling must be implemented in the GUI application using Apple Event handlers.
    
    // Debug log all arguments and environment for URL handler debugging
    hookanchor::utils::debug_log("MAIN_DEBUG", &format!("Arguments: {:?}", args));
    hookanchor::utils::debug_log("MAIN_DEBUG", &format!("Args length: {}", args.len()));
    if let Ok(pwd) = env::var("PWD") {
        hookanchor::utils::debug_log("MAIN_DEBUG", &format!("PWD: {}", pwd));
    }
    
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
        // GUI mode - check if we're handling a URL by checking process arguments after a brief delay
        // macOS may take a moment to deliver URL events to the process
        std::thread::sleep(std::time::Duration::from_millis(50));
        
        // Check if any URL was passed via environment or if we can detect URL handling
        if let Ok(url) = env::var("HOOK_URL_HANDLER") {
            hookanchor::utils::debug_log("URL_HANDLER", &format!("URL handler mode detected: {}", url));
            hookanchor::cmd::run_command_line_mode(vec!["ha".to_string(), url]);
            return Ok(());
        }
        
        // For now, implement a check for recent URL file to handle URL events
        // This is a temporary solution until we implement proper Apple Event handling
        let url_marker = "/tmp/hookanchor_url_launch";
        if std::path::Path::new(url_marker).exists() {
            if let Ok(url_content) = std::fs::read_to_string(url_marker) {
                let url = url_content.trim();
                if !url.is_empty() && url.starts_with("hook://") {
                    hookanchor::utils::debug_log("URL_HANDLER", &format!("Processing URL from marker file: {}", url));
                    let _ = std::fs::remove_file(url_marker);
                    hookanchor::cmd::run_command_line_mode(vec!["ha".to_string(), url.to_string()]);
                    return Ok(());
                }
            }
            let _ = std::fs::remove_file(url_marker); // Clean up even if invalid
        }
        
        // No URL detected - proceed with normal GUI mode
        hookanchor::utils::debug_log("GUI_MODE", "No URL detected, proceeding with GUI");
        let result = hookanchor::ui::run_gui_with_prompt("", ApplicationState::minimal());
        
        result
    }
}