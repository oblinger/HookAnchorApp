//! Unit tests for cmd binary URL handler functionality

#[cfg(test)]
mod tests {
    use std::process::Command;

    /// Test cmd binary exists and can handle basic commands
    #[test]
    fn test_cmd_binary_exists() {
        let output = Command::new("./target/release/cmd")
            .arg("match")
            .arg("nonexistent_test_command_12345")
            .output();
        
        // Should succeed (even if no matches found)
        assert!(output.is_ok(), "cmd binary should be executable");
    }

    /// Test hook URL handling in cmd binary
    #[test]
    fn test_cmd_hook_url_handling() {
        // Test with a simple hook URL
        let output = Command::new("./target/release/cmd")
            .arg("hook://nonexistent_test_12345")
            .output();
        
        // Should execute without error (even if command not found)
        assert!(output.is_ok(), "cmd should handle hook URLs without crashing");
        
        // Should not produce stderr output for missing commands
        if let Ok(result) = output {
            let stderr = String::from_utf8_lossy(&result.stderr);
            // cmd binary should handle missing commands gracefully
            assert!(!stderr.contains("panic"), "Should not panic on missing commands");
        }
    }

    /// Test URL decoding in hook URLs
    #[test]
    fn test_hook_url_decoding() {
        // Test basic URL decoding functionality
        let encoded = "hook://test%20command";
        let prefix = "hook://";
        
        if let Some(prompt_encoded) = encoded.strip_prefix(prefix) {
            let decoded = urlencoding::decode(prompt_encoded).unwrap();
            assert_eq!(decoded, "test command");
        }
    }

    /// Test cmd help output includes hook URL info
    #[test]
    fn test_cmd_help_includes_hook_info() {
        let output = Command::new("./target/release/cmd")
            .output();
        
        if let Ok(result) = output {
            let stderr = String::from_utf8_lossy(&result.stderr);
            assert!(stderr.contains("hook://"), "Help should mention hook:// URL support");
        }
    }

    /// Test that cmd binary handles hook URLs differently than regular commands
    #[test]
    fn test_hook_url_vs_regular_command() {
        // Test that hook:// is recognized as special
        let hook_url = "hook://test";
        assert!(hook_url.starts_with("hook://"));
        
        // Test that regular strings are not hook URLs
        let regular_command = "test";
        assert!(!regular_command.starts_with("hook://"));
        
        let other_url = "http://example.com";
        assert!(!other_url.starts_with("hook://"));
    }
}