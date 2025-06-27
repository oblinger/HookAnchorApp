use anchor_selector::{command_matches_query, command_matches_query_with_debug, merge_similar_commands, filter_commands, get_submenu_display_positions, get_submenu_prefix, Command};

// Testing for submenus
#[test]
fn test_exact_match_with_dots() {
    assert!(command_matches_query("test.me", "test.me"));
    assert!(command_matches_query("test.me", "me"));
    assert!(command_matches_query("test.me", "test.m"));
    assert!(command_matches_query("test.me", "tes.m"));
    //assert!(command_matches_query("test.me", "test.me"));
}

// Integer return value tests
#[test]
fn test_integer_return_values_exact_matches() {
    // Complete matches should return command length
    assert_eq!(command_matches_query_with_debug("hello", "hello", false), 5);
    assert_eq!(command_matches_query_with_debug("test", "test", false), 4);
    assert_eq!(command_matches_query_with_debug("a", "a", false), 1);
}

#[test]
fn test_integer_return_values_partial_matches() {
    // Partial matches should return position of first unmatched non-space/dot char
    assert_eq!(command_matches_query_with_debug("hello", "h", false), 1);
    assert_eq!(command_matches_query_with_debug("hello", "hel", false), 3);
    assert_eq!(command_matches_query_with_debug("testing", "test", false), 4);
    assert_eq!(command_matches_query_with_debug("application", "app", false), 3);
}

#[test]
fn test_integer_return_values_no_matches() {
    // No matches should return -1
    assert_eq!(command_matches_query_with_debug("hello", "xyz", false), -1);
    assert_eq!(command_matches_query_with_debug("test", "abc", false), -1);
    assert_eq!(command_matches_query_with_debug("command", "zzz", false), -1);
}

#[test]
fn test_integer_return_values_with_spaces() {
    // Spaces should be skipped when finding first unmatched char
    assert_eq!(command_matches_query_with_debug("hello world", "hello", false), 6); // 'w' at position 6
    assert_eq!(command_matches_query_with_debug("my test command", "my", false), 3); // 't' at position 3
    assert_eq!(command_matches_query_with_debug("open file", "open", false), 5); // 'f' at position 5
}

#[test]
fn test_integer_return_values_with_underscores() {
    // Underscores should be skipped when finding first unmatched char
    assert_eq!(command_matches_query_with_debug("hello_world", "hello", false), 6); // 'w' at position 6
    assert_eq!(command_matches_query_with_debug("my_test_command", "my", false), 3); // 't' at position 3
    assert_eq!(command_matches_query_with_debug("open_file", "open", false), 5); // 'f' at position 5
}

#[test]
fn test_integer_return_values_with_dots() {
    // Dots should be skipped when finding first unmatched char
    assert_eq!(command_matches_query_with_debug("test.me", "test", false), 5); // 'm' at position 5
    assert_eq!(command_matches_query_with_debug("file.txt", "file", false), 5); // 't' at position 5
    assert_eq!(command_matches_query_with_debug("config.json", "config", false), 7); // 'j' at position 7
}

#[test]
fn test_integer_return_values_complete_matches_with_separators() {
    // Complete matches with separators should return full length
    assert_eq!(command_matches_query_with_debug("hello world", "hello world", false), 11);
    assert_eq!(command_matches_query_with_debug("test_command", "test_command", false), 12);
    assert_eq!(command_matches_query_with_debug("file.txt", "file.txt", false), 8);
}

#[test]
fn test_integer_return_values_empty_query() {
    // Empty query should return command length (matches everything)
    assert_eq!(command_matches_query_with_debug("hello", "", false), 5);
    assert_eq!(command_matches_query_with_debug("test command", "", false), 12);
    assert_eq!(command_matches_query_with_debug("", "", false), 0);
}

#[test]
fn test_integer_return_values_multi_word_scenarios() {
    // Multi-word queries should return appropriate positions
    assert_eq!(command_matches_query_with_debug("hello world test", "hello world", false), 12); // 't' at position 12
    assert_eq!(command_matches_query_with_debug("my_test_command", "my test", false), 8); // 'c' at position 8
    assert_eq!(command_matches_query_with_debug("open file manager", "open file", false), 10); // 'm' at position 10
    
    // Complete multi-word matches
    assert_eq!(command_matches_query_with_debug("hello world", "hello world", false), 11);
    assert_eq!(command_matches_query_with_debug("test_command", "test command", false), 12);
}

#[test]
fn test_integer_return_values_flexible_matching() {
    // Test flexible character matching across word boundaries
    assert_eq!(command_matches_query_with_debug("hello world", "helloworld", false), 11);
    assert_eq!(command_matches_query_with_debug("test_command", "testcommand", false), 12);
    assert_eq!(command_matches_query_with_debug("my test app", "mytestapp", false), 11);
}

