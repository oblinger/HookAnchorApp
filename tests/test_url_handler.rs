//! Unit tests for URL handler functionality

#[cfg(test)]
mod tests {
    use hookanchor::Command;

    /// Test URL decoding functionality
    #[test]
    fn test_url_decoding() {
        // Test basic URL decoding
        let encoded = "hook://test%20command";
        let prefix = "hook://";
        
        if let Some(prompt_encoded) = encoded.strip_prefix(prefix) {
            let decoded = urlencoding::decode(prompt_encoded).unwrap();
            assert_eq!(decoded, "test command");
        }
    }

    /// Test URL decoding with special characters
    #[test]
    fn test_url_decoding_special_chars() {
        let test_cases = vec![
            ("hook://spot%20%2A", "spot *"),
            ("hook://test%20%26%20more", "test & more"),
            ("hook://simple", "simple"),
            ("hook://with%2Bplus", "with+plus"),
        ];
        
        for (encoded_url, expected) in test_cases {
            if let Some(prompt_encoded) = encoded_url.strip_prefix("hook://") {
                let decoded = urlencoding::decode(prompt_encoded).unwrap();
                assert_eq!(decoded, expected, "Failed for input: {}", encoded_url);
            }
        }
    }

    /// Test hook URL parsing logic
    #[test]
    fn test_hook_url_parsing() {
        let test_urls = vec![
            "hook://spot",
            "hook://test%20command",
            "hook://",  // empty prompt
            "not-hook://test",  // wrong scheme
        ];
        
        for url in test_urls {
            let is_hook = url.starts_with("hook://");
            if is_hook {
                let prompt = url.strip_prefix("hook://").unwrap_or("");
                let decoded = urlencoding::decode(prompt).unwrap_or_else(|_| prompt.into());
                
                // Should not panic and should produce reasonable results
                assert!(decoded.len() <= url.len()); // Decoded should not be longer than original
                
                if url == "hook://spot" {
                    assert_eq!(decoded, "spot");
                } else if url == "hook://test%20command" {
                    assert_eq!(decoded, "test command");
                } else if url == "hook://" {
                    assert_eq!(decoded, "");
                }
            } else {
                assert!(!is_hook, "Should not be recognized as hook URL: {}", url);
            }
        }
    }

    /// Test command selection logic with a known command
    #[test]
    fn test_command_matching() {
        let commands = vec![
            Command {
                group: String::new(),
                command: "spot".to_string(),
                action: "anchor".to_string(),
                arg: "/Users/test/spot.md".to_string(),
                flags: String::new(),
                full_line: "spot : anchor; /Users/test/spot.md".to_string(),
            },
            Command {
                group: String::new(),
                command: "test command".to_string(),
                action: "app".to_string(),
                arg: "TestApp".to_string(),
                flags: String::new(),
                full_line: "test command : app; TestApp".to_string(),
            },
        ];
        
        // Test exact match
        let matching = commands.iter().find(|cmd| cmd.command == "spot");
        assert!(matching.is_some());
        assert_eq!(matching.unwrap().action, "anchor");
        
        // Test partial match (this would be filtered by the actual filter_commands function)
        let matching_partial = commands.iter().find(|cmd| cmd.command.starts_with("test"));
        assert!(matching_partial.is_some());
        assert_eq!(matching_partial.unwrap().command, "test command");
    }

    /// Test direct command execution logic
    #[test]
    fn test_direct_execution_logic() {
        // Test that empty prompt should not execute
        let empty_prompt = "";
        let should_execute_empty = !empty_prompt.is_empty();
        assert!(!should_execute_empty);
        
        // Test that non-empty prompt should execute
        let non_empty_prompt = "spot";
        let should_execute_non_empty = !non_empty_prompt.is_empty();
        assert!(should_execute_non_empty);
        
        // Test URL prefix detection
        let hook_url = "hook://spot";
        let is_hook_url = hook_url.starts_with("hook://");
        assert!(is_hook_url);
        
        let not_hook_url = "http://example.com";
        let is_not_hook_url = not_hook_url.starts_with("hook://");
        assert!(!is_not_hook_url);
    }
}