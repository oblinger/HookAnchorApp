use hookanchor::core::commands::{Command, create_patches_hashmap, run_patch_inference};
use std::collections::HashMap;

fn main() {
    println!("=== Test Startup Patch Inference Scenario ===\n");
    
    // Create test commands simulating a startup scenario where Adam Alg has no patch
    let mut commands = vec![
        // The DL Alg anchor command (creates the "dl alg" patch) - anchors use 'A' flag
        Command {
            patch: "RR DL".to_string(),
            command: "DL Alg".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/RR/RR DL/DL Alg/DL Alg.md".to_string(),
            flags: "A".to_string(),  // Anchor flag
            full_line: "RR DL! DL Alg : markdown; F:=A A:=/Users/oblinger/ob/kmr/RR/RR DL/DL Alg/DL Alg.md".to_string(),
        },
        // Some other commands in the DL Alg folder
        Command {
            patch: "DL Alg".to_string(),
            command: "LSTM".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/RR/RR DL/DL Alg/LSTM.md".to_string(),
            flags: String::new(),
            full_line: "DL Alg! LSTM : markdown; /Users/oblinger/ob/kmr/RR/RR DL/DL Alg/LSTM.md".to_string(),
        },
        Command {
            patch: "DL Alg".to_string(),
            command: "RAG".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/RR/RR DL/DL Alg/RAG.md".to_string(),
            flags: String::new(),
            full_line: "DL Alg! RAG : markdown; /Users/oblinger/ob/kmr/RR/RR DL/DL Alg/RAG.md".to_string(),
        },
        // The Adam Alg command with empty patch (simulating what user described)
        Command {
            patch: String::new(), // Empty patch to simulate the condition
            command: "Adam Alg".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/RR/RR DL/DL Alg/Adam Alg.md".to_string(),
            flags: String::new(),
            full_line: "Adam Alg : markdown; /Users/oblinger/ob/kmr/RR/RR DL/DL Alg/Adam Alg.md".to_string(),
        },
    ];
    
    println!("Initial commands state:");
    for (i, cmd) in commands.iter().enumerate() {
        let patch_display = if cmd.patch.is_empty() { "(empty)" } else { &cmd.patch };
        println!("  [{}] {} -> patch: '{}'", i, cmd.command, patch_display);
    }
    println!();
    
    // Create patches hashmap from commands
    let patches = create_patches_hashmap(&commands);
    
    println!("Available patches from anchor commands:");
    for (key, patch) in &patches {
        println!("  {} -> {}", key, patch.name);
        if let Some(ref linked_cmd) = patch.linked_command {
            println!("    Linked to: {} ({})", linked_cmd.command, linked_cmd.action);
        }
    }
    println!();
    
    // Test different inference scenarios
    println!("=== Testing inference scenarios ===");
    
    // Scenario 1: Normal startup inference (only empty patches)
    println!("1. Normal startup inference (apply_changes=true, overwrite_patch=false):");
    let mut commands_test1 = commands.clone();
    let (assigned, new_patches) = run_patch_inference(
        &mut commands_test1, 
        &patches, 
        true,  // apply_changes = true
        true,  // print_to_stdout = true
        false  // overwrite_patch = false (only process empty patches)
    );
    println!("   Patches assigned: {}", assigned);
    println!("   New patches to add: {:?}", new_patches);
    
    println!("\nAfter normal inference:");
    for (i, cmd) in commands_test1.iter().enumerate() {
        let patch_display = if cmd.patch.is_empty() { "(empty)" } else { &cmd.patch };
        println!("  [{}] {} -> patch: '{}'", i, cmd.command, patch_display);
    }
    println!();
    
    // Scenario 2: Force overwrite all patches
    println!("2. Force overwrite inference (apply_changes=false, overwrite_patch=true):");
    let mut commands_test2 = commands.clone();
    let (assigned2, new_patches2) = run_patch_inference(
        &mut commands_test2, 
        &patches, 
        false, // apply_changes = false (just analyze)
        true,  // print_to_stdout = true
        true   // overwrite_patch = true (process all commands)
    );
    println!("   Would assign patches: {}", assigned2);
    println!("   Would add new patches: {:?}", new_patches2);
    println!();
    
    // Check specific command conditions
    println!("=== Checking specific conditions that might prevent assignment ===");
    let adam_cmd = &commands[3];
    
    println!("Adam Alg command analysis:");
    println!("  Patch is empty: {}", adam_cmd.patch.is_empty());
    println!("  Action: {}", adam_cmd.action);
    println!("  Is path-based: {}", adam_cmd.is_path_based());
    println!("  File path: {}", adam_cmd.arg);
    
    // Check if the file actually exists
    use std::path::Path;
    let file_exists = Path::new(&adam_cmd.arg).exists();
    println!("  File exists: {}", file_exists);
    
    // Manual inference test
    use hookanchor::core::commands::infer_patch;
    match infer_patch(adam_cmd, &patches) {
        Some(inferred) => {
            println!("  Manual inference result: {} -> {}", adam_cmd.command, inferred);
            
            // Check if it would be assigned
            let should_assign = adam_cmd.patch.is_empty() && adam_cmd.patch != inferred;
            println!("  Would be assigned in normal operation: {}", should_assign);
        }
        None => {
            println!("  Manual inference result: No patch inferred");
        }
    }
}