#[test]
fn test_integer_return_values_single_characters() {
    // Single character queries
    assert_eq!(command_matches_query_with_debug("test", "t", false), 1); // 'e' at position 1
    assert_eq!(command_matches_query_with_debug("hello", "h", false), 1); // 'e' at position 1
    assert_eq!(command_matches_query_with_debug("a", "a", false), 1); // Complete match
    
    // Single character commands
    assert_eq!(command_matches_query_with_debug("x", "x", false), 1); // Complete match
    assert_eq!(command_matches_query_with_debug("a", "b", false), -1); // No match
}

// Basic matching tests
#[test]
fn test_exact_match() {
    assert!(command_matches_query("test", "test"));
    assert!(command_matches_query("command", "command"));
    assert!(command_matches_query("hello", "hello"));
    
    // Test integer return values for exact matches
    assert_eq!(command_matches_query_with_debug("test", "test", false), 4);
    assert_eq!(command_matches_query_with_debug("command", "command", false), 7);
    assert_eq!(command_matches_query_with_debug("hello", "hello", false), 5);
}

#[test]
fn test_case_insensitive() {
    assert!(command_matches_query("Test", "test"));
    assert!(command_matches_query("TEST", "test"));
    assert!(command_matches_query("test", "TEST"));
    assert!(command_matches_query("CamelCase", "camelcase"));
}

#[test]
fn test_empty_inputs() {
    assert!(command_matches_query("", ""));
    assert!(command_matches_query("test", ""));
    assert!(!command_matches_query("", "test"));
    
    // Test integer return values for empty inputs
    assert_eq!(command_matches_query_with_debug("", "", false), 0);
    assert_eq!(command_matches_query_with_debug("test", "", false), 4);
    assert_eq!(command_matches_query_with_debug("", "test", false), -1);
}

// Prefix matching tests
#[test]
fn test_prefix_matching() {
    assert!(command_matches_query("testing", "test"));
    assert!(command_matches_query("application", "app"));
    assert!(command_matches_query("configuration", "config"));
    assert!(command_matches_query("development", "dev"));
    
    // Test integer return values for prefix matches
    assert_eq!(command_matches_query_with_debug("testing", "test", false), 4); // 'i' at position 4
    assert_eq!(command_matches_query_with_debug("application", "app", false), 3); // 'l' at position 3
    assert_eq!(command_matches_query_with_debug("configuration", "config", false), 6); // 'u' at position 6
    assert_eq!(command_matches_query_with_debug("development", "dev", false), 3); // 'e' at position 3
}

#[test]
fn test_prefix_no_match() {
    assert!(!command_matches_query("test", "testing"));
    assert!(!command_matches_query("app", "application"));
    assert!(!command_matches_query("cmd", "command"));
    
    // Test integer return values for no matches
    assert_eq!(command_matches_query_with_debug("test", "testing", false), -1);
    assert_eq!(command_matches_query_with_debug("app", "application", false), -1);
    assert_eq!(command_matches_query_with_debug("cmd", "command", false), -1);
}

// Word boundary tests with spaces
#[test]
fn test_space_separated_words() {
    assert!(command_matches_query("hello world", "hello"));
    assert!(command_matches_query("hello world", "world"));
    assert!(command_matches_query("hello world", "hello world"));
    assert!(command_matches_query("my test command", "test"));
    assert!(command_matches_query("open file manager", "file"));
}

#[test]
fn test_space_separated_prefix() {
    assert!(command_matches_query("hello world", "hel"));
    assert!(command_matches_query("hello world", "wor"));
    assert!(command_matches_query("test command line", "comm"));
}

// Word boundary tests with underscores
#[test]
fn test_underscore_separated_words() {
    assert!(command_matches_query("hello_world", "hello"));
    assert!(command_matches_query("hello_world", "world"));
    assert!(command_matches_query("my_test_command", "test"));
    assert!(command_matches_query("open_file_manager", "file"));
    assert!(command_matches_query("hello_world", "helloworld"));
}

#[test]
fn test_underscore_separated_prefix() {
    assert!(command_matches_query("hello_world", "hel"));
    assert!(command_matches_query("hello_world", "wor"));
    assert!(command_matches_query("test_command_line", "comm"));
}

// Mixed separators tests
#[test]
fn test_mixed_separators() {
    assert!(command_matches_query("hello world_test", "hello"));
    assert!(command_matches_query("hello world_test", "world"));
    assert!(command_matches_query("hello world_test", "test"));
    assert!(command_matches_query("my_command line", "command"));
    assert!(command_matches_query("open file_manager", "manager"));
}

