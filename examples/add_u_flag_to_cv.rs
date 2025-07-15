use hookanchor::{load_commands_raw, save_commands_to_file};

fn main() {
    println!("Adding 'U' flag to all commands with patch 'CV'...");
    println!("==================================================");
    
    // Load all commands
    let mut commands = load_commands_raw();
    println!("Loaded {} total commands", commands.len());
    
    // Find indices of commands with patch "CV"
    let cv_indices: Vec<usize> = commands.iter()
        .enumerate()
        .filter(|(_, cmd)| cmd.patch == "CV")
        .map(|(index, _)| index)
        .collect();
    
    println!("Found {} commands with patch 'CV':", cv_indices.len());
    
    let mut modified_count = 0;
    
    // Update each CV command to add U flag
    for index in &cv_indices {
        let cmd = &commands[*index];
        println!("  {}: '{}' | action: {} | current flags: '{}'", 
            index + 1, cmd.command, cmd.action, cmd.flags);
        
        // Check if U flag is already present
        if cmd.flags.contains('U') {
            println!("    ⏭ Already has 'U' flag, skipping");
        } else {
            // Add U flag
            let new_flags = if cmd.flags.is_empty() {
                "U".to_string()
            } else {
                format!("{} U", cmd.flags)
            };
            
            // Update the command
            commands[*index].flags = new_flags.clone();
            println!("    ✅ Added 'U' flag: '{}'", new_flags);
            modified_count += 1;
        }
    }
    
    println!("\nSummary:");
    println!("  Total CV commands: {}", cv_indices.len());
    println!("  Commands modified: {}", modified_count);
    println!("  Commands already had U flag: {}", cv_indices.len() - modified_count);
    
    if modified_count > 0 {
        println!("\nSaving updated commands to file...");
        match save_commands_to_file(&commands) {
            Ok(()) => {
                println!("✅ Successfully saved {} commands to file", commands.len());
                println!("✅ Added 'U' flag to {} CV commands", modified_count);
            }
            Err(e) => {
                println!("❌ Failed to save commands: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("\nNo changes needed - all CV commands already have 'U' flag");
    }
}