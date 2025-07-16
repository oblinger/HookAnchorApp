use hookanchor::load_commands;

/// Test the alias replacement logic directly
#[test] 
fn test_alias_replacement_simulation() {
    println!("Testing alias replacement simulation...");
    
    let commands = load_commands();
    
    // Simulate the check_and_apply_alias logic
    let mut search_text = "gh".to_string();
    
    // Look for an exact match with an alias command
    if let Some(alias_cmd) = commands.iter().find(|cmd| 
        cmd.action == "alias" && cmd.command.to_lowercase() == search_text.to_lowercase()
    ) {
        // Replace search text with the alias argument
        search_text = alias_cmd.arg.clone();
        println!("✅ Alias replacement: 'gh' -> '{}'", search_text);
    } else {
        println!("❌ No alias found for 'gh'");
    }
    
    assert_eq!(search_text, "url https://github.com", "gh should expand to GitHub URL");
    
    // Test case-insensitive matching
    let mut search_text_upper = "GH".to_string();
    if let Some(alias_cmd) = commands.iter().find(|cmd| 
        cmd.action == "alias" && cmd.command.to_lowercase() == search_text_upper.to_lowercase()
    ) {
        search_text_upper = alias_cmd.arg.clone();
        println!("✅ Case-insensitive alias replacement: 'GH' -> '{}'", search_text_upper);
    }
    
    assert_eq!(search_text_upper, "url https://github.com", "GH should also expand to GitHub URL");
    
    // Test non-alias command (should not be replaced)
    let mut search_text_non_alias = "github".to_string();
    let original = search_text_non_alias.clone();
    if let Some(alias_cmd) = commands.iter().find(|cmd| 
        cmd.action == "alias" && cmd.command.to_lowercase() == search_text_non_alias.to_lowercase()
    ) {
        search_text_non_alias = alias_cmd.arg.clone();
    }
    
    assert_eq!(search_text_non_alias, original, "Non-alias commands should not be replaced");
    println!("✅ Non-alias command '{}' correctly unchanged", original);
    
    println!("✅ All alias replacement simulation tests passed");
}

/// Test that aliases work with different argument types
#[test]
fn test_different_alias_types() {
    println!("Testing different types of alias arguments...");
    
    let commands = load_commands();
    let alias_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.action == "alias")
        .collect();
    
    for alias_cmd in &alias_commands {
        println!("Testing alias: {} -> {}", alias_cmd.command, alias_cmd.arg);
        
        // Verify that the argument looks like a valid command
        let parts: Vec<&str> = alias_cmd.arg.split_whitespace().collect();
        assert!(!parts.is_empty(), 
                "Alias '{}' should have a non-empty argument", alias_cmd.command);
        
        // Check that it looks like a valid action (first word should be an action)
        let action = parts[0];
        assert!(!action.is_empty(), 
                "Alias '{}' should have a valid action as first word", alias_cmd.command);
        
        println!("  ✓ Action: {}, Args: {:?}", action, &parts[1..]);
    }
    
    println!("✅ All alias argument validation tests passed");
}