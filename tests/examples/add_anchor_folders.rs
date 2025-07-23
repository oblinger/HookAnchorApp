use hookanchor::{load_commands_raw, save_commands_to_file, Command};

fn main() {
    println!("Adding/updating anchor folder commands with U flag...");
    println!("=====================================================");
    
    // Define the anchor folder mappings
    let anchor_folders = [
        ("root", "/"),
        ("applications", "/Applications/Utilities"),
        ("oblinger", "/Users/oblinger"),
        ("bin", "/Users/oblinger/bin"),
        ("data", "/Users/oblinger/data"),
        ("timestamps", "/Users/oblinger/ob/data/Timestamps"),
        ("large", "/Users/oblinger/large"),
        ("tofile", "/Users/oblinger/large/_TO_FILE_"),
        ("uploads", "/Users/oblinger/My Drive/Oblio/public-upload"),
        ("ob", "/Users/oblinger/ob"),
        ("kmr", "/Users/oblinger/ob/kmr"),
        ("pkg", "/Users/oblinger/ob/pkg"),
        ("proj", "/Users/oblinger/ob/proj"),
    ];
    
    // Load existing commands
    let mut commands = load_commands_raw();
    println!("Loaded {} total commands", commands.len());
    
    let mut added_count = 0;
    let mut updated_count = 0;
    
    for (command_name, folder_path) in anchor_folders {
        println!("\nProcessing '{}' -> '{}'", command_name, folder_path);
        
        // Check if command already exists
        if let Some(existing_index) = commands.iter().position(|cmd| cmd.command.to_lowercase() == command_name.to_lowercase()) {
            // Command exists - update it
            let existing_cmd = &mut commands[existing_index];
            println!("  Found existing command:");
            println!("    Current: action='{}', arg='{}', flags='{}'", 
                existing_cmd.action, existing_cmd.arg, existing_cmd.flags);
            
            // Update the command
            existing_cmd.action = "folder".to_string();
            existing_cmd.arg = folder_path.to_string();
            
            // Add U flag if not present
            if !existing_cmd.flags.contains('U') {
                existing_cmd.flags = if existing_cmd.flags.is_empty() {
                    "U".to_string()
                } else {
                    format!("{} U", existing_cmd.flags)
                };
            }
            
            // Update full_line
            existing_cmd.full_line = format!("{} : folder {};{}", 
                existing_cmd.command, 
                existing_cmd.arg,
                if existing_cmd.flags.is_empty() { "".to_string() } else { format!(" {}", existing_cmd.flags) }
            );
            
            println!("    Updated: action='{}', arg='{}', flags='{}'", 
                existing_cmd.action, existing_cmd.arg, existing_cmd.flags);
            updated_count += 1;
        } else {
            // Command doesn't exist - create new one
            let new_command = Command {
                patch: String::new(),
                command: command_name.to_string(),
                action: "folder".to_string(),
                arg: folder_path.to_string(),
                flags: "U".to_string(),
                full_line: format!("{} : folder {}; U", command_name, folder_path),
            };
            
            commands.push(new_command);
            println!("  ✅ Created new command with U flag");
            added_count += 1;
        }
    }
    
    println!("\nSummary:");
    println!("  Commands added: {}", added_count);
    println!("  Commands updated: {}", updated_count);
    println!("  Total anchor folders processed: {}", anchor_folders.len());
    
    // Save updated commands
    println!("\nSaving updated commands to file...");
    match save_commands_to_file(&commands) {
        Ok(()) => {
            println!("✅ Successfully saved {} commands to file", commands.len());
            println!("✅ All anchor folder commands now have U flag and are protected from scanner");
        }
        Err(e) => {
            println!("❌ Failed to save commands: {}", e);
            std::process::exit(1);
        }
    }
    
    // Show the final commands for verification
    println!("\nFinal anchor folder commands:");
    for (command_name, _) in anchor_folders {
        if let Some(cmd) = commands.iter().find(|c| c.command.to_lowercase() == command_name.to_lowercase()) {
            println!("  '{}' : {} {}; {}", cmd.command, cmd.action, cmd.arg, cmd.flags);
        }
    }
}