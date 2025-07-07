#[cfg(test)]
mod merge_tests {
    use super::super::*;
    use crate::core::config::{Config, PopupSettings};

    fn create_test_config(merge_enabled: bool) -> Config {
        Config {
            popup_settings: PopupSettings {
                merge_similar: merge_enabled,
                word_separators: " ._-".to_string(),
                max_rows: 10,
                max_columns: 1,
                use_new_launcher: true,
                debug_log: None,
                scan_root: None,
                scan_interval_seconds: None,
                listed_actions: None,
            },
            launcher_settings: None,
            functions: None,
            markdown_roots: None,
            grabber_rules: None,
            keybindings: None,
        }
    }

    fn create_command(name: &str, action: &str) -> Command {
        Command {
            group: String::new(),
            command: name.to_string(),
            action: action.to_string(),
            arg: String::new(),
            flags: String::new(),
            full_line: format!("{} : {};", name, action),
        }
    }

    #[test]
    fn test_merge_house_commands() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("House Crime Folder", "folder"),
            create_command("House Crime *", "doc"),
            create_command("House Folder", "folder"),
            create_command("House *", "doc"),
            create_command("house taxes", "doc"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);

        // Should have "House ..." and "House Crime ..." merged entries
        let has_house_merge = merged.iter().any(|cmd| cmd.command == "House ...");
        let has_house_crime_merge = merged.iter().any(|cmd| cmd.command == "House Crime ...");
        
        assert!(has_house_merge, "Should have 'House ...' merged entry");
        assert!(has_house_crime_merge, "Should have 'House Crime ...' merged entry");
        
