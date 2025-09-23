use std::collections::HashMap;
use crate::core::{load_commands_with_data, load_commands_for_inference, filter_commands, run_patch_inference};
use crate::core::commands::save_commands_to_file;
use crate::utils;
use crate::utils::logging::print;
use crate::execute::{execute_on_server, make_action, command_to_action, execute_on_server_with_parameters};

/// Main entry point for command-line mode
pub fn run_command_line_mode(args: Vec<String>) {
    // Check for hook:// URL as first argument
    if args.len() >= 2 && args[1].starts_with("hook://") {
        handle_hook_url(&args[1]);
        return;
    }
    
    // Check for help flags
    if args.len() >= 2 && (args[1] == "-h" || args[1] == "--help") {
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
        "-f" | "--folders" => run_folder_command(&args),
        "-F" | "--named-folders" => run_folder_with_commands(&args),
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
        "--popup" => run_popup_command(&args),
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
    print("Usage:");
    print(&format!("  {}                          # Run GUI mode", program_name));
    print(&format!("  {} -h, --help               # Show help", program_name));
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
    print(&format!("  {} --popup [show|hide|delete|status] # Manage popup window", program_name));
    print(&format!("  {} --hook <url>             # Handle hook:// URL (for URL handler)", program_name));
    print("  open 'hook://query'         # Handle hook URL via URL handler");
    print("");
    print("Examples:");
    print(&format!("  {}           # Launch GUI", program_name));
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
    let (sys_data, _) = crate::core::sys_data::get_sys_data();
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
    let _ = execute_on_server(&action);
    utils::detailed_log("URL_HANDLER", "Command executed");
}

fn run_match_command(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} -m, --match <query> [debug]", args[0]));
        std::process::exit(1);
    }
    
    let query = &args[2];
    let debug = args.len() > 3 && args[3] == "debug";
    
    let (sys_data, _) = crate::core::sys_data::get_sys_data();
    let filtered = if debug {
        filter_commands(&sys_data.commands, query, 10, debug)  // Keep debug mode using original function
    } else {
        // Use new display logic that handles prefix menu filtering correctly
        let (display_commands, _is_prefix_menu, _prefix_menu_info, _prefix_menu_count) =
            crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches);
        display_commands
    };

    // Print first 50 matches to see prefix menu results
    for cmd in filtered.iter().take(50) {
        print(&format!("{}", cmd.command));
    }
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
    let (sys_data, _) = crate::core::sys_data::get_sys_data();
    
    // Find the command by name (case-insensitive)
    let matching_cmd = sys_data.commands.iter()
        .find(|cmd| cmd.command.to_lowercase() == command.to_lowercase());
    
    if let Some(cmd) = matching_cmd {
        // Execute the actual Command object
        // Convert command to action and execute
        let action = command_to_action(&cmd);
        let _ = execute_on_server(&action);
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
    
    let (sys_data, _) = crate::core::sys_data::get_sys_data();

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
    use crate::core::state::save_last_executed_command;
    crate::utils::detailed_log("STATE_SAVE", &format!("CLI: Attempting to save last executed command: '{}'", original_name));
    match save_last_executed_command(&original_name) {
        Ok(_) => crate::utils::detailed_log("STATE_SAVE", "CLI: Successfully saved last executed command"),
        Err(e) => crate::utils::detailed_log("STATE_SAVE", &format!("CLI: Failed to save last executed command: {}", e)),
    }

    
    // Use server-based execution for consistent environment
    // Convert command to action and execute
    let action = command_to_action(&top_command_obj);
    let _ = execute_on_server(&action);
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
    let config = crate::core::sys_data::get_config();
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
            
            // Execute the action with simplified parameters
            let arg = if !arg_value.is_empty() { Some(arg_value.as_str()) } else { None };
            match execute_on_server_with_parameters(action, arg, Some(variables)) {
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
    let _ = execute_on_server(&action);
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
    
    // Execute through server
    use crate::execute::execute_on_server;
    let _ = execute_on_server(&action);
    print(&format!("Action '{}' completed", action_type));
}

fn run_folder_command(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} -f, --folders <query>", args[0]));
        std::process::exit(1);
    }
    
    let query = &args[2];
    let (sys_data, _) = crate::core::sys_data::get_sys_data();
    
    // Helper function to extract folder path from a command
    let extract_folder_path = |command: &crate::core::Command| -> Option<String> {
        match command.action.as_str() {
            "folder" => {
                // For folder commands, the arg is already the folder path
                Some(command.arg.clone())
            },
            "anchor" => {
                // For anchor commands, the arg is the full path to the .md file
                // Return the directory containing the .md file
                if let Some(last_slash) = command.arg.rfind('/') {
                    Some(command.arg[..last_slash].to_string())
                } else {
                    Some(command.arg.clone())
                }
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
    };
    
    // Use the same display logic as the popup, with aliases expanded
    let (mut display_commands, _, _, _) = crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches);
    // Expand aliases for display
    display_commands = display_commands.into_iter().map(|cmd| {
        if cmd.action == "alias" {
            cmd.resolve_alias(&sys_data.commands)
        } else {
            cmd
        }
    }).take(100).collect();
    
    if display_commands.is_empty() {
        print(&format!("No commands found matching: {}", query));
        std::process::exit(1);
    }
    
    // Check for exact match first (if first command is exact match, return just that)
    if !display_commands.is_empty() && !crate::ui::PopupState::is_separator_command(&display_commands[0]) {
        let first_cmd = &display_commands[0];
        let query_lower = query.to_lowercase();
        let query_folder_pattern = format!("{} folder", query_lower);
        let cmd_lower = first_cmd.command.to_lowercase();
        
        if cmd_lower == query_lower || cmd_lower == query_folder_pattern {
            // Found exact match, return just that folder
            if let Some(folder_path) = extract_folder_path(first_cmd) {
                print(&format!("{}", folder_path));
                return;
            }
        }
    }
    
    // Collect unique folder paths from all matching commands (skip separators)
    let mut folder_paths = std::collections::HashSet::new();
    
    for command in &display_commands {
        // Skip separator commands
        if crate::ui::PopupState::is_separator_command(command) {
            continue;
        }
        
        // Add to set if we got a valid folder path
        if let Some(path) = extract_folder_path(command) {
            folder_paths.insert(path);
        }
    }
    
    // Output each unique folder path on its own line
    let mut paths: Vec<String> = folder_paths.into_iter().collect();
    paths.sort(); // Sort for consistent output
    
    for path in paths {
        print(&format!("{}", path));
    }
}

