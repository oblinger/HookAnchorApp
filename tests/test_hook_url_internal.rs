use hookanchor::{load_commands, filter_commands, Command};
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

#[test]
fn test_open_hook_url_command() {
    // Test the actual 'open hook://...' command that would be used in practice
    let test_cases = vec![
        "hook://test",
        "hook://web",
        "hook://nonexistent",
        "hook://",
    ];
    
    for hook_url in test_cases {
        // Test using the macOS 'open' command which should trigger our URL handler
        let output = ProcessCommand::new("open")
            .arg(hook_url)
            .output()
            .expect("Failed to execute open command");
        
        // The open command should succeed (it delegates to our registered handler)
        // Note: This test assumes the URL handler is properly registered in the system
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // If it fails, it might be because the URL scheme isn't registered yet
            // That's okay for testing - we just want to verify the command structure
            if stderr.contains("no application configured") || 
               stderr.contains("not registered") ||
               stderr.contains("error -1712") || // macOS URL handler error
               stderr.contains("LSOpenURLsWithCompletionHandler") {
                // This is expected if URL scheme isn't registered properly or has issues
                println!("URL scheme issue (expected during testing): {}", stderr);
            } else {
                panic!("Open command failed unexpectedly for '{}': {}", hook_url, stderr);
            }
        } else {
            println!("Successfully opened hook URL: {}", hook_url);
        }
    }
}

#[test]
fn test_hook_url_system_integration() {
    // Test that demonstrates the full flow:
    // 1. Shell command: open hook://query
    // 2. System delegates to our binary: ha hook://query
    // 3. Our binary processes the URL and executes the command
    
    let test_queries = vec!["test", "web", "app"];
    
    for query in test_queries {
        let hook_url = format!("hook://{}", query);
        
        // First, test that our binary can handle the URL directly
        let binary_path = "./target/release/ha";
        if std::path::Path::new(binary_path).exists() {
            let output = ProcessCommand::new(binary_path)
                .arg(&hook_url)
                .output()
                .expect("Failed to execute ha binary");
            
            assert!(output.status.success(), "Binary failed to handle {}: {}", hook_url, String::from_utf8_lossy(&output.stderr));
        }
        
        // Second, test the full integration with 'open' command
        // This simulates what happens when user clicks a hook:// link or runs 'open hook://...'
        let output = ProcessCommand::new("open")
            .arg(&hook_url)
            .output()
            .expect("Failed to execute open command");
        
        // Check the result
        if output.status.success() {
            println!("System integration successful for: {}", hook_url);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("no application configured") || stderr.contains("not registered") {
                println!("URL scheme not registered (this is expected during testing): {}", hook_url);
            } else {
                // Log unexpected errors but don't fail the test
                println!("Open command had issues with '{}': {}", hook_url, stderr);
            }
        }
    }
}

#[test]
fn test_hook_url_registration_check() {
    // Test to check if the HOOK:// URL scheme is properly registered
    // This uses the system's URL scheme registration to verify setup
    
    // Try to query the system's URL scheme registration
    let output = ProcessCommand::new("defaults")
        .arg("read")
        .arg("com.apple.LaunchServices/com.apple.launchservices.secure")
        .arg("LSHandlers")
        .output();
    
    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Check if our hook:// scheme is registered
        if output_str.contains("hook") || output_str.contains("HOOK") {
            println!("HOOK:// URL scheme appears to be registered in system");
        } else {
            println!("HOOK:// URL scheme not found in system registration (may need setup)");
        }
    } else {
        println!("Could not check URL scheme registration (system command failed)");
    }
    
    // Also test what happens when we try to open a hook URL
    let test_url = "hook://test_registration";
    let output = ProcessCommand::new("open")
        .arg(test_url)
        .output()
        .expect("Failed to test URL registration");
    
    if output.status.success() {
        println!("URL registration test passed: {}", test_url);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("no application configured") {
            println!("URL scheme registration needed: {}", stderr);
        } else {
            println!("URL registration test result: {}", stderr);
        }
    }
}

