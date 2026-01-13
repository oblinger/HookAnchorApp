//! Anchor Operations
//!
//! All anchor-related business logic consolidated in one place.
//! This is the single source of truth for anchor state management.
//!
//! An "anchor" is a command that provides context (especially folder context)
//! for other commands. When activated, an anchor's folder becomes available
//! as a template variable for subsequent commands.

use std::path::Path;
use crate::core::Command;

/// Information about the currently active anchor
#[derive(Debug, Clone, Default)]
pub struct AnchorInfo {
    /// Name of the anchor command
    pub name: Option<String>,
    /// Unix timestamp when anchor was activated
    pub timestamp: Option<i64>,
    /// Folder path associated with the anchor
    pub folder: Option<String>,
}

impl AnchorInfo {
    /// Check if an anchor is currently active
    pub fn is_active(&self) -> bool {
        self.name.is_some()
    }

    /// Check if the anchor has expired based on timeout
    pub fn is_expired(&self, timeout_seconds: i64) -> bool {
        if let Some(timestamp) = self.timestamp {
            let now = chrono::Local::now().timestamp();
            (now - timestamp) > timeout_seconds
        } else {
            true // No timestamp means effectively expired
        }
    }
}

// =============================================================================
// ANCHOR STATE MANAGEMENT
// =============================================================================

/// Get the currently active anchor info from application state
pub fn get_anchor() -> AnchorInfo {
    let state = crate::core::data::get_state();
    AnchorInfo {
        name: state.anchor_name,
        timestamp: state.anchor_timestamp,
        folder: state.anchor_folder,
    }
}

/// Set the active anchor - single source of truth for anchor activation
///
/// This function is the centralized way to set the active anchor. All code that
/// needs to activate an anchor should call this function.
///
/// # Arguments
/// * `name` - The name of the anchor command
/// * `folder` - Optional folder path associated with the anchor
///
/// # Returns
/// * `Ok(())` if the anchor was successfully set
/// * `Err` if saving state failed
pub fn set_anchor(name: String, folder: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    crate::core::data::set_active_anchor(name, folder)
}

/// Clear the active anchor
///
/// Removes the current anchor from state, effectively deactivating it.
pub fn clear_anchor() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = crate::core::data::get_state();
    state.anchor_name = None;
    state.anchor_timestamp = None;
    state.anchor_folder = None;
    crate::core::data::set_state(&state)
}

// =============================================================================
// FOLDER EXTRACTION
// =============================================================================

/// Extract folder path from a command's arg (file path)
///
/// Returns the parent directory if arg is a file, or the directory itself if arg is a directory.
/// This is used when activating an anchor to determine its folder context.
///
/// # Arguments
/// * `cmd` - The command to extract folder from
///
/// # Returns
/// * `Some(folder_path)` if a folder could be extracted
/// * `None` if the command has no arg or folder couldn't be determined
pub fn extract_folder_from_command(cmd: &Command) -> Option<String> {
    if cmd.arg.is_empty() {
        return None;
    }

    // Expand tilde in path
    let expanded_path = if cmd.arg.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            home.join(&cmd.arg[2..]).to_string_lossy().to_string()
        } else {
            cmd.arg.clone()
        }
    } else {
        cmd.arg.clone()
    };

    let path = Path::new(&expanded_path);
    if path.is_file() {
        path.parent().map(|p| p.to_string_lossy().to_string())
    } else if path.is_dir() {
        Some(expanded_path)
    } else {
        // Path doesn't exist, try to get parent anyway
        path.parent().map(|p| p.to_string_lossy().to_string())
    }
}

/// Extract folder path from a file path string
///
/// Returns the parent directory of the given path.
pub fn extract_folder_from_path(path: &str) -> Option<String> {
    let path = Path::new(path);
    path.parent()
        .and_then(|p| p.to_str())
        .map(|s| s.to_string())
}

/// Extract folder from a shell command that starts with `cd`
///
/// Parses commands like `cd "/path/to/folder" && ...` and extracts the folder path.
///
/// # Arguments
/// * `arg` - The command argument string
///
/// # Returns
/// * `Ok(folder_path)` if a cd command was found and parsed
/// * `Err(message)` if parsing failed
pub fn extract_folder_from_cd_command(arg: &str) -> Result<String, String> {
    let trimmed = arg.trim();

    // Check if it starts with cd
    if !trimmed.starts_with("cd ") {
        return Err("Command does not start with 'cd'".to_string());
    }

    // Find the && terminator
    let cd_part = if let Some(idx) = trimmed.find(" &&") {
        &trimmed[3..idx] // Skip "cd " and take up to " &&"
    } else {
        return Err("No && found after cd command".to_string());
    };

    // Trim and remove quotes
    let folder = cd_part.trim();
    let folder = if (folder.starts_with('"') && folder.ends_with('"')) ||
                    (folder.starts_with('\'') && folder.ends_with('\'')) {
        &folder[1..folder.len()-1]
    } else {
        folder
    };

    Ok(folder.to_string())
}

