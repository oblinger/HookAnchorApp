//! Command history tracking using SQLite
//!
//! This module provides functionality to track all changes to commands over time.
//! It maintains a SQLite database with command history and provides query capabilities.

use rusqlite::{Connection, Result as SqlResult, params};
use std::path::PathBuf;
use crate::core::Command;

/// Get the path to the history database
fn get_history_db_path() -> PathBuf {
    let config_dir = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config")
        .join("hookanchor");

    // Ensure directory exists
    std::fs::create_dir_all(&config_dir).ok();

    config_dir.join("history.db")
}

/// Initialize the history database and create tables if they don't exist
pub fn initialize_history_db() -> SqlResult<Connection> {
    let db_path = get_history_db_path();
    let conn = Connection::open(db_path)?;

    // Create the command_history table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS command_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,
            change_type TEXT NOT NULL,

            patch TEXT NOT NULL,
            command TEXT NOT NULL,
            action TEXT NOT NULL,

            arg TEXT,
            flags TEXT,
            file_path TEXT,

            changed_fields TEXT,
            old_values TEXT,
            new_values TEXT,
            edit_size INTEGER
        )",
        [],
    )?;

    // Add edit_size column if it doesn't exist (migration for existing databases)
    conn.execute(
        "ALTER TABLE command_history ADD COLUMN edit_size INTEGER",
        [],
    ).ok(); // Ignore error if column already exists

    // Create indexes for fast querying
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_command_history_timestamp
         ON command_history(timestamp)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_command_history_file_path
         ON command_history(file_path)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_command_history_patch
         ON command_history(patch)",
        [],
    )?;

    Ok(conn)
}

/// Record a command creation in the history database
pub fn record_command_created(conn: &Connection, cmd: &Command, timestamp: i64) -> SqlResult<()> {
    let file_path_obj = cmd.get_absolute_file_path(&crate::core::sys_data::get_config());
    let file_path = file_path_obj.as_ref().map(|p| p.to_string_lossy().to_string());

    // For file-based commands, try to get the file's creation time (birth time)
    // For non-file commands, use the provided timestamp
    let actual_timestamp = if cmd.is_path_based() {
        if let Some(ref path) = file_path_obj {
            // Try to get file creation time
            if let Ok(metadata) = std::fs::metadata(path) {
                if let Ok(created) = metadata.created() {
                    if let Ok(duration) = created.duration_since(std::time::UNIX_EPOCH) {
                        duration.as_secs() as i64
                    } else {
                        timestamp // Fallback to provided timestamp
                    }
                } else {
                    timestamp // Fallback to provided timestamp
                }
            } else {
                timestamp // File doesn't exist, use provided timestamp
            }
        } else {
            timestamp // No file path, use provided timestamp
        }
    } else {
        timestamp // Non-file command, use provided timestamp
    };

    // Check if a "created" entry already exists for this command
    // A command can only be created once, so we check by command name regardless of timestamp
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM command_history WHERE change_type = 'created' AND command = ?1)",
        params![&cmd.command],
        |row| row.get(0),
    )?;

    if exists {
        // Already have a creation record for this command - skip duplicate
        crate::utils::log(&format!(
            "HISTORY_CREATE: Skipping duplicate creation record for '{}'",
            cmd.command
        ));
        return Ok(());
    }

    // Calculate edit_size: for new files, it's just the current file size
    let edit_size = cmd.file_size.map(|size| size as i64);

    conn.execute(
        "INSERT INTO command_history
         (timestamp, change_type, patch, command, action, arg, flags, file_path, changed_fields, old_values, new_values, edit_size)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, NULL, NULL, ?9)",
        params![
            actual_timestamp,
            "created",
            &cmd.patch,
            &cmd.command,
            &cmd.action,
            &cmd.arg,
            &cmd.flags,
            file_path,
            edit_size,
        ],
    )?;

    // Log the history entry creation
    use chrono::{Local, TimeZone};
    let date_str = if let Some(dt) = Local.timestamp_opt(actual_timestamp, 0).single() {
        dt.format("%Y-%m-%d").to_string()
    } else {
        "unknown".to_string()
    };

    let size_str = if let Some(size) = edit_size {
        format!(" size={}b", size)
    } else {
        String::new()
    };

    crate::utils::log(&format!(
        "HISTORY_CREATE: [{}] action={} cmd='{}' patch='{}'{}",
        date_str, cmd.action, cmd.command, cmd.patch, size_str
    ));

    Ok(())
}

