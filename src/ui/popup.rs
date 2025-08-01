//! Popup interface module for the Anchor Selector
//! 
//! This module contains the AnchorSelector struct and all popup-specific UI logic.

use eframe::egui::{self, IconData};
use std::process;
use std::sync::OnceLock;
use crate::{
    Command, execute_via_server, 
    Config, load_state, save_state, scanner, grabber
};
use crate::core::config::{load_config_with_error, ConfigResult};
use super::{PopupState, LayoutArrangement};

use super::command_editor::{CommandEditor, CommandEditorResult};
use super::dialog::Dialog;

/// Window sizing modes for different UI states
#[derive(Clone, Debug, PartialEq)]
enum WindowSizeMode {
    /// Normal popup mode - sized for command list
    Normal,
    /// Command editor is active - sized for editor
    Editor,
    /// Dialog is active - sized for dialog content
    Dialog,
}

/// Main application state for the Anchor Selector popup window
pub struct AnchorSelector {
    /// Core popup state (business logic)
    popup_state: PopupState,
    /// Last saved window position for change detection
    last_saved_position: Option<egui::Pos2>,
    /// Whether window position has been set on startup
    position_set: bool,
    /// Command editor dialog state
    command_editor: CommandEditor,
    /// Dialog system for user input
    dialog: Dialog,
    /// Current window sizing mode
    window_size_mode: WindowSizeMode,
    /// Track the last size for each mode to avoid unnecessary resize commands
    last_normal_size: Option<egui::Vec2>,
    last_editor_size: Option<egui::Vec2>,
    last_dialog_size: Option<egui::Vec2>,
    /// Flag to track if scanner check is pending
    scanner_check_pending: bool,
    /// Grabber countdown state (None = not active, Some(n) = countdown from n)
    grabber_countdown: Option<u8>,
    /// Timestamp for countdown timing
    countdown_last_update: Option<std::time::Instant>,
    /// Track if focus has been successfully set on the input field
    focus_set: bool,
    /// Frame counter to track how many frames have passed since startup
    frame_count: u32,
    /// Track if window activation has been attempted
    window_activated: bool,
    /// Config error to show in dialog if config loading failed
    config_error: Option<String>,
    /// Last user interaction time for idle timeout
    last_interaction_time: std::time::Instant,
    /// Loading state for deferred initialization
    loading_state: LoadingState,
    /// Buffer for keyboard input captured before full initialization
    pre_init_input_buffer: String,
}

/// Loading state for deferred initialization
#[derive(Clone, Debug, PartialEq)]
enum LoadingState {
    /// UI is shown but data not loaded yet
    NotLoaded,
    /// Currently loading data in background
    Loading,
    /// Data loading completed
    Loaded,
    /// Error occurred during loading
    #[allow(dead_code)]
    Error(String),
}

impl AnchorSelector {
    // =============================================================================
    // Window Size Management
    // =============================================================================
    
    /// Determine the appropriate window size mode based on current UI state
    fn determine_window_size_mode(&self) -> WindowSizeMode {
        if self.dialog.visible {
            WindowSizeMode::Dialog
        } else if self.command_editor.visible {
            WindowSizeMode::Editor
        } else {
            WindowSizeMode::Normal
        }
    }
    
    /// Calculate the required window size for normal mode (command list)
    fn calculate_normal_size(&self, command_count: usize) -> egui::Vec2 {
        let base_width = 500.0;
        let base_height = 120.0; // Base height for input field
        
        // Add height for command list (approximately 30 pixels per command)
        let command_list_height = if command_count > 0 {
            (command_count.min(12) as f32) * 30.0 // Cap at 12 visible commands
        } else {
            0.0
        };
        
        // Add extra height if grabber countdown is active
        let countdown_height = if self.grabber_countdown.is_some() { 60.0 } else { 0.0 };
        
        egui::vec2(base_width, base_height + command_list_height + countdown_height)
    }
    
    /// Calculate the required window size for editor mode
    fn calculate_editor_size(&self) -> egui::Vec2 {
        // Command editor needs space for all input fields
        egui::vec2(600.0, 400.0) // Fixed size for command editor
    }
    
    /// Calculate the required window size for dialog mode
    fn calculate_dialog_size(&self) -> egui::Vec2 {
        let (dialog_width, dialog_height) = self.dialog.calculate_required_size();
        let required_width = dialog_width.max(500.0); // Minimum width
        let required_height = dialog_height + 60.0; // Add padding
        egui::vec2(required_width, required_height)
    }
    
