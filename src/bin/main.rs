//! Top-level application coordinator for HookAnchor
//!
//! This module handles global application state and dispatches to either
//! GUI mode (popup) or CLI mode (command-line processing).

use std::env;
use hookanchor::prelude::*;
use hookanchor::core::ApplicationState;

/// Main application entry point
/// 
/// Determines whether to run in GUI mode (no arguments) or CLI mode (with arguments)
fn main() -> Result<(), eframe::Error> {
    // Initialize global binary path for consistent process spawning
    hookanchor::utils::init_binary_path();

    // Initialize the global error queue for user error display
    hookanchor::utils::init_error_queue();

    // Initialize minimal sys_data (config + empty commands) for GUI mode
    // This prevents panics when UI tries to access data before it's loaded
    // Full data loading happens in deferred_loading after window is shown
    let _ = hookanchor::core::initialize_minimal();

    let args: Vec<String> = env::args().collect();

    // Parse optional --popup, --input and --action flags (these are for GUI mode)
    let mut input_text: Option<String> = None;
    let mut action_name: Option<String> = None;
    let mut force_popup = false;
    let mut has_other_args = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--popup" => {
                force_popup = true;
                i += 1;
            }
            "--input" if i + 1 < args.len() => {
                input_text = Some(args[i + 1].clone());
                i += 2;
            }
            "--action" if i + 1 < args.len() => {
                action_name = Some(args[i + 1].clone());
                i += 2;
            }
            _ => {
                has_other_args = true;
                i += 1;
            }
        }
    }

    // ⚠️ CRITICAL URL HANDLING WARNING ⚠️
    // READ docs/URL_HANDLING.md BEFORE MODIFYING ANY URL HANDLING CODE!
    // Incorrect URL handling has caused system-wide lockups and lost hours of work.
    //
    // IMPORTANT: macOS does NOT pass URLs via command line arguments when handling URL schemes!
    // macOS uses Apple Events to pass URLs to app bundles, not command line arguments.
    // When a URL like "hook://cnnp" is opened, macOS launches the app with no arguments (args.len() == 1)
    // and sends the URL via Apple Events to the running application.
    // URL handling must be implemented in the GUI application using Apple Event handlers.


    // If arguments are provided (other than --popup/--input/--action), run in command-line mode (no GUI)
    // Unless --popup flag is present, which forces GUI mode
    if has_other_args && !force_popup {
        // CLI mode - initialize sys_data immediately (needed for commands to work)
        match hookanchor::core::initialize() {
            Ok(()) => {
                // Sys data initialized successfully (config + cache loaded)
            }
            Err(init_error) => {
                log_error(&format!("Failed to initialize sys_data: {}", init_error));
                // Continue with default config
            }
        }

        // CLI mode needs server - ensure it's running
        if let Err(e) = hookanchor::execute::activate_command_server(false) {
            log_error(&format!("Failed to activate command server: {}", e));
            // Continue - commands will show error dialogs when server is needed
        }

        // CLI mode can create its own ApplicationState
        hookanchor::cmd::run_command_line_mode(args);

        // Note: We don't shutdown the server here since CLI commands
        // may spawn background processes that need the server
        Ok(())
    } else {
        // GUI mode - check if we're handling a URL immediately (no delay needed)
        // Check if any URL was passed via environment
        if let Ok(url) = env::var("HOOK_URL_HANDLER") {
            // URL handler mode - initialize sys_data immediately
            let _ = hookanchor::core::initialize();
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
                    let _ = std::fs::remove_file(url_marker);
                    // URL handler mode - initialize sys_data immediately
                    let _ = hookanchor::core::initialize();
                    hookanchor::cmd::run_command_line_mode(vec!["ha".to_string(), url.to_string()]);
                    return Ok(());
                }
            }
            let _ = std::fs::remove_file(url_marker); // Clean up even if invalid
        }
        
        // Check for install marker file - if present, run installer instead of main app
        if let Ok(exe_path) = std::env::current_exe() {
            let marker_path = exe_path
                .parent() // MacOS
                .and_then(|p| p.parent()) // Contents
                .map(|p| p.join("Resources").join("install_pending"));

            if let Some(marker) = marker_path {
                if marker.exists() {
                    // Launch installer GUI instead of main popup
                    let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
                    let installer_path = exe_dir.join("hookanchor_installer");

                    match std::process::Command::new(&installer_path).spawn() {
                        Ok(_) => {
                            // Installer launched successfully, exit this process
                            std::process::exit(0);
                        }
                        Err(e) => {
                            log_error(&format!("Failed to launch installer: {}", e));
                            // Continue with normal app - fallback behavior
                        }
                    }
                }
            }
        }

        // No URL detected and no installer needed - proceed with normal GUI mode
        let initial_input = input_text.as_deref().unwrap_or("");
        let initial_action = action_name.as_deref();
        let result = hookanchor::ui::run_gui_with_prompt(initial_input, initial_action, ApplicationState::minimal());

        result
    }
}