/// Record a command modification in the history database
pub fn record_command_modified(
    conn: &Connection,
    old_cmd: &Command,
    new_cmd: &Command,
    timestamp: i64,
) -> SqlResult<()> {
    let file_path_obj = new_cmd.get_absolute_file_path(&crate::core::sys_data::get_config());
    let file_path = file_path_obj.as_ref().map(|p| p.to_string_lossy().to_string());

    // For file-based commands, try to get the file's modification time
    // For non-file commands, use the provided timestamp
    let actual_timestamp = if new_cmd.is_path_based() {
        if let Some(ref path) = file_path_obj {
            // Try to get file modification time
            if let Ok(metadata) = std::fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                        duration.as_secs() as i64
                    } else {
                        timestamp // Fallback to provided timestamp
                    }
                } else {
                    timestamp // Fallback to provided timestamp
                }
            } else {
                timestamp // File doesn't exist, use provided timestamp
            }
        } else {
            timestamp // No file path, use provided timestamp
        }
    } else {
        timestamp // Non-file command, use provided timestamp
    };

    // Check if a "modified" entry already exists for this command at approximately this timestamp
    // Allow a 60-second window to account for slight timing variations
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM command_history
         WHERE change_type = 'modified'
         AND command = ?1
         AND ABS(timestamp - ?2) < 60)",
        params![&new_cmd.command, actual_timestamp],
        |row| row.get(0),
    )?;

    if exists {
        // Already have a modification record for this command at this timestamp - skip duplicate
        crate::utils::log(&format!(
            "HISTORY_MODIFIED: Skipping duplicate modification record for '{}'",
            new_cmd.command
        ));
        return Ok(());
    }

    // Calculate edit_size: new_size - old_size (can be negative if file shrunk)
    let edit_size = match (new_cmd.file_size, old_cmd.file_size) {
        (Some(new_size), Some(old_size)) => Some((new_size as i64) - (old_size as i64)),
        (Some(new_size), None) => Some(new_size as i64), // File was created
        (None, Some(old_size)) => Some(-(old_size as i64)), // File was deleted
        (None, None) => None, // Not a file-based command
    };

    // Determine what changed
    let mut changed_fields = Vec::new();
    let mut old_values = serde_json::Map::new();
    let mut new_values = serde_json::Map::new();

    if old_cmd.patch != new_cmd.patch {
        changed_fields.push("patch");
        old_values.insert("patch".to_string(), serde_json::Value::String(old_cmd.patch.clone()));
        new_values.insert("patch".to_string(), serde_json::Value::String(new_cmd.patch.clone()));
    }
    if old_cmd.arg != new_cmd.arg {
        changed_fields.push("arg");
        old_values.insert("arg".to_string(), serde_json::Value::String(old_cmd.arg.clone()));
        new_values.insert("arg".to_string(), serde_json::Value::String(new_cmd.arg.clone()));
    }
    if old_cmd.flags != new_cmd.flags {
        changed_fields.push("flags");
        old_values.insert("flags".to_string(), serde_json::Value::String(old_cmd.flags.clone()));
        new_values.insert("flags".to_string(), serde_json::Value::String(new_cmd.flags.clone()));
    }
    if old_cmd.action != new_cmd.action {
        changed_fields.push("action");
        old_values.insert("action".to_string(), serde_json::Value::String(old_cmd.action.clone()));
        new_values.insert("action".to_string(), serde_json::Value::String(new_cmd.action.clone()));
    }
    if old_cmd.file_size != new_cmd.file_size {
        changed_fields.push("file_size");
        old_values.insert("file_size".to_string(),
            serde_json::Value::String(format!("{:?}", old_cmd.file_size)));
        new_values.insert("file_size".to_string(),
            serde_json::Value::String(format!("{:?}", new_cmd.file_size)));
    }

    let changed_fields_json = serde_json::to_string(&changed_fields).unwrap();
    let old_values_json = serde_json::to_string(&old_values).unwrap();
    let new_values_json = serde_json::to_string(&new_values).unwrap();

    conn.execute(
        "INSERT INTO command_history
         (timestamp, change_type, patch, command, action, arg, flags, file_path, changed_fields, old_values, new_values, edit_size)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            actual_timestamp,
            "modified",
            &new_cmd.patch,
            &new_cmd.command,
            &new_cmd.action,
            &new_cmd.arg,
            &new_cmd.flags,
            file_path,
            changed_fields_json,
            old_values_json,
            new_values_json,
            edit_size,
        ],
    )?;

    Ok(())
}