    /// Update window size based on current mode, only if mode or size changed
    fn update_window_size(&mut self, ctx: &egui::Context) {
        let new_mode = self.determine_window_size_mode();
        
        // Calculate the required size for the new mode
        let required_size = match new_mode {
            WindowSizeMode::Normal => {
                let (display_commands, _, _, _) = self.get_display_commands();
                self.calculate_normal_size(display_commands.len())
            }
            WindowSizeMode::Editor => self.calculate_editor_size(),
            WindowSizeMode::Dialog => self.calculate_dialog_size(),
        };
        
        // Check if we need to update the window size
        let should_resize = match (&self.window_size_mode, &new_mode) {
            (WindowSizeMode::Normal, WindowSizeMode::Normal) => {
                // Check if normal mode size changed
                self.last_normal_size.map_or(true, |last| (last - required_size).length() > 1.0)
            }
            (WindowSizeMode::Editor, WindowSizeMode::Editor) => {
                // Check if editor mode size changed
                self.last_editor_size.map_or(true, |last| (last - required_size).length() > 1.0)
            }
            (WindowSizeMode::Dialog, WindowSizeMode::Dialog) => {
                // Check if dialog mode size changed
                self.last_dialog_size.map_or(true, |last| (last - required_size).length() > 1.0)
            }
            _ => true, // Mode changed, definitely resize
        };
        
        if should_resize {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(required_size));
            
            // Update the mode and cache the size
            self.window_size_mode = new_mode.clone();
            match new_mode {
                WindowSizeMode::Normal => self.last_normal_size = Some(required_size),
                WindowSizeMode::Editor => self.last_editor_size = Some(required_size),
                WindowSizeMode::Dialog => self.last_dialog_size = Some(required_size),
            }
            
            crate::utils::debug_log("WINDOW_SIZE", &format!(
                "Window resized to {:?} mode: {}x{}", 
                new_mode, required_size.x, required_size.y
            ));
        }
    }

    // =============================================================================
    // Scanner Management
    // =============================================================================
    
    /// Perform scanner check before exiting to update commands for next launch
    fn perform_exit_scanner_check(&mut self) {
        if self.scanner_check_pending {
            self.scanner_check_pending = false;
            let updated_commands = scanner::file_scan_check(self.popup_state.get_commands().to_vec());
            if updated_commands.len() != self.popup_state.get_commands().len() {
                // Commands have changed, update the popup state for next time
                self.popup_state.set_commands(updated_commands);
                crate::utils::debug_log("SCANNER", "Updated commands on exit");
            }
        }
    }

    // =============================================================================
    // Centralized Key Handling
    // =============================================================================
    
    /// Check if a key press would generate the given text
    fn key_generates_text(key: &egui::Key, text: &str) -> bool {
        match key {
            egui::Key::Equals => text == "=",
            egui::Key::Plus => text == "+",
            egui::Key::Minus => text == "-",
            egui::Key::Backslash => text == "\\",
            egui::Key::Slash => text == "/",
            egui::Key::Semicolon => text == ";",
            egui::Key::Backtick => text == "`",
            egui::Key::Space => text == " ",
            egui::Key::Comma => text == ",",
            egui::Key::Period => text == ".",
            egui::Key::A => text == "a" || text == "A",
            egui::Key::B => text == "b" || text == "B",
            egui::Key::C => text == "c" || text == "C",
            egui::Key::D => text == "d" || text == "D",
            egui::Key::E => text == "e" || text == "E",
            egui::Key::F => text == "f" || text == "F",
            egui::Key::G => text == "g" || text == "G",
            egui::Key::H => text == "h" || text == "H",
            egui::Key::I => text == "i" || text == "I",
            egui::Key::J => text == "j" || text == "J",
            egui::Key::K => text == "k" || text == "K",
            egui::Key::L => text == "l" || text == "L",
            egui::Key::M => text == "m" || text == "M",
            egui::Key::N => text == "n" || text == "N",
            egui::Key::O => text == "o" || text == "O",
            egui::Key::P => text == "p" || text == "P",
            egui::Key::Q => text == "q" || text == "Q",
            egui::Key::R => text == "r" || text == "R",
            egui::Key::S => text == "s" || text == "S",
            egui::Key::T => text == "t" || text == "T",
            egui::Key::U => text == "u" || text == "U",
            egui::Key::V => text == "v" || text == "V",
            egui::Key::W => text == "w" || text == "W",
            egui::Key::X => text == "x" || text == "X",
            egui::Key::Y => text == "y" || text == "Y",
            egui::Key::Z => text == "z" || text == "Z",
            egui::Key::Num0 => text == "0",
            egui::Key::Num1 => text == "1",
            egui::Key::Num2 => text == "2",
            egui::Key::Num3 => text == "3",
            egui::Key::Num4 => text == "4",
            egui::Key::Num5 => text == "5",
            egui::Key::Num6 => text == "6",
            egui::Key::Num7 => text == "7",
            egui::Key::Num8 => text == "8",
            egui::Key::Num9 => text == "9",
            // Arrow keys, Enter, Escape, etc. don't generate normal text
            _ => false,
        }
    }
    
    /// Centralized key handling for the main popup interface
    /// Returns true if any keys were processed
    fn handle_popup_keys(&mut self, ctx: &egui::Context, editor_handled_escape: bool) -> bool {
        let mut keys_processed = false;
        
        // If still loading, only handle escape key
        if self.loading_state != LoadingState::Loaded {
            ctx.input_mut(|input| {
                // Only check for escape during loading
                for event in &input.events {
                    if let egui::Event::Key { key: egui::Key::Escape, pressed: true, .. } = event {
                        self.perform_exit_scanner_check();
                        std::process::exit(0);
                    }
                }
            });
            return false; // No other keys processed during loading
        }
        
        ctx.input_mut(|input| {
            // Special handling for escape key - process it first and remove it completely
            let mut escape_pressed = false;
            input.events.retain(|event| {
                if let egui::Event::Key { key: egui::Key::Escape, pressed: true, .. } = event {
                    escape_pressed = true;
                    false // Remove escape key event completely
                } else {
                    true
                }
            });
            
            // Handle escape key immediately if it was pressed
            if escape_pressed {
                // Only exit if the editor didn't already handle the escape key
                if !self.command_editor.visible && !editor_handled_escape {
                    self.perform_exit_scanner_check();
                    std::process::exit(0);
                }
            }
            
            // Collect actions to perform to avoid borrowing conflicts
            let mut actions_to_perform = Vec::new();
            let mut consumed_keys = Vec::new();
            let mut consumed_text = Vec::new();
            
            // Check for special text events that should trigger actions
            for event in &input.events {
                if let egui::Event::Text(text) = event {
                    match text.as_str() {
                        ">" => {
                            actions_to_perform.push("add_alias");
                            consumed_text.push(text.clone());
                            keys_processed = true;
                        },
                        _ => {}
                    }
                }
            }
            
            // Check for special uninstall command
            let search_text_lower = self.popup_state.search_text.to_lowercase();
            if search_text_lower == "uninstall hookanchor" || search_text_lower == "!uninstall" {
                actions_to_perform.push("uninstall_app");
                // Clear the search text to avoid confusion
                self.popup_state.update_search(String::new());
                keys_processed = true;
            }
            
            // Check each pressed key against configured keybindings
            {
                let config = &self.popup_state.config;
                
                for event in &input.events {
                    if let egui::Event::Key { key, pressed, modifiers, .. } = event {
                        if *pressed {
                            let key_name = format!("{:?}", key);
                            
                            // Build key string with modifiers
                            let mut key_string = String::new();
                            if modifiers.ctrl {
                                key_string.push_str("ctrl+");
                            }
                            if modifiers.alt {
                                key_string.push_str("alt+");
                            }
                            if modifiers.shift {
                                key_string.push_str("shift+");
                            }
                            if modifiers.command {
                                key_string.push_str("cmd+");
                            }
                            key_string.push_str(&key_name.to_lowercase());
                            
                            // Check what action (if any) is bound to this key combination
                            if let Some(action) = config.get_action_for_key(&key_string).or_else(|| {
                                // Fallback to check the key name without modifiers
                                config.get_action_for_key(&key_name)
                            }) {
                                // Skip escape key since we handle it specially above
                                if key_name == "Escape" {
                                    continue;
                                }
                                
                                // Skip delete_command and cancel_editor actions - these are handled by the command editor
                                if action == "delete_command" || action == "cancel_editor" {
                                    continue;
                                }
                                
                                // Always consume the key if it's bound to any action
                                consumed_keys.push(*key);
                                keys_processed = true;
                                
                                match action {
                                    "navigate_up" => actions_to_perform.push("navigate_up"),
                                    "navigate_down" => actions_to_perform.push("navigate_down"),
                                    "navigate_left" => actions_to_perform.push("navigate_left"),
                                    "navigate_right" => actions_to_perform.push("navigate_right"),
                                    "force_rebuild" => actions_to_perform.push("force_rebuild"),
                                    "start_grabber" => actions_to_perform.push("start_grabber"),
                                    "show_folder" => actions_to_perform.push("show_folder"),
                                    "exit_app" => actions_to_perform.push("exit_app"),
                                    "execute_command" => actions_to_perform.push("execute_command"),
                                    "open_editor" => actions_to_perform.push("open_editor"),
                                    "add_alias" => actions_to_perform.push("add_alias"),
                                    "edit_active_command" => actions_to_perform.push("edit_active_command"),
                                    "link_to_clipboard" => actions_to_perform.push("link_to_clipboard"),
                                    "uninstall_app" => actions_to_perform.push("uninstall_app"),
                                    _ => {} // Unknown action - still consume the key
                                }
                            }
                        }
                    }
                }
            }
            
            // Remove all consumed key events AND their text equivalents
            input.events.retain(|event| {
                match event {
                    egui::Event::Key { key, .. } => !consumed_keys.contains(key),
                    egui::Event::Text(text) => {
                        // Filter out text that was explicitly consumed
                        if consumed_text.contains(text) {
                            return false;
                        }
                        // Filter out text that corresponds to consumed keys
                        // Check if any consumed key would generate this text
                        let should_filter = consumed_keys.iter().any(|key| {
                            Self::key_generates_text(key, text)
                        });
                        !should_filter
                    },
                    _ => true
                }
            });
            
            // Clear the pressed state for all consumed keys
            for key in consumed_keys {
                input.keys_down.remove(&key);
            }
            
            
            // Execute all actions
            for action in actions_to_perform {
                self.execute_key_action(action, editor_handled_escape, ctx);
            }
        });
        
        keys_processed
    }
    
    /// Execute a specific key action
    fn execute_key_action(&mut self, action: &str, _editor_handled_escape: bool, ctx: &egui::Context) {
        match action {
            "navigate_up" => self.navigate_vertical(-1),
            "navigate_down" => self.navigate_vertical(1),
            "navigate_left" => self.navigate_horizontal(-1),
            "navigate_right" => self.navigate_horizontal(1),
            "force_rebuild" => self.trigger_rebuild(),
            "start_grabber" => self.start_grabber_countdown(ctx),
            "show_folder" => self.show_folder(),
            "exit_app" => {
                // Only exit if the command editor is not currently visible
                if !self.command_editor.visible {
                    self.perform_exit_scanner_check();
                    std::process::exit(0);
                }
            },
            "execute_command" => self.execute_selected_command(),
            "open_editor" => self.open_command_editor(),
            "add_alias" => self.handle_add_alias(),
            "edit_active_command" => self.edit_active_command(),
            "link_to_clipboard" => self.handle_link_to_clipboard(),
            "uninstall_app" => self.handle_uninstall_app(),
            _ => {}
        }
    }
    
    /// Display an error dialog to the user
    /// This is a generic function for showing errors in a popup dialog
    pub fn show_error_dialog(&mut self, error_message: &str) {
        self.dialog.show_error(error_message);
    }
    
    /// Execute the currently selected command
    fn execute_selected_command(&mut self) {
        // Log what the user actually typed with visual separator
        crate::utils::debug_log("", "=================================================================");
        crate::utils::debug_log("USER INPUT", &format!("GUI: '{}'", self.popup_state.search_text));
        
        if !self.filtered_commands().is_empty() {
            let (display_commands, _is_submenu, _menu_prefix, _inside_count) = self.get_display_commands();
            
            if self.selected_index() < display_commands.len() {
                let selected_cmd = &display_commands[self.selected_index()];
                
                // Don't execute if it's a separator
                if !PopupState::is_separator_command(selected_cmd) {
                    // Always use server-based execution for consistent environment
                    let launcher_command = if selected_cmd.arg.is_empty() {
                        selected_cmd.action.clone()
                    } else {
                        format!("{} {}", selected_cmd.action, selected_cmd.arg)
                    };
                    
                    match execute_via_server(&launcher_command, None, None, false) {
                        Ok(_) => {
                            // Command sent to server successfully
                        },
                        Err(e) => {
                            eprintln!("Error executing command via server: {:?}", e);
                            std::process::exit(1);
                        }
                    }
                    self.perform_exit_scanner_check();
                    std::process::exit(0);
                }
            }
        }
    }
    
    /// Open the command editor
    fn open_command_editor(&mut self) {
        let commands = self.commands().clone();
        let search_text = self.popup_state.search_text.clone();
        let exact_match = commands.iter().find(|cmd| 
            cmd.command.to_lowercase() == search_text.to_lowercase()
        );
        
        if let Some(matching_command) = exact_match {
            self.command_editor.edit_command(Some(matching_command), &self.popup_state.search_text);
        } else {
            self.command_editor.edit_command(None, &self.popup_state.search_text);
        }
    }
    
    /// Handle add alias command - opens command editor with alias action and last executed command as argument
    fn handle_add_alias(&mut self) {
        let state = crate::core::state::load_state();
        if let Some(last_command) = state.last_executed_command {
            let search_text = self.popup_state.search_text.clone();
            
            // Create a new command with alias action
            let alias_command = crate::Command {
                patch: String::new(),
                command: search_text.clone(),
                action: "alias".to_string(),
                arg: last_command,
                flags: String::new(),
                full_line: String::new(), // Will be reconstructed by the editor
            };
            
            // Open command editor with the pre-filled alias command
            self.command_editor.open_with_command(alias_command);
        } else {
            // If no last command available, show error dialog
            let dialog_spec = vec![
                "=Error".to_string(),
                "No last executed command available for alias creation.".to_string(),
                "!OK".to_string(),
            ];
            self.dialog.show(dialog_spec);
        }
    }
    
    fn edit_active_command(&mut self) {
        if !self.filtered_commands().is_empty() {
            let (display_commands, _is_submenu, _menu_prefix, _inside_count) = self.get_display_commands();
            
            if self.selected_index() < display_commands.len() {
                let selected_cmd = &display_commands[self.selected_index()];
                
                // Don't edit if it's a separator or a merged command
                if !PopupState::is_separator_command(selected_cmd) && selected_cmd.get_flag('M').is_none() {
                    // Edit the selected command, ignoring the search text
                    self.command_editor.edit_command(Some(selected_cmd), &selected_cmd.command);
                }
            }
        }
    }
    
    fn handle_link_to_clipboard(&mut self) {
        if !self.filtered_commands().is_empty() {
            let (display_commands, _is_submenu, _menu_prefix, _inside_count) = self.get_display_commands();
            
            if self.selected_index() < display_commands.len() {
                let selected_cmd = &display_commands[self.selected_index()];
                
                // Don't copy link if it's a separator
                if !PopupState::is_separator_command(selected_cmd) {
                    // Use launcher to execute the link_to_clipboard action
                    let command_line = format!("link_to_clipboard {}", selected_cmd.command);
                    match execute_via_server(&command_line, None, None, false) {
                        Ok(response) => {
                            if response.success {
                                crate::utils::debug_log("CLIPBOARD", &format!("Link copied for command: {}", selected_cmd.command));
                            } else {
                                crate::utils::debug_log("CLIPBOARD", &format!("Failed to copy link: {}", response.stderr));
                            }
                        },
                        Err(e) => {
                            crate::utils::debug_log("CLIPBOARD", &format!("Server communication failed: {}", e));
                        }
                    }
                }
            }
        }
    }
    
    /// Handle uninstall app request
    fn handle_uninstall_app(&mut self) {
        // Show simple warning dialog with OK/Cancel
        let spec_strings = vec![
            "=Uninstall HookAnchor".to_string(),
            "#This will remove HookAnchor app and Karabiner config.".to_string(),
            "'⚠️  Your commands and settings will be preserved.".to_string(),
            "!OK".to_string(),
            "!Cancel".to_string(),
        ];
        
        self.dialog.show(spec_strings);
    }
    
    // =============================================================================
    // Initialization
    // =============================================================================
    
    pub fn new() -> Self {
        Self::new_with_prompt("")
    }
    
    pub fn new_with_prompt(initial_prompt: &str) -> Self {
        let startup_time = std::time::Instant::now();
        crate::utils::debug_log("STARTUP_FAST", "Creating UI with minimal initialization");
        
        // Load only app state for window positioning - this is fast
        let state = load_state();
        crate::utils::debug_log("STARTUP_FAST", &format!("App state loaded in {:?}", startup_time.elapsed()));
        
        // Create minimal popup state for immediate UI display
        let mut popup_state = PopupState::new_minimal();
        popup_state.app_state = state; // Use loaded state for window position
        
        // Set initial prompt if provided
        if !initial_prompt.is_empty() {
            popup_state.search_text = initial_prompt.to_string();
        }
        
        let result = Self {
            popup_state,
            last_saved_position: None,
            position_set: false,
            command_editor: CommandEditor::new(),
            dialog: Dialog::new(),
            window_size_mode: WindowSizeMode::Normal,
            last_normal_size: None,
            last_editor_size: None,
            last_dialog_size: None,
            scanner_check_pending: true,
            grabber_countdown: None,
            countdown_last_update: None,
            focus_set: false,
            frame_count: 0,
            window_activated: false,
            config_error: None,
            last_interaction_time: std::time::Instant::now(),
            loading_state: LoadingState::NotLoaded,
            pre_init_input_buffer: initial_prompt.to_string(),
        };
        
        result
    }
    
    /// Start loading data in the background after UI is shown
    fn start_deferred_loading(&mut self) {
        if self.loading_state != LoadingState::NotLoaded {
            return; // Already loading or loaded
        }
        
        self.loading_state = LoadingState::Loading;
        let start_time = std::time::Instant::now();
        crate::utils::debug_log("DEFERRED_LOAD", "Starting deferred data loading (without scanning)");
        
        // Load commands from disk only (no filesystem scanning to avoid UI blocking)
        let commands = crate::core::commands::load_commands_raw();
        let (config, config_error) = match load_config_with_error() {
            ConfigResult::Success(config) => (config, None),
            ConfigResult::Error(error) => (crate::core::sys_data::get_config(), Some(error)),
        };
        
        // Create new popup state with loaded data but preserve app_state
        let app_state = self.popup_state.app_state.clone();
        self.popup_state = PopupState::new(commands, config, app_state);
        
        // Apply any buffered input
        if !self.pre_init_input_buffer.is_empty() {
            self.popup_state.search_text = self.pre_init_input_buffer.clone();
            self.popup_state.update_search(self.pre_init_input_buffer.clone());
            self.pre_init_input_buffer.clear();
        }
        
        // Show config error if any
        if let Some(error) = config_error {
            self.config_error = Some(error.clone());
            self.dialog.show_error(&error);
        }
        
        self.loading_state = LoadingState::Loaded;
        let elapsed = start_time.elapsed();
        crate::utils::debug_log("DEFERRED_LOAD", &format!("Data loading completed in {:?} (disk only)", elapsed));
        
        // Trigger background filesystem scan after UI is responsive
        self.trigger_background_scan();
    }
    
    /// Trigger filesystem scanning in background to avoid blocking UI
    fn trigger_background_scan(&mut self) {
        crate::utils::debug_log("DEFERRED_LOAD", "Triggering background filesystem scan");
        
        // Note: For now, we'll skip automatic scanning to keep UI responsive
        // Users can manually rescan with --rescan command if needed
        // In the future, this could spawn a background thread for scanning
        
        crate::utils::debug_log("DEFERRED_LOAD", "Background scan skipped to maintain UI responsiveness");
    }
    
    // =============================================================================
    // Helper Properties for Backward Compatibility
    // =============================================================================
    
    /// Backward compatibility: access to commands
    fn commands(&self) -> &Vec<Command> {
        &self.popup_state.commands
    }
    
    /// Backward compatibility: mutable access to commands
    fn commands_mut(&mut self) -> &mut Vec<Command> {
        &mut self.popup_state.commands
    }
    
    
    /// Backward compatibility: access to filtered commands
    fn filtered_commands(&self) -> &Vec<Command> {
        &self.popup_state.filtered_commands
    }
    
    /// Backward compatibility: access to config
    fn config(&self) -> &Config {
        &self.popup_state.config
    }
    
    
    /// Backward compatibility: access to selected index
    fn selected_index(&self) -> usize {
        self.popup_state.selection.command_index
    }
    
    /// Backward compatibility: set selected index
    fn set_selected_index(&mut self, index: usize) {
        // Directly set the command_index if valid
        if index < self.popup_state.display_layout.commands.len() {
            self.popup_state.selection.command_index = index;
            // Update visual position based on layout
            let (_rows, cols) = self.popup_state.get_layout_dimensions();
            if cols > 0 {
                let row = index / cols;
                let col = index % cols;
                self.popup_state.selection.visual_position = (row, col);
            }
        }
    }
    
    // =============================================================================
    // Build Time and Version Info
    // =============================================================================
    
    /// Generate hint text with build version info (only for first 10 minutes)
    fn get_hint_text(&self) -> String {
        self.popup_state.get_hint_text()
    }
    
    // =============================================================================
    // Command Filtering and Management
    // =============================================================================
    
    /// Check if the current search text exactly matches an alias command
    /// If so, replace the search text with the alias's argument
    fn check_and_apply_alias(&mut self) {
        self.popup_state.check_and_apply_alias();
    }
    
    
    // =============================================================================
    // Layout and Display Logic
    // =============================================================================
    
    
    /// Compute the commands to display based on current menu state
    /// Returns (commands_to_display, is_in_submenu, menu_prefix, inside_count)
    fn get_display_commands(&self) -> (Vec<Command>, bool, Option<String>, usize) {
        self.popup_state.get_display_commands()
    }
    
    // =============================================================================
    // Navigation Logic
    // =============================================================================
    
    // Navigate left/right in the multi-column layout
    fn navigate_horizontal(&mut self, direction: i32) {
        self.popup_state.navigate_horizontal(direction);
    }
    
    fn navigate_vertical(&mut self, direction: i32) {
        self.popup_state.navigate_vertical(direction);
    }
    
    /// Start the grabber countdown
    fn start_grabber_countdown(&mut self, _ctx: &egui::Context) {
        let config = crate::core::sys_data::get_config();
        let countdown_seconds = config.popup_settings.countdown_seconds.unwrap_or(5);
        let flip_focus = config.launcher_settings.as_ref().and_then(|ls| ls.flip_focus).unwrap_or(false);
        
        crate::utils::debug_log("GRAB", &format!("Starting grabber countdown from {} (flip_focus={})", 
            countdown_seconds, flip_focus));
            
        self.grabber_countdown = Some(countdown_seconds);
        self.countdown_last_update = Some(std::time::Instant::now());
        // Note: Don't call ctx.request_repaint_after() as it causes UI lockup
        // The UI update loop handles repaints automatically
    }
    
    /// Update countdown and handle grabber logic
    fn update_grabber_countdown(&mut self, ctx: &egui::Context) {
        if let Some(count) = self.grabber_countdown {
            if let Some(last_update) = self.countdown_last_update {
                let elapsed = last_update.elapsed();
                if elapsed.as_secs() >= 1 {
                    if count > 1 {
                        // Continue countdown
                        let new_count = count - 1;
                        self.grabber_countdown = Some(new_count);
                        self.countdown_last_update = Some(std::time::Instant::now());
                        
                        // Handle focus flipping during countdown if enabled
                        let config = crate::core::sys_data::get_config();
                        let flip_focus = config.launcher_settings
                            .as_ref()
                            .and_then(|ls| ls.flip_focus)
                            .unwrap_or(false);
                        
                        if flip_focus && new_count == 1 {
                            // On count 1, flip focus away (like the old behavior)
                            self.flip_focus_away();
                        }
                    } else {
                        // Countdown finished, execute grab
                        crate::utils::debug_log("GRAB", "=== STARTING GRAB EXECUTION ===");
                        let start_time = std::time::Instant::now();
                        self.execute_grab(ctx);
                        let total_time = start_time.elapsed().as_millis();
                        crate::utils::debug_log("GRAB", &format!("=== GRAB EXECUTION COMPLETED in {}ms ===", total_time));
                        self.grabber_countdown = None;
                        self.countdown_last_update = None;
                    }
                }
            }
        }
    }
    
    /// Flip focus away from HookAnchor (used when flip_focus is enabled)
    fn flip_focus_away(&self) {
        crate::utils::debug_log("GRAB", "Flipping focus away from HookAnchor");
        
        let force_finder_script = r#"
            -- First minimize HookAnchor if it exists
            try
                tell application "System Events"
                    tell application process "popup"
                        set visible to false
                    end tell
                end tell
            end try
            
            -- Force Finder to activate across all screens
            tell application "Finder"
                activate
                reopen
            end tell
            
            delay 0.2
            
            -- Make sure Finder windows are at front on all screens
            tell application "System Events"
                tell application process "Finder"
                    set frontmost to true
                    repeat with w in windows
                        try
                            set frontmost of w to true
                            set index of w to 1
                        end try
                    end repeat
                end tell
            end tell
            
            -- Additional step: bring the most recent Finder window to absolute front
            tell application "Finder"
                if (count of windows) > 0 then
                    tell window 1 to select
                end if
            end tell
        "#;
        
        if let Err(e) = std::process::Command::new("osascript")
            .arg("-e")
            .arg(force_finder_script)
            .output() {
            crate::utils::debug_log("GRAB", &format!("Failed to flip focus away: {}", e));
        }
    }
    
    /// Execute the grab operation - simplified synchronous version
    fn execute_grab(&mut self, ctx: &egui::Context) {
        let config = crate::core::sys_data::get_config();
        
        // Check if we should flip focus
        let flip_focus = config.launcher_settings
            .as_ref()
            .and_then(|ls| ls.flip_focus)
            .unwrap_or(false);
        
        crate::utils::debug_log("GRAB", &format!("execute_grab: flip_focus = {}", flip_focus));
        
        if flip_focus {
            // Focus was already flipped during countdown, just wait briefly
            crate::utils::debug_log("GRAB", "Focus already flipped, waiting 200ms before grab");
            let sleep_start = std::time::Instant::now();
            std::thread::sleep(std::time::Duration::from_millis(200));
            crate::utils::debug_log("GRAB", &format!("Sleep completed in {}ms", sleep_start.elapsed().as_millis()));
        } else {
            // User is responsible for changing focus manually during countdown
            crate::utils::debug_log("GRAB", "Manual focus mode - no pre-grab delay");
        }
        
        // Now capture from the focused application
        crate::utils::debug_log("GRAB", "Starting grabber::grab()");
        let grab_start = std::time::Instant::now();
        match grabber::grab(&config) {
            Ok(grab_result) => {
                crate::utils::debug_log("GRAB", &format!("grabber::grab() completed in {}ms", grab_start.elapsed().as_millis()));
                crate::utils::debug_log("GRAB", "Processing grab result...");
                match grab_result {
                    grabber::GrabResult::RuleMatched(rule_name, mut command) => {
                        crate::utils::debug_log("GRAB", &format!("Rule matched: {}", rule_name));
                        // Use the current search text as the command name, or default if empty
                        let command_name = if self.popup_state.search_text.trim().is_empty() {
                            format!("Grabbed {}", rule_name)
                        } else {
                            self.popup_state.search_text.clone()
                        };
                        
                        // Fill in the command with all the grabbed information
                        command.command = command_name;
                        command.full_line = format!("{} : {} {}", command.command, command.action, command.arg);
                        
                        // Open command editor with the pre-filled grabbed command
                        crate::utils::debug_log("GRAB", "Opening command editor with grabbed command");
                        self.command_editor.open_with_command(command);
                        crate::utils::debug_log("GRAB", "Command editor opened successfully");
                    }
                    grabber::GrabResult::NoRuleMatched(context) => {
                        crate::utils::debug_log("GRAB", "No rule matched - showing template dialog");
                        // Generate the template text
                        let template_text = grabber::generate_rule_template_text(&context);
                        
                        // Show template dialog using the new TextBox field type
                        let dialog_spec = vec![
                            format!("=Grabber Rule Template - {}", context.app_name),
                            format!("&{}", template_text),
                            "!OK".to_string(),
                        ];
                        
                        crate::utils::debug_log("GRAB", "Showing template dialog");
                        self.dialog.show(dialog_spec);
                        crate::utils::debug_log("GRAB", "Template dialog shown - window size will be handled automatically");
                    }
                }
                crate::utils::debug_log("GRAB", "Grab result processing completed");
            }
            Err(err) => {
                crate::utils::debug_log("GRAB", &format!("Grabber error: {}", err));
            }
        }
        
        // Check if we should restore HookAnchor visibility (only when flip_focus is enabled)
        let flip_focus_for_restore = config.launcher_settings
            .as_ref()
            .and_then(|ls| ls.flip_focus)
            .unwrap_or(false);
            
        if flip_focus_for_restore {
            crate::utils::debug_log("GRAB", "Starting HookAnchor visibility restoration (flip_focus enabled)");
            // Restore HookAnchor visibility
            let restore_script = r#"
                tell application "System Events"
                    tell application process "popup"
                        set visible to true
                        set frontmost to true
                    end tell
                end tell
            "#;
            
            let restore_start = std::time::Instant::now();
            if let Err(e) = std::process::Command::new("osascript")
                .arg("-e")
                .arg(restore_script)
                .output() {
                crate::utils::debug_log("GRAB", &format!("Failed to restore HookAnchor: {}", e));
            }
            crate::utils::debug_log("GRAB", &format!("HookAnchor visibility restoration completed in {}ms", restore_start.elapsed().as_millis()));
        } else {
            crate::utils::debug_log("GRAB", "Skipping HookAnchor visibility restoration (flip_focus disabled)");
        }
        
        // Regain focus back to anchor selector after grab operation (only if flip_focus is enabled)
        let flip_focus = config.launcher_settings
            .as_ref()
            .and_then(|ls| ls.flip_focus)
            .unwrap_or(false);
        
        crate::utils::debug_log("GRAB", &format!("flip_focus setting: {}", flip_focus));
        
        let focus_start = std::time::Instant::now();
        if flip_focus {
            // Give time for window restoration only when flip_focus is enabled
            crate::utils::debug_log("GRAB", "flip_focus enabled - regaining focus");
            let sleep_start = std::time::Instant::now();
            std::thread::sleep(std::time::Duration::from_millis(200));
            crate::utils::debug_log("GRAB", &format!("Focus restoration sleep completed in {}ms", sleep_start.elapsed().as_millis()));
            
            let regain_start = std::time::Instant::now();
            if let Err(e) = self.regain_focus() {
                crate::utils::debug_log("GRAB", &format!("Failed to regain focus: {}", e));
            } else {
                crate::utils::debug_log("GRAB", &format!("regain_focus() completed in {}ms", regain_start.elapsed().as_millis()));
            }
        } else {
            crate::utils::debug_log("GRAB", "Manual focus mode - skipping regain_focus entirely");
        }
        crate::utils::debug_log("GRAB", &format!("Focus restoration phase completed in {}ms", focus_start.elapsed().as_millis()));
    }
    
    
    /// Regain focus back to anchor selector after grab operation
    fn regain_focus(&self) -> Result<(), String> {
        crate::utils::debug_log("GRAB", "regain_focus() called - executing JavaScript");
        // Use the JavaScript function to regain focus
        let config = crate::core::sys_data::get_config();
        if let Some(functions) = &config.functions {
            if let Some(regain_focus_fn) = functions.get("regain_focus") {
                if let Some(js_code) = regain_focus_fn.as_str() {
                    match crate::js_runtime::execute_business_logic(js_code) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("Failed to execute regain_focus function: {}", e))
                    }
                } else {
                    Err("regain_focus function is not a JavaScript function".to_string())
                }
            } else {
                Err("regain_focus function not found in config".to_string())
            }
        } else {
            Err("No functions defined in config".to_string())
        }
    }
    
    /// Clean up the debug log file before starting a rescan
    
    /// Trigger rebuild: restart server and rescan filesystem (full reset)
    fn trigger_rebuild(&mut self) {
        crate::utils::debug_log("REBUILD", "=== TRIGGER_REBUILD FUNCTION CALLED ===");
        
        println!("🏗️  HookAnchor Rebuild - Clean Server Restart");
        println!("===============================================");
        
        // Step 1: Server teardown using clean top-level function
        println!("\n🔄 Step 1/3: Tearing down server...");
        match crate::command_server_management::kill_existing_server() {
            Ok(()) => {
                crate::utils::debug_log("REBUILD", "Server teardown completed successfully");
                println!("  ✅ Server teardown completed");
            }
            Err(e) => {
                crate::utils::debug_log("REBUILD", &format!("Server teardown warning: {}", e));
                println!("  ⚠️  Server teardown warning: {}", e);
            }
        }
        
        // Step 2: Server setup using clean top-level function  
        println!("\n🚀 Step 2/3: Setting up new server...");
        match crate::command_server_management::start_server_via_terminal() {
            Ok(()) => {
                crate::utils::debug_log("REBUILD", "Server setup completed successfully");
                println!("  ✅ New server started in Terminal window");
            }
            Err(e) => {
                crate::utils::debug_log("REBUILD", &format!("Server setup failed: {}", e));
                println!("  ❌ Server setup failed: {}", e);
                return;
            }
        }
        
        // Brief wait for server to be ready
        println!("  ⏳ Waiting for server to initialize...");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        
        // Step 3: Call "ha --rescan" to let the server handle the rescan
        println!("\n📁 Step 3/3: Triggering filesystem rescan via server...");
        let current_exe = std::env::current_exe().unwrap_or_else(|_| "popup".into());
        match std::process::Command::new(current_exe)
            .arg("--rescan")
            .status() {
            Ok(status) => {
                if status.success() {
                    crate::utils::debug_log("REBUILD", "Rescan via server completed successfully");
                    println!("  ✅ Filesystem rescan completed");
                    println!("\n🎉 Rebuild completed successfully!");
                } else {
                    crate::utils::debug_log("REBUILD", "Rescan via server failed");
                    println!("  ❌ Filesystem rescan failed");
                }
            }
            Err(e) => {
                crate::utils::debug_log("REBUILD", &format!("Failed to execute rescan: {}", e));
                println!("  ❌ Failed to start rescan: {}", e);
            }
        }
    }
    
    /// Recursively resolve aliases to find the final target command
    fn resolve_aliases_recursively<'a>(&self, cmd: &'a Command, all_commands: &'a [Command]) -> &'a Command {
        use crate::utils;
        
        let mut current_cmd = cmd;
        let mut visited = std::collections::HashSet::new();
        
        while current_cmd.action == "alias" {
            // Prevent infinite loops
            if visited.contains(&current_cmd.command) {
                utils::debug_log("SHOW_FOLDER", &format!("Circular alias detected for '{}', stopping resolution", current_cmd.command));
                break;
            }
            visited.insert(current_cmd.command.clone());
            
            utils::debug_log("SHOW_FOLDER", &format!("Resolving alias '{}' to target '{}'", current_cmd.command, current_cmd.arg));
            
            // Find the target command
            if let Some(target) = all_commands.iter().find(|c| c.command == current_cmd.arg) {
                utils::debug_log("SHOW_FOLDER", &format!("Alias resolved to: '{}' (action: {}, arg: {})", 
                    target.command, target.action, target.arg));
                current_cmd = target;
            } else {
                utils::debug_log("SHOW_FOLDER", &format!("Failed to resolve alias '{}' - target '{}' not found", current_cmd.command, current_cmd.arg));
                break;
            }
        }
        
        current_cmd
    }
    
    /// Show folder functionality - launches the first folder matching current search
    fn show_folder(&mut self) {
        use crate::utils;
        
        let search_text = &self.popup_state.search_text;
        utils::debug_log("SHOW_FOLDER", &format!("Triggered with search text: '{}'", search_text));
        
        // Get the current filtered commands from popup state
        let display_commands = self.popup_state.filtered_commands.clone();
        utils::debug_log("SHOW_FOLDER", &format!("Found {} filtered commands", display_commands.len()));
        
        if display_commands.is_empty() {
            utils::debug_log("SHOW_FOLDER", "No filtered commands to show folder for");
            return;
        }
        
        // Get all commands for alias resolution
        let all_commands = self.popup_state.get_commands();
        
        // Log first few commands for debugging
        for (i, cmd) in display_commands.iter().take(3).enumerate() {
            utils::debug_log("SHOW_FOLDER", &format!("  Command {}: '{}' (action: {}, arg: {})", 
                i, cmd.command, cmd.action, cmd.arg));
        }
        
        // Use the currently selected command and extract its folder
        let selected_idx = self.selected_index();
        if selected_idx >= display_commands.len() {
            utils::debug_log("SHOW_FOLDER", &format!("Selected index {} is out of bounds ({})", selected_idx, display_commands.len()));
            return;
        }
        
        let cmd = &display_commands[selected_idx];
        utils::debug_log("SHOW_FOLDER", &format!("Using selected command at index {}: '{}'", selected_idx, cmd.command));
        
        if PopupState::is_separator_command(cmd) {
            utils::debug_log("SHOW_FOLDER", &format!("Selected command is a separator: '{}'", cmd.command));
            return;
        }
            
        utils::debug_log("SHOW_FOLDER", &format!("Processing command: '{}' (action: {})", cmd.command, cmd.action));
        
        // Recursively resolve aliases
        let resolved_cmd = self.resolve_aliases_recursively(cmd, &all_commands);
        
        // Extract folder path using the command's built-in method
        let folder_path = if let Some(abs_path) = resolved_cmd.get_absolute_folder_path(&self.popup_state.config) {
            let path_str = abs_path.to_string_lossy().to_string();
            utils::debug_log("SHOW_FOLDER", &format!("Found {} action, extracted folder: {}", resolved_cmd.action, path_str));
            Some(path_str)
        } else if resolved_cmd.action == "alias" {
            // This shouldn't happen since we recursively resolved aliases above
            utils::debug_log("SHOW_FOLDER", &format!("Unresolved alias found: '{}'", resolved_cmd.command));
            None
        } else {
            utils::debug_log("SHOW_FOLDER", &format!("Command '{}' has non-folder action: {}", resolved_cmd.command, resolved_cmd.action));
            None
        };
        
        if let Some(path) = folder_path {
            utils::debug_log("SHOW_FOLDER", &format!("Attempting to launch folder: '{}'", path));
            
            // Launch the folder (popup stays open)
            match execute_via_server(&format!("folder {}", path), None, None, false) {
                Ok(response) => {
                    if response.success {
                        utils::debug_log("SHOW_FOLDER", &format!("Successfully launched folder: '{}'", path));
                    } else {
                        utils::debug_log("SHOW_FOLDER", &format!("Failed to launch folder '{}': {}", path, response.stderr));
                    }
                },
                Err(e) => {
                    utils::debug_log("SHOW_FOLDER", &format!("Server communication failed: {}", e));
                }
            }
        } else {
            utils::debug_log("SHOW_FOLDER", &format!("No folder found for selected command: '{}'", cmd.command));
        }
    }
    
}

