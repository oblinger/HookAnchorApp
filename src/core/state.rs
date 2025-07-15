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
    
    if let Ok(contents) = fs::read_to_string(&state_path) {
        match serde_json::from_str::<AppState>(&contents) {
            Ok(state) => state,
            Err(_) => AppState::default()
        }
    } else {
        AppState::default()
    }
}

/// Saves application state to state.json file
pub fn save_state(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let state_path = get_state_file_path();
    
    // Ensure config directory exists
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Serialize and save
    let json_content = serde_json::to_string_pretty(state)?;
    fs::write(&state_path, json_content)?;
    
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
    let mut state = load_state();
    state.last_executed_command = Some(command_name.to_string());
    save_state(&state)
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