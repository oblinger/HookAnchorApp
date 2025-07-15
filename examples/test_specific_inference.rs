use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch, Command};

fn main() {
    println!("Testing specific patch inference cases...");
    
    // Load commands and create patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    println!("Loaded {} commands and {} patches", commands.len(), patches.len());
    
    // Test cases for patch inference
    let test_commands = vec![
        // Test case 1: First word matches existing patch
        Command {
            patch: String::new(),
            command: "Application Test Command".to_string(),
            action: "app".to_string(),
            arg: "TestApp".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        // Test case 2: Command without matching prefix
        Command {
            patch: String::new(),
            command: "Unique Random Command".to_string(),
            action: "app".to_string(),
            arg: "SomeApp".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        // Test case 3: Single word command
        Command {
            patch: String::new(),
            command: "Terminal".to_string(),
            action: "app".to_string(),
            arg: "Terminal".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
    ];
    
    println!("\nTesting inference on specific commands:");
    for (i, cmd) in test_commands.iter().enumerate() {
        let inferred = infer_patch(cmd, &patches);
        println!("Test {}: '{}' → {}", 
                i + 1, 
                cmd.command, 
                inferred.map_or("None".to_string(), |p| format!("'{}'", p)));
    }
    
    // Show some existing patches for reference
    println!("\nExisting patches (first 20):");
    for (i, (key, patch)) in patches.iter().enumerate() {
        if i >= 20 { break; }
        let linked = if let Some(ref cmd) = patch.linked_command {
            format!("→ '{}'", cmd.command)
        } else {
            "→ None".to_string()
        };
        println!("  '{}' {}", key, linked);
    }
}