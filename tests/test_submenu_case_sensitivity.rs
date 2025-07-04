//! Test for case sensitivity bug in submenu detection
//! 
//! This test ensures that commands with the same prefix but different cases
//! are properly grouped together for submenu detection.

use anchor_selector::core::commands::{Command, get_current_submenu_prefix_from_commands};

fn create_command(command: &str, action: &str, arg: &str) -> Command {
    Command {
        group: "".to_string(),
        command: command.to_string(),
        action: action.to_string(),
        arg: arg.to_string(),
        flags: "".to_string(),
        full_line: format!("{} : {} {}", command, action, arg),
    }
}

#[test]
fn test_case_insensitive_submenu_detection() {
    // Test case: Mixed case commands should be grouped together
    let commands = vec![
        create_command("cc core", "app", "cc core"),      // lowercase
        create_command("Cc Coach", "app", "Cc Coach"),    // mixed case
        create_command("CC Tool", "app", "CC Tool"),      // uppercase
        create_command("Different App", "app", "Different"), // unrelated
    ];

    let word_separators = " ._-";
    let search_text = "CC";

    // Should detect submenu because all 3 commands have "cc" prefix (case-insensitive)
    let result = get_current_submenu_prefix_from_commands(&commands, search_text, word_separators);
    
    assert!(result.is_some(), "Should detect submenu with mixed case CC prefixes");
    assert_eq!(result.unwrap(), "CC", "Should normalize to search text case");
}

#[test]
fn test_space_requires_two_plus_commands() {
    // Test case: Even with space, should only show submenu if 2+ commands exist with prefix
    let commands = vec![
        create_command("CC Tool", "app", "CC Tool"),         // only 1 CC command
        create_command("Different App", "app", "Different"), // no CC prefix
        create_command("Another App", "app", "Another"),     // no CC prefix
    ];

    let word_separators = " ._-";
    let search_text = "CC ";

    // Should NOT detect submenu because only 1 command has CC prefix
    let result = get_current_submenu_prefix_from_commands(&commands, search_text, word_separators);
    
    assert!(result.is_none(), "Should not detect submenu with only 1 matching command, even with space");
}

#[test]
fn test_space_with_sufficient_commands() {
    // Test case: With space and 2+ commands, should show submenu
    let commands = vec![
        create_command("CC Tool", "app", "CC Tool"),
        create_command("CC Core", "app", "CC Core"),         // 2 CC commands
        create_command("Different App", "app", "Different"), 
    ];

    let word_separators = " ._-";
    let search_text = "CC ";

    // Should detect submenu because 2+ commands have CC prefix
    let result = get_current_submenu_prefix_from_commands(&commands, search_text, word_separators);
    
    assert!(result.is_some(), "Should detect submenu with 2+ matching commands, with space");
    assert_eq!(result.unwrap(), "CC", "Should use the prefix before the space");
}

#[test]
fn test_no_false_submenu_with_insufficient_matches() {
    // Test case: Only one command with matching prefix should not trigger submenu (without space)
    let commands = vec![
        create_command("CC Tool", "app", "CC Tool"),
        create_command("Different App", "app", "Different"),
        create_command("Another App", "app", "Another"),
    ];

    let word_separators = " ._-";
    let search_text = "CC";

    let result = get_current_submenu_prefix_from_commands(&commands, search_text, word_separators);
    
    assert!(result.is_none(), "Should not detect submenu with only 1 matching command");
}

#[test]
fn test_submenu_threshold_with_mixed_case() {
    // Test case: Exactly 2 mixed case commands should trigger submenu
    let commands = vec![
        create_command("cc core", "app", "cc core"),      // lowercase - count 1
        create_command("CC Tool", "app", "CC Tool"),      // uppercase - count 2 (same group)
        create_command("Different App", "app", "Different"), // unrelated
    ];

    let word_separators = " ._-";
    let search_text = "CC";

    let result = get_current_submenu_prefix_from_commands(&commands, search_text, word_separators);
    
    assert!(result.is_some(), "Should detect submenu with 2 mixed case CC prefixes");
}

#[test]
fn test_case_preservation_in_result() {
    // Test case: Result should prefer exact case match with search text
    let commands = vec![
        create_command("cc core", "app", "cc core"),      // lowercase first
        create_command("CC Tool", "app", "CC Tool"),      // exact match with search
        create_command("Cc Coach", "app", "Cc Coach"),    // mixed case
    ];

    let word_separators = " ._-";
    let search_text = "CC";

    let result = get_current_submenu_prefix_from_commands(&commands, search_text, word_separators);
    
    assert_eq!(result.unwrap(), "CC", "Should prefer exact case match with search text");
}

#[test]
fn test_regression_no_cc_prefix_commands() {
    // Test case: Commands that don't actually have CC prefix should not trigger submenu
    // This reproduces the original user's issue
    let commands = vec![
        create_command("code_core *", "obs", "prj/binproj/code_core/code_core.md"),
        create_command("CAR Coach App", "app", "CAR Coach"),
        create_command("CV Confluence", "url", "https://confluence.example.com"),
        create_command("Cookie Clicker *", "obs", "games/Cookie Clicker.md"),
        create_command("Christmas Cards *", "obs", "personal/Christmas Cards.md"),
    ];

    let word_separators = " ._-";
    let search_text = "CC";

    let result = get_current_submenu_prefix_from_commands(&commands, search_text, word_separators);
    
    assert!(result.is_none(), "Should not detect submenu when no commands actually have CC prefix");
}