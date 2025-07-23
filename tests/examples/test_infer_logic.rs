use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch};

fn main() {
    println!("Testing updated --infer logic...");
    println!("================================");
    
    // Load commands and patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Test 1: Check that U flag commands are being identified
    println!("1. Commands with U flag (should be skipped):");
    let u_flag_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.flags.contains('U'))
        .take(10)
        .collect();
    
    for cmd in u_flag_commands {
        println!("  '{}' | patch: '{}' | flags: '{}'", cmd.command, cmd.patch, cmd.flags);
    }
    
    // Test 2: Find examples where inference would suggest a parent directory
    println!("\n2. Testing parent directory logic:");
    
    // Look for commands that might have this issue
    let test_commands = [
        ("FIN", "KMR"), // FIN might be under KMR directory
        ("Misc", "T"),   // Misc might be under T directory  
        ("MED", "T"),    // MED might be under T directory
    ];
    
    for (current, expected_parent) in test_commands {
        if let (Some(current_patch), Some(parent_patch)) = (patches.get(&current.to_lowercase()), patches.get(&expected_parent.to_lowercase())) {
            println!("  Checking {} vs {}: ", current, expected_parent);
            
            if let (Some(ref current_cmd), Some(ref parent_cmd)) = (&current_patch.linked_command, &parent_patch.linked_command) {
                println!("    {} linked to: {}", current, current_cmd.arg);
                println!("    {} linked to: {}", expected_parent, parent_cmd.arg);
                
                // Check if this would be considered a parent directory change
                let is_parent = crate::cmd::is_parent_directory_patch(current, expected_parent, &patches);
                println!("    Is {} parent of {}? {}", expected_parent, current, is_parent);
            }
        }
    }
    
    // Test 3: Count total changes with new logic vs old logic
    println!("\n3. Comparing old vs new logic:");
    
    let mut old_logic_changes = 0;
    let mut new_logic_changes = 0;
    let mut skipped_user_edited = 0;
    let mut skipped_parent_dir = 0;
    
    for command in &commands {
        let current_patch = &command.patch;
        let inferred_patch = infer_patch(command, &patches);
        
        // Old logic: count all changes
        if let Some(ref new_patch) = inferred_patch {
            if (current_patch.is_empty() && !new_patch.is_empty()) || 
               (current_patch != new_patch) {
                old_logic_changes += 1;
            }
        }
        
        // New logic: apply filters
        if command.flags.contains('U') {
            skipped_user_edited += 1;
            continue;
        }
        
        if let Some(new_patch) = inferred_patch {
            let should_change = if current_patch.is_empty() && !new_patch.is_empty() {
                true
            } else if current_patch != &new_patch {
                // This would need access to the helper function - let's estimate instead
                if !current_patch.is_empty() {
                    // Simple heuristic: if new patch is shorter, it might be a parent
                    if new_patch.len() < current_patch.len() && current_patch.starts_with(&new_patch) {
                        skipped_parent_dir += 1;
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            } else {
                false
            };
            
            if should_change {
                new_logic_changes += 1;
            }
        }
    }
    
    println!("  Old logic would change: {} commands", old_logic_changes);
    println!("  New logic would change: {} commands", new_logic_changes);
    println!("  Skipped (user edited): {}", skipped_user_edited);
    println!("  Skipped (parent dir estimate): {}", skipped_parent_dir);
    println!("  Reduction: {} commands", old_logic_changes - new_logic_changes);
}