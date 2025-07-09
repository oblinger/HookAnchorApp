use hookanchor::{load_commands, scanner::scan_contacts};
use std::time::Instant;

fn main() {
    println!("Testing contact scanning only...\n");
    
    let commands = load_commands();
    println!("Starting with {} commands", commands.len());
    
    let start_time = Instant::now();
    let updated_commands = scan_contacts(commands);
    let duration = start_time.elapsed();
    
    println!("Contact scan completed in: {:.2} seconds", duration.as_secs_f64());
    
    let contact_count = updated_commands.iter()
        .filter(|cmd| cmd.action == "contact")
        .count();
    
    println!("Found {} contact commands", contact_count);
    
    if contact_count > 0 {
        println!("\nFirst 5 contacts:");
        for cmd in updated_commands.iter()
            .filter(|cmd| cmd.action == "contact")
            .take(5) {
            println!("  {}", cmd.command);
        }
    }
}