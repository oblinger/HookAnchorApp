//! HookAnchor Dispatcher
//! 
//! Main entry point that routes execution based on launch context:
//! - URL handling (--hook) → Server communication
//! - CLI arguments → Command line mode
//! - No arguments → GUI popup
//!
//! ⚠️ CRITICAL: READ docs/URL_HANDLING.md BEFORE MODIFYING URL HANDLING ⚠️

use std::env;
use std::process::{Command, exit};
use std::path::PathBuf;

fn main() {
    // Initialize binary path for consistent spawning
    hookanchor::init_binary_path();
    
    // Initialize config FIRST - this must happen before any other operations
    if let Err(config_error) = hookanchor::core::sys_data::initialize_config() {
        hookanchor::utils::log_error(&format!("Failed to load config: {}", config_error));
        // Continue with default config
    }
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Determine execution mode based on arguments
    match args.as_slice() {
        // No arguments - launch GUI popup (normal use case)
        [] | [_] => launch_popup(),
        
        // GUI mode - explicitly launch popup
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
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    
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
    use hookanchor::{execute_via_server, utils};
    use hookanchor::core::{sys_data, commands};
    
    // Extract the query from hook://query
    let query = url.strip_prefix("hook://").unwrap_or("");
    
    // URL decode the query
    let decoded_query = match urlencoding::decode(query) {
        Ok(decoded) => decoded,
        Err(_) => {
            utils::debug_log("DISPATCHER", &format!("Failed to decode URL: {}", url));
            return;
        }
    };
    
    if decoded_query.is_empty() {
        utils::debug_log("DISPATCHER", "Empty query in hook URL");
        return;
    }
    
    // Visual separator for URL handler execution
    utils::debug_log("", "=================================================================");
    utils::debug_log("USER INPUT", &format!("URL: '{}'", decoded_query));
    utils::debug_log("DISPATCHER", &format!("Processing hook URL: {} -> query: '{}'", url, decoded_query));
    
    // Find the top matching command using the same logic as CLI and GUI
    let sys_data = sys_data::get_sys_data();
    let filtered = commands::get_display_commands(&sys_data, &decoded_query, 1);
    
    if filtered.is_empty() {
        utils::debug_log("DISPATCHER", &format!("No commands found for query: '{}'", decoded_query));
        return;
    }
    
    let top_command_obj = &filtered[0];
    utils::debug_log("DISPATCHER", &format!("Executing command: {}", top_command_obj.command));
    
    // Execute via server to avoid GUI context and ensure consistent execution
    let launcher_cmd = if top_command_obj.arg.is_empty() {
        top_command_obj.action.clone()
    } else {
        format!("{} {}", top_command_obj.action, top_command_obj.arg)
    };
    
    utils::debug_log("DISPATCHER", &format!("Launching via server: {}", launcher_cmd));
    
    match execute_via_server(&launcher_cmd, None, None, false) {
        Ok(response) => {
            if response.success {
                utils::debug_log("DISPATCHER", "Command executed successfully via server");
            } else {
                utils::debug_log("DISPATCHER", &format!("Server execution failed: {}", response.stderr));
            }
        }
        Err(e) => {
            utils::debug_log("DISPATCHER", &format!("Server communication failed: {}", e));
        }
    }
}