use crate::{load_commands, load_commands_raw, filter_commands, launcher, CommandTarget, execute_command, utils, grabber, load_config, create_patches_hashmap, run_patch_inference, save_commands_to_file};

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
        "--user-info" => print_user_info(),
        "--test-grabber" => run_test_grabber(),
        "--grab" => run_grab_command(&args),
        "--infer" => run_infer_patches(&args),
        "--infer-all" => run_infer_all_patches(&args),
        "--start-server" => run_start_server(),
        "--start-server-daemon" => run_start_server_daemon(),
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
    eprintln!("  {} --test-grabber           # Test grabber functionality", program_name);
    eprintln!("  {} --grab [delay]           # Grab active app after delay", program_name);
    eprintln!("  {} --start-server           # Force restart command server", program_name);
    eprintln!("  open 'hook://query'         # Handle hook URL");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {}           # Launch GUI", program_name);
    eprintln!("  {} -m spot   # Find 'spot' CMDS", program_name);
    eprintln!("  {} -x spot   # Execute top 'spot'", program_name);
    eprintln!("  {} -f spot   # Get 'spot' folders", program_name);
    eprintln!("  {} -F spo    # Get 'spo' CMDS->paths", program_name);
    eprintln!("  {} -r Spot   # Execute 'Spot' CMD", program_name);
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
    let commands = load_commands();
    let config = load_config();
    let filtered = crate::core::commands::get_display_commands(&commands, &decoded_query, &config, 1);
    
    if filtered.is_empty() {
        utils::debug_log("URL_HANDLER", &format!("No commands found for query: '{}'", decoded_query));
        return;
    }
    
    let top_command_obj = &filtered[0];
    utils::debug_log("URL_HANDLER", &format!("Executing command: {}", top_command_obj.command));
    
    // Execute the command using the same path as CLI commands
    // The command server ensures consistent environment regardless of execution context
    let _result = execute_command(top_command_obj);
}

fn run_match_command(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} -m, --match <query> [debug]", args[0]);
        std::process::exit(1);
    }
    
    let query = &args[2];
    let debug = args.len() > 3 && args[3] == "debug";
    
    let commands = load_commands();
    let config = load_config();
    let filtered = if debug {
        filter_commands(&commands, query, 10, debug)  // Keep debug mode using original function
    } else {
        crate::core::commands::get_display_commands(&commands, query, &config, 10)
    };
    
    // Print first 10 matches (like the GUI)
    for cmd in filtered.iter().take(10) {
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
    
    // Use the new launcher system for execution
    match launcher::launch(command) {
        Ok(()) => {
            println!("Command completed successfully");
        },
        Err(e) => {
            eprintln!("Command failed: {:?}", e);
            std::process::exit(1);
        }
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
    
    let commands = load_commands();
    let config = load_config();
    let filtered = crate::core::commands::get_display_commands(&commands, query, &config, 1);
    
    if filtered.is_empty() {
        eprintln!("No commands found matching: {}", query);
        std::process::exit(1);
    }
    
    let top_command_obj = &filtered[0];
    println!("Executing top match: {}", top_command_obj.command);
    
    // Use execute_command which properly handles parsed action/arg
    let result = execute_command(top_command_obj);
    match result {
        CommandTarget::Command(_) => {
            println!("Command completed successfully");
        },
        CommandTarget::Alias(next_cmd) => {
            println!("Command completed successfully (alias to: {})", next_cmd);
        }
    }
}

fn run_test_action(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: {} -a, --action <action> <arg>", args[0]);
        std::process::exit(1);
    }
    
    let action = &args[2];
    let arg = &args[3];
    let command_line = format!("{} {}", action, arg);
    
    // Visual separator for action testing
    utils::debug_log("", "=================================================================");
    utils::debug_log("USER INPUT", &format!("ACTION: '{}' ARG: '{}'", action, arg));
    
    println!("Testing action '{}' with arg '{}': {}", action, arg, command_line);
    
    // Use the new launcher system for testing actions directly
    match launcher::launch(&command_line) {
        Ok(()) => {
            println!("Action completed successfully");
        },
        Err(e) => {
            eprintln!("Action failed: {:?}", e);
            std::process::exit(1);
        }
    }
}


