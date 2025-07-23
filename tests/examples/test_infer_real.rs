use hookanchor::{load_commands_raw, create_patches_hashmap, infer_patch, Command};

fn main() {
    println!("Testing --infer command with realistic scenarios...");
    
    // Load existing commands and patches  
    let mut commands = load_commands_raw();
    let patches = create_patches_hashmap(&commands);
    
    println!("Found {} patches", patches.len());
    
    // Test scenarios that would get patches inferred
    let test_commands = vec![
        // This should get Application patch (first word matches)
        Command {
            patch: String::new(),
            command: "Application New Test".to_string(),
            action: "app".to_string(),
            arg: "TestApp".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        // This should get RR patch (relative path starts with RR)
        Command {
            patch: String::new(),
            command: "Test RR File".to_string(),
            action: "obs".to_string(),
            arg: "RR/Test/NewFile.md".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
        // This should get no patch (no matching criteria)
        Command {
            patch: String::new(),
            command: "Random Unique Command".to_string(),
            action: "chrome".to_string(),
            arg: "https://example.com".to_string(),
            flags: String::new(),
            full_line: String::new(),
        },
    ];
    
    // Test each command
    for test_cmd in &test_commands {
        let inferred = infer_patch(test_cmd, &patches);
        println!("Testing: '{}' -> {:?}", test_cmd.command, inferred);
        
        if let Some(patch) = inferred {
            println!("  Would show: {}: (empty) -> {}", test_cmd.command, patch);
        } else {
            println!("  Would show: no change needed");
        }
    }
}