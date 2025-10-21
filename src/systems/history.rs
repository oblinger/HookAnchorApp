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
    let file_path_obj = cmd.get_absolute_file_path(&crate::core::data::get_config());
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
    let file_path_obj = new_cmd.get_absolute_file_path(&crate::core::data::get_config());
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
