use std::collections::HashMap;
use crate::core::{load_commands_for_inference, filter_commands, run_patch_inference, Command};
use crate::utils;
use crate::utils::logging::print;
use crate::execute::{execute_on_server, make_action, command_to_action};

/// Main entry point for command-line mode
pub fn run_command_line_mode(args: Vec<String>) {
    // Check for hook:// URL as first argument
    if args.len() >= 2 && args[1].starts_with("hook://") {
        handle_hook_url(&args[1]);
        return;
    }
    
    // Check for help flags with optional subcommands
    if args.len() >= 2 && (args[1] == "-h" || args[1] == "--help") {
        if args.len() >= 3 {
            match args[2].to_lowercase().as_str() {
                "vars" => {
                    print_help_vars();
                    return;
                }
                "config" => {
                    print_help_config();
                    return;
                }
                "fns" => {
                    print_help_fns();
                    return;
                }
                _ => {
                    print(&format!("Unknown help topic: {}", args[2]));
                    print("Available topics: vars, config, fns");
                    std::process::exit(1);
                }
            }
        }
        print_help(&args[0]);
        return;
    }
    
    if args.len() < 2 {
        print_help(&args[0]);
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "-m" | "--match" => run_match_command(&args),
        "-r" | "--run_fn" => run_exec_command(&args),
        "-x" | "--execute" => run_execute_top_match(&args),
        "-c" | "--command" => run_test_command(&args),  // Renamed: test command with action and arg
        "-a" | "--action" => run_action_directly(&args),  // New: execute action directly
        "--hook" => handle_hook_option(&args),
        "--user-info" => print_user_info(),
        "--test-grabber" => run_test_grabber(),
        "--test-permissions" => run_test_permissions(),
        "--grab" => run_grab_command(&args),
        "--infer" => run_infer_patches(&args),
        "--infer-all" => run_infer_all_patches(&args),
        "--rescan" => run_rescan_command(),
        "--rebuild" => run_rebuild_command(),
        "--start-server" => run_start_server(),
        "--start-server-daemon" => run_start_server_daemon(),
        "--restart" => run_restart_server(),
        "--process-health" => run_process_health(),
        "--process-status" => run_process_status(),
        "--install" => run_install(&args),
        "--uninstall" => run_uninstall(),
        "--execute-launcher-command" => run_execute_launcher_command(&args),
        "--search" => run_search_command(),
        "--delete-history" => run_delete_history(&args),
        "--load-legacy-and-compare" => run_load_legacy_and_compare(&args),
        _ => {
            print(&format!("Unknown command: {}", args[1]));
            print("Use -h or --help for usage information");
            std::process::exit(1);
        }
    }
}

pub fn print_help(program_name: &str) {
    print("HookAnchor - Universal Command Launcher");
    print("");
    print("Help Topics:");
    print(&format!("  {} -h, --help               # Show this help message", program_name));
    print(&format!("  {} --help vars              # Template variables ({{{{input}}}}, {{{{date.year}}}}, etc.)", program_name));
    print(&format!("  {} --help config            # Configuration file structure (YAML)", program_name));
    print(&format!("  {} --help fns               # JavaScript functions (log, run_command, etc.)", program_name));
    print("");
    print("Usage:");
    print(&format!("  {} -m, --match <query>      # Search CMDS", program_name));
    print(&format!("  {} -r, --run_fn <cmd>       # Execute specific CMD", program_name));
    print(&format!("  {} -x, --execute <query>    # Execute top match", program_name));
    print(&format!("  {} -f, --folders <query>    # Get folder paths", program_name));
    print(&format!("  {} -F, --named-folders <q>  # Get CMDS->paths", program_name));
    print(&format!("  {} -c, --command <act> <arg># Test command with action+arg", program_name));
    print(&format!("  {} -a, --action <name>      # Execute action directly", program_name));
    print(&format!("  {} --infer [command]        # Show patch inference changes", program_name));
    print(&format!("  {} --infer-all              # Show changes and prompt to apply", program_name));
    print(&format!("  {} --rescan                 # Rescan filesystem with verbose output", program_name));
    print(&format!("  {} --rebuild                # Rebuild: restart server and rescan filesystem", program_name));
    print(&format!("  {} --delete-history [--force] # Delete history database and cache", program_name));
    print(&format!("  {} --test-grabber           # Test grabber functionality", program_name));
    print(&format!("  {} --test-permissions       # Test accessibility permissions", program_name));
    print(&format!("  {} --grab [delay]           # Grab active app after delay", program_name));
    print(&format!("  {} --start-server           # Force restart command server", program_name));
    print(&format!("  {} --restart                # Kill and restart command server in new Terminal", program_name));
    print(&format!("  {} --process-health         # Check for hung processes", program_name));
    print(&format!("  {} --process-status         # Show detailed process status", program_name));
    print(&format!("  {} --install                # Run setup assistant (preserves configs)", program_name));
    print(&format!("  {} --install --force        # Force reinstall (overwrites configs)", program_name));
    print(&format!("  {} --uninstall              # Uninstall HookAnchor", program_name));
    print(&format!("  {} --popup                  # Launch popup search interface", program_name));
    print(&format!("  {} --search                 # Launch history viewer", program_name));
    print(&format!("  {} --hook <url>             # Handle hook:// URL (for URL handler)", program_name));
    print(&format!("  {} --load-legacy-and-compare <path> # Load legacy commands and compare", program_name));
    print("  open 'hook://query'         # Handle hook URL via URL handler");
    print("");
    print("Examples:");
    print(&format!("  {} --popup           # Launch popup search interface", program_name));
    print(&format!("  {} -m spot   # Find 'spot' CMDS", program_name));
    print(&format!("  {} -x spot   # Execute top 'spot'", program_name));
    print(&format!("  {} -f spot   # Get 'spot' folders", program_name));
    print(&format!("  {} -F spo    # Get 'spo' CMDS->paths", program_name));
    print(&format!("  {} -r Spot   # Execute 'Spot' CMD", program_name));
}


fn handle_hook_option(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} --hook <url>", args[0]));
        std::process::exit(1);
    }
    
    handle_hook_url(&args[2]);
}

fn handle_hook_url(url: &str) {
    // Extract the query from hook://query
    let query = url.strip_prefix("hook://").unwrap_or("");
    
    // URL decode the query
    let decoded_query = urlencoding::decode(query).unwrap_or_else(|_| query.into());
    
    if decoded_query.is_empty() {
        utils::detailed_log("URL_HANDLER", "Empty query in hook URL");
        return;
    }
    
    // Visual separator for URL handler execution
    utils::detailed_log("", "=================================================================");
    utils::detailed_log("USER INPUT", &format!("URL: '{}'", decoded_query));
    utils::detailed_log("URL_HANDLER", &format!("Processing hook URL: {} -> query: '{}'", url, decoded_query));
    
    // Client environment logging is now handled automatically in execute_command based on action type
    
    // Use the same logic as -x command
    let (sys_data, _) = crate::core::data::get_sys_data();
    let (display_commands, _, _, _) = crate::core::get_new_display_commands(&decoded_query, &sys_data.commands, &sys_data.patches);
    let filtered = display_commands.into_iter().take(1).collect::<Vec<_>>();
    
    if filtered.is_empty() {
        utils::detailed_log("URL_HANDLER", &format!("No commands found for query: '{}'", decoded_query));
        return;
    }
    
    let top_command_obj = &filtered[0];
    utils::detailed_log("URL_HANDLER", &format!("Executing command: {}", top_command_obj.command));
    
    // For URL handling, execute directly via launcher (like -a action test) to avoid GUI context
    // This prevents the popup from showing when handling URLs
    utils::detailed_log("URL_HANDLER", &format!("Launching via launcher: {} {}", top_command_obj.action, top_command_obj.arg));
    
    // Convert command to action and execute
    let action = command_to_action(&top_command_obj);
    let mut variables = HashMap::new();
    variables.insert("arg".to_string(), top_command_obj.arg.clone());
    let _ = execute_on_server(&action, Some(variables));
    utils::detailed_log("URL_HANDLER", "Command executed");
}

// Helper: Resolve alias to target command
fn resolve_alias_to_target<'a>(cmd: &'a crate::core::commands::Command, all_commands: &'a [crate::core::commands::Command]) -> &'a crate::core::commands::Command {
    if cmd.action == "alias" {
        // Find the target command
        let target_lower = cmd.arg.to_lowercase();
        if let Some(target_cmd) = all_commands.iter().find(|c|
            c.command.to_lowercase() == target_lower ||
            // Handle patch! prefix format
            (c.command.contains('!') && c.command.split('!').nth(1).map(|s| s.trim().to_lowercase()) == Some(target_lower.clone()))
        ) {
            return target_cmd;
        }
    }
    // Return original if not an alias or target not found
    cmd
}

// Helper: Extract folder path from a command
fn get_command_folder(cmd: &crate::core::commands::Command) -> Option<String> {
    // For folder/anchor commands, the arg is the folder path
    if cmd.action == "folder" || cmd.action == "anchor" {
        if !cmd.arg.is_empty() {
            return Some(cmd.arg.clone());
        }
    }
    // For file-based commands (markdown, etc.), extract directory from file path in arg
    if cmd.action == "markdown" || cmd.action == "file" {
        if !cmd.arg.is_empty() {
            if let Some(parent) = std::path::Path::new(&cmd.arg).parent() {
                return Some(parent.to_string_lossy().to_string());
            }
        }
    }
    None
}

// Helper: Extract full file path from a command
fn get_command_path(cmd: &crate::core::commands::Command) -> Option<String> {
    // For file/markdown commands, the arg is the full path
    if (cmd.action == "markdown" || cmd.action == "file" || cmd.action == "folder" || cmd.action == "anchor") && !cmd.arg.is_empty() {
        return Some(cmd.arg.clone());
    }
    None
}

