use anchor_selector::{command_matches_query, command_matches_query_with_debug};

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