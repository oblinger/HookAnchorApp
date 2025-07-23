//! Global application state management
//! 
//! This module contains the ApplicationState struct that manages global state
//! spanning both GUI and CLI modes, including commands, configuration, and search state.

use super::{Command, Config};
use super::config::{load_config_with_error, ConfigResult};
use super::state::AppState;
use crate::{load_state, save_state, utils};

/// Global application state that spans both GUI and CLI modes
pub struct ApplicationState {
    /// All available commands
    pub commands: Vec<Command>,
    /// Current search text
    pub search_text: String,
    /// Commands that match current search
    pub filtered_commands: Vec<Command>,
    /// Application configuration
    pub config: Config,
    /// Persistent application state
    pub app_state: AppState,
    /// Config error if loading failed
    pub config_error: Option<String>,
}

impl ApplicationState {
    /// Create minimal application state for immediate GUI startup
    pub fn minimal() -> Self {
        Self {
            commands: Vec::new(),
            search_text: String::new(),
            filtered_commands: Vec::new(),
            config: Config::default(),
            app_state: AppState::default(),
            config_error: None,
        }
    }
    
    /// Create new application state by loading from files
    pub fn new() -> Self {
        // Load config with error handling first
        let config_result = load_config_with_error();
        let (config, config_error) = match config_result {
            ConfigResult::Success(cfg) => (cfg, None),
            ConfigResult::Error(error) => {
                utils::debug_log("CONFIG_ERROR", &format!("Failed to load config: {}", error));
                (crate::core::sys_data::get_config(), Some(error)) // Use fallback config
            }
        };
        
        // Load sys data once to get commands
        let sys_data = super::sys_data::get_sys_data();
        let commands = sys_data.commands;
        
        // Don't scan at startup - only scan at termination
        // commands = scanner::startup_check(commands);
        
        let app_state = load_state();
        
        let application_state = Self {
            commands,
            search_text: String::new(),
            filtered_commands: Vec::new(),
            config,
            app_state,
            config_error,
        };
        
        // Note: Application now uses resources from within the project directory
        // No external data directory initialization needed
        
        application_state
    }
    
    /// Create new application state with initial search text
    pub fn new_with_search(initial_search: &str) -> Self {
        let mut state = Self::new();
        if !initial_search.is_empty() {
            state.update_search(initial_search.to_string());
        }
        state
    }
    
    /// Update search text and recompute filtered commands
    pub fn update_search(&mut self, new_search: String) {
        self.search_text = new_search;
        self.recompute_filtered_commands();
    }
    
    /// Get a reference to all commands
    pub fn get_commands(&self) -> &[Command] {
        &self.commands
    }
    
    /// Get mutable reference to commands
    pub fn get_commands_mut(&mut self) -> &mut Vec<Command> {
        &mut self.commands
    }
    
    /// Update the command list (used for deferred scanner updates)
    pub fn set_commands(&mut self, commands: Vec<Command>) {
        self.commands = commands;
        self.recompute_filtered_commands();
    }
    
    /// Check if a command is a separator
    pub fn is_separator_command(cmd: &Command) -> bool {
        cmd.action == "separator" && (cmd.command == "---" || cmd.command.ends_with("---"))
    }
    
    /// Check and apply alias if search text matches a rewrite command
    pub fn check_and_apply_alias(&mut self) {
        // Check if search text ends with any word separator and the prefix matches a rewrite command
        if let Some(last_char) = self.search_text.chars().last() {
            if self.config.popup_settings.word_separators.contains(last_char) {
                let prefix = self.search_text.trim_end_matches(&self.config.popup_settings.word_separators.chars().collect::<Vec<_>>()[..]);
                if let Some(rewrite_cmd) = self.commands.iter().find(|cmd| 
                    cmd.action == "rewrite" && cmd.command.to_lowercase() == prefix.to_lowercase()
                ) {
                    // Replace search text with the rewrite argument followed by the same separator
                    let new_search = format!("{}{}", rewrite_cmd.arg, last_char);
                    self.search_text = new_search;
                    self.recompute_filtered_commands();
                }
            }
        }
    }
    
    /// Get display commands with submenu information
    /// Returns (commands_to_display, is_in_submenu, menu_prefix, inside_count)
    pub fn get_display_commands(&self) -> (Vec<Command>, bool, Option<String>, usize) {
        use super::commands::get_current_submenu_prefix_from_commands;
        
        // Check if we're in submenu mode
        if let Some(menu_prefix) = get_current_submenu_prefix_from_commands(&self.filtered_commands, &self.search_text, &self.config.popup_settings.word_separators) {
            // We're in submenu mode - the commands are already organized
            // Just find the separator to count inside commands
            let separator_index = self.filtered_commands.iter().position(|cmd| Self::is_separator_command(cmd));
            let inside_count = separator_index.unwrap_or(self.filtered_commands.len());
            
            (self.filtered_commands.clone(), true, Some(menu_prefix), inside_count)
        } else {
            // Not in submenu mode, use filtered commands directly
            (self.filtered_commands.clone(), false, None, 0)
        }
    }
    
    /// Get hint text for the search box
    pub fn get_hint_text(&self) -> String {
        let base_text = "Type to search commands...";
        
        // Add build time if recent (within 10 minutes)
        if let Some(build_time) = self.app_state.build_time {
            let current_time = chrono::Local::now().timestamp();
            let seconds_since_build = current_time - build_time;
            
            if seconds_since_build < 600 { // 10 minutes
                return format!("{} {}s", base_text, seconds_since_build);
            }
        }
        
        base_text.to_string()
    }
    
    /// Update window position in app state
    pub fn update_window_position(&mut self, position: (f32, f32)) {
        self.app_state.window_position = Some(position);
        let _ = save_state(&self.app_state);
    }
    
    /// Get saved window position
    pub fn get_window_position(&self) -> Option<(f32, f32)> {
        self.app_state.window_position
    }
    
    // Note: All application data directory functionality has been removed
    // Resources are now kept within the project directory at resources/common/
    
    /// Recompute filtered commands based on current search
    fn recompute_filtered_commands(&mut self) {
        use super::commands::get_display_commands;
        
        let total_limit = self.config.popup_settings.max_rows * self.config.popup_settings.max_columns;
        let sys_data = super::sys_data::SysData {
            config: self.config.clone(),
            commands: self.commands.clone(),
            patches: std::collections::HashMap::new(), // Empty for filtering
        };
        self.filtered_commands = get_display_commands(&sys_data, &self.search_text, total_limit);
    }
}