//! URL Handler for HookAnchor
//! 
//! A minimal binary for handling hook:// URLs without GUI popups.
//! This prevents system lockups that occurred when GUI popups were triggered from URL schemes.
//! 
//! ⚠️ CRITICAL: READ docs/URL_HANDLING.md BEFORE MODIFYING ⚠️

use std::env;
use hookanchor::{init_binary_path, execute_via_server, utils};
use hookanchor::core::{sys_data, commands};

fn main() {
    // Initialize binary path for consistent spawning
    init_binary_path();
    
    // Initialize config FIRST - this must happen before any other operations
    if let Err(config_error) = sys_data::initialize_config() {
        crate::utils::log_error(&format!("Failed to load config: {}", config_error));
        // Continue with default config
    }
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // For URL handling, macOS passes the URL via Apple Events, not command line args
    // However, we can also support direct command line invocation for testing
    if args.len() >= 2 && args[1].starts_with("hook://") {
        handle_hook_url(&args[1]);
    } else {
        eprintln!("URL Handler for HookAnchor");
        eprintln!("This binary handles hook:// URLs via the command server.");
        eprintln!("Usage: {} hook://query", args[0]);
        eprintln!();
        eprintln!("Note: Normally invoked by macOS via Apple Events for URL schemes.");
        std::process::exit(1);
    }
}

fn handle_hook_url(url: &str) {
    // Extract the query from hook://query
    let query = url.strip_prefix("hook://").unwrap_or("");
    
    // URL decode the query
    let decoded_query = match urlencoding::decode(query) {
        Ok(decoded) => decoded,
        Err(_) => {
            utils::debug_log("URL_HANDLER", &format!("Failed to decode URL: {}", url));
            return;
        }
    };
    
    if decoded_query.is_empty() {
        utils::debug_log("URL_HANDLER", "Empty query in hook URL");
        return;
    }
    
    // Visual separator for URL handler execution
    utils::debug_log("", "=================================================================");
    utils::debug_log("USER INPUT", &format!("URL: '{}'", decoded_query));
    utils::debug_log("URL_HANDLER", &format!("Processing hook URL: {} -> query: '{}'", url, decoded_query));
    
    // Find the top matching command using the same logic as CLI and GUI
    let sys_data = sys_data::get_sys_data();
    let filtered = commands::get_display_commands(&sys_data, &decoded_query, 1);
    
    if filtered.is_empty() {
        utils::debug_log("URL_HANDLER", &format!("No commands found for query: '{}'", decoded_query));
        return;
    }
    
    let top_command_obj = &filtered[0];
    utils::debug_log("URL_HANDLER", &format!("Executing command: {}", top_command_obj.command));
    
    // Execute via server to avoid GUI context and ensure consistent execution
    utils::debug_log("URL_HANDLER", &format!("Launching via server: {} ({})", top_command_obj.command, top_command_obj.action));
    
    match execute_via_server(&top_command_obj) {
        Ok(response) => {
            if response.success {
                utils::debug_log("URL_HANDLER", "Command executed successfully via server");
            } else {
                utils::debug_log("URL_HANDLER", &format!("Server execution failed: {}", response.stderr));
            }
        }
        Err(e) => {
            utils::debug_log("URL_HANDLER", &format!("Server communication failed: {}", e));
        }
    }
}