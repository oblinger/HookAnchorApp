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
use hookanchor::prelude::*;

fn main() {
    // Initialize binary path for consistent spawning
    hookanchor::utils::init_binary_path();

    // Initialize sys_data (config + cache) - this must happen before any other operations
    if let Err(init_error) = hookanchor::core::initialize() {
        log_error(&format!("Failed to initialize sys_data: {}", init_error));
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

        // Popup mode - explicitly launch popup (possibly with --input and --action args)
        [_, flag, ..] if flag == "--popup" => launch_popup_with_args(&args[1..]),

        // GUI mode - explicitly launch popup (deprecated, use --popup instead)
        [_, flag] if flag == "--gui" => launch_popup_with_args(&[]),

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

/// Launch the GUI popup application with optional arguments
///
/// Supports:
/// - `ha --popup` - open popup with empty input
/// - `ha --popup "text"` - open popup with text pre-filled
/// - `ha --popup "text" "action"` - open popup with text and trigger action
/// - `ha --popup --input "text" --action "action"` - explicit flag syntax (backward compatible)
fn launch_popup_with_args(args: &[String]) {
    let exe_dir = hookanchor::utils::get_binary_dir();
    let popup_path = exe_dir.join("popup");

    // Build command with arguments
    let mut cmd = Command::new(&popup_path);
    cmd.arg("--popup"); // Always pass --popup to the popup_server

    // Parse and pass through arguments
    let mut i = 1; // Skip "--popup" which is args[0]
    while i < args.len() {
        match args[i].as_str() {
            "--input" if i + 1 < args.len() => {
                cmd.arg("--input").arg(&args[i + 1]);
                i += 2;
            }
            "--action" if i + 1 < args.len() => {
                cmd.arg("--action").arg(&args[i + 1]);
                i += 2;
            }
            _ if !args[i].starts_with("--") => {
                // Non-flag argument - treat as input text
                cmd.arg(&args[i]);
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    // Launch popup as a separate process
    match cmd.spawn() {
        Ok(_) => {
            // Dispatcher exits after launching popup
            exit(0);
        }
        Err(e) => {
            log_error(&format!("Failed to launch popup: {}", e));
            log_error(&format!("Expected popup at: {}", popup_path.display()));
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
            detailed_log("DISPATCHER", &format!("Failed to decode URL: {}", url));
            return;
        }
    };

    if decoded_query.is_empty() {
        detailed_log("DISPATCHER", "Empty query in hook URL");
        return;
    }

    // Visual separator for URL handler execution
    detailed_log("", "=================================================================");
    detailed_log("USER INPUT", &format!("URL: '{}'", decoded_query));
    detailed_log("DISPATCHER", &format!("Processing hook URL: {} -> query: '{}'", url, decoded_query));

    // Check for special prefixes: p/ (popup), a/ (action), x/ (explicit execute)
    if decoded_query.starts_with("p/") {
        handle_popup_url_ha(&decoded_query);
        return;
    }

    if decoded_query.starts_with("a/") {
        handle_action_url_ha(&decoded_query);
        return;
    }

    if decoded_query.starts_with("x/") {
        // Explicit execute - strip prefix and process as normal search
        let search_query = decoded_query.strip_prefix("x/").unwrap_or("");
        handle_execute_url_ha(search_query);
        return;
    }

    // Default: search-and-execute (backward compatible)
    handle_execute_url_ha(&decoded_query);
}

/// Handle popup mode URLs: hook://p/ or hook://p/TEXT or hook://p/TEXT/ACTION
fn handle_popup_url_ha(query: &str) {
    // Extract parts: p/SEARCH_TEXT or p/SEARCH_TEXT/ACTION
    let without_prefix = query.strip_prefix("p/").unwrap_or("");
    let parts: Vec<&str> = without_prefix.splitn(2, '/').collect();

    let search_text = parts.get(0).unwrap_or(&"").to_string();
    let action_name = parts.get(1).map(|s| s.to_string());

    detailed_log("DISPATCHER", &format!("Popup mode - input='{}' action='{:?}'", search_text, action_name));

    // Launch popup via launch_popup_with_args
    let mut args = vec!["--popup".to_string()];
    if !search_text.is_empty() {
        args.push(search_text);
    }
    if let Some(action) = action_name {
        args.push(action);
    }

    launch_popup_with_args(&args);
}

/// Handle action mode URLs: hook://a/ACTION_NAME/ARG or hook://a/ACTION_NAME/ARG?param=val
fn handle_action_url_ha(query: &str) {
    use hookanchor::execute;

    // Extract: a/ACTION_NAME/ARG?param1=value1&param2=value2
    let without_prefix = query.strip_prefix("a/").unwrap_or("");
    let (path_part, params_str) = without_prefix.split_once('?')
        .unwrap_or((without_prefix, ""));

    let parts: Vec<&str> = path_part.splitn(2, '/').collect();
    let action_name = parts.get(0).unwrap_or(&"").to_string();
    let arg_value = parts.get(1).unwrap_or(&"").to_string();

    detailed_log("DISPATCHER", &format!("Action mode - action='{}' arg='{}' params='{}'", action_name, arg_value, params_str));

    // Get action from config
    let config = hookanchor::core::get_config();
    if let Some(actions) = &config.actions {
        if let Some(action) = actions.get(&action_name) {
            // Parse query parameters
            let mut variables = std::collections::HashMap::new();
            if !arg_value.is_empty() {
                variables.insert("arg".to_string(), arg_value.clone());
                variables.insert("input".to_string(), arg_value.clone()); // Also set as input for compatibility
            }

            for param_pair in params_str.split('&').filter(|s| !s.is_empty()) {
                if let Some((key, value)) = param_pair.split_once('=') {
                    variables.insert(key.to_string(), value.to_string());
                }
            }

            detailed_log("DISPATCHER", &format!("Executing action '{}' with {} variables", action_name, variables.len()));
            let _ = execute::execute_on_server(action, if variables.is_empty() { None } else { Some(variables) });
            detailed_log("DISPATCHER", "Action executed");
            return;
        }
    }

    detailed_log("DISPATCHER", &format!("Action '{}' not found in config", action_name));
}

/// Handle execute mode URLs: search and execute top match
fn handle_execute_url_ha(search_query: &str) {
    use hookanchor::execute;
    use hookanchor::core::get_sys_data;

    // Find the top matching command using the same logic as CLI and GUI
    let (sys_data, _) = get_sys_data();
    let config = hookanchor::core::get_config();
    let (display_commands, _, _, _) = hookanchor::core::get_new_display_commands(search_query, &sys_data.commands, &sys_data.patches, &config);
    let filtered = display_commands.into_iter().take(1).collect::<Vec<_>>();

    if filtered.is_empty() {
        detailed_log("DISPATCHER", &format!("No commands found for query: '{}'", search_query));
        return;
    }

    let top_command_obj = &filtered[0];
    detailed_log("DISPATCHER", &format!("Executing command: {}", top_command_obj.command));

    // Execute via server to avoid GUI context and ensure consistent execution
    detailed_log("DISPATCHER", &format!("Launching via server: {} ({})", top_command_obj.command, top_command_obj.action));

    // Execute command - handles all retries internally
    let action = execute::command_to_action(&top_command_obj);
    let mut variables = std::collections::HashMap::new();
    variables.insert("arg".to_string(), top_command_obj.arg.clone());
    let _ = execute::execute_on_server(&action, Some(variables));
    detailed_log("DISPATCHER", "Command sent to server");
}
