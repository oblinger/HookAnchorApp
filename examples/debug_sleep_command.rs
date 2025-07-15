use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch};

fn main() {
    println!("Debugging Sleep command patch inference...");
    println!("==========================================");
    
    // Load commands and patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Find Sleep commands
    let sleep_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.command.to_lowercase().contains("sleep"))
        .collect();
    
    println!("Found {} commands containing 'sleep':", sleep_commands.len());
    for (i, cmd) in sleep_commands.iter().enumerate() {
        println!("  {}: '{}' | action: {} | arg: {} | current patch: '{}'", 
            i + 1, cmd.command, cmd.action, cmd.arg, cmd.patch);
    }
    
    // Check if MISC patch exists
    println!("\nChecking for MISC patch...");
    if let Some(misc_patch) = patches.get("misc") {
        println!("✅ MISC patch found!");
        if let Some(ref linked_cmd) = misc_patch.linked_command {
            println!("  Linked command: '{}' | action: {} | arg: {}", 
                linked_cmd.command, linked_cmd.action, linked_cmd.arg);
        }
    } else {
        println!("❌ MISC patch NOT found");
        
        // Look for commands that might create a MISC patch
        let misc_candidates: Vec<_> = commands.iter()
            .filter(|cmd| cmd.action == "anchor" && cmd.command.to_lowercase() == "misc")
            .collect();
        
        if misc_candidates.is_empty() {
            println!("  No anchor commands with command name 'MISC' found");
            
            // Look for any commands with MISC in the path
            let misc_path_commands: Vec<_> = commands.iter()
                .filter(|cmd| cmd.arg.to_lowercase().contains("misc"))
                .take(5)
                .collect();
            
            if !misc_path_commands.is_empty() {
                println!("  Commands with 'misc' in path:");
                for cmd in misc_path_commands {
                    println!("    '{}' | action: {} | arg: {}", cmd.command, cmd.action, cmd.arg);
                }
            }
        } else {
            println!("  Found {} anchor commands with name 'MISC':", misc_candidates.len());
            for cmd in misc_candidates {
                println!("    '{}' | arg: {}", cmd.command, cmd.arg);
                
                // Check if this would create a patch
                let file_path = &cmd.arg;
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
                    
                    println!("      file: '{}', folder: '{}', match: {}", 
                        file_name, folder_name, file_name.to_lowercase() == folder_name.to_lowercase());
                }
            }
        }
    }
    
    // Now check what inference returns for each Sleep command
    println!("\nInference results for Sleep commands:");
    for cmd in &sleep_commands {
        let inferred = infer_patch(cmd, &patches);
        println!("  '{}' -> inferred: {:?}", cmd.command, inferred);
        
        // Show detailed analysis
        if cmd.action == "obs" || cmd.action == "anchor" {
            println!("    Path: {}", cmd.arg);
            // Show what directory this is in
            if let Some(last_slash) = cmd.arg.rfind('/') {
                let dir_path = &cmd.arg[..last_slash];
                if let Some(parent_slash) = dir_path.rfind('/') {
                    let parent_dir = &dir_path[parent_slash + 1..];
                    println!("    Parent directory: '{}'", parent_dir);
                }
            }
        }
    }
    
    // Show all available patches for reference
    println!("\nAll available patches (first 20):");
    let mut patch_names: Vec<_> = patches.keys().collect();
    patch_names.sort();
    for (i, name) in patch_names.iter().take(20).enumerate() {
        println!("  {}: '{}'", i + 1, name);
    }
    if patch_names.len() > 20 {
        println!("  ... and {} more", patch_names.len() - 20);
    }
}