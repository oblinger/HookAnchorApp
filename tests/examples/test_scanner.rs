use hookanchor::{load_commands, scanner::{scan, scan_contacts}};
use std::time::Instant;

fn main() {
    println!("Testing markdown scanner...\n");
    
    // Load current commands
    let commands = load_commands();
    println!("Current command count: {}", commands.len());
    
    // Count existing obs/anchor commands (anchors identified by 'A' flag)
    let existing_obs_anchor = commands.iter()
        .filter(|cmd| cmd.action == "obs" || cmd.is_anchor())
        .count();
    println!("Existing obs/anchor commands: {}", existing_obs_anchor);
    
    // Test markdown roots - using current project directory which has markdown files
    let test_roots = vec![
        ".".to_string(),
        "~/Documents".to_string(),
    ];
    
    // Test contact scanning first
    println!("Testing contact scanning...");
    let contact_start = Instant::now();
    let commands_with_contacts = scan_contacts(commands.clone());
    let contact_duration = contact_start.elapsed();
    
    let new_contacts = commands_with_contacts.iter()
        .filter(|cmd| cmd.action == "contact")
        .count();
    
    println!("Contact scan completed in: {:.2} seconds", contact_duration.as_secs_f64());
    println!("Found {} contacts\n", new_contacts);
    
    if new_contacts > 0 {
        println!("First 5 contacts:");
        for cmd in commands_with_contacts.iter()
            .filter(|cmd| cmd.action == "contact")
            .take(5) {
            println!("  {}", cmd.command);
        }
        println!();
    }

    // Scan files and contacts with timing
    let start_time = Instant::now();
    let updated_commands = scan(commands.clone(), &test_roots);
    let scan_duration = start_time.elapsed();
    
    println!("\nAfter scanning:");
    println!("Scan completed in: {:.2} seconds", scan_duration.as_secs_f64());
    println!("Updated command count: {}", updated_commands.len());
    
    // Count new obs/anchor commands (anchors identified by 'A' flag)
    let new_obs_anchor = updated_commands.iter()
        .filter(|cmd| cmd.action == "obs" || cmd.is_anchor())
        .count();
    let final_contacts = updated_commands.iter()
        .filter(|cmd| cmd.action == "contact")
        .count();
    println!("New obs/anchor commands: {}", new_obs_anchor);
    println!("Total contact commands: {}", final_contacts);
    
    let net_change = updated_commands.len() as i32 - commands.len() as i32;
    println!("Net change in total commands: {}", net_change);
    
    // Calculate scanning rate
    if scan_duration.as_secs_f64() > 0.0 {
        let files_per_second = new_obs_anchor as f64 / scan_duration.as_secs_f64();
        println!("Scanning rate: {:.0} files/second", files_per_second);
    }
    
    // Show some of the new commands
    let new_commands: Vec<_> = updated_commands.iter()
        .filter(|cmd| cmd.command.ends_with(" *"))
        .take(10)
        .collect();
    
    if !new_commands.is_empty() {
        println!("\nFirst 10 markdown commands found:");
        for cmd in new_commands {
            println!("  {} -> {} {}", cmd.command, cmd.action, cmd.arg);
        }
    } else {
        println!("\nNo markdown files found in the specified directories.");
    }
    
    // Check for anchor files (anchors identified by 'A' flag)
    let anchor_commands: Vec<_> = updated_commands.iter()
        .filter(|cmd| cmd.is_anchor())
        .take(5)
        .collect();

    if !anchor_commands.is_empty() {
        println!("\nFound {} anchor files:", anchor_commands.len());
        for cmd in anchor_commands {
            println!("  {} -> {}", cmd.command, cmd.arg);
        }
    }
}