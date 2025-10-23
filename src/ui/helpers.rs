//! UI Helper Functions
//!
//! Utility functions shared across UI components

use crate::core::data::get_config;

/// Gets the list of actions for the command editor dropdown
/// Returns the configured actions from popup_settings.listed_actions, or default actions if not configured
/// Always includes an empty entry at the front to allow clearing the action
pub fn get_listed_actions() -> Vec<String> {
    let config = get_config();

    let mut actions = match config.popup_settings.listed_actions {
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
                "console".to_string(),
                "chrome".to_string(),
            ]
        }
    };

    // Insert empty entry at the front to allow clearing the action
    actions.insert(0, String::new());

    actions
}