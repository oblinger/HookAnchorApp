//! Popup interface module for the Anchor Selector
//! 
//! This module contains the AnchorSelector struct and all popup-specific UI logic.

use eframe::egui;
use std::process;
use crate::{
    Command, execute_command, load_commands, 
    load_config, Config, load_state, save_state, scanner, grabber
};
use crate::core::config::{load_config_with_error, ConfigResult};
use super::{PopupState, LayoutArrangement};

use super::command_editor::{CommandEditor, CommandEditorResult};
use super::dialog::Dialog;

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
    /// Flag to disable automatic window resizing when we want manual control
    manual_resize_mode: bool,
    /// Track the last manual resize request to avoid repeatedly sending commands
    last_manual_size: Option<egui::Vec2>,
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
    /// Config error to show in dialog if config loading failed
    config_error: Option<String>,
}

impl AnchorSelector {
    // =============================================================================
    // Scanner Management
    // =============================================================================
    
    /// Perform scanner check before exiting to update commands for next launch
    fn perform_exit_scanner_check(&mut self) {
        if self.scanner_check_pending {
            self.scanner_check_pending = false;
            let updated_commands = scanner::startup_check(self.popup_state.get_commands().to_vec());
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
            
            // Check each pressed key against configured keybindings
            {
                let config = &self.popup_state.config;
                
                for event in &input.events {
                    if let egui::Event::Key { key, pressed, .. } = event {
                        if *pressed {
                            let key_name = format!("{:?}", key);
                            
                            // Check what action (if any) is bound to this key
                            if let Some(action) = config.get_action_for_key(&key_name) {
                                // Skip escape key since we handle it specially above
                                if key_name == "Escape" {
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
                                    "force_rescan" => actions_to_perform.push("force_rescan"),
                                    "start_grabber" => actions_to_perform.push("start_grabber"),
                                    "show_folder" => actions_to_perform.push("show_folder"),
                                    "exit_app" => actions_to_perform.push("exit_app"),
                                    "execute_command" => actions_to_perform.push("execute_command"),
                                    "open_editor" => actions_to_perform.push("open_editor"),
                                    "edit_active_command" => actions_to_perform.push("edit_active_command"),
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
            "force_rescan" => self.trigger_rescan(),
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
            "edit_active_command" => self.edit_active_command(),
            _ => {}
        }
    }
    
    /// Execute the currently selected command
    fn execute_selected_command(&mut self) {
        if !self.filtered_commands().is_empty() {
            let (display_commands, _is_submenu, _menu_prefix, _inside_count) = self.get_display_commands();
            
            if self.selected_index() < display_commands.len() {
                let selected_cmd = &display_commands[self.selected_index()];
                
                // Don't execute if it's a separator
                if !PopupState::is_separator_command(selected_cmd) {
                    if selected_cmd.command.ends_with("...") {
                        // Execute merged command directly
                        let launcher_command = if selected_cmd.arg.is_empty() {
                            selected_cmd.action.clone()
                        } else {
                            format!("{} {}", selected_cmd.action, selected_cmd.arg)
                        };
                        
                        let config = load_config();
                        if config.popup_settings.use_new_launcher {
                            use crate::launcher::launch;
                            if let Err(e) = launch(&launcher_command) {
                                eprintln!("Error executing command with new launcher: {:?}", e);
                                std::process::exit(1);
                            }
                        } else {
                            use std::fs;
                            let content = format!("execute {}\n", launcher_command);
                            if let Err(e) = fs::write("/tmp/cmd_file", content) {
                                eprintln!("Error writing to /tmp/cmd_file: {}", e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        execute_command(&selected_cmd);
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
    
    // =============================================================================
    // Initialization
    // =============================================================================
    
    pub fn new() -> Self {
        Self::new_with_prompt("")
    }
    
    pub fn new_with_prompt(initial_prompt: &str) -> Self {
        let commands = load_commands();
        let (config, config_error) = match load_config_with_error() {
            ConfigResult::Success(config) => (config, None),
            ConfigResult::Error(error) => (load_config(), Some(error)), // Fall back to default config
        };
        let state = load_state();
        
        // Create popup state with business logic
        let mut popup_state = PopupState::new(commands, config, state);
        
        // Set initial prompt if provided
        if !initial_prompt.is_empty() {
            popup_state.search_text = initial_prompt.to_string();
            popup_state.update_search(initial_prompt.to_string());
        }
        
        let mut result = Self {
            popup_state,
            last_saved_position: None,
            position_set: false,
            command_editor: CommandEditor::new(),
            dialog: Dialog::new(),
            manual_resize_mode: false,
            last_manual_size: None,
            scanner_check_pending: true,
            grabber_countdown: None,
            countdown_last_update: None,
            focus_set: false,
            frame_count: 0,
            config_error,
        };
        
        // Show config error dialog if there was an error
        if let Some(error) = &result.config_error {
            result.dialog.show_error(error);
        }
        
        result
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
    
    /// Start the grabber countdown (5, 4, 3, 2, 1)
    fn start_grabber_countdown(&mut self, _ctx: &egui::Context) {
        self.grabber_countdown = Some(5);
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
                        self.grabber_countdown = Some(count - 1);
                        self.countdown_last_update = Some(std::time::Instant::now());
                    } else {
                        // Countdown finished, execute grab
                        self.execute_grab(ctx);
                        self.grabber_countdown = None;
                        self.countdown_last_update = None;
                    }
                }
            }
        }
    }
    
    /// Execute the grab operation - simplified synchronous version
    fn execute_grab(&mut self, ctx: &egui::Context) {
        let config = load_config();
        
        // Give up focus AND properly activate the previous application window
        if let Err(e) = self.give_up_focus_and_activate() {
            crate::utils::debug_log("GRAB", &format!("Failed to give up focus: {}", e));
        }
        
        // Brief delay to ensure focus change and window activation completes
        std::thread::sleep(std::time::Duration::from_millis(200));
        
        match grabber::grab(&config) {
            Ok(grab_result) => {
                match grab_result {
                    grabber::GrabResult::RuleMatched(rule_name, mut command) => {
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
                        self.command_editor.open_with_command(command);
                    }
                    grabber::GrabResult::NoRuleMatched(context) => {
                        // Generate the template text
                        let template_text = grabber::generate_rule_template_text(&context);
                        
                        // Show template dialog using the new TextBox field type
                        let dialog_spec = vec![
                            format!("=Grabber Rule Template - {}", context.app_name),
                            format!("&{}", template_text),
                            "!OK".to_string(),
                        ];
                        
                        self.dialog.show(dialog_spec);
                        
                        // Calculate required dialog size and resize window
                        let (dialog_width, dialog_height) = self.dialog.calculate_required_size();
                        let final_width = dialog_width.max(600.0); // Minimum width for readability
                        let final_height = dialog_height.max(400.0); // Minimum height
                        
                        // Enable manual resize mode and set window size
                        self.manual_resize_mode = true;
                        self.last_manual_size = Some([final_width, final_height].into());
                        
                        // Actually resize the window
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([final_width, final_height].into()));
                    }
                }
            }
            Err(err) => {
                crate::utils::debug_log("GRAB", &format!("Grabber error: {}", err));
            }
        }
        
        // Regain focus back to anchor selector after grab operation
        if let Err(e) = self.regain_focus() {
            crate::utils::debug_log("GRAB", &format!("Failed to regain focus: {}", e));
        }
    }
    
    
    /// Regain focus back to anchor selector after grab operation
    fn regain_focus(&self) -> Result<(), String> {
        // Use the JavaScript function to regain focus
        let config = load_config();
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
    
    /// Trigger an immediate filesystem rescan
    fn trigger_rescan(&mut self) {
        crate::utils::debug_log("SCANNER2", "=== TRIGGER_RESCAN FUNCTION CALLED ===");
        use crate::scanner;
        let config = load_config();
        
        // Get markdown roots from config
        if let Some(markdown_roots) = &config.markdown_roots {
            crate::utils::debug_log("SCANNER2", &format!("Force scanning markdown files, roots: {:?}", markdown_roots));
            
            // Force scan markdown files
            let current_commands = self.popup_state.get_commands().to_vec();
            let updated_commands = scanner::scan(current_commands, markdown_roots);
            
            // Save the updated commands to file
            use crate::save_commands_to_file;
            if let Err(e) = save_commands_to_file(&updated_commands) {
                crate::utils::debug_log("RESCAN", &format!("Failed to save commands: {}", e));
            } else {
                crate::utils::debug_log("RESCAN", "Commands saved successfully");
            }
            
            // Update commands in popup state
            self.popup_state.set_commands(updated_commands);
            
            // Refresh current search results if there's an active search
            if !self.popup_state.search_text.trim().is_empty() {
                let current_search = self.popup_state.search_text.clone();
                self.popup_state.update_search(current_search);
            }
            
            crate::utils::debug_log("SCANNER2", "Rescan completed");
        } else {
            crate::utils::debug_log("RESCAN", "No markdown roots configured in config file");
        }
    }
    
    /// Show folder functionality - launches the first folder matching current search
    fn show_folder(&mut self) {
        use crate::{launcher, utils};
        
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
        
        // Use the first non-separator command and extract its folder
        for cmd in &display_commands {
            if PopupState::is_separator_command(cmd) {
                utils::debug_log("SHOW_FOLDER", &format!("Skipping separator: '{}'", cmd.command));
                continue;
            }
            
            utils::debug_log("SHOW_FOLDER", &format!("Processing command: '{}' (action: {})", cmd.command, cmd.action));
            
            // Resolve alias if needed
            let resolved_cmd = if cmd.action == "alias" {
                utils::debug_log("SHOW_FOLDER", &format!("Resolving alias '{}' to target '{}'", cmd.command, cmd.arg));
                // Find the target command
                if let Some(target) = all_commands.iter().find(|c| c.command == cmd.arg) {
                    utils::debug_log("SHOW_FOLDER", &format!("Alias resolved to: '{}' (action: {}, arg: {})", 
                        target.command, target.action, target.arg));
                    target
                } else {
                    utils::debug_log("SHOW_FOLDER", &format!("Failed to resolve alias '{}' - target '{}' not found", cmd.command, cmd.arg));
                    cmd
                }
            } else {
                cmd
            };
            
            // Extract folder based on action type (using resolved command)
            let folder_path = match resolved_cmd.action.as_str() {
                "folder" => {
                    utils::debug_log("SHOW_FOLDER", &format!("Found folder action, path: {}", resolved_cmd.arg));
                    Some(resolved_cmd.arg.clone())
                },
                "anchor" | "obs" => {
                    // For anchor/obs, get the directory containing the file
                    if let Some(idx) = resolved_cmd.arg.rfind('/') {
                        let path = resolved_cmd.arg[..idx].to_string();
                        utils::debug_log("SHOW_FOLDER", &format!("Found {}, extracted folder: {}", resolved_cmd.action, path));
                        Some(path)
                    } else {
                        utils::debug_log("SHOW_FOLDER", &format!("Found {} but no slash in path: {}", resolved_cmd.action, resolved_cmd.arg));
                        None
                    }
                },
                "alias" => {
                    // This shouldn't happen since we already resolved aliases above
                    utils::debug_log("SHOW_FOLDER", &format!("Unresolved alias found: '{}'", cmd.command));
                    None
                },
                other => {
                    utils::debug_log("SHOW_FOLDER", &format!("Command '{}' has non-folder action: {}", cmd.command, other));
                    None
                }
            };
            
            if let Some(path) = folder_path {
                utils::debug_log("SHOW_FOLDER", &format!("Attempting to launch folder: '{}'", path));
                
                // Launch the folder (popup stays open)
                match launcher::launch(&format!("folder {}", path)) {
                    Ok(()) => {
                        utils::debug_log("SHOW_FOLDER", &format!("Successfully launched folder: '{}'", path));
                    },
                    Err(e) => {
                        utils::debug_log("SHOW_FOLDER", &format!("Failed to launch folder '{}': {:?}", path, e));
                    }
                }
                return;
            }
        }
        
        utils::debug_log("SHOW_FOLDER", "No folder found in filtered commands");
    }
    
    /// Give up focus and properly activate the previous application window
    fn give_up_focus_and_activate(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;
        
        crate::utils::debug_log("FOCUS", "Switching to previous app with Cmd+Tab");
        
        // Use Cmd+Tab to switch to previous application (this was working)
        Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to key code 48 using command down")
            .output()?;
        
        // Brief delay to let the switch complete
        std::thread::sleep(std::time::Duration::from_millis(300));
        
        crate::utils::debug_log("FOCUS", "Raising windows of frontmost application");
        
        // Now raise/activate all windows of the frontmost application
        Command::new("osascript")
            .arg("-e")
            .arg(r#"
                tell application "System Events"
                    set frontApp to first application process whose frontmost is true
                    tell frontApp
                        set frontmost to true
                        -- Raise all windows
                        repeat with w in windows
                            set index of w to 1
                        end repeat
                    end tell
                end tell
            "#)
            .output()?;
        
        Ok(())
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Increment frame counter for initial setup only
        if self.frame_count < 10 {
            self.frame_count += 1;
        }
        
        // Check if we need active updates (animations, countdowns, etc.)
        let has_input = ctx.input(|i| !i.events.is_empty() || i.pointer.any_down() || i.keys_down.len() > 0);
        let has_active_ui = self.command_editor.visible 
            || self.dialog.visible 
            || self.grabber_countdown.is_some();
            
        // For idle state, request slower repaints to reduce CPU usage
        if !has_input && !has_active_ui && self.frame_count >= 10 {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
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
        
        
        // Draw custom rounded background with heavy shadow
        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(egui::LayerId::background());
        
        // Draw multiple shadow layers for a much darker shadow effect
        let shadow_offsets = [12.0, 10.0, 8.0, 6.0, 4.0, 2.0];
        let shadow_alphas = [50, 70, 90, 110, 130, 150];
        
        for (offset, alpha) in shadow_offsets.iter().zip(shadow_alphas.iter()) {
            let shadow_rect = screen_rect.translate(egui::vec2(*offset, *offset));
            painter.rect_filled(
                shadow_rect,
                egui::Rounding::same(12.0),
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
        
        // Update dialog system
        if self.dialog.update(ctx) {
            if let Some(result) = self.dialog.take_result() {
                // Check if the "Exit" button was clicked
                if let Some(button_text) = result.get("exit") {
                    if button_text == "Exit" {
                        self.perform_exit_scanner_check();
                        std::process::exit(0);
                    }
                }
                
                // Resize back to normal when dialog closes and re-enable automatic resizing
                self.manual_resize_mode = false;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([500.0, 300.0].into()));
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
                
                let response = ui.add_enabled(
                    !self.command_editor.visible && self.grabber_countdown.is_none(), // Disable when dialog is open or countdown active
                    egui::TextEdit::singleline(&mut self.popup_state.search_text)
                        .desired_width(ui.available_width())
                        .hint_text(hint_text)
                        .font(font_id)
                );
                
                // Always update search when text field is changed
                if response.changed() {
                    
                    // Removed hardcoded character handlers - these should be handled via keybindings
                    
                    // Check for alias replacement
                    self.check_and_apply_alias();
                    
                    // ALWAYS update search results after any text change
                    let current_search = self.popup_state.search_text.clone();
                    self.popup_state.update_search(current_search);
                    
                    // Removed hardcoded slash handler - editor opening now handled via keybindings
                }
                
                // Focus the text input on startup or when command editor closes
                // Only attempt focus for a limited number of frames to avoid continuous repainting
                let should_focus = !self.focus_set && (
                    self.frame_count <= 5 || command_editor_just_closed
                );
                
                if should_focus {
                    response.request_focus();
                    if response.has_focus() {
                        self.focus_set = true;
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
                if !self.filtered_commands().is_empty() {
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
                            
                            let column_width = 250.0; // Width per column
                            let total_width = (cols_to_use as f32 * column_width) + 50.0; // Add some padding
                            let total_height = input_height + mid_drag_height + header_height + (rows_per_col as f32 * row_height) + bottom_drag_height + padding;
                            (total_width, total_height)
                        }
                        LayoutArrangement::SingleColumn => {
                            let window_width = 500.0;
                            let required_height = input_height + mid_drag_height + header_height + (display_commands.len() as f32 * row_height) + bottom_drag_height + padding;
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
                    
                    // Resize window to accommodate content
                    if self.manual_resize_mode {
                        // Use manual size if set (works even when dialog is open)
                        if let Some(manual_size) = self.last_manual_size {
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(manual_size));
                        }
                    } else if self.dialog.visible {
                        // When dialog is visible, make sure main window is large enough to accommodate it
                        let (dialog_width, dialog_height) = self.dialog.calculate_required_size();
                        let required_width = dialog_width.max(final_width);
                        let required_height = dialog_height.max(final_height);
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(required_width, required_height)));
                    } else {
                        // Use automatic size when dialog is not open
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(final_width, final_height)));
                    }
                    
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
                                                execute_command(&cmd);
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
                                        execute_command(&cmd);
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
                    // No commands - resize to minimal height (just input field), but expand if command editor is open
                    let input_height = 60.0;
                    let padding = 50.0;
                    let base_height = input_height + padding;
                    
                    let (final_width, final_height) = if self.command_editor.visible {
                        // Make window larger when command editor is open, even with no commands
                        (500.0, 400.0f32.max(base_height))
                    } else {
                        (500.0, base_height)
                    };
                    
                    // Resize window (same logic as when there are commands)
                    if self.manual_resize_mode {
                        // Use manual size if set (works even when dialog is open)
                        if let Some(manual_size) = self.last_manual_size {
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(manual_size));
                        }
                    } else if self.dialog.visible {
                        // When dialog is visible, make sure main window is large enough to accommodate it
                        let (dialog_width, dialog_height) = self.dialog.calculate_required_size();
                        let required_width = dialog_width.max(final_width);
                        let required_height = dialog_height.max(final_height);
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(required_width, required_height)));
                    } else {
                        // Use automatic size when dialog is not open
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(final_width, final_height)));
                    }
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

/// Run the popup GUI with an optional initial prompt and application state
pub fn run_gui_with_prompt(initial_prompt: &str, _app_state: super::ApplicationState) -> Result<(), eframe::Error> {
    // Capture the prompt for the closure
    let prompt = initial_prompt.to_string();
    
    // Manual window sizing - no auto-sizing constraints
    let viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size([500.0, 120.0]) // Initial size - will be dynamically resized
        .with_resizable(false) // Disable manual resizing - we control size programmatically
        .with_decorations(false); // Remove title bar and window controls
        // .with_transparent(true); // DISABLED: May cause hanging
    
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
            
            
            // Set light grey background for corner areas
            if let Some(gl) = cc.gl.as_ref() {
                use eframe::glow::HasContext as _;
                unsafe {
                    // Light grey (200/255 = 0.78) with full opacity for corners
                    gl.clear_color(0.78, 0.78, 0.78, 1.0);
                }
            }
            
            Ok(Box::new(AnchorSelector::new_with_prompt(&prompt)))
        }),
    )
}