//! Test helper functions for display logic testing
//!
//! This module provides utilities for:
//! - Parsing command scaffolds from multi-line strings
//! - Creating test fixtures (commands, patches, config)
//! - Assertion helpers for verifying menu behavior

use hookanchor::core::{Command, Patch, Config, parse_command_line, create_patches_hashmap};
use std::collections::HashMap;
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize global config for tests
fn init_test_config() {
    INIT.call_once(|| {
        // Initialize minimal config for testing
        let _ = hookanchor::core::initialize_config();
    });
}

/// Test scaffold containing all data needed for display tests
pub struct TestScaffold {
    pub commands: Vec<Command>,
    pub patches: HashMap<String, Patch>,
    pub config: Config,
}

/// Parse multi-line command definitions into Vec<Command>
///
/// Each line should be a valid command in the format:
/// `command:action flags; arg`
///
/// Empty lines and lines starting with "//" are skipped
pub fn parse_commands(text: &str) -> Vec<Command> {
    let mut commands = Vec::new();

    for (line_num, line) in text.lines().enumerate() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        match parse_command_line(trimmed) {
            Ok(command) => commands.push(command),
            Err(e) => {
                eprintln!("Warning: Failed to parse line {}: {}", line_num + 1, e);
                eprintln!("  Line: {}", trimmed);
            }
        }
    }

    commands
}

/// Create patches HashMap from anchor commands
///
/// Uses the same logic as the production code
pub fn create_patches(commands: &[Command]) -> HashMap<String, Patch> {
    create_patches_hashmap(commands)
}

/// Create a minimal test Config with specified word_separators
///
/// # Arguments
/// * `word_separators` - String of separator characters (e.g., " ._-!")
pub fn test_config(word_separators: &str) -> Config {
    // Create minimal config - most fields can be default/empty
    // The key field we care about is word_separators
    let mut config = Config::default();
    config.popup_settings.word_separators = word_separators.to_string();
    config
}

/// Create default test config with standard separators
pub fn default_config() -> Config {
    test_config(" ._-!")
}

/// One-shot scaffold creation from command text
///
/// # Arguments
/// * `commands_text` - Multi-line string with command definitions
///
/// # Returns
/// TestScaffold with parsed commands, generated patches, and default config
pub fn scaffold(commands_text: &str) -> TestScaffold {
    init_test_config(); // Ensure global config is initialized

    let commands = parse_commands(commands_text);
    let patches = create_patches(&commands);
    let config = default_config();

    TestScaffold {
        commands,
        patches,
        config,
    }
}

// ============================================================================
// ASSERTION HELPERS
// ============================================================================

/// Assert that a command with the given name appears in the result
pub fn assert_contains(result: &[Command], name: &str) {
    let found = result.iter().any(|cmd| cmd.command == name);
    assert!(found, "Expected to find command '{}' in result, but it was not present.\nActual commands: {:?}",
        name,
        result.iter().map(|c| &c.command).collect::<Vec<_>>()
    );
}

/// Assert that a command with the given name does NOT appear in the result
pub fn assert_not_contains(result: &[Command], name: &str) {
    let found = result.iter().any(|cmd| cmd.command == name);
    assert!(!found, "Expected NOT to find command '{}' in result, but it was present.\nActual commands: {:?}",
        name,
        result.iter().map(|c| &c.command).collect::<Vec<_>>()
    );
}

/// Assert that first_name appears before second_name in the result
pub fn assert_order(result: &[Command], first_name: &str, second_name: &str) {
    let first_idx = result.iter().position(|cmd| cmd.command == first_name);
    let second_idx = result.iter().position(|cmd| cmd.command == second_name);

    match (first_idx, second_idx) {
        (Some(first), Some(second)) => {
            assert!(first < second,
                "Expected '{}' (at index {}) to appear before '{}' (at index {}).\nActual order: {:?}",
                first_name, first, second_name, second,
                result.iter().map(|c| &c.command).collect::<Vec<_>>()
            );
        }
        (None, _) => panic!("Command '{}' not found in result", first_name),
        (_, None) => panic!("Command '{}' not found in result", second_name),
    }
}

/// Assert that the result contains exactly the given command names in exact order
pub fn assert_exact_names(result: &[Command], expected: &[&str]) {
    let actual: Vec<&str> = result.iter().map(|c| c.command.as_str()).collect();
    assert_eq!(actual, expected,
        "Result does not match expected command names.\nExpected: {:?}\nActual: {:?}",
        expected, actual
    );
}

/// Assert that a separator appears at the given index
pub fn assert_separator_at(result: &[Command], index: usize) {
    assert!(index < result.len(),
        "Index {} is out of bounds (result has {} commands)", index, result.len());

    let cmd = &result[index];
    assert_eq!(cmd.action, "separator",
        "Expected separator at index {}, but found command '{}' with action '{}'",
        index, cmd.command, cmd.action
    );
    assert_eq!(cmd.command, "============",
        "Separator at index {} has wrong command name: '{}'", index, cmd.command);
}

/// Find the index of a command by name
///
/// # Returns
/// * `Some(index)` if found
/// * `None` if not found
pub fn find_index(result: &[Command], name: &str) -> Option<usize> {
    result.iter().position(|cmd| cmd.command == name)
}

/// Assert that is_prefix_menu matches expected value
pub fn assert_prefix_menu(is_prefix: bool, expected: bool) {
    assert_eq!(is_prefix, expected,
        "Expected is_prefix_menu={}, but got {}", expected, is_prefix);
}

/// Assert that prefix_menu_count matches expected value
pub fn assert_prefix_count(count: usize, expected: usize) {
    assert_eq!(count, expected,
        "Expected prefix_menu_count={}, but got {}", expected, count);
}

/// Assert that the result has exactly N commands
pub fn assert_count(result: &[Command], expected: usize) {
    assert_eq!(result.len(), expected,
        "Expected {} commands, but got {}\nActual commands: {:?}",
        expected, result.len(),
        result.iter().map(|c| &c.command).collect::<Vec<_>>()
    );
}
