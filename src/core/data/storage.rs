//! Low-level storage operations for commands
//!
//! This module handles reading and writing commands to disk and cache.
//! All functions in this module are private to the data layer.

use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use crate::core::Command;
use crate::core::commands::COMMANDS_FORMAT_VERSION;

/// Returns the path to the commands.txt file
pub(super) fn get_commands_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/hookanchor/commands.txt")
}

/// Returns the path to the backups folder
fn get_backups_folder_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/hookanchor/backups")
}

/// Creates a backup of the commands file before saving
fn backup_commands_file() -> Result<(), Box<dyn std::error::Error>> {
    let commands_path = get_commands_file_path();
    let backups_path = get_backups_folder_path();

    // Create backups directory if it doesn't exist
    fs::create_dir_all(&backups_path)?;

    // Only backup if the commands file exists
    if commands_path.exists() {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("commands_{}.txt", timestamp);
        let backup_path = backups_path.join(backup_name);

        fs::copy(&commands_path, backup_path)?;
    }

    Ok(())
}

/// Loads commands from the commands.txt file without any processing
/// This is the raw loading function used by sys_data::load_data()
pub(super) fn load_commands_raw() -> Vec<Command> {
    let path = get_commands_file_path();

    if !path.exists() {
        crate::utils::log_error(&format!("commands.txt not found at {:?}", path));
        return vec![];
    }

    match fs::read_to_string(&path) {
        Ok(contents) => {
            let mut commands = Vec::new();
            for (line_num, line) in contents.lines().enumerate() {
                if line.trim().is_empty() {
                    continue;
                }

                match crate::core::commands::parse_command_line(line) {
                    Ok(command) => {
                        // Debug: Log the first few commands to see if patches are being preserved
                        if line_num < 5 {
                            crate::utils::detailed_log("PARSE_DEBUG", &format!("Parsed line {}: patch='{}', command='{}'",
                                line_num + 1, command.patch, command.command));
                        }
                        // Also log the Patents command specifically
                        if command.command == "Patents" {
                            crate::utils::detailed_log("PARSE_DEBUG", &format!("Found Patents command: patch='{}', command='{}', action='{}'",
                                command.patch, command.command, command.action));
                        }
                        commands.push(command);
                    },
                    Err(e) => crate::utils::log_error(&format!("Failed to parse line {} in commands.txt: {} - Line: '{}'",
                        line_num + 1, e, line)),
                }
            }
            commands
        }
        Err(e) => {
            crate::utils::log_error(&format!("Error reading commands.txt: {}", e));
            vec![]
        }
    }
}

/// Saves commands to file with safety checks
/// NOTE: Deduplication should happen in flush() before calling this
pub(super) fn save_commands_to_file(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    // Create backup before saving
    backup_commands_file()?;

    let path = get_commands_file_path();

    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Update cmd commands without patches to have "Cmd" patch
    let mut updated_commands = commands.to_vec();
    for cmd in &mut updated_commands {
        if cmd.action == "cmd" && cmd.patch.is_empty() {
            cmd.patch = "Cmd".to_string();
        }
    }

    // Sort commands by patch string first, then by command name before writing to file
    updated_commands.sort_by(|a, b| {
        match a.patch.cmp(&b.patch) {
            std::cmp::Ordering::Equal => a.command.cmp(&b.command),
            other => other
        }
    });

    // Debug: Check what patches look like right before serialization
    let mut empty_patch_count = 0;
    for cmd in &updated_commands {
        if cmd.patch.is_empty() {
            empty_patch_count += 1;

            // Only log as potential bug for actions that typically need patches
            let actions_that_need_patches = ["anchor", "markdown", "doc", "cmd"];
            if actions_that_need_patches.contains(&cmd.action.as_str()) {
                crate::utils::detailed_log("EMPTY_PATCH_BUG", &format!("Command with EMPTY patch during save: '{}' (action: {}, arg: {})",
                    cmd.command, cmd.action, cmd.arg));
            }
        }
    }

    // SAFETY CHECKS: Prevent saving corrupted data
    if updated_commands.len() > 10000 {
        let error_msg = format!("CORRUPTION DETECTED: Attempting to save {} commands (> 10000 limit). This indicates command inflation. Save operation CANCELLED.", updated_commands.len());
        crate::utils::log_error(&error_msg);
        crate::utils::detailed_log("CORRUPTION", &error_msg);
        return Err("Command count exceeds safety limit".into());
    }

    if empty_patch_count > 200 {
        let error_msg = format!("CORRUPTION DETECTED: Attempting to save {} commands with empty patches (> 200 limit). This indicates patch stripping. Save operation CANCELLED.", empty_patch_count);
        crate::utils::log_error(&error_msg);
        crate::utils::detailed_log("CORRUPTION", &error_msg);
        return Err("Empty patch count exceeds safety limit".into());
    }

    // Build file contents with version header
    let mut contents = String::new();

    // Add version header as first line
    contents.push_str(&format!("// HookAnchor Commands Format - version:={}\n", COMMANDS_FORMAT_VERSION));

    // Convert all commands to new format and join with newlines
    let commands_text = updated_commands.iter()
        .map(|cmd| cmd.to_new_format())
        .collect::<Vec<_>>()
        .join("\n");

    contents.push_str(&commands_text);

    // Write with better error handling that includes the file path
    if let Err(e) = fs::write(&path, &contents) {
        let error_msg = format!("Cannot write to file '{}': {}", path.display(), e);
        crate::utils::log_error(&error_msg);
        return Err(error_msg.into());
    }

    crate::utils::detailed_log("AUTO_SAVE", &format!("Saved {} commands to {}", commands.len(), path.display()));
    Ok(())
}

