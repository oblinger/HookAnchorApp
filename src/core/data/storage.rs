//! Low-level storage operations for commands
//!
//! This module handles reading and writing commands to disk and cache.
//! All functions in this module are private to the data layer.

use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use crate::core::Command;
use crate::core::commands::COMMANDS_FORMAT_VERSION;
use crate::prelude::*;

/// Returns the HookAnchor config directory path (~/.config/hookanchor/)
///
/// This is the ONLY path function that should be used outside the data layer.
/// Use this for non-data files (logs, sockets, scripts, etc.).
///
/// DO NOT use this to access commands.txt or commands_cache.json - those paths
/// are intentionally hidden and must only be accessed through data layer functions.
pub fn get_config_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config").join("hookanchor")
}

/// Returns the path to the commands.txt file
/// This function is private to the data layer to enforce encapsulation
pub(in crate::core) fn get_commands_file_path() -> PathBuf {
    get_config_dir().join("commands.txt")
}

/// Returns the path to the backups folder
fn get_backups_folder_path() -> PathBuf {
    get_config_dir().join("backups")
}

/// Creates a timestamped backup of a file in the backups folder
///
/// # Arguments
/// * `source_path` - Path to the file to backup
/// * `prefix` - Prefix for the backup filename (e.g., "commands" or "cache")
/// * `extension` - File extension (e.g., "txt" or "json")
fn backup_file(source_path: &Path, prefix: &str, extension: &str) -> Result<(), Box<dyn std::error::Error>> {
    let backups_path = get_backups_folder_path();

    // Create backups directory if it doesn't exist
    fs::create_dir_all(&backups_path)?;

    // Only backup if the source file exists
    if source_path.exists() {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("{}_{}.{}", prefix, timestamp, extension);
        let backup_path = backups_path.join(backup_name);

        fs::copy(source_path, backup_path)?;
    }

    Ok(())
}

/// Loads commands from the commands.txt file without any processing
/// This is the raw loading function used by sys_data::load_data()
pub(crate) fn load_commands_raw() -> Vec<Command> {
    let path = get_commands_file_path();

    if !path.exists() {
        log_error(&format!("commands.txt not found at {:?}", path));
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
                        // NOTE: No tilde expansion needed here - commands.txt should already have
                        // fully expanded paths. Tildes are expanded:
                        // 1. When config is loaded (file_roots, etc.)
                        // 2. When user edits commands (in prepare_save_command)
                        // 3. When scanner creates commands (uses already-expanded config paths)
                        commands.push(command);
                    },
                    Err(e) => log_error(&format!("Failed to parse line {} in commands.txt: {} - Line: '{}'",
                        line_num + 1, e, line)),
                }
            }
            commands
        }
        Err(e) => {
            log_error(&format!("Error reading commands.txt: {}", e));
            vec![]
        }
    }
}

/// Saves commands to file with safety checks
/// NOTE: Deduplication should happen in flush() before calling this
pub(in crate::core) fn save_commands_to_file(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_commands_file_path();

    // Create backup before saving
    backup_file(&path, "commands", "txt")?;

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
            let actions_that_need_patches = ["markdown", "doc", "cmd"];
            if actions_that_need_patches.contains(&cmd.action.as_str()) {
                detailed_log("EMPTY_PATCH_BUG", &format!("Command with EMPTY patch during save: '{}' (action: {}, arg: {})",
                    cmd.command, cmd.action, cmd.arg));
            }
        }
    }

    // SAFETY CHECKS: Prevent saving corrupted data
    if updated_commands.len() > 10000 {
        let error_msg = format!("CORRUPTION DETECTED: Attempting to save {} commands (> 10000 limit). This indicates command inflation. Save operation CANCELLED.", updated_commands.len());
        log_error(&error_msg);
        detailed_log("CORRUPTION", &error_msg);
        return Err("Command count exceeds safety limit".into());
    }

    if empty_patch_count > 200 {
        let error_msg = format!("CORRUPTION DETECTED: Attempting to save {} commands with empty patches (> 200 limit). This indicates patch stripping. Save operation CANCELLED.", empty_patch_count);
        log_error(&error_msg);
        detailed_log("CORRUPTION", &error_msg);
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
        log_error(&error_msg);
        return Err(error_msg.into());
    }

    detailed_log("AUTO_SAVE", &format!("Saved {} commands to {}", commands.len(), path.display()));
    Ok(())
}

/// Get the path to the commands cache file
/// This function is private to the data layer to enforce encapsulation
pub(super) fn get_commands_cache_path() -> PathBuf {
    get_config_dir().join("commands_cache.json")
}

/// Save commands to cache (JSON format with metadata)
pub(super) fn save_commands_to_cache(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_commands_cache_path();

    // Create backup before saving
    backup_file(&path, "cache", "json")?;

    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Serialize to pretty JSON for readability
    let json = serde_json::to_string_pretty(commands)?;

    fs::write(&path, json)?;

    detailed_log("CACHE_SAVE", &format!("Saved {} commands to cache", commands.len()));
    Ok(())
}