pub fn save_window_position(pos: egui::Pos2) {
    let mut state = load_state();
    state.window_position = Some((pos.x, pos.y));
    let _ = save_state(&state);
}

pub fn load_window_position() -> Option<egui::Pos2> {
    let state = load_state();
    state.window_position.map(|(x, y)| egui::pos2(x, y))
}

pub fn get_previous_window_location(ctx: &egui::Context, window_size: egui::Vec2) -> egui::Pos2 {
    // First try to load the previous position
    if let Some(previous_pos) = load_window_position() {
        // Check if this position is visible on any current display
        if is_position_visible(previous_pos, window_size) {
            return previous_pos;
        }
    }
    
    // If no previous position or not visible, center on main display
    center_on_main_display(ctx, window_size)
}

pub fn is_position_visible(pos: egui::Pos2, window_size: egui::Vec2) -> bool {
    // Get all available monitors/displays
    // For now, we'll use a basic check - ensure the window isn't completely off-screen
    // This is a simplified version - in a full implementation you'd query actual display bounds
    
    let window_rect = egui::Rect::from_min_size(pos, window_size);
    
    // Basic bounds check - assume main display is at least 1024x768
    // In a real implementation, you'd query actual display information
    let main_display_rect = egui::Rect::from_min_size(
        egui::pos2(0.0, 0.0), 
        egui::vec2(1024.0, 768.0)
    );
    
    // Check if at least part of the window is visible
    // Allow for window to be partially off-screen but require some overlap
    let min_visible_area = window_size.x.min(window_size.y) * 0.3; // 30% of smaller dimension
    
    let intersection = main_display_rect.intersect(window_rect);
    intersection.width() * intersection.height() >= min_visible_area * min_visible_area
}

