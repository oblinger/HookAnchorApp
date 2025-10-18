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
            new_values TEXT
        )",
        [],
    )?;

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
    let file_path = cmd.get_absolute_file_path(&crate::core::sys_data::get_config())
        .map(|p| p.to_string_lossy().to_string());

    conn.execute(
        "INSERT INTO command_history
         (timestamp, change_type, patch, command, action, arg, flags, file_path, changed_fields, old_values, new_values)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, NULL, NULL)",
        params![
            timestamp,
            "created",
            &cmd.patch,
            &cmd.command,
            &cmd.action,
            &cmd.arg,
            &cmd.flags,
            file_path,
        ],
    )?;

    Ok(())
}

/// Record a command modification in the history database
pub fn record_command_modified(
    conn: &Connection,
    old_cmd: &Command,
    new_cmd: &Command,
    timestamp: i64,
) -> SqlResult<()> {
    let file_path = new_cmd.get_absolute_file_path(&crate::core::sys_data::get_config())
        .map(|p| p.to_string_lossy().to_string());

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
         (timestamp, change_type, patch, command, action, arg, flags, file_path, changed_fields, old_values, new_values)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            timestamp,
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
        "SELECT id, timestamp, change_type, patch, command, action, arg, flags, file_path, changed_fields, old_values, new_values
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
        "SELECT id, timestamp, change_type, patch, command, action, arg, flags, file_path, changed_fields, old_values, new_values
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
}
