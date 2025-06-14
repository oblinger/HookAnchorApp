use std::env;
use crate::{load_commands, filter_commands};

pub fn run_match_command(args: &[String]) {
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

pub fn handle_command_line() -> Option<()> {
    let args: Vec<String> = env::args().collect();
    
    // Check if this is command-line match mode
    if args.len() >= 3 && args[1] == "match" {
        run_match_command(&args);
        return Some(());
    }
    
    None // Not a command-line mode, should run GUI
}