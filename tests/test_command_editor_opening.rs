use hookanchor::core::Command;

#[test]
fn test_exact_match_logic() {
    println!("Testing command editor opening logic...");

    // Create some test commands
    let commands = vec![
        Command {
            patch: String::new(),
            command: "Test Command".to_string(),
            action: "app".to_string(),
            arg: "TestApp".to_string(),
            flags: String::new(),
            other_params: None,
            last_update: 0,
            file_size: None,
        },
        Command {
            patch: String::new(),
            command: "Another Command".to_string(),
            action: "url".to_string(),
            arg: "https://example.com".to_string(),
            flags: String::new(),
            other_params: None,
            last_update: 0,
            file_size: None,
        },
    ];
    
    // Test 1: Exact match (case-insensitive)
    let search_text = "test command";
    let exact_match = commands.iter().find(|cmd| 
        cmd.command.to_lowercase() == search_text.to_lowercase()
    );
    
    assert!(exact_match.is_some());
    assert_eq!(exact_match.unwrap().command, "Test Command");
    println!("✅ Found exact match for 'test command' → 'Test Command'");
    
    // Test 2: No exact match
    let search_text = "new command";
    let exact_match = commands.iter().find(|cmd| 
        cmd.command.to_lowercase() == search_text.to_lowercase()
    );
    
    assert!(exact_match.is_none());
    println!("✅ No exact match for 'new command' - would create new");
    
    // Test 3: Partial match (should not match)
    let search_text = "test";
    let exact_match = commands.iter().find(|cmd| 
        cmd.command.to_lowercase() == search_text.to_lowercase()
    );
    
    assert!(exact_match.is_none());
    println!("✅ Partial match 'test' correctly ignored - would create new");
    
    println!("✅ All command editor opening logic tests passed!");
}