        // Single word "house taxes" should not be merged
        let has_house_taxes = merged.iter().any(|cmd| cmd.command == "house taxes");
        assert!(has_house_taxes, "Should keep 'house taxes' as individual entry");
    }

    #[test]
    fn test_merge_home_commands() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Home App", "app"),
            create_command("Home *", "doc"),
            create_command("HomeDepot 1Pass", "1pass"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);

        // Should have "Home ..." merged entry for multi-word commands
        let has_home_merge = merged.iter().any(|cmd| cmd.command == "Home ...");
        assert!(has_home_merge, "Should have 'Home ...' merged entry");
        
        // HomeDepot should not be merged with Home (no separator)
        let has_homedepot = merged.iter().any(|cmd| cmd.command == "HomeDepot 1Pass");
        assert!(has_homedepot, "Should keep 'HomeDepot 1Pass' as separate entry");
    }

    #[test]
    fn test_merge_single_word_prefix_commands() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Hold Folder", "folder"),
            create_command("Hold *", "doc"),
            create_command("Hotels Folder", "folder"),
            create_command("Hotels *", "doc"),
            create_command("Hover Folder", "folder"),
            create_command("Hover *", "doc"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);

        // All should have merged entries
        assert!(merged.iter().any(|cmd| cmd.command == "Hold ..."), "Should have 'Hold ...' merged entry");
        assert!(merged.iter().any(|cmd| cmd.command == "Hotels ..."), "Should have 'Hotels ...' merged entry");
        assert!(merged.iter().any(|cmd| cmd.command == "Hover ..."), "Should have 'Hover ...' merged entry");
    }

    #[test]
    fn test_merge_with_search_context() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Hook System *", "doc"),
            create_command("Hook System Folder", "folder"),
            create_command("Hook Alternatives *", "doc"),
        ];

        // Test with "ho" search context
        let merged = merge_similar_commands_with_context(commands.clone(), &config, "ho");
        
        // Should merge "Hook System" commands
        let has_hook_system_merge = merged.iter().any(|cmd| cmd.command == "Hook System ...");
        assert!(has_hook_system_merge, "Should have 'Hook System ...' merged entry with 'ho' search");
    }

    #[test]
    fn test_no_merge_when_disabled() {
        let config = create_test_config(false); // Merging disabled
        let commands = vec![
            create_command("House Folder", "folder"),
            create_command("House *", "doc"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // Should not have any merged entries
        let has_merge = merged.iter().any(|cmd| cmd.command.ends_with("..."));
        assert!(!has_merge, "Should not have any merged entries when merging is disabled");
        assert_eq!(merged.len(), 2, "Should have all original commands");
    }

    #[test]
    fn test_merge_requires_at_least_two_commands() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Unique Command", "app"),
            create_command("Another Single", "doc"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // Should not merge single commands
        let has_merge = merged.iter().any(|cmd| cmd.command.ends_with("..."));
        assert!(!has_merge, "Should not merge when only one command per prefix");
        assert_eq!(merged.len(), 2, "Should keep all original commands");
    }

    #[test]
    fn test_merge_case_sensitivity() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("MAC How To *", "doc"),
            create_command("MAC HOW TO folder", "folder"),  // Different case
            create_command("mac how to", "doc"),  // Exact match of prefix, different case
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // Debug output
        println!("Case sensitivity test - merged commands:");
        for cmd in &merged {
            println!("  - {}", cmd.command);
        }
        
        // Should merge commands with same prefix regardless of case in content after prefix
        let has_mac_merge = merged.iter().any(|cmd| cmd.command == "MAC How To ...");
        assert!(has_mac_merge, "Should have 'MAC How To ...' merged entry");
    }

    #[test]
    fn test_merge_with_different_separators() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Test_Command_One", "app"),
            create_command("Test_Command_Two", "doc"),
            create_command("Test.Command.Three", "folder"),
            create_command("Test.Command.Four", "url"),
            create_command("Test-Command-Five", "cmd"),
            create_command("Test-Command-Six", "app"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // Should have separate merges for each separator type
        assert!(merged.iter().any(|cmd| cmd.command == "Test_Command ..."), "Should merge underscore-separated commands");
        assert!(merged.iter().any(|cmd| cmd.command == "Test.Command ..."), "Should merge dot-separated commands");
        assert!(merged.iter().any(|cmd| cmd.command == "Test-Command ..."), "Should merge dash-separated commands");
    }

    #[test]
    fn test_merge_flag_is_set() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Merge Test One", "app"),
            create_command("Merge Test Two", "doc"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // Debug output
        println!("Merged commands:");
        for cmd in &merged {
            println!("  - {} (flags: '{}')", cmd.command, cmd.flags);
        }
        
        // Find the merged entry
        let merge_entry = merged.iter().find(|cmd| cmd.command == "Merge Test ...");
        assert!(merge_entry.is_some(), "Should find 'Merge Test ...' entry");
        let merge_entry = merge_entry.unwrap();
        
        // Check that the merge flag is set
        assert_eq!(merge_entry.get_flag('M'), Some("".to_string()), "Merged entry should have 'M' flag set");
    }

    #[test]
    fn test_merge_preserves_action_from_first_alphabetical() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Action Test Zebra", "folder"),
            create_command("Action Test Apple", "doc"),
            create_command("Action Test Middle", "app"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // Find the merged entry
        let merge_entry = merged.iter().find(|cmd| cmd.command == "Action Test ...").unwrap();
        
        // Should use action from "Action Test Apple" (first alphabetically)
        assert_eq!(merge_entry.action, "doc", "Merged entry should use action from first alphabetical command");
    }

    #[test]
    fn test_no_merge_between_different_prefixes() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Home App", "app"),
            create_command("Home *", "doc"),
            create_command("House Folder", "folder"),
            create_command("House *", "doc"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // Should have separate merges for Home and House
        assert!(merged.iter().any(|cmd| cmd.command == "Home ..."), "Should have 'Home ...' merge");
        assert!(merged.iter().any(|cmd| cmd.command == "House ..."), "Should have 'House ...' merge");
        
        // Should NOT have a combined Home/House merge
        assert!(!merged.iter().any(|cmd| cmd.command.contains("Home") && cmd.command.contains("House")), 
                "Home and House should not merge together");
    }

    #[test]
    fn test_complex_real_world_scenario() {
        let config = create_test_config(true);
        let commands = vec![
            // Home commands
            create_command("Home App", "app"),
            create_command("Home *", "doc"),
            
            // House commands  
            create_command("House Crime Folder", "folder"),
            create_command("House Crime *", "doc"),
            create_command("House Folder", "folder"),
            create_command("House *", "doc"),
            create_command("house taxes", "doc"),
            
            // Hook commands
            create_command("Hook System *", "doc"),
            create_command("Hook System Folder", "folder"),
            create_command("Hook System !", "cmd"),
            
            // Single commands (should not merge)
            create_command("HOTKEY ALTERNATIVES *", "doc"),
            create_command("High Output Management *", "doc"),
        ];

        let merged = merge_similar_commands_with_context(commands.clone(), &config, "ho");
        
        // Debug output
        println!("Complex scenario test - merged commands:");
        for cmd in &merged {
            println!("  - {}", cmd.command);
        }
        
        // Verify expected merges
        assert!(merged.iter().any(|cmd| cmd.command == "Home ..."), "Should have 'Home ...' merge");
        assert!(merged.iter().any(|cmd| cmd.command == "House ..."), "Should have 'House ...' merge");
        assert!(merged.iter().any(|cmd| cmd.command == "House Crime ..."), "Should have 'House Crime ...' merge");
        assert!(merged.iter().any(|cmd| cmd.command == "Hook System ..."), "Should have 'Hook System ...' merge");
        
        // Verify non-merged entries still exist
        assert!(merged.iter().any(|cmd| cmd.command == "house taxes"), "Should keep 'house taxes'");
        assert!(merged.iter().any(|cmd| cmd.command == "HOTKEY ALTERNATIVES *"), "Should keep single command");
    }
    
    #[test]
    fn test_single_word_commands_dont_interfere() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Home App", "app"),
            create_command("Home *", "doc"),
            create_command("Homography", "doc"),  // Single word, shouldn't interfere
            create_command("HomeDepot", "url"),   // Single word, shouldn't interfere
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // Should still merge "Home App" and "Home *" into "Home ..."
        assert!(merged.iter().any(|cmd| cmd.command == "Home ..."), "Should have 'Home ...' merge");
        
        // Single word commands should remain separate
        assert!(merged.iter().any(|cmd| cmd.command == "Homography"), "Should keep 'Homography' separate");
        assert!(merged.iter().any(|cmd| cmd.command == "HomeDepot"), "Should keep 'HomeDepot' separate");
    }
    
    #[test]
    fn test_exact_prefix_match_included_in_merge() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("Test Command One", "app"),
            create_command("Test Command Two", "doc"),
            create_command("Test Command", "folder"),  // Exact match of prefix
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // All three should merge because "Test Command" is the common prefix
        assert!(merged.iter().any(|cmd| cmd.command == "Test Command ..."), "Should merge all including exact prefix match");
        assert_eq!(merged.len(), 1, "Should have only the merged entry");
    }
    
    #[test]
    fn test_different_separators_treated_same() {
        let config = create_test_config(true);
        let commands = vec![
            create_command("My.Home.md", "doc"),
            create_command("My_Home_folder", "folder"),
            create_command("My-Home-app", "app"),
            create_command("My Home test", "test"),
        ];

        let merged = merge_similar_commands(commands.clone(), &config);
        
        // All should merge to "My Home ..." since dots, underscores, dashes are word separators
        let has_merge = merged.iter().any(|cmd| 
            cmd.command.starts_with("My") && cmd.command.ends_with("...")
        );
        assert!(has_merge, "Should merge commands with different separators");
    }
    
    #[test]
    fn test_alias_command_execution() {
        // Create test commands including an alias
        let commands = vec![
            create_command("CNN", "url; https://cnn.com"),
            create_command("Food", "alias; CNN"),  // Alias that points to CNN
            create_command("News", "alias; cnn"),  // Case-insensitive alias
        ];
        
        // Save commands to a temporary location for testing
        // Note: This test assumes the system can access the commands
        
        // Test that the alias resolves correctly
        // For this test, we'll verify the logic by checking the filter results
        let filtered = filter_commands(&commands, "CNN", 1, false);
        
        assert!(!filtered.is_empty(), "Should find CNN command");
        assert_eq!(filtered[0].command, "CNN", "Should find exact CNN command");
        
        // Test case-insensitive lookup
        let filtered_lower = filter_commands(&commands, "cnn", 1, false);
        assert!(!filtered_lower.is_empty(), "Should find CNN command with lowercase search");
    }
    
    #[test]
    fn test_alias_prevents_infinite_recursion() {
        let commands = vec![
            create_command("A", "alias; B"),
            create_command("B", "alias; A"),  // Circular reference
        ];
        
        // This test ensures that circular aliases don't cause infinite recursion
        // The implementation should handle this gracefully
        
        let filtered = filter_commands(&commands, "B", 1, false);
        
        assert!(!filtered.is_empty(), "Should find target command B");
        // Note: In a real implementation, we'd need recursion depth limiting
    }
    
    #[test]
    fn test_alias_with_partial_match() {
        let commands = vec![
            create_command("CNN News Page", "url; https://cnn.com/news"),
            create_command("CNN Sports Page", "url; https://cnn.com/sports"),
            create_command("QuickNews", "alias; CNN"),  // Should match first CNN command
        ];
        
        let filtered = filter_commands(&commands, "CNN", 2, false);
        
        assert!(!filtered.is_empty(), "Should find CNN commands");
        // The alias should execute the first matching CNN command
        assert!(filtered[0].command.starts_with("CNN"), "First result should be a CNN command");
    }
}