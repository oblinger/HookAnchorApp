use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch, Command};

fn main() {
    println!("Debugging ANOVA command patch inference...");
    
    // Load commands and create patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Find the ANOVA command
    let anova_cmd = commands.iter().find(|cmd| cmd.command == "ANOVA");
    
    if let Some(cmd) = anova_cmd {
        println!("Found ANOVA command:");
        println!("  Command: '{}'", cmd.command);
        println!("  Action: '{}'", cmd.action);
        println!("  Arg: '{}'", cmd.arg);
        println!("  Current patch: '{}'", cmd.patch);
        
        // Test inference
        let inferred = infer_patch(cmd, &patches);
        println!("  Inferred patch: {:?}", inferred);
    } else {
        println!("ANOVA command not found");
    }
    
    // Look for any RR-related patches
    println!("\nLooking for RR-related patches:");
    for (key, patch) in patches.iter() {
        if key.contains("rr") || key.to_uppercase().contains("RR") {
            println!("  '{}' → {:?}", key, patch.linked_command.as_ref().map(|c| &c.command));
        }
    }
    
    // Look for any commands with RR in their arg
    println!("\nCommands with RR/ in their path (first 10):");
    let mut count = 0;
    for cmd in &commands {
        if cmd.arg.starts_with("RR/") {
            println!("  '{}' : {} ; {} (patch: '{}')", cmd.command, cmd.action, cmd.arg, cmd.patch);
            count += 1;
            if count >= 10 { break; }
        }
    }
    
    // Check if there's a command named "RR"
    println!("\nLooking for command named 'RR':");
    for cmd in &commands {
        if cmd.command.to_lowercase() == "rr" {
            println!("  Found RR command: '{}' : {} ; {} (patch: '{}')", cmd.command, cmd.action, cmd.arg, cmd.patch);
        }
    }
}