//! Application state management
//! 
//! This module handles persistent application state that needs to be preserved
//! between runs, such as window position and build time.

use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use serde::{Deserialize, Serialize};
use chrono::Local;

/// History viewer state - all persistent state for the history viewer window
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HistoryViewerState {
    /// Window position (top-left corner)
    pub position: Option<(f32, f32)>,
    /// Window size (width, height)
    pub window_size: Option<(f32, f32)>,
    /// Patch/anchor name filter
    pub patch_filter: String,
    /// Command name search filter
    pub name_filter: String,
    /// Minimum edit size filter in bytes (None = blank/no filter, Some(n) = filter)
    pub min_edit_size: Option<i64>,
    /// Selected action types for filtering
    pub selected_action_types: Vec<String>,
    /// Sidebar width in pixels (user can resize this)
    pub sidebar_width: Option<f32>,
    /// Show current commands instead of history
    pub show_current_commands: bool,
}

impl Default for HistoryViewerState {
    fn default() -> Self {
        HistoryViewerState {
            position: None,
            window_size: None,
            patch_filter: String::new(),
            name_filter: String::new(),
            min_edit_size: Some(500), // Default to 500 bytes
            selected_action_types: Vec::new(),
            sidebar_width: None, // Use config default if not set
            show_current_commands: false, // Default to history mode
        }
    }
}

/// Application state that persists between runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Last saved window position
    pub window_position: Option<(f32, f32)>,
    /// History viewer state (all persistent state for history viewer)
    #[serde(default)]
    pub history_viewer_state: HistoryViewerState,
    /// Unix timestamp when this build was created
    pub build_time: Option<i64>,
    /// Unix timestamp when filesystem scan was last performed
    pub last_scan_time: Option<i64>,
    /// Checksum derived from last filesystem scan results
    pub last_scan_checksum: Option<String>,
    /// Last executed command (used for add_alias functionality)
    pub last_executed_command: Option<String>,
    /// Process ID of the running command server
    pub server_pid: Option<u32>,
    /// ISO timestamp when Notion was last scanned
    pub notion_last_scan: Option<String>,
    /// Anchor name - last executed anchor name
    pub anchor_name: Option<String>,
    /// Anchor timestamp - Unix timestamp when anchor was last set
    pub anchor_timestamp: Option<i64>,
    /// Anchor folder - folder path associated with last anchor
    pub anchor_folder: Option<String>,
    /// Whether to show files in prefix menus (default: true)
    #[serde(default = "default_show_files")]
    pub show_files: bool,
}

fn default_show_files() -> bool {
    true
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            window_position: None,
            history_viewer_state: HistoryViewerState::default(),
            build_time: None,
            last_scan_time: None,
            last_scan_checksum: None,
            last_executed_command: None,
            server_pid: None,
            notion_last_scan: None,
            anchor_name: None,
            anchor_timestamp: None,
            anchor_folder: None,
            show_files: true,
        }
    }
}

/// Returns the path to the state.json file
fn get_state_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/hookanchor/state.json")
}

/// Loads application state from state.json file, returns default if file doesn't exist or is invalid
/// Private to data module - external code must use get_state() from sys_data
pub(super) fn load_state() -> AppState {
    let state_path = get_state_file_path();
    if let Ok(contents) = fs::read_to_string(&state_path) {
        match serde_json::from_str::<AppState>(&contents) {
            Ok(state) => {
                crate::utils::detailed_log("STATE_IO", &format!(
                    "Loaded state from disk (build_time={:?}, last_cmd={:?}, server_pid={:?}, anchor={:?}, anchor_folder={:?})",
                    state.build_time,
                    state.last_executed_command,
                    state.server_pid,
                    state.anchor_name,
                    state.anchor_folder
                ));
                state
            },
            Err(_) => {
                crate::utils::detailed_log("STATE_IO", "Failed to parse state.json, using default");
                AppState::default()
            }
        }
    } else {
        crate::utils::detailed_log("STATE_IO", "state.json not found, using default");
        AppState::default()
    }
}

/// Saves application state to state.json file
/// Private to data module - external code must use set_state() from sys_data
pub(super) fn save_state(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let state_path = get_state_file_path();
    // Ensure config directory exists
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Serialize and save
    let json_content = serde_json::to_string_pretty(state)?;
    fs::write(&state_path, json_content)?;

    crate::utils::detailed_log("STATE_IO", &format!(
        "Saved state to disk (build_time={:?}, last_cmd={:?}, server_pid={:?}, anchor={:?}, anchor_folder={:?})",
        state.build_time,
        state.last_executed_command,
        state.server_pid,
        state.anchor_name,
        state.anchor_folder
    ));

    Ok(())
}