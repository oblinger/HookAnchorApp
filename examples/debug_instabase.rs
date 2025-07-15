use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch, Command};

fn main() {
    println!("Debugging Instabase command patch inference...");
    
    // Load commands and create patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Find the Instabase command
    let instabase_cmd = commands.iter().find(|cmd| cmd.command == "Instabase");
    
    if let Some(cmd) = instabase_cmd {
        println!("Found Instabase command:");
        println!("  Command: '{}'", cmd.command);
        println!("  Action: '{}'", cmd.action);
        println!("  Arg: '{}'", cmd.arg);
        println!("  Current patch: '{}'", cmd.patch);
        
        // Test inference
        let inferred = infer_patch(cmd, &patches);
        println!("  Inferred patch: {:?}", inferred);
    } else {
        println!("Instabase command not found");
    }
    
    // Look for T-related patches
    println!("\nLooking for T-related patches:");
    for (key, patch) in patches.iter() {
        if key.contains("t") || key == "t" {
            println!("  '{}' → {:?}", key, patch.linked_command.as_ref().map(|c| &c.command));
        }
    }
    
    // Look for the T command specifically
    println!("\nLooking for command named 'T':");
    for cmd in &commands {
        if cmd.command.to_lowercase() == "t" {
            println!("  Found T command: '{}' : {} ; {} (patch: '{}')", cmd.command, cmd.action, cmd.arg, cmd.patch);
        }
    }
    
    // Check path components
    println!("\nAnalyzing Instabase path:");
    if let Some(cmd) = instabase_cmd {
        let path_components: Vec<&str> = cmd.arg.split('/').collect();
        for (i, component) in path_components.iter().enumerate() {
            println!("  [{}]: '{}'", i, component);
        }
        
        // Check if T is in the path
        if cmd.arg.contains("/T/") {
            println!("\n✅ Path contains '/T/' directory");
            
            // Check what's preventing the inference
            println!("\nDiagnosing why T patch isn't inferred:");
            
            // The current logic only checks first component for relative paths
            // But this is an absolute path, so it won't match on first component
            if cmd.arg.starts_with('/') {
                println!("  ❌ Path is absolute, not relative - first component check won't apply");
            }
            
            // Check if we're looking for directory matches
            println!("  🔍 Would need to check if any patch's linked command is in a parent directory");
        }
    }
}