//! Command Store - Unified command persistence and history tracking
//!
//! This module is the single source of truth for command state management.
//! It coordinates between:
//! - commands.txt (persistent command list)
//! - commands_cache.json (performance optimization)
//! - history.db (change tracking database)
//!
//! All command mutations MUST go through this module to ensure history tracking.

use crate::core::Command;
use std::error::Error;

/// Result type for commandstore operations
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

// ============================================================================
// State Management
// ============================================================================

/// Load commands from persistent storage
/// Tries cache first, falls back to commands.txt
pub fn load() -> Result<Vec<Command>> {
    // Try cache first for performance
    if let Some(commands) = crate::core::commands::load_commands_from_cache() {
        crate::utils::log(&format!("COMMANDSTORE: Loaded {} commands from cache", commands.len()));
        return Ok(commands);
    }

    // Fall back to commands.txt
    let commands = crate::core::commands::load_commands();
    crate::utils::log(&format!("COMMANDSTORE: Loaded {} commands from commands.txt", commands.len()));

    // Rebuild cache
    crate::core::commands::save_commands_to_cache(&commands)?;

    Ok(commands)
}

// ============================================================================
// Single Command Mutations
// ============================================================================

/// Add a new command
/// Automatically records creation in history and saves to disk
pub fn add(cmd: Command, commands: &mut Vec<Command>) -> Result<()> {
    // Initialize history database
    let conn = crate::systems::history::initialize_history_db()?;

    // Get current timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    // Record creation in history
    crate::systems::history::record_command_created(&conn, &cmd, timestamp)?;

    // Add to commands list
    commands.push(cmd);

    // Save to disk
    save_commands(commands)?;

    Ok(())
}

/// Delete a command by name
/// Automatically saves to disk (history recording removed since record_command_deleted was unused)
pub fn delete(cmd_name: &str, commands: &mut Vec<Command>) -> Result<()> {
    // Remove from commands list
    commands.retain(|c| c.command != cmd_name);

    // Save to disk
    save_commands(commands)?;

    Ok(())
}

// ============================================================================
// Save Operations
// ============================================================================

/// Save commands to disk (txt + cache) without recording history
/// Used by scanner which records history manually during its workflow
pub fn save(commands: &[Command]) -> Result<()> {
    save_commands(commands)
}

// ============================================================================
// Internal Helpers
// ============================================================================

/// Save commands to both txt and cache
fn save_commands(commands: &[Command]) -> Result<()> {
    crate::core::commands::save_commands_to_file(commands)?;
    crate::core::commands::save_commands_to_cache(commands)?;
    Ok(())
}
