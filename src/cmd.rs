use std::collections::HashMap;
use crate::{load_commands_with_data, load_commands_for_inference, filter_commands, execute_via_server, utils, grabber, run_patch_inference, save_commands_to_file};

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
        "-a" | "--action" => run_test_action(&args),
        "-f" | "--folders" => run_folder_command(&args),
        "-F" | "--named-folders" => run_folder_with_commands(&args),
        "--hook" => handle_hook_option(&args),
        "--user-info" => print_user_info(),
        "--test-grabber" => run_test_grabber(),
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
        "--install" => run_install(),
        "--uninstall" => run_uninstall(),
        "--execute-launcher-command" => run_execute_launcher_command(&args),
        "--popup" => run_popup_command(&args),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use -h or --help for usage information");
            std::process::exit(1);
        }
    }
}

pub fn print_help(program_name: &str) {
    eprintln!("HookAnchor - Universal Command Launcher");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {}                          # Run GUI mode", program_name);
    eprintln!("  {} -h, --help               # Show help", program_name);
    eprintln!("  {} -m, --match <query>      # Search CMDS", program_name);
    eprintln!("  {} -r, --run_fn <cmd>       # Execute specific CMD", program_name);
    eprintln!("  {} -x, --execute <query>    # Execute top match", program_name);
    eprintln!("  {} -f, --folders <query>    # Get folder paths", program_name);
    eprintln!("  {} -F, --named-folders <q>  # Get CMDS->paths", program_name);
    eprintln!("  {} -a, --action <act> <arg> # Test action", program_name);
    eprintln!("  {} --infer [command]        # Show patch inference changes", program_name);
    eprintln!("  {} --infer-all              # Show changes and prompt to apply", program_name);
    eprintln!("  {} --rescan                 # Rescan filesystem with verbose output", program_name);
    eprintln!("  {} --rebuild                # Rebuild: restart server and rescan filesystem", program_name);
    eprintln!("  {} --test-grabber           # Test grabber functionality", program_name);
    eprintln!("  {} --grab [delay]           # Grab active app after delay", program_name);
    eprintln!("  {} --start-server           # Force restart command server", program_name);
    eprintln!("  {} --restart                # Kill and restart command server in new Terminal", program_name);
    eprintln!("  {} --process-health         # Check for hung processes", program_name);
    eprintln!("  {} --process-status         # Show detailed process status", program_name);
    eprintln!("  {} --install                # Run setup assistant (install)", program_name);
    eprintln!("  {} --uninstall              # Uninstall HookAnchor", program_name);
    eprintln!("  {} --popup [show|hide|delete|status] # Manage popup window", program_name);
    eprintln!("  {} --hook <url>             # Handle hook:// URL (for URL handler)", program_name);
    eprintln!("  open 'hook://query'         # Handle hook URL via URL handler");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {}           # Launch GUI", program_name);
    eprintln!("  {} -m spot   # Find 'spot' CMDS", program_name);
    eprintln!("  {} -x spot   # Execute top 'spot'", program_name);
    eprintln!("  {} -f spot   # Get 'spot' folders", program_name);
    eprintln!("  {} -F spo    # Get 'spo' CMDS->paths", program_name);
    eprintln!("  {} -r Spot   # Execute 'Spot' CMD", program_name);
}


fn handle_hook_option(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} --hook <url>", args[0]);
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
        utils::debug_log("URL_HANDLER", "Empty query in hook URL");
        return;
    }
    
    // Visual separator for URL handler execution
    utils::debug_log("", "=================================================================");
    utils::debug_log("USER INPUT", &format!("URL: '{}'", decoded_query));
    utils::debug_log("URL_HANDLER", &format!("Processing hook URL: {} -> query: '{}'", url, decoded_query));
    
    // Client environment logging is now handled automatically in execute_command based on action type
    
    // Use the same logic as -x command
    let sys_data = crate::core::sys_data::get_sys_data();
    let filtered = crate::core::commands::get_display_commands(&sys_data, &decoded_query, 1);
    
    if filtered.is_empty() {
        utils::debug_log("URL_HANDLER", &format!("No commands found for query: '{}'", decoded_query));
        return;
    }
    
    let top_command_obj = &filtered[0];
    utils::debug_log("URL_HANDLER", &format!("Executing command: {}", top_command_obj.command));
    
    // For URL handling, execute directly via launcher (like -a action test) to avoid GUI context
    // This prevents the popup from showing when handling URLs
    utils::debug_log("URL_HANDLER", &format!("Launching via launcher: {} {}", top_command_obj.action, top_command_obj.arg));
    
    // execute_via_server now returns void and handles all retries internally
    execute_via_server(&top_command_obj);
    utils::debug_log("URL_HANDLER", "Command executed via server");
}

