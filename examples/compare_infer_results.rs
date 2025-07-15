use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch};

fn main() {
    println!("Comparing infer results with new logic...");
    println!("=========================================");
    
    // Load commands and patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Check U flag commands specifically
    println!("Commands with U flag (should NOT appear in --infer output):");
    let u_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.flags.contains('U'))
        .collect();
    
    println!("Found {} commands with U flag", u_commands.len());
    for cmd in u_commands.iter().take(10) {
        let inferred = infer_patch(cmd, &patches);
        println!("  '{}' | current: '{}' | would infer: {:?} | flags: '{}'", 
            cmd.command, cmd.patch, inferred, cmd.flags);
    }
    
    // Look for some specific examples that might show parent directory skipping
    println!("\nLooking for potential parent directory examples:");
    
    // Find commands where inference might suggest moving to parent
    let mut examples_found = 0;
    for cmd in &commands {
        if cmd.flags.contains('U') {
            continue; // Skip user edited
        }
        
        if let Some(inferred) = infer_patch(cmd, &patches) {
            let current = &cmd.patch;
            
            // Look for cases where we're moving from specific to general
            if !current.is_empty() && current != &inferred {
                // Simple heuristic checks for potential parent relationships
                if (current == "FIN" && inferred == "KMR") ||
                   (current == "Misc" && inferred == "T") ||
                   (current == "MED" && inferred == "T") ||
                   (current.len() > inferred.len() && examples_found < 5) {
                    
                    println!("  '{}': {} -> {} (potential parent dir change)", 
                        cmd.command, current, inferred);
                    examples_found += 1;
                }
            }
        }
        
        if examples_found >= 10 { break; }
    }
    
    println!("\nSummary of filtering effects:");
    println!("  Commands with U flag: {}", u_commands.len());
    println!("  These commands will be skipped in --infer output");
    println!("  Parent directory filtering is active for remaining commands");
}