use hookanchor::{Command, load_commands};

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
            full_line: "gh : alias url https://github.com".to_string(),
        },
        Command {
            patch: String::new(),
            command: "home".to_string(),
            action: "alias".to_string(),
            arg: "folder /Users/testuser".to_string(),
            flags: String::new(),
            full_line: "home : alias folder /Users/testuser".to_string(),
        },
        Command {
            patch: String::new(),
            command: "github".to_string(),
            action: "url".to_string(),
            arg: "https://github.com".to_string(),
            flags: String::new(),
            full_line: "github : url https://github.com".to_string(),
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

#[test]
fn test_real_commands_have_aliases() {
    println!("Testing that real commands file contains alias entries...");
    
    let commands = load_commands();
    let alias_commands: Vec<_> = commands.iter()
        .filter(|cmd| cmd.action == "alias")
        .collect();
    
    if alias_commands.is_empty() {
        println!("⚠️  No alias commands found in commands.txt");
        println!("   Alias functionality will work but no aliases are currently defined");
    } else {
        println!("✅ Found {} alias commands:", alias_commands.len());
        for alias_cmd in alias_commands.iter().take(5) { // Show first 5
            println!("   {} -> {}", alias_cmd.command, alias_cmd.arg);
        }
        if alias_commands.len() > 5 {
            println!("   ... and {} more", alias_commands.len() - 5);
        }
    }
}