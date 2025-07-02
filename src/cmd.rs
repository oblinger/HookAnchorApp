use std::env;
use anchor_selector::{load_commands, filter_commands, launcher, CommandTarget};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {} match <query> [debug]  - Search and list matching commands", args[0]);
        eprintln!("  {} exec <command>         - Execute a specific command", args[0]);
        eprintln!("  {} -x <query>             - Execute top matching command for query", args[0]);
        eprintln!("  {} -a <action> <arg>      - Test action directly with argument", args[0]);
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "match" => run_match_command(&args),
        "exec" => run_exec_command(&args),
        "-x" => run_execute_top_match(&args),
        "-a" => run_test_action(&args),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use 'match', 'exec', '-x', or '-a'");
            std::process::exit(1);
        }
    }
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
    let result = anchor_selector::execute_command(top_command_obj);
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