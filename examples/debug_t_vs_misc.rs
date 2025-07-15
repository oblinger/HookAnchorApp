use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch};

fn main() {
    println!("Debugging T vs Misc patch priority...");
    println!("====================================");
    
    // Load commands and patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Check if T patch exists
    if let Some(t_patch) = patches.get("t") {
        println!("✅ T patch found!");
        if let Some(ref linked_cmd) = t_patch.linked_command {
            println!("  Linked command: '{}' | action: {} | arg: {}", 
                linked_cmd.command, linked_cmd.action, linked_cmd.arg);
                
            // Analyze the T patch path
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
                
                println!("  T patch: file='{}', folder='{}', full_path={}", 
                    file_name, folder_name, file_path);
            }
        }
    } else {
        println!("❌ T patch NOT found");
    }
    
    // Check Misc patch details  
    if let Some(misc_patch) = patches.get("misc") {
        println!("\n✅ MISC patch found!");
        if let Some(ref linked_cmd) = misc_patch.linked_command {
            println!("  Linked command: '{}' | action: {} | arg: {}", 
                linked_cmd.command, linked_cmd.action, linked_cmd.arg);
                
            // Analyze the Misc patch path
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
                
                println!("  MISC patch: file='{}', folder='{}', full_path={}", 
                    file_name, folder_name, file_path);
            }
        }
    }
    
    // Test the Sleep command specifically
    let sleep_cmd = commands.iter()
        .find(|cmd| cmd.command == "Sleep")
        .expect("Sleep command not found");
    
    println!("\n🔍 Analyzing Sleep command inference step by step:");
    println!("  Sleep command: action={}, arg={}", sleep_cmd.action, sleep_cmd.arg);
    
    // Manually walk through inference logic
    println!("\n1. Checking first word match:");
    if let Some(space_idx) = sleep_cmd.command.find(' ') {
        let first_word = &sleep_cmd.command[..space_idx];
        println!("  First word: '{}' (has space)", first_word);
    } else {
        println!("  First word: '{}' (no space, whole command)", sleep_cmd.command);
        let first_word_lower = sleep_cmd.command.to_lowercase();
        if let Some(patch) = patches.get(&first_word_lower) {
            println!("  ✅ Found exact patch match for '{}'", first_word_lower);
        } else {
            println!("  ❌ No patch found for '{}'", first_word_lower);
        }
    }
    
    println!("\n2. Checking path-based inference:");
    if sleep_cmd.action == "obs" || sleep_cmd.action == "anchor" || sleep_cmd.action == "folder" {
        println!("  Command is path-based (action: {})", sleep_cmd.action);
        
        // Show the file path structure
        let path = &sleep_cmd.arg;
        let path_parts: Vec<&str> = path.split('/').collect();
        println!("  Path parts: {:?}", path_parts);
        
        // Check each directory level for patch matches
        for (i, part) in path_parts.iter().enumerate() {
            if !part.is_empty() {
                let part_lower = part.to_lowercase();
                if let Some(_patch) = patches.get(&part_lower) {
                    println!("  ✅ Found patch '{}' at level {} (part: '{}')", part_lower, i, part);
                } else {
                    println!("  ❌ No patch for '{}' at level {} (part: '{}')", part_lower, i, part);
                }
            }
        }
    }
    
    // Final inference result
    let inferred = infer_patch(sleep_cmd, &patches);
    println!("\n🎯 Final inference result: {:?}", inferred);
    
    if let Some(patch_name) = inferred {
        if patch_name == "T" {
            println!("  ⚠️  ISSUE: Getting 'T' instead of 'Misc' for path T/Misc/Sleep.md");
            println!("  Expected: Should get 'Misc' since file is in Misc folder");
        }
    }
}