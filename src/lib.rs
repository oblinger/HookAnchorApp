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
// Global Application State
// =============================================================================

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
    /// Create new application state by loading from files
    pub fn new() -> Self {
        let mut commands = load_commands();
        
        // Load config with error handling
        let config_result = core::config::load_config_with_error();
        let (config, config_error) = match config_result {
            core::config::ConfigResult::Success(cfg) => (cfg, None),
            core::config::ConfigResult::Error(error) => {
                utils::debug_log("CONFIG_ERROR", &format!("Failed to load config: {}", error));
                (load_config(), Some(error)) // Use fallback config
            }
        };
        
        // Always run scanner startup check which handles timing internally
        commands = scanner::startup_check(commands);
        
        let app_state = load_state();
        
        Self {
            commands,
            search_text: String::new(),
            filtered_commands: Vec::new(),
            config,
            app_state,
            config_error,
        }
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
        // Check if search text ends with space and the prefix matches a rewrite command
        if self.search_text.ends_with(' ') {
            let prefix = self.search_text.trim_end();
            if let Some(rewrite_cmd) = self.commands.iter().find(|cmd| 
                cmd.action == "rewrite" && cmd.command.to_lowercase() == prefix.to_lowercase()
            ) {
                // Replace search text with the rewrite argument followed by space
                let new_search = format!("{} ", rewrite_cmd.arg);
                self.search_text = new_search;
                self.recompute_filtered_commands();
            }
        }
    }
    
    /// Get display commands with submenu information
    /// Returns (commands_to_display, is_in_submenu, menu_prefix, inside_count)
    pub fn get_display_commands(&self) -> (Vec<Command>, bool, Option<String>, usize) {
        use core::commands::get_current_submenu_prefix_from_commands;
        
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
    
    /// Recompute filtered commands based on current search
    fn recompute_filtered_commands(&mut self) {
        use core::commands::get_display_commands;
        
        let total_limit = self.config.popup_settings.max_rows * self.config.popup_settings.max_columns;
        self.filtered_commands = get_display_commands(&self.commands, &self.search_text, &self.config, total_limit);
    }
}

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