fn run_match_command(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} -m, --match <query> [debug]", args[0]);
        std::process::exit(1);
    }
    
    let query = &args[2];
    let debug = args.len() > 3 && args[3] == "debug";
    
    let sys_data = crate::core::sys_data::get_sys_data();
    let filtered = if debug {
        filter_commands(&sys_data.commands, query, 10, debug)  // Keep debug mode using original function
    } else {
        crate::core::commands::get_display_commands(&sys_data, query, 50)
    };
    
    // Print first 50 matches to see submenu results
    for cmd in filtered.iter().take(50) {
        println!("{}", cmd.command);
    }
}

fn run_exec_command(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} -r, --run_fn <command>", args[0]);
        std::process::exit(1);
    }
    
    let command = &args[2];
    
    // Visual separator for run function execution
    utils::debug_log("", "=================================================================");
    utils::debug_log("USER INPUT", &format!("RUN_FN: '{}'", command));
    
    println!("Executing command function: {}", command);
    
    // Load system data to find the actual command
    let sys_data = crate::core::sys_data::get_sys_data();
    
    // Find the command by name (case-insensitive)
    let matching_cmd = sys_data.commands.iter()
        .find(|cmd| cmd.command.to_lowercase() == command.to_lowercase());
    
    if let Some(cmd) = matching_cmd {
        // Execute the actual Command object via server
        // execute_via_server now returns void and handles all retries internally
        execute_via_server(&cmd);
        println!("Command completed");
    } else {
        eprintln!("Command '{}' not found", command);
        std::process::exit(1);
    }
}

fn run_execute_top_match(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} -x, --execute <query>", args[0]);
        std::process::exit(1);
    }
    
    let query = &args[2];
    
    // Visual separator for command execution
    utils::debug_log("", "=================================================================");
    utils::debug_log("USER INPUT", &format!("CLI: '{}'", query));
    
    // Client environment logging is now handled automatically in execute_command based on action type
    
    let sys_data = crate::core::sys_data::get_sys_data();
    let filtered = crate::core::commands::get_display_commands(&sys_data, query, 1);
    
    if filtered.is_empty() {
        eprintln!("No commands found matching: {}", query);
        std::process::exit(1);
    }
    
    let top_command_obj = &filtered[0];
    println!("Executing top match: {}", top_command_obj.command);
    
    // Save the last executed command for add_alias functionality
    use crate::core::state::save_last_executed_command;
    crate::utils::detailed_log("STATE_SAVE", &format!("CLI: Attempting to save last executed command: '{}'", top_command_obj.command));
    match save_last_executed_command(&top_command_obj.command) {
        Ok(_) => crate::utils::detailed_log("STATE_SAVE", "CLI: Successfully saved last executed command"),
        Err(e) => crate::utils::detailed_log("STATE_SAVE", &format!("CLI: Failed to save last executed command: {}", e)),
    }
    
    // Use server-based execution for consistent environment
    // execute_via_server now returns void and handles all retries internally
    execute_via_server(&top_command_obj);
    println!("Command completed");
}