// Multi-word query tests
#[test]
fn test_multi_word_queries() {
    assert!(command_matches_query("hello world", "hello world"));
    assert!(command_matches_query("hello_world", "hello world"));
    assert!(command_matches_query("my test command", "my test"));
    assert!(command_matches_query("open file manager", "file manager"));
}

#[test]
fn test_multi_word_partial_match() {
    assert!(command_matches_query("hello world test", "hel wor"));
    assert!(command_matches_query("my_test_command", "my test"));
    assert!(command_matches_query("open file manager", "open file"));
    assert!(command_matches_query("command_line_interface", "command line"));
}

#[test]
fn test_multi_word_no_match() {
    assert!(!command_matches_query("hello world", "world hello"));
    assert!(!command_matches_query("test command", "command test"));
    assert!(!command_matches_query("hello", "hello world"));
}

// Sequential character matching tests
#[test]
fn test_sequential_character_matching() {
    assert!(command_matches_query("hello", "hel"));
    assert!(command_matches_query("testing", "test"));
    assert!(command_matches_query("command", "com"));
    assert!(command_matches_query("application", "app"));
}

#[test]
fn test_sequential_across_words() {
    assert!(command_matches_query("hello world", "helloworld"));
    assert!(command_matches_query("test_command", "testcommand"));
    assert!(command_matches_query("my test app", "mytestapp"));
    assert!(command_matches_query("open_file_manager", "openfilemanager"));
}

#[test]
fn test_sequential_no_match() {
    assert!(!command_matches_query("hello", "hlelo")); // wrong order
    assert!(!command_matches_query("test", "tset"));   // wrong order
    assert!(!command_matches_query("hello", "xyz"));   // no matching chars
}

// Edge cases
#[test]
fn test_special_characters() {
    assert!(command_matches_query("test-command", "test"));
    assert!(command_matches_query("test-command", "test-command"));
    assert!(command_matches_query("file@server", "file"));
    assert!(command_matches_query("user:password", "user"));
}

#[test]
fn test_numbers() {
    assert!(command_matches_query("test123", "test"));
    assert!(command_matches_query("test123", "test123"));
    assert!(command_matches_query("file2backup", "file"));
    assert!(command_matches_query("version2.1", "version"));
}

#[test]
fn test_single_character_queries() {
    assert!(command_matches_query("test", "t"));
    assert!(command_matches_query("hello", "h"));
    assert!(command_matches_query("application", "a"));
    assert!(!command_matches_query("test", "x"));
}

#[test]
fn test_single_character_commands() {
    assert!(command_matches_query("a", "a"));
    assert!(command_matches_query("x", "x"));
    assert!(!command_matches_query("a", "b"));
    assert!(command_matches_query("a", ""));
}

// Performance edge cases
#[test]
fn test_long_strings() {
    let long_command = "very_long_command_name_with_many_words_and_underscores_here";
    assert!(command_matches_query(long_command, "very"));
    assert!(command_matches_query(long_command, "command"));
    assert!(command_matches_query(long_command, "here"));
    assert!(command_matches_query(long_command, "vlcnwmwau"));
}

#[test]
fn test_repeated_characters() {
    assert!(command_matches_query("hello", "hel"));
    assert!(command_matches_query("bookkeeper", "book"));
    assert!(command_matches_query("mississippi", "miss"));
    assert!(command_matches_query("aaa", "aa"));
}

// Real-world examples
#[test]
fn test_realistic_commands() {
    // File operations
    assert!(command_matches_query("open_file", "open"));
    assert!(command_matches_query("save_document", "save"));
    assert!(command_matches_query("copy_to_clipboard", "copy"));
    
    // System commands
    assert!(command_matches_query("system_preferences", "sys"));
    assert!(command_matches_query("network_settings", "net"));
    assert!(command_matches_query("disk_utility", "disk"));
    
    // Development commands
    assert!(command_matches_query("git_commit", "git"));
    assert!(command_matches_query("run_tests", "test"));
    assert!(command_matches_query("build_project", "build"));
    
    // Application shortcuts
    assert!(command_matches_query("chrome_browser", "chrome"));
    assert!(command_matches_query("text_editor", "editor"));
    assert!(command_matches_query("terminal_app", "term"));
}

#[test]
fn test_partial_word_matching() {
    assert!(command_matches_query("browser", "brow"));
    assert!(command_matches_query("terminal", "term"));
    assert!(command_matches_query("application", "app"));
    assert!(command_matches_query("configuration", "config"));
    assert!(command_matches_query("documentation", "doc"));
}

