//! Popup state management separated from UI rendering
//! 
//! This module handles the core state and business logic of the popup window,
//! separated from egui-specific rendering code.

use crate::core::Command;
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
    /// Whether we're currently in submenu mode
    pub is_in_submenu: bool,
    /// Submenu information: (original_command, resolved_command) 
    pub submenu_info: Option<(Command, Command)>,
    /// Number of commands in the submenu (before separator)
    pub submenu_count: usize,
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
            is_in_submenu: false,
            submenu_info: None,
            submenu_count: 0,
        }
    }
    
    /// Create minimal popup state for early UI display
    /// Uses default config and empty commands to show UI immediately
    pub fn new_minimal() -> Self {
        let config = Config::default();
        let app_state = AppState::default();
        let filtered_commands = Vec::new();
        let display_layout = DisplayLayout::new(Vec::new(), &config);
        let selection = Selection::new();
        
        PopupState {
            commands: Vec::new(),
            search_text: String::new(),
            filtered_commands,
            display_layout,
            selection,
            config,
            app_state,
            is_in_submenu: false,
            submenu_info: None,
            submenu_count: 0,
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
    
    /// Get a reference to all commands
    pub fn get_commands(&self) -> &[Command] {
        &self.commands
    }
    
    /// Update the command list (used for deferred scanner updates)
    pub fn set_commands(&mut self, commands: Vec<Command>) {
        self.commands = commands;
        self.recompute_filtered_commands();
        self.update_display_layout();
        self.selection.reset(&self.display_layout);
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
    
    /// Recompute filtered commands based on current search using new approach
    fn recompute_filtered_commands(&mut self) {
        use crate::core::get_new_display_commands;
        
        let sys_data = crate::core::sys_data::get_sys_data();
        let (display_commands, is_submenu, submenu_info, submenu_count) = 
            get_new_display_commands(&self.search_text, &sys_data.commands, &sys_data.patches);
        
        // Apply max limit
        let total_limit = self.config.popup_settings.max_rows * self.config.popup_settings.max_columns;
        let mut limited_commands = display_commands;
        limited_commands.truncate(total_limit);
        
        self.filtered_commands = limited_commands;
        
        // Store submenu information for get_display_commands
        self.is_in_submenu = is_submenu;
        self.submenu_info = submenu_info;
        self.submenu_count = submenu_count;
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
    
    /// Check and apply alias if search text matches - removed since rewrite action is deprecated
    pub fn check_and_apply_alias(&mut self) {
        // This functionality has been removed along with the rewrite action type
    }
    
    /// Get display commands with submenu information
    /// Returns (commands_to_display, is_in_submenu, menu_prefix, inside_count)
    pub fn get_display_commands(&self) -> (Vec<Command>, bool, Option<String>, usize) {
        if self.is_in_submenu {
            // Extract the resolved command name for the menu prefix
            let menu_prefix = if let Some((_, resolved_command)) = &self.submenu_info {
                Some(resolved_command.command.clone())
            } else {
                None
            };
            
            (self.filtered_commands.clone(), true, menu_prefix, self.submenu_count)
        } else {
            (self.filtered_commands.clone(), false, None, 0)
        }
    }
    
    /// Get submenu information for breadcrumb display and prefix trimming
    /// Returns (original_command, resolved_command) if in submenu mode
    pub fn get_submenu_command_info(&self) -> Option<&(Command, Command)> {
        if self.is_in_submenu {
            self.submenu_info.as_ref()
        } else {
            None
        }
    }
}