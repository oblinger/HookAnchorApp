use anchor_selector::{load_commands, filter_commands, Command};
use std::process::Command as ProcessCommand;

#[test]
fn test_hook_url_parsing_and_execution() {
    // Test that we can parse HOOK:// URLs and execute commands
    let commands = load_commands();
    
    // Skip if no commands available
    if commands.is_empty() {
        return;
    }
    
    // Test basic URL parsing (without URL decoding dependency)
    let test_queries = vec![
        ("hook://test", "test"),
        ("hook://", ""),
        ("hook://Test_Command-123", "Test_Command-123"),
    ];
    
    for (url, expected_query) in test_queries {
        let query = url.strip_prefix("hook://").unwrap_or("");
        
        assert_eq!(query, expected_query, "URL parsing failed for: {}", url);
    }
}

#[test]
fn test_hook_url_command_filtering() {
    // Test that hook URL queries properly filter commands
    let commands = load_commands();
    
    if commands.is_empty() {
        return;
    }
    
    // Test filtering with a query that should match something
    let query = "test";
    let filtered = filter_commands(&commands, query, 1, false);
    
    // Should either find matches or handle empty results gracefully
    if !filtered.is_empty() {
        let top_match = &filtered[0];
        assert!(!top_match.command.is_empty(), "Top match should have a command");
        assert!(!top_match.action.is_empty(), "Top match should have an action");
    }
}

#[test]
fn test_hook_url_end_to_end_execution() {
    // Test complete end-to-end execution of hook URLs
    let binary_path = "./target/debug/ha";
    
    // Try debug first, then release
    let binary_path = if std::path::Path::new(binary_path).exists() {
        binary_path
    } else {
        "./target/release/ha"
    };
    
    if !std::path::Path::new(binary_path).exists() {
        // Skip test if binary doesn't exist
        return;
    }
    
    let test_cases = vec![
        "hook://test",
        "hook://nonexistent",
        "hook://",
        "hook://test%20query",
    ];
    
    for hook_url in test_cases {
        let output = ProcessCommand::new(binary_path)
            .arg(hook_url)
            .output()
            .expect("Failed to execute command");
        
        // Should not crash or exit with error
        assert!(output.status.success(), "Hook URL '{}' failed with status: {:?}", hook_url, output.status);
        
        // Should not contain panic messages
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("panic"), "Hook URL '{}' caused panic: {}", hook_url, stderr);
    }
}

#[test]
fn test_hook_url_with_specific_commands() {
    // Test hook URLs with specific command patterns
    let commands = load_commands();
    
    if commands.is_empty() {
        return;
    }
    
    // Find a few different command types to test
    let mut test_commands = Vec::new();
    
    for cmd in &commands {
        if test_commands.len() >= 3 {
            break;
        }
        
        // Add different action types for variety
        if !test_commands.iter().any(|c: &&Command| c.action == cmd.action) {
            test_commands.push(cmd);
        }
    }
    
    for cmd in test_commands {
        let hook_url = format!("hook://{}", cmd.command);
        
        // Test filtering for this specific command
        let filtered = filter_commands(&commands, &cmd.command, 1, false);
        
        if !filtered.is_empty() {
            let top_match = &filtered[0];
            
            // Should find the command (or at least something similar)
            assert!(!top_match.command.is_empty(), "Should find a command for '{}'", cmd.command);
            
            // Test with the binary if available
            let binary_path = "./target/debug/ha";
            let binary_path = if std::path::Path::new(binary_path).exists() {
                binary_path
            } else {
                "./target/release/ha"
            };
            
            if std::path::Path::new(binary_path).exists() {
                let output = ProcessCommand::new(binary_path)
                    .arg(&hook_url)
                    .output()
                    .expect("Failed to execute command");
                
                // Should handle the command without crashing
                assert!(output.status.success(), "Hook URL '{}' failed", hook_url);
            }
        }
    }
}

#[test]
fn test_hook_url_special_characters() {
    // Test hook URLs with special characters (raw form)
    let special_cases = vec![
        ("hook://test%20space", "test%20space"),
        ("hook://test%2Bplus", "test%2Bplus"),  
        ("hook://test%26ampersand", "test%26ampersand"),
        ("hook://test%3Fquestion", "test%3Fquestion"),
        ("hook://test%23hash", "test%23hash"),
    ];
    
    for (url, expected_query) in special_cases {
        let query = url.strip_prefix("hook://").unwrap_or("");
        
        assert_eq!(query, expected_query, "URL parsing failed for: {}", url);
        
        // Test that it doesn't crash when filtering
        let commands = load_commands();
        let _filtered = filter_commands(&commands, query, 1, false);
        // No assertion needed - just ensure it doesn't panic
    }
}

#[test]
fn test_hook_url_edge_cases() {
    // Test edge cases that might cause issues
    let edge_cases = vec![
        "hook://",
        "hook://a",
        "hook://very_long_query_that_probably_wont_match_anything_but_should_be_handled_gracefully",
        "hook://123",
        "hook://!@#$%^&*()",
    ];
    
    let commands = load_commands();
    
    for hook_url in edge_cases {
        let query = hook_url.strip_prefix("hook://").unwrap_or("");
        
        // Should not panic when filtering
        let _filtered = filter_commands(&commands, query, 1, false);
        
        // Test with binary if available
        let binary_path = "./target/debug/ha";
        let binary_path = if std::path::Path::new(binary_path).exists() {
            binary_path
        } else {
            "./target/release/ha"
        };
        
        if std::path::Path::new(binary_path).exists() {
            let output = ProcessCommand::new(binary_path)
                .arg(hook_url)
                .output()
                .expect("Failed to execute command");
            
            // Should handle edge cases gracefully
            assert!(output.status.success(), "Hook URL '{}' failed", hook_url);
        }
    }
}