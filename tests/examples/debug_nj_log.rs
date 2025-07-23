use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch};

fn main() {
    println!("Debugging NJ LOG command...");
    
    // Load commands and patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Look for NJ LOG command
    let nj_log_cmd = commands.iter().find(|cmd| cmd.command.contains("NJ") && cmd.command.contains("LOG"));
    
    if let Some(cmd) = nj_log_cmd {
        println!("Found command: '{}'", cmd.command);
        println!("  Action: '{}'", cmd.action);
        println!("  Arg: '{}'", cmd.arg);
        println!("  Current patch: '{}'", cmd.patch);
        
        // Test inference
        let inferred = infer_patch(cmd, &patches);
        println!("  Inferred patch: {:?}", inferred);
        
        // Check if there's an NJ patch
        if let Some(nj_patch) = patches.get("nj") {
            println!("  NJ patch exists: {:?}", nj_patch.linked_command.as_ref().map(|c| &c.command));
        } else {
            println!("  NJ patch does not exist");
        }
        
        // Show first word analysis
        if let Some(space_idx) = cmd.command.find(' ') {
            let first_word = &cmd.command[..space_idx];
            println!("  First word: '{}'", first_word);
            println!("  First word lowercase: '{}'", first_word.to_lowercase());
            
            if let Some(patch) = patches.get(&first_word.to_lowercase()) {
                println!("  Patch exists for first word: {:?}", patch.linked_command.as_ref().map(|c| &c.command));
            } else {
                println!("  No patch exists for first word");
            }
        }
    } else {
        // Let's search more broadly
        println!("Searching for any command containing 'NJ' and 'LOG'...");
        for cmd in &commands {
            if cmd.command.to_uppercase().contains("NJ") && cmd.command.to_uppercase().contains("LOG") {
                println!("Found: '{}' (patch: '{}')", cmd.command, cmd.patch);
            }
        }
        
        // Also search for just NJ commands
        println!("\nSearching for commands starting with 'NJ'...");
        let mut count = 0;
        for cmd in &commands {
            if cmd.command.starts_with("NJ") {
                println!("  '{}' (patch: '{}')", cmd.command, cmd.patch);
                count += 1;
                if count >= 5 { break; }
            }
        }
    }
}