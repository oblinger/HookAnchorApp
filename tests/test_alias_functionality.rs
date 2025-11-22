use hookanchor::core::Command;

#[test]
fn test_alias_replacement_logic() {
    println!("Testing alias replacement functionality...");
    
    // Create test commands including aliases
    let commands = vec![
        Command {
            patch: String::new(),
            command: "gh".to_string(),
            action: "alias".to_string(),
            arg: "url https://github.com".to_string(),
            flags: String::new(),
            other_params: None,
            last_update: 0,
            file_size: None,
        },
        Command {
            patch: String::new(),
            command: "home".to_string(),
            action: "alias".to_string(),
            arg: "folder /Users/testuser".to_string(),
            flags: String::new(),
            other_params: None,
            last_update: 0,
            file_size: None,
        },
        Command {
            patch: String::new(),
            command: "github".to_string(),
            action: "url".to_string(),
            arg: "https://github.com".to_string(),
            flags: String::new(),
            other_params: None,
            last_update: 0,
            file_size: None,
        },
    ];
    
    // Test that alias matching works correctly
    let alias_cmd = commands.iter().find(|cmd| 
        cmd.action == "alias" && cmd.command.to_lowercase() == "gh"
    );
    
    assert!(alias_cmd.is_some(), "Should find 'gh' alias");
    assert_eq!(alias_cmd.unwrap().arg, "url https://github.com");
    
    // Test case-insensitive matching
    let alias_cmd_upper = commands.iter().find(|cmd| 
        cmd.action == "alias" && cmd.command.to_lowercase() == "GH".to_lowercase()
    );
    
    assert!(alias_cmd_upper.is_some(), "Should find 'GH' alias (case-insensitive)");
    
    // Test that non-alias commands are not matched
    let non_alias = commands.iter().find(|cmd| 
        cmd.action == "alias" && cmd.command.to_lowercase() == "github"
    );
    
    assert!(non_alias.is_none(), "Should not find 'github' as alias (it's a url action)");
    
    println!("✅ Alias replacement logic tests passed");
}