fn run_test_action(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} -a, --action <action_name> [--arg <value>] [--input <value>] [--param key=value]...", args[0]);
        eprintln!("Examples:");
        eprintln!("  {} -a open_url --arg https://github.com", args[0]);
        eprintln!("  {} -a template --input \"My Note\" --param action=markdown", args[0]);
        eprintln!("  {} -a popup --param popup_action=navigate --param dx=0 --param dy=1", args[0]);
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
                    eprintln!("--arg requires a value");
                    std::process::exit(1);
                }
            }
            "--input" => {
                if i + 1 < args.len() {
                    input_value = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("--input requires a value");
                    std::process::exit(1);
                }
            }
            "--param" => {
                if i + 1 < args.len() {
                    // Parse key=value
                    if let Some((key, value)) = args[i + 1].split_once('=') {
                        extra_params.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                    } else {
                        eprintln!("--param requires key=value format");
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("--param requires a value");
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
    utils::debug_log("", "=================================================================");
    utils::debug_log("USER INPUT", &format!("ACTION: '{}' ARG: '{}' INPUT: '{}'", action_name, arg_value, input_value));
    
    // Try to execute as a unified action first
    let config = crate::core::sys_data::get_config();
    if let Some(actions) = &config.actions {
        if let Some(action) = actions.get(action_name) {
            println!("Testing unified action '{}' (type: {})", action_name, action.action_type);
            
            // Prepare variables for action execution
            let mut variables = HashMap::new();
            if !input_value.is_empty() {
                variables.insert("input".to_string(), input_value);
            }
            
            // Add extra parameters to variables
            for (key, value) in extra_params {
                if let serde_json::Value::String(s) = value {
                    variables.insert(key, s);
                }
            }
            
            // Execute the action with simplified parameters
            let arg = if !arg_value.is_empty() { Some(arg_value.as_str()) } else { None };
            match crate::core::unified_actions::execute_action(action, arg, Some(variables)) {
                Ok(result) => {
                    println!("Action completed successfully: {}", result);
                }
                Err(e) => {
                    println!("Action failed: {}", e);
                    std::process::exit(1);
                }
            }
            return;
        }
    }
    
    // Fall back to legacy action testing (for backward compatibility)
    let command_line = format!("{} {}", action_name, arg_value);
    println!("Testing legacy action '{}' with arg '{}': {}", action_name, arg_value, command_line);
    
    // Use server-based execution for testing actions
    let cmd_obj = crate::command_server::make_command(&action_name, &arg_value);
    // execute_via_server now returns void and handles all retries internally
    execute_via_server(&cmd_obj);
    println!("Action completed");
}


fn run_folder_command(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} -f, --folders <query>", args[0]);
        std::process::exit(1);
    }
    
    let query = &args[2];
    let sys_data = crate::core::sys_data::get_sys_data();
    
    // Helper function to extract folder path from a command
    let extract_folder_path = |command: &crate::Command| -> Option<String> {
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
    let display_commands = crate::core::commands::get_display_commands_with_options(&sys_data, query, 100, true);
    
    if display_commands.is_empty() {
        eprintln!("No commands found matching: {}", query);
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
                println!("{}", folder_path);
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
        println!("{}", path);
    }
}

fn run_folder_with_commands(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} -F, --named-folders <query>", args[0]);
        std::process::exit(1);
    }
    
    let query = &args[2];
    let sys_data = crate::core::sys_data::get_sys_data();
    
    // Helper function to extract folder path from a command
    let extract_folder_path = |command: &crate::Command| -> Option<String> {
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
    let display_commands = crate::core::commands::get_display_commands_with_options(&sys_data, query, 100, true);
    
    if display_commands.is_empty() {
        eprintln!("No commands found matching: {}", query);
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
                println!("{} -> {}", first_cmd.command, folder_path);
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
            println!("{} -> {}", command.command, path);
        }
    }
}

fn print_user_info() {
    println!("=== User Environment Information ===");
    
    // Environment variables
    println!("USER: {:?}", std::env::var("USER"));
    println!("LOGNAME: {:?}", std::env::var("LOGNAME"));
    println!("HOME: {:?}", std::env::var("HOME"));
    println!("SHELL: {:?}", std::env::var("SHELL"));
    println!("PATH: {:?}", std::env::var("PATH"));
    
    // Try whoami command
    use std::process::Command;
    match Command::new("whoami").output() {
        Ok(output) => {
            let whoami_user = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("whoami: '{}'", whoami_user);
        }
        Err(e) => {
            println!("whoami failed: {}", e);
        }
    }
    
    // Try id command for more user info
    match Command::new("id").output() {
        Ok(output) => {
            let id_output = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("id: {}", id_output);
        }
        Err(e) => {
            println!("id failed: {}", e);
        }
    }
    
    // Try to get user's shell profile paths
    if let Ok(home) = std::env::var("HOME") {
        println!("Checking shell profiles in {}", home);
        let profiles = vec![".zshrc", ".bash_profile", ".bashrc", ".profile"];
        for profile in profiles {
            let path = format!("{}/{}", home, profile);
            if std::path::Path::new(&path).exists() {
                println!("  {} exists", path);
            } else {
                println!("  {} not found", path);
            }
        }
    }
}

