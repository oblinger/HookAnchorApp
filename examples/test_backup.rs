use hookanchor::{load_commands, save_commands_to_file};

fn main() {
    println!("Testing backup functionality...");
    
    // Load existing commands
    let commands = load_commands();
    println!("Loaded {} commands", commands.len());
    
    // Save commands (this should trigger backup)
    match save_commands_to_file(&commands) {
        Ok(_) => println!("Commands saved successfully"),
        Err(e) => println!("Error saving commands: {}", e),
    }
    
    // Check if backup was created
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let backup_dir = std::path::Path::new(&home)
        .join(".config/hookanchor/backups");
    
    if backup_dir.exists() {
        println!("Backup directory created successfully!");
        
        if let Ok(entries) = std::fs::read_dir(&backup_dir) {
            println!("Backup files:");
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  - {}", entry.file_name().to_string_lossy());
                }
            }
        }
    } else {
        println!("No backup directory found");
    }
}