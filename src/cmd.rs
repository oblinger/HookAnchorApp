use std::env;
use anchor_selector::{load_commands, filter_commands, execute_command};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {} match <query> [debug]  - Search and list matching commands", args[0]);
        eprintln!("  {} exec <command>         - Execute a specific command", args[0]);
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "match" => run_match_command(&args),
        "exec" => run_exec_command(&args),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Use 'match' or 'exec'");
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
    let filtered = filter_commands(&commands, query, debug);
    
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
    execute_command(command);
}