fn run_folder_command(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} -f, --folders <query>", args[0]);
        std::process::exit(1);
    }
    
    let query = &args[2];
    let commands = load_commands();
    let config = crate::load_config();
    
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
    let display_commands = crate::core::commands::get_display_commands_with_options(&commands, query, &config, 100, true);
    
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
    let commands = load_commands();
    let config = crate::load_config();
    
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
    let display_commands = crate::core::commands::get_display_commands_with_options(&commands, query, &config, 100, true);
    
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
    let config = load_config();
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
    let config = load_config();
    
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
            eprintln!("Error capturing active app: {}", e);
            std::process::exit(1);
        }
    }
}

/// Force restart the command server
fn run_start_server() {
    println!("Restarting command server...");
    
    // Kill existing server if running
    if let Err(e) = crate::server_management::kill_existing_server() {
        eprintln!("Warning: Failed to kill existing server: {}", e);
    }
    
    // Start new server via Terminal
    match crate::server_management::start_server_via_terminal() {
        Ok(()) => {
            println!("Command server restart initiated via Terminal");
            println!("The server will start with full shell environment in a few seconds");
        }
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    }
}

/// Internal daemon mode - starts persistent server
fn run_start_server_daemon() {
    use crate::core::state::save_server_pid;
    
    println!("Starting command server daemon...");
    
    // Change to home directory
    if let Ok(home) = std::env::var("HOME") {
        if let Err(e) = std::env::set_current_dir(&home) {
            eprintln!("Warning: Could not change to home directory: {}", e);
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
                eprintln!("Warning: Could not save server PID: {}", e);
            }
            
            println!("Command server started successfully with PID: {}", server_pid);
            println!("Server will run persistently until killed or system restart");
            
            // Keep the process alive - this is the daemon
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3600)); // Sleep 1 hour at a time
            }
        }
        Err(e) => {
            eprintln!("Failed to start persistent server: {}", e);
            std::process::exit(1);
        }
    }
}

/// Checks if the new_patch is associated with a parent directory of the current_patch
fn is_parent_directory_patch(current_patch: &str, new_patch: &str, patches: &std::collections::HashMap<String, crate::Patch>) -> bool {
    use std::path::Path;
    
    // Get the linked commands for both patches
    let current_patch_lower = current_patch.to_lowercase();
    let new_patch_lower = new_patch.to_lowercase();
    
    let current_linked = patches.get(&current_patch_lower).and_then(|p| p.linked_command.as_ref());
    let new_linked = patches.get(&new_patch_lower).and_then(|p| p.linked_command.as_ref());
    
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
    
    // Load current commands (without inference)
    let mut commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Run inference without applying changes, but print to stdout
    let (changes_found, _) = run_patch_inference(
        &mut commands, 
        &patches, 
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true (show proposed changes)
        true   // overwrite_patch = true (show all potential changes)
    );
    
    println!("\nSummary:");
    println!("  Commands that would change: {}", changes_found);
    if changes_found == 0 {
        println!("  No patch changes would be made.");
    } else {
        println!("  Use --infer-all to apply these changes (normal startup only fills empty patches).");
    }
}

/// Show patch inference for a specific command
fn run_infer_single_command(command_name: &str) {
    // Load current commands (without inference)
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
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
    
    // Load current commands (without inference)
    let mut commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // First run: Show changes without applying
    let (changes_found, _) = run_patch_inference(
        &mut commands, 
        &patches, 
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true (show proposed changes)
        true   // overwrite_patch = true (show all potential changes including overwriting existing patches)
    );
    
    if changes_found == 0 {
        println!("\nNo patch changes would be made.");
        return;
    }
    
    // Prompt user for confirmation
    println!("\nFound {} commands that would change.", changes_found);
    print!("Apply all changes? (y/n): ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_lowercase();
    
    if input == "y" || input == "yes" {
        // Reload commands and apply changes
        let mut commands = load_commands_raw();
        let patches = create_patches_hashmap(&commands);
        
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
                    eprintln!("Updated {} commands but failed to save: {}", applied_count, e);
                }
            }
        } else {
            println!("No commands were actually updated.");
        }
    } else {
        println!("No changes applied.");
    }
}