//! Display Operations
//!
//! Functions for filtering, sorting, and displaying commands.
//! This module provides a clean API surface for the display system.

use crate::core::Command;
use crate::core::display;

// Re-export key types
pub use crate::ui::layout::PrefixMenuInfo;

// =============================================================================
// COMMAND FILTERING
// =============================================================================

/// Check if a command matches a search query.
///
/// Uses sophisticated word-boundary matching:
/// - Characters match beginnings of words in command name
/// - Characters must match in order (left-to-right)
/// - Matching is case-insensitive
///
/// # Arguments
/// * `command_name` - Name of command to check
/// * `query` - Search query string
///
/// # Returns
/// true if command matches the query
pub fn command_matches(command_name: &str, query: &str) -> bool {
    display::command_matches_query(command_name, query)
}

/// Check if two strings match exactly (ignoring whitespace and case).
///
/// # Arguments
/// * `str1` - First string
/// * `str2` - Second string
///
/// # Returns
/// true if strings are exact matches
pub fn exact_match(str1: &str, str2: &str) -> bool {
    display::exact_match(str1, str2)
}

/// Filter commands by search text with a maximum result count.
///
/// # Arguments
/// * `commands` - All commands to filter
/// * `search_text` - Search query
/// * `max_results` - Maximum number of results to return
///
/// # Returns
/// Filtered and sorted vector of matching commands
pub fn filter_commands(commands: &[Command], search_text: &str, max_results: usize) -> Vec<Command> {
    crate::core::commands::filter_commands(commands, search_text, max_results, false)
}

// =============================================================================
// DISPLAY LIST BUILDING
// =============================================================================

/// Result of building a prefix menu.
pub struct PrefixMenuResult {
    /// Commands in the prefix menu (children of anchor)
    pub menu_commands: Vec<Command>,
    /// The original typed command (e.g., "FB" alias)
    pub original_command: Option<Command>,
    /// The resolved anchor command (e.g., "Fireball")
    pub resolved_command: Option<Command>,
    /// The filter text (remaining input after anchor match)
    pub filter_text: String,
    /// Whether a prefix menu was found
    pub found: bool,
}

/// Build prefix menu for an input string.
///
/// Scans backwards through input to find an anchor match, then builds
/// the prefix menu with children of that anchor.
///
/// # Arguments
/// * `input` - Search input string
/// * `commands` - All available commands
/// * `config` - Application config
///
/// # Returns
/// PrefixMenuResult with menu commands and anchor info
pub fn build_prefix_menu(
    input: &str,
    commands: &[Command],
    config: &crate::core::Config,
) -> PrefixMenuResult {
    let patches = crate::core::commands::create_patches_hashmap(commands);
    match display::build_prefix_menu(input, commands, &patches, config) {
        Some((menu_commands, original, resolved, filter_text)) => {
            PrefixMenuResult {
                menu_commands,
                original_command: Some(original),
                resolved_command: Some(resolved),
                filter_text,
                found: true,
            }
        }
        None => {
            PrefixMenuResult {
                menu_commands: vec![],
                original_command: None,
                resolved_command: None,
                filter_text: String::new(),
                found: false,
            }
        }
    }
}

/// Result of building a display list.
pub struct DisplayListResult {
    /// Commands to display
    pub commands: Vec<Command>,
    /// Whether in prefix menu mode
    pub is_prefix_menu: bool,
    /// Prefix menu info (original, resolved command) if in prefix menu
    pub prefix_menu_info: Option<(Command, Command)>,
    /// Number of prefix menu commands
    pub prefix_menu_count: usize,
    /// Separator index (between prefix menu and other commands)
    pub separator_index: Option<usize>,
    /// Debug info
    pub debug_info: String,
}

/// Build the complete display list for a search input.
///
/// This is the main entry point for building the command list to display.
/// Handles both prefix menu mode and global search mode.
///
/// # Arguments
/// * `input` - Search input string
/// * `commands` - All available commands
/// * `config` - Application config
///
/// # Returns
/// DisplayListResult with all display information
pub fn build_display_list(
    input: &str,
    commands: &[Command],
    config: &crate::core::Config,
) -> DisplayListResult {
    let patches = crate::core::commands::create_patches_hashmap(commands);
    let (cmds, is_prefix, info, count, sep_idx, debug) =
        display::get_new_display_commands(input, commands, &patches, config);

    DisplayListResult {
        commands: cmds,
        is_prefix_menu: is_prefix,
        prefix_menu_info: info,
        prefix_menu_count: count,
        separator_index: sep_idx,
        debug_info: debug,
    }
}

// =============================================================================
// COMMAND SORTING
// =============================================================================

/// Sort order for commands.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortOrder {
    /// Sort by relevance to search query
    Relevance,
    /// Sort alphabetically by name
    Alphabetical,
    /// Sort by most recently used
    Recent,
}

/// Get the prefix of a command name (before any separator).
///
/// Separators are: space, dot, underscore, hyphen, exclamation.
///
/// # Arguments
/// * `command_name` - Command name to extract prefix from
///
/// # Returns
/// The prefix portion of the command name
pub fn get_command_prefix(command_name: &str) -> String {
    crate::core::commands::get_command_prefix(command_name, " ._-!")
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_matches() {
        assert!(command_matches("Project Tasks", "PT"));
        assert!(command_matches("Project Tasks", "proj"));
        assert!(command_matches("Project Tasks", "tasks"));
        assert!(!command_matches("Project Tasks", "xyz"));
    }

    #[test]
    fn test_exact_match() {
        assert!(exact_match("hello", "HELLO"));
        assert!(exact_match("hello world", "helloworld"));
        assert!(!exact_match("hello", "world"));
    }

    #[test]
    fn test_get_command_prefix() {
        assert_eq!(get_command_prefix("Project Tasks"), "Project");
        assert_eq!(get_command_prefix("Project.Sub"), "Project");
        assert_eq!(get_command_prefix("Single"), "Single");
    }
}
