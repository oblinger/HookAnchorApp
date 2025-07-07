use anchor_selector::load_commands;
use std::process::Command as ProcessCommand;

#[test]
fn test_hook_url_direct_execution() {
    // Test the hook URL handler directly by calling the binary
    let output = ProcessCommand::new("./target/release/ha")
        .arg("hook://test")
        .output()
        .expect("Failed to execute command");
    
    // Should complete without errors
    assert!(output.status.success(), "Hook URL execution failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Should not have any error output
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.is_empty() || !stderr.contains("Error"), "Hook URL handler produced errors: {}", stderr);
}

#[test]
fn test_hook_url_with_existing_command() {
    // Load commands to find one that exists
    let commands = load_commands();
    
    if commands.is_empty() {
        // Skip test if no commands are available
        return;
    }
    
    // Use the first command for testing
    let test_command = &commands[0];
    let query = &test_command.command;
    
    // Create hook URL
    let hook_url = format!("hook://{}", query);
    
    // Test the hook URL handler
    let output = ProcessCommand::new("./target/release/ha")
        .arg(&hook_url)
        .output()
        .expect("Failed to execute command");
    
    // Should complete without errors
    assert!(output.status.success(), "Hook URL execution failed for '{}': {}", hook_url, String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_hook_url_with_url_encoded_query() {
    // Test URL encoding handling
    let encoded_query = "hook://BBC%20Page"; // "BBC Page" URL encoded
    
    let output = ProcessCommand::new("./target/release/ha")
        .arg(encoded_query)
        .output()
        .expect("Failed to execute command");
    
    // Should complete without errors
    assert!(output.status.success(), "Hook URL with encoding failed: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_hook_url_with_nonexistent_command() {
    // Test with a query that won't match any commands
    let hook_url = "hook://nonexistent_command_xyz_123";
    
    let output = ProcessCommand::new("./target/release/ha")
        .arg(hook_url)
        .output()
        .expect("Failed to execute command");
    
    // Should complete without errors even if no command is found
    assert!(output.status.success(), "Hook URL handler should handle nonexistent commands gracefully: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_hook_url_empty_query() {
    // Test with empty query
    let hook_url = "hook://";
    
    let output = ProcessCommand::new("./target/release/ha")
        .arg(hook_url)
        .output()
        .expect("Failed to execute command");
    
    // Should complete without errors
    assert!(output.status.success(), "Hook URL with empty query should be handled gracefully: {}", String::from_utf8_lossy(&output.stderr));
}

#[test] 
fn test_hook_url_query_parsing() {
    // Test various query formats
    let test_cases = vec![
        "hook://simple",
        "hook://with%20spaces", 
        "hook://with-dashes",
        "hook://with_underscores",
        "hook://123numbers",
    ];
    
    for hook_url in test_cases {
        let output = ProcessCommand::new("./target/release/ha")
            .arg(hook_url)
            .output()
            .expect("Failed to execute command");
        
        // Should complete without errors regardless of query format
        assert!(output.status.success(), "Hook URL '{}' failed: {}", hook_url, String::from_utf8_lossy(&output.stderr));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_hook_url_integration_with_real_commands() {
        // Ensure binary exists before running integration tests
        let binary_path = "./target/release/ha";
        assert!(Path::new(binary_path).exists(), "Binary not found at {}. Run 'cargo build --release' first.", binary_path);
        
        // Test with a few common command patterns that are likely to exist
        let test_queries = vec!["web", "app", "folder", "chrome"];
        
        for query in test_queries {
            let hook_url = format!("hook://{}", query);
            
            let output = ProcessCommand::new(binary_path)
                .arg(&hook_url)
                .output()
                .expect("Failed to execute command");
            
            // Should complete without critical errors
            assert!(output.status.success(), "Hook URL '{}' execution failed: {}", hook_url, String::from_utf8_lossy(&output.stderr));
            
            // Check that stderr doesn't contain critical error messages
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(!stderr.contains("panic"), "Hook URL '{}' caused panic: {}", hook_url, stderr);
            assert!(!stderr.contains("failed to open"), "Hook URL '{}' had file access issues: {}", hook_url, stderr);
        }
    }
}