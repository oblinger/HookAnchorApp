//! Application state management
//! 
//! This module handles persistent application state that needs to be preserved
//! between runs, such as window position and build time.

use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use serde::{Deserialize, Serialize};
use chrono::Local;

/// Application state that persists between runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Last saved window position
    pub window_position: Option<(f32, f32)>,
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
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            window_position: None,
            build_time: None,
            last_scan_time: None,
            last_scan_checksum: None,
            last_executed_command: None,
            server_pid: None,
        }
    }
}

/// Returns the path to the state.json file
pub fn get_state_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/hookanchor/state.json")
}

/// Loads application state from state.json file, returns default if file doesn't exist or is invalid
pub fn load_state() -> AppState {
    let state_path = get_state_file_path();
    crate::utils::debug_log("STATE_LOAD", &format!("=== LOADING STATE FROM: {} ===", state_path.display()));
    
    if let Ok(contents) = fs::read_to_string(&state_path) {
        match serde_json::from_str::<AppState>(&contents) {
            Ok(state) => {
                crate::utils::debug_log("STATE_LOAD", &format!("Successfully loaded state - last_executed_command: {:?}", state.last_executed_command));
                state
            },
            Err(e) => {
                crate::utils::debug_log("STATE_LOAD", &format!("Failed to parse state JSON: {}", e));
                AppState::default()
            }
        }
    } else {
        crate::utils::debug_log("STATE_LOAD", "State file doesn't exist or can't be read, using default");
        AppState::default()
    }
}

/// Saves application state to state.json file
pub fn save_state(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let state_path = get_state_file_path();
    crate::utils::debug_log("STATE_SAVE_DETAIL", &format!("=== SAVING STATE TO: {} ===", state_path.display()));
    crate::utils::debug_log("STATE_SAVE_DETAIL", &format!("State being saved - last_executed_command: {:?}", state.last_executed_command));
    
    // Ensure config directory exists
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Serialize and save
    let json_content = serde_json::to_string_pretty(state)?;
    crate::utils::debug_log("STATE_SAVE_DETAIL", &format!("JSON content to write: {}", json_content.chars().take(200).collect::<String>()));
    fs::write(&state_path, json_content)?;
    crate::utils::debug_log("STATE_SAVE_DETAIL", "State file successfully written to disk");
    
    Ok(())
}

/// Updates build time in state.json file
pub fn save_state_with_build_time() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = load_state();
    state.build_time = Some(Local::now().timestamp());
    save_state(&state)
}

/// Updates last executed command in state.json file
pub fn save_last_executed_command(command_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    crate::utils::debug_log("STATE_SAVE", &format!("=== SAVING LAST EXECUTED COMMAND: '{}' ===", command_name));
    let mut state = load_state();
    let old_command = state.last_executed_command.clone();
    state.last_executed_command = Some(command_name.to_string());
    crate::utils::debug_log("STATE_SAVE", &format!("Previous command was: {:?}, new command: '{}'", old_command, command_name));
    let result = save_state(&state);
    if result.is_ok() {
        crate::utils::debug_log("STATE_SAVE", "Successfully wrote state file");
    } else {
        crate::utils::debug_log("STATE_SAVE", &format!("Failed to write state file: {:?}", result));
    }
    result
}

/// Updates server PID in state.json file
pub fn save_server_pid(pid: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = load_state();
    state.server_pid = Some(pid);
    save_state(&state)
}

/// Clears server PID from state.json file
pub fn clear_server_pid() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = load_state();
    state.server_pid = None;
    save_state(&state)
}