pub fn center_on_main_display(ctx: &egui::Context, window_size: egui::Vec2) -> egui::Pos2 {
    // Try to get screen size from context, fallback to reasonable defaults
    let screen_size = ctx.screen_rect().size();
    
    // If screen size is not available or seems wrong, use reasonable defaults
    let display_size = if screen_size.x > 800.0 && screen_size.y > 600.0 {
        screen_size
    } else {
        egui::vec2(1440.0, 900.0) // Common laptop resolution
    };
    
    // Center the window on the display
    egui::pos2(
        (display_size.x - window_size.x) / 2.0,
        (display_size.y - window_size.y) / 2.0
    )
}

impl eframe::App for AnchorSelector {
    /// Return transparent clear color to enable rounded corners
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Increment frame counter for initial setup only
        if self.frame_count < 20 {
            self.frame_count += 1;
        }
        
        // Start deferred loading on second frame (after UI is shown)
        if self.frame_count == 2 && self.loading_state == LoadingState::NotLoaded {
            self.start_deferred_loading();
            ctx.request_repaint(); // Ensure we update when loading completes
        }
        
        // On the first few frames, ensure the window is properly activated and positioned
        if self.frame_count <= 3 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            ctx.request_repaint(); // Ensure continuous updates during initialization
            
            // Also ensure proper window positioning on startup
            if self.frame_count == 2 && !self.position_set {
                let window_size = egui::vec2(500.0, 120.0); // Default size
                let position = center_on_main_display(ctx, window_size);
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(position));
                self.position_set = true;
            }
        }
        
        // Check if we need active updates (animations, countdowns, etc.)
        let has_input = ctx.input(|i| !i.events.is_empty() || i.pointer.any_down() || i.keys_down.len() > 0);
        let has_active_ui = self.command_editor.visible 
            || self.dialog.visible 
            || self.grabber_countdown.is_some();
        
        // Update interaction time if there's user input
        if has_input {
            self.last_interaction_time = std::time::Instant::now();
        }
        
        // Check for idle timeout
        if self.frame_count >= 10 { // Don't timeout during initial setup
            let timeout_seconds = self.popup_state.config.popup_settings.idle_timeout_seconds.unwrap_or(60);
            let idle_time = self.last_interaction_time.elapsed().as_secs();
            
            if idle_time >= timeout_seconds {
                // Close command editor if open
                if self.command_editor.visible {
                    self.command_editor.hide();
                }
                // Close dialog if open  
                if self.dialog.visible {
                    self.dialog.hide();
                }
                // Exit the application
                process::exit(0);
            }
        }
            
        // For idle state, request slower repaints to reduce CPU usage
        if !has_input && !has_active_ui && self.frame_count >= 10 {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
        
        // During countdown, ensure frequent updates for smooth 1-second timing
        if self.grabber_countdown.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }
        
        
        // Check for window focus state changes and log for debugging
        // if self.frame_count <= 20 {
        //     let has_focus = ctx.input(|i| i.focused);
        //     if self.frame_count % 5 == 0 { // Log every 5th frame for first 20 frames
        //         crate::utils::debug_log("FOCUS", &format!("Frame {}: Window focused={}, input focus_set={}", 
        //             self.frame_count, has_focus, self.focus_set));
        //     }
        // }
        
        
        // Set position on first frame after window is created
        if !self.position_set {
            // Use a reasonable default window size for positioning - the actual size will be auto-calculated
            let window_size = egui::vec2(500.0, 300.0);
            let pos = get_previous_window_location(ctx, window_size);
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
            self.position_set = true;
        }
        
        // DISABLED: Focus detection causing immediate exit
        // if self.startup_time.elapsed().as_millis() > 500 {
        //     if !ctx.input(|i| i.focused) {
        //         std::process::exit(0);
        //     }
        // }
        
        // Scanner check is now performed on exit instead of startup to avoid delays
        
        // Update grabber countdown
        self.update_grabber_countdown(ctx);
        
        // Update window size based on current UI state
        self.update_window_size(ctx);
        
        // Draw custom rounded background with heavy shadow
        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(egui::LayerId::background());
        
        // Draw multiple shadow layers for a much darker shadow effect
        // Use negative offsets to draw shadows "behind" the main rect, not extending beyond window bounds
        let shadow_offsets = [6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let shadow_alphas = [20, 30, 40, 50, 60, 70];
        
        for (offset, alpha) in shadow_offsets.iter().zip(shadow_alphas.iter()) {
            // Draw shadow inward from the main rect to avoid extending beyond transparent window bounds
            let shadow_rect = screen_rect.shrink(*offset);
            painter.rect_filled(
                shadow_rect,
                egui::Rounding::same(12.0 - *offset * 0.5), // Slightly reduce rounding for inner shadows
                egui::Color32::from_black_alpha(*alpha)
            );
        }
        
        // Draw main background
        painter.rect_filled(
            screen_rect,
            egui::Rounding::same(12.0),
            egui::Color32::from_gray(240)
        );
        
        // Update command editor dialog BEFORE the main UI so it renders as a top-level window
        let commands = self.commands().clone();
        let config = self.config().clone();
        self.command_editor.update_commands(&commands);
        let editor_result = self.command_editor.update(ctx, &config);
        let mut editor_handled_escape = false;
        
        // Check for queued errors and display them
        if crate::error_display::has_errors() {
            if let Some(error_message) = crate::error_display::take_next_error() {
                self.show_error_dialog(&error_message);
            }
        }
        
        // Update dialog system
        if self.dialog.update(ctx) {
            if let Some(result) = self.dialog.take_result() {
                // Check if the "Exit" button was clicked
                if let Some(button_text) = result.get("exit") {
                    if button_text == "Exit" {
                        self.perform_exit_scanner_check();
                        std::process::exit(0);
                    } else if button_text == "OK" {
                        // Check if this is from the uninstall dialog (has the warning about Karabiner)
                        // by looking at the dialog title or content - for simplicity, assume OK from uninstall dialog
                        
                        // Execute uninstall in background thread
                        std::thread::spawn(|| {
                            match crate::setup_assistant::uninstall_hookanchor() {
                                Ok(()) => {
                                    // Exit successfully - no stdout message
                                    std::process::exit(0);
                                },
                                Err(e) => {
                                    // Show error dialog instead of stderr
                                    // Since we're in a thread, we can't easily show dialog here
                                    // Just exit with error code
                                    std::process::exit(1);
                                }
                            }
                        });
                    }
                }
            }
        }
        // Track if command editor was just closed to restore focus
        let mut command_editor_just_closed = false;
        
        match editor_result {
            CommandEditorResult::Cancel => {
                self.command_editor.hide();
                editor_handled_escape = true;
                command_editor_just_closed = true;
            }
            CommandEditorResult::Save(_new_command, _original_command_name) => {
                // Get the save data from command editor
                let (command_to_delete, new_command) = self.command_editor.prepare_save_command();
                
                // Delete original command if needed
                if let Some(cmd_name) = command_to_delete {
                    use crate::delete_command;
                    let deleted = delete_command(&cmd_name, self.commands_mut());
                    if deleted.is_err() {
                        eprintln!("Warning: Original command '{}' not found for deletion", cmd_name);
                    }
                }
                
                // Add the new command
                use crate::{add_command, save_commands_to_file};
                let _ = add_command(new_command, self.commands_mut());
                
                // Save to file
                match save_commands_to_file(&self.commands()) {
                    Ok(_) => {
                        // Update the filtered list if we're currently filtering
                        if !self.popup_state.search_text.trim().is_empty() {
                            // Refresh the search with updated commands
                            let current_search = self.popup_state.search_text.clone();
                            self.popup_state.update_search(current_search);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error saving commands to file: {}", e);
                    }
                }
                
                self.command_editor.hide();
                command_editor_just_closed = true;
            }
            CommandEditorResult::Delete(command_name) => {
                // Delete the specified command and save to file
                use crate::{delete_command, save_commands_to_file};
                
                let deleted = delete_command(&command_name, self.commands_mut());
                if deleted.is_err() {
                    eprintln!("Warning: Command '{}' not found for deletion", command_name);
                } else {
                    // Save the updated command list back to commands.txt
                    if let Err(e) = save_commands_to_file(&self.commands()) {
                        eprintln!("Error saving commands to file after deletion: {}", e);
                    } else {
                        // Update the filtered list if we're currently filtering
                        if !self.popup_state.search_text.trim().is_empty() {
                            // Refresh the search with updated commands
                            let current_search = self.popup_state.search_text.clone();
                            self.popup_state.update_search(current_search);
                        }
                    }
                }
                self.command_editor.hide();
                command_editor_just_closed = true;
            }
            CommandEditorResult::None => {
                // Continue normal operation
            }
        }
        
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(18.0))
                    .fill(egui::Color32::TRANSPARENT) // Transparent frame background
                    .shadow(egui::Shadow::NONE) // Remove shadow to avoid artifacts
            )
            .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Handle all popup-level keyboard input in centralized location
                let _keys_processed = self.handle_popup_keys(ctx, editor_handled_escape);
                
                // Top draggable area (minimal padding to match input box side borders)
                let top_drag = ui.allocate_response(
                    egui::Vec2::new(ui.available_width(), 0.0),
                    egui::Sense::drag()
                );
                if top_drag.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                
                // Search input with larger, bold font (50% bigger than heading)
                let mut font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                font_id.size *= 1.5; // Make 50% larger
                
                // Show loading indicator if still loading
                if self.loading_state == LoadingState::Loading {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.spinner();
                        ui.label("Loading commands...");
                    });
                    ui.add_space(10.0);
                }
                
                // Show grabber countdown if active
                if let Some(count) = self.grabber_countdown {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        
                        // Large countdown number
                        let mut countdown_font = font_id.clone();
                        countdown_font.size *= 2.0; // Even bigger for countdown
                        
                        ui.label(egui::RichText::new(format!("Grabbing in: {}", count))
                            .font(countdown_font)
                            .color(egui::Color32::from_rgb(255, 100, 100))); // Red color
                    });
                    ui.add_space(10.0);
                }
                
                // Calculate hint text before mutable borrow
                let hint_text = if self.grabber_countdown.is_some() {
                    "Countdown active...".to_string()
                } else {
                    self.get_hint_text()
                };
                
                // Use different text edit based on loading state
                let response = {
                    // Temporarily modify style for more rounded text input corners
                    let mut style = ui.style().as_ref().clone();
                    style.visuals.widgets.inactive.rounding = egui::Rounding::same(6.0); // Half the window corner radius
                    style.visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
                    style.visuals.widgets.active.rounding = egui::Rounding::same(6.0);
                    ui.set_style(style);
                    
                    if self.loading_state == LoadingState::Loaded {
                        // Normal operation - edit popup state directly
                        ui.add_enabled(
                            !self.command_editor.visible && self.grabber_countdown.is_none(),
                            egui::TextEdit::singleline(&mut self.popup_state.search_text)
                                .desired_width(ui.available_width())
                                .hint_text(hint_text)
                                .font(font_id)
                        )
                    } else {
                        // Still loading - edit the buffer instead
                        ui.add_enabled(
                            !self.command_editor.visible && self.grabber_countdown.is_none(),
                            egui::TextEdit::singleline(&mut self.pre_init_input_buffer)
                                .desired_width(ui.available_width())
                                .hint_text(hint_text)
                                .font(font_id)
                        )
                    }
                };
                
                // Always update search when text field is changed
                if response.changed() {
                    if self.loading_state == LoadingState::Loaded {
                        // Normal operation
                        // Check for alias replacement
                        self.check_and_apply_alias();
                        
                        // ALWAYS update search results after any text change
                        let current_search = self.popup_state.search_text.clone();
                        self.popup_state.update_search(current_search);
                    }
                    // If still loading, changes are captured in pre_init_input_buffer
                }
                
                // Focus the text input on startup or when command editor closes
                // Extended focus attempt duration and window activation for better reliability
                let should_focus = !self.focus_set && (
                    self.frame_count <= 15 || command_editor_just_closed
                );
                
                if should_focus {
                    // On early frames, also try to activate the window to ensure proper focus
                    if self.frame_count <= 10 && !self.window_activated {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                        self.window_activated = true;
                    }
                    
                    response.request_focus();
                    if response.has_focus() {
                        self.focus_set = true;
                        crate::utils::debug_log("FOCUS", &format!("Focus successfully set on frame {}", self.frame_count));
                    } else if self.frame_count % 5 == 0 && self.frame_count <= 15 {
                        // Log focus attempts every 5 frames for debugging
                        crate::utils::debug_log("FOCUS", &format!("Focus attempt on frame {} - window focused: {}", 
                            self.frame_count, ctx.input(|i| i.focused)));
                    }
                }
                
                
                
                // Draggable space between input and list
                let mid_drag = ui.allocate_response(
                    egui::Vec2::new(ui.available_width(), 18.0),
                    egui::Sense::drag()
                );
                if mid_drag.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                
                // Command list - check for submenu and display accordingly  
                // No scroll area - window will size to accommodate max_rows
                // Only show commands if fully loaded
                if self.loading_state == LoadingState::Loaded && !self.filtered_commands().is_empty() {
                    // Get the display commands using our new method
                    let (display_commands, is_submenu, menu_prefix, inside_count) = self.get_display_commands();
                    
                    // Calculate required window dimensions based on final display commands
                    // Use actual font metrics for precise sizing
                    let mut input_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                    input_font_id.size *= 1.5; // Same scaling as used for input
                    let input_height = ui.fonts(|f| f.row_height(&input_font_id)) + 20.0; // Add padding for input box
                    
                    let mut list_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                    list_font_id.size *= 1.5; // Same scaling as used for command list
                    let row_height = ui.fonts(|f| f.row_height(&list_font_id)) + 4.0; // Add more spacing between rows
                    
                    let header_height = if is_submenu {
                        let mut header_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                        header_font_id.size *= 1.3; // Same scaling as used for header
                        ui.fonts(|f| f.row_height(&header_font_id)) + 8.0 // Add the gap after header
                    } else {
                        0.0
                    };
                    
                    let padding = 60.0; // Top and bottom margins - increased for safety
                    let bottom_drag_height = 20.0; // Height of bottom draggable area
                    let mid_drag_height = 18.0; // Height between input and list
                    
                    let (window_width, required_height) = match &self.popup_state.display_layout.arrangement {
                        LayoutArrangement::MultiColumn { rows, cols } => {
                            let rows_per_col = *rows;
                            let cols_to_use = *cols;
                            
                            // Calculate actual column widths by measuring the text
                            let mut column_widths = vec![0.0f32; cols_to_use];
                            
                            // Get display commands with same logic as rendering
                            let (display_commands, is_submenu, menu_prefix, inside_count) = self.get_display_commands();
                            
                            // Measure each command's text width
                            for col in 0..cols_to_use {
                                for row in 0..rows_per_col {
                                    let i = col * rows_per_col + row;
                                    if i >= display_commands.len() {
                                        break;
                                    }
                                    
                                    let cmd = &display_commands[i];
                                    let is_separator = PopupState::is_separator_command(cmd);
                                    
                                    // Determine display text with same logic as rendering
                                    let display_text = if is_submenu && !is_separator && i < inside_count {
                                        if let Some(ref prefix) = menu_prefix {
                                            if cmd.command.len() > prefix.len() {
                                                let prefix_end = prefix.len();
                                                if cmd.command[..prefix_end].eq_ignore_ascii_case(prefix) {
                                                    if let Some(ch) = cmd.command.chars().nth(prefix_end) {
                                                        if self.popup_state.config.popup_settings.word_separators.contains(ch) {
                                                            cmd.command[prefix_end + 1..].to_string()
                                                        } else {
                                                            cmd.command.clone()
                                                        }
                                                    } else {
                                                        cmd.command.clone()
                                                    }
                                                } else {
                                                    cmd.command.clone()
                                                }
                                            } else {
                                                cmd.command.clone()
                                            }
                                        } else {
                                            cmd.command.clone()
                                        }
                                    } else {
                                        cmd.command.clone()
                                    };
                                    
                                    // Measure the text width
                                    let text_width = if is_separator {
                                        ui.fonts(|f| f.glyph_width(&list_font_id, '—') * 3.0)
                                    } else {
                                        // Add some padding for selection highlight and margins
                                        let base_width = ui.fonts(|f| {
                                            let galley = f.layout_no_wrap(
                                                display_text.clone(),
                                                list_font_id.clone(),
                                                egui::Color32::WHITE
                                            );
                                            galley.rect.width()
                                        });
                                        // Add padding for selection, bold text (if merged), and margins
                                        base_width + 40.0
                                    };
                                    
                                    column_widths[col] = column_widths[col].max(text_width);
                                }
                            }
                            
                            // Calculate total width with spacing between columns
                            let column_spacing = 10.0;
                            let total_columns_width: f32 = column_widths.iter().sum::<f32>() + (cols_to_use.saturating_sub(1) as f32 * column_spacing);
                            let total_width = total_columns_width + 50.0; // Add some padding on sides
                            
                            let total_height = input_height + mid_drag_height + header_height + (rows_per_col as f32 * row_height) + bottom_drag_height + padding;
                            (total_width, total_height)
                        }
                        LayoutArrangement::SingleColumn => {
                            let (display_commands_single, _, _, _) = self.get_display_commands();
                            let window_width = 500.0;
                            let required_height = input_height + mid_drag_height + header_height + (display_commands_single.len() as f32 * row_height) + bottom_drag_height + padding;
                            (window_width, required_height)
                        }
                    };
                    
                    // Adjust window size if command editor is visible - make it larger to accommodate the dialog
                    let (final_width, final_height) = if self.command_editor.visible {
                        // Make window significantly larger when command editor is open
                        let editor_width = 500.0f32.max(window_width); // At least 500px wide
                        let editor_height = 400.0f32.max(required_height); // At least 400px tall
                        (editor_width, editor_height)
                    } else {
                        (window_width, required_height)
                    };
                    
                    // Window sizing is now handled automatically by update_window_size()
                    
                    // Use DisplayLayout to determine arrangement
                    match &self.popup_state.display_layout.arrangement {
                        LayoutArrangement::MultiColumn { rows, cols } => {
                            // Multi-column display
                            let rows_per_col = *rows;
                            let cols_to_use = *cols;
                            
                            // Show submenu header if applicable (even in multi-column mode)
                            if is_submenu {
                                if let Some(ref prefix) = menu_prefix {
                                    let mut header_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                                    header_font_id.size *= 1.3;
                                    
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(format!("{} ->", prefix)).font(header_font_id));
                                    });
                                    
                                    ui.add_space(8.0);
                                }
                            }
                        
                        ui.horizontal(|ui| {
                            for col in 0..cols_to_use {
                                ui.vertical(|ui| {
                                    for row in 0..rows_per_col {
                                        let i = col * rows_per_col + row;
                                        if i >= display_commands.len() {
                                            break;
                                        }
                                        
                                        let cmd = &display_commands[i];
                                        let is_selected = i == self.selected_index();
                                        let is_separator = PopupState::is_separator_command(cmd);
                                        
                                        // Determine display text based on submenu mode and position
                                        let display_text = if is_submenu && !is_separator && i < inside_count {
                                            // This is an inside menu command - apply trimming logic
                                            if let Some(ref prefix) = menu_prefix {
                                                // Check if this command should have its prefix trimmed
                                                if cmd.command.len() > prefix.len() {
                                                    let prefix_end = prefix.len();
                                                    if cmd.command[..prefix_end].eq_ignore_ascii_case(prefix) {
                                                        // Check if there's a separator right after the prefix
                                                        if let Some(ch) = cmd.command.chars().nth(prefix_end) {
                                                            if self.popup_state.config.popup_settings.word_separators.contains(ch) {
                                                                // Trim the prefix and separator
                                                                cmd.command[prefix_end + 1..].to_string()
                                                            } else {
                                                                cmd.command.clone()
                                                            }
                                                        } else {
                                                            cmd.command.clone()
                                                        }
                                                    } else {
                                                        cmd.command.clone()
                                                    }
                                                } else {
                                                    cmd.command.clone()
                                                }
                                            } else {
                                                cmd.command.clone()
                                            }
                                        } else {
                                            // This is either not in submenu mode OR it's an outside menu command
                                            // Always show full command name
                                            cmd.command.clone()
                                        };
                                        
                                        if is_separator {
                                            // Separator - not selectable, different styling
                                            let mut separator_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                                            separator_font_id.size *= 1.2;
                                            ui.label(egui::RichText::new("---").font(separator_font_id).color(egui::Color32::GRAY));
                                        } else {
                                            // Regular command
                                            let mut list_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                                            list_font_id.size *= 1.5; // Make 50% larger
                                            
                                            // Make merged entries (ending with "...") bold
                                            let text = if display_text.ends_with("...") {
                                                egui::RichText::new(&display_text).font(list_font_id.clone()).strong()
                                            } else {
                                                egui::RichText::new(&display_text).font(list_font_id)
                                            };
                                            
                                            let response = ui.selectable_label(
                                                is_selected,
                                                text
                                            );
                                            
                                            if response.clicked() {
                                                self.set_selected_index(i);
                                                // Execute command via server for consistency
                                                let launcher_command = if cmd.arg.is_empty() {
                                                    cmd.action.clone()
                                                } else {
                                                    format!("{} {}", cmd.action, cmd.arg)
                                                };
                                                
                                                match execute_via_server(&launcher_command, None, None, false) {
                                                    Ok(response) => {
                                                        if !response.success {
                                                            crate::utils::debug_log("EXECUTE", &format!("Command failed: {}", response.stderr));
                                                        }
                                                    }
                                                    Err(e) => {
                                                        crate::utils::debug_log("EXECUTE", &format!("Server communication failed: {}", e));
                                                    }
                                                }
                                                self.perform_exit_scanner_check();
                                                process::exit(0);
                                            }
                                        }
                                    }
                                });
                                
                                // Add space between columns (except after last column)
                                if col < cols_to_use - 1 {
                                    ui.add_space(10.0);
                                }
                            }
                        });
                        }
                        LayoutArrangement::SingleColumn => {
                            // Single-column display
                        
                        // Show submenu header if applicable
                        if is_submenu {
                            if let Some(ref prefix) = menu_prefix {
                                let mut header_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                                header_font_id.size *= 1.3;
                                
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(format!("{} ->", prefix)).font(header_font_id));
                                });
                                
                                ui.add_space(8.0);
                            }
                        }
                        
                        for (i, cmd) in display_commands.iter().enumerate() {
                            let is_selected = i == self.selected_index();
                            let is_separator = PopupState::is_separator_command(cmd);
                            
                            if is_separator {
                                // Separator - not selectable, different styling
                                let mut separator_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                                separator_font_id.size *= 1.2;
                                ui.label(egui::RichText::new("---").font(separator_font_id).color(egui::Color32::GRAY));
                            } else {
                                // Determine display text based on submenu mode and position
                                let display_text = if is_submenu && i < inside_count {
                                    // This is an inside menu command - apply trimming logic
                                    if let Some(ref prefix) = menu_prefix {
                                        // Check if this command should have its prefix trimmed
                                        if cmd.command.len() > prefix.len() {
                                            let prefix_end = prefix.len();
                                            if cmd.command[..prefix_end].eq_ignore_ascii_case(prefix) {
                                                // Check if there's a separator right after the prefix
                                                if let Some(ch) = cmd.command.chars().nth(prefix_end) {
                                                    if self.popup_state.config.popup_settings.word_separators.contains(ch) {
                                                        // Trim the prefix and separator
                                                        cmd.command[prefix_end + 1..].to_string()
                                                    } else {
                                                        cmd.command.clone()
                                                    }
                                                } else {
                                                    cmd.command.clone()
                                                }
                                            } else {
                                                cmd.command.clone()
                                            }
                                        } else {
                                            cmd.command.clone()
                                        }
                                    } else {
                                        cmd.command.clone()
                                    }
                                } else {
                                    // This is either not in submenu mode OR it's an outside menu command
                                    // Always show full command name
                                    cmd.command.clone()
                                };
                                
                                // Left-justified selectable label with draggable margins
                                ui.horizontal(|ui| {
                                    // Left margin draggable area
                                    let left_drag = ui.allocate_response(
                                        egui::Vec2::new(10.0, ui.text_style_height(&egui::TextStyle::Body)),
                                        egui::Sense::drag()
                                    );
                                    if left_drag.dragged() {
                                        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                                    }
                                    
                                    // Use larger font for command list (50% bigger than body)
                                    let mut list_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                                    list_font_id.size *= 1.5; // Make 50% larger
                                    
                                    // Make merged entries (ending with "...") bold
                                    let text = if display_text.ends_with("...") {
                                        egui::RichText::new(&display_text).font(list_font_id.clone()).strong()
                                    } else {
                                        egui::RichText::new(&display_text).font(list_font_id)
                                    };
                                    
                                    let response = ui.selectable_label(
                                        is_selected,
                                        text
                                    );
                                    
                                    if response.clicked() {
                                        self.set_selected_index(i);
                                        // Execute command via server for consistency
                                        let launcher_command = if cmd.arg.is_empty() {
                                            cmd.action.clone()
                                        } else {
                                            format!("{} {}", cmd.action, cmd.arg)
                                        };
                                        
                                        match execute_via_server(&launcher_command, None, None, false) {
                                            Ok(response) => {
                                                if !response.success {
                                                    crate::utils::debug_log("EXECUTE", &format!("Command failed: {}", response.stderr));
                                                }
                                            }
                                            Err(e) => {
                                                crate::utils::debug_log("EXECUTE", &format!("Server communication failed: {}", e));
                                            }
                                        }
                                        self.perform_exit_scanner_check();
                                        process::exit(0);
                                    }
                                    
                                    // Right margin draggable area
                                    let right_drag = ui.allocate_response(
                                        egui::Vec2::new(10.0, ui.text_style_height(&egui::TextStyle::Body)),
                                        egui::Sense::drag()
                                    );
                                    if right_drag.dragged() {
                                        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                                    }
                                });
                            }
                        }
                        }
                    }
                } else {
                    // No commands case - window sizing is handled automatically by update_window_size()
                }
                
                // Bottom draggable area
                let bottom_drag = ui.allocate_response(
                    egui::Vec2::new(ui.available_width(), 20.0),
                    egui::Sense::drag()
                );
                if bottom_drag.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
            });
        });
        
        // Handle dragging with middle mouse button anywhere in the window
        ctx.input(|i| {
            if i.pointer.middle_down() && i.pointer.is_decidedly_dragging() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
        });
        
        // Save window position immediately when it changes
        if let Some(current_pos) = ctx.input(|i| i.viewport().outer_rect.map(|r| r.min)) {
            if self.last_saved_position.is_none() {
                save_window_position(current_pos);
                self.last_saved_position = Some(current_pos);
            } else if let Some(last_pos) = self.last_saved_position {
                let moved = (current_pos.x - last_pos.x).abs() > 1.0 || (current_pos.y - last_pos.y).abs() > 1.0;
                if moved {
                    save_window_position(current_pos);
                    self.last_saved_position = Some(current_pos);
                }
            }
        }
        
    }
}

