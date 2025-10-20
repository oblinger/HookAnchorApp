//! HookAnchor Dispatcher
//!
//! Main entry point that routes execution based on launch context:
//! - URL handling (--hook) → Server communication
//! - --popup → Launch GUI popup
//! - CLI arguments → Command line mode
//! - No arguments → Show help
//!
//! ⚠️ CRITICAL: READ docs/URL_HANDLING.md BEFORE MODIFYING URL HANDLING ⚠️

use std::env;
use std::process::{Command, exit};
use std::path::PathBuf;

fn main() {
    // Initialize binary path for consistent spawning
    hookanchor::utils::init_binary_path();
    
    // Initialize config FIRST - this must happen before any other operations
    if let Err(config_error) = hookanchor::core::initialize_config() {
        hookanchor::utils::log_error(&format!("Failed to load config: {}", config_error));
        // Continue with default config
    }
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Determine execution mode based on arguments
    match args.as_slice() {
        // No arguments - show help (user should use --popup to launch GUI)
        [] | [_] => {
            hookanchor::cmd::run_command_line_mode(args);
        }

        // Popup mode - explicitly launch popup
        [_, flag] if flag == "--popup" => launch_popup(),

        // GUI mode - explicitly launch popup (deprecated, use --popup instead)
        [_, flag] if flag == "--gui" => launch_popup(),

        // URL handler mode - process hook URLs via server
        [_, flag, query] if flag == "--hook" => {
            // Add hook:// prefix if not present
            let url = if query.starts_with("hook://") {
                query.to_string()
            } else {
                format!("hook://{}", query)
            };
            handle_hook_url(&url);
        }

        // CLI mode - forward to command line processor
        _ => {
            // Forward to CLI handling in cmd module
            hookanchor::cmd::run_command_line_mode(args);
        }
    }
}

/// Launch the GUI popup application
fn launch_popup() {
    let exe_dir = hookanchor::utils::get_binary_dir();
    let popup_path = exe_dir.join("popup");
    
    // Launch popup as a separate process
    match Command::new(&popup_path).spawn() {
        Ok(_) => {
            // Dispatcher exits after launching popup
            exit(0);
        }
        Err(e) => {
            hookanchor::utils::log_error(&format!("Failed to launch popup: {}", e));
            hookanchor::utils::log_error(&format!("Expected popup at: {}", popup_path.display()));
            exit(1);
        }
    }
}

/// Handle hook:// URLs by sending them to the command server
fn handle_hook_url(url: &str) {
    use hookanchor::utils;
    use hookanchor::core::get_sys_data;
    use hookanchor::execute;
    
    // Extract the query from hook://query
    let query = url.strip_prefix("hook://").unwrap_or("");
    
    // URL decode the query
    let decoded_query = match urlencoding::decode(query) {
        Ok(decoded) => decoded,
        Err(_) => {
            utils::detailed_log("DISPATCHER", &format!("Failed to decode URL: {}", url));
            return;
        }
    };
    
    if decoded_query.is_empty() {
        utils::detailed_log("DISPATCHER", "Empty query in hook URL");
        return;
    }
    
    // Visual separator for URL handler execution
    utils::detailed_log("", "=================================================================");
    utils::detailed_log("USER INPUT", &format!("URL: '{}'", decoded_query));
    utils::detailed_log("DISPATCHER", &format!("Processing hook URL: {} -> query: '{}'", url, decoded_query));
    
    // Find the top matching command using the same logic as CLI and GUI
    let (sys_data, _) = get_sys_data();
    let (display_commands, _, _, _) = hookanchor::core::get_new_display_commands(&decoded_query, &sys_data.commands, &sys_data.patches);
    let filtered = display_commands.into_iter().take(1).collect::<Vec<_>>();
    
    if filtered.is_empty() {
        utils::detailed_log("DISPATCHER", &format!("No commands found for query: '{}'", decoded_query));
        return;
    }
    
    let top_command_obj = &filtered[0];
    utils::detailed_log("DISPATCHER", &format!("Executing command: {}", top_command_obj.command));
    
    // Execute via server to avoid GUI context and ensure consistent execution
    utils::detailed_log("DISPATCHER", &format!("Launching via server: {} ({})", top_command_obj.command, top_command_obj.action));
    
    // Execute command - handles all retries internally
    let action = execute::command_to_action(&top_command_obj);
    let mut variables = std::collections::HashMap::new();
    variables.insert("arg".to_string(), decoded_query.to_string());
    let _ = execute::execute_on_server(&action, Some(variables));
    utils::detailed_log("DISPATCHER", "Command sent to server");
}