fn run_match_command(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} -m, --match <query> [--exact] [--format=name|folder|path|json]", args[0]));
        std::process::exit(1);
    }

    let query = &args[2];

    // Parse flags
    let mut exact_mode = false;
    let mut format = "name"; // default format
    let mut debug = false;

    for arg in args.iter().skip(3) {
        if arg == "--exact" {
            exact_mode = true;
        } else if arg.starts_with("--format=") {
            format = arg.strip_prefix("--format=").unwrap_or("name");
        } else if arg == "debug" {
            debug = true;
        }
    }

    let (sys_data, _) = crate::core::data::get_sys_data();

    // Always use the shared display logic (same as popup)
    let (mut filtered, is_prefix_menu) = if debug {
        // Legacy debug mode only
        (filter_commands(&sys_data.commands, query, 10, debug), false)
    } else {
        // Use shared popup matching logic - handles aliases, prefix menus, scoring
        let (display_commands, is_prefix_menu, _prefix_menu_info, _prefix_menu_count) =
            crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches);
        (display_commands, is_prefix_menu)
    };

    // Detect if first result is an exact match or prefix menu match
    let is_exact_match = !filtered.is_empty() && (
        crate::core::display::exact_match(&filtered[0].command, query) ||
        is_prefix_menu  // Prefix menu from alias resolution counts as exact match
    );

    // Apply exact mode filtering if requested
    if exact_mode {
        if is_exact_match {
            // Keep only the first result (the exact/prefix menu match)
            filtered.truncate(1);
        } else {
            // No exact match found - filter all results for exact matches
            filtered.retain(|cmd| crate::core::display::exact_match(&cmd.command, query));
        }
    }

    // Determine exit code based on number of matches
    let exit_code = match filtered.len() {
        0 => 2, // No matches
        1 => 0, // Unique match
        _ => 1, // Multiple matches (ambiguous)
    };

    // Output based on format
    match format {
        "name" => {
            // Print first 50 command names
            for cmd in filtered.iter().take(50) {
                print(&format!("{}", cmd.command));
            }
        }
        "folder" => {
            // Print folder paths (resolve aliases, skip commands without folders)
            for cmd in filtered.iter().take(50) {
                let resolved = resolve_alias_to_target(cmd, &sys_data.commands);
                if let Some(folder) = get_command_folder(resolved) {
                    print(&folder);
                }
            }
        }
        "path" => {
            // Print full file paths (resolve aliases, skip commands without file paths)
            for cmd in filtered.iter().take(50) {
                let resolved = resolve_alias_to_target(cmd, &sys_data.commands);
                if let Some(path) = get_command_path(resolved) {
                    print(&path);
                }
            }
        }
        "json" => {
            // Print JSON objects (one per line)
            for cmd in filtered.iter().take(50) {
                if let Ok(json) = serde_json::to_string(cmd) {
                    print(&json);
                }
            }
        }
        _ => {
            print(&format!("Error: Unknown format '{}'", format));
            std::process::exit(1);
        }
    }

    std::process::exit(exit_code);
}

fn run_exec_command(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} -r, --run_fn <command>", args[0]));
        std::process::exit(1);
    }
    
    let command = &args[2];
    
    // Visual separator for run function execution
    utils::detailed_log("", "=================================================================");
    utils::detailed_log("USER INPUT", &format!("RUN_FN: '{}'", command));
    
    print(&format!("Executing command function: {}", command));
    
    // Load system data to find the actual command
    let (sys_data, _) = crate::core::data::get_sys_data();
    
    // Find the command by name (case-insensitive)
    let matching_cmd = sys_data.commands.iter()
        .find(|cmd| cmd.command.to_lowercase() == command.to_lowercase());
    
    if let Some(cmd) = matching_cmd {
        // Execute the actual Command object
        // Convert command to action and execute
        let action = command_to_action(&cmd);
        let mut variables = HashMap::new();
        variables.insert("arg".to_string(), cmd.arg.clone());
        let _ = execute_on_server(&action, Some(variables));
        print("Command completed");
    } else {
        print(&format!("Command '{}' not found", command));
        std::process::exit(1);
    }
}

fn run_execute_top_match(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} -x, --execute <query>", args[0]));
        std::process::exit(1);
    }
    
    let query = &args[2];
    
    // Visual separator for command execution
    utils::detailed_log("", "=================================================================");
    utils::detailed_log("USER INPUT", &format!("CLI: '{}'", query));
    
    // Client environment logging is now handled automatically in execute_command based on action type
    
    let (sys_data, _) = crate::core::data::get_sys_data();

    // First, find if there's an exact match (including aliases)
    let exact_match = sys_data.commands.iter()
        .find(|cmd| cmd.command.eq_ignore_ascii_case(query))
        .cloned();

    // Get display commands which may resolve aliases
    let (display_commands, _, _, _) = crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches);
    let filtered = display_commands.into_iter().take(1).collect::<Vec<_>>();

    if filtered.is_empty() {
        print(&format!("No commands found matching: {}", query));
        std::process::exit(1);
    }

    // Use the exact match if we have one (preserves original alias), otherwise use filtered result
    let (top_command_obj, original_name) = if let Some(exact) = exact_match.as_ref() {
        // We have an exact match - use it to preserve the original alias name
        (exact.clone(), exact.command.clone())
    } else {
        // No exact match, use the filtered result
        (filtered[0].clone(), filtered[0].command.clone())
    };

    print(&format!("Executing top match: {}", top_command_obj.command));

    // Save the last executed command for add_alias functionality
    crate::utils::detailed_log("STATE_SAVE", &format!("CLI: Attempting to save last executed command: '{}'", original_name));
    let mut state = crate::core::data::get_state();
    state.last_executed_command = Some(original_name.clone());
    match crate::core::data::set_state(&state) {
        Ok(_) => crate::utils::detailed_log("STATE_SAVE", "CLI: Successfully saved last executed command"),
        Err(e) => crate::utils::detailed_log("STATE_SAVE", &format!("CLI: Failed to save last executed command: {}", e)),
    }

    
    // Use server-based execution for consistent environment
    // Convert command to action and execute with parameters
    let action = command_to_action(&top_command_obj);
    let mut variables = std::collections::HashMap::new();
    variables.insert("arg".to_string(), top_command_obj.arg.clone());
    let _ = execute_on_server(&action, Some(variables));
    print("Command completed");
}

fn run_test_command(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} -c, --command <action_name> [--arg <value>] [--input <value>] [--param key=value]...", args[0]));
        print("Examples:");
        print(&format!("  {} -c open_url --arg https://github.com", args[0]));
        print(&format!("  {} -c template --input \"My Note\" --param action=markdown", args[0]));
        print(&format!("  {} -c popup --param popup_action=navigate --param dx=0 --param dy=1", args[0]));
        std::process::exit(1);
    }
    
    let action_name = &args[2];
    
    // Parse additional parameters
    let mut arg_value = String::new();
    let mut input_value = String::new();
    let mut extra_params = std::collections::HashMap::new();
    
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--arg" => {
                if i + 1 < args.len() {
                    arg_value = args[i + 1].clone();
                    i += 2;
                } else {
                    print("--arg requires a value");
                    std::process::exit(1);
                }
            }
            "--input" => {
                if i + 1 < args.len() {
                    input_value = args[i + 1].clone();
                    i += 2;
                } else {
                    print("--input requires a value");
                    std::process::exit(1);
                }
            }
            "--param" => {
                if i + 1 < args.len() {
                    // Parse key=value
                    if let Some((key, value)) = args[i + 1].split_once('=') {
                        extra_params.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                    } else {
                        print("--param requires key=value format");
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    print("--param requires a value");
                    std::process::exit(1);
                }
            }
            _ => {
                // For backward compatibility, treat as arg if no flag
                if i == 3 && !args[i].starts_with("--") {
                    arg_value = args[i].clone();
                }
                i += 1;
            }
        }
    }
    
    // Visual separator for action testing
    utils::detailed_log("", "=================================================================");
    utils::detailed_log("USER INPUT", &format!("ACTION: '{}' ARG: '{}' INPUT: '{}'", action_name, arg_value, input_value));
    
    // Try to execute as a unified action first
    let config = crate::core::data::get_config();
    if let Some(actions) = &config.actions {
        if let Some(action) = actions.get(action_name) {
            print(&format!("Testing unified action '{}' (type: {})", action_name, action.action_type()));
            
            // Prepare variables for action execution (as JsonValue)
            let mut variables = HashMap::new();
            if !input_value.is_empty() {
                variables.insert("input".to_string(), serde_json::Value::String(input_value));
            }
            
            // Add extra parameters to variables (already JsonValue)
            for (key, value) in extra_params {
                variables.insert(key.clone(), value.clone());
            }
            
            // Execute the action with simplified parameters - convert variables to String HashMap
            let mut string_variables = std::collections::HashMap::new();

            // Add arg if present
            if !arg_value.is_empty() {
                string_variables.insert("arg".to_string(), arg_value.clone());
            }

            // Convert JsonValue variables to String variables
            for (key, value) in variables {
                if let Some(string_value) = value.as_str() {
                    string_variables.insert(key, string_value.to_string());
                }
            }

            match execute_on_server(action, if string_variables.is_empty() { None } else { Some(string_variables) }) {
                Ok(result) => {
                    print(&format!("Action completed successfully: {}", result));
                }
                Err(e) => {
                    print(&format!("Action failed: {}", e));
                    std::process::exit(1);
                }
            }
            return;
        }
    }
    
    // Fall back to legacy action testing (for backward compatibility)
    let command_line = format!("{} {}", action_name, arg_value);
    print(&format!("Testing legacy action '{}' with arg '{}': {}", action_name, arg_value, command_line));
    
    // Use server-based execution for testing actions
    // Create action directly and execute
    let action = make_action(&action_name, &arg_value);
    let mut variables = HashMap::new();
    variables.insert("arg".to_string(), arg_value.clone());
    let _ = execute_on_server(&action, Some(variables));
    print("Action completed");
}