/// Embedded icon data compiled into the binary at build time
static EMBEDDED_ICON_PNG: &[u8] = include_bytes!("../../resources/icons/popup.png");

/// Cached icon data to avoid repeated PNG decoding
static CACHED_ICON: OnceLock<IconData> = OnceLock::new();

/// Load app icon with compile-time embedded data (no file I/O or ICNS parsing)
fn load_app_icon() -> IconData {
    CACHED_ICON.get_or_init(|| {
        // Decode the embedded PNG data
        if let Ok(decoded) = decode_png_to_rgba(EMBEDDED_ICON_PNG) {
            IconData {
                rgba: decoded.0,
                width: decoded.1,
                height: decoded.2,
            }
        } else {
            // Fallback: create a simple colored icon
            crate::utils::debug_log("ICON", "Failed to decode embedded icon, using fallback");
            create_fallback_icon()
        }
    }).clone()
}


fn decode_png_to_rgba(png_data: &[u8]) -> Result<(Vec<u8>, u32, u32), Box<dyn std::error::Error>> {
    use image::io::Reader as ImageReader;
    use image::ImageFormat;
    use std::io::Cursor;
    
    // Create a cursor from the PNG data
    let cursor = Cursor::new(png_data);
    let reader = ImageReader::with_format(cursor, ImageFormat::Png);
    
    // Decode the image
    let img = reader.decode()?;
    
    // Convert to RGBA8
    let rgba_img = img.to_rgba8();
    let width = rgba_img.width();
    let height = rgba_img.height();
    
    // Convert to Vec<u8>
    let rgba_data = rgba_img.into_raw();
    
    Ok((rgba_data, width, height))
}

