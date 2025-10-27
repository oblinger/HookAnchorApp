//! Command history tracking using SQLite
//!
//! This module provides functionality to track all changes to commands over time.
//! It maintains a SQLite database with command history and provides query capabilities.
//!
//! # Architecture
//!
//! History recording is **completely automatic** - it happens inside sys_data functions.
//! External code never calls history functions directly.
//!
//! - Command appears: Recorded with its normal action (markdown, app, etc.)
//! - Command changes: New entry recorded with updated values
//! - Command deleted: Special entry recorded with action="$DELETED$"
//!
//! The only public function is `get_history_entries()` which is exposed via sys_data
//! for the history_viewer binary to read the history database.

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
///
/// This is pub(super) so only sys_data can call it
pub(super) fn initialize_history_db() -> SqlResult<Connection> {
    let db_path = get_history_db_path();
    let conn = Connection::open(db_path)?;

    // Create the command_history table (without change_type column)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS command_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,

            patch TEXT NOT NULL,
            command TEXT NOT NULL,
            action TEXT NOT NULL,

            arg TEXT,
            flags TEXT,
            file_path TEXT,

            edit_size INTEGER
        )",
        [],
    )?;

    // Migration: Remove change_type column if it exists (from old schema)
    // SQLite doesn't support DROP COLUMN directly, so we check if it exists first
    let has_change_type: bool = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('command_history') WHERE name='change_type'",
        [],
        |row| {
            let count: i32 = row.get(0)?;
            Ok(count > 0)
        },
    ).unwrap_or(false);

    if has_change_type {
        crate::utils::log("HISTORY_MIGRATION: Removing change_type column from history database");

        // Create new table without change_type
        conn.execute(
            "CREATE TABLE IF NOT EXISTS command_history_new (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                patch TEXT NOT NULL,
                command TEXT NOT NULL,
                action TEXT NOT NULL,
                arg TEXT,
                flags TEXT,
                file_path TEXT,
                edit_size INTEGER
            )",
            [],
        )?;

        // Copy data, mapping change_type to action for deletions
        conn.execute(
            "INSERT INTO command_history_new (id, timestamp, patch, command, action, arg, flags, file_path, edit_size)
             SELECT id, timestamp, patch, command,
                    CASE WHEN change_type = 'deleted' THEN '$DELETED$' ELSE action END,
                    arg, flags, file_path, edit_size
             FROM command_history",
            [],
        )?;

        // Drop old table and rename new one
        conn.execute("DROP TABLE command_history", [])?;
        conn.execute("ALTER TABLE command_history_new RENAME TO command_history", [])?;

        crate::utils::log("HISTORY_MIGRATION: Migration complete");
    }

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

/// Append a command to the history database
///
/// This is the ONLY write operation. It simply appends the command's current state.
/// To record a deletion, the command's action should be set to "$DELETED$".
///
/// This is pub(super) so only sys_data can call it.
pub(super) fn append_command(conn: &Connection, cmd: &Command, timestamp: i64) -> SqlResult<()> {
    let file_path_obj = cmd.get_absolute_file_path(&crate::core::data::get_config());
    let file_path = file_path_obj.as_ref().map(|p| p.to_string_lossy().to_string());

    // For file-based commands, try to get the file's creation/modification time
    // For non-file commands or deletions, use the provided timestamp
    let actual_timestamp = if cmd.is_path_based() && cmd.action != "$DELETED$" {
        if let Some(ref path) = file_path_obj {
            // Try to get file modification time for better tracking
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
        timestamp // Non-file command or deletion, use provided timestamp
    };

    // Calculate edit_size for file-based commands
    let edit_size = if cmd.action != "$DELETED$" {
        cmd.file_size.map(|size| size as i64)
    } else {
        None // Deletions don't have a size
    };

    conn.execute(
        "INSERT INTO command_history
         (timestamp, patch, command, action, arg, flags, file_path, edit_size)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            actual_timestamp,
            &cmd.patch,
            &cmd.command,
            &cmd.action,
            &cmd.arg,
            &cmd.flags,
            file_path,
            edit_size,
        ],
    )?;

    // Log the history entry
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

    crate::utils::detailed_log("HISTORY", &format!(
        "[{}] action={} cmd='{}' patch='{}'{}",
        date_str, cmd.action, cmd.command, cmd.patch, size_str
    ));

    Ok(())
}


/// Get history entries with a limit
///
/// This is the public read function exposed via sys_data for the history_viewer.
/// Returns entries ordered by timestamp DESC (newest first).
///
/// # Arguments
/// * `limit` - Maximum number of entries to return
/// * `exclude_deletions` - If true, filters out entries with action="$DELETED$"
pub fn get_history_entries(limit: usize, exclude_deletions: bool) -> SqlResult<Vec<HistoryEntry>> {
    let conn = initialize_history_db()?;

    let query = format!(
        "SELECT id, timestamp, patch, command, action, arg, flags, file_path, edit_size
         FROM command_history
         ORDER BY timestamp DESC
         LIMIT {}",
        limit
    );

    let mut stmt = conn.prepare(&query)?;
    let entries = stmt.query_map([], |row| {
        Ok(HistoryEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            patch: row.get(2)?,
            command: row.get(3)?,
            action: row.get(4)?,
            arg: row.get(5)?,
            flags: row.get(6)?,
            file_path: row.get(7)?,
            edit_size: row.get(8)?,
        })
    })?;

    let mut result = Vec::new();
    for entry in entries {
        result.push(entry?);
    }

    if exclude_deletions {
        result.retain(|entry| !entry.is_deletion());
    }

    Ok(result)
}


/// Delete the history database file
///
/// This function removes the SQLite database file at ~/.config/hookanchor/history.db.
/// All command execution history and file modification tracking records are permanently lost.
///
/// This is pub(super) so only sys_data can call it as part of coordinated cleanup operations.
///
/// # Returns
/// * `Ok(true)` - Database file existed and was deleted
/// * `Ok(false)` - Database file did not exist (nothing to delete)
/// * `Err(String)` - Failed to delete existing database file
pub(super) fn delete_history_db() -> Result<bool, String> {
    let db_path = get_history_db_path();

    if db_path.exists() {
        std::fs::remove_file(&db_path)
            .map_err(|e| format!("Failed to delete history database: {}", e))?;
        crate::utils::log(&format!("Deleted history database: {}", db_path.display()));
        Ok(true)
    } else {
        Ok(false)
    }
}


/// Represents a single history entry from the database
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub id: i64,
    pub timestamp: i64,
    // Note: No change_type field - deletions are identified by action = "$DELETED$"
    pub patch: String,
    pub command: String,
    pub action: String,
    pub arg: Option<String>,
    pub flags: Option<String>,
    pub file_path: Option<String>,
    pub edit_size: Option<i64>,
}

impl HistoryEntry {
    /// Check if this entry represents a deletion
    pub fn is_deletion(&self) -> bool {
        self.action == "$DELETED$"
    }
}
