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
// RENAME OPERATIONS
// =============================================================================

/// Parameters for a full rename operation (with file/folder side effects)
pub struct FullRenameParams {
    pub old_name: String,
    pub new_name: String,
    pub current_arg: String,
    pub action: String,
    pub original_command_to_delete: Option<String>,
    pub new_command_action: String,
    pub new_command_patch: String,
    pub new_command_flags: String,
}

/// Execute a full rename operation with file/folder side effects.
///
/// This performs the rename via `rename_associated_data`, optionally renames
/// the containing folder if it's an anchor file, and saves the updated command.
///
/// # Returns
/// Ok(new_command_name) on success, for UI feedback
pub fn execute_full_rename(params: FullRenameParams) -> Result<String, Box<dyn std::error::Error>> {
    use crate::prelude::*;
    use crate::core::rename_associated_data;

    let config = crate::core::data::get_config();
    let mut commands = crate::core::data::get_commands();
    let (sys_data, _) = crate::core::get_sys_data();
    let mut patches = sys_data.patches;

    log(&format!("RENAME: Executing full rename: '{}' -> '{}'", params.old_name, params.new_name));

    // Execute the rename with dry_run = false
    let (mut updated_arg, _actions) = rename_associated_data(
        &params.old_name,
        &params.new_name,
        &params.current_arg,
        &params.action,
        &mut commands,
        &mut patches,
        &config,
        false, // dry_run = false
    )?;
    log(&format!("RENAME: rename_associated_data completed, updated_arg: {}", updated_arg));

    // Check if this is an anchor file rename where filename matches folder name
    if params.new_command_flags.contains('A') {
        use std::path::Path;
        let arg_path = Path::new(&params.current_arg);

        // Check if the file stem matches the old command name
        if let Some(file_stem) = arg_path.file_stem() {
            if let Some(file_stem_str) = file_stem.to_str() {
                if file_stem_str == params.old_name {
                    // Check if folder name also matches
                    if let Some(parent) = arg_path.parent() {
                        if let Some(folder_name) = parent.file_name() {
                            if let Some(folder_name_str) = folder_name.to_str() {
                                if folder_name_str == params.old_name {
                                    // Rename the folder too
                                    use crate::core::command_ops::rename_folder;
                                    log(&format!("RENAME_FOLDER: Executing folder rename: '{}' -> '{}'", folder_name_str, params.new_name));
                                    match rename_folder(
                                        parent.to_str().unwrap_or(""),
                                        &params.new_name,
                                        &mut commands,
                                        false, // dry_run = false
                                    ) {
                                        Ok(_) => {
                                            log("RENAME_FOLDER: Folder rename completed successfully");
                                            // Update the arg path to reflect new folder name
                                            let new_folder_path = parent.parent()
                                                .map(|p| p.join(&params.new_name))
                                                .unwrap_or_else(|| std::path::PathBuf::from(&params.new_name));
                                            let new_file_name = format!("{}.md", params.new_name);
                                            let new_full_path = new_folder_path.join(&new_file_name);
                                            updated_arg = new_full_path.to_string_lossy().to_string();
                                            log(&format!("RENAME_FOLDER: Updated arg to: {}", updated_arg));
                                        }
                                        Err(e) => {
                                            return Err(format!("Failed to rename folder: {}", e).into());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Delete original command if needed
    if let Some(ref cmd_name) = params.original_command_to_delete {
        if !cmd_name.is_empty() {
            log(&format!("RENAME: Deleting original command '{}'", cmd_name));
            commands.retain(|c| &c.command != cmd_name);
        }
    }

    // Create and add the new command
    let new_command = Command {
        command: params.new_name.clone(),
        action: params.new_command_action,
        arg: updated_arg,
        patch: params.new_command_patch,
        flags: params.new_command_flags,
        other_params: None,
        last_update: 0,
        file_size: None,
    };
    log(&format!("RENAME: Adding new command '{}'", new_command.command));
    commands.push(new_command);

    // Save all commands
    crate::core::data::set_commands(commands)?;
    log(&format!("RENAME: Successfully renamed '{}' to '{}'", params.old_name, params.new_name));

    Ok(params.new_name)
}

/// Execute a simple command rename (command name only, no file/folder changes).
///
/// # Returns
/// Ok(new_command_name) on success, for UI feedback
pub fn execute_command_only_rename(
    old_name: &str,
    new_name: &str,
    original_command_to_delete: Option<&str>,
    new_command_action: &str,
    new_command_arg: &str,
    new_command_patch: &str,
    new_command_flags: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    use crate::prelude::*;

    log(&format!("CMD_ONLY_RENAME: Renaming '{}' -> '{}' (no side effects)", old_name, new_name));

    let mut commands = crate::core::data::get_commands();

    // Delete original command if needed
    if let Some(cmd_name) = original_command_to_delete {
        if !cmd_name.is_empty() {
            log(&format!("CMD_ONLY_RENAME: Deleting original command '{}'", cmd_name));
            commands.retain(|c| c.command != cmd_name);
        }
    }

    // Create the new command with original arg (no rename modifications)
    let new_command = Command {
        command: new_name.to_string(),
        action: new_command_action.to_string(),
        arg: new_command_arg.to_string(),
        patch: new_command_patch.to_string(),
        flags: new_command_flags.to_string(),
        other_params: None,
        last_update: 0,
        file_size: None,
    };
    log(&format!("CMD_ONLY_RENAME: Adding new command '{}'", new_command.command));
    commands.push(new_command);

    // Save commands
    crate::core::data::set_commands(commands)?;
    log(&format!("CMD_ONLY_RENAME: Successfully renamed '{}' to '{}'", old_name, new_name));

    Ok(new_name.to_string())
}

// =============================================================================
// COMMAND PERSISTENCE
// =============================================================================

/// Save a command atomically - handles both new commands and edits.
///
/// If `old_command_name` is Some, this is an edit/rename - the old command
/// is deleted first. This ensures patch inference only runs once after both
/// delete and add are complete.
///
/// # Arguments
/// * `new_command` - The command to save
/// * `old_command_name` - If editing/renaming, the original command name to delete
///
/// # Returns
/// Ok(()) on success, or an error if saving fails
pub fn save_command_atomic(new_command: Command, old_command_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    use crate::prelude::*;

    // Get current commands from singleton
    let mut commands = crate::core::data::get_commands();

    // Delete original command if this is an edit/rename
    if let Some(old_name) = old_command_name {
        if !old_name.is_empty() {
            log(&format!("SAVE: Deleting original command '{}' for atomic save", old_name));
            commands.retain(|c| c.command != old_name);
        }
    }

    // Add the new command
    log(&format!("SAVE: Adding command '{}' (action: {}, patch: {})",
        new_command.command, new_command.action, new_command.patch));
    commands.push(new_command);

    // Save all commands to disk - patch inference runs once here
    crate::core::data::set_commands(commands)?;
    log("SAVE: Saved all commands to disk");

    Ok(())
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
