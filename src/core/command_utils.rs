//! Utility functions for working with commands
//!
//! This module contains read-only utility functions that query or transform
//! commands without mutating the command list.

use crate::core::{Command, Config};
use std::fs;

/// Get list of files from an anchor's folder as Command objects
///
/// Returns a vector of temporary Command structs with action type "file"
/// representing the files in the anchor's folder.
///
/// # Arguments
/// * `anchor` - The anchor command whose folder to scan
/// * `config` - Application configuration (for path resolution)
///
/// # Returns
/// Vector of Command objects representing files, or empty vec if folder doesn't exist
pub fn get_folder_files(anchor: &Command, config: &Config) -> Vec<Command> {
    // Get the folder path from the anchor
    let folder_path = match anchor.get_absolute_folder_path(config) {
        Some(path) => path,
        None => return Vec::new(),
    };

    // Check if folder exists
    if !folder_path.exists() || !folder_path.is_dir() {
        return Vec::new();
    }

    // Read directory entries (non-recursive)
    let entries = match fs::read_dir(&folder_path) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    // Collect files and sort alphabetically
    let mut files: Vec<(String, String)> = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            // Only include files, not directories
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    let name = file_name.to_string_lossy().to_string();
                    let full_path = path.to_string_lossy().to_string();
                    files.push((name, full_path));
                }
            }
        }
    }

    // Sort alphabetically by file name
    files.sort_by(|a, b| a.0.cmp(&b.0));

    // Convert to Command structs with "file" action type
    files.into_iter().map(|(name, path)| {
        Command {
            command: name,
            action: "file".to_string(),
            arg: path,
            patch: String::new(),
            flags: String::new(),
            other_params: None,
            last_update: 0,
            file_size: None,
        }
    }).collect()
}