// Stress tests
#[test]
fn test_whitespace_handling() {
    assert!(command_matches_query("  hello  world  ", "hello"));
    assert!(command_matches_query("test", "  test  "));
    assert!(command_matches_query("  spaced  command  ", "spaced command"));
}

#[test]
fn test_unicode_support() {
    assert!(command_matches_query("café", "café"));
    assert!(command_matches_query("naïve", "naïve"));
    assert!(command_matches_query("résumé", "résumé"));
}

#[test]
fn test_word_boundary_matching() {
    // Test that matching only starts at word boundaries
    // "amap" should match "ama page" but not "Roadmap"
    assert!(command_matches_query("ama page", "amap"), "Should match: ama page -> amap");
    assert!(!command_matches_query("Roadmap", "amap"), "Should NOT match: Roadmap -> amap");
    assert!(command_matches_query("AMA.Page", "amap"), "Should match: AMA.Page -> amap");
    
    // Additional word boundary tests
    assert!(!command_matches_query("abc defg", "bcd"), "Should NOT match: abc defg -> bcd (b not at word start)");
    assert!(command_matches_query("abc def", "ad"), "Should match: abc def -> ad (a at start, d at word start)");
    assert!(!command_matches_query("test app", "sta"), "Should NOT match: test app -> sta (s not at word start)");
}

// Merge similar commands tests
#[ignore] // Disabled for now - focusing on simple grouping first
#[test]  
fn test_merge_similar_commands() {
    // Test case 1: Commands with similar prefixes after search match
    let commands = vec![
        Command {
            group: String::new(),
            command: "FIN Retire Data".to_string(),
            action: "folder".to_string(),
            arg: "~/retire/data".to_string(),
            full_line: "FIN Retire Data : folder ~/retire/data".to_string(),
        },
        Command {
            group: String::new(),
            command: "FIN Retire Folder".to_string(),
            action: "folder".to_string(),
            arg: "~/retire".to_string(),
            full_line: "FIN Retire Folder : folder ~/retire".to_string(),
        },
        Command {
            group: String::new(),
            command: "FIN Retire Obsidian".to_string(),
            action: "obs".to_string(),
            arg: "Retire".to_string(),
            full_line: "FIN Retire Obsidian : obs Retire".to_string(),
        },
        Command {
            group: String::new(),
            command: "FIN Budget".to_string(),
            action: "folder".to_string(),
            arg: "~/budget".to_string(),
            full_line: "FIN Budget : folder ~/budget".to_string(),
        },
    ];
    
    // When searching for "finr", it should merge the three "FIN Retire" commands
    // First filter, then merge (like the actual app does)
    let filtered = filter_commands(&commands, "finr", 10, false);
    let merged = merge_similar_commands(&filtered, "finr");
    
    // Should have 1 command: "FIN Retire..." (FIN Budget doesn't match "finr")
    assert_eq!(merged.len(), 1);
    
    // Find the merged command
    let retire_merged = &merged[0];
    assert_eq!(retire_merged.command, "FIN Retire...");
    // Should use the first command's action and arg
    assert_eq!(retire_merged.action, "folder");
    assert_eq!(retire_merged.arg, "~/retire/data");
}

#[test]
fn test_merge_similar_no_whitespace_after_match() {
    // Test when there's no whitespace after the match
    let commands = vec![
        Command {
            group: String::new(),
            command: "test".to_string(),
            action: "app".to_string(),
            arg: "Test".to_string(),
            full_line: "test : app Test".to_string(),
        },
        Command {
            group: String::new(),
            command: "testing".to_string(),
            action: "app".to_string(),
            arg: "Testing".to_string(),
            full_line: "testing : app Testing".to_string(),
        },
    ];
    
    let merged = merge_similar_commands(&commands, "test");
    
    // Should not merge because there's no whitespace after "test" in either command
    assert_eq!(merged.len(), 2);
    assert!(merged.iter().all(|c| !c.command.ends_with("...")));
}

#[test]
fn test_merge_similar_project_commands() {
    // Test that commands with same prefix get merged
    let commands = vec![
        Command {
            group: String::new(),
            command: "project web frontend".to_string(),
            action: "folder".to_string(),
            arg: "~/proj/web/front".to_string(),
            full_line: "project web frontend : folder ~/proj/web/front".to_string(),
        },
        Command {
            group: String::new(),
            command: "project mobile app".to_string(),
            action: "folder".to_string(),
            arg: "~/proj/mobile".to_string(),
            full_line: "project mobile app : folder ~/proj/mobile".to_string(),
        },
    ];
    
    let merged = merge_similar_commands(&commands, "proj");
    
    // Should merge because both have "project " as prefix after match
    assert_eq!(merged.len(), 1);
    let merged_cmd = &merged[0];
    assert_eq!(merged_cmd.command, "project...");
    assert_eq!(merged_cmd.action, "folder"); // Should use first command's action
    assert_eq!(merged_cmd.arg, "~/proj/web/front"); // Should use first command's arg
}

