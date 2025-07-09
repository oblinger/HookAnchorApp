use hookanchor::{load_commands, delete_command, add_command, save_commands_to_file, Command};

fn main() {
    println!("Loading commands...");
    let mut commands = load_commands();
    let original_count = commands.len();
    println!("Loaded {} commands", original_count);
    
    // Test 1: Find a command to edit
    let test_command = commands.iter().find(|cmd| cmd.command.contains("2000-00-00 IBM-years Note")).cloned();
    
    if let Some(original_cmd) = test_command {
        println!("\nOriginal command found: {}", original_cmd.command);
        println!("Original action: {}", original_cmd.action);
        println!("Original arg: {}", original_cmd.arg);
        
        // Test 2: Delete the original command
        println!("\nDeleting original command...");
        let deleted = delete_command(&mut commands, &original_cmd.command);
        println!("Deletion successful: {}", deleted);
        println!("Commands after deletion: {}", commands.len());
        
        // Test 3: Create a modified command
        let modified_command = Command {
            group: original_cmd.group.clone(),
            command: original_cmd.command.clone(),
            action: "obs".to_string(), // Keep same action
            arg: format!("{} - MODIFIED", original_cmd.arg), // Add MODIFIED to the arg
            full_line: format!("{}:     obs     {} - MODIFIED", original_cmd.command, original_cmd.arg),
        };
        
        println!("\nAdding modified command...");
        println!("New arg: {}", modified_command.arg);
        add_command(&mut commands, modified_command);
        println!("Commands after addition: {}", commands.len());
        
        // Test 4: Save to a test file first
        println!("\nSaving to test file...");
        match hookanchor::save_commands_formatted(&commands, "spot_cmds_test.txt") {
            Ok(_) => println!("Successfully saved to spot_cmds_test.txt"),
            Err(e) => println!("Error saving: {}", e),
        }
        
        // Verify the modification worked by loading from the test file
        println!("\nVerifying the save worked...");
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let test_file_path = std::path::Path::new(&home).join("ob/data/spot_cmds/spot_cmds_test.txt");
        
        if let Ok(contents) = std::fs::read_to_string(&test_file_path) {
            if contents.contains("MODIFIED") {
                println!("✅ SUCCESS: Command was successfully modified and saved!");
                // Find the line with our modified command
                for line in contents.lines() {
                    if line.contains(&original_cmd.command) && line.contains("MODIFIED") {
                        println!("Modified line: {}", line);
                        break;
                    }
                }
            } else {
                println!("❌ FAILED: Modified command not found in saved file");
            }
        } else {
            println!("❌ FAILED: Could not read test file");
        }
        
    } else {
        println!("Test command not found!");
    }
}