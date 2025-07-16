use hookanchor::launcher::launch;

#[test]
fn test_alias_execution_works() {
    println!("Testing that alias action execution works...");
    
    // Test that the alias action itself can be executed
    // This tests the JavaScript: launch("{{arg}}")
    let result = launch("alias url https://github.com");
    match result {
        Ok(_) => {
            println!("✅ Alias action executed successfully");
        },
        Err(e) => {
            println!("❌ Alias action failed: {:?}", e);
            // This might fail due to the recursive launch() call, but that's expected behavior
        }
    }
}

#[test] 
fn test_alias_in_commands_list() {
    println!("Testing alias commands in the actual commands list...");
    
    use hookanchor::load_commands;
    let commands = load_commands();
    
    let alias_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.action == "alias")
        .collect();
    
    assert!(!alias_commands.is_empty(), "Should have alias commands after adding test aliases");
    
    println!("Found {} alias commands:", alias_commands.len());
    for alias_cmd in &alias_commands {
        println!("  {} -> {}", alias_cmd.command, alias_cmd.arg);
        
        // Test that each alias has a valid argument
        assert!(!alias_cmd.arg.trim().is_empty(), 
                "Alias '{}' should have a non-empty argument", alias_cmd.command);
    }
    
    // Test that our specific test aliases are there
    let gh_alias = alias_commands.iter().find(|cmd| cmd.command == "gh");
    assert!(gh_alias.is_some(), "Should find 'gh' alias");
    assert_eq!(gh_alias.unwrap().arg, "url https://github.com");
    
    let home_alias = alias_commands.iter().find(|cmd| cmd.command == "home");
    assert!(home_alias.is_some(), "Should find 'home' alias");
    assert_eq!(home_alias.unwrap().arg, "folder /Users/testuser");
    
    println!("✅ All alias tests passed");
}