/// Get the path to the commands cache file
fn get_commands_cache_path() -> PathBuf {
    let config_dir = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config")
        .join("hookanchor");

    config_dir.join("commands_cache.json")
}

/// Save commands to cache (JSON format with metadata)
pub(super) fn save_commands_to_cache(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_commands_cache_path();

    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Serialize to pretty JSON for readability
    let json = serde_json::to_string_pretty(commands)?;

    fs::write(&path, json)?;

    crate::utils::detailed_log("CACHE_SAVE", &format!("Saved {} commands to cache", commands.len()));
    Ok(())
}

/// Load commands from cache (JSON format with metadata)
/// Returns None if cache doesn't exist or is invalid
pub(super) fn load_commands_from_cache() -> Option<Vec<Command>> {
    let path = get_commands_cache_path();

    if !path.exists() {
        crate::utils::detailed_log("CACHE_LOAD", "Cache file does not exist");
        return None;
    }

    match fs::read_to_string(&path) {
        Ok(contents) => {
            match serde_json::from_str::<Vec<Command>>(&contents) {
                Ok(commands) => {
                    crate::utils::detailed_log("CACHE_LOAD", &format!("Loaded {} commands from cache", commands.len()));
                    Some(commands)
                }
                Err(e) => {
                    crate::utils::log_error(&format!("Failed to parse commands cache: {}", e));
                    None
                }
            }
        }
        Err(e) => {
            crate::utils::log_error(&format!("Failed to read commands cache: {}", e));
            None
        }
    }
}

/// Deduplicates commands by keeping the best version of each command
/// Commands are considered duplicates if they have the same name
/// Priority: commands with patches > commands without patches
/// Then by flags: commands with flags > commands without flags
pub(super) fn deduplicate_commands(commands: Vec<Command>) -> Vec<Command> {
    use std::collections::HashMap;

    let original_count = commands.len();
    let mut best_commands: HashMap<String, Command> = HashMap::new();

    for command in commands {
        // Use command name as the unique key - only one command per name allowed
        let key = command.command.clone();

        if let Some(existing) = best_commands.get(&key) {
            // Decide which command to keep
            if should_replace_command(existing, &command) {
                best_commands.insert(key, command);
            }
        } else {
            best_commands.insert(key, command);
        }
    }

    let result: Vec<Command> = best_commands.into_values().collect();
    let deduped_count = original_count - result.len();

    if deduped_count > 0 {
        crate::utils::detailed_log("DEDUPE", &format!("Removed {} duplicate commands", deduped_count));
    }

    result
}

/// Determines if new_command should replace current_command
fn should_replace_command(current: &Command, new: &Command) -> bool {
    // Priority 0: ALWAYS prefer anchor commands - they define the patch hierarchy
    // Anchors must never be removed during deduplication or the hierarchy breaks
    if new.is_anchor() && !current.is_anchor() {
        return true;
    }
    if current.is_anchor() && !new.is_anchor() {
        return false;
    }

    // Priority 1: Prefer commands with patches
    if !new.patch.is_empty() && current.patch.is_empty() {
        return true;
    }
    if new.patch.is_empty() && !current.patch.is_empty() {
        return false;
    }

    // Priority 2: Prefer commands with flags
    if !new.flags.is_empty() && current.flags.is_empty() {
        return true;
    }
    if new.flags.is_empty() && !current.flags.is_empty() {
        return false;
    }

    // Default: keep current
    false
}