fn run_action_directly(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} -a, --action <action_type> [--key value]...", args[0]));
        print("Examples:");
        print(&format!("  {} -a markdown --arg /path/to/file.md", args[0]));
        print(&format!("  {} -a cmd --arg \"ls -la\" --flags W", args[0]));
        std::process::exit(1);
    }
    
    let action_type = &args[2];
    
    // Parse all --key value pairs
    let mut params = std::collections::HashMap::new();
    let mut i = 3;
    while i < args.len() {
        if args[i].starts_with("--") {
            let key = args[i].trim_start_matches("--");
            if i + 1 < args.len() {
                params.insert(key.to_string(), args[i + 1].clone());
                i += 2;
            } else {
                print(&format!("Error: {} requires a value", args[i]));
                std::process::exit(1);
            }
        } else {
            print(&format!("Error: Expected --key but got: {}", args[i]));
            std::process::exit(1);
        }
    }
    
    // Build the action structure based on type and parameters
    use crate::execute::Action;
    use serde_json::Value as JsonValue;
    
    print(&format!("Executing action '{}' with parameters:", action_type));
    for (key, value) in &params {
        print(&format!("  {}: {}", key, value));
    }
    
    // Create the action as a HashMap with JsonValue entries
    let mut action_params = std::collections::HashMap::new();
    action_params.insert("action_type".to_string(), JsonValue::String(action_type.to_string()));
    
    // Add all parameters from command line
    for (key, value) in params {
        action_params.insert(key, JsonValue::String(value));
    }
    
    let action = Action { params: action_params };

    // Execute through server - convert JsonValue params to String variables
    use crate::execute::execute_on_server;
    let mut variables = HashMap::new();

    // Extract all string parameters for template expansion
    for (key, value) in &action.params {
        if let Some(string_val) = value.as_str() {
            variables.insert(key.clone(), string_val.to_string());
        }
    }

    let _ = execute_on_server(&action, if variables.is_empty() { None } else { Some(variables) });
    print(&format!("Action '{}' completed", action_type));
}

/// Extract folder path from a command
fn extract_folder_path(command: &crate::core::Command) -> Option<String> {
    match command.action.as_str() {
        "folder" => {
            // For folder commands, the arg is already the folder path
            Some(command.arg.clone())
        },
        "markdown" => {
            // For markdown commands, arg is already absolute path - return the directory
            if let Some(last_slash) = command.arg.rfind('/') {
                Some(command.arg[..last_slash].to_string())
            } else {
                Some(command.arg.clone())
            }
        },
        _ => {
            // Skip commands that don't have folders
            None
        }
    }
}

/// Shared folder matching logic for both -f and -F flags
/// Returns (command_name, folder_path) pairs
/// For exact matches, returns single result; for fuzzy matches, returns multiple
fn get_folder_matches(query: &str, include_command_names: bool) -> Vec<(String, String)> {
    let (sys_data, _) = crate::core::data::get_sys_data();

    // Use the same display logic as the popup to get matching commands
    let (display_commands, _, _, _) = crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches);

    if display_commands.is_empty() {
        return Vec::new();
    }

    // Check for exact match BEFORE resolving aliases (check original command names)
    // Find first non-separator command
    let first_non_separator = display_commands.iter()
        .find(|cmd| cmd.action != "separator");

    if let Some(first_cmd) = first_non_separator {
        let query_lower = query.to_lowercase();
        let query_folder_pattern = format!("{} folder", query_lower);
        let cmd_lower = first_cmd.command.to_lowercase();

        // Check if the original command name (before alias resolution) matches exactly
        if cmd_lower == query_lower || cmd_lower == query_folder_pattern {
            // Found exact match - resolve alias if needed
            let resolved_cmd = if first_cmd.action == "alias" {
                first_cmd.resolve_alias(&sys_data.commands)
            } else {
                first_cmd.clone()
            };

            // Return just that single match
            if let Some(folder_path) = extract_folder_path(&resolved_cmd) {
                return vec![(resolved_cmd.command, folder_path)];
            }
        }
    }

    // No exact match found - expand aliases for fuzzy results and take first 100
    let display_commands: Vec<_> = display_commands.into_iter()
        .filter(|cmd| cmd.action != "separator")  // Skip separators
        .map(|cmd| {
            if cmd.action == "alias" {
                cmd.resolve_alias(&sys_data.commands)
            } else {
                cmd
            }
        })
        .take(100)
        .collect();

    // Collect folder matches
    if include_command_names {
        // For -F flag: return all command -> path pairs
        display_commands.iter()
            .filter_map(|cmd| {
                extract_folder_path(cmd).map(|path| (cmd.command.clone(), path))
            })
            .collect()
    } else {
        // For -f flag: return unique folder paths (deduplicate)
        let mut folder_paths = std::collections::HashSet::new();
        for command in &display_commands {
            if let Some(path) = extract_folder_path(command) {
                folder_paths.insert(path);
            }
        }

        let mut paths: Vec<String> = folder_paths.into_iter().collect();
        paths.sort(); // Sort for consistent output

        // Return as (empty_name, path) pairs for consistency
        paths.into_iter().map(|path| (String::new(), path)).collect()
    }
}

// Removed: -f and -F flags - use -m --format=folder instead

fn print_user_info() {
    print("=== User Environment Information ===");
    
    // Environment variables
    print(&format!("USER: {:?}", std::env::var("USER")));
    print(&format!("LOGNAME: {:?}", std::env::var("LOGNAME")));
    print(&format!("HOME: {:?}", std::env::var("HOME")));
    print(&format!("SHELL: {:?}", std::env::var("SHELL")));
    print(&format!("PATH: {:?}", std::env::var("PATH")));
    
    // Try whoami command
    use std::process::Command;
    match Command::new("whoami").output() {
        Ok(output) => {
            let whoami_user = String::from_utf8_lossy(&output.stdout).trim().to_string();
            print(&format!("whoami: '{}'", whoami_user));
        }
        Err(e) => {
            print(&format!("whoami failed: {}", e));
        }
    }
    
    // Try id command for more user info
    match Command::new("id").output() {
        Ok(output) => {
            let id_output = String::from_utf8_lossy(&output.stdout).trim().to_string();
            print(&format!("id: {}", id_output));
        }
        Err(e) => {
            print(&format!("id failed: {}", e));
        }
    }
    
    // Try to get user's shell profile paths
    if let Ok(home) = std::env::var("HOME") {
        print(&format!("Checking shell profiles in {}", home));
        let profiles = vec![".zshrc", ".bash_profile", ".bashrc", ".profile"];
        for profile in profiles {
            let path = format!("{}/{}", home, profile);
            if std::path::Path::new(&path).exists() {
                print(&format!("  {} exists", path));
            } else {
                print(&format!("  {} not found", path));
            }
        }
    }
}

/// Test the grabber functionality - capture context and try to match rules
fn run_test_grabber() {
    print("Testing grabber functionality...");
    print("==================================");
    
    // Load config
    let config = crate::core::data::get_config();
    print(&format!("Loaded config with {} grabber rules", 
        config.grabber_rules.as_ref().map(|r| r.len()).unwrap_or(0)));
    
    // Try to capture and test context
    print("\nCapturing active application context...");
    match crate::systems::grab_debug(&config) {
        Ok(context) => {
            print("SUCCESS: Captured app context");
            print(&format!("  App: '{}'", context.app_name));
            print(&format!("  Bundle ID: '{}'", context.bundle_id));
            print(&format!("  Title: '{}'", context.window_title));
            
            // Show properties if any
            if let Some(props_obj) = context.properties.as_object() {
                if !props_obj.is_empty() {
                    print("  Properties:");
                    for (key, value) in props_obj {
                        print(&format!("    {}: '{}'", key, value.as_str().unwrap_or("(complex value)")));
                    }
                }
            }
        }
        Err(e) => {
            print(&format!("ERROR: Failed to capture app context: {}", e));
            std::process::exit(1);
        }
    }
    
    print("\nGrabber test completed successfully!");
}

/// Test accessibility permissions for HookAnchor
fn run_test_permissions() {
    print("Testing Accessibility Permissions");
    print("==================================");
    print("Checking if HookAnchor has the required accessibility permissions...");
    print("");

    match crate::systems::setup_assistant::SetupAssistant::test_accessibility_permissions() {
        Ok(true) => {
            print("✅ SUCCESS: Accessibility permissions are granted!");
            print("HookAnchor can capture app context and send keystrokes.");
            print("");
            print("This means:");
            print("• Obsidian URL capture should work");
            print("• Notion URL capture should work");
            print("• Window title detection works");
            print("• Grabber functionality is fully operational");
        }
        Ok(false) => {
            print("❌ FAILED: Accessibility permissions are missing!");
            print("");
            print("This explains why:");
            print("• Obsidian grabber defaults to generic app capture");
            print("• Notion URL capture fails");
            print("• Some grabber features don't work");
            print("");
            print("To fix this, run:");
            print("  ha --install");
            print("");
            print("Or manually:");
            print("1. Open System Preferences → Security & Privacy → Privacy → Accessibility");
            print("2. Click the lock and enter your password");
            print("3. Add Terminal (or your current app) to the allowed list");
            print("4. Test again with: ha --test-permissions");
        }
        Err(e) => {
            print(&format!("❌ ERROR: Could not test permissions: {}", e));
            print("This might indicate a system configuration issue.");
        }
    }
}

