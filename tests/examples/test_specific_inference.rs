use hookanchor::core::{load_commands_with_data, infer_patch, Command, Patch};
use std::collections::HashMap;

fn main() {
    println!("Testing specific patch inference cases...");
    
    // Load commands and create patches
    let (_config, commands, patches) = load_commands_with_data();
    
    println!("Loaded {} commands and {} patches", commands.len(), patches.len());
    
    // Test cases for patch inference (anchors use 'A' flag, not action type)
    let test_commands = vec![
        // Test case 1: Meditation command from Misc patch
        Command {
            patch: "Misc".to_string(),
            command: "meditation".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/T/Misc/Meditation/meditation.md".to_string(),
            flags: "A".to_string(),  // Anchor flag
            full_line: "Misc! meditation : markdown; F:=A A:=/Users/oblinger/ob/kmr/T/Misc/Meditation/meditation.md".to_string(),
        },
        // Test case 2: Meditation command from orphans patch
        Command {
            patch: "orphans".to_string(),
            command: "meditation".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/SYS/Closet/Orphans/meditation/meditation.md".to_string(),
            flags: "A".to_string(),  // Anchor flag
            full_line: "meditation! meditation : markdown; F:=A A:=/Users/oblinger/ob/kmr/SYS/Closet/Orphans/meditation/meditation.md".to_string(),
        },
    ];
    
    println!("\nTesting inference on specific commands:");
    for (i, cmd) in test_commands.iter().enumerate() {
        println!("\nTest {}: '{}'", i + 1, cmd.command);
        println!("  Current patch: '{}'", cmd.patch);
        println!("  Action: '{}'", cmd.action);
        println!("  Arg: '{}'", cmd.arg);
        println!("  Flags: '{}'", cmd.flags);
        
        let inferred = infer_patch(cmd, &patches);
        println!("  Inferred patch: {}", 
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