use crate::{load_commands, filter_commands, launcher, CommandTarget, execute_command, utils};

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
    
    utils::debug_log("URL_HANDLER", &format!("Processing hook URL: {} -> query: '{}'", url, decoded_query));
    
    // Use the same logic as -x command
    let commands = load_commands();
    let filtered = filter_commands(&commands, &decoded_query, 1, false);
    
    if filtered.is_empty() {
        utils::debug_log("URL_HANDLER", &format!("No commands found for query: '{}'", decoded_query));
        return;
    }
    
    let top_command_obj = &filtered[0];
    utils::debug_log("URL_HANDLER", &format!("Executing command: {}", top_command_obj.command));
    
    // Execute the command (same as -x logic but without the verbose output)
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
    let filtered = filter_commands(&commands, query, 10, debug);
    
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
    let commands = load_commands();
    let filtered = filter_commands(&commands, query, 1, false);
    
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
            "obs" => {
                // For obs commands, need to resolve the full path and return the directory
                // The arg format is like "T/Career/NJ/TG/TG AI Safety.md"
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                let vault_path = format!("{}/ob/kmr", home); // Default vault path
                let full_path = format!("{}/{}", vault_path, command.arg);
                
                // Return the directory containing the .md file
                if let Some(last_slash) = full_path.rfind('/') {
                    Some(full_path[..last_slash].to_string())
                } else {
                    Some(full_path)
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
            "obs" => {
                // For obs commands, need to resolve the full path and return the directory
                // The arg format is like "T/Career/NJ/TG/TG AI Safety.md"
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                let vault_path = format!("{}/ob/kmr", home); // Default vault path
                let full_path = format!("{}/{}", vault_path, command.arg);
                
                // Return the directory containing the .md file
                if let Some(last_slash) = full_path.rfind('/') {
                    Some(full_path[..last_slash].to_string())
                } else {
                    Some(full_path)
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