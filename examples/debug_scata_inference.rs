use hookanchor::{load_data};

fn main() {
    println!("Debugging SCATA command patch inference...");
    
    // Load data
    let (_config, commands, patches) = load_data();
    
    // Find the SCATA command
    let scata_command = commands.iter()
        .find(|cmd| cmd.command.contains("2009-05 SCATA Security"));
    
    if let Some(cmd) = scata_command {
        println!("Found SCATA command:");
        println!("  Command: {}", cmd.command);
        println!("  Current patch: '{}'", cmd.patch);
        println!("  Action: {}", cmd.action);
        println!("  Arg: {}", cmd.arg);
        
        // Test patch inference on this command
        if let Some(inferred) = hookanchor::core::commands::infer_patch(cmd, &patches) {
            println!("  Inferred patch: '{}'", inferred);
            
            // Check if this differs from current patch
            if cmd.patch != inferred {
                println!("  ⚠️  Patch would change from '{}' to '{}'", cmd.patch, inferred);
            } else {
                println!("  ✅ Patch matches current value");
            }
        } else {
            println!("  No patch could be inferred");
        }
        
        // Show some context about the path
        println!("\n--- Path Analysis ---");
        println!("File path: {}", cmd.arg);
        
        // Check what patches exist for "Log"
        if let Some(log_patch) = patches.get("log") {
            println!("\n--- Log Patch Info ---");
            println!("Log patch name: {}", log_patch.name);
            if let Some(ref linked_cmd) = log_patch.linked_command {
                println!("Log linked command: {} -> {}", linked_cmd.command, linked_cmd.arg);
            }
        }
        
        // Check what patches exist for the current patch
        let current_patch_key = cmd.patch.to_lowercase();
        if let Some(current_patch) = patches.get(&current_patch_key) {
            println!("\n--- Current Patch ('{}') Info ---", cmd.patch);
            println!("Patch name: {}", current_patch.name);
            if let Some(ref linked_cmd) = current_patch.linked_command {
                println!("Linked command: {} -> {}", linked_cmd.command, linked_cmd.arg);
            }
        } else {
            println!("\n--- Current Patch ('{}') Not Found ---", cmd.patch);
        }
        
    } else {
        println!("SCATA command not found!");
        
        // Show some commands that contain SCATA or DARPA
        println!("\nSearching for related commands...");
        for cmd in commands.iter() {
            if cmd.command.to_lowercase().contains("scata") || 
               cmd.command.to_lowercase().contains("darpa") {
                println!("  {} (patch: '{}') -> {}", cmd.command, cmd.patch, cmd.arg);
            }
        }
    }
}