use hookanchor::{load_commands_raw, create_patches_hashmap};

fn main() {
    println!("Testing new patch detection logic...");
    println!("=====================================");
    
    // Load commands
    let commands = load_commands_raw();
    println!("Loaded {} total commands", commands.len());
    
    // Create patches using new logic (only anchor commands where file name matches folder)
    let patches = create_patches_hashmap(&commands);
    println!("Found {} patches with new logic", patches.len());
    
    // Show all detected patches
    if !patches.is_empty() {
        println!("\nDetected patches:");
        let mut patch_names: Vec<_> = patches.keys().collect();
        patch_names.sort();
        
        for patch_name in patch_names {
            let patch = &patches[patch_name];
            if let Some(ref linked_cmd) = patch.linked_command {
                println!("  '{}' -> command: '{}' | action: {} | file: {}", 
                    patch_name, linked_cmd.command, linked_cmd.action, linked_cmd.arg);
                
                // Show the file/folder name analysis
                let file_path = &linked_cmd.arg;
                if let Some(last_slash) = file_path.rfind('/') {
                    let file_part = &file_path[last_slash + 1..];
                    let file_name = if let Some(dot_pos) = file_part.rfind('.') {
                        &file_part[..dot_pos]
                    } else {
                        file_part
                    };
                    
                    let folder_part = &file_path[..last_slash];
                    let folder_name = if let Some(parent_slash) = folder_part.rfind('/') {
                        &folder_part[parent_slash + 1..]
                    } else {
                        folder_part
                    };
                    
                    println!("    -> file: '{}', folder: '{}' (match: {})", 
                        file_name, folder_name, file_name.to_lowercase() == folder_name.to_lowercase());
                }
            }
        }
    } else {
        println!("\nNo patches detected with new logic!");
    }
    
    // Count anchor commands to understand the candidate pool (anchors identified by 'A' flag)
    let anchor_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.is_anchor())
        .collect();
    println!("\nTotal anchor commands (with A flag): {}", anchor_commands.len());

    // Show some example anchor commands to understand the data
    println!("\nExample anchor commands:");
    for (i, cmd) in anchor_commands.iter().take(10).enumerate() {
        println!("  {}: '{}' -> {}", i + 1, cmd.command, cmd.arg);
    }

    if anchor_commands.len() > 10 {
        println!("  ... and {} more", anchor_commands.len() - 10);
    }
}