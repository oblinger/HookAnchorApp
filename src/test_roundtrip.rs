use anchor_selector::{load_commands, save_commands_formatted};
use std::process;

fn main() {
    println!("Loading commands from spot_cmds.txt...");
    let commands = load_commands();
    println!("Loaded {} commands", commands.len());
    
    println!("Writing commands to spot_cmds2.txt...");
    match save_commands_formatted(&commands, "spot_cmds2.txt") {
        Ok(_) => println!("Successfully wrote to spot_cmds2.txt"),
        Err(e) => {
            eprintln!("Error writing to spot_cmds2.txt: {}", e);
            process::exit(1);
        }
    }
    
    println!("\nNow compare the files:");
    println!("diff ~/ob/data/spot_cmds/spot_cmds.txt ~/ob/data/spot_cmds/spot_cmds2.txt");
    
    // Also show first few lines of each file for comparison
    println!("\nFirst 5 lines of original file:");
    if let Ok(content) = std::fs::read_to_string(std::path::Path::new(&std::env::var("HOME").unwrap_or(".".to_string())).join("ob/data/spot_cmds/spot_cmds.txt")) {
        for (i, line) in content.lines().enumerate() {
            if i >= 5 { break; }
            println!("{}", line);
        }
    }
    
    println!("\nFirst 5 lines of generated file:");
    if let Ok(content) = std::fs::read_to_string(std::path::Path::new(&std::env::var("HOME").unwrap_or(".".to_string())).join("ob/data/spot_cmds/spot_cmds2.txt")) {
        for (i, line) in content.lines().enumerate() {
            if i >= 5 { break; }
            println!("{}", line);
        }
    }
}