/// Record a command deletion in the history database
pub fn record_command_deleted(conn: &Connection, cmd: &Command, timestamp: i64) -> SqlResult<()> {
    let file_path = cmd.get_absolute_file_path(&crate::core::sys_data::get_config())
        .map(|p| p.to_string_lossy().to_string());

    conn.execute(
        "INSERT INTO command_history
         (timestamp, change_type, patch, command, action, arg, flags, file_path, changed_fields, old_values, new_values)
         VALUES (?1, ?2, ?3, ?4, ?5, NULL, NULL, ?6, NULL, NULL, NULL)",
        params![
            timestamp,
            "deleted",
            &cmd.patch,
            &cmd.command,
            &cmd.action,
            file_path,
        ],
    )?;

    Ok(())
}

/// Query history entries within a date range
pub fn query_history_by_date_range(
    conn: &Connection,
    start_timestamp: i64,
    end_timestamp: i64,
    limit: usize,
    offset: usize,
) -> SqlResult<Vec<HistoryEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, timestamp, change_type, patch, command, action, arg, flags, file_path, changed_fields, old_values, new_values, edit_size
         FROM command_history
         WHERE timestamp BETWEEN ?1 AND ?2
         ORDER BY timestamp DESC
         LIMIT ?3 OFFSET ?4"
    )?;

    let entries = stmt.query_map(params![start_timestamp, end_timestamp, limit, offset], |row| {
        Ok(HistoryEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            change_type: row.get(2)?,
            patch: row.get(3)?,
            command: row.get(4)?,
            action: row.get(5)?,
            arg: row.get(6)?,
            flags: row.get(7)?,
            file_path: row.get(8)?,
            changed_fields: row.get(9)?,
            old_values: row.get(10)?,
            new_values: row.get(11)?,
            edit_size: row.get(12)?,
        })
    })?;

    entries.collect()
}

/// Query history entries by file path prefix
pub fn query_history_by_path_prefix(
    conn: &Connection,
    path_prefix: &str,
    limit: usize,
    offset: usize,
) -> SqlResult<Vec<HistoryEntry>> {
    let pattern = format!("{}%", path_prefix);

    let mut stmt = conn.prepare(
        "SELECT id, timestamp, change_type, patch, command, action, arg, flags, file_path, changed_fields, old_values, new_values, edit_size
         FROM command_history
         WHERE file_path LIKE ?1
         ORDER BY timestamp DESC
         LIMIT ?2 OFFSET ?3"
    )?;

    let entries = stmt.query_map(params![pattern, limit, offset], |row| {
        Ok(HistoryEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            change_type: row.get(2)?,
            patch: row.get(3)?,
            command: row.get(4)?,
            action: row.get(5)?,
            arg: row.get(6)?,
            flags: row.get(7)?,
            file_path: row.get(8)?,
            changed_fields: row.get(9)?,
            old_values: row.get(10)?,
            new_values: row.get(11)?,
            edit_size: row.get(12)?,
        })
    })?;

    entries.collect()
}

/// Represents a single history entry from the database
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub id: i64,
    pub timestamp: i64,
    pub change_type: String,
    pub patch: String,
    pub command: String,
    pub action: String,
    pub arg: Option<String>,
    pub flags: Option<String>,
    pub file_path: Option<String>,
    pub changed_fields: Option<String>,
    pub old_values: Option<String>,
    pub new_values: Option<String>,
    pub edit_size: Option<i64>,
}

/// Compare two commands to determine if they're functionally different
/// Ignores last_update field since that's metadata, not content
fn commands_differ(a: &Command, b: &Command) -> bool {
    a.patch != b.patch
        || a.command != b.command
        || a.action != b.action
        || a.arg != b.arg
        || a.flags != b.flags
        || a.file_size != b.file_size
}

