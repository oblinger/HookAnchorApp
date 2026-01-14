//! Command Operations
//!
//! Functions for searching and filtering command collections.
//!
//! This module provides collection-level operations on commands.
//! For single-command properties and methods, use `Command` directly:
//! - `command.is_anchor()` - check if command has anchor flag
//! - `command.is_alias()` - check if action is "alias"
//! - `command.has_patch()` - check if command has a patch
//! - `command.is_path_based()` - check if command has path context
//! - `command.resolve_alias(commands)` - follow alias to target
//! - `command.get_absolute_folder_path(config)` - get folder path
//! - `command.get_absolute_file_path(config)` - get file path

use crate::core::Command;

// =============================================================================
// COMMAND FINDING
// =============================================================================

/// Find a command by name (case-insensitive).
///
/// # Arguments
/// * `name` - Command name to find
/// * `commands` - List of commands to search
///
/// # Returns
/// The matching command if found
pub fn find_command(name: &str, commands: &[Command]) -> Option<Command> {
    let name_lower = name.to_lowercase();
    commands.iter()
        .find(|cmd| cmd.command.to_lowercase() == name_lower)
        .cloned()
}

/// Find a command by exact name match (case-sensitive).
///
/// # Arguments
/// * `name` - Command name to find
/// * `commands` - List of commands to search
///
/// # Returns
/// The matching command if found
pub fn find_command_exact(name: &str, commands: &[Command]) -> Option<Command> {
    commands.iter()
        .find(|cmd| cmd.command == name)
        .cloned()
}

/// Find all commands matching a prefix (case-insensitive).
///
/// # Arguments
/// * `prefix` - Prefix to match
/// * `commands` - List of commands to search
///
/// # Returns
/// Vector of commands whose names start with the prefix
pub fn find_commands_by_prefix(prefix: &str, commands: &[Command]) -> Vec<Command> {
    let prefix_lower = prefix.to_lowercase();
    commands.iter()
        .filter(|cmd| cmd.command.to_lowercase().starts_with(&prefix_lower))
        .cloned()
        .collect()
}

/// Find all commands in a specific patch.
///
/// # Arguments
/// * `patch_name` - Patch name to filter by
/// * `commands` - List of commands to search
///
/// # Returns
/// Vector of commands in the specified patch
pub fn find_commands_in_patch(patch_name: &str, commands: &[Command]) -> Vec<Command> {
    let patch_lower = patch_name.to_lowercase();
    commands.iter()
        .filter(|cmd| cmd.patch.to_lowercase() == patch_lower)
        .cloned()
        .collect()
}

// =============================================================================
// COMMAND DATA ACCESS
// =============================================================================

/// Get all commands from the system data.
///
/// # Returns
/// Arc containing all loaded commands
pub fn get_all_commands() -> std::sync::Arc<Vec<Command>> {
    crate::core::data::get_commands_arc()
}

/// Get a command count.
///
/// # Returns
/// Number of commands loaded
pub fn count_commands() -> usize {
    crate::core::data::get_commands_arc().len()
}

// =============================================================================
// PATH EXTRACTION
// =============================================================================

/// Extract folder path from a command using dynamic action type lookup.
///
/// This differs from `Command::get_absolute_folder_path` in that it:
/// - Uses dynamic action type configuration instead of hardcoded action names
/// - Returns a simple String instead of PathBuf
/// - Doesn't require Config for path resolution
pub fn get_command_folder(cmd: &Command) -> Option<String> {
    let arg_type = crate::execute::get_action_arg_type(&cmd.action);

    match arg_type.as_deref() {
        Some("folder") => {
            if !cmd.arg.is_empty() {
                return Some(cmd.arg.clone());
            }
        }
        Some("file") => {
            if !cmd.arg.is_empty() {
                if let Some(parent) = std::path::Path::new(&cmd.arg).parent() {
                    return Some(parent.to_string_lossy().to_string());
                }
            }
        }
        _ => {}
    }
    None
}

/// Extract full file/folder path from a command using dynamic action type lookup.
///
/// Returns the arg if the action type is "file" or "folder".
pub fn get_command_path(cmd: &Command) -> Option<String> {
    let arg_type = crate::execute::get_action_arg_type(&cmd.action);

    match arg_type.as_deref() {
        Some("file") | Some("folder") => {
            if !cmd.arg.is_empty() {
                return Some(cmd.arg.clone());
            }
        }
        _ => {}
    }
    None
}

// =============================================================================
// ALIAS RESOLUTION
// =============================================================================

/// Resolve alias to target command, returning a reference.
///
/// This is a simpler version of `Command::resolve_alias` that:
/// - Returns a reference instead of cloning
/// - Doesn't do cycle detection (suitable for display purposes)
/// - Handles patch!command format
pub fn resolve_alias_to_target<'a>(cmd: &'a Command, all_commands: &'a [Command]) -> &'a Command {
    if cmd.action == "alias" {
        let target_lower = cmd.arg.to_lowercase();
        if let Some(target_cmd) = all_commands.iter().find(|c|
            c.command.to_lowercase() == target_lower ||
            // Handle patch!command format
            (c.command.contains('!') && c.command.split('!').nth(1).map(|s| s.trim().to_lowercase()) == Some(target_lower.clone()))
        ) {
            return target_cmd;
        }
    }
    cmd
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_commands() -> Vec<Command> {
        // Command::new(patch, command, action, arg, flags)
        vec![
            Command::new("".to_string(), "test".to_string(), "url".to_string(), "https://example.com".to_string(), "".to_string()),
            Command::new("".to_string(), "project".to_string(), "folder".to_string(), "/path/to/project".to_string(), "A".to_string()),
            Command::new("".to_string(), "proj".to_string(), "alias".to_string(), "project".to_string(), "".to_string()),
            Command::new("MyPatch".to_string(), "patched".to_string(), "url".to_string(), "https://example.com".to_string(), "".to_string()),
        ]
    }

    #[test]
    fn test_find_command() {
        let commands = make_test_commands();

        assert!(find_command("test", &commands).is_some());
        assert!(find_command("TEST", &commands).is_some()); // Case insensitive
        assert!(find_command("nonexistent", &commands).is_none());
    }

    #[test]
    fn test_find_command_exact() {
        let commands = make_test_commands();

        assert!(find_command_exact("test", &commands).is_some());
        assert!(find_command_exact("TEST", &commands).is_none()); // Case sensitive
    }

    #[test]
    fn test_find_commands_by_prefix() {
        let commands = make_test_commands();

        let results = find_commands_by_prefix("proj", &commands);
        assert_eq!(results.len(), 2); // "project" and "proj"
    }

    #[test]
    fn test_find_commands_in_patch() {
        let commands = make_test_commands();

        let results = find_commands_in_patch("MyPatch", &commands);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].command, "patched");
    }
}
