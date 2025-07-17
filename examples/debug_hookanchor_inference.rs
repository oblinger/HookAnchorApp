use hookanchor::{load_data, infer_patch};

fn main() {
    println!("Debugging HookAnchor command patch inference...");
    
    // Load data
    let (_config, commands, patches) = load_data();
    
    // Find the HookAnchor command
    let hookanchor_command = commands.iter()
        .find(|cmd| cmd.command == "2025-06 HookAnchor");
    
    if let Some(cmd) = hookanchor_command {
        println!("Found command:");
        println!("  Command: {}", cmd.command);
        println!("  Current patch: '{}'", cmd.patch);
        println!("  Action: {}", cmd.action);
        println!("  Arg: {}", cmd.arg);
        
        // Check if it's path-based
        println!("  Is path-based: {}", cmd.is_path_based());
        
        // Test each inference method individually
        println!("\n--- Testing Individual Inference Methods ---");
        
        // Method 1: Alias (should not apply)
        println!("1. Alias inference: N/A (action != 'alias')");
        
        // Method 2: Path-based inference  
        println!("2. Path-based inference:");
        println!("   Is path-based: {}", cmd.is_path_based());
        
        // Debug the path components
        let components: Vec<&str> = cmd.arg.split('/').collect();
        println!("   Path components: {:?}", components);
        
        // Check which patches exist in the system
        println!("   Available patch keys:");
        for patch_key in patches.keys() {
            println!("     '{}'", patch_key);
        }
        
        // Check which components match patches
        println!("   Component matches:");
        for (depth, component) in components.iter().enumerate() {
            let component_lower = component.to_lowercase();
            if patches.contains_key(&component_lower) {
                println!("     Depth {}: '{}' -> patch found", depth, component);
            }
        }
        
        // Method 3: First word matching
        println!("3. First word matching:");
        let first_word = cmd.command.split_whitespace().next().unwrap_or("");
        println!("   First word: '{}'", first_word);
        
        // Check for year pattern
        println!("4. Year pattern check:");
        if first_word.len() == 4 && first_word.chars().all(|c| c.is_ascii_digit()) {
            let year: i32 = first_word.parse().unwrap_or(0);
            if year >= 2000 && year <= 3000 {
                println!("   Would match year patch: '{}'", first_word);
            }
        }
        
        // Test overall inference
        println!("\n--- Overall Inference Result ---");
        let result = infer_patch(cmd, &patches);
        println!("Final result: {:?}", result);
        
        // Show available patches in the path
        println!("\n--- Available Patches Along Path ---");
        use std::path::Path;
        let file_path = Path::new(&cmd.arg);
        let mut current_path = file_path.parent();
        let mut level = 0;
        
        while let Some(path) = current_path {
            level += 1;
            println!("Level {}: {:?}", level, path);
            
            // Check for matching patches
            for (patch_key, patch) in &patches {
                if let Some(ref linked_cmd) = patch.linked_command {
                    if linked_cmd.is_path_based() {
                        let linked_path = Path::new(&linked_cmd.arg);
                        let linked_dir = if linked_path.is_file() || linked_cmd.arg.contains('.') {
                            linked_path.parent()
                        } else {
                            Some(linked_path)
                        };
                        
                        if let Some(linked_dir) = linked_dir {
                            if path == linked_dir {
                                println!("  -> MATCHES patch '{}' ({})", patch.name, linked_cmd.command);
                            }
                        }
                    }
                }
            }
            
            current_path = path.parent();
            if level > 10 { break; } // Prevent infinite loop
        }
        
    } else {
        println!("HookAnchor command not found!");
    }
}