/// Update a command with history tracking
///
/// This is the ONLY way commands should be modified - enforces history tracking.
///
/// # Arguments
/// * `commands` - Mutable reference to the commands list
/// * `old_cmd` - Optional old version of the command (None for new commands)
/// * `new_cmd` - New version of the command
///
/// # Returns
/// Ok(()) if successful, Err if history recording fails
///
/// # Side Effects
/// - Updates the commands list
/// - Records change in history.db
/// - Sets last_update timestamp on new_cmd
pub fn update_command(
    commands: &mut Vec<Command>,
    old_cmd: Option<&Command>,
    mut new_cmd: Command,
    flush: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize history database
    let conn = initialize_history_db()?;

    // Get current timestamp (fallback for non-file commands)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    // Update file_size and last_update timestamp for file-based commands
    if new_cmd.is_path_based() {
        if let Some(file_path) = new_cmd.get_absolute_file_path(&crate::core::sys_data::get_config()) {
            if let Ok(metadata) = std::fs::metadata(&file_path) {
                new_cmd.file_size = Some(metadata.len());

                // Set last_update to file's modification time
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                        new_cmd.last_update = duration.as_secs() as i64;
                    } else {
                        new_cmd.last_update = now;
                    }
                } else {
                    new_cmd.last_update = now;
                }
            } else {
                // File doesn't exist - use current time
                new_cmd.last_update = now;
            }
        } else {
            new_cmd.last_update = now;
        }
    } else {
        // Non-file command - use current time
        new_cmd.last_update = now;
    }

    match old_cmd {
        Some(old) => {
            // Check if command actually changed
            if !commands_differ(old, &new_cmd) {
                // No change, nothing to do
                return Ok(());
            }

            // Record modification
            record_command_modified(&conn, old, &new_cmd, now)?;

            // Update in commands list
            if let Some(pos) = commands.iter().position(|c|
                c.patch == old.patch && c.command == old.command && c.action == old.action
            ) {
                commands[pos] = new_cmd;
            }
        }
        None => {
            // New command - record creation
            record_command_created(&conn, &new_cmd, now)?;

            // Add to commands list
            commands.push(new_cmd);
        }
    }

    // If flush requested, save to disk immediately
    if flush {
        crate::core::commands::save_commands_to_file(commands)?;
        crate::core::commands::save_commands_to_cache(commands)?;
    }

    Ok(())
}

/// Flush commands to disk (write to commands.txt and cache)
/// Call this after a batch of update_command calls with flush=false
pub fn flush_commands(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    crate::core::commands::save_commands_to_file(commands)?;
    crate::core::commands::save_commands_to_cache(commands)?;
    Ok(())
}

/// Delete a command with history tracking
///
/// # Arguments
/// * `commands` - Mutable reference to the commands list
/// * `cmd` - Command to delete
///
/// # Returns
/// Ok(()) if successful, Err if history recording fails
pub fn delete_command(
    commands: &mut Vec<Command>,
    cmd: &Command,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize history database
    let conn = initialize_history_db()?;

    // Get current timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    // Record deletion
    record_command_deleted(&conn, cmd, now)?;

    // Remove from commands list
    commands.retain(|c|
        !(c.patch == cmd.patch && c.command == cmd.command && c.action == cmd.action)
    );

    Ok(())
}