#[test] 
fn test_merge_similar_different_prefixes() {
    // Test that commands with different prefixes don't get merged
    let commands = vec![
        Command {
            group: String::new(),
            command: "project web frontend".to_string(),
            action: "folder".to_string(),
            arg: "~/proj/web/front".to_string(),
            full_line: "project web frontend : folder ~/proj/web/front".to_string(),
        },
        Command {
            group: String::new(),
            command: "prototype mobile app".to_string(),
            action: "folder".to_string(),
            arg: "~/proto/mobile".to_string(),
            full_line: "prototype mobile app : folder ~/proto/mobile".to_string(),
        },
    ];
    
    let merged = merge_similar_commands(&commands, "pro");
    
    // Should not merge because "project " != "prototype "
    assert_eq!(merged.len(), 2);
    assert!(merged.iter().all(|c| !c.command.ends_with("...")));
}

#[test]
fn test_debug_merge_analysis_bills() {
    // Debug test for the user's issue
    let commands = vec![
        Command {
            group: String::new(),
            command: "analysis daily".to_string(),
            action: "obs".to_string(),
            arg: "Analysis Daily".to_string(),
            full_line: "analysis daily : obs Analysis Daily".to_string(),
        },
        Command {
            group: String::new(),
            command: "analysis weekly".to_string(),
            action: "obs".to_string(),
            arg: "Analysis Weekly".to_string(),
            full_line: "analysis weekly : obs Analysis Weekly".to_string(),
        },
        Command {
            group: String::new(),
            command: "BILLS monthly".to_string(),
            action: "folder".to_string(),
            arg: "~/bills".to_string(),
            full_line: "BILLS monthly : folder ~/bills".to_string(),
        },
        Command {
            group: String::new(),
            command: "BILLS yearly".to_string(),
            action: "folder".to_string(),
            arg: "~/bills/yearly".to_string(),
            full_line: "BILLS yearly : folder ~/bills/yearly".to_string(),
        },
    ];

    println!("\n=== Debug: 'anal' search ===");
    let filtered_anal = filter_commands(&commands, "anal", 10, false);
    println!("After filtering: {} commands", filtered_anal.len());
    for cmd in &filtered_anal {
        println!("  - {}", cmd.command);
    }
    
    // Debug the grouping logic for 'anal'
    println!("Debug grouping for 'anal':");
    for cmd in &filtered_anal {
        let cmd_lower = cmd.command.to_lowercase();
        let search_lower = "anal";
        if let Some(search_end_pos) = cmd_lower.find(&search_lower) {
            let starts_with_search = search_end_pos == 0;
            println!("  '{}' -> search_end_pos={}, starts_with_search={}", cmd.command, search_end_pos, starts_with_search);
        }
    }
    
    let merged_anal = merge_similar_commands(&filtered_anal, "anal");
    println!("After merging: {} commands", merged_anal.len());
    for (i, cmd) in merged_anal.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", cmd.command);
    }
    println!();
    
    println!("\n=== Debug: 'BILLS' search ===");
    let filtered_bills = filter_commands(&commands, "BILLS", 10, false);
    println!("After filtering: {} commands", filtered_bills.len());
    for cmd in &filtered_bills {
        println!("  - {}", cmd.command);
    }
    
    // Debug the grouping logic
    println!("Debug grouping for 'BILLS':");
    for cmd in &filtered_bills {
        let cmd_lower = cmd.command.to_lowercase();
        let search_lower = "bills";
        if let Some(search_end_pos) = cmd_lower.find(&search_lower) {
            let after_search_start = search_end_pos + "BILLS".len();
            let after_search = &cmd.command[after_search_start..];
            let after_search_trimmed = after_search.trim_start();
            let group_key = if after_search_trimmed.is_empty() {
                "".to_string()
            } else if let Some(space_pos) = after_search_trimmed.find(' ') {
                after_search_trimmed[..space_pos].to_string()
            } else {
                after_search_trimmed.to_string()
            };
            println!("  '{}' -> after_search='{}' -> group_key='{}'", cmd.command, after_search_trimmed, group_key);
        }
    }
    
    let merged_bills = merge_similar_commands(&filtered_bills, "BILLS");
    println!("After merging: {} commands", merged_bills.len());
    for (i, cmd) in merged_bills.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", cmd.command);
    }
    println!();
    
    // For 'anal' searching, should merge the analysis commands
    assert!(merged_anal.iter().any(|c| c.command == "analysis..."));
    assert!(!merged_anal.iter().any(|c| c.command == "analysis daily"));
    assert!(!merged_anal.iter().any(|c| c.command == "analysis weekly"));
    
    // For 'BILLS' searching, should merge the BILLS commands  
    assert!(merged_bills.iter().any(|c| c.command == "BILLS..."));
    assert!(!merged_bills.iter().any(|c| c.command == "BILLS monthly"));
    assert!(!merged_bills.iter().any(|c| c.command == "BILLS yearly"));
}