/// Run grab command to capture active app and output result
fn run_grab_command(args: &[String]) {
    // Parse optional delay parameter (defaults to 0)
    let delay_seconds = if args.len() > 2 {
        match args[2].parse::<u64>() {
            Ok(d) => d,
            Err(_) => {
                print(&format!("Invalid delay value: {}", args[2]));
                print(&format!("Usage: {} --grab [delay_seconds]", args[0]));
                std::process::exit(1);
            }
        }
    } else {
        0
    };
    
    // If delay requested, sleep before capturing
    if delay_seconds > 0 {
        print(&format!("Waiting {} seconds before capture...", delay_seconds));
        std::thread::sleep(std::time::Duration::from_secs(delay_seconds));
    }
    
    // Load config for grabber rules
    let config = crate::core::data::get_config();
    
    // Perform the grab
    match crate::systems::grabber::grab(&config) {
        Ok(grab_result) => {
            match grab_result {
                crate::systems::grabber::GrabResult::RuleMatched(rule_name, command) => {
                    // Output action, argument, rule name, and suffix for the popup to process
                    // Format: "action arg RULE:rule_name FLAGS:suffix"
                    if command.flags.is_empty() {
                        print(&format!("{} {} RULE:{}", command.action, command.arg, rule_name));
                    } else {
                        print(&format!("{} {} RULE:{} FLAGS:{}", command.action, command.arg, rule_name, command.flags));
                    }
                }
                crate::systems::grabber::GrabResult::NoRuleMatched(context) => {
                    // No rule matched - output context information
                    print("No grabber rule matched for:");
                    print(&format!("  App: {}", context.app_name));
                    print(&format!("  Bundle ID: {}", context.bundle_id));
                    print(&format!("  Title: {}", context.window_title));
                    if let Some(props) = context.properties.as_object() {
                        for (key, value) in props {
                            print(&format!("  props.{}: {}", key, value.as_str().unwrap_or("(complex)")));
                        }
                    }
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            utils::log_error(&format!("Error capturing active app: {}", e));
            std::process::exit(1);
        }
    }
}

/// Force restart the command server
fn run_start_server() {
    utils::detailed_log("SYSTEM", &format!("Restarting command server..."));
    
    // Restart the server (kill existing and start new)
    match crate::execute::activate_command_server(true) {
        Ok(()) => {
            print("Command server restart initiated");
            print("The server will start with full shell environment in a few seconds");
        }
        Err(e) => {
            utils::log_error(&format!("Failed to restart server: {}", e));
            print(&format!("Failed to restart server: {}", e));
            std::process::exit(1);
        }
    }
}

/// Internal daemon mode - starts persistent server
fn run_start_server_daemon() {
    // Config will be initialized automatically by get_config() when needed
    // No need to initialize it explicitly here
    
    utils::detailed_log("SYSTEM", &format!("Starting command server daemon..."));
    
    // Change to home directory
    if let Ok(home) = std::env::var("HOME") {
        if let Err(e) = std::env::set_current_dir(&home) {
            utils::log_error(&format!("Could not change to home directory: {}", e));
        }
    }
    
    // Clean up any existing socket file
    let socket_path = std::path::Path::new(&std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
        .join(".config")
        .join("hookanchor")
        .join("execution_server.sock");
    let _ = std::fs::remove_file(&socket_path);
    
    // Run the command server (this never returns)
    match crate::execute::run_command_server() {
        Ok(_) => {
            // Should never reach here
        }
        Err(e) => {
            utils::log_error(&format!("Failed to start persistent server: {}", e));
            std::process::exit(1);
        }
    }
}


/// Show which commands would have their patches changed by inference
fn run_infer_patches(args: &[String]) {
    // Check if a specific command name was provided
    if args.len() >= 3 {
        let command_name = &args[2];
        run_infer_single_command(command_name);
        return;
    }
    
    print("Analyzing patch inference changes...");
    
    // Load current commands (without inference and virtual anchor creation)
    let (_config, mut commands, patches) = load_commands_for_inference();
    
    print("\n=== COMMAND PATCH CHANGES ===");
    
    // Run inference without applying changes, but print to stdout
    let (changes_found, new_patches_to_add) = run_patch_inference(
        &mut commands, 
        &patches, 
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true (show proposed changes)
        true   // overwrite_patch = true (show all potential changes)
    );
    
    if changes_found == 0 {
        print("  No commands would have their patches changed.");
    }
    
    // Show orphan patches that would be created
    if !new_patches_to_add.is_empty() {
        print("\n=== PATCHES NEEDING VIRTUAL ANCHORS ===");
        for new_patch in &new_patches_to_add {
            print(&format!("  New patch needing virtual anchor: {}", new_patch));
        }
    }
    
    print("\n=== SUMMARY ===");
    print(&format!("  Commands that would change: {}", changes_found));
    print(&format!("  Patches needing virtual anchors: {}", new_patches_to_add.len()));
    
    if changes_found == 0 && new_patches_to_add.is_empty() {
        print("  No changes would be made.");
    } else {
        print("  Use --infer-all to apply these changes (normal startup only fills empty patches).");
    }
}

/// Show patch inference for a specific command
fn run_infer_single_command(command_name: &str) {
    // Get current commands from singleton
    let (sys_data, _) = crate::core::get_sys_data();
    let commands_arc = sys_data.commands;
    let patches = sys_data.patches;

    // Build folder map for unified inference
    let folder_map = crate::core::inference::build_folder_to_patch_map(&commands_arc);

    // Find the command by name
    let found_command = commands_arc.iter().find(|cmd| cmd.command == command_name);

    match found_command {
        Some(command) => {
            print(&format!("Command: {}", command.command));
            print(&format!("Current patch: '{}'", command.patch));

            // Test patch inference on this specific command
            match crate::core::inference::infer_patch_unified(command, &patches, &folder_map) {
                Some(inferred_patch) => {
                    print(&format!("Inferred patch: '{}'", inferred_patch));
                    
                    if command.patch == inferred_patch {
                        print("✅ No change needed - current patch matches inferred patch");
                    } else if command.patch.is_empty() {
                        print(&format!("📄 Would assign patch: '{}' -> '{}'", command.patch, inferred_patch));
                    } else {
                        print(&format!("🔄 Would change patch: '{}' -> '{}'", command.patch, inferred_patch));
                    }
                }
                None => {
                    if command.patch.is_empty() {
                        print("No patch could be inferred (would remain empty)");
                    } else {
                        print(&format!("No patch could be inferred (would keep current: '{}')", command.patch));
                    }
                }
            }
        }
        None => {
            print(&format!("Command '{}' not found.", command_name));

            // Show similar commands as suggestions
            let similar_commands: Vec<&crate::core::Command> = commands_arc.iter()
                .filter(|cmd| cmd.command.to_lowercase().contains(&command_name.to_lowercase()))
                .take(5)
                .collect();
            
            if !similar_commands.is_empty() {
                print("\nSimilar commands found:");
                for cmd in similar_commands {
                    print(&format!("  {}", cmd.command));
                }
            }
        }
    }
}

/// Show patch inference changes and prompt user to apply them
fn run_infer_all_patches(_args: &[String]) {
    print("Analyzing patch inference changes...");
    
    // Load current commands (without inference and virtual anchor creation)
    let (_config, mut commands, patches) = load_commands_for_inference();
    
    print("\n=== COMMAND PATCH CHANGES ===");
    
    // First run: Show changes without applying
    let (changes_found, new_patches_to_add) = run_patch_inference(
        &mut commands, 
        &patches, 
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true (show proposed changes)
        true   // overwrite_patch = true (show all potential changes including overwriting existing patches)
    );
    
    if changes_found == 0 {
        print("  No commands would have their patches changed.");
    }
    
    // Show orphan patches that would be created
    if !new_patches_to_add.is_empty() {
        print("\n=== PATCHES NEEDING VIRTUAL ANCHORS ===");
        for new_patch in &new_patches_to_add {
            print(&format!("  New patch needing virtual anchor: {}", new_patch));
        }
    }
    
    if changes_found == 0 && new_patches_to_add.is_empty() {
        print("\nNo changes would be made.");
        return;
    }
    
    // Prompt user for confirmation
    print("\n=== SUMMARY ===");
    print(&format!("  Commands that would change: {}", changes_found));
    print(&format!("  Patches needing virtual anchors: {}", new_patches_to_add.len()));
    print!("\nApply all changes? (y/n): ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_lowercase();
    
    if input == "y" || input == "yes" {
        // Get commands from singleton and apply changes
        let (sys_data, _) = crate::core::get_sys_data();
        let mut commands = (*sys_data.commands).clone();
        let patches = sys_data.patches;
        
        // Second run: Apply changes without printing (already shown above)
        let (applied_count, _) = run_patch_inference(
            &mut commands, 
            &patches, 
            true,  // apply_changes = true (apply the changes)
            false, // print_to_stdout = false (don't print since we already showed them)
            true   // overwrite_patch = true (apply all changes including overwriting existing patches)
        );
        
        // Save the updated commands to file
        if applied_count > 0 {
            match crate::core::set_commands(commands) {
                Ok(()) => {
                    print(&format!("Updated {} commands and saved to file.", applied_count));
                }
                Err(e) => {
                    utils::log_error(&format!("Updated {} commands but failed to save: {}", applied_count, e));
                }
            }
        } else {
            print("No commands were actually updated.");
        }
    } else {
        print("No changes applied.");
    }
}

fn run_rescan_command() {
    print("🚀 HookAnchor Rescan - Verbose Mode");
    print("====================================");

    // Load configuration
    let config = crate::core::data::get_config();

    // Debug output
    print("📋 Config loaded - checking file_roots...");

    // Get file roots
    let _file_roots = match &config.popup_settings.file_roots {
        Some(roots) => {
            print(&format!("✅ File roots found: {:?}", roots));
            roots.clone()
        },
        None => {
            print("❌ No file roots configured in config file");
            print(&format!("    popup_settings exists: {}", config.popup_settings.file_roots.is_some()));
            std::process::exit(1);
        }
    };

    // === NEW CLEAN SINGLETON ARCHITECTURE ===
    // This eliminates duplicate history entries by using a single shared list

    print("\n📂 Step 1: Loading last known state from cache...");

    // Load from cache through sys_data
    let mut commands = crate::core::get_commands();
    if !commands.is_empty() {
        print(&format!("   ✅ Loaded {} commands from cache", commands.len()));
    } else {
        print("   Starting with empty commands");
    }

    print("\n✏️  Step 2: Merging manual edits from commands.txt...");

    // Load commands.txt and merge on top of cache state
    // This captures any manual edits while preserving file sizes and history from cache
    match crate::systems::load_manual_edits(&mut commands, true) {
        Ok(edits_count) => {
            print(&format!("   ✅ Merged {} edits from commands.txt", edits_count));
        }
        Err(e) => {
            print(&format!("   ⚠️  Error loading manual edits: {}", e));
        }
    }
    print(&format!("   📊 Total commands after merge: {}", commands.len()));

    // Track HA alias after loading manual edits
    let ha_after_load = commands.iter().find(|c| c.command == "HA" && c.action == "alias");
    if let Some(ha) = ha_after_load {
        crate::utils::log(&format!("ALIAS_TRACK: HA alias found after load_manual_edits - patch:'{}' flags:'{}' arg:'{}'", ha.patch, ha.flags, ha.arg));
    } else {
        crate::utils::log("ALIAS_TRACK: HA alias NOT found after load_manual_edits");
    }

    print("\n🔍 Step 3: Scanning filesystem (discovering new files, removing stale entries)...");

    // Scan filesystem - this will:
    // - Add newly discovered files
    // - Remove commands for files that no longer exist
    // - Preserve user-edited commands
    let (global_data, _) = crate::core::data::get_sys_data();

    let scanned_commands = crate::systems::scan_new_files(
        commands.clone(),
        &global_data,
        true
    );

    commands = scanned_commands;
    print(&format!("   ✅ Scan complete - now tracking {} total commands", commands.len()));

    // Track HA alias after filesystem scan
    let ha_after_scan = commands.iter().find(|c| c.command == "HA" && c.action == "alias");
    if let Some(ha) = ha_after_scan {
        crate::utils::log(&format!("ALIAS_TRACK: HA alias found after scan_new_files - patch:'{}' flags:'{}' arg:'{}'", ha.patch, ha.flags, ha.arg));
    } else {
        crate::utils::log("ALIAS_TRACK: HA alias NOT found after scan_new_files");
    }

    print("\n📝 Step 4: Detecting file modifications...");

    // Check if any file sizes changed and record history
    match crate::systems::scan_modified_files(&mut commands, true) {
        Ok(modified_count) => {
            print(&format!("   ✅ Processed modifications ({} files changed)", modified_count));
        }
        Err(e) => {
            print(&format!("   ⚠️  Error scanning modifications: {}", e));
        }
    }

    // Track HA alias after modification scan
    let ha_after_mod = commands.iter().find(|c| c.command == "HA" && c.action == "alias");
    if let Some(ha) = ha_after_mod {
        crate::utils::log(&format!("ALIAS_TRACK: HA alias found after scan_modified_files - patch:'{}' flags:'{}' arg:'{}'", ha.patch, ha.flags, ha.arg));
    } else {
        crate::utils::log("ALIAS_TRACK: HA alias NOT found after scan_modified_files");
    }

    print("\n🧹 Step 5: Cleaning up invalid aliases...");

    // Delete aliases that point to non-existent commands
    match crate::systems::delete_invalid_aliases(&mut commands, true) {
        Ok(removed_count) => {
            if removed_count > 0 {
                print(&format!("   ✅ Removed {} invalid alias(es)", removed_count));
            } else {
                print("   ✅ All aliases are valid");
            }
        }
        Err(e) => {
            print(&format!("   ⚠️  Error validating aliases: {}", e));
        }
    }

    print("\n🔄 Step 6: Running inference and saving...");

    // Save final state through sys_data (runs inference + saves to cache + commands.txt)
    let command_count = commands.len();
    match crate::core::set_commands(commands) {
        Ok(_) => {
            print(&format!("   ✅ Inference complete and saved {} commands", command_count));
        }
        Err(e) => {
            print(&format!("   ⚠️  Error during inference/save: {}", e));
        }
    }

    // Reload commands for summary (after save moved ownership)
    let commands = crate::core::get_commands();

    // Track HA alias after set_commands (final state)
    let ha_after_save = commands.iter().find(|c| c.command == "HA" && c.action == "alias");
    if let Some(ha) = ha_after_save {
        crate::utils::log(&format!("ALIAS_TRACK: HA alias found after set_commands - patch:'{}' flags:'{}' arg:'{}'", ha.patch, ha.flags, ha.arg));
    } else {
        crate::utils::log("ALIAS_TRACK: HA alias NOT found after set_commands - IT WAS LOST!");
    }

    print("\n📊 Step 8: Final Summary:");
    print(&format!("   Total commands: {}", commands.len()));

    // Count commands by action type
    let mut action_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for cmd in &commands {
        *action_counts.entry(cmd.action.clone()).or_insert(0) += 1;
    }

    print("\n   Commands by action type:");
    let mut sorted_actions: Vec<_> = action_counts.iter().collect();
    sorted_actions.sort_by_key(|(action, _)| (*action).clone());

    for (action, count) in sorted_actions {
        print(&format!("     {}: {}", action, count));
    }

    print("\n✅ Rescan complete!");
}

/// Check for hung processes
fn run_process_health() {
    print("Checking process health...");
    crate::utils::subprocess::check_system_health();
    print("Health check complete. See debug logs for details.");
}

/// Show detailed process status
fn run_process_status() {
    print("Process status:");
    crate::utils::subprocess::show_process_status();
}

/// Run the setup assistant (install) - Launch GUI installer
fn run_install(args: &[String]) {
    // Check if --force flag is present for future use
    let _force = args.len() > 2 && args[2] == "--force";

    print("🚀 Launching HookAnchor GUI installer...");
    print("");

    // Get the current executable path to find the installer_gui binary
    let exe_dir = utils::get_binary_dir();
    let installer_path = exe_dir.join("installer_gui");

    // Launch the GUI installer
    match std::process::Command::new(&installer_path)
        .spawn()
    {
        Ok(_) => {
            print("✅ GUI installer launched successfully!");
            print("   Complete the installation in the GUI window.");
        }
        Err(e) => {
            print(&format!("❌ Failed to launch GUI installer: {}", e));
            print("");
            print("Troubleshooting:");
            print(&format!("  - Make sure installer_gui binary exists at: {}", installer_path.display()));
            print("  - Try rebuilding with: cargo build --release");
            print("  - Check file permissions");
        }
    }
}

/// Uninstall HookAnchor
fn run_uninstall() {
    // Check if uninstall script exists and use it
    if let Ok(home) = std::env::var("HOME") {
        let uninstall_script = format!("{}/.config/hookanchor/uninstall.sh", home);
        if std::path::Path::new(&uninstall_script).exists() {
            print("🗑️  Found uninstall script - launching...");
            print("");

            // Execute the uninstall script directly (it will handle prompts)
            match std::process::Command::new("bash")
                .arg(&uninstall_script)
                .status()
            {
                Ok(status) => {
                    if status.success() {
                        print("✅ Uninstall completed via script");
                    } else {
                        print("⚠️  Uninstall script completed with warnings");
                    }
                }
                Err(e) => {
                    print(&format!("⚠️  Could not run uninstall script: {}", e));
                    print("Falling back to manual uninstall...");
                    run_manual_uninstall();
                }
            }
            return;
        }
    }

    // Fallback to manual uninstall if script not found
    print("🗑️  Uninstall script not found - using manual method");
    run_manual_uninstall();
}

/// Manual uninstall fallback
fn run_manual_uninstall() {
    print("🗑️  HookAnchor Uninstall");
    print("========================");
    print("");

    print("This will remove:");
    print("  - Karabiner configuration for HookAnchor");
    print("  - Configuration directory: ~/.config/hookanchor");
    print("  - URL handler registration");
    print("");

    print!("Are you sure you want to uninstall HookAnchor? (y/N): ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        print("");
        print("🗑️  Uninstalling HookAnchor...");

        // Remove configuration directory
        if let Ok(home) = std::env::var("HOME") {
            let config_dir = format!("{}/.config/hookanchor", home);
            if std::path::Path::new(&config_dir).exists() {
                match std::fs::remove_dir_all(&config_dir) {
                    Ok(()) => print("✅ Removed configuration directory"),
                    Err(e) => print(&format!("⚠️  Could not remove config directory: {}", e)),
                }
            }
        }

        // Remove Karabiner configuration
        let setup_assistant = crate::systems::setup_assistant::SetupAssistant::new();
        match setup_assistant.remove_karabiner_config() {
            Ok(()) => print("✅ Removed Karabiner configuration"),
            Err(e) => print(&format!("⚠️  Could not remove Karabiner configuration: {}", e)),
        }

        print("");
        print("✅ HookAnchor uninstalled successfully!");
        print("You may need to manually remove any remaining files:");
        print("  - App bundles in /Applications");
        print("  - Binary files if installed to system paths");
    } else {
        print("Uninstall cancelled.");
    }
}

/// Restart both command server and popup server - launch both from Terminal for proper permissions
fn run_restart_server() {
    print("🔄 Restarting both servers (command + popup)...");
    
    // First, hide the popup if it's visible by sending command to socket
    print("  Hiding popup window...");
    use std::os::unix::net::UnixStream;
    use std::io::Write;
    
    if let Ok(mut stream) = UnixStream::connect("/tmp/hookanchor_popup.sock") {
        if let Err(e) = stream.write_all(b"hide") {
            print(&format!("  ⚠️  Could not send hide command to popup: {}", e));
        } else {
            print("  ✅ Popup window hidden");
        }
    } else {
        print("  ⚠️  Popup server not running (window may already be hidden)");
    }
    
    // Kill any existing popup_server first
    print("  Killing existing popup_server...");
    use std::process::Command;
    match Command::new("pkill")
        .arg("-f")
        .arg("popup_server")
        .output() {
        Ok(_) => print("  ✅ Existing popup_server killed"),
        Err(e) => print(&format!("  ⚠️  Failed to kill popup_server: {}", e)),
    }

    // Clean up stale popup socket file
    let popup_socket_path = std::path::Path::new("/tmp/hookanchor_popup.sock");
    if popup_socket_path.exists() {
        if let Err(e) = std::fs::remove_file(popup_socket_path) {
            print(&format!("  ⚠️  Failed to remove popup socket file: {}", e));
        } else {
            print("  ✅ Cleaned up popup socket file");
        }
    }
    
    // Start popup_server (no Terminal tab needed - runs in background)
    print("  Starting popup_server...");
    match crate::systems::start_popup_server() {
        Ok(path) => {
            print(&format!("  ✅ popup_server started: {}", path.display()));
        }
        Err(e) => {
            print(&format!("  ⚠️  Failed to start popup_server: {}", e));
        }
    }
    
    // Restart the server (kill existing and start new)
    print("  Restarting server...");
    
    // Clear the socket file to ensure clean start
    let socket_path = std::path::Path::new("/Users/oblinger/.config/hookanchor/execution_server.sock");
    if socket_path.exists() {
        if let Err(e) = std::fs::remove_file(socket_path) {
            print(&format!("  ⚠️  Failed to remove socket file: {}", e));
        }
    }
    
    match crate::execute::activate_command_server(true) {
        Ok(()) => {
            print("  ✅ Server restart initiated via Terminal");
            print("  📱 A new Terminal window should open with the server daemon");
            print("  📄 Server output will be logged to ~/.config/hookanchor/server.log");
            
            // Wait for the server to start - check multiple times
            print("  ⏳ Waiting for server to start...");
            let mut server_started = false;
            let max_attempts = 50; // 50 attempts * 200ms = 10 seconds max
            let socket_path = std::path::Path::new("/Users/oblinger/.config/hookanchor/execution_server.sock");
            
            for attempt in 1..=max_attempts {
                std::thread::sleep(std::time::Duration::from_millis(200));
                
                // First check if socket file exists
                if socket_path.exists() {
                    // Give it a moment for the socket to be fully ready
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    
                    // Server socket exists, assume it's ready
                    server_started = true;
                    break;
                }
                
                // Show progress every second
                if attempt % 5 == 0 {
                    print!(".");
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                }
            }
            
            if server_started {
                print("\n  ✅ Server started successfully!");
                
                // Also verify by checking the PID in state
                let state = crate::core::data::get_state();
                if let Some(pid) = state.server_pid {
                    print(&format!("  📊 Server running with PID: {}", pid));
                }
                
                print("  🎯 Server is ready to accept commands");
            } else {
                print("\n  ❌ Server failed to start within 10 seconds");
                print("  💡 Check the Terminal window for error messages");
                std::process::exit(1);
            }
        }
        Err(e) => {
            print(&format!("  ❌ Failed to start server: {}", e));
            print("  💡 Try running manually: ha --start-server-daemon");
            std::process::exit(1);
        }
    }
}

/// Execute a launcher command - used by launchctl asuser to run commands in GUI session
fn run_execute_launcher_command(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} --execute-launcher-command <launcher_command>", args[0]));
        std::process::exit(1);
    }
    
    let launcher_command = &args[2];
    
    // Visual separator for launcher command execution
    utils::detailed_log("", "=================================================================");
    utils::detailed_log("LAUNCHER_CMD", &format!("Executing in GUI session: '{}'", launcher_command));
    
    // Execute the launcher command via server (consistent with all execution)
    // Parse the launcher command to create a Command object
    let parts: Vec<&str> = launcher_command.split_whitespace().collect();
    let (action, arg) = if parts.len() > 1 {
        (parts[0], parts[1..].join(" "))
    } else {
        (launcher_command.as_str(), String::new())
    };
    
    // Create action directly and execute
    let action_obj = make_action(action, &arg);
    let mut variables = HashMap::new();
    variables.insert("arg".to_string(), arg.clone());
    let _ = execute_on_server(&action_obj, Some(variables));
    utils::detailed_log("LAUNCHER_CMD", "Command completed");
}

/// Run rebuild command: restart server and rescan filesystem
fn run_rebuild_command() {
    // Clear log file before starting rebuild
    crate::utils::clear_debug_log();
    
    // Generate a unique build identifier (timestamp-based)
    let build_timestamp = chrono::Local::now();
    let build_id = build_timestamp.format("%Y%m%d_%H%M%S").to_string();
    
    // Log the rebuild header with timestamp and build ID
    crate::utils::log(&"=".repeat(80));
    crate::utils::detailed_log("SYSTEM", &format!("REBUILD SESSION: {}", build_id));
    crate::utils::log(&format!("TIMESTAMP: {}", build_timestamp.format("%Y-%m-%d %H:%M:%S%.3f")));
    crate::utils::log(&"=".repeat(80));
    crate::utils::detailed_log("REBUILD", &format!("Starting rebuild session {}", build_id));
    
    print("🏗️  HookAnchor Rebuild - Full Reset");
    print("===================================");
    
    // Step 1: Build the release binary
    print("\n🔨 Step 1/3: Building release binary...");
    let build_start = std::time::Instant::now();
    
    // Run cargo build --release
    let build_output = std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir("/Users/oblinger/ob/proj/HookAnchor")
        .output();
    
    match build_output {
        Ok(output) => {
            if output.status.success() {
                let build_time = build_start.elapsed();
                print(&format!("  ✅ Build successful ({:.1}s)", build_time.as_secs_f32()));
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                print("  ❌ Build failed:");
                print(&format!("{}", stderr));
                print("\n⚠️  Cannot proceed with rebuild due to compilation errors");
                std::process::exit(1);
            }
        }
        Err(e) => {
            print(&format!("  ❌ Failed to run cargo build: {}", e));
            print("\n⚠️  Make sure cargo is installed and in PATH");
            std::process::exit(1);
        }
    }
    
    print("\n🔄 Step 2/3: Restarting command server...");
    
    // Clear the socket file to ensure clean start
    let socket_path = std::path::Path::new("/Users/oblinger/.config/hookanchor/execution_server.sock");
    if socket_path.exists() {
        if let Err(e) = std::fs::remove_file(socket_path) {
            print(&format!("  ⚠️  Failed to remove socket file: {}", e));
        }
    }
    
    // Restart the server
    match crate::execute::activate_command_server(true) {
        Ok(()) => {
            print("  ✅ Server restart initiated");
            print("  📱 A new Terminal window should open with the server daemon");
        },
        Err(e) => {
            print(&format!("  ❌ Failed to restart server: {}", e));
            return;
        }
    }
    
    print("\n📁 Step 3/3: Rescanning filesystem...");
    
    // Run filesystem rescan
    run_rescan_command();
    
    print("\n🎉 Rebuild complete!");
}

/// Launch history viewer
fn run_search_command() {
    use std::process::Command;

    utils::detailed_log("SEARCH_CMD", "Search command requested - launching history viewer");

    // Find history viewer binary
    let exe_dir = utils::get_binary_dir();
    let viewer_path = exe_dir.join("HookAnchorHistoryViewer");

    if !viewer_path.exists() {
        utils::log_error(&format!("History viewer not found at: {:?}", viewer_path));
        print(&format!("History viewer not found at: {:?}", viewer_path));
        std::process::exit(1);
    }

    utils::detailed_log("SEARCH_CMD", &format!("Launching history viewer: {:?}", viewer_path));

    // Launch history viewer as a background process
    match Command::new(&viewer_path)
        .spawn() {
        Ok(_) => {
            utils::detailed_log("SEARCH_CMD", "History viewer launched successfully");
            print("History viewer launched");
        }
        Err(e) => {
            utils::log_error(&format!("Failed to launch history viewer: {}", e));
            print(&format!("Failed to launch history viewer: {}", e));
            std::process::exit(1);
        }
    }
}

/// Delete history database and command cache
///
/// This CLI command performs a complete reset by calling `delete_history()` from the
/// data layer, which executes the following steps:
///
/// 1. **Delete history database** (~/.config/hookanchor/history.db)
///    - Removes all command execution history
///    - Removes all file modification tracking records
///
/// 2. **Delete command cache** (~/.config/hookanchor/commands_cache.json)
///    - Forces full filesystem rescan on next startup
///    - Ensures cache and history stay in sync (no orphaned cache entries)
///
/// After deletion, this command performs a two-step rebuild:
/// - Step 1: Scan filesystem with empty commands to record file creation timestamps
/// - Step 2: Run full rescan to merge manual edits from commands.txt
///
/// This operation is irreversible. Historical execution data is lost forever, but the
/// next rescan will rebuild the cache from scratch by scanning all file_roots.
fn run_delete_history(args: &[String]) {
    use std::io::{self, Write};

    // Check for --force flag
    let force = args.iter().any(|arg| arg == "--force");

    if !force {
        print("");
        print("⚠️  WARNING: This will permanently delete:");
        print("  • Command history database (~/.config/hookanchor/history.db)");
        print("  • Command cache file (~/.config/hookanchor/commands_cache.json)");
        print("");
        print("This action cannot be undone!");
        print("");
        print("Type 'yes' to confirm deletion: ");

        // Flush stdout to ensure prompt appears
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input != "yes" {
            print("Deletion cancelled.");
            return;
        }
    }

    // Delete history and cache through data layer
    match crate::core::data::delete_history() {
        Ok((history_deleted, cache_deleted)) => {
            print("");
            if history_deleted {
                print("✓ Deleted history database");
            } else {
                print("  History database not found (already deleted)");
            }
            if cache_deleted {
                print("✓ Deleted command cache");
            } else {
                print("  Command cache not found (already deleted)");
            }

            let deleted_count = [history_deleted, cache_deleted].iter().filter(|&&x| x).count();

            // Report results
            print("");
            if deleted_count > 0 {
                print(&format!("✓ Successfully deleted {} file(s)", deleted_count));
                print("");
                print("📝 Rebuilding history from scratch with accurate file creation dates...");
                print("");

                // CRITICAL ORDER: Scan filesystem FIRST with empty commands, THEN load commands.txt
                //
                // Step 1: Pure filesystem scan with EMPTY commands
                // - Discovers all files and records "created" history entries with birth timestamps
                // - Files are NOT in commands list yet, so scanner treats them as NEW
                // - Saves to cache with metadata
                print("🔍 Step 1: Scanning filesystem to record file creation history...");
                let (global_data, _) = crate::core::data::get_sys_data();
                let scanned_commands = crate::systems::scan_new_files(Vec::new(), &global_data, true);

                // Save to cache so next step loads these as existing files
                if let Err(e) = crate::core::commands::save_commands_to_cache(&scanned_commands) {
                    print(&format!("   ⚠️  Failed to save initial cache: {}", e));
                } else {
                    print(&format!("   ✅ Recorded creation history for {} files", scanned_commands.len()));
                }

                print("");
                print("✏️  Step 2: Merging manual edits from commands.txt...");
                // Step 2: Now run full rescan to merge commands.txt edits
                // - Cache now has all files with metadata
                // - commands.txt edits are applied on top
                // - No duplicate "created" entries because files already exist in cache
                run_rescan_command();
            }
        }
        Err(e) => {
            print("");
            print(&format!("✗ Error: {}", e));
            std::process::exit(1);
        }
    }
}

fn print_help_vars() {
    print("");
    print("TEMPLATE VARIABLES");
    print("");
    print("These variables are available in two contexts:");
    print("  1. In config.yaml templates using {{variable}} syntax");
    print("  2. In config.js action functions via the ctx object");
    print("");
    print("Command objects with fields (.name .path .arg .patch .folder* .action .flags):");
    print("  selected        Currently selected command in popup");
    print("  last_executed   Last command executed (any type)");
    print("  last_anchor     Last anchor command executed");
    print("");
    print("Date object with fields (.year .year2 .month .month_short .month_name .month_abbr");
    print("                         .day .day_short .weekday .weekday_abbr .hour .hour12");
    print("                         .minute .second .ampm .timestamp .iso):");
    print("  date            Current date/time components");
    print("");
    print("Grab object with fields (.action .arg .app .title .text .suffix):");
    print("  grabbed         Context from active application");
    print("");
    print("Environment object with fields (.home .user .hostname .os .config_dir):");
    print("  env             System environment information");
    print("");
    print("Simple variables:");
    print("  input           User's search input text");
    print("  raw_input       Raw input without anchor fallback");
    print("  arg             Action argument for template expansion");
    print("");
    print("* .folder throws error if empty - use only for commands with file context");
    print("");
    print("YAML Template Examples (in config.yaml):");
    print("  arg: \"{{selected.name}}\"           Command name of selected item");
    print("  arg: \"{{last_executed.folder}}\"    Folder of last command");
    print("  name: \"{{input}}\"                  User's search text");
    print("  arg: \"{{date.year}}-{{date.month}}-{{date.day}}\"  Current date");
    print("");
    print("JavaScript Examples (in config.js action functions):");
    print("  const name = ctx.selected.name;");
    print("  const folder = ctx.last_executed.folder;");
    print("  const input = ctx.input;");
    print("  const year = ctx.date.year;");
    print("");
}

fn print_help_config() {
    print("");
    print("CONFIGURATION STRUCTURE (~/.config/hookanchor/config.yaml)");
    print("");
    print("popup_settings:");
    print("  # Display & Layout");
    print("  max_rows:               20            # Max rows in popup grid");
    print("  max_columns:            3             # Max columns in popup grid");
    print("  max_characters:         30            # Max length for command names");
    print("  max_window_size:        \"1700x1100\"   # Maximum window size (widthxheight)");
    print("  default_window_size:    \"600x400\"     # Default popup size (widthxheight)");
    print("");
    print("  # Logging & Debug");
    print("  verbose_logging:        false         # Enable detailed logging");
    print("  max_log_file_size:      1000000       # Max log size in bytes (1MB default)");
    print("");
    print("  # Behavior");
    print("  run_in_background:      false         # Keep app running for instant popup");
    print("  merge_similar:          true          # Merge commands ending with \"...\"");
    print("  word_separators:        \" ._-\"        # Characters for word boundaries");
    print("  listed_actions:         \"app,url,folder,cmd,chrome\"  # Editor dropdown");
    print("  preferred_anchor:       \"markdown\"    # Preferred anchor type when multiple exist");
    print("");
    print("  # Timeouts");
    print("  scan_interval_seconds:  600           # Seconds between file scans");
    print("  idle_timeout_seconds:   60            # Seconds before auto-close");
    print("  countdown_seconds:      5             # Grabber countdown duration");
    print("  anchor_timeout_seconds: 180           # Seconds to remember last anchor");
    print("");
    print("  # File Scanning");
    print("  file_roots:             [\"~/path\"]    # Directories to scan");
    print("  doc_file_extensions:    \"pdf,docx\"    # Extensions for DOC commands");
    print("  skip_directory_patterns:              # Patterns to exclude from scan");
    print("    - \"/pattern/\"");
    print("");
    print("  # Rename Behavior");
    print("  rename_doc:             false         # Rename file when renaming DOC command");
    print("  rename_folder:          false         # Rename folder when renaming anchor");
    print("  rename_patch:           false         # Update all commands with renamed patch");
    print("  rename_prefix:          false         # Update commands with renamed prefix");
    print("");
    print("launcher_settings:");
    print("  js_timeout_ms:          5000          # Max ms for JS action execution");
    print("  obsidian_app_name:      \"Obsidian\"    # Obsidian application name");
    print("  obsidian_vault_name:    \"kmr\"         # Obsidian vault name");
    print("  obsidian_vault_path:    \"~/Documents\" # Path to Obsidian vault");
    print("  flip_focus:             false         # Flip focus during grabber countdown");
    print("  use_javascript_tmux_activation: null  # Use JS for tmux activation");
    print("  tmux_startup_command:   null          # Command to run in tmux after activation");
    print("");
    print("history_viewer:");
    print("  viewable_history_limit: 50000         # Max history entries to load");
    print("  tree_sidebar_width:     250           # Tree sidebar width in pixels");
    print("  tree_sidebar_min_width: 50            # Minimum sidebar width in pixels");
    print("  tree_indent_pixels:     10            # Tree indent per level");
    print("  tree_show_guides:       true          # Show tree indent guide lines");
    print("  peek_on_hover:          true          # Show history on tree item hover");
    print("  key_bindings:");
    print("    edit_selection:       \";\"           # Key to edit selected history item");
    print("");
    print("actions:");
    print("  action_name:                          # Custom action name");
    print("    description:          \"...\"         # What this action does");
    print("    action_type:          template      # Type: template, js, or command");
    print("    key:                  \"+\"           # Optional keyboard binding");
    print("    name:                 \"{{input}}\"   # Template for command name");
    print("    action:               \"markdown\"    # Command action type");
    print("    arg:                  \"{{...}}\"     # Template for command argument");
    print("    patch:                \"{{...}}\"     # Template for patch");
    print("    edit:                 true          # Open editor after creation");
    print("    use_existing:         false         # Use existing command if name matches");
    print("    file:                 \"{{...}}\"     # Optional file/folder to create");
    print("    contents:             \"{{...}}\"     # Optional file contents");
    print("    grab:                 5             # Seconds to wait before grabbing");
    print("    validate_previous_folder: false     # Validate previous command has folder");
    print("    file_rescan:          false         # Rescan filesystem after creation");
    print("");
    print("grabber_rules:");
    print("  - app_name:             \"AppName\"     # Application to match");
    print("    action:               \"url\"         # Action type for grabbed content");
    print("    title_contains:       \"pattern\"     # Match window title");
    print("");
    print("grabber_suffix_map:");
    print("  \"@github\":             \"github\\.com\" # Map regex patterns to suffixes");
    print("  \"@docs\":               \"docs\\.google\\.com\"");
    print("");
}

fn print_help_fns() {
    print("");
    print("FUNCTIONS AVAILABLE FOR \"config.js\" EXTENSIONS");
    print("");
    print("Logging:");
    print("  log(msg)                       Write to log file only");
    print("  print(msg)                     Print to console AND log to file");
    print("  detailedLog(category, msg)     Write msg when verbose_logging is true");
    print("  error(msg)                     Queue error for user display");
    print("");
    print("File System:");
    print("  readFile(path)                 Read text file contents");
    print("  writeFile(path, text)          Write text to file");
    print("  fileExists(path)               Check if file exists (boolean)");
    print("  isDirectory(path)              Check if path is directory (boolean)");
    print("  listFiles(dir, pattern)        List files matching glob pattern");
    print("");
    print("Path Utilities:");
    print("  joinPath(part1, part2)         Join two path components");
    print("  dirname(path)                  Get directory name from path");
    print("  basename(path)                 Get filename from path");
    print("  expandHome(path)               Expand ~ to home directory");
    print("  getExtension(path)             Get file extension");
    print("");
    print("Text and Data Processing:");
    print("  testRegex(text, pattern)       Test if text matches regex (boolean)");
    print("  parseYaml(text)                Parse YAML into JavaScript object");
    print("");
    print("Application Control:");
    print("  launchApp(app, arg?)           Launch app with optional argument");
    print("  openFolder(path)               Open folder in Finder");
    print("  openUrl(url, browser?)         Open URL (optional browser name)");
    print("  activateApp(app_name)          Bring application to foreground");
    print("  appIsRunning(app_name)         Check if app is running (boolean)");
    print("");
    print("Shell Commands:");
    print("  shell(command)                 Execute in background (non-blocking)");
    print("  shellSync(command)             Execute and wait (returns output)");
    print("  shellWithExitCode(cmd)         Returns {stdout, stderr, exitCode}");
    print("  commandExists(command)         Check if command in PATH (boolean)");
    print("  changeDirectory(path)          Change working directory");
    print("  spawnDetached(command)         Spawn detached background process");
    print("  spawnDetachedWithArgs(args)    Spawn with argument array");
    print("");
    print("Other Commands:");
    print("  runAppleScript(script)         Execute AppleScript, return result");
    print("  launch(command_name)           Execute another HookAnchor command");
    print("  saveAnchor(name, folder?)      Save anchor for next popup open");
    print("  getConfigString(path)          Get config value by dot-separated path");
    print("                                 Example: getConfigString(\"launcher_settings.obsidian_vault_path\")");
    print("");
    print("===============================================================================");
    print("CONTEXT OBJECT (for action functions in config.js)");
    print("===============================================================================");
    print("");
    print("  ctx.arg                   Action argument (file path, URL, etc.)");
    print("  ctx.input                 User's search input text");
    print("  ctx.previous.name         Previously executed command name");
    print("  ctx.previous.folder       Previously executed command folder");
    print("  ctx.previous.patch        Previously executed command patch");
    print("  ctx.grabbed.action        Grabbed content action type");
    print("  ctx.grabbed.arg           Grabbed content argument");
    print("  ctx.date.YYYY             Current year (4 digits)");
    print("  ctx.date.MM               Current month (2 digits)");
    print("  ctx.date.DD               Current day (2 digits)");
    print("  ctx.builtins              Object with all built-in functions");
    print("");
    print("Example usage in config.js:");
    print("  action_folder: function(ctx) {");
    print("    const { log, openFolder } = ctx.builtins;");
    print("    openFolder(ctx.arg);");
    print("  }");
    print("");
}

/// Comparison result between two command sets
#[derive(Debug)]
struct ComparisonResult {
    unchanged: Vec<String>,
    changed: Vec<CommandChange>,
    removed: Vec<Command>,
    added: Vec<Command>,
}

/// A single command change showing old and new values
#[derive(Debug)]
struct CommandChange {
    command_name: String,
    old: Command,
    new: Command,
}

/// Compare two command vectors and return detailed comparison
fn compare_commands(old_commands: &[Command], new_commands: &[Command]) -> ComparisonResult {
    use std::collections::HashMap;

    // Build hash maps for quick lookup using the same deduplication key
    // This ensures we're comparing commands the same way the system deduplicates them
    let old_map: HashMap<String, &Command> = old_commands.iter()
        .map(|cmd| (crate::core::data::command_dedup_key(cmd), cmd))
        .collect();

    let new_map: HashMap<String, &Command> = new_commands.iter()
        .map(|cmd| (crate::core::data::command_dedup_key(cmd), cmd))
        .collect();

    let mut unchanged = Vec::new();
    let mut changed = Vec::new();
    let mut removed = Vec::new();
    let mut added = Vec::new();

    // Check old commands
    for (key, old_cmd) in &old_map {
        if let Some(new_cmd) = new_map.get(key) {
            // Command exists in both - check if changed
            if commands_equal(old_cmd, new_cmd) {
                unchanged.push(old_cmd.command.clone());
            } else {
                changed.push(CommandChange {
                    command_name: old_cmd.command.clone(),
                    old: (*old_cmd).clone(),
                    new: (*new_cmd).clone(),
                });
            }
        } else {
            // Command was removed
            removed.push((*old_cmd).clone());
        }
    }

    // Check for added commands
    for (key, new_cmd) in &new_map {
        if !old_map.contains_key(key) {
            added.push((*new_cmd).clone());
        }
    }

    ComparisonResult {
        unchanged,
        changed,
        removed,
        added,
    }
}

/// Check if two commands are equal (comparing all relevant fields)
fn commands_equal(cmd1: &Command, cmd2: &Command) -> bool {
    cmd1.command == cmd2.command &&
    cmd1.action == cmd2.action &&
    cmd1.arg == cmd2.arg &&
    cmd1.flags == cmd2.flags &&
    cmd1.patch == cmd2.patch
}

/// Print comparison results to console and log
fn print_comparison(title: &str, result: &ComparisonResult) {
    print("");
    print(&format!("=== {} ===", title));
    print("");

    print(&format!("📊 Summary:"));
    print(&format!("  Unchanged: {}", result.unchanged.len()));
    print(&format!("  Changed:   {}", result.changed.len()));
    print(&format!("  Removed:   {}", result.removed.len()));
    print(&format!("  Added:     {}", result.added.len()));
    print("");

    if !result.changed.is_empty() {
        print(&format!("🔄 Changed Commands ({}):", result.changed.len()));
        for change in &result.changed {
            print(&format!("  {}", change.command_name));

            if change.old.action != change.new.action {
                print(&format!("    action:  '{}' → '{}'", change.old.action, change.new.action));
            }
            if change.old.arg != change.new.arg {
                print(&format!("    arg:     '{}' → '{}'", change.old.arg, change.new.arg));
            }
            if change.old.flags != change.new.flags {
                print(&format!("    flags:   '{}' → '{}'", change.old.flags, change.new.flags));
            }
            if change.old.patch != change.new.patch {
                print(&format!("    patch:   '{}' → '{}'", change.old.patch, change.new.patch));
            }
        }
        print("");
    }

    if !result.removed.is_empty() {
        print(&format!("➖ Removed Commands ({}):", result.removed.len()));
        for cmd in result.removed.iter().take(20) {
            print(&format!("  {} ({}:{})", cmd.command, cmd.patch, cmd.action));
        }
        if result.removed.len() > 20 {
            print(&format!("  ... and {} more", result.removed.len() - 20));
        }
        print("");
    }

    if !result.added.is_empty() {
        print(&format!("➕ Added Commands ({}):", result.added.len()));
        for cmd in result.added.iter().take(20) {
            print(&format!("  {} ({}:{})", cmd.command, cmd.patch, cmd.action));
        }
        if result.added.len() > 20 {
            print(&format!("  ... and {} more", result.added.len() - 20));
        }
        print("");
    }
}

/// Load legacy commands and compare with current state
fn run_load_legacy_and_compare(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} --load-legacy-and-compare <path>", args[0]));
        print("Example: ha --load-legacy-and-compare ~/.config/hookanchor/commands_20251017_232108.txt");
        std::process::exit(1);
    }

    let legacy_path = &args[2];
    let legacy_path_expanded = crate::utils::expand_tilde(legacy_path);

    print("🔍 Legacy Commands Load and Compare");
    print("===================================");
    print("");
    print(&format!("📁 Legacy file: {}", legacy_path_expanded));
    print("");

    // Step 1: Rescan
    print("🔄 Step 1: Performing initial rescan...");
    run_rescan_command();
    print("✅ Initial rescan complete");
    print("");

    // Step 2: Load legacy commands
    print("📥 Step 2: Loading legacy commands from file...");
    let legacy_commands = load_commands_from_file(&legacy_path_expanded);
    print(&format!("✅ Loaded {} legacy commands", legacy_commands.len()));
    print("");

    // Step 3: Save legacy commands (which triggers flush)
    print("💾 Step 3: Saving legacy commands (triggers flush)...");
    let commands_before_save = legacy_commands.clone();
    match crate::core::set_commands(legacy_commands) {
        Ok(()) => {
            print("✅ Legacy commands saved and flushed");
        }
        Err(e) => {
            print(&format!("❌ Failed to save commands: {}", e));
            std::process::exit(1);
        }
    }
    print("");

    // Step 4: First comparison - check if save/flush corrupted data
    print("🔬 Step 4: First comparison (legacy vs. saved)...");
    let commands_after_save = crate::core::get_commands();
    let comparison1 = compare_commands(&commands_before_save, &commands_after_save);
    print_comparison("POST-SAVE COMPARISON", &comparison1);

    // Step 5: Rescan again
    print("🔄 Step 5: Performing second rescan...");
    run_rescan_command();
    print("✅ Second rescan complete");
    print("");

    // Step 6: Second comparison - check if rescan corrupted data
    print("🔬 Step 6: Second comparison (saved vs. rescanned)...");
    let commands_after_rescan = crate::core::get_commands();
    let comparison2 = compare_commands(&commands_after_save, &commands_after_rescan);
    print_comparison("POST-RESCAN COMPARISON", &comparison2);

    print("✅ Legacy load and compare complete!");
}

/// Load commands from a specific file path
fn load_commands_from_file(path: &str) -> Vec<Command> {
    use std::fs;

    if !std::path::Path::new(path).exists() {
        print(&format!("❌ File not found: {}", path));
        std::process::exit(1);
    }

    match fs::read_to_string(path) {
        Ok(contents) => {
            let mut commands = Vec::new();
            for (line_num, line) in contents.lines().enumerate() {
                let trimmed = line.trim();

                // Skip empty lines and comments
                if trimmed.is_empty() || trimmed.starts_with("//") {
                    continue;
                }

                match crate::core::commands::parse_command_line(line) {
                    Ok(command) => {
                        commands.push(command);
                    },
                    Err(e) => {
                        crate::utils::log_error(&format!("Failed to parse line {} in {}: {} - Line: '{}'",
                            line_num + 1, path, e, line));
                    }
                }
            }
            commands
        }
        Err(e) => {
            print(&format!("❌ Failed to read file: {}", e));
            std::process::exit(1);
        }
    }
}