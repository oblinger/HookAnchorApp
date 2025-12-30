use hookanchor::core::{load_commands_with_data, infer_patch, Command};

fn main() {
    println!("Debugging meditation command patch inference...\n");
    
    // Load all data
    let (_config, commands, patches) = load_commands_with_data();
    
    // Find all meditation-related commands
    let meditation_commands: Vec<&Command> = commands.iter()
        .filter(|cmd| cmd.command.to_lowercase().contains("meditation"))
        .collect();
    
    println!("Found {} meditation-related commands:", meditation_commands.len());
    for cmd in &meditation_commands {
        println!("\nCommand: '{}'", cmd.command);
        println!("  Patch: '{}'", cmd.patch);
        println!("  Action: '{}'", cmd.action);
        println!("  Arg: '{}'", cmd.arg);
        println!("  Flags: '{}'", cmd.flags);
    }
    
    // Check if there's a meditation patch
    let has_meditation_patch = patches.contains_key("meditation");
    println!("\n'meditation' patch exists: {}", has_meditation_patch);
    
    if has_meditation_patch {
        let meditation_patch = &patches["meditation"];
        println!("Meditation patch details:");
        if let Some(ref linked_cmd) = meditation_patch.linked_command {
            println!("  Linked command: '{}'", linked_cmd.command);
            println!("  Linked action: '{}'", linked_cmd.action);
            println!("  Linked arg: '{}'", linked_cmd.arg);
        }
    }
    
    // Test inference on a new meditation command
    println!("\n\nTesting inference on new meditation commands:");
    
    let test_cases = vec![
        ("Empty patch", Command {
            patch: String::new(),
            command: "meditation".to_string(),
            action: "app".to_string(),
            arg: "Meditation App".to_string(),
            flags: String::new(),
            full_line: String::new(),
        }),
        ("With Misc patch", Command {
            patch: "Misc".to_string(),
            command: "meditation".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/T/Misc/Meditation/meditation.md".to_string(),
            flags: "A".to_string(),  // Anchor flag
            full_line: String::new(),
        }),
        ("Empty patch with Misc path", Command {
            patch: String::new(),
            command: "meditation".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/T/Misc/Meditation/meditation.md".to_string(),
            flags: "A".to_string(),  // Anchor flag
            full_line: String::new(),
        }),
        ("Empty patch with orphans path", Command {
            patch: String::new(),
            command: "meditation".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/SYS/Closet/Orphans/meditation/meditation.md".to_string(),
            flags: "A".to_string(),  // Anchor flag
            full_line: String::new(),
        }),
    ];
    
    for (desc, cmd) in test_cases {
        let inferred = infer_patch(&cmd, &patches);
        println!("\n{}: patch='{}' → inferred='{}'", 
                desc, 
                cmd.patch,
                inferred.unwrap_or_else(|| "None".to_string()));
    }
    
    // Check all patches that start with 'm'
    println!("\n\nAll patches starting with 'm':");
    let m_patches: Vec<_> = patches.keys()
        .filter(|k| k.to_lowercase().starts_with("m"))
        .collect();
    
    for patch_name in m_patches {
        println!("  '{}'", patch_name);
    }
}