#[ignore] // Disabled for now - focusing on simple grouping first  
#[test]
fn test_submenu_aware_merging() {
    // Test case for the user's "fin" issue - should be submenu-aware
    let commands = vec![
        Command {
            group: String::new(),
            command: "FIN".to_string(),
            action: "folder".to_string(),
            arg: "~/fin".to_string(),
            full_line: "FIN : folder ~/fin".to_string(),
        },
        Command {
            group: String::new(),
            command: "FIN Retire Data".to_string(),
            action: "folder".to_string(),
            arg: "~/retire/data".to_string(),
            full_line: "FIN Retire Data : folder ~/retire/data".to_string(),
        },
        Command {
            group: String::new(),
            command: "FIN Retire Folder".to_string(),
            action: "folder".to_string(),
            arg: "~/retire".to_string(),
            full_line: "FIN Retire Folder : folder ~/retire".to_string(),
        },
        Command {
            group: String::new(),
            command: "FIN Budget".to_string(),
            action: "folder".to_string(),
            arg: "~/budget".to_string(),
            full_line: "FIN Budget : folder ~/budget".to_string(),
        },
    ];

    // Filter then merge for "fin" search (should create submenu)
    let filtered = filter_commands(&commands, "fin", 10, false);
    let merged = merge_similar_commands(&filtered, "fin");
    
    
    // Should have hierarchical merging: FIN + merged sub-groups
    // FIN, Analysis..., Bills..., Budget, Retire...
    assert_eq!(merged.len(), 5);
    
    // Should have the main FIN command
    assert!(merged.iter().any(|c| c.command == "FIN"));
    
    // Should have merged sub-groups
    assert!(merged.iter().any(|c| c.command == "Analysis..."));
    assert!(merged.iter().any(|c| c.command == "Bills..."));
    assert!(merged.iter().any(|c| c.command == "Budget"));  // Single item, not merged
    assert!(merged.iter().any(|c| c.command == "Retire..."));
    
    // Should NOT have the original long commands
    assert!(!merged.iter().any(|c| c.command == "FIN Analysis"));
    assert!(!merged.iter().any(|c| c.command == "FIN Bills"));
    assert!(!merged.iter().any(|c| c.command == "FIN Retire Data"));
}

