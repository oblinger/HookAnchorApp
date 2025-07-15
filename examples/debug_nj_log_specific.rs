use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch};

fn main() {
    println!("Debugging specific 'NJ Log' command...");
    
    // Load commands and patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Find the specific NJ Log command
    let nj_log_cmd = commands.iter().find(|cmd| cmd.command == "NJ Log");
    
    if let Some(cmd) = nj_log_cmd {
        println!("Found 'NJ Log' command:");
        println!("  Command: '{}'", cmd.command);
        println!("  Action: '{}'", cmd.action);
        println!("  Arg: '{}'", cmd.arg);
        println!("  Current patch: '{}'", cmd.patch);
        
        // Test what inference would return
        let inferred = infer_patch(cmd, &patches);
        println!("  Inferred patch: {:?}", inferred);
        
        // Manual analysis of why it's getting T instead of NJ
        println!("\nDebugging inference logic:");
        
        // Check first word matching (Method 1)
        if let Some(space_idx) = cmd.command.find(' ') {
            let first_word = &cmd.command[..space_idx];
            let first_word_lower = first_word.to_lowercase();
            println!("  First word: '{}'", first_word);
            println!("  First word lowercase: '{}'", first_word_lower);
            
            if let Some(patch) = patches.get(&first_word_lower) {
                println!("  ✅ NJ patch exists: {:?}", patch.linked_command.as_ref().map(|c| &c.command));
                println!("  This should return 'NJ' patch");
            } else {
                println!("  ❌ No patch exists for 'nj'");
            }
        }
        
        // Check if it's path-based (Method 2)
        if cmd.is_path_based() {
            println!("  Command is path-based, checking file inference...");
            println!("  Path: '{}'", cmd.arg);
        } else {
            println!("  Command is not path-based (action: {})", cmd.action);
        }
        
        // Show what SHOULD happen
        println!("\n🤔 Expected behavior:");
        println!("  Since 'NJ Log' starts with 'NJ' and there's an 'nj' patch,");
        println!("  the inferred patch should be 'NJ', not 'T'.");
        
        if inferred.is_none() {
            println!("  🚨 BUG: infer_patch returned None when it should return Some('NJ')");
        } else if let Some(ref patch_name) = inferred {
            if patch_name != "NJ" {
                println!("  🚨 BUG: infer_patch returned '{}' when it should return 'NJ'", patch_name);
            } else {
                println!("  ✅ infer_patch correctly returns 'NJ'");
                println!("  This confirms the --infer command should work correctly now");
            }
        }
    } else {
        println!("'NJ Log' command not found!");
    }
}