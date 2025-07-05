//! Anchor Selector Library
//! 
//! A command management and filtering library that provides fuzzy matching
//! and prioritized search for command execution.
//!
//! For complete configuration documentation including JavaScript API, built-in functions,
//! and usage examples, see `docs.md` in the project root.

// Core modules
pub mod core;

// UI modules  
pub mod ui;

// New launcher modules
pub mod eval;
pub mod launcher;
pub mod js_runtime;
pub mod business_logic;
pub mod builtin_fns;

// Command-line interface
pub mod cmd;
pub mod utils;
pub mod scanner;
pub mod grabber;

// Re-export commonly used types from core modules
pub use core::commands::{Command, CommandTarget, filter_commands, get_display_commands, merge_similar_commands, 
                         merge_similar_commands_with_context, load_commands, save_commands_to_file, 
                         add_command, delete_command, parse_command_line, split_commands, 
                         get_current_submenu_prefix, execute_command, migrate_commands_to_new_format,
                         command_matches_query, command_matches_query_with_debug, get_command_prefix};
pub use core::config::{Config, PopupSettings, LauncherSettings, load_config};
pub use core::state::{AppState, load_state, save_state};

// =============================================================================
// Additional Helper Functions
// =============================================================================

/// Gets the list of actions for the command editor dropdown
/// Returns the configured actions from popup_settings.listed_actions, or default actions if not configured
pub fn get_listed_actions() -> Vec<String> {
    let config = load_config();
    
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