fn run_folder_with_commands(args: &[String]) {
    if args.len() < 3 {
        print(&format!("Usage: {} -F, --named-folders <query>", args[0]));
        std::process::exit(1);
    }
    
    let query = &args[2];
    let (sys_data, _) = crate::core::sys_data::get_sys_data();
    
    // Helper function to extract folder path from a command
    let extract_folder_path = |command: &crate::core::Command| -> Option<String> {
        match command.action.as_str() {
            "folder" => {
                // For folder commands, the arg is already the folder path
                Some(command.arg.clone())
            },
            "anchor" => {
                // For anchor commands, the arg is the full path to the .md file
                // Return the directory containing the .md file
                if let Some(last_slash) = command.arg.rfind('/') {
                    Some(command.arg[..last_slash].to_string())
                } else {
                    Some(command.arg.clone())
                }
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
    };
    
    // Use the same display logic as the popup, with aliases expanded
    let (mut display_commands, _, _, _) = crate::core::get_new_display_commands(query, &sys_data.commands, &sys_data.patches);
    // Expand aliases for display
    display_commands = display_commands.into_iter().map(|cmd| {
        if cmd.action == "alias" {
            cmd.resolve_alias(&sys_data.commands)
        } else {
            cmd
        }
    }).take(100).collect();
    
    if display_commands.is_empty() {
        print(&format!("No commands found matching: {}", query));
        std::process::exit(1);
    }
    
    // Check for exact match first (if first command is exact match, return just that)
    if !display_commands.is_empty() && !crate::ui::PopupState::is_separator_command(&display_commands[0]) {
        let first_cmd = &display_commands[0];
        let query_lower = query.to_lowercase();
        let query_folder_pattern = format!("{} folder", query_lower);
        let cmd_lower = first_cmd.command.to_lowercase();
        
        if cmd_lower == query_lower || cmd_lower == query_folder_pattern {
            // Found exact match, return just that command -> path
            if let Some(folder_path) = extract_folder_path(first_cmd) {
                print(&format!("{} -> {}", first_cmd.command, folder_path));
                return;
            }
        }
    }
    
    // Output in "command -> path" format for fuzzy finder (skip separators)
    for command in &display_commands {
        // Skip separator commands
        if crate::ui::PopupState::is_separator_command(command) {
            continue;
        }
        
        if let Some(path) = extract_folder_path(command) {
                print(&format!("{} -> {}", command.command, path));
        }
    }
}

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
    let config = crate::core::sys_data::get_config();
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
    let config = crate::core::sys_data::get_config();
    
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
    // Initialize config FIRST - this must happen before any other operations
    if let Err(config_error) = crate::core::sys_data::initialize_config() {
        utils::log_error(&format!("Failed to load config: {}", config_error));
        // Continue with default config
    }
    
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

/// Checks if the new_patch is associated with a parent directory of the current_patch
#[allow(dead_code)]
fn is_parent_directory_patch(current_patch: &str, new_patch: &str, patches: &std::collections::HashMap<String, crate::core::Patch>) -> bool {
    use std::path::Path;
    
    // Get the linked commands for both patches
    let current_linked = crate::core::commands::get_patch(current_patch, patches).and_then(|p| p.primary_anchor());
    let new_linked = crate::core::commands::get_patch(new_patch, patches).and_then(|p| p.primary_anchor());
    
    if let (Some(current_cmd), Some(new_cmd)) = (current_linked, new_linked) {
        // Both patches have linked commands - compare their directory paths
        if current_cmd.is_path_based() && new_cmd.is_path_based() {
            let current_path = Path::new(&current_cmd.arg);
            let new_path = Path::new(&new_cmd.arg);
            
            // Get directories containing the files
            let current_dir = if current_path.is_file() || current_cmd.arg.contains('.') {
                current_path.parent()
            } else {
                Some(current_path)
            };
            
            let new_dir = if new_path.is_file() || new_cmd.arg.contains('.') {
                new_path.parent()
            } else {
                Some(new_path)
            };
            
            if let (Some(current_dir), Some(new_dir)) = (current_dir, new_dir) {
                // Check if new_dir is a parent of current_dir
                return current_dir.starts_with(new_dir) && current_dir != new_dir;
            }
        }
    }
    
    false
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
    // Load current commands (without inference and virtual anchor creation)
    let (_config, commands, patches) = load_commands_with_data();
    
    // Find the command by name
    let found_command = commands.iter().find(|cmd| cmd.command == command_name);
    
    match found_command {
        Some(command) => {
            print(&format!("Command: {}", command.command));
            print(&format!("Current patch: '{}'", command.patch));
            
            // Test patch inference on this specific command
            match crate::core::commands::infer_patch(command, &patches) {
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
            let similar_commands: Vec<&crate::core::Command> = commands.iter()
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
        // Reload commands and apply changes
        let (_config, mut commands, patches) = load_commands_with_data();
        
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
            match save_commands_to_file(&commands) {
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
    let config = crate::core::sys_data::get_config();
    
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
    
    print("\n📋 Initial data load...");
    
    // Load existing commands from disk first (to preserve patches), then run verbose load
    let existing_commands = crate::core::load_commands();
    let global_data = crate::core::sys_data::load_data(existing_commands, true);
    
    print("\n🔍 Starting filesystem scan...");
    
    // Run scan with verbose output
    let scanned_commands = crate::systems::scan_verbose(
        global_data.commands.clone(),
        &global_data,
        true
    );
    
    print("\n📊 Final Summary:");
    print(&format!("   Total commands after rescan: {}", scanned_commands.len()));
    
    // Count commands by action type
    let mut action_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for cmd in &scanned_commands {
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
    match std::env::current_exe() {
        Ok(exe_path) => {
            // Resolve symlinks to get the actual binary location
            let resolved_exe = std::fs::canonicalize(&exe_path).unwrap_or(exe_path);
            let exe_dir = resolved_exe.parent().unwrap_or(std::path::Path::new("."));
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
        Err(e) => {
            print(&format!("❌ Could not determine executable path: {}", e));
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
    
    // Start popup_server via Terminal for proper permissions
    print("  Starting popup_server via Terminal...");
    let binary_path = crate::utils::get_binary_path()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("/Users/oblinger/ob/proj/HookAnchor/target/release"));
    let popup_server_path = binary_path.join("popup_server");
    
    if popup_server_path.exists() {
        let escaped_path = popup_server_path.display().to_string().replace("\"", "\\\"");
        let script = format!(
            r#"tell application "Terminal" to do script "cd ~ && \"{}\" 2>&1""#,
            escaped_path
        );
        
        match Command::new("osascript")
            .args(["-e", &script])
            .output() {
            Ok(output) => {
                if output.status.success() {
                    print("  ✅ popup_server started via Terminal (inherits Accessibility permissions)");
                } else {
                    print(&format!("  ⚠️  Failed to start popup_server via Terminal: {}", 
                            String::from_utf8_lossy(&output.stderr)));
                }
            }
            Err(e) => print(&format!("  ⚠️  Failed to execute AppleScript: {}", e)),
        }
    } else {
        print(&format!("  ⚠️  popup_server not found at: {}", popup_server_path.display()));
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
                let state = crate::core::state::load_state();
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
    let _ = execute_on_server(&action_obj);
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

/// Run popup management command
fn run_popup_command(args: &[String]) {
    use std::process::Command;
    
    // Determine the popup action
    let action = if args.len() >= 3 {
        args[2].as_str()
    } else {
        "show"  // Default action
    };
    
    utils::detailed_log("POPUP_CMD", &format!("Popup action requested: {}", action));
    
    // Validate action
    match action {
        "show" | "hide" | "delete" | "status" => {},
        _ => {
            utils::log_error(&format!("Unknown popup action: {}", action));
            print(&format!("Unknown popup action: {}", action));
            print("Valid actions: show, hide, delete, status");
            std::process::exit(1);
        }
    }
    
    // Find popup launcher binary
    let exe_path = std::env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
    let popup_path = exe_dir.join("popup");
    
    if !popup_path.exists() {
        utils::log_error(&format!("Popup launcher not found at: {:?}", popup_path));
        print(&format!("Popup launcher not found at: {:?}", popup_path));
        std::process::exit(1);
    }
    
    utils::detailed_log("POPUP_CMD", &format!("Executing popup launcher: {:?} {}", popup_path, action));
    
    // Execute popup launcher with action
    match Command::new(&popup_path)
        .arg(action)
        .output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                utils::detailed_log("POPUP_CMD", &format!("Popup {} succeeded: {}", action, stdout.trim()));
                print!("{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                utils::log_error(&format!("Popup {} failed: {}", action, stderr.trim()));
                eprint!("{}", stderr);
                std::process::exit(1);
            }
        }
        Err(e) => {
            utils::log_error(&format!("Failed to execute popup launcher: {}", e));
            print(&format!("Failed to execute popup launcher: {}", e));
            std::process::exit(1);
        }
    }
}