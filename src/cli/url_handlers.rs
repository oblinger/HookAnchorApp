//! URL Handler Commands
//!
//! Functions for handling hook:// URL scheme requests.

use std::collections::HashMap;
use crate::utils::logging::print;
use crate::prelude::*;
use crate::execute::{execute_on_server, command_to_action};

/// Handle --hook command line option.
pub fn handle_hook_option(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha --hook <url>");
        std::process::exit(1);
    }

    handle_hook_url(&args[2]);
}

/// Handle a hook:// URL.
pub fn handle_hook_url(url: &str) {
    // Extract the query from hook://query
    let query = url.strip_prefix("hook://").unwrap_or("");

    // URL decode the query
    let decoded_query = urlencoding::decode(query).unwrap_or_else(|_| query.into());

    if decoded_query.is_empty() {
        detailed_log("URL_HANDLER", "Empty query in hook URL");
        return;
    }

    // Visual separator for URL handler execution
    detailed_log("", "=================================================================");
    detailed_log("USER INPUT", &format!("URL: '{}'", decoded_query));
    detailed_log("URL_HANDLER", &format!("Processing hook URL: {} -> query: '{}'", url, decoded_query));

    // Check for special prefixes: p/ (popup), a/ (action), x/ (explicit execute)
    if decoded_query.starts_with("p/") {
        handle_popup_url(&decoded_query);
        return;
    }

    if decoded_query.starts_with("a/") {
        handle_action_url(&decoded_query);
        return;
    }

    if decoded_query.starts_with("x/") {
        // Explicit execute - strip prefix and process as normal search
        let search_query = decoded_query.strip_prefix("x/").unwrap_or("");
        handle_execute_url(search_query);
        return;
    }

    // Default: search-and-execute (backward compatible)
    handle_execute_url(&decoded_query);
}

/// Handle popup mode URLs: hook://p/ or hook://p/TEXT or hook://p/TEXT/ACTION
fn handle_popup_url(query: &str) {
    // Extract parts: p/SEARCH_TEXT or p/SEARCH_TEXT/ACTION
    let without_prefix = query.strip_prefix("p/").unwrap_or("");
    let parts: Vec<&str> = without_prefix.splitn(2, '/').collect();

    let search_text = parts.get(0).unwrap_or(&"").to_string();
    let action_name = parts.get(1).map(|s| s.to_string());

    detailed_log("URL_HANDLER", &format!("Popup mode - input='{}' action='{:?}'", search_text, action_name));

    // Launch popup via binary
    let exe_dir = crate::utils::get_binary_dir();
    let popup_path = exe_dir.join("popup");

    let mut cmd = std::process::Command::new(&popup_path);
    cmd.arg("--popup");

    if !search_text.is_empty() {
        cmd.arg("--input").arg(&search_text);
    }

    if let Some(action) = action_name {
        cmd.arg("--action").arg(&action);
    }

    match cmd.spawn() {
        Ok(_) => detailed_log("URL_HANDLER", "Popup launched successfully"),
        Err(e) => detailed_log("URL_HANDLER", &format!("Failed to launch popup: {}", e)),
    }
}

/// Handle action mode URLs: hook://a/ACTION_NAME/ARG or hook://a/ACTION_NAME/ARG?param=val
fn handle_action_url(query: &str) {
    // Extract: a/ACTION_NAME/ARG?param1=value1&param2=value2
    let without_prefix = query.strip_prefix("a/").unwrap_or("");
    let (path_part, params_str) = without_prefix.split_once('?')
        .unwrap_or((without_prefix, ""));

    let parts: Vec<&str> = path_part.splitn(2, '/').collect();
    let action_name = parts.get(0).unwrap_or(&"").to_string();
    let arg_value = parts.get(1).unwrap_or(&"").to_string();

    detailed_log("URL_HANDLER", &format!("Action mode - action='{}' arg='{}' params='{}'", action_name, arg_value, params_str));

    // Get action from config
    let config = crate::core::data::get_config();
    if let Some(actions) = &config.actions {
        if let Some(action) = actions.get(&action_name) {
            // Parse query parameters
            let mut variables = HashMap::new();
            if !arg_value.is_empty() {
                variables.insert("arg".to_string(), arg_value.clone());
                variables.insert("input".to_string(), arg_value.clone()); // Also set as input for compatibility
            }

            for param_pair in params_str.split('&').filter(|s| !s.is_empty()) {
                if let Some((key, value)) = param_pair.split_once('=') {
                    variables.insert(key.to_string(), value.to_string());
                }
            }

            detailed_log("URL_HANDLER", &format!("Executing action '{}' with {} variables", action_name, variables.len()));
            let _ = execute_on_server(action, if variables.is_empty() { None } else { Some(variables) });
            detailed_log("URL_HANDLER", "Action executed");
            return;
        }
    }

    detailed_log("URL_HANDLER", &format!("Action '{}' not found in config", action_name));
}

/// Handle execute mode URLs: search and execute top match
fn handle_execute_url(search_query: &str) {
    // Use the same logic as -x command
    let (sys_data, _) = crate::core::data::get_sys_data();
    let config = crate::core::data::get_config();
    let (display_commands, _, _, _, _, _) = crate::core::get_new_display_commands(search_query, &sys_data.commands, &sys_data.patches, &config);
    let filtered = display_commands.into_iter().take(1).collect::<Vec<_>>();

    if filtered.is_empty() {
        detailed_log("URL_HANDLER", &format!("No commands found for query: '{}'", search_query));
        return;
    }

    let top_command_obj = &filtered[0];
    detailed_log("URL_HANDLER", &format!("Executing command: {}", top_command_obj.command));

    // For URL handling, execute directly via launcher (like -a action test) to avoid GUI context
    // This prevents the popup from showing when handling URLs
    detailed_log("URL_HANDLER", &format!("Launching via launcher: {} {}", top_command_obj.action, top_command_obj.arg));

    // Convert command to action and execute
    let action = command_to_action(&top_command_obj);
    let mut variables = HashMap::new();
    variables.insert("arg".to_string(), top_command_obj.arg.clone());
    let _ = execute_on_server(&action, Some(variables));
    detailed_log("URL_HANDLER", "Command executed");
}
