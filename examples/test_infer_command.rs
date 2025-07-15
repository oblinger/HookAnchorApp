use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch, Command};

fn main() {
    println!("Testing --infer command logic with a mock scenario...");
    
    // Load existing commands and patches
    let mut commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Create a test command that would get inferred
    let test_command = Command {
        patch: String::new(), // No current patch
        command: "Test Application Command".to_string(),
        action: "app".to_string(),
        arg: "TestApp".to_string(),
        flags: String::new(),
        full_line: String::new(),
    };
    
    // Add it to our list
    commands.push(test_command);
    
    // Simulate the --infer logic
    let mut changes_found = 0;
    let total_checked = commands.len();
    
    for command in &commands {
        let current_patch = &command.patch;
        
        // Compute what the inferred patch would be
        let inferred_patch = infer_patch(command, &patches);
        
        // Check if they're different (and show example output)
        match inferred_patch {
            Some(new_patch) => {
                if current_patch.is_empty() && !new_patch.is_empty() {
                    // Would add a patch
                    if command.command == "Test Application Command" {
                        println!("{}: (empty) -> {}", command.command, new_patch);
                        changes_found += 1;
                    }
                } else if current_patch != &new_patch {
                    // Would change an existing patch
                    if command.command == "Test Application Command" {
                        println!("{}: {} -> {}", command.command, current_patch, new_patch);
                        changes_found += 1;
                    }
                }
            }
            None => {
                if command.command == "Test Application Command" {
                    println!("{}: would remain (empty)", command.command);
                }
            }
        }
    }
    
    println!("\nExample output format:");
    println!("Commands checked: {}", total_checked);
    println!("Commands that would change: {}", changes_found);
}