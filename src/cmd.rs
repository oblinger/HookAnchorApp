use crate::{load_commands, filter_commands, launcher, CommandTarget, execute_command, utils};

/// Main entry point for command-line mode
pub fn run_command_line_mode(args: Vec<String>) {
    // Check for hook:// URL as first argument
    if args.len() >= 2 && args[1].starts_with("hook://") {
        handle_hook_url(&args[1]);
        return;
    }
    
    // Check for help flags
    if args.len() >= 2 && (args[1] == "--help" || args[1] == "-h" || args[1] == "help") {
        print_help(&args[0]);
        return;
    }
    
    if args.len() < 2 {
        print_help(&args[0]);
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "match" => run_match_command(&args),
        "exec" => run_exec_command(&args),
        "-x" => run_execute_top_match(&args),
        "-a" => run_test_action(&args),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use --help for usage information");
            std::process::exit(1);
        }
    }
}

pub fn print_help(program_name: &str) {
    eprintln!("Anchor Selector - Universal Command Launcher");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {}                           - Run GUI mode (interactive popup)", program_name);
    eprintln!("  {} --help, -h, help          - Show this help message", program_name);
    eprintln!("  {} match <query> [debug]     - Search and list matching commands", program_name);
    eprintln!("  {} exec <command>            - Execute a specific command", program_name);
    eprintln!("  {} -x <query>                - Execute top matching command for query", program_name);
    eprintln!("  {} -a <action> <arg>         - Test action directly with argument", program_name);
    eprintln!("  {} hook://query              - Handle hook:// URL (execute top match)", program_name);
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {}                           # Launch interactive GUI", program_name);
    eprintln!("  {} match spot                # Find commands matching 'spot'", program_name);
    eprintln!("  {} -x spot                   # Execute the top match for 'spot'", program_name);
    eprintln!("  {} exec \"spot\"               # Execute the exact command 'spot'", program_name);
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
        eprintln!("Usage: {} match <query> [debug]", args[0]);
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
        eprintln!("Usage: {} exec <command>", args[0]);
        std::process::exit(1);
    }
    
    let command = &args[2];
    println!("Executing command: {}", command);
    
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
        eprintln!("Usage: {} -x <query>", args[0]);
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
        eprintln!("Usage: {} -a <action> <arg>", args[0]);
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