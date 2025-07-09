use hookanchor::{load_commands, delete_command, save_commands_formatted};

fn main() {
    println!("Testing delete functionality...");
    
    // Load commands
    let mut commands = load_commands();
    let original_count = commands.len();
    println!("Original command count: {}", original_count);
    
    // Find a test command to delete
    let test_command_name = "2000-00 Lean Canvas Note";
    let found_command = commands.iter().find(|cmd| cmd.command == test_command_name);
    
    if let Some(cmd) = found_command {
        println!("Found command to delete: {}", cmd.command);
        println!("Command details: {}", cmd.full_line);
        
        // Delete the command
        let deleted = delete_command(&mut commands, test_command_name);
        println!("Delete operation successful: {}", deleted);
        println!("New command count: {}", commands.len());
        
        if deleted && commands.len() == original_count - 1 {
            println!("✅ Command successfully deleted from memory!");
            
            // Save to test file
            match save_commands_formatted(&commands, "spot_cmds_delete_test.txt") {
                Ok(_) => {
                    println!("✅ Commands saved to test file");
                    
                    // Verify the command is gone from the file
                    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                    let test_file_path = std::path::Path::new(&home).join("ob/data/spot_cmds/spot_cmds_delete_test.txt");
                    
                    if let Ok(contents) = std::fs::read_to_string(&test_file_path) {
                        if !contents.contains(test_command_name) {
                            println!("✅ SUCCESS: Command '{}' successfully deleted from file!", test_command_name);
                        } else {
                            println!("❌ FAILED: Command still found in saved file");
                        }
                        
                        // Clean up test file
                        let _ = std::fs::remove_file(&test_file_path);
                    } else {
                        println!("❌ FAILED: Could not read test file");
                    }
                }
                Err(e) => {
                    println!("❌ Error saving test file: {}", e);
                }
            }
        } else {
            println!("❌ FAILED: Delete operation did not work correctly");
        }
    } else {
        println!("❌ Test command '{}' not found", test_command_name);
    }
}