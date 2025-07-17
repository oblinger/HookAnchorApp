use hookanchor::core::commands::{load_commands_raw, create_patches_hashmap, run_patch_inference};

fn main() {
    println!("=== Current Adam Alg State Analysis ===\n");
    
    // Load actual commands from the commands.txt file
    let mut commands = load_commands_raw();
    
    // Find Adam Alg and DL Alg related commands
    println!("Current Adam Alg and DL Alg commands in system:");
    let mut adam_alg_cmd = None;
    let mut dl_alg_commands = Vec::new();
    
    for (i, cmd) in commands.iter().enumerate() {
        if cmd.command.contains("Adam Alg") {
            println!("  [{}] {} -> patch: '{}'", i, cmd.command, cmd.patch);
            if cmd.command == "Adam Alg" {
                adam_alg_cmd = Some(cmd.clone());
            }
        }
        if cmd.command.contains("DL Alg") || cmd.patch == "DL Alg" {
            dl_alg_commands.push((i, cmd.clone()));
        }
    }
    
    if let Some(adam_cmd) = &adam_alg_cmd {
        println!("\nAdam Alg command details:");
        println!("  Command: '{}'", adam_cmd.command);
        println!("  Patch: '{}'", adam_cmd.patch);
        println!("  Action: {}", adam_cmd.action);
        println!("  Arg: {}", adam_cmd.arg);
        println!("  Patch is empty: {}", adam_cmd.patch.is_empty());
    } else {
        println!("\n❌ No 'Adam Alg' command found!");
    }
    
    println!("\nAll DL Alg related commands:");
    for (i, cmd) in &dl_alg_commands {
        println!("  [{}] {} -> patch: '{}' (action: {})", i, cmd.command, cmd.patch, cmd.action);
    }
    
    // Create patches hashmap
    let patches = create_patches_hashmap(&commands);
    
    println!("\nAvailable patches:");
    for (key, patch) in &patches {
        if key.contains("dl") || key.contains("alg") {
            println!("  {} -> {}", key, patch.name);
            if let Some(ref linked_cmd) = patch.linked_command {
                println!("    Linked to: {} ({})", linked_cmd.command, linked_cmd.action);
            }
        }
    }
    
    // Test what would happen if we ran inference again
    println!("\n=== Testing if inference would change anything ===");
    
    let mut test_commands = commands.clone();
    let (assigned, new_patches) = run_patch_inference(
        &mut test_commands, 
        &patches, 
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true
        false  // overwrite_patch = false (only empty patches)
    );
    
    if assigned == 0 {
        println!("✅ No changes needed - all commands already have appropriate patches");
    } else {
        println!("⚠️ {} commands would be changed", assigned);
    }
    
    // Test what would happen with force overwrite
    println!("\n=== Testing force overwrite (to see all potential assignments) ===");
    
    let mut test_commands2 = commands.clone();
    let (assigned2, _new_patches2) = run_patch_inference(
        &mut test_commands2, 
        &patches, 
        false, // apply_changes = false (just analyze)
        false, // print_to_stdout = false (too much output)
        true   // overwrite_patch = true (all commands)
    );
    
    if assigned2 == 0 {
        println!("✅ No changes even with force overwrite - all patches are optimal");
    } else {
        println!("⚠️ {} commands would be changed with force overwrite", assigned2);
        
        // Show what would change for Adam Alg specifically
        if let Some(adam_cmd) = &adam_alg_cmd {
            use hookanchor::core::commands::infer_patch;
            if let Some(inferred) = infer_patch(adam_cmd, &patches) {
                if adam_cmd.patch != inferred {
                    println!("  Adam Alg would change: '{}' -> '{}'", adam_cmd.patch, inferred);
                } else {
                    println!("  Adam Alg would stay the same: '{}'", adam_cmd.patch);
                }
            }
        }
    }
    
    // Check for potential issues
    println!("\n=== Potential Issues Analysis ===");
    
    if let Some(adam_cmd) = &adam_alg_cmd {
        if adam_cmd.patch.is_empty() {
            println!("❌ Adam Alg currently has empty patch - this is the problem!");
        } else if adam_cmd.patch == "DL Alg" {
            println!("✅ Adam Alg already correctly assigned to 'DL Alg' patch");
        } else {
            println!("⚠️ Adam Alg has patch '{}' but might be better assigned elsewhere", adam_cmd.patch);
        }
        
        // Check file existence
        use std::path::Path;
        let file_exists = Path::new(&adam_cmd.arg).exists();
        if !file_exists {
            println!("❌ Adam Alg file does not exist: {}", adam_cmd.arg);
        } else {
            println!("✅ Adam Alg file exists");
        }
    }
}