fn create_fallback_icon() -> IconData {
    // Create a simple 32x32 blue anchor icon as fallback
    let width = 32u32;
    let height = 32u32;
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    
    // Create a simple anchor shape in blue
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let dx = x as i32 - 16;
            let dy = y as i32 - 16;
            
            // Simple anchor shape
            let is_anchor = (dx.abs() < 2 && dy > -10 && dy < 10) || 
                          (dy.abs() < 2 && dx.abs() < 8 && dy > 8) ||
                          (dx * dx + (dy + 8) * (dy + 8) < 16) ||
                          (dx * dx + (dy - 8) * (dy - 8) < 16);
            
            if is_anchor {
                rgba[idx] = 0;     // R
                rgba[idx + 1] = 100; // G
                rgba[idx + 2] = 200; // B
                rgba[idx + 3] = 255; // A
            } else {
                rgba[idx + 3] = 0; // Transparent
            }
        }
    }
    
    IconData {
        rgba,
        width,
        height,
    }
}

pub fn run_gui_with_prompt(initial_prompt: &str, _app_state: super::ApplicationState) -> Result<(), eframe::Error> {
    // Debug: Log when popup is being opened
    crate::utils::debug_log("POPUP_OPEN", &format!("Opening popup with initial prompt: '{}'", initial_prompt));
    
    // Capture the prompt for the closure
    let prompt = initial_prompt.to_string();
    
    // Manual window sizing - no auto-sizing constraints
    let viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size([500.0, 120.0]) // Initial size - will be dynamically resized
        .with_resizable(false) // Disable manual resizing - we control size programmatically
        .with_decorations(false) // Remove title bar and window controls
        .with_transparent(true); // Enable transparency for rounded corners
        // Skip icon loading for faster startup - can be added later if needed
    
    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        renderer: eframe::Renderer::Glow,
        run_and_return: false,
        vsync: true, // Re-enable vsync - disabling it causes unlimited FPS
        ..Default::default()
    };
    
    eframe::run_native(
        "Anchor Selector",
        options,
        Box::new(move |cc| {
            // Set light theme
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            
            // Configure for minimal CPU usage
            let mut style = (*cc.egui_ctx.style()).clone();
            style.animation_time = 0.0; // Disable animations
            cc.egui_ctx.set_style(style);
            
            
            // Set transparent background for corner areas
            if let Some(gl) = cc.gl.as_ref() {
                use eframe::glow::HasContext as _;
                unsafe {
                    // Transparent background to allow rounded corners
                    gl.clear_color(0.0, 0.0, 0.0, 0.0);
                }
            }
            
            Ok(Box::new(AnchorSelector::new_with_prompt(&prompt)))
        }),
    )
}