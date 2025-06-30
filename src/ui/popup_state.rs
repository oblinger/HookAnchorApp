//! Popup state management separated from UI rendering
//! 
//! This module handles the core state and business logic of the popup window,
//! separated from egui-specific rendering code.

use crate::core::commands::{Command, filter_commands, merge_similar_commands};
use crate::core::config::Config;
use crate::core::state::AppState;
use crate::ui::layout::{DisplayLayout, Selection, Direction};

/// Core popup state separated from UI concerns
pub struct PopupState {
    /// All available commands
    pub commands: Vec<Command>,
    /// Current search text
    pub search_text: String,
    /// Commands that match current search
    pub filtered_commands: Vec<Command>,
    /// Display layout and navigation
    pub display_layout: DisplayLayout,
    /// Current selection
    pub selection: Selection,
    /// Application configuration
    pub config: Config,
    /// Persistent application state
    pub app_state: AppState,
}

impl PopupState {
    /// Create new popup state
    pub fn new(commands: Vec<Command>, config: Config, app_state: AppState) -> Self {
        let filtered_commands = Vec::new();
        let display_layout = DisplayLayout::new(Vec::new(), &config);
        let selection = Selection::new();
        
        PopupState {
            commands,
            search_text: String::new(),
            filtered_commands,
            display_layout,
            selection,
            config,
            app_state,
        }
    }
    
    /// Update search text and recompute filtered commands
    pub fn update_search(&mut self, new_search: String) {
        self.search_text = new_search;
        self.recompute_filtered_commands();
        self.update_display_layout();
        self.selection.reset(&self.display_layout);
    }
    
    /// Navigate selection in the given direction
    pub fn navigate(&mut self, direction: Direction) -> bool {
        self.selection.navigate(direction, &self.display_layout)
    }
    
    /// Get the currently selected command
    pub fn get_selected_command(&self) -> Option<&Command> {
        self.selection.get_command(&self.display_layout)
    }
    
    /// Get the command at a specific visual position
    pub fn get_command_at_position(&self, row: usize, col: usize) -> Option<&Command> {
        self.display_layout.get_command_at_position(row, col)
    }
    
    /// Set selection to a specific visual position
    pub fn set_selection_to_position(&mut self, row: usize, col: usize) -> bool {
        if let Some(index) = self.display_layout.visual_to_index(row, col) {
            self.selection = Selection::from_index(index, &self.display_layout);
            true
        } else {
            false
        }
    }
    
    /// Get display layout dimensions
    pub fn get_layout_dimensions(&self) -> (usize, usize) {
        self.display_layout.get_dimensions()
    }
    
    /// Check if a command should be displayed
    pub fn should_display_command(&self, cmd: &Command) -> bool {
        // Don't display separator commands in the UI
        cmd.action != "separator"
    }
    
    /// Get submenu information if in submenu mode
    pub fn get_submenu_info(&self) -> Option<&crate::ui::layout::SubmenuInfo> {
        self.display_layout.submenu_info.as_ref()
    }
    
    /// Get display commands for rendering (excludes separators, handles columns)
    pub fn get_display_commands_for_rendering(&self) -> Vec<&Command> {
        self.display_layout.commands.iter()
            .filter(|cmd| self.should_display_command(cmd))
            .collect()
    }
    
    /// Recompute filtered commands based on current search
    fn recompute_filtered_commands(&mut self) {
        if self.search_text.is_empty() {
            self.filtered_commands = Vec::new();
            return;
        }
        
        // Apply filtering
        let total_limit = self.config.popup_settings.max_rows * self.config.popup_settings.max_columns;
        let filtered = filter_commands(&self.commands, &self.search_text, total_limit * 2, false);
        
        // Check for submenu mode first
        use crate::core::commands::{get_current_submenu_prefix_from_commands, get_command_prefix};
        
        let final_commands = if let Some(menu_prefix) = get_current_submenu_prefix_from_commands(&filtered, &self.search_text, &self.config.popup_settings.word_separators) {
            // SUBMENU MODE: Split first, then merge and sort each list separately
            let mut inside_commands = Vec::new();
            let mut outside_commands = Vec::new();
            
            // Split into inside and outside lists
            for cmd in filtered {
                if cmd.action == "separator" {
                    continue; // Skip any existing separators
                }
                
                let cmd_prefix = get_command_prefix(&cmd.command, &self.config.popup_settings.word_separators);
                if cmd_prefix.eq_ignore_ascii_case(&menu_prefix) {
                    inside_commands.push(cmd);
                } else {
                    outside_commands.push(cmd);
                }
            }
            
            // Apply merging to each list separately if enabled
            if self.config.popup_settings.merge_similar {
                inside_commands = merge_similar_commands(inside_commands, &self.config);
                outside_commands = merge_similar_commands(outside_commands, &self.config);
            }
            
            // Combine: inside + separator + outside
            let mut result = inside_commands;
            
            if !result.is_empty() && !outside_commands.is_empty() {
                // Add single separator between inside and outside
                result.push(Command {
                    group: String::new(),
                    command: "---".to_string(),
                    action: "separator".to_string(),
                    arg: String::new(),
                    flags: String::new(),
                    full_line: String::new(),
                });
            }
            
            result.extend(outside_commands);
            result
            
        } else {
            // NORMAL MODE: Don't merge or create separators when not in submenu mode
            // Just return the filtered commands sorted by our matching logic
            // NO MERGING
            filtered
        };
        
        // Limit final results and store
        let mut limited_commands = final_commands;
        limited_commands.truncate(total_limit);
        self.filtered_commands = limited_commands;
    }
    
    /// Update display layout based on current filtered commands
    fn update_display_layout(&mut self) {
        self.display_layout = DisplayLayout::new(self.filtered_commands.clone(), &self.config);
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
    }
    
    /// Get saved window position
    pub fn get_window_position(&self) -> Option<(f32, f32)> {
        self.app_state.window_position
    }
    
    /// Navigate horizontally (left/right) in multi-column layout
    pub fn navigate_horizontal(&mut self, direction: i32) -> bool {
        self.navigate(if direction > 0 { Direction::Right } else { Direction::Left })
    }
    
    /// Navigate vertically (up/down)
    pub fn navigate_vertical(&mut self, direction: i32) -> bool {
        self.navigate(if direction > 0 { Direction::Down } else { Direction::Up })
    }
    
    /// Check if a command is a separator
    pub fn is_separator_command(cmd: &Command) -> bool {
        cmd.action == "separator" && (cmd.command == "---" || cmd.command.ends_with("---"))
    }
    
    /// Check and apply alias if search text matches an alias command
    pub fn check_and_apply_alias(&mut self) {
        // Look for an exact match with an alias command
        if let Some(alias_cmd) = self.commands.iter().find(|cmd| 
            cmd.action == "alias" && cmd.command.to_lowercase() == self.search_text.to_lowercase()
        ) {
            // Replace search text with the alias argument and update
            let new_search = alias_cmd.arg.clone();
            self.update_search(new_search);
        }
    }
    
    /// Get display commands with submenu information
    /// Returns (commands_to_display, is_in_submenu, menu_prefix, inside_count)
    pub fn get_display_commands(&self) -> (Vec<Command>, bool, Option<String>, usize) {
        use crate::core::commands::get_current_submenu_prefix_from_commands;
        
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
}