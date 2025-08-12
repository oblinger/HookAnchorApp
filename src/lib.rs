//! Anchor Selector Library
//! 
//! A command management and filtering library that provides fuzzy matching
//! and prioritized search for command execution.
//!
//! For complete configuration documentation including JavaScript API, built-in functions,
//! and usage examples, see `docs.md` in the project root.

use std::sync::OnceLock;
use std::path::PathBuf;

/// Global static to store the path of the currently running binary
/// This ensures all parts of the application use the same binary path for spawning processes
static BINARY_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Initialize the global binary path - should be called once at application startup
pub fn init_binary_path() {
    if let Ok(exe_path) = std::env::current_exe() {
        let _ = BINARY_PATH.set(exe_path);
    }
}

/// Get the path of the currently running binary
pub fn get_binary_path() -> Option<&'static PathBuf> {
    BINARY_PATH.get()
}

// Core modules
pub mod core;

// UI modules  
pub mod ui;

// New launcher modules
pub mod eval;
pub mod command_launcher;
pub mod js_runtime;
pub mod business_logic;
pub mod builtin_fns;

// Simplified execution system (PRD)
pub mod execute;

// Command-line interface
pub mod cmd;
pub mod utils;
pub mod scanner;
pub mod grabber;
pub mod vault;
pub mod command_operations;

// Background command server
pub mod command_server;
pub use command_server::{CommandClient, execute_via_server};

// Setup assistant for first-run configuration
pub mod setup_assistant;

// Process monitoring for non-blocking execution
pub mod process_monitor;

// Error display system
pub mod error_display;

// Server management
pub mod command_server_management;

// Popup server control
pub mod popup_server_control;

// Re-export commonly used types from core modules
pub use core::commands::{Command, CommandTarget, Patch, filter_commands, get_display_commands, get_display_commands_with_options, merge_similar_commands, 
                         merge_similar_commands_with_context, load_commands, load_commands_with_data, load_commands_for_inference, save_commands_to_file, 
                         add_command, delete_command, parse_command_line, split_commands,
                         get_current_submenu_prefix, migrate_commands_to_new_format,
                         command_matches_query, command_matches_query_with_debug, get_command_prefix, auto_assign_patches, infer_patch, run_patch_inference,
                         get_patch_for_command};
pub use core::sys_data::{SysData, load_data, get_sys_data, get_config, clear_sys_data};
// Note: Path accessor methods (get_absolute_file_path, get_absolute_folder_path, is_path_based) are available as Command impl methods
pub use core::config::{Config, PopupSettings, LauncherSettings};
pub use core::state::{AppState, load_state, save_state};
pub use core::ApplicationState;

// =============================================================================
// Additional Helper Functions
// =============================================================================

/// Gets the list of actions for the command editor dropdown
/// Returns the configured actions from popup_settings.listed_actions, or default actions if not configured
pub fn get_listed_actions() -> Vec<String> {
    let config = get_config();
    
    match config.popup_settings.listed_actions {
        Some(actions_str) => {
            // Split by comma and trim whitespace
            actions_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
        None => {
            // Default actions if not configured
            vec![
                "app".to_string(),
                "url".to_string(),
                "folder".to_string(),
                "cmd".to_string(),
                "chrome".to_string(),
                "anchor".to_string(),
            ]
        }
    }
}
