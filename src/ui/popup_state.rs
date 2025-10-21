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
    /// Whether we're currently in prefix menu mode
    pub is_in_prefix_menu: bool,
    /// Prefix menu information: (original_command, resolved_command)
    pub prefix_menu_info: Option<(Command, Command)>,
    /// Number of commands in the prefix menu (before separator)
    pub prefix_menu_count: usize,
    /// Whether to show files from anchor folder in prefix menu
    pub show_files: bool,
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
            show_files: app_state.show_files,  // Load from persistent state
            app_state,
            is_in_prefix_menu: false,
            prefix_menu_info: None,
            prefix_menu_count: 0,
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
            show_files: app_state.show_files,  // Load from persistent state
            app_state,
            is_in_prefix_menu: false,
            prefix_menu_info: None,
            prefix_menu_count: 0,
        }
    }
    
    /// Update search text and recompute filtered commands
    pub fn update_search(&mut self, new_search: String) {

        self.search_text = new_search;
        let was_reloaded = self.recompute_filtered_commands();
        self.update_display_layout();
        self.selection.reset(&self.display_layout);
        
        if was_reloaded {
            crate::utils::detailed_log("POPUP_REFRESH", "Commands were reloaded - display fully refreshed");
        }
    }

    /// Get the current search text (with ghost input fallback)
    pub fn search_text(&self) -> String {
        // If search text is empty, try to use anchor name
        if self.search_text.is_empty() {
            if let (Some(anchor_name), Some(anchor_timestamp)) = (&self.app_state.anchor_name, self.app_state.anchor_timestamp) {
                let current_time = chrono::Local::now().timestamp();
                let seconds_since_anchor = current_time - anchor_timestamp;
                let anchor_timeout = self.config.popup_settings.ghost_timeout_seconds.unwrap_or(180) as i64;

                if seconds_since_anchor < anchor_timeout {
                    // crate::utils::detailed_log("ANCHOR_INPUT", &format!("Using anchor name '{}' ({}s ago)", anchor_name, seconds_since_anchor));
                    return anchor_name.clone();
                } else {
                    // crate::utils::detailed_log("ANCHOR_INPUT", &format!("Anchor name '{}' expired ({}s ago, timeout: {}s)", anchor_name, seconds_since_anchor, anchor_timeout));
                }
            }
        }

        self.search_text.clone()
    }

    /// Get the raw search text (without ghost input fallback)
    /// Used by UI components that need the actual input box text
    pub fn raw_search_text(&self) -> &str {
        &self.search_text
    }

    /// Get mutable access to search text for UI binding only
    /// WARNING: Direct changes bypass update_search logic
    pub fn search_text_mut(&mut self) -> &mut String {
        &mut self.search_text
    }

    /// Set search text during initialization without triggering updates
    /// Used only during construction/initialization
    pub fn set_search_text_during_init(&mut self, text: String) {
        self.search_text = text;
    }

    /// Check if commands were modified and refresh if needed
    pub fn check_for_reload(&mut self) -> bool {
        let was_reloaded = self.recompute_filtered_commands();
        if was_reloaded {
            self.update_display_layout();
            self.selection.reset(&self.display_layout);
            crate::utils::detailed_log("POPUP_REFRESH", "Commands were reloaded - display refreshed");
        }
        was_reloaded
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
    
    /// Get prefix menu information if in prefix menu mode
    pub fn get_prefix_menu_info(&self) -> Option<&crate::ui::layout::PrefixMenuInfo> {
        self.display_layout.prefix_menu_info.as_ref()
    }
    
    /// Get display commands for rendering (excludes separators, handles columns)
    pub fn get_display_commands_for_rendering(&self) -> Vec<&Command> {
        self.display_layout.commands.iter()
            .filter(|cmd| self.should_display_command(cmd))
            .collect()
    }
    
    /// Recompute filtered commands based on current search using new approach
    fn recompute_filtered_commands(&mut self) -> bool {
        use crate::core::get_new_display_commands;

        let (sys_data, was_reloaded) = crate::core::sys_data::get_sys_data();
        if was_reloaded {
            crate::utils::detailed_log("POPUP_REFRESH", "Commands were reloaded - rebuilding search results");
        }
        let (display_commands, is_prefix_menu, prefix_menu_info, prefix_menu_count) =
            get_new_display_commands(&self.search_text(), &sys_data.commands, &sys_data.patches);

        // Apply max limit
        let total_limit = self.config.popup_settings.max_rows * self.config.popup_settings.max_columns;
        let mut limited_commands = display_commands;
        limited_commands.truncate(total_limit);

        self.filtered_commands = limited_commands;

        // Store prefix menu information for get_display_commands
        self.is_in_prefix_menu = is_prefix_menu;
        self.prefix_menu_info = prefix_menu_info;
        self.prefix_menu_count = prefix_menu_count;
        
        // Return whether we reloaded
        was_reloaded
    }
    
    /// Update display layout based on current filtered commands
    /// Note: This calculates layout based on filtered_commands only (without folder files).
    /// The actual rendering calculates layout dynamically from get_display_commands() which
    /// includes folder files, ensuring window sizing and rendering use the same command count.
    fn update_display_layout(&mut self) {
        self.display_layout = DisplayLayout::new(self.filtered_commands.clone(), &self.config);
    }
    
    /// Get hint text for the search box
    pub fn get_hint_text(&self) -> String {
        // Check for anchor name first
        if let (Some(anchor_name), Some(anchor_timestamp)) = (&self.app_state.anchor_name, self.app_state.anchor_timestamp) {
            let current_time = chrono::Local::now().timestamp();
            let seconds_since_anchor = current_time - anchor_timestamp;

            // Get anchor timeout from config (default 180 seconds)
            let anchor_timeout = self.config.popup_settings.ghost_timeout_seconds.unwrap_or(180) as i64;

            if seconds_since_anchor < anchor_timeout {
                // crate::utils::detailed_log("ANCHOR_DISPLAY", &format!("📱 Displaying anchor name: '{}'", anchor_name));
                return anchor_name.clone();
            } else {
                // crate::utils::detailed_log("ANCHOR_DISPLAY", &format!("👻 Anchor name '{}' expired ({}s ago, timeout: {}s)", anchor_name, seconds_since_anchor, anchor_timeout));
            }
        }

        let base_text = "Type to search commands...";

        // Add build time if recent (within 10 minutes)
        if let Some(build_time) = self.app_state.build_time {
            let current_time = chrono::Local::now().timestamp();
            let seconds_since_build = current_time - build_time;

            if seconds_since_build < 600 { // 10 minutes
                let hint_text = format!("{} {}s", base_text, seconds_since_build);
                // crate::utils::detailed_log("GHOST_DISPLAY", &format!("💡 Displaying build time hint: '{}'", hint_text));
                return hint_text;
            }
        }

        // crate::utils::detailed_log("GHOST_DISPLAY", &format!("📝 Displaying default hint: '{}'", base_text));
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
    
    /// Get display commands with prefix menu information
    /// Returns (commands_to_display, is_in_prefix_menu, menu_prefix, inside_count)
    pub fn get_display_commands(&self) -> (Vec<Command>, bool, Option<String>, usize) {
        if self.is_in_prefix_menu {
            // Extract the resolved command name for the menu prefix
            let menu_prefix = if let Some((_, resolved_command)) = &self.prefix_menu_info {
                Some(resolved_command.command.clone())
            } else {
                None
            };

            (self.filtered_commands.clone(), true, menu_prefix, self.prefix_menu_count)
        } else {
            (self.filtered_commands.clone(), false, None, 0)
        }
    }

    /// Get display commands with file entries if show_files is enabled
    /// This is called from AnchorSelector to get the final display list
    /// Returns (commands_to_display, is_in_prefix_menu, menu_prefix, inside_count)
    pub fn get_display_commands_with_files(&self, folder_files: Vec<Command>) -> (Vec<Command>, bool, Option<String>, usize) {
        if self.is_in_prefix_menu && self.show_files && !folder_files.is_empty() {
            let mut commands = self.filtered_commands.clone();

            // Find the position of the "============" separator
            let separator_pos = commands.iter().position(|cmd|
                cmd.action == "separator" && cmd.command.starts_with("=")
            );

            if let Some(pos) = separator_pos {
                // Create dash separator with alternating dashes and spaces for visibility
                let dash_separator = Command {
                    patch: String::new(),
                    command: "- - - - - - - - - - - -".to_string(),
                    action: "separator".to_string(),
                    arg: String::new(),
                    flags: String::new(),
                    last_update: 0,
                    file_size: None,
                };

                // Insert dash separator before "============"
                commands.insert(pos, dash_separator);

                // Insert file entries after dash separator
                for (i, file_cmd) in folder_files.into_iter().enumerate() {
                    commands.insert(pos + 1 + i, file_cmd);
                }
            }

            // Extract menu prefix
            let menu_prefix = if let Some((_, resolved_command)) = &self.prefix_menu_info {
                Some(resolved_command.command.clone())
            } else {
                None
            };

            (commands, true, menu_prefix, self.prefix_menu_count)
        } else {
            // No files to add, return normal display commands
            self.get_display_commands()
        }
    }
    
    /// Get prefix menu information for breadcrumb display and prefix trimming
    /// Returns (original_command, resolved_command) if in prefix menu mode
    pub fn get_prefix_menu_command_info(&self) -> Option<&(Command, Command)> {
        if self.is_in_prefix_menu {
            self.prefix_menu_info.as_ref()
        } else {
            None
        }
    }
}