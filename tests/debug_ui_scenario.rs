use anchor_selector::{load_commands, filter_commands, merge_similar_commands, get_current_submenu_prefix, split_commands};

#[test]
fn debug_ui_scenario_step_by_step() {
    println!("\n=== Debug UI Scenario Step by Step ===");
    
    // Simulate exact UI flow: type "fin" and process
    let search_text = "fin";
    
    // Step 1: Load commands 
    let all_commands = load_commands();
    println!("1. Loaded {} commands", all_commands.len());
    
    // Step 2: Filter commands (like UI does)
    let filtered = filter_commands(&all_commands, search_text, 10, false);
    println!("2. After filtering for '{}': {} commands", search_text, filtered.len());
    
    // Step 3: Apply merge (should be no-op since disabled)
    let merged = merge_similar_commands(&filtered, search_text);
    println!("3. After merging: {} commands", merged.len());
    
    // Step 4: Check for submenu prefix
    let submenu_prefix = get_current_submenu_prefix(&merged, search_text);
    println!("4. Submenu prefix: {:?}", submenu_prefix);
    
    // Step 5: Split commands (if there's a submenu)
    if let Some(prefix) = submenu_prefix {
        let (inside_menu, outside_menu) = split_commands(&merged, &prefix);
        
        println!("5. Split results:");
        println!("   Inside menu: {} commands", inside_menu.len());
        println!("   Outside menu: {} commands", outside_menu.len());
        
        // Look for Findem specifically
        println!("\n=== Findem Analysis ===");
        
        // Check if Findem is in inside menu (WRONG)
        for (i, cmd) in inside_menu.iter().enumerate() {
            if cmd.command.contains("Findem") {
                println!("❌ FOUND IN INSIDE MENU [{}]: '{}'", i, cmd.command);
                
                // Simulate the trimming logic that would happen in display
                if cmd.command.len() > prefix.len() {
                    let prefix_end = prefix.len();
                    if cmd.command[..prefix_end].eq_ignore_ascii_case(&prefix) {
                        if let Some(ch) = cmd.command.chars().nth(prefix_end) {
                            if ch == ' ' || ch == '.' || ch == '_' {
                                let trimmed = &cmd.command[prefix_end + 1..];
                                println!("   → Would be displayed as: '{}'", trimmed);
                            } else {
                                println!("   → No separator after prefix, would show full: '{}'", cmd.command);
                                
                                // This is the bug! "FINDEM" starts with "FIN" but has no separator
                                // Yet it's being put in inside menu and might get trimmed incorrectly
                                if cmd.command.to_uppercase().starts_with("FIN") {
                                    let after_fin = &cmd.command[3..];
                                    println!("   → BUG: This would incorrectly trim to: '{}...'", after_fin.to_lowercase());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Check if Findem is in outside menu (CORRECT)
        for (i, cmd) in outside_menu.iter().enumerate() {
            if cmd.command.contains("Findem") {
                println!("✅ FOUND IN OUTSIDE MENU [{}]: '{}'", i, cmd.command);
                println!("   → Would be displayed as: '{}' (full name)", cmd.command);
            }
        }
        
        // Debug the split logic for "Findem" specifically
        println!("\n=== Debugging Split Logic for Findem Commands ===");
        for cmd in &merged {
            if cmd.command.contains("Findem") {
                println!("Analyzing: '{}'", cmd.command);
                
                // Replicate split_commands logic
                if cmd.command.len() > prefix.len() {
                    let prefix_end = prefix.len();
                    println!("  prefix_end: {}", prefix_end);
                    
                    if cmd.command[..prefix_end].eq_ignore_ascii_case(&prefix) {
                        println!("  ✓ Starts with prefix '{}'", prefix);
                        
                        if let Some(ch) = cmd.command.chars().nth(prefix_end) {
                            println!("  Character at position {}: '{}'", prefix_end, ch);
                            if ch == ' ' || ch == '.' || ch == '_' {
                                println!("  → INSIDE menu (has separator)");
                            } else {
                                println!("  → OUTSIDE menu (no separator) - character '{}' is not a separator", ch);
                            }
                        } else {
                            println!("  → No character at prefix_end");
                        }
                    } else {
                        println!("  ✗ Does not start with prefix '{}'", prefix);
                        println!("  → OUTSIDE menu");
                    }
                } else if cmd.command.eq_ignore_ascii_case(&prefix) {
                    println!("  → INSIDE menu (exact match)");
                } else {
                    println!("  → OUTSIDE menu (shorter than prefix)");
                }
                println!();
            }
        }
    } else {
        println!("5. No submenu detected");
    }
}