#[ignore] // Disabled for now - focusing on simple grouping first
#[test]
fn test_merge_visual_examples() {
    // Test case 1: "fin" search in submenu mode - should show individual commands
    println!("\n=== Test 1: 'fin' search (submenu mode) ===");
    let commands1 = vec![
        cmd("FIN", "folder", "~/fin"),
        cmd("FIN Analysis", "folder", "~/analysis"), 
        cmd("FIN Analysis Folder", "folder", "~/analysis/folder"),
        cmd("FIN Analysis Note", "obs", "Analysis"),
        cmd("FIN Bills", "folder", "~/bills"),
        cmd("FIN Bills Folder", "folder", "~/bills/folder"), 
        cmd("FIN Bills Note", "obs", "Bills"),
        cmd("FIN Budget", "folder", "~/budget"),
        cmd("FIN Retire Data", "folder", "~/retire/data"),
        cmd("FIN Retire Folder", "folder", "~/retire"),
    ];
    
    let filtered1 = filter_commands(&commands1, "fin", 20, false);
    let merged1 = merge_similar_commands(&filtered1, "fin");
    
    println!("Input commands: [FIN, FIN Analysis, FIN Analysis Folder, FIN Analysis Note, FIN Bills, FIN Bills Folder, FIN Bills Note, FIN Budget, FIN Retire Data, FIN Retire Folder]");
    println!("Expected: [FIN, Analysis..., Bills..., Budget, Retire...] (hierarchical merging in submenu)");
    print!("Actual:   [");
    for (i, cmd) in merged1.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", cmd.command);
    }
    println!("]");
    
    println!("Actual result count: {}", merged1.len());
    
    // Test case 2: "analysis" search - should merge Analysis commands
    println!("\n=== Test 2: 'analysis' search ===");
    let commands2 = vec![
        cmd("Analysis", "folder", "~/analysis"),
        cmd("Analysis Folder", "folder", "~/analysis/folder"), 
        cmd("Analysis Note", "obs", "Analysis"),
    ];
    
    let filtered2 = filter_commands(&commands2, "analysis", 20, false);
    println!("DEBUG: Submenu prefix for 'analysis': {:?}", get_submenu_prefix(&filtered2, "analysis"));
    println!("DEBUG: Submenu positions for 'analysis': {:?}", get_submenu_display_positions(&filtered2, "analysis"));
    let merged2 = merge_similar_commands(&filtered2, "analysis");
    
    println!("Input commands: [Analysis, Analysis Folder, Analysis Note]");
    println!("Expected: [Analysis...] (merge all Analysis commands)");
    print!("Actual:   [");
    for (i, cmd) in merged2.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", cmd.command);
    }
    println!("]");
    
    // For now, let's see what actually happens and adjust our expectations
    println!("Analysis result count: {}", merged2.len());
    
    // Test case 3: Financial model merging
    println!("\n=== Test 3: '2007-02-00' search ===");
    let commands3 = vec![
        cmd("2007-02-00 New Financial Model", "obs", "Model1"),
        cmd("2007-02-00 New Financial Model Note", "obs", "Model Note"),
        cmd("2007-02-00 New Financial Model Obsidian", "obs", "Model Obs"),
    ];
    
    let filtered3 = filter_commands(&commands3, "2007-02-00", 20, false);
    let merged3 = merge_similar_commands(&filtered3, "2007-02-00");
    
    println!("Input commands: [2007-02-00 New Financial Model, 2007-02-00 New Financial Model Note, 2007-02-00 New Financial Model Obsidian]");
    println!("Expected: [2007-02-00 New Financial Model...] (merge at Model level)");
    print!("Actual:   [");
    for (i, cmd) in merged3.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", cmd.command);
    }
    println!("]");
    
    assert_eq!(merged3.len(), 1);
    assert_eq!(merged3[0].command, "2007-02-00 New Financial Model...");
    
    // Test case 4: Mixed commands with different prefixes
    println!("\n=== Test 4: '2007-02-00 new' search with mixed commands ===");
    let commands4 = vec![
        cmd("2007-02-00 New Financial Model", "obs", "Model1"),
        cmd("2007-02-00 New Financial Model Note", "obs", "Model Note"), 
        cmd("2007-02-00 New Analysis Report", "obs", "Analysis"),
    ];
    
    let filtered4 = filter_commands(&commands4, "2007-02-00 new", 20, false);
    let merged4 = merge_similar_commands(&filtered4, "2007-02-00 new");
    
    println!("Input commands: [2007-02-00 New Financial Model, 2007-02-00 New Financial Model Note, 2007-02-00 New Analysis Report]");
    println!("Expected: [2007-02-00 New...] (merge at New level since Financial != Analysis)");
    print!("Actual:   [");
    for (i, cmd) in merged4.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", cmd.command);
    }
    println!("]");
    
    assert_eq!(merged4.len(), 1);
    assert_eq!(merged4[0].command, "2007-02-00 New...");
}

#[test]
fn test_fin_no_incorrect_merging() {
    // Test case: when searching "FIN", commands "FIN analysis" and "FIN flows" should NOT be merged
    // because there's only one of each type
    println!("\n=== Test: 'FIN' search should not merge single commands ===");
    
    // Simulate commands similar to what's in the screenshot
    let commands = vec![
        cmd("FIN", "folder", "~/fin"),
        cmd("FIN Budget Obsidian", "obs", "Budget"),
        cmd("FIN Flows Analysis", "obs", "Flows"),
        cmd("FIN Log Daily", "obs", "Log"),
        cmd("FIN Recurring Obsidian", "obs", "Recurring"),
        cmd("FIN Tracking Obsidian", "obs", "Tracking"),
        cmd("FIN Main", "obs", "Main"),
        cmd("FIN SV Analysis", "obs", "SV"),
    ];
    
    println!("Input commands: multiple FIN commands from screenshot");
    
    let filtered = filter_commands(&commands, "fin", 10, false); // lowercase like in user's screenshot
    println!("After filtering: {} commands", filtered.len());
    for cmd in &filtered {
        println!("  - {}", cmd.command);
    }
    
    let merged = merge_similar_commands(&filtered, "fin"); // lowercase like in user's screenshot
    println!("After merging: {} commands", merged.len());
    for cmd in &merged {
        println!("  - {} (action: {}, arg: {})", cmd.command, cmd.action, cmd.arg);
    }
    
    // Should NOT merge into one big "FIN..." - each command should remain separate since there's only one of each type
    // We expect 8 separate commands, no merging should happen
    assert_eq!(merged.len(), 8);
    assert!(merged.iter().any(|c| c.command == "FIN"));
    assert!(merged.iter().any(|c| c.command == "FIN Budget Obsidian"));
    assert!(merged.iter().any(|c| c.command == "FIN Flows Analysis"));
    assert!(merged.iter().any(|c| c.command == "FIN Log Daily"));
    
    // Should NOT have any merged entries since each is unique
    assert!(!merged.iter().any(|c| c.command.ends_with("...")));
}