/// Load commands from cache (JSON format with metadata)
/// Returns None if cache doesn't exist or is invalid
pub(super) fn load_commands_from_cache() -> Option<Vec<Command>> {
    let path = get_commands_cache_path();

    if !path.exists() {
        detailed_log("CACHE_LOAD", "Cache file does not exist");
        return None;
    }

    match fs::read_to_string(&path) {
        Ok(contents) => {
            match serde_json::from_str::<Vec<Command>>(&contents) {
                Ok(mut commands) => {
                    // Sanitize flags for all commands loaded from cache
                    // Flags should only contain alphabetic characters (no commas, spaces, etc.)
                    for cmd in &mut commands {
                        cmd.flags = cmd.flags.chars()
                            .filter(|c| c.is_alphabetic())
                            .collect();
                    }
                    detailed_log("CACHE_LOAD", &format!("Loaded {} commands from cache", commands.len()));
                    Some(commands)
                }
                Err(e) => {
                    log_error(&format!("Failed to parse commands cache: {}", e));
                    None
                }
            }
        }
        Err(e) => {
            log_error(&format!("Failed to read commands cache: {}", e));
            None
        }
    }
}

/// Delete the command cache file
///
/// This function removes the JSON cache file at ~/.config/hookanchor/commands_cache.json.
/// The next rescan will rebuild the cache from scratch by scanning all file_roots.
///
/// This is pub(super) so only sys_data can call it as part of coordinated cleanup operations.
///
/// # Returns
/// * `Ok(true)` - Cache file existed and was deleted
/// * `Ok(false)` - Cache file did not exist (nothing to delete)
/// * `Err(String)` - Failed to delete existing cache file
pub(super) fn delete_cache() -> Result<bool, String> {
    let cache_path = get_commands_cache_path();

    if cache_path.exists() {
        std::fs::remove_file(&cache_path)
            .map_err(|e| format!("Failed to delete command cache: {}", e))?;
        log(&format!("Deleted command cache: {}", cache_path.display()));
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Generates a unique key for deduplication
/// Commands with the same key are considered duplicates
pub fn command_dedup_key(cmd: &Command) -> String {
    format!("{}:{}:{}", cmd.command, cmd.action, cmd.arg)
}

/// Builds a HashMap for efficient command lookup using dedup keys
/// This ensures consistent comparison logic across the codebase
///
/// Returns a HashMap where:
/// - Key: deduplication key (command:action:arg)
/// - Value: reference to the Command
pub fn build_command_map<'a>(commands: &'a [Command]) -> std::collections::HashMap<String, &'a Command> {
    commands.iter()
        .map(|cmd| (command_dedup_key(cmd), cmd))
        .collect()
}

/// Deduplicates commands by keeping the best version of each command
/// Commands are considered duplicates if they have the same name, action, AND arg
/// This allows multiple commands with the same name pointing to different files
/// Priority: commands with patches > commands without patches
/// Then by flags: commands with flags > commands without flags
/// SPECIAL CASE: Virtual orphan anchors are removed if a real anchor with same name exists
pub(super) fn deduplicate_commands(commands: Vec<Command>) -> Vec<Command> {
    use std::collections::HashMap;

    let original_count = commands.len();
    let mut best_commands: HashMap<String, Command> = HashMap::new();
    let mut commands_by_name: HashMap<String, Vec<Command>> = HashMap::new();

    // First pass: Standard deduplication by (name, action, arg)
    for command in commands {
        let key = command_dedup_key(&command);
        let name_key = command.command.to_lowercase();

        // Track all commands with this name for cross-action deduplication
        commands_by_name.entry(name_key).or_insert_with(Vec::new).push(command.clone());

        if let Some(existing) = best_commands.get(&key) {
            // Decide which command to keep
            if should_replace_command(existing, &command) {
                detailed_log("DEDUPE", &format!(
                    "Replacing '{}' (action:'{}' patch:'{}' flags:'{}' arg:'{}') with better version (patch:'{}' flags:'{}' arg:'{}')",
                    existing.command, existing.action, existing.patch, existing.flags, existing.arg,
                    command.patch, command.flags, command.arg
                ));
                best_commands.insert(key, command);
            } else {
                detailed_log("DEDUPE", &format!(
                    "Keeping existing '{}' (action:'{}' patch:'{}' flags:'{}' arg:'{}') over duplicate (patch:'{}' flags:'{}' arg:'{}')",
                    existing.command, existing.action, existing.patch, existing.flags, existing.arg,
                    command.patch, command.flags, command.arg
                ));
            }
        } else {
            best_commands.insert(key, command);
        }
    }

    // Second pass: Remove virtual orphan anchors when real anchors exist
    let mut to_remove = Vec::new();
    for (name, cmds) in commands_by_name.iter() {
        // Check if we have both a virtual orphan anchor and a real anchor
        // A "real anchor" is a command with 'A' flag that's not in orphans patch
        // and has some action (virtual anchors have empty action)
        let has_real_anchor = cmds.iter().any(|cmd|
            cmd.is_anchor() &&
            cmd.patch != "orphans" &&
            !cmd.action.is_empty() // Must have some action to be a real anchor
        );

        if has_real_anchor {
            // Find virtual orphan anchors with this name
            for cmd in cmds.iter() {
                if cmd.is_anchor() &&
                   cmd.action.is_empty() && // Virtual anchors have empty action
                   cmd.patch == "orphans" &&
                   !cmd.flags.contains('U') { // Not user-edited
                    let key = command_dedup_key(cmd);
                    to_remove.push(key.clone());
                    detailed_log("DEDUPE", &format!(
                        "Removing virtual orphan anchor '{}' (patch:'{}') because real anchor exists",
                        cmd.command, cmd.patch
                    ));
                }
            }
        }
    }

    // Remove virtual orphan anchors
    for key in to_remove {
        best_commands.remove(&key);
    }

    let result: Vec<Command> = best_commands.into_values().collect();
    let deduped_count = original_count - result.len();

    if deduped_count > 0 {
        detailed_log("DEDUPE", &format!("Removed {} duplicate commands (same name + action + arg + virtual orphans)", deduped_count));
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