/// Extract and validate folder from a command based on its action type
///
/// Different action types store folder information differently:
/// - `cmd`: Look for `cd "/path" && ...` pattern
/// - `folder`, `tmux`: The arg is the folder itself
/// - `markdown`, `doc`, `text`: Extract parent from file path, or use directly if directory
///
/// # Arguments
/// * `cmd` - The command to extract folder from
///
/// # Returns
/// * `Ok(folder_path)` - Normalized, validated folder path
/// * `Err(message)` - If folder couldn't be extracted or doesn't exist
pub fn extract_and_validate_folder(cmd: &Command) -> Result<String, String> {
    let folder = match cmd.action.as_str() {
        "cmd" => {
            // Special handling for cmd - look for cd command
            extract_folder_from_cd_command(&cmd.arg)?
        },
        "folder" | "tmux" => {
            // For folder/tmux actions, the arg is the folder itself
            cmd.arg.clone()
        },
        "markdown" | "doc" | "text" => {
            // For file-based actions, check if arg is a directory or file
            let expanded_arg = if cmd.arg.starts_with("~/") {
                cmd.arg.replacen("~", &std::env::var("HOME").unwrap_or_default(), 1)
            } else {
                cmd.arg.clone()
            };

            let path = Path::new(&expanded_arg);
            if path.is_dir() {
                // If it's already a directory (like /path/to/folder), use it directly
                expanded_arg
            } else {
                // If it's a file, extract parent folder
                extract_folder_from_path(&cmd.arg)
                    .ok_or_else(|| format!("Could not extract folder from path: {}", cmd.arg))?
            }
        },
        _ => {
            // Other actions don't have folder context
            return Err(format!("Action '{}' does not provide folder context", cmd.action));
        }
    };

    // Expand tilde if present
    let expanded = if folder.starts_with("~/") {
        folder.replacen("~", &std::env::var("HOME").unwrap_or_default(), 1)
    } else {
        folder.clone()
    };

    // Verify folder exists
    if !Path::new(&expanded).exists() {
        return Err(format!("Folder does not exist: {}", expanded));
    }

    // Remove trailing slash to avoid double slashes in templates
    let normalized = if expanded.ends_with('/') && expanded.len() > 1 {
        expanded[..expanded.len() - 1].to_string()
    } else {
        expanded
    };

    Ok(normalized)
}

// =============================================================================
// ANCHOR CONTEXT FOR TEMPLATES
// =============================================================================

/// Get anchor context for template variable expansion
///
/// Returns the anchor information formatted for use in template contexts.
/// If no anchor is active or it's expired, returns empty strings.
///
/// # Arguments
/// * `timeout_seconds` - How long an anchor remains active
///
/// # Returns
/// Tuple of (anchor_name, anchor_folder) - empty strings if no active anchor
pub fn get_anchor_context(timeout_seconds: i64) -> (String, String) {
    let anchor = get_anchor();

    if anchor.is_active() && !anchor.is_expired(timeout_seconds) {
        (
            anchor.name.unwrap_or_default(),
            anchor.folder.unwrap_or_default(),
        )
    } else {
        (String::new(), String::new())
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_folder_from_cd_command() {
        // Valid cd command with double quotes
        assert_eq!(
            extract_folder_from_cd_command("cd \"/path/to/folder\" && ls"),
            Ok("/path/to/folder".to_string())
        );

        // Valid cd command with single quotes
        assert_eq!(
            extract_folder_from_cd_command("cd '/path/to/folder' && echo hi"),
            Ok("/path/to/folder".to_string())
        );

        // Valid cd command without quotes
        assert_eq!(
            extract_folder_from_cd_command("cd /simple/path && pwd"),
            Ok("/simple/path".to_string())
        );

        // Invalid - no cd
        assert!(extract_folder_from_cd_command("ls /path").is_err());

        // Invalid - no &&
        assert!(extract_folder_from_cd_command("cd /path").is_err());
    }

    #[test]
    fn test_extract_folder_from_path() {
        assert_eq!(
            extract_folder_from_path("/path/to/file.txt"),
            Some("/path/to".to_string())
        );

        assert_eq!(
            extract_folder_from_path("/root"),
            Some("/".to_string())
        );
    }

    #[test]
    fn test_anchor_info_is_expired() {
        let mut anchor = AnchorInfo::default();

        // No timestamp means expired
        assert!(anchor.is_expired(60));

        // Recent timestamp means not expired
        anchor.timestamp = Some(chrono::Local::now().timestamp());
        assert!(!anchor.is_expired(60));

        // Old timestamp means expired
        anchor.timestamp = Some(chrono::Local::now().timestamp() - 120);
        assert!(anchor.is_expired(60));
    }
}
