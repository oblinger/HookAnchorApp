use hookanchor::core::commands::{Command, create_patches_hashmap, infer_patch};

fn main() {
    println!("=== Debug Adam Alg Patch Inference ===\n");
    
    // Create test commands that simulate the actual scenario
    let commands = vec![
        // The DL Alg anchor command (creates the "dl alg" patch) - anchors use 'A' flag
        Command {
            patch: "RR DL".to_string(),
            command: "DL Alg".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/RR/RR DL/DL Alg/DL Alg.md".to_string(),
            flags: "A".to_string(),  // Anchor flag
            full_line: "RR DL! DL Alg : markdown; F:=A A:=/Users/oblinger/ob/kmr/RR/RR DL/DL Alg/DL Alg.md".to_string(),
        },
        // The Adam Alg command with empty patch (to simulate startup)
        Command {
            patch: String::new(), // Empty patch to simulate startup condition
            command: "Adam Alg".to_string(),
            action: "markdown".to_string(),
            arg: "/Users/oblinger/ob/kmr/RR/RR DL/DL Alg/Adam Alg.md".to_string(),
            flags: String::new(),
            full_line: "Adam Alg : markdown; /Users/oblinger/ob/kmr/RR/RR DL/DL Alg/Adam Alg.md".to_string(),
        }
    ];
    
    // Create patches hashmap from commands
    let patches = create_patches_hashmap(&commands);
    
    println!("Available patches:");
    for (key, patch) in &patches {
        println!("  {} -> {:?}", key, patch.name);
        if let Some(ref linked_cmd) = patch.linked_command {
            println!("    Linked to command: {} (action: {}, arg: {})", 
                     linked_cmd.command, linked_cmd.action, linked_cmd.arg);
        }
    }
    println!();
    
    // Test inference for Adam Alg command
    let adam_cmd = &commands[1];
    println!("Testing inference for command: '{}'", adam_cmd.command);
    println!("  Current patch: '{}'", adam_cmd.patch);
    println!("  Action: {}", adam_cmd.action);
    println!("  Arg: {}", adam_cmd.arg);
    println!();
    
    // Test the inference logic step by step
    println!("=== Inference Analysis ===");
    
    // Check first word matching
    if let Some(space_idx) = adam_cmd.command.find(' ') {
        let first_word = &adam_cmd.command[..space_idx];
        let first_word_lower = first_word.to_lowercase();
        println!("First word: '{}' (lowercase: '{}')", first_word, first_word_lower);
        
        if let Some(patch) = patches.get(&first_word_lower) {
            println!("Found matching patch for first word: {}", patch.name);
            if let Some(ref linked_cmd) = patch.linked_command {
                println!("  Linked command: {}", linked_cmd.command);
                
                // Check self-assignment prevention
                let proposed_patch = if let Some(linked_space_idx) = linked_cmd.command.find(' ') {
                    linked_cmd.command[..linked_space_idx].to_string()
                } else {
                    linked_cmd.command.clone()
                };
                
                println!("  Proposed patch: '{}'", proposed_patch);
                println!("  Command to check: '{}'", adam_cmd.command);
                println!("  Would be self-assignment? {}", proposed_patch.to_lowercase() == adam_cmd.command.to_lowercase());
            }
        } else {
            println!("No patch found for first word");
        }
    } else {
        println!("No space found in command name");
    }
    println!();
    
    // Run actual inference
    match infer_patch(adam_cmd, &patches) {
        Some(inferred) => {
            println!("✅ Inference result: '{}' -> '{}'", adam_cmd.command, inferred);
        }
        None => {
            println!("❌ No patch inferred for '{}'", adam_cmd.command);
        }
    }
    
    println!("\n=== Testing with 'Adam' as first word ===");
    
    // Test what happens if we look for just "Adam" 
    let adam_lower = "adam".to_string();
    if let Some(patch) = patches.get(&adam_lower) {
        println!("Found patch for 'adam': {}", patch.name);
    } else {
        println!("No patch found for 'adam'");
    }
    
    // Also test "dl" as first word
    let dl_lower = "dl".to_string();
    if let Some(patch) = patches.get(&dl_lower) {
        println!("Found patch for 'dl': {}", patch.name);
    } else {
        println!("No patch found for 'dl'");
    }
    
    println!("\n=== Testing Path-based Inference ===");
    
    println!("Adam Alg file path: {}", adam_cmd.arg);
    println!("Path components:");
    for (i, component) in adam_cmd.arg.split('/').enumerate() {
        let component_lower = component.to_lowercase();
        if patches.contains_key(&component_lower) {
            println!("  [{}] {} -> MATCHES PATCH '{}'", i, component, component_lower);
        } else {
            println!("  [{}] {}", i, component);
        }
    }
}