#[test]
fn test_fin_correct_merging() {
    // Test case: when there ARE multiple commands with same word after search text, they SHOULD merge
    println!("\n=== Test: 'fin' search with correct merging ===");
    
    let commands = vec![
        cmd("FIN Budget Daily", "obs", "Budget Daily"),
        cmd("FIN Budget Weekly", "obs", "Budget Weekly"),
        cmd("FIN Budget Monthly", "obs", "Budget Monthly"),
        cmd("FIN Analysis Note", "obs", "Analysis"),
        cmd("FIN Flows Report", "obs", "Flows"),
    ];
    
    println!("Input commands with multiple Budget entries");
    
    let filtered = filter_commands(&commands, "fin", 10, false);
    println!("After filtering: {} commands", filtered.len());
    for cmd in &filtered {
        println!("  - {}", cmd.command);
    }
    
    let merged = merge_similar_commands(&filtered, "fin");
    println!("After merging: {} commands", merged.len());
    for cmd in &merged {
        println!("  - {} (action: {}, arg: {})", cmd.command, cmd.action, cmd.arg);
    }
    
    // Should merge the 3 Budget commands into "FIN Budget..."
    assert_eq!(merged.len(), 3); // Budget merged + Analysis + Flows individual
    assert!(merged.iter().any(|c| c.command == "FIN Budget..."));
    assert!(merged.iter().any(|c| c.command == "FIN Analysis Note"));
    assert!(merged.iter().any(|c| c.command == "FIN Flows Report"));
    
    // Should NOT merge the single commands
    assert!(!merged.iter().any(|c| c.command == "FIN Budget Daily"));
    assert!(!merged.iter().any(|c| c.command == "FIN Budget Weekly"));
    assert!(!merged.iter().any(|c| c.command == "FIN Budget Monthly"));
}

#[test]
fn test_fin_simple_grouping() {
    // Test case for simple grouping based on first word after search text
    println!("\n=== Test: 'fin' search with simple grouping ===");
    
    // Create test commands similar to what user showed in screenshot
    // Commands that start with "FIN " should be grouped by the next word
    let commands = vec![
        cmd("FIN", "folder", "~/fin"),
        cmd("FIN Budget Obsidian", "obs", "Budget"),
        cmd("FIN Budget Note", "obs", "Budget Note"),
        cmd("FIN Flows Analysis", "obs", "Flows"),
        cmd("FIN Log Daily", "obs", "Log"),
        cmd("FIN Log Weekly", "obs", "Log"),
        cmd("Findem", "app", "Findem"),  // No space after "fin"
        cmd("Finicky App", "app", "Finicky"), // No space after "fin"
    ];
    
    println!("Input commands similar to screenshot");
    
    let filtered = filter_commands(&commands, "fin", 10, false);
    println!("After filtering: {} commands", filtered.len());
    for cmd in &filtered {
        println!("  - {}", cmd.command);
    }
    
    let merged = merge_similar_commands(&filtered, "fin");
    println!("After merging: {} commands", merged.len());
    for cmd in &merged {
        println!("  - {} (action: {}, arg: {})", cmd.command, cmd.action, cmd.arg);
    }
    
    // Should group FIN Budget commands together (2 commands)
    assert!(merged.iter().any(|c| c.command == "FIN Budget..."));
    
    // Should group FIN Log commands together (2 commands) 
    assert!(merged.iter().any(|c| c.command == "FIN Log..."));
    
    // Should keep individual commands that don't have groups
    assert!(merged.iter().any(|c| c.command == "FIN")); // Single command
    assert!(merged.iter().any(|c| c.command == "FIN Flows Analysis")); // Single command
    assert!(merged.iter().any(|c| c.command == "Findem")); // No space after "fin"
    assert!(merged.iter().any(|c| c.command == "Finicky App")); // No space after "fin"
}

// Helper function to create commands more concisely
fn cmd(command: &str, action: &str, arg: &str) -> Command {
    Command {
        group: String::new(),
        command: command.to_string(),
        action: action.to_string(),
        arg: arg.to_string(),
        full_line: format!("{} : {} {}", command, action, arg),
    }
}