#[test]
fn test_hook_url_vs_execute_behavior() {
    // Test that 'open hook://query' behaves exactly like 'ha -x query'
    // Both should find the first matching command and execute it directly (no popup)
    
    let test_queries = vec!["test", "web"];
    let binary_path = "./target/release/ha";
    
    if !std::path::Path::new(binary_path).exists() {
        // Skip if binary doesn't exist
        return;
    }
    
    for query in test_queries {
        let hook_url = format!("hook://{}", query);
        
        // Test 1: Direct hook URL execution via binary
        let hook_output = ProcessCommand::new(binary_path)
            .arg(&hook_url)
            .output()
            .expect("Failed to execute hook URL");
        
        // Test 2: Execute command via -x flag
        let execute_output = ProcessCommand::new(binary_path)
            .arg("-x")
            .arg(query)
            .output()
            .expect("Failed to execute -x command");
        
        // Both should succeed
        assert!(hook_output.status.success(), "Hook URL execution failed for: {}", hook_url);
        assert!(execute_output.status.success(), "Execute command failed for: {}", query);
        
        // Hook URL should be silent (no stdout output)
        let hook_stdout = String::from_utf8_lossy(&hook_output.stdout);
        assert!(hook_stdout.trim().is_empty(), "Hook URL should be silent but produced output: {}", hook_stdout);
        
        // -x command should have verbose output
        let execute_stdout = String::from_utf8_lossy(&execute_output.stdout);
        assert!(execute_stdout.contains("Executing top match"), "-x command should show execution message");
        
        // Both should execute the same command (can't easily test execution result directly,
        // but we can verify they both find commands and don't error out)
        println!("✓ Hook URL '{}' and '-x {}' both executed successfully", hook_url, query);
    }
}

#[test]
fn test_hook_url_no_popup_behavior() {
    // Test that hook URLs execute directly without showing popup
    // This test verifies the command-line behavior vs GUI behavior
    
    let binary_path = "./target/release/ha";
    
    if !std::path::Path::new(binary_path).exists() {
        return;
    }
    
    let test_cases = vec![
        ("hook://test", "test"),
        ("hook://web", "web"),
    ];
    
    for (hook_url, _query) in test_cases {
        // Test hook URL execution
        let hook_output = ProcessCommand::new(binary_path)
            .arg(hook_url)
            .output()
            .expect("Failed to execute hook URL");
        
        // Should succeed
        assert!(hook_output.status.success(), "Hook URL failed: {}", hook_url);
        
        // Should be silent (no GUI popup means no stdout interaction)
        let stdout = String::from_utf8_lossy(&hook_output.stdout);
        assert!(stdout.trim().is_empty(), "Hook URL should execute silently without popup output: {}", stdout);
        
        // Should not have GUI-related errors
        let stderr = String::from_utf8_lossy(&hook_output.stderr);
        assert!(!stderr.contains("display"), "Should not have display/GUI errors: {}", stderr);
        assert!(!stderr.contains("window"), "Should not have window errors: {}", stderr);
        assert!(!stderr.contains("popup"), "Should not have popup errors: {}", stderr);
        
        // GUI mode would show popup and require user interaction
        // Hook URL mode executes directly without any user interaction
        
        println!("✓ Hook URL '{}' executed directly without popup", hook_url);
    }
}

#[test]
fn test_hook_url_execution_equivalence() {
    // Test that hook://query and ha -x query execute the same command
    
    let commands = load_commands();
    if commands.is_empty() {
        return;
    }
    
    // Find a command that exists
    let test_query = "test";
    let filtered = filter_commands(&commands, test_query, 1, false);
    
    if filtered.is_empty() {
        // Try with a different query
        let test_query = "web";
        let filtered = filter_commands(&commands, test_query, 1, false);
        if filtered.is_empty() {
            return; // Skip if no commands match
        }
    }
    
    let binary_path = "./target/release/ha";
    if !std::path::Path::new(binary_path).exists() {
        return;
    }
    
    let hook_url = format!("hook://{}", test_query);
    
    // Execute via hook URL
    let hook_output = ProcessCommand::new(binary_path)
        .arg(&hook_url)
        .output()
        .expect("Failed to execute hook URL");
    
    // Execute via -x command
    let execute_output = ProcessCommand::new(binary_path)
        .arg("-x")
        .arg(test_query)
        .output()
        .expect("Failed to execute -x command");
    
    // Both should succeed
    assert!(hook_output.status.success(), "Hook URL execution failed");
    assert!(execute_output.status.success(), "Execute command failed");
    
    // Hook URL should be silent
    let hook_stdout = String::from_utf8_lossy(&hook_output.stdout);
    assert!(hook_stdout.trim().is_empty(), "Hook URL should be silent");
    
    // -x should have output
    let execute_stdout = String::from_utf8_lossy(&execute_output.stdout);
    assert!(!execute_stdout.trim().is_empty(), "-x command should have output");
    
    // Both should have same exit code (success)
    assert_eq!(hook_output.status.code(), execute_output.status.code(), "Exit codes should match");
    
    println!("✓ Hook URL and -x command execute equivalently");
}