/// Test the grabber functionality - capture context and try to match rules
fn run_test_grabber() {
    println!("Testing grabber functionality...");
    println!("==================================");
    
    // Load config
    let config = crate::core::sys_data::get_config();
    println!("Loaded config with {} grabber rules", 
        config.grabber_rules.as_ref().map(|r| r.len()).unwrap_or(0));
    
    // Try to capture active app context
    println!("\nCapturing active application context...");
    match grabber::capture_active_app() {
        Ok(context) => {
            println!("SUCCESS: Captured app context");
            println!("  App: '{}'", context.app_name);
            println!("  Bundle ID: '{}'", context.bundle_id);
            println!("  Title: '{}'", context.window_title);
            
            // Show properties if any
            if let Some(props_obj) = context.properties.as_object() {
                if !props_obj.is_empty() {
                    println!("  Properties:");
                    for (key, value) in props_obj {
                        println!("    {}: '{}'", key, value.as_str().unwrap_or("(complex value)"));
                    }
                }
            }
            
            // Enrich the context
            println!("\nEnriching context with app-specific information...");
            let enriched_context = grabber::enrich_context(context);
            
            if let Some(enriched_props) = enriched_context.properties.as_object() {
                if enriched_props.len() > enriched_context.properties.as_object().map(|o| o.len()).unwrap_or(0) {
                    println!("  Added enriched properties:");
                    for (key, value) in enriched_props {
                        println!("    {}: '{}'", key, value.as_str().unwrap_or("(complex value)"));
                    }
                }
            }
            
            // Try to match against grabber rules
            println!("\nMatching against grabber rules...");
            if let Some(rules) = &config.grabber_rules {
                match grabber::match_grabber_rules(&enriched_context, rules, &config) {
                    Some((rule_name, command)) => {
                        println!("SUCCESS: Matched rule '{}'", rule_name);
                        println!("  Action: '{}'", command.action);
                        println!("  Arg: '{}'", command.arg);
                        println!("  Patch: '{}'", command.patch);
                        println!("  Would create command: {} : {} {}", 
                            "[user-provided-name]", command.action, command.arg);
                    }
                    None => {
                        println!("No rules matched - would show template dialog");
                        println!("\nGenerated rule template:");
                        println!("{}", grabber::generate_rule_template_text(&enriched_context));
                    }
                }
            } else {
                println!("No grabber rules configured in config.yaml");
            }
        }
        Err(e) => {
            println!("ERROR: Failed to capture app context: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("\nGrabber test completed successfully!");
}

/// Run grab command to capture active app and output result
fn run_grab_command(args: &[String]) {
    // Parse optional delay parameter (defaults to 0)
    let delay_seconds = if args.len() > 2 {
        match args[2].parse::<u64>() {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Invalid delay value: {}", args[2]);
                eprintln!("Usage: {} --grab [delay_seconds]", args[0]);
                std::process::exit(1);
            }
        }
    } else {
        0
    };
    
    // If delay requested, sleep before capturing
    if delay_seconds > 0 {
        eprintln!("Waiting {} seconds before capture...", delay_seconds);
        std::thread::sleep(std::time::Duration::from_secs(delay_seconds));
    }
    
    // Load config for grabber rules
    let config = crate::core::sys_data::get_config();
    
    // Perform the grab
    match grabber::grab(&config) {
        Ok(grab_result) => {
            match grab_result {
                grabber::GrabResult::RuleMatched(_rule_name, command) => {
                    // Output the action and argument for easy testing
                    println!("{} {}", command.action, command.arg);
                }
                grabber::GrabResult::NoRuleMatched(context) => {
                    // No rule matched - output context information
                    eprintln!("No grabber rule matched for:");
                    eprintln!("  App: {}", context.app_name);
                    eprintln!("  Bundle ID: {}", context.bundle_id);
                    eprintln!("  Title: {}", context.window_title);
                    if let Some(props) = context.properties.as_object() {
                        for (key, value) in props {
                            eprintln!("  props.{}: {}", key, value.as_str().unwrap_or("(complex)"));
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
    utils::log("Restarting command server...");
    
    // Kill existing server if running
    if let Err(e) = crate::command_server_management::kill_existing_server() {
        utils::log_error(&format!("Failed to kill existing server: {}", e));
    }
    
    // Start new server via Terminal
    match crate::command_server_management::start_server_via_terminal() {
        Ok(()) => {
            println!("Command server restart initiated via Terminal");
            println!("The server will start with full shell environment in a few seconds");
        }
        Err(e) => {
            utils::log_error(&format!("Failed to start server: {}", e));
            std::process::exit(1);
        }
    }
}

/// Internal daemon mode - starts persistent server
fn run_start_server_daemon() {
    use crate::core::state::save_server_pid;
    
    // Initialize config FIRST - this must happen before any other operations
    if let Err(config_error) = crate::core::sys_data::initialize_config() {
        utils::log_error(&format!("Failed to load config: {}", config_error));
        // Continue with default config
    }
    
    utils::log("Starting command server daemon...");
    
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
        .join("command_server.sock");
    let _ = std::fs::remove_file(&socket_path);
    
    // Start the persistent server
    match crate::command_server::start_persistent_server() {
        Ok(server_pid) => {
            // Save PID to state
            if let Err(e) = save_server_pid(server_pid) {
                utils::log_error(&format!("Could not save server PID: {}", e));
            }
            
            
            // Keep the process alive - this is the daemon
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3600)); // Sleep 1 hour at a time
            }
        }
        Err(e) => {
            utils::log_error(&format!("Failed to start persistent server: {}", e));
            std::process::exit(1);
        }
    }
}

/// Checks if the new_patch is associated with a parent directory of the current_patch
#[allow(dead_code)]
fn is_parent_directory_patch(current_patch: &str, new_patch: &str, patches: &std::collections::HashMap<String, crate::Patch>) -> bool {
    use std::path::Path;
    
    // Get the linked commands for both patches
    let current_linked = crate::core::commands::get_patch(current_patch, patches).and_then(|p| p.linked_command.as_ref());
    let new_linked = crate::core::commands::get_patch(new_patch, patches).and_then(|p| p.linked_command.as_ref());
    
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
    
    println!("Analyzing patch inference changes...");
    
    // Load current commands (without inference and orphan creation)
    let (_config, mut commands, patches) = load_commands_for_inference();
    
    println!("\n=== COMMAND PATCH CHANGES ===");
    
    // Run inference without applying changes, but print to stdout
    let (changes_found, new_patches_to_add) = run_patch_inference(
        &mut commands, 
        &patches, 
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true (show proposed changes)
        true   // overwrite_patch = true (show all potential changes)
    );
    
    if changes_found == 0 {
        println!("  No commands would have their patches changed.");
    }
    
    // Show orphan patches that would be created
    if !new_patches_to_add.is_empty() {
        println!("\n=== ORPHAN PATCHES THAT WOULD BE CREATED ===");
        for new_patch in &new_patches_to_add {
            println!("  New orphan patch: {}", new_patch);
        }
    }
    
    println!("\n=== SUMMARY ===");
    println!("  Commands that would change: {}", changes_found);
    println!("  Orphan patches that would be created: {}", new_patches_to_add.len());
    
    if changes_found == 0 && new_patches_to_add.is_empty() {
        println!("  No changes would be made.");
    } else {
        println!("  Use --infer-all to apply these changes (normal startup only fills empty patches).");
    }
}

/// Show patch inference for a specific command
fn run_infer_single_command(command_name: &str) {
    // Load current commands (without inference and orphan creation)
    let (_config, commands, patches) = load_commands_with_data();
    
    // Find the command by name
    let found_command = commands.iter().find(|cmd| cmd.command == command_name);
    
    match found_command {
        Some(command) => {
            println!("Command: {}", command.command);
            println!("Current patch: '{}'", command.patch);
            
            // Test patch inference on this specific command
            match crate::core::commands::infer_patch(command, &patches) {
                Some(inferred_patch) => {
                    println!("Inferred patch: '{}'", inferred_patch);
                    
                    if command.patch == inferred_patch {
                        println!("✅ No change needed - current patch matches inferred patch");
                    } else if command.patch.is_empty() {
                        println!("📄 Would assign patch: '{}' -> '{}'", command.patch, inferred_patch);
                    } else {
                        println!("🔄 Would change patch: '{}' -> '{}'", command.patch, inferred_patch);
                    }
                }
                None => {
                    if command.patch.is_empty() {
                        println!("No patch could be inferred (would remain empty)");
                    } else {
                        println!("No patch could be inferred (would keep current: '{}')", command.patch);
                    }
                }
            }
        }
        None => {
            println!("Command '{}' not found.", command_name);
            
            // Show similar commands as suggestions
            let similar_commands: Vec<&crate::Command> = commands.iter()
                .filter(|cmd| cmd.command.to_lowercase().contains(&command_name.to_lowercase()))
                .take(5)
                .collect();
            
            if !similar_commands.is_empty() {
                println!("\nSimilar commands found:");
                for cmd in similar_commands {
                    println!("  {}", cmd.command);
                }
            }
        }
    }
}

/// Show patch inference changes and prompt user to apply them
fn run_infer_all_patches(_args: &[String]) {
    println!("Analyzing patch inference changes...");
    
    // Load current commands (without inference and orphan creation)
    let (_config, mut commands, patches) = load_commands_for_inference();
    
    println!("\n=== COMMAND PATCH CHANGES ===");
    
    // First run: Show changes without applying
    let (changes_found, new_patches_to_add) = run_patch_inference(
        &mut commands, 
        &patches, 
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true (show proposed changes)
        true   // overwrite_patch = true (show all potential changes including overwriting existing patches)
    );
    
    if changes_found == 0 {
        println!("  No commands would have their patches changed.");
    }
    
    // Show orphan patches that would be created
    if !new_patches_to_add.is_empty() {
        println!("\n=== ORPHAN PATCHES THAT WOULD BE CREATED ===");
        for new_patch in &new_patches_to_add {
            println!("  New orphan patch: {}", new_patch);
        }
    }
    
    if changes_found == 0 && new_patches_to_add.is_empty() {
        println!("\nNo changes would be made.");
        return;
    }
    
    // Prompt user for confirmation
    println!("\n=== SUMMARY ===");
    println!("  Commands that would change: {}", changes_found);
    println!("  Orphan patches that would be created: {}", new_patches_to_add.len());
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
                    println!("Updated {} commands and saved to file.", applied_count);
                }
                Err(e) => {
                    utils::log_error(&format!("Updated {} commands but failed to save: {}", applied_count, e));
                }
            }
        } else {
            println!("No commands were actually updated.");
        }
    } else {
        println!("No changes applied.");
    }
}

fn run_rescan_command() {
    println!("🚀 HookAnchor Rescan - Verbose Mode");
    println!("====================================");
    
    // Load configuration
    let config = crate::core::sys_data::get_config();
    
    // Get markdown roots
    let _markdown_roots = match &config.popup_settings.markdown_roots {
        Some(roots) => roots.clone(),
        None => {
            eprintln!("❌ No markdown roots configured in config file");
            std::process::exit(1);
        }
    };
    
    println!("\n📋 Initial data load...");
    
    // Load existing commands from disk first (to preserve patches), then run verbose load
    let existing_commands = crate::load_commands();
    let global_data = crate::core::sys_data::load_data(existing_commands, true);
    
    println!("\n🔍 Starting filesystem scan...");
    
    // Run scan with verbose output
    let scanned_commands = crate::scanner::scan_verbose(
        global_data.commands.clone(),
        &global_data,
        true
    );
    
    println!("\n📊 Final Summary:");
    println!("   Total commands after rescan: {}", scanned_commands.len());
    
    // Count commands by action type
    let mut action_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for cmd in &scanned_commands {
        *action_counts.entry(cmd.action.clone()).or_insert(0) += 1;
    }
    
    println!("\n   Commands by action type:");
    let mut sorted_actions: Vec<_> = action_counts.iter().collect();
    sorted_actions.sort_by_key(|(action, _)| (*action).clone());
    
    for (action, count) in sorted_actions {
        println!("     {}: {}", action, count);
    }
    
    println!("\n✅ Rescan complete!");
}

/// Check for hung processes
fn run_process_health() {
    println!("Checking process health...");
    crate::process_monitor::check_system_health();
    println!("Health check complete. See debug logs for details.");
}

/// Show detailed process status
fn run_process_status() {
    println!("Process status:");
    crate::process_monitor::show_process_status();
}

/// Run the setup assistant (install)
fn run_install() {
    println!("🔄 Running HookAnchor setup assistant...");
    println!("================================================");
    println!();
    
    // Run the setup assistant with force flag
    match crate::setup_assistant::SetupAssistant::new().run_setup(true) {
        Ok(()) => {
            println!();
            println!("✅ Installation completed successfully!");
            println!();
            println!("Changes made:");
            println!("  - Karabiner configuration updated/reinstalled");
            println!("  - Configuration files validated");
            println!("  - Setup assistant completed");
            println!();
            println!("You may need to:");
            println!("  1. Enable the HookAnchor rule in Karabiner-Elements preferences");
            println!("  2. Restart HookAnchor if currently running");
        },
        Err(e) => {
            eprintln!("❌ Installation failed: {}", e);
            eprintln!();
            eprintln!("Please check the error message above and try again.");
            std::process::exit(1);
        }
    }
}

/// Uninstall HookAnchor
fn run_uninstall() {
    println!("🗑️  HookAnchor Uninstall");
    println!("========================");
    println!();
    
    println!("This will remove:");
    println!("  - Karabiner configuration for HookAnchor");
    println!("  - Configuration directory: ~/.config/hookanchor");  
    println!("  - URL handler registration");
    println!();
    
    print!("Are you sure you want to uninstall HookAnchor? (y/N): ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_lowercase();
    
    if input == "y" || input == "yes" {
        println!();
        println!("🗑️  Uninstalling HookAnchor...");
        
        // Remove configuration directory
        if let Ok(home) = std::env::var("HOME") {
            let config_dir = format!("{}/.config/hookanchor", home);
            if std::path::Path::new(&config_dir).exists() {
                match std::fs::remove_dir_all(&config_dir) {
                    Ok(()) => println!("✅ Removed configuration directory"),
                    Err(e) => println!("⚠️  Could not remove config directory: {}", e),
                }
            }
        }
        
        // TODO: Remove Karabiner configuration
        // TODO: Unregister URL handler
        
        println!();
        println!("✅ HookAnchor uninstalled successfully!");
        println!("You may need to manually remove any remaining files:");
        println!("  - App bundles in /Applications");
        println!("  - Binary files if installed to system paths");
    } else {
        println!("Uninstall cancelled.");
    }
}

/// Restart the command server - kill existing and start new one in Terminal
fn run_restart_server() {
    println!("🔄 Restarting command server...");
    
    // First, hide the popup if it's visible by sending command to socket
    println!("  Hiding popup window...");
    use std::os::unix::net::UnixStream;
    use std::io::Write;
    
    if let Ok(mut stream) = UnixStream::connect("/tmp/hookanchor_popup.sock") {
        if let Err(e) = stream.write_all(b"hide") {
            println!("  ⚠️  Could not send hide command to popup: {}", e);
        } else {
            println!("  ✅ Popup window hidden");
        }
    } else {
        println!("  ⚠️  Popup server not running (window may already be hidden)");
    }
    
    // Kill any existing popup_server first
    println!("  Killing existing popup_server...");
    use std::process::Command;
    match Command::new("pkill")
        .arg("-f")
        .arg("popup_server")
        .output() {
        Ok(_) => println!("  ✅ Existing popup_server killed"),
        Err(e) => println!("  ⚠️  Failed to kill popup_server: {}", e),
    }
    
    // Kill any existing command server
    println!("  Killing existing server...");
    match crate::command_server_management::kill_existing_server() {
        Ok(()) => println!("  ✅ Existing server killed"),
        Err(e) => println!("  ⚠️  No existing server found or kill failed: {}", e),
    }
    
    // Clear the socket file to ensure clean start
    let socket_path = std::path::Path::new("/Users/oblinger/.config/hookanchor/command_server.sock");
    if socket_path.exists() {
        if let Err(e) = std::fs::remove_file(socket_path) {
            println!("  ⚠️  Failed to remove socket file: {}", e);
        }
    }
    
    // Reset server check flag to force restart
    crate::command_server_management::reset_server_check();
    
    // Start new server via Terminal (same method as auto-start)
    println!("  Starting new server in Terminal...");
    match crate::command_server_management::start_server_via_terminal() {
        Ok(()) => {
            println!("  ✅ Server restart initiated via Terminal");
            println!("  📱 A new Terminal window should open with the server daemon");
            println!("  📄 Server output will be logged to ~/.config/hookanchor/server.log");
            
            // Wait for the server to start - check multiple times
            println!("  ⏳ Waiting for server to start...");
            let mut server_started = false;
            let max_attempts = 50; // 50 attempts * 200ms = 10 seconds max
            let socket_path = std::path::Path::new("/Users/oblinger/.config/hookanchor/command_server.sock");
            
            for attempt in 1..=max_attempts {
                std::thread::sleep(std::time::Duration::from_millis(200));
                
                // First check if socket file exists
                if socket_path.exists() {
                    // Give it a moment for the socket to be fully ready
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    
                    // Then check if server is available
                    match crate::command_server::CommandClient::new() {
                        Ok(client) => {
                            if client.is_server_available() {
                                server_started = true;
                                break;
                            }
                        }
                        Err(_) => {
                            // Client creation failed, continue waiting
                        }
                    }
                }
                
                // Show progress every second
                if attempt % 5 == 0 {
                    print!(".");
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                }
            }
            
            if server_started {
                println!("\n  ✅ Server started successfully!");
                
                // Also verify by checking the PID in state
                let state = crate::core::state::load_state();
                if let Some(pid) = state.server_pid {
                    println!("  📊 Server running with PID: {}", pid);
                }
                
                println!("  🎯 Server is ready to accept commands");
            } else {
                println!("\n  ❌ Server failed to start within 10 seconds");
                println!("  💡 Check the Terminal window for error messages");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("  ❌ Failed to start server: {}", e);
            eprintln!("  💡 Try running manually: ha --start-server-daemon");
            std::process::exit(1);
        }
    }
}

/// Execute a launcher command - used by launchctl asuser to run commands in GUI session
fn run_execute_launcher_command(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} --execute-launcher-command <launcher_command>", args[0]);
        std::process::exit(1);
    }
    
    let launcher_command = &args[2];
    
    // Visual separator for launcher command execution
    utils::debug_log("", "=================================================================");
    utils::detailed_log("LAUNCHER_CMD", &format!("Executing in GUI session: '{}'", launcher_command));
    
    // Execute the launcher command via server (consistent with all execution)
    // Parse the launcher command to create a Command object
    let parts: Vec<&str> = launcher_command.split_whitespace().collect();
    let (action, arg) = if parts.len() > 1 {
        (parts[0], parts[1..].join(" "))
    } else {
        (launcher_command.as_str(), String::new())
    };
    
    let cmd_obj = crate::command_server::make_command(action, &arg);
    // execute_via_server now returns void and handles all retries internally
    execute_via_server(&cmd_obj);
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
    crate::utils::log(&format!("REBUILD SESSION: {}", build_id));
    crate::utils::log(&format!("TIMESTAMP: {}", build_timestamp.format("%Y-%m-%d %H:%M:%S%.3f")));
    crate::utils::log(&"=".repeat(80));
    crate::utils::debug_log("REBUILD", &format!("Starting rebuild session {}", build_id));
    
    println!("🏗️  HookAnchor Rebuild - Full Reset");
    println!("===================================");
    
    // Step 1: Build the release binary
    println!("\n🔨 Step 1/3: Building release binary...");
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
                println!("  ✅ Build successful ({:.1}s)", build_time.as_secs_f32());
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("  ❌ Build failed:");
                println!("{}", stderr);
                println!("\n⚠️  Cannot proceed with rebuild due to compilation errors");
                std::process::exit(1);
            }
        }
        Err(e) => {
            println!("  ❌ Failed to run cargo build: {}", e);
            println!("\n⚠️  Make sure cargo is installed and in PATH");
            std::process::exit(1);
        }
    }
    
    println!("\n🔄 Step 2/3: Restarting command server...");
    
    // Kill existing server
    match crate::command_server_management::kill_existing_server() {
        Ok(()) => println!("  ✅ Existing server killed"),
        Err(e) => println!("  ⚠️  Server kill warning: {}", e),
    }
    
    // Clear the socket file to ensure clean start
    let socket_path = std::path::Path::new("/Users/oblinger/.config/hookanchor/command_server.sock");
    if socket_path.exists() {
        if let Err(e) = std::fs::remove_file(socket_path) {
            println!("  ⚠️  Failed to remove socket file: {}", e);
        }
    }
    
    // Reset server check flag to force restart
    crate::command_server_management::reset_server_check();
    
    // Start new server via Terminal (same method as run_restart_server)
    println!("  Starting new server in Terminal...");
    match crate::command_server_management::start_server_via_terminal() {
        Ok(()) => {
            println!("  ✅ Server restart initiated via Terminal");
            println!("  📱 A new Terminal window should open with the server daemon");
        },
        Err(e) => {
            println!("  ❌ Failed to start server: {}", e);
            return;
        }
    }
    
    println!("\n📁 Step 3/3: Rescanning filesystem...");
    
    // Run filesystem rescan
    run_rescan_command();
    
    println!("\n🎉 Rebuild complete!");
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
    
    utils::debug_log("POPUP_CMD", &format!("Popup action requested: {}", action));
    
    // Validate action
    match action {
        "show" | "hide" | "delete" | "status" => {},
        _ => {
            utils::log_error(&format!("Unknown popup action: {}", action));
            eprintln!("Unknown popup action: {}", action);
            eprintln!("Valid actions: show, hide, delete, status");
            std::process::exit(1);
        }
    }
    
    // Find popup launcher binary
    let exe_path = std::env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
    let popup_path = exe_dir.join("popup");
    
    if !popup_path.exists() {
        utils::log_error(&format!("Popup launcher not found at: {:?}", popup_path));
        eprintln!("Popup launcher not found at: {:?}", popup_path);
        std::process::exit(1);
    }
    
    utils::debug_log("POPUP_CMD", &format!("Executing popup launcher: {:?} {}", popup_path, action));
    
    // Execute popup launcher with action
    match Command::new(&popup_path)
        .arg(action)
        .output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                utils::debug_log("POPUP_CMD", &format!("Popup {} succeeded: {}", action, stdout.trim()));
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
            eprintln!("Failed to execute popup launcher: {}", e);
            std::process::exit(1);
        }
    }
}