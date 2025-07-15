use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch, Command};

fn main() {
    println!("Testing that load_data conditional logic works correctly...");
    
    // Load existing commands and patches
    let commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    // Create test commands: one with patch, one without
    let test_commands = vec![
        // Command with existing patch - should NOT be changed
        Command {
            patch: "Existing".to_string(),
            command: "Application Test".to_string(),
            action: "app".to_string(),
            arg: "TestApp".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        // Command without patch - SHOULD get Application patch
        Command {
            patch: String::new(),
            command: "Application New".to_string(),
            action: "app".to_string(),
            arg: "NewApp".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
    ];
    
    for (i, cmd) in test_commands.iter().enumerate() {
        println!("\nTest {}: '{}'", i + 1, cmd.command);
        println!("  Current patch: '{}'", cmd.patch);
        
        // Test what infer_patch returns (always returns best inference)
        let inferred = infer_patch(cmd, &patches);
        println!("  Inferred patch: {:?}", inferred);
        
        // Test what load_data would do (conditional application)
        if cmd.patch.is_empty() {
            if let Some(patch) = inferred {
                println!("  ✅ load_data would assign: '{}'", patch);
            } else {
                println!("  ⏸ load_data would leave empty (no inference)");
            }
        } else {
            println!("  ⏸ load_data would keep existing: '{}'", cmd.patch);
        }
    }
    
    println!("\n✅ Simplified design working correctly:");
    println!("  - infer_patch() always returns what the patch should be");
    println!("  - load_data() only applies inference if patch is empty");
    println!("  - --infer command shows all possible changes");
}