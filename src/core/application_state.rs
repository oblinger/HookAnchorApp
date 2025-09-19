//! Global application state management
//! 
//! This module contains the ApplicationState struct that manages global state
//! spanning both GUI and CLI modes, including commands, configuration, and search state.

use super::Config;
use super::Command;
use super::config::{load_config_with_error, ConfigResult};
use super::state::AppState;
use crate::{core::state::{load_state, save_state}, utils};

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
                utils::detailed_log("CONFIG_ERROR", &format!("Failed to load config: {}", error));
                (crate::core::sys_data::get_config(), Some(error)) // Use fallback config
            }
        };
        
        // Load sys data once to get commands
        let (sys_data, _) = super::sys_data::get_sys_data();
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
    
    /// Check and apply alias if search text matches - removed since rewrite action is deprecated
    pub fn check_and_apply_alias(&mut self) {
        // This functionality has been removed along with the rewrite action type
    }
    
    /// Get display commands with prefix menu information
    /// Returns (commands_to_display, is_in_prefix_menu, menu_prefix, inside_count)
    pub fn get_display_commands(&self) -> (Vec<Command>, bool, Option<String>, usize) {
        // Legacy ApplicationState is not used - return simple response
        (self.filtered_commands.clone(), false, None, 0)
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
        // Legacy ApplicationState is not used - simplified implementation
        let total_limit = self.config.popup_settings.max_rows * self.config.popup_settings.max_columns;
        self.filtered_commands = self.commands.clone();
        self.filtered_commands.truncate(total_limit);
    }
}