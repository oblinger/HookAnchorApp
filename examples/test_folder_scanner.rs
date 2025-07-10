use hookanchor::{Command, scanner};

fn main() {
    // Create test directory structure
    let test_dir = "/tmp/anchor_test_scanner";
    std::fs::create_dir_all(format!("{}/folder1/subfolder1", test_dir)).unwrap();
    std::fs::create_dir_all(format!("{}/folder2", test_dir)).unwrap();
    std::fs::create_dir_all(format!("{}/.hidden_folder", test_dir)).unwrap();
    std::fs::create_dir_all(format!("{}/node_modules", test_dir)).unwrap();
    
    // Create some test markdown files
    std::fs::write(format!("{}/test.md", test_dir), "# Test").unwrap();
    std::fs::write(format!("{}/folder1/doc1.md", test_dir), "# Doc1").unwrap();
    std::fs::write(format!("{}/folder1/subfolder1/doc2.md", test_dir), "# Doc2").unwrap();
    
    // Create some existing commands to test collision detection
    let existing_commands = vec![
        Command {
            group: String::new(),
            command: "folder1".to_string(),
            action: "app".to_string(),
            arg: "SomeApp".to_string(),
            flags: String::new(),
            full_line: "folder1 : app; SomeApp".to_string(),
        },
    ];
    
    println!("Testing scanner with directory structure:");
    println!("  {}/", test_dir);
    println!("  ├── test.md");
    println!("  ├── folder1/ (conflicts with existing command)");
    println!("  │   ├── doc1.md");
    println!("  │   └── subfolder1/");
    println!("  │       └── doc2.md");
    println!("  ├── folder2/");
    println!("  ├── .hidden_folder/ (should be skipped)");
    println!("  └── node_modules/ (should be skipped)");
    println!();
    
    // Test scanning
    let roots = vec![test_dir.to_string()];
    let config = hookanchor::load_config();
    let updated_commands = scanner::scan_files(existing_commands, &roots, &config);
    
    println!("Commands after scanning:");
    
    // Group by action type
    let mut by_action: std::collections::HashMap<String, Vec<&Command>> = std::collections::HashMap::new();
    for cmd in &updated_commands {
        by_action.entry(cmd.action.clone()).or_insert_with(Vec::new).push(cmd);
    }
    
    // Sort action types for consistent output
    let mut action_types: Vec<String> = by_action.keys().cloned().collect();
    action_types.sort();
    
    for action in action_types {
        if let Some(cmds) = by_action.get(&action) {
            println!("\n{} commands:", action);
            let mut sorted_cmds = cmds.clone();
            sorted_cmds.sort_by(|a, b| a.command.cmp(&b.command));
            for cmd in sorted_cmds {
                println!("  '{}' -> {}", cmd.command, cmd.arg);
            }
        }
    }
    
    // Cleanup
    std::fs::remove_dir_all(test_dir).unwrap();
}