/// Rescan and update commands with history tracking
///
/// This function implements the complete rescan workflow:
/// 1. Load cached commands (last known state)
/// 2. Load commands.txt (check for manual edits)
/// 3. Scan filesystem (check for file changes)
/// 4. Call update_command() for any differences
/// 5. Flush to both commands.txt and commands_cache.json
///
/// # Returns
/// Updated commands list with all changes tracked in history
pub fn rescan_with_history() -> Result<Vec<Command>, Box<dyn std::error::Error>> {
    crate::utils::log("HISTORY_RESCAN: Starting rescan with history tracking...");

    // Step 1: Load cached commands (last known state)
    let cached_commands = crate::core::commands::load_commands_from_cache()
        .unwrap_or_else(Vec::new);

    crate::utils::log(&format!("HISTORY_RESCAN: Loaded {} commands from cache", cached_commands.len()));

    // Step 2: Load commands.txt (detect manual edits)
    let txt_commands = crate::core::commands::load_commands_raw();
    crate::utils::log(&format!("HISTORY_RESCAN: Loaded {} commands from commands.txt", txt_commands.len()));

    // Start with empty commands list that we'll build up
    let mut updated_commands = Vec::new();

    // Step 3: Compare commands.txt with cache to detect manual edits
    for txt_cmd in &txt_commands {
        // Find corresponding cached command
        let cached_cmd = cached_commands.iter().find(|c|
            c.patch == txt_cmd.patch && c.command == txt_cmd.command && c.action == txt_cmd.action
        );

        match cached_cmd {
            Some(old) => {
                // Command exists in cache - check if it changed
                let mut new_cmd = txt_cmd.clone();

                // If it's a file-based command, we'll update file_size later in step 4
                // For now, just detect if the command definition itself changed
                if commands_differ(old, &new_cmd) {
                    crate::utils::log(&format!("HISTORY_RESCAN: Command '{}' modified in commands.txt", txt_cmd.command));
                    update_command(&mut updated_commands, Some(old), new_cmd, false)?; // Don't flush yet
                } else {
                    // No change - just add the old command
                    updated_commands.push(old.clone());
                }
            }
            None => {
                // New command in commands.txt
                crate::utils::log(&format!("HISTORY_RESCAN: New command '{}' found in commands.txt", txt_cmd.command));
                update_command(&mut updated_commands, None, txt_cmd.clone(), false)?; // Don't flush yet
            }
        }
    }

    // Check for deleted commands (in cache but not in txt)
    for cached_cmd in &cached_commands {
        let still_exists = txt_commands.iter().any(|c|
            c.patch == cached_cmd.patch && c.command == cached_cmd.command && c.action == cached_cmd.action
        );

        if !still_exists {
            crate::utils::log(&format!("HISTORY_RESCAN: Command '{}' deleted from commands.txt", cached_cmd.command));
            delete_command(&mut updated_commands, cached_cmd)?;
        }
    }

    // Step 4: Scan filesystem for file changes
    crate::utils::log("HISTORY_RESCAN: Scanning filesystem for file changes...");
    let mut file_changes = 0;

    for cmd in updated_commands.iter_mut() {
        // Only check file-based commands
        if !cmd.is_path_based() {
            continue;
        }

        // Get file path
        let file_path = match cmd.get_absolute_file_path(&crate::core::sys_data::get_config()) {
            Some(path) => path,
            None => continue,
        };

        // Check if file exists and get current size
        let current_metadata = match std::fs::metadata(&file_path) {
            Ok(meta) => meta,
            Err(_) => continue, // File doesn't exist or can't be read
        };

        let current_size = current_metadata.len();
        let current_mtime = current_metadata.modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);

        // Check if file size changed
        // IMPORTANT: Only record modifications if we have BOTH old and new file sizes
        // If old file_size is None, we're just setting it for the first time (not a modification)
        if let Some(old_size) = cmd.file_size {
            if old_size != current_size {
                crate::utils::log(&format!("HISTORY_RESCAN: File size changed for '{}' ({} -> {})",
                    cmd.command, old_size, current_size));

                // Create updated command with new file size
                let old_cmd = cmd.clone();
                cmd.file_size = Some(current_size);
                if let Some(mtime) = current_mtime {
                    cmd.last_update = mtime;
                }

                // Record the change in history
                let conn = initialize_history_db()?;
                let timestamp = current_mtime.unwrap_or_else(|| {
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64
                });
                record_command_modified(&conn, &old_cmd, cmd, timestamp)?;

                file_changes += 1;
            }
        } else {
            // File exists but we don't have a stored size - just set it without recording modification
            cmd.file_size = Some(current_size);
            if let Some(mtime) = current_mtime {
                cmd.last_update = mtime;
            }
        }
    }

    crate::utils::log(&format!("HISTORY_RESCAN: Detected {} file changes", file_changes));

    // Step 5: Flush to disk
    crate::utils::log("HISTORY_RESCAN: Flushing commands to disk...");
    crate::core::commands::flush_commands(&updated_commands)?;

    crate::utils::log(&format!("HISTORY_RESCAN: Rescan complete. {} total commands", updated_commands.len()));

    Ok(updated_commands)
}
