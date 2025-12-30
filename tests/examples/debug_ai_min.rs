use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch};

fn main() {
    println!("Debugging AI Min command inference...");
    println!("====================================");
    
    // Load commands and patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Find AI Min command
    let ai_min_cmd = commands.iter()
        .find(|cmd| cmd.command == "AI Min")
        .expect("AI Min command not found");
    
    println!("Found AI Min command:");
    println!("  Command: '{}'", ai_min_cmd.command);
    println!("  Action: '{}'", ai_min_cmd.action);
    println!("  Arg: '{}'", ai_min_cmd.arg);
    println!("  Current patch: '{}'", ai_min_cmd.patch);
    
    // Check if AI patch exists
    println!("\nChecking for AI patch...");
    if let Some(ai_patch) = patches.get("ai") {
        println!("✅ AI patch found!");
        if let Some(ref linked_cmd) = ai_patch.linked_command {
            println!("  Linked command: '{}' | action: {} | arg: {}", 
                linked_cmd.command, linked_cmd.action, linked_cmd.arg);
                
            // Analyze the AI patch creation
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
                
                println!("  AI patch created from: file='{}', folder='{}', match={}", 
                    file_name, folder_name, file_name.to_lowercase() == folder_name.to_lowercase());
            }
        }
    } else {
        println!("❌ AI patch NOT found");
    }
    
    // Step through inference logic for AI Min
    println!("\n🔍 Step-by-step inference for 'AI Min':");
    
    // Check first word matching (Method 1)
    println!("1. First word matching:");
    if let Some(space_idx) = ai_min_cmd.command.find(' ') {
        let first_word = &ai_min_cmd.command[..space_idx];
        let first_word_lower = first_word.to_lowercase();
        println!("  First word: '{}' (lowercase: '{}')", first_word, first_word_lower);
        
        if let Some(_patch) = patches.get(&first_word_lower) {
            println!("  ✅ Found patch match for first word '{}'!", first_word_lower);
            println!("  This explains why 'AI Min' gets 'AI' patch - first word matching!");
        } else {
            println!("  ❌ No patch found for first word '{}'", first_word_lower);
        }
    } else {
        println!("  No space found in command, whole command is first word");
    }
    
    // Check if it's path-based (Method 2)
    println!("\n2. Path-based inference:");
    if ai_min_cmd.action == "obs" || ai_min_cmd.is_anchor() || ai_min_cmd.action == "folder" {
        println!("  Command is path-based (action: {})", ai_min_cmd.action);
        println!("  Path: {}", ai_min_cmd.arg);
    } else {
        println!("  Command is NOT path-based (action: {})", ai_min_cmd.action);
        println!("  So path-based inference won't apply");
    }
    
    // Get final inference result
    let inferred = infer_patch(ai_min_cmd, &patches);
    println!("\n🎯 Final inference result: {:?}", inferred);
    
    // Show some other AI-related commands to understand the pattern
    println!("\nOther commands starting with 'AI':");
    let ai_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.command.to_lowercase().starts_with("ai "))
        .take(10)
        .collect();
        
    for cmd in ai_commands {
        let inferred = infer_patch(cmd, &patches);
        println!("  '{}' | action: {} | current: '{}' | inferred: {:?}", 
            cmd.command, cmd.action, cmd.patch, inferred);
    }
}