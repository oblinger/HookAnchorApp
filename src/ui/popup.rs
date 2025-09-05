//! Popup interface module for the Anchor Selector
//! 
//! This module contains the AnchorSelector struct and all popup-specific UI logic.

use eframe::egui::{self, IconData};
use std::sync::OnceLock;
use std::collections::HashMap;
use crate::core::Command;
use crate::core::{
    Config, load_state, save_state
};
use crate::core::key_processing::{PopupInterface, KeyRegistry, create_default_key_registry};
use crate::core::commands::get_patch_path;

#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;
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
    /// Track if current position was set automatically (centering, bottom-avoidance) vs user action
    was_auto_positioned: bool,
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
    /// Request focus on next frame
    request_focus: bool,
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
    /// Pending template info for grab functionality (template_name, context)
    pending_template: Option<(String, crate::core::template_creation::TemplateContext)>,
    /// Key registry for unified key processing
    key_registry: Option<KeyRegistry>,
    /// Configurable exit keystrokes
    exit_app_key: Option<crate::core::key_processing::Keystroke>,
    /// Flag to request exit/hide on next update
    should_exit: bool,
    /// Track if window is currently hidden
    is_hidden: bool,
    /// Flag to track if rebuild is pending (deferred to next frame)
    pending_rebuild: bool,
    /// Pending action waiting for user confirmation or input
    pending_action: Option<PendingAction>,
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

/// Action to be executed after user confirms via dialog or other UI component
pub struct PendingAction {
    /// Context data passed to the action
    pub context: HashMap<String, String>,
    /// Callback function to execute with final context (including user input)
    pub callback: Box<dyn FnOnce(&HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>>>,
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
        let config = crate::core::sys_data::get_config();
        let default_width = config.popup_settings.get_default_window_width() as f32;
        let max_width = config.popup_settings.get_max_window_width() as f32;
        let max_height = config.popup_settings.get_max_window_height() as f32;
        
        // Start with default width, but allow it to grow based on content
        let mut calculated_width = default_width;
        
        // If we have commands, calculate optimal width based on content
        if command_count > 0 {
            let (display_commands, _, _, _) = self.get_display_commands();
            
            // Estimate the width needed for the longest command text
            let mut max_text_width: f32 = 0.0;
            for cmd in &display_commands {
                // Rough estimation: ~8 pixels per character + padding
                let estimated_width = (cmd.command.len() as f32 * 8.0) + 40.0;
                max_text_width = max_text_width.max(estimated_width);
            }
            
            // Choose between content-based width and default width, but don't exceed max
            calculated_width = max_text_width.max(default_width).min(max_width);
        }
        
        // Calculate height based on content
        let base_height = 120.0; // Base height for input field
        
        // Add height for command list (approximately 30 pixels per command)
        let command_list_height = if command_count > 0 {
            (command_count.min(12) as f32) * 30.0 // Cap at 12 visible commands
        } else {
            0.0
        };
        
        // Add extra height if grabber countdown is active
        let countdown_height = if self.grabber_countdown.is_some() { 60.0 } else { 0.0 };
        
        let calculated_height = base_height + command_list_height + countdown_height;
        let final_height = calculated_height.min(max_height);
        
        egui::vec2(calculated_width, final_height)
    }
    
    /// Calculate the required window size for editor mode
    fn calculate_editor_size(&self) -> egui::Vec2 {
        // Command editor needs space for all input fields
        let config = crate::core::sys_data::get_config();
        let default_width = config.popup_settings.get_default_window_width() as f32;
        let default_height = config.popup_settings.get_default_window_height() as f32;
        let max_width = config.popup_settings.get_max_window_width() as f32;
        let max_height = config.popup_settings.get_max_window_height() as f32;
        
        // Calculate width needed for editor fields
        // Editor has: Command, Action, Argument, Patch, Flags, Priority fields + labels + buttons
        let field_width = 300.0f32; // Width for input fields
        let label_width = 80.0f32;   // Width for field labels  
        let button_width = 100.0f32; // Width for Delete button
        let padding = 40.0f32;       // Left/right padding
        
        let calculated_width = (label_width + field_width + button_width + padding).max(default_width).min(max_width);
        
        // Calculate height needed for editor fields 
        let field_height = 35.0f32;   // Height per input field
        let field_spacing = 10.0f32;  // Spacing between fields
        let num_fields = 6.0f32;      // Command, Action, Argument, Patch, Flags, Priority
        let header_height = 40.0f32;  // "Command Editor" header
        let button_row_height = 50.0f32; // Bottom button row
        let vertical_padding = 30.0f32;  // Top/bottom padding
        
        let calculated_height = (header_height + (num_fields * (field_height + field_spacing)) + button_row_height + vertical_padding).max(default_height).min(max_height);
        
        egui::vec2(calculated_width, calculated_height)
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
            WindowSizeMode::Dialog => {
                // Get the dialog's calculated size
                if self.dialog.visible {
                    let (width, height) = self.dialog.calculate_required_size(ctx);
                    egui::vec2(width, height)
                } else {
                    // Default reasonable size if dialog not yet visible
                    let config = crate::core::sys_data::get_config();
                    let width = config.popup_settings.get_default_window_width() as f32;
                    let height = config.popup_settings.get_default_window_height() as f32;
                    egui::vec2(width, height)
                }
            }
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
            
            crate::utils::detailed_log("WINDOW_SIZE", &format!(
                "Window resized to {:?} mode: {}x{}", 
                new_mode, required_size.x, required_size.y
            ));
        }
    }

    // =============================================================================
    // Scanner Management
    // =============================================================================
    
    /// Exit or hide the application based on background mode setting
    fn exit_or_hide(&mut self, ctx: &egui::Context) {
        // Prevent multiple calls in the same frame
        static mut HIDING: bool = false;
        unsafe {
            if HIDING {
                crate::utils::verbose_log("EXIT_OR_HIDE", "Already hiding, returning early");
                return; // Already hiding, don't call again
            }
            HIDING = true;
        }
        
        // Check if we're in direct mode (set by launcher via environment variable)
        let direct_mode = std::env::var("HOOKANCHOR_DIRECT_MODE").is_ok();
        crate::utils::verbose_log("EXIT_OR_HIDE", &format!("Direct mode: {}", direct_mode));
        
        if !direct_mode {
            // Server mode - hide window using AppleScript
            crate::utils::verbose_log("EXIT_OR_HIDE", "Server mode - hiding window");
            
            // Save current position before hiding
            if let Some(current_pos) = ctx.input(|i| i.viewport().outer_rect.map(|r| r.min)) {
                crate::utils::log(&format!("[EXIT_OR_HIDE] Saving position before hide: {:?}", current_pos));
                save_window_position(current_pos);
                self.last_saved_position = Some(current_pos);
            }
            
            // Clear the search input for next time
            self.popup_state.search_text.clear();
            self.popup_state.update_search(String::new());
            
            // IMPORTANT: Restore focus to the previous application before hiding
            // This ensures that text insertion and other operations work correctly
            crate::utils::verbose_log("EXIT_OR_HIDE", "Restoring focus to previous application");
            if let Err(e) = self.regain_focus() {
                crate::utils::log_error(&format!("Failed to restore focus before hiding: {}", e));
            }
            
            // Hide the window using egui's viewport command
            crate::utils::verbose_log("EXIT_OR_HIDE", "Sending ViewportCommand::Visible(false)");
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            
            crate::utils::verbose_log("EXIT_OR_HIDE", &format!("Setting is_hidden=true (was {})", self.is_hidden));
            self.is_hidden = true;
            
            // Reset interaction time to prevent timeout loop
            self.last_interaction_time = std::time::Instant::now();
            
            let viewport_info = ctx.input(|i| {
                format!("Viewport after hide: focused={}, visible={:?}, position={:?}",
                    i.focused,
                    i.viewport().inner_rect,
                    i.viewport().outer_rect
                )
            });
            crate::utils::verbose_log("EXIT_OR_HIDE", &viewport_info);
            crate::utils::verbose_log("EXIT_OR_HIDE", "Window hidden via viewport commands");
            // Reset the flag after a delay to allow for future hide operations
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(100));
                unsafe { 
                    HIDING = false;
                    crate::utils::verbose_log("EXIT_OR_HIDE", "HIDING flag reset to false");
                }
            });
        } else {
            // Direct mode - exit the application
            crate::utils::log("EXIT: Exiting application (direct mode)");
            std::process::exit(0);
        }
    }
    
    /// Perform scanner check before exiting to update commands for next launch
    fn perform_exit_scanner_check(&mut self) {
        if self.scanner_check_pending {
            self.scanner_check_pending = false;
            let updated_commands = crate::systems::scan_check(self.popup_state.get_commands().to_vec());
            if updated_commands.len() != self.popup_state.get_commands().len() {
                // Commands have changed, update the popup state for next time
                self.popup_state.set_commands(updated_commands);
            }
        }
    }

    // =============================================================================
    // Centralized Key Handling
    // =============================================================================
    
    
    /// Centralized key handling for the main popup interface
    /// Dispatch key handling to the appropriate interface
    /// Returns true if any keys were processed
    fn handle_keys(&mut self, ctx: &egui::Context) -> bool {
        // Check which interface is active first
        if self.command_editor.visible {
            // When command editor is visible, we don't process ANY keys here
            // The command editor will handle its own keys in its update() method
            return false; // No keys processed here
        }
        
        if self.dialog.visible {
            // When dialog is visible, we don't process ANY keys here
            // The dialog will handle its own keys in its update() method
            return false; // No keys processed here
        }
        
        // Neither command editor nor dialog is open, handle popup keys
        // Extract events - only needed when we're actually going to process them
        let mut events_to_process = Vec::new();
        ctx.input_mut(|input| {
            events_to_process = input.events.clone();
        });
        
        return self.handle_popup_keys_with_events(ctx, events_to_process);
    }
    
    /// Handle keys for the main popup interface using registry system
    /// Returns true if any keys were processed
    fn handle_popup_keys_with_events(&mut self, ctx: &egui::Context, events_to_process: Vec<egui::Event>) -> bool {
        // If still loading, only handle escape key
        if self.loading_state != LoadingState::Loaded {
            crate::utils::detailed_log("KEY_REGISTRY", &format!("Cannot process keys, loading_state = {:?}", self.loading_state));
            return false; // No keys processed during loading
        }
        
        // Check if we have registry and process events
        if self.key_registry.is_some() {
            // Take ownership of the registry temporarily to avoid borrowing issues
            let registry = self.key_registry.take().unwrap();
            let handled = registry.process_events(&events_to_process, self, ctx);
            
            // Put the registry back
            self.key_registry = Some(registry);
            
            if handled {
                ctx.input_mut(|input| {
                    // Clear all processed events from input
                    input.events.clear();
                    // Also clear key states to prevent stuck keys
                    input.keys_down.clear();
                });
            }
            
            handled // Return whether any keys were actually handled
        } else {
            // Fallback: registry not initialized yet
            false
        }
    }}

// ================================================================================================
// POPUP INTERFACE IMPLEMENTATION FOR KEY REGISTRY
// ================================================================================================

impl PopupInterface for AnchorSelector {
    fn navigate_vertical(&mut self, delta: i32) {
        self.navigate_vertical(delta);
    }
    
    fn navigate_horizontal(&mut self, delta: i32) {
        self.navigate_horizontal(delta);
    }
    
    fn trigger_rebuild(&mut self, ctx: &egui::Context) {
        self.trigger_rebuild(ctx);
    }
    
    fn start_grabber_countdown(&mut self, ctx: &egui::Context) {
        self.start_grabber_countdown(ctx);
    }
    
    fn show_folder(&mut self) {
        self.show_folder_impl();
    }
    
    fn show_contact(&mut self) {
        self.show_contact_impl();
    }
    
    // COMMENTED OUT: Unused function - replaced by activate_tmux (formerly activate_anchor)
    // fn tmux_activate(&mut self) {
    //     self.tmux_activate();
    // }
    
    fn activate_tmux(&mut self) {
        self.activate_tmux();
    }
    
    fn perform_exit_scanner_check(&mut self) {
        self.perform_exit_scanner_check();
    }
    
    fn request_exit(&mut self) {
        // Don't perform scanner check on exit - it blocks the UI
        // Scanner checks should be done periodically in the background instead
        self.should_exit = true;
    }
    
    fn execute_selected_command(&mut self) {
        // Call the real implementation
        self.execute_selected_command_impl();
    }
    
    fn open_command_editor(&mut self) {
        // Call the real implementation
        self.open_command_editor_impl();
    }
    
    fn handle_add_alias(&mut self) {
        // Call the real implementation
        self.handle_add_alias_impl();
    }
    
    fn edit_active_command(&mut self) {
        // Call the real implementation
        self.edit_active_command_impl();
    }
    
    fn edit_input_command(&mut self) {
        // NOTE: This is now handled by template with edit:true flag in config.yaml
        // The "=" key is configured as a template that creates a new command
        // with the input text as the name and opens the editor (edit: true)
        // This method is kept for backward compatibility with old configs
        self.edit_input_command_impl();
    }
    
    fn handle_link_to_clipboard(&mut self) {
        // Call the real implementation
        self.handle_link_to_clipboard_impl();
    }
    
    fn show_keys_dialog(&mut self) {
        // Call the real implementation
        self.show_keys_dialog_impl();
    }
    
    fn handle_uninstall_app(&mut self) {
        // Call the real implementation
        self.handle_uninstall_app_impl();
    }
    
    fn handle_template_create(&mut self) {
        // Call the real implementation
        self.handle_template_create_impl();
    }
    
    fn handle_template_create_named(&mut self, template_name: &str) {
        // Call the real implementation
        self.handle_template_create_named_impl(template_name);
    }
    
    fn is_command_editor_visible(&self) -> bool {
        self.command_editor.visible
    }
    
    fn is_dialog_visible(&self) -> bool {
        self.dialog.visible
    }
    
    fn close_command_editor(&mut self) {
        self.command_editor.visible = false;
        // Reset window size when closing editor
        self.window_size_mode = WindowSizeMode::Normal;
        // Request focus on input field
        self.request_focus = true;
    }
    
    fn close_dialog(&mut self) {
        self.dialog.visible = false;
        // Reset window size when closing dialog
        self.window_size_mode = WindowSizeMode::Normal;
        // Request focus on input field
        self.request_focus = true;
    }
    
    fn get_search_text(&self) -> &str {
        &self.popup_state.search_text
    }
    
    fn update_search(&mut self, text: String) {
        self.popup_state.update_search(text);
    }
}

impl AnchorSelector {
    
    /// Display an error dialog to the user
    /// This is a generic function for showing errors in a popup dialog
    /// This is private - errors should be queued via queue_user_error() 
    /// and displayed automatically by the UI
    fn show_error_dialog(&mut self, error_message: &str) {
        self.dialog.show_error(error_message);
    }
    
    // =============================================================================
    // Unified Action Execution System
    // =============================================================================
    
    /// Main entry point for executing actions from the popup
    fn execute(&mut self, action: &crate::execute::Action) {
        use crate::utils::debug_log;
        
        debug_log("POPUP_EXEC", &format!("Executing action type: {}", action.action_type()));
        
        // Execute the action - will route to server or local as appropriate
        match action.action_type() {
            // UI actions need special handling in popup
            "template" | "popup" => {
                debug_log("POPUP_EXEC", "Executing UI action in popup");
                self.execute_in_popup(action);
            }
            // All other actions go to server
            _ => {
                let _ = crate::execute::execute_on_server(action);
                debug_log("POPUP_EXEC", "Action sent to server");
                self.should_exit = true;
            }
        }
    }
    
    /// Execute an action in the popup context (for UI-requiring actions)
    fn execute_in_popup(&mut self, action: &crate::execute::Action) {
        use crate::utils::{debug_log, log_error};
        
        debug_log("POPUP_LOCAL", &format!("Executing {} locally", action.action_type()));
        
        // For template actions, we need special UI handling
        if action.action_type() == "template" {
            // Get template name from action or use default
            let template_name = action.get_string("name").unwrap_or("default");
            debug_log("POPUP_LOCAL", &format!("Processing template UI: {}", template_name));
            
            // Use existing template UI handling
            self.handle_template_create_named_impl(template_name);
        } else if action.action_type() == "popup" {
            // Handle popup control actions
            debug_log("POPUP_LOCAL", "Handling popup control action");
            // TODO: Implement popup control (show/hide/etc)
            self.should_exit = true;
        } else {
            // This shouldn't happen - non-UI actions should go to server
            log_error(&format!("Unexpected action type in execute_in_popup: {}", action.action_type()));
        }
    }
    
    /// Execute the currently selected command
    fn execute_selected_command_impl(&mut self) {
        
        if !self.filtered_commands().is_empty() {
            let (display_commands, _is_submenu, _menu_prefix, _inside_count) = self.get_display_commands();
            
            if self.selected_index() < display_commands.len() {
                let selected_cmd = &display_commands[self.selected_index()];
                
                // Don't execute if it's a separator
                if !PopupState::is_separator_command(selected_cmd) {
                    // Save the last executed command for add_alias functionality
                    use crate::core::state::save_last_executed_command;
                    let _ = save_last_executed_command(&selected_cmd.command);
                    
                    // Convert command to action and execute through unified system
                    crate::utils::log("=== UNIFIED ACTION SYSTEM: Converting command to action ===");
                    use crate::execute::command_to_action;
                    let action = command_to_action(selected_cmd);
                    crate::utils::log(&format!("UNIFIED ACTION: Created action type='{}' from command='{}'", 
                        action.action_type(), selected_cmd.command));
                    self.execute(&action);
                } else {
                    // Command is a separator, not executing
                }
            } else {
                // Index out of bounds
            }
        } else {
            // No filtered commands available
        }
    }
    
    /// Open the command editor
    fn open_command_editor_impl(&mut self) {
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
    fn handle_add_alias_impl(&mut self) {
        let state = crate::core::state::load_state();
        crate::utils::detailed_log("ADD_ALIAS", &format!("=== ADD ALIAS TRIGGERED ==="));
        crate::utils::detailed_log("ADD_ALIAS", &format!("Last executed command from state: {:?}", state.last_executed_command));
        if let Some(last_command) = state.last_executed_command {
            let search_text = self.popup_state.search_text.clone();
            
            // Create a new command with alias action
            let alias_command = crate::core::Command {
                patch: String::new(),
                command: search_text.clone(),
                action: "alias".to_string(),
                arg: last_command,
                flags: String::new(),
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
    
    fn edit_active_command_impl(&mut self) {
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
    
    fn edit_input_command_impl(&mut self) {
        // Create a new blank command with the input text as the name
        let input_text = self.popup_state.search_text.clone();
        
        if !input_text.is_empty() {
            // Create a new command with the input as the name
            let new_command = crate::core::Command {
                command: input_text.clone(),
                action: String::new(),  // Blank action for user to fill
                arg: String::new(),     // Blank arg for user to fill
                patch: String::new(),   // Blank patch
                flags: String::new(),   // No flags
            };
            
            // Open the command editor with this new command
            self.command_editor.edit_command(Some(&new_command), &input_text);
        }
    }
    
    fn handle_link_to_clipboard_impl(&mut self) {
        if !self.filtered_commands().is_empty() {
            let (display_commands, _is_submenu, _menu_prefix, _inside_count) = self.get_display_commands();
            
            if self.selected_index() < display_commands.len() {
                let selected_cmd = &display_commands[self.selected_index()];
                
                // Don't copy link if it's a separator
                if !PopupState::is_separator_command(selected_cmd) {
                    // Use launcher to execute the link_to_clipboard action
                    let link_action = crate::execute::make_action("link_to_clipboard", &selected_cmd.command);
                    // Execute the action
                    let _ = crate::execute::execute_on_server(&link_action);
                }
            }
        }
    }
    
    /// Handle uninstall app request
    fn handle_uninstall_app_impl(&mut self) {
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
    
    /// Show all available key bindings in a dialog
    fn show_keys_dialog_impl(&mut self) {
        let config = crate::core::sys_data::get_config();
        let mut key_lines = vec![];
        
        // Dialog title
        key_lines.push("=Key Bindings".to_string());
        key_lines.push("#Available keyboard shortcuts:".to_string());
        key_lines.push("".to_string()); // Empty line for spacing
        
        // Collect all key bindings with proper formatting
        let mut formatted_lines = vec![];
        
        // Get actions with keyboard bindings from the unified actions system
        if let Some(ref actions) = config.actions {
            // Separate actions by type for organized display
            let mut popup_actions = vec![];
            let mut template_actions = vec![];
            let mut other_actions = vec![];
            
            // Sort actions by key for consistent display
            for (name, action) in actions {
                if let Some(key) = action.key() {
                    let desc = action.description()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| name.clone());
                    
                    let entry = (key.to_string(), desc);
                    
                    match action.action_type() {
                        "popup" => popup_actions.push(entry),
                        "template" => template_actions.push(entry),
                        _ => other_actions.push(entry),
                    }
                }
            }
            
            // Sort each group
            popup_actions.sort_by(|a, b| a.0.cmp(&b.0));
            template_actions.sort_by(|a, b| a.0.cmp(&b.0));
            other_actions.sort_by(|a, b| a.0.cmp(&b.0));
            
            // Add popup actions
            if !popup_actions.is_empty() {
                formatted_lines.push(("Navigation & Control:".to_string(), "".to_string()));
                for entry in popup_actions {
                    formatted_lines.push(entry);
                }
            }
            
            // Add template actions
            if !template_actions.is_empty() {
                if !formatted_lines.is_empty() {
                    formatted_lines.push(("".to_string(), "".to_string())); // Empty line
                }
                formatted_lines.push(("Templates:".to_string(), "".to_string()));
                for entry in template_actions {
                    formatted_lines.push(entry);
                }
            }
            
            // Add other actions
            if !other_actions.is_empty() {
                if !formatted_lines.is_empty() {
                    formatted_lines.push(("".to_string(), "".to_string())); // Empty line
                }
                formatted_lines.push(("Other Actions:".to_string(), "".to_string()));
                for entry in other_actions {
                    formatted_lines.push(entry);
                }
            }
        }
        
        // Build the properly formatted content using TextBox for monospace alignment
        let mut content = String::new();
        
        // Find the maximum key width for proper alignment
        let max_key_width = formatted_lines.iter()
            .map(|(key, _)| key.len())
            .max()
            .unwrap_or(10)
            .max(10); // Minimum width of 10
        
        for (key, desc) in formatted_lines {
            if key.is_empty() && desc.is_empty() {
                content.push('\n'); // Empty line
            } else if desc.is_empty() {
                // Section header
                content.push_str(&format!("\n{}\n", key));
            } else {
                // Regular key binding - use proper spacing for alignment
                content.push_str(&format!("  {:<width$}  {}\n", key, desc, width = max_key_width));
            }
        }
        
        // Remove trailing newline
        if content.ends_with('\n') {
            content.pop();
        }
        
        // Use TextBox for monospaced display with proper alignment
        key_lines.push(format!("&{}", content));
        
        // Add some spacing before the button
        key_lines.push("".to_string());
        key_lines.push("".to_string());
        
        // Add OK button
        key_lines.push("!OK".to_string());
        
        // Show the dialog
        self.dialog.show(key_lines);
    }
    
    fn handle_template_create_impl(&mut self) {
        self.handle_template_create_named("default");
    }
    
    fn handle_template_create_named_impl(&mut self, template_name: &str) {
        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: === ENTER handle_template_create_named_impl('{}') ===", template_name));
        use crate::core::template_creation::TemplateContext;
        
        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 0 - About to get current context"));
        // Get the current context
        let input = self.popup_state.search_text.clone();
        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 0.1 - input = '{}'", input));
        let selected_command = if !self.filtered_commands().is_empty() {
            let (display_commands, _, _, _) = self.get_display_commands();
            if self.selected_index() < display_commands.len() {
                Some(display_commands[self.selected_index()].clone())
            } else {
                None
            }
        } else {
            None
        };
        
        // Get previous command from state
        let state = crate::core::state::load_state();
        crate::utils::detailed_log("TEMPLATE", &format!("=== TEMPLATE CREATION: '{}' ===", template_name));
        crate::utils::detailed_log("TEMPLATE", &format!("State loaded - last_executed_command: {:?}", state.last_executed_command));
        
        // Load all commands to find previous command, not just filtered ones
        let all_commands = crate::core::sys_data::get_sys_data().commands;
        
        let previous_command = state.last_executed_command.as_ref()
            .and_then(|name| {
                crate::utils::detailed_log("TEMPLATE", &format!("Looking for previous command: '{}'", name));
                let found = all_commands.iter().find(|c| c.command == *name).cloned();
                if found.is_some() {
                    crate::utils::detailed_log("TEMPLATE", &format!("Found previous command: '{}'", name));
                } else {
                    crate::utils::detailed_log("TEMPLATE", &format!("Previous command '{}' not found in {} available commands", name, all_commands.len()));
                }
                found
            });
        
        
        // Create template context
        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 1 - About to create TemplateContext"));
        let context = TemplateContext::new(&input, selected_command.as_ref(), previous_command.as_ref());
        
        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 2 - TemplateContext created, about to get config"));
        // Get the specified template/action
        let config = crate::core::sys_data::get_config();
        
        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 3 - Config retrieved successfully"));
        // First try to find in unified actions
        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: Looking for action '{}'", template_name));
        let found_action = if let Some(ref actions) = config.actions {
            crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 4 - config.actions is Some, Found {} actions in config", actions.len()));
            // Debug: log all action names
            let action_names: Vec<String> = actions.keys().cloned().collect();
            crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: Available actions: {:?}", action_names));
            if let Some(action) = actions.get(template_name) {
                crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 5 - Found action '{}', type='{}'", template_name, action.action_type()));
                if action.action_type() == "template" {
                    crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 6 - Action type is 'template'"));
                    // Extract grab parameter if present
                    crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 7 - About to extract grab parameter"));
                    let grab_seconds = action.params.get("grab")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);
                    
                    crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 8 - grab_seconds = {:?}", grab_seconds));
                    if let Some(grab_secs) = grab_seconds {
                        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 9 - grab_secs = {}", grab_secs));
                        // Store template info for after grab completes
                        crate::utils::detailed_log("GRAB", &format!("GRAB: Setting pending_template to '{}' with context", template_name));
                        self.pending_template = Some((template_name.to_string(), context));
                        crate::utils::detailed_log("GRAB", &format!("GRAB: pending_template is now Some"));
                        
                        // Start countdown
                        self.grabber_countdown = Some(grab_secs as u8);
                        self.countdown_last_update = Some(std::time::Instant::now());
                        crate::utils::detailed_log("GRAB", &format!("GRAB: Started countdown for template '{}' with {} seconds", template_name, grab_secs));
                        return; // Early return for grab templates
                    }
                    
                    // For non-grab template actions, we need to convert to old Template format
                    // This is a temporary compatibility layer
                    true
                } else {
                    let error_msg = format!("ERROR: Action '{}' exists but is not a template (type='{}').\nTo use this key binding, rename your template action to avoid name collision.", 
                        template_name, action.action_type());
                    crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: {}", error_msg));
                    self.show_error_dialog(&error_msg);
                    false
                }
            } else {
                crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: Action '{}' not found in actions", template_name));
                // Don't show error here - it might be in old templates
                false
            }
        } else {
            false
        };
        
        // Fall back to old templates system if not found in actions
        if !found_action {
            if let Some(ref templates) = config.templates {
                if let Some(template) = templates.get(template_name) {
                    // Check if this template requires grab functionality
                    if let Some(grab_seconds) = template.grab {
                        // Store template info for after grab completes
                        self.pending_template = Some((template_name.to_string(), context));
                        
                        // Start countdown
                        self.grabber_countdown = Some(grab_seconds as u8);
                        self.countdown_last_update = Some(std::time::Instant::now());
                    } else {
                        // Check for existing command if use_existing is true
                        let existing_command = if template.use_existing {
                            // Look for a command with the same name (case-insensitive)
                            let expanded_name = context.expand(&template.name);
                            self.popup_state.commands.iter().find(|cmd| 
                                cmd.command.eq_ignore_ascii_case(&expanded_name)
                            ).cloned()
                        } else {
                            None
                        };
                        
                        if let Some(existing) = existing_command {
                            // Edit the existing command
                            if template.edit {
                                self.command_editor.edit_command(Some(&existing), &existing.command);
                                // Don't clear search text when opening editor - preserve input
                            }
                            // If edit is false with use_existing, do nothing (command already exists)
                        } else {
                            // Process template to create new command
                            match crate::core::template_creation::process_template(template, &context, &config) {
                                Ok(new_command) => {
                                    if template.edit {
                                        // Open command editor with the prefilled command
                                        self.command_editor.edit_command(Some(&new_command), &self.popup_state.search_text);
                                        // Store template for post-save file creation
                                        self.command_editor.set_pending_template(template.clone(), context.clone());
                                        // Don't clear search text when opening editor - preserve input
                                } else {
                                    // Add the new command directly
                                    match crate::core::commands::add_command(new_command, &mut self.popup_state.commands) {
                                        Ok(_) => {
                                            // Save commands to file
                                            if let Err(e) = crate::core::commands::save_commands_to_file(&self.popup_state.commands) {
                                                crate::utils::log_error(&format!("Failed to save commands: {}", e));
                                            }
                                            // Clear search and update display
                                            self.popup_state.search_text.clear();
                                            self.popup_state.update_search(String::new());
                                            
                                            // Trigger rescan if requested
                                            if template.file_rescan {
                                                self.trigger_rescan();
                                            }
                                        }
                                        Err(e) => {
                                            self.show_error_dialog(&format!("Failed to add command: {}", e));
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                self.show_error_dialog(&format!("Failed to create command from '{}' template: {}", template_name, e));
                            }
                        }
                        }
                    }
                    return; // Early return after processing old template
                }
            }
            self.show_error_dialog(&format!("Template/Action '{}' not found in configuration", template_name));
        } else {
            // Process the unified action as a template
            crate::utils::detailed_log("SYSTEM", &format!("Processing unified action template: {}", template_name));
            
            // Convert unified action to template and process it
            if let Some(ref actions) = config.actions {
                if let Some(action) = actions.get(template_name) {
                    if let Some(template) = action.to_template() {
                        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: Converted action to template, edit={}", template.edit));
                        
                        // Check for existing command if use_existing is true
                        let existing_command = if template.use_existing {
                            let expanded_name = context.expand(&template.name);
                            self.popup_state.commands.iter().find(|cmd| 
                                cmd.command.eq_ignore_ascii_case(&expanded_name)
                            ).cloned()
                        } else {
                            None
                        };
                        
                        if let Some(existing) = existing_command {
                            // Edit the existing command
                            if template.edit {
                                self.command_editor.edit_command(Some(&existing), &existing.command);
                                // Don't clear search text when opening editor - preserve input
                            }
                        } else {
                            // Process template to create new command
                            match crate::core::template_creation::process_template(&template, &context, &config) {
                                Ok(new_command) => {
                                    if template.edit {
                                        crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: Opening command editor for sub_anchor template"));
                                        // Open command editor with the prefilled command
                                        self.command_editor.edit_command(Some(&new_command), &self.popup_state.search_text);
                                        // Store template for post-save file creation
                                        self.command_editor.set_pending_template(template.clone(), context.clone());
                                        // Don't clear search text when opening editor - preserve input
                                } else {
                                    // Add the new command directly
                                    match crate::core::commands::add_command(new_command, &mut self.popup_state.commands) {
                                        Ok(_) => {
                                            // Save commands to file
                                            if let Err(e) = crate::core::commands::save_commands_to_file(&self.popup_state.commands) {
                                                crate::utils::log_error(&format!("Failed to save commands: {}", e));
                                            }
                                            // Clear search and update display
                                            self.popup_state.search_text.clear();
                                            self.popup_state.update_search(String::new());
                                            
                                            // Trigger rescan if requested
                                            if template.file_rescan {
                                                self.trigger_rescan();
                                            }
                                        }
                                        Err(e) => {
                                            self.show_error_dialog(&format!("Failed to add command: {}", e));
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                self.show_error_dialog(&format!("Failed to create command from '{}' template: {}", template_name, e));
                            }
                        }
                        }
                    } else {
                        // Not a template action, execute directly
                        // Convert HashMap<String, String> to HashMap<String, JsonValue>
                        let json_variables: std::collections::HashMap<String, serde_json::Value> = 
                            context.variables.iter()
                                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                                .collect();
                        match crate::execute::execute_on_server_with_parameters(
                            action, 
                            None,  // No primary arg for templates
                            Some(json_variables)
                        ) {
                            Ok(result) => {
                                crate::utils::detailed_log("SYSTEM", &format!("Template action completed: {}", result));
                                // Clear search and update display
                                self.popup_state.search_text.clear();
                                self.popup_state.update_search(String::new());
                            }
                            Err(e) => {
                                self.show_error_dialog(&format!("Failed to execute template action '{}': {}", template_name, e));
                            }
                        }
                    }
                }
            }
        }
    }
    
    // =============================================================================
    // Initialization
    // =============================================================================
    
    /// Redirect stdout and stderr to the anchor log file for centralized debugging
    fn setup_log_redirection() {
        use std::fs::OpenOptions;
        use std::os::unix::io::AsRawFd;
        use std::sync::OnceLock;
        
        static LOG_REDIRECT_SETUP: OnceLock<()> = OnceLock::new();
        
        // Only set up redirection once
        LOG_REDIRECT_SETUP.get_or_init(|| {
            if let Some(home_dir) = dirs::home_dir() {
                let config_dir = home_dir.join(".config").join("hookanchor");
                let log_path = config_dir.join("anchor.log");
                
                // Open log file in append mode
                if let Ok(log_file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_path) {
                    
                    let log_fd = log_file.as_raw_fd();
                    
                    // Add a separator to the log
                    if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open(&log_path) {
                        use std::io::Write;
                        let _ = writeln!(f, "\n=== GUI SESSION START {} ===", 
                                       chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
                    }
                    
                    unsafe {
                        // Redirect stdout (fd 1) and stderr (fd 2) to log file
                        libc::dup2(log_fd, 1);
                        libc::dup2(log_fd, 2);
                    }
                    
                    // Force flush stdout/stderr buffers to ensure redirection works
                    let _ = std::io::Write::flush(&mut std::io::stdout());
                    let _ = std::io::Write::flush(&mut std::io::stderr());
                    
                    // Output redirection setup completed
                }
            }
        });
    }
    
    pub fn new() -> Self {
        Self::new_with_prompt("")
    }
    
    pub fn new_with_prompt(initial_prompt: &str) -> Self {
        let _startup_time = std::time::Instant::now();
        
        // Redirect stdout/stderr to anchor log for centralized debugging
        Self::setup_log_redirection();
        
        // Load only app state for window positioning - this is fast
        let state = load_state();
        
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
            was_auto_positioned: false,
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
            request_focus: false,
            frame_count: 0,
            window_activated: false,
            config_error: None,
            last_interaction_time: std::time::Instant::now(),
            loading_state: LoadingState::NotLoaded,
            pre_init_input_buffer: initial_prompt.to_string(),
            pending_template: None,
            key_registry: None, // Will be initialized when config is loaded
            exit_app_key: None, // Will be populated from config
            should_exit: false,
            is_hidden: false,
            pending_rebuild: false,
            pending_action: None,
        };
        
        result
    }
    
    /// Start loading data in the background after UI is shown
    fn start_deferred_loading(&mut self) {
        crate::utils::detailed_log("DEFERRED_LOADING", &format!("DEFERRED_LOADING: start_deferred_loading called"));
        if self.loading_state != LoadingState::NotLoaded {
            crate::utils::detailed_log("DEFERRED_LOADING", &format!("DEFERRED_LOADING: Already loading or loaded, state={:?}", self.loading_state));
            return; // Already loading or loaded
        }
        
        crate::utils::detailed_log("DEFERRED_LOADING", &format!("DEFERRED_LOADING: Starting to load"));
        self.loading_state = LoadingState::Loading;
        let start_time = std::time::Instant::now();
        
        // Load commands from disk only (no filesystem scanning to avoid UI blocking)
        let commands = crate::core::commands::load_commands_raw();
        let (config, config_error) = match load_config_with_error() {
            ConfigResult::Success(config) => (config, None),
            ConfigResult::Error(error) => (crate::core::sys_data::get_config(), Some(error)),
        };
        
        // Initialize key registry with the loaded config
        self.key_registry = Some(create_default_key_registry(&config));
        
        // Populate exit keystrokes from config
        if let Some(ref keybindings) = config.keybindings {
            use crate::core::key_processing::Keystroke;
            
            // Debug log all keybindings
            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
                use std::io::Write;
                let _ = writeln!(file, "🔧 CONFIG: Found {} keybindings", keybindings.len());
            }
            
            if let Some(exit_app_str) = keybindings.get("exit_app") {
                match Keystroke::from_key_string(exit_app_str) {
                    Ok(keystroke) => {
                        self.exit_app_key = Some(keystroke);
                        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
                            use std::io::Write;
                            let _ = writeln!(file, "🔧 CONFIG: exit_app = '{}' -> {:?}", exit_app_str, self.exit_app_key.as_ref().unwrap().key);
                        }
                    }
                    Err(e) => {
                        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
                            use std::io::Write;
                            let _ = writeln!(file, "🔧 CONFIG: ERROR parsing exit_app '{}': {}", exit_app_str, e);
                        }
                    }
                }
            }
            
            
        } else {
            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
                use std::io::Write;
                let _ = writeln!(file, "🔧 CONFIG: No keybindings section found in config!");
            }
        }
        
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
        let _elapsed = start_time.elapsed();
        
        // Debug log
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
            use std::io::Write;
            let _ = writeln!(file, "📦 DEFERRED LOADING: Complete!");
        }
        
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

    // =============================================================================
    // Action System - General UI Action Queue
    // =============================================================================

    /// Queue an action to be executed after user confirms via dialog or other UI component
    pub fn queue_action(
        &mut self,
        context: HashMap<String, String>,
        callback: Box<dyn FnOnce(&HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>>>,
    ) {
        crate::utils::log(&format!("ACTION: Queueing action with {} context keys", context.len()));
        self.pending_action = Some(PendingAction { context, callback });
    }

    /// Complete the pending action by merging user input and executing the callback
    pub fn complete_action(&mut self, user_input: HashMap<String, String>) {
        if let Some(action) = self.pending_action.take() {
            crate::utils::log(&format!("ACTION: Completing action with {} user inputs", user_input.len()));
            
            // Merge action context with user input
            let mut final_context = action.context;
            final_context.extend(user_input);
            
            // Check if this is a rename operation that needs special handling
            if final_context.get("operation") == Some(&"rename".to_string()) {
                let empty_string = String::new();
                let exit_button = final_context.get("exit").unwrap_or(&empty_string);
                if exit_button == "OK" {
                    // User confirmed rename - execute it with access to self
                    match self.execute_rename_with_ui_update(&final_context) {
                        Ok(()) => {
                            crate::utils::log("ACTION: Rename action executed successfully");
                        }
                        Err(e) => {
                            let error_msg = format!("Rename action failed: {}", e);
                            crate::utils::log_error(&error_msg);
                            self.show_error_dialog(&error_msg);
                        }
                    }
                }
                // If Cancel or Escape, do nothing
            } else {
                // Execute the regular callback with merged context
                match (action.callback)(&final_context) {
                    Ok(()) => {
                        crate::utils::log("ACTION: Action executed successfully");
                    }
                    Err(e) => {
                        let error_msg = format!("Action failed: {}", e);
                        crate::utils::log_error(&error_msg);
                        self.show_error_dialog(&error_msg);
                    }
                }
            }
        }
    }

    /// Cancel the pending action without executing it
    pub fn cancel_action(&mut self) {
        if self.pending_action.is_some() {
            crate::utils::log("ACTION: Cancelling pending action");
            self.pending_action = None;
        }
    }

    /// Check if there is a pending action waiting for user input
    pub fn has_pending_action(&self) -> bool {
        self.pending_action.is_some()
    }

    /// Execute a rename operation with UI update - has access to &mut self unlike static version
    fn execute_rename_with_ui_update(&mut self, context: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let old_name = context.get("old_name").ok_or("Missing old_name")?;
        let new_name = context.get("new_name").ok_or("Missing new_name")?;
        let current_arg = context.get("current_arg").ok_or("Missing current_arg")?;
        let action = context.get("action").ok_or("Missing action")?;
        let original_command_to_delete = context.get("original_command_to_delete");

        // Get the new command details from context
        let new_command_action = context.get("new_command_action").ok_or("Missing new_command_action")?;
        let new_command_arg = context.get("new_command_arg").ok_or("Missing new_command_arg")?;
        let new_command_patch = context.get("new_command_patch").ok_or("Missing new_command_patch")?;
        let new_command_flags = context.get("new_command_flags").ok_or("Missing new_command_flags")?;

        let config = crate::core::sys_data::get_config();

        // Execute the rename with dry_run = false on UI commands
        use crate::core::commands::rename_associated_data;
        let mut patches = crate::core::commands::create_patches_hashmap(&self.commands());
        let (updated_arg, _actions) = rename_associated_data(
            old_name,
            new_name,
            current_arg,
            action,
            self.commands_mut(),
            &mut patches,
            &config,
            false, // dry_run = false
        )?;

        // Delete original command from UI if needed
        if let Some(cmd_name) = original_command_to_delete {
            if !cmd_name.is_empty() {
                use crate::core::commands::delete_command;
                let _ = delete_command(cmd_name, self.commands_mut());
            }
        }

        // Create the new command with potentially updated arg
        let new_command = crate::core::Command {
            command: new_name.clone(),
            action: new_command_action.clone(),
            arg: updated_arg, // Use the updated arg from rename operation
            patch: new_command_patch.clone(),
            flags: new_command_flags.clone(),
        };

        // Add the new command to UI
        use crate::core::commands::{add_command, save_commands_to_file};
        let _ = add_command(new_command, self.commands_mut());

        // Save commands to file
        save_commands_to_file(&self.commands())?;

        // Update the filtered list if we're currently filtering
        if !self.popup_state.search_text.trim().is_empty() {
            // Refresh the search with updated commands
            let current_search = self.popup_state.search_text.clone();
            self.popup_state.update_search(current_search);
        }

        crate::utils::log(&format!("RENAME: Successfully renamed '{}' to '{}' with side effects and UI update", old_name, new_name));
        Ok(())
    }

    /// Execute a rename operation from the action context (static version for non-UI contexts)
    fn execute_rename_operation(context: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let old_name = context.get("old_name").ok_or("Missing old_name")?;
        let new_name = context.get("new_name").ok_or("Missing new_name")?;
        let current_arg = context.get("current_arg").ok_or("Missing current_arg")?;
        let action = context.get("action").ok_or("Missing action")?;
        let original_command_to_delete = context.get("original_command_to_delete");

        // Get the new command details from context
        let new_command_action = context.get("new_command_action").ok_or("Missing new_command_action")?;
        let new_command_arg = context.get("new_command_arg").ok_or("Missing new_command_arg")?;
        let new_command_patch = context.get("new_command_patch").ok_or("Missing new_command_patch")?;
        let new_command_flags = context.get("new_command_flags").ok_or("Missing new_command_flags")?;

        // Load current state (this is a static method, so we need to load fresh)
        let mut commands = crate::core::commands::load_commands();
        let mut patches = crate::core::commands::create_patches_hashmap(&commands);
        let config = crate::core::sys_data::get_config();

        // Execute the rename with dry_run = false
        use crate::core::commands::rename_associated_data;
        let (updated_arg, _actions) = rename_associated_data(
            old_name,
            new_name,
            current_arg,
            action,
            &mut commands,
            &mut patches,
            &config,
            false, // dry_run = false
        )?;

        // Delete original command if needed
        if let Some(cmd_name) = original_command_to_delete {
            if !cmd_name.is_empty() {
                use crate::core::commands::delete_command;
                let _ = delete_command(cmd_name, &mut commands);
            }
        }

        // Create the new command with potentially updated arg
        let new_command = crate::core::Command {
            command: new_name.clone(),
            action: new_command_action.clone(),
            arg: updated_arg, // Use the updated arg from rename operation
            patch: new_command_patch.clone(),
            flags: new_command_flags.clone(),
        };

        // Add the new command
        use crate::core::commands::{add_command, save_commands_to_file};
        let _ = add_command(new_command, &mut commands);

        // Save commands to file (patches are embedded in commands, so no separate save needed)
        save_commands_to_file(&commands)?;

        crate::utils::log(&format!("RENAME: Successfully renamed '{}' to '{}' with side effects", old_name, new_name));
        Ok(())
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
        
        crate::utils::detailed_log("GRAB", &format!("Starting grabber countdown from {} (flip_focus={})", 
            countdown_seconds, flip_focus));
            
        self.grabber_countdown = Some(countdown_seconds);
        self.countdown_last_update = Some(std::time::Instant::now());
        // Note: Don't call ctx.request_repaint_after() as it causes UI lockup
        // The UI update loop handles repaints automatically
    }
    
    /// Update countdown and handle grabber logic
    fn update_grabber_countdown(&mut self, ctx: &egui::Context) {
        if let Some(count) = self.grabber_countdown {
            crate::utils::verbose_log("GRAB", &format!("update_grabber_countdown: count={}", count));
            if let Some(last_update) = self.countdown_last_update {
                let elapsed = last_update.elapsed();
                crate::utils::verbose_log("GRAB", &format!("Elapsed: {}ms", elapsed.as_millis()));
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
                        crate::utils::log("GRAB: Countdown reached 0, calling execute_grab");
                        self.execute_grab(ctx);
                        self.grabber_countdown = None;
                        self.countdown_last_update = None;
                    }
                }
            }
        }
    }
    
    /// Flip focus away from HookAnchor (used when flip_focus is enabled)
    fn flip_focus_away(&self) {
        
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
            crate::utils::log_error(&format!("Failed to flip focus away: {}", e));
        }
    }
    
    /// Execute the grab operation - simplified synchronous version
    fn execute_grab(&mut self, _ctx: &egui::Context) {
        crate::utils::log("GRAB: execute_grab() called");
        let config = crate::core::sys_data::get_config();
        
        // Check if we should flip focus
        let flip_focus = config.launcher_settings
            .as_ref()
            .and_then(|ls| ls.flip_focus)
            .unwrap_or(false);
        
        
        if flip_focus {
            // Focus was already flipped during countdown, just wait briefly
            std::thread::sleep(std::time::Duration::from_millis(200));
        } else {
            // User is responsible for changing focus manually during countdown
        }
        
        // Now capture from the focused application
        crate::utils::detailed_log("GRAB", &format!("GRAB: Calling crate::systems::grabber::grab, pending_template={:?}", 
            self.pending_template.as_ref().map(|(name, _)| name)));
        match crate::systems::grabber::grab(&config) {
            Ok(grab_result) => {
                crate::utils::log(&format!("GRAB: crate::systems::grabber::grab returned result"));
                match grab_result {
                    crate::systems::grabber::GrabResult::RuleMatched(rule_name, mut command) => {
                        crate::utils::log(&format!("GRAB: RuleMatched - rule='{}', command.action='{}', command.arg='{}'", rule_name, command.action, command.arg));
                        
                        // Check if we have a pending template
                        if let Some((template_name, mut context)) = self.pending_template.take() {
                            crate::utils::detailed_log("GRAB", &format!("GRAB: Found pending template '{}' - will process it", template_name));
                            
                            // Add grabbed variables to template context
                            context.add_variable("grabbed_action".to_string(), command.action.clone());
                            context.add_variable("grabbed_arg".to_string(), command.arg.clone());
                            context.add_variable("grabbed_rule".to_string(), rule_name.clone());
                            
                            // Process the template with grabbed context
                            let config = crate::core::sys_data::get_config();
                            
                            // Try to find template in unified actions first
                            let template_found = if let Some(ref actions) = config.actions {
                                if let Some(action) = actions.get(&template_name) {
                                    crate::utils::detailed_log("GRAB", &format!("GRAB: Found action '{}' in unified actions", template_name));
                                    if action.action_type() == "template" {
                                        crate::utils::detailed_log("GRAB", &format!("GRAB: Action type is 'template'"));
                                        // Convert action to Template for processing
                                        if let Some(template) = action.to_template() {
                                            crate::utils::detailed_log("GRAB", &format!("GRAB: Successfully converted action to template"));
                                            crate::utils::detailed_log("GRAB", &format!("GRAB: Calling process_template"));
                                            match crate::core::template_creation::process_template(&template, &context, &config) {
                                                Ok(new_command) => {
                                                    crate::utils::detailed_log("GRAB", &format!("GRAB: process_template succeeded, command='{}'\n", new_command.command));
                                                    // Check edit flag from action params
                                                    let should_edit = action.params.get("edit")
                                                        .and_then(|v| v.as_bool())
                                                        .unwrap_or(false);
                                                    
                                                    crate::utils::detailed_log("GRAB", &format!("GRAB: Template created command '{}', should_edit={}", 
                                                        new_command.command, should_edit));
                                                    
                                                    if should_edit {
                                                        crate::utils::detailed_log("GRAB", &format!("GRAB: Opening command editor for template (should_edit=true)"));
                                                        crate::utils::log(&format!("GRAB: command_editor.visible before={}", self.command_editor.visible));
                                                        self.command_editor.edit_command(Some(&new_command), &self.popup_state.search_text);
                                                        crate::utils::log(&format!("GRAB: command_editor.visible after={}", self.command_editor.visible));
                                                    } else {
                                                        crate::utils::detailed_log("GRAB", &format!("GRAB: Not opening editor (should_edit=false)"));
                                                        // Add command directly
                                                        match crate::core::commands::add_command(new_command, &mut self.popup_state.commands) {
                                                            Ok(_) => {
                                                                if let Err(e) = crate::core::commands::save_commands_to_file(&self.popup_state.commands) {
                                                                    crate::utils::log_error(&format!("Failed to save commands: {}", e));
                                                                }
                                                                self.popup_state.search_text.clear();
                                                                self.popup_state.update_search(String::new());
                                                                
                                                                // Trigger rescan if requested
                                                                let file_rescan = action.params.get("file_rescan")
                                                                    .and_then(|v| v.as_bool())
                                                                    .unwrap_or(false);
                                                                if file_rescan {
                                                                    self.trigger_rescan();
                                                                }
                                                            }
                                                            Err(e) => {
                                                                self.show_error_dialog(&format!("Failed to add command: {}", e));
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    crate::utils::detailed_log("GRAB", &format!("GRAB: process_template failed: {}", e));
                                                    self.show_error_dialog(&format!("Failed to create command from template: {}", e));
                                                }
                                            }
                                            true
                                        } else {
                                            crate::utils::detailed_log("GRAB", &format!("GRAB: Failed to convert action '{}' to template", template_name));
                                            false
                                        }
                                    } else {
                                        crate::utils::detailed_log("GRAB", &format!("GRAB: Action '{}' is not a template type (type={})", template_name, action.action_type()));
                                        false
                                    }
                                } else {
                                    crate::utils::detailed_log("GRAB", &format!("GRAB: Action '{}' not found in unified actions", template_name));
                                    false
                                }
                            } else {
                                crate::utils::log("GRAB: No unified actions configured");
                                false
                            };
                            
                            // Fall back to old templates if not found in actions
                            if !template_found {
                                crate::utils::detailed_log("GRAB", &format!("GRAB: Template '{}' not found in actions, checking old templates", template_name));
                                if let Some(ref templates) = config.templates {
                                    if let Some(template) = templates.get(&template_name) {
                                        match crate::core::template_creation::process_template(template, &context, &config) {
                                            Ok(new_command) => {
                                                if template.edit {
                                                    self.command_editor.edit_command(Some(&new_command), &self.popup_state.search_text);
                                                } else {
                                                    // Add command directly
                                                    match crate::core::commands::add_command(new_command, &mut self.popup_state.commands) {
                                                        Ok(_) => {
                                                            if let Err(e) = crate::core::commands::save_commands_to_file(&self.popup_state.commands) {
                                                                crate::utils::log_error(&format!("Failed to save commands: {}", e));
                                                            }
                                                            self.popup_state.search_text.clear();
                                                            self.popup_state.update_search(String::new());
                                                            
                                                            // Trigger rescan if requested
                                                            if template.file_rescan {
                                                                self.trigger_rescan();
                                                            }
                                                        }
                                                        Err(e) => {
                                                            self.show_error_dialog(&format!("Failed to add command: {}", e));
                                                        }
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                self.show_error_dialog(&format!("Failed to create command from template: {}", e));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            crate::utils::detailed_log("GRAB", &format!("GRAB: No pending template - using normal grab behavior"));
                            // Normal grab behavior (no template)
                            // Use the current search text as the command name, or default if empty
                            let command_name = if self.popup_state.search_text.trim().is_empty() {
                                format!("Grabbed {}", rule_name)
                            } else {
                                self.popup_state.search_text.clone()
                            };
                            
                            // Fill in the command with all the grabbed information
                            command.command = command_name;
                            
                            // Open command editor with the pre-filled grabbed command
                            self.command_editor.open_with_command(command);
                        }
                    }
                    crate::systems::grabber::GrabResult::NoRuleMatched(grab_context) => {
                        // Check if we have a pending template
                        if let Some((template_name, mut context)) = self.pending_template.take() {
                            
                            // Add generic grabbed variables when no rule matched
                            context.add_variable("grabbed_action".to_string(), "app".to_string());
                            context.add_variable("grabbed_arg".to_string(), grab_context.app_name.clone());
                            context.add_variable("grabbed_rule".to_string(), "NoRuleMatched".to_string());
                            
                            // Process the template
                            let config = crate::core::sys_data::get_config();
                            
                            // Try to find template in unified actions first
                            let template_found = if let Some(ref actions) = config.actions {
                                if let Some(action) = actions.get(&template_name) {
                                    if action.action_type() == "template" {
                                        crate::utils::detailed_log("GRAB", &format!("GRAB: Action type is 'template'"));
                                        // Convert action to Template for processing
                                        if let Some(template) = action.to_template() {
                                            crate::utils::detailed_log("GRAB", &format!("GRAB: Successfully converted action to template"));
                                            crate::utils::detailed_log("GRAB", &format!("GRAB: Calling process_template"));
                                            match crate::core::template_creation::process_template(&template, &context, &config) {
                                                Ok(new_command) => {
                                                    crate::utils::detailed_log("GRAB", &format!("GRAB: process_template succeeded, command='{}'\n", new_command.command));
                                                    // Check edit flag from action params
                                                    let should_edit = action.params.get("edit")
                                                        .and_then(|v| v.as_bool())
                                                        .unwrap_or(false);
                                                    
                                                    crate::utils::detailed_log("GRAB", &format!("GRAB: Template created command '{}', should_edit={}", 
                                                        new_command.command, should_edit));
                                                    
                                                    if should_edit {
                                                        crate::utils::detailed_log("GRAB", &format!("GRAB: Opening command editor for template (should_edit=true)"));
                                                        crate::utils::log(&format!("GRAB: command_editor.visible before={}", self.command_editor.visible));
                                                        self.command_editor.edit_command(Some(&new_command), &self.popup_state.search_text);
                                                        crate::utils::log(&format!("GRAB: command_editor.visible after={}", self.command_editor.visible));
                                                    } else {
                                                        crate::utils::detailed_log("GRAB", &format!("GRAB: Not opening editor (should_edit=false)"));
                                                        // Add command directly
                                                        match crate::core::commands::add_command(new_command, &mut self.popup_state.commands) {
                                                            Ok(_) => {
                                                                if let Err(e) = crate::core::commands::save_commands_to_file(&self.popup_state.commands) {
                                                                    crate::utils::log_error(&format!("Failed to save commands: {}", e));
                                                                }
                                                                self.popup_state.search_text.clear();
                                                                self.popup_state.update_search(String::new());
                                                                
                                                                // Trigger rescan if requested
                                                                let file_rescan = action.params.get("file_rescan")
                                                                    .and_then(|v| v.as_bool())
                                                                    .unwrap_or(false);
                                                                if file_rescan {
                                                                    self.trigger_rescan();
                                                                }
                                                            }
                                                            Err(e) => {
                                                                self.show_error_dialog(&format!("Failed to add command: {}", e));
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    crate::utils::detailed_log("GRAB", &format!("GRAB: process_template failed: {}", e));
                                                    self.show_error_dialog(&format!("Failed to create command from template: {}", e));
                                                }
                                            }
                                            true
                                        } else {
                                            crate::utils::detailed_log("GRAB", &format!("GRAB: Failed to convert action '{}' to template", template_name));
                                            false
                                        }
                                    } else {
                                        crate::utils::detailed_log("GRAB", &format!("GRAB: Action '{}' is not a template type (type={})", template_name, action.action_type()));
                                        false
                                    }
                                } else {
                                    crate::utils::detailed_log("GRAB", &format!("GRAB: Action '{}' not found in unified actions", template_name));
                                    false
                                }
                            } else {
                                crate::utils::log("GRAB: No unified actions configured");
                                false
                            };
                            
                            // Fall back to old templates if not found in actions
                            if !template_found {
                                crate::utils::detailed_log("GRAB", &format!("GRAB: Template '{}' not found in actions, checking old templates", template_name));
                                if let Some(ref templates) = config.templates {
                                    if let Some(template) = templates.get(&template_name) {
                                        match crate::core::template_creation::process_template(template, &context, &config) {
                                            Ok(new_command) => {
                                                if template.edit {
                                                    self.command_editor.edit_command(Some(&new_command), &self.popup_state.search_text);
                                                } else {
                                                    // Add command directly
                                                    match crate::core::commands::add_command(new_command, &mut self.popup_state.commands) {
                                                        Ok(_) => {
                                                            if let Err(e) = crate::core::commands::save_commands_to_file(&self.popup_state.commands) {
                                                                crate::utils::log_error(&format!("Failed to save commands: {}", e));
                                                            }
                                                            self.popup_state.search_text.clear();
                                                            self.popup_state.update_search(String::new());
                                                            
                                                            // Trigger rescan if requested
                                                            if template.file_rescan {
                                                                self.trigger_rescan();
                                                            }
                                                        }
                                                        Err(e) => {
                                                            self.show_error_dialog(&format!("Failed to add command: {}", e));
                                                        }
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                self.show_error_dialog(&format!("Failed to create command from template: {}", e));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Normal grab behavior (no template)
                            // Generate the template text
                            let template_text = crate::systems::grabber::generate_rule_template_text(&grab_context);
                            
                            // Show template dialog using the new TextBox field type
                            let dialog_spec = vec![
                                format!("=Grabber Rule Template - {}", grab_context.app_name),
                                format!("&{}", template_text),
                                "!OK".to_string(),
                            ];
                            
                            self.dialog.show(dialog_spec);
                        }
                    }
                }
            }
            Err(err) => {
                crate::utils::log_error(&format!("Grabber error: {}", err));
            }
        }
        
        // Check if we should restore HookAnchor visibility (only when flip_focus is enabled)
        let flip_focus_for_restore = config.launcher_settings
            .as_ref()
            .and_then(|ls| ls.flip_focus)
            .unwrap_or(false);
            
        if flip_focus_for_restore {
            // Restore HookAnchor visibility
            let restore_script = r#"
                tell application "System Events"
                    tell application process "popup"
                        set visible to true
                        set frontmost to true
                    end tell
                end tell
            "#;
            
            let _restore_start = std::time::Instant::now();
            if let Err(e) = std::process::Command::new("osascript")
                .arg("-e")
                .arg(restore_script)
                .output() {
                crate::utils::log_error(&format!("Failed to restore HookAnchor: {}", e));
            }
        } else {
        }
        
        // Regain focus back to anchor selector after grab operation (only if flip_focus is enabled)
        let flip_focus = config.launcher_settings
            .as_ref()
            .and_then(|ls| ls.flip_focus)
            .unwrap_or(false);
        
        let _focus_start = std::time::Instant::now();
        if flip_focus {
            // Give time for window restoration only when flip_focus is enabled
            std::thread::sleep(std::time::Duration::from_millis(200));
            
            if let Err(e) = self.regain_focus() {
                crate::utils::log_error(&format!("Failed to regain focus: {}", e));
            }
        } else {
        }
    }
    
    
    /// Regain focus back to anchor selector after grab operation
    fn regain_focus(&self) -> Result<(), String> {
        // Call the regain_focus function from config.js
        let js_code = "regain_focus({})";
        match crate::js::execute(js_code) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to execute regain_focus function: {}", e))
        }
    }
    
    /// Clean up the debug log file before starting a rescan
    
    /// Trigger rebuild: restart server and rescan filesystem (full reset)
    fn trigger_rebuild(&mut self, ctx: &egui::Context) {
        // Set the search text to show rebuilding status
        self.popup_state.search_text = "Rebuilding...".to_string();
        self.popup_state.update_search("Rebuilding...".to_string());
        
        // Force immediate repaint to show the "Rebuilding..." text
        ctx.request_repaint();
        
        // Schedule the actual rebuild work for the next frame
        // This allows the UI to update with "Rebuilding..." before we block
        self.pending_rebuild = true;
        return;
    }
    
    /// Perform the actual rebuild work (called after UI update)
    fn perform_rebuild(&mut self) {
        // Generate a unique build identifier (timestamp-based)
        let build_timestamp = chrono::Local::now();
        let build_id = build_timestamp.format("%Y%m%d_%H%M%S").to_string();
        
        // Log the rebuild header with timestamp and build ID
        crate::utils::log(&"=".repeat(80));
        crate::utils::detailed_log("SYSTEM", &format!("REBUILD SESSION: {}", build_id));
        crate::utils::log(&format!("TIMESTAMP: {}", build_timestamp.format("%Y-%m-%d %H:%M:%S%.3f")));
        crate::utils::log(&"=".repeat(80));
        
        println!("🏗️  HookAnchor Rebuild - Clean Server Restart");
        println!("===============================================");
        
        // Step 1 & 2: Restart the server (kill existing and start new)
        println!("\n🔄 Step 1&2/3: Restarting server...");
        let server_restart_success = match crate::execute::activate_command_server(true) {
            Ok(()) => {
                crate::utils::detailed_log("REBUILD", "Server restart completed successfully");
                println!("  ✅ Server restarted successfully");
                true
            }
            Err(e) => {
                crate::utils::detailed_log("REBUILD", &format!("Server restart failed: {}", e));
                println!("  ⚠️  Server restart failed: {}", e);
                println!("  ⚠️  Continuing with popup restart anyway...");
                false
            }
        };
        
        // Only do rescan if server restart succeeded
        if server_restart_success {
            // Brief wait for server to be ready
            println!("  ⏳ Waiting for server to initialize...");
            std::thread::sleep(std::time::Duration::from_millis(1000));
            
            // Step 2: Call "ha --rescan" to let the CLI handle the rescan
            println!("\n📁 Step 2/3: Triggering filesystem rescan via server...");
            // Use the ha binary for CLI operations, not popup_server
            let ha_binary = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|dir| dir.join("ha")))
                .unwrap_or_else(|| "ha".into());
            match std::process::Command::new(ha_binary)
                .arg("--rescan")
                .status() {
                Ok(status) => {
                    if status.success() {
                        crate::utils::detailed_log("REBUILD", "Rescan via server completed successfully");
                        println!("  ✅ Filesystem rescan completed");
                    } else {
                        crate::utils::detailed_log("REBUILD", "Rescan via server failed");
                        println!("  ❌ Filesystem rescan failed");
                    }
                }
                Err(e) => {
                    crate::utils::detailed_log("REBUILD", &format!("Failed to execute rescan: {}", e));
                    println!("  ❌ Failed to start rescan: {}", e);
                }
            }
        }
        
        // Step 3: Re-exec ourselves to pick up any new binary
        println!("\n🔄 Step 3/3: Restarting popup with fresh binary...");
        
        // Get our current executable path - this will re-exec whatever is currently at this path
        if let Ok(current_exe) = std::env::current_exe() {
            println!("  📦 Re-executing: {}", current_exe.display());
            crate::utils::detailed_log("REBUILD", &format!("Re-executing popup at: {}", current_exe.display()));
            
            // Launch the new popup instance (no args - it will run as normal GUI)
            match std::process::Command::new(&current_exe)
                .spawn() {
                Ok(_) => {
                    println!("  ✅ New popup instance launched");
                    crate::utils::detailed_log("REBUILD", "Successfully spawned new popup instance");
                    
                    // Exit this instance completely (not just hide)
                    println!("  👋 Exiting old popup instance...");
                    std::process::exit(0);
                }
                Err(e) => {
                    println!("  ❌ Failed to restart popup: {}", e);
                    crate::utils::detailed_log("REBUILD", &format!("Failed to re-exec popup: {}", e));
                }
            }
        } else {
            println!("  ⚠️  Could not determine executable path");
            crate::utils::detailed_log("REBUILD", "Could not get current executable path");
        }
        
        println!("\n🎉 Rebuild completed successfully!");
    }
    
    /// Trigger filesystem rescan
    fn trigger_rescan(&mut self) {
        crate::utils::log("Triggering filesystem rescan...");
        // Use the ha binary for CLI operations, not popup_server
        let ha_binary = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|dir| dir.join("ha")))
            .unwrap_or_else(|| "ha".into());
        match std::process::Command::new(ha_binary)
            .arg("--rescan")
            .status() {
            Ok(status) => {
                if status.success() {
                    crate::utils::detailed_log("RESCAN", "Filesystem rescan completed successfully");
                } else {
                    crate::utils::log_error("Filesystem rescan failed");
                }
            }
            Err(e) => {
                crate::utils::log_error(&format!("Failed to start rescan: {}", e));
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
                utils::detailed_log("SHOW_FOLDER", &format!("Circular alias detected for '{}', stopping resolution", current_cmd.command));
                break;
            }
            visited.insert(current_cmd.command.clone());
            
            utils::detailed_log("SHOW_FOLDER", &format!("Resolving alias '{}' to target '{}'", current_cmd.command, current_cmd.arg));
            
            // Find the target command
            if let Some(target) = all_commands.iter().find(|c| c.command == current_cmd.arg) {
                utils::detailed_log("SHOW_FOLDER", &format!("Alias resolved to: '{}' (action: {}, arg: {})", 
                    target.command, target.action, target.arg));
                current_cmd = target;
            } else {
                utils::detailed_log("SHOW_FOLDER", &format!("Failed to resolve alias '{}' - target '{}' not found", current_cmd.command, current_cmd.arg));
                break;
            }
        }
        
        current_cmd
    }
    
    /// Show folder functionality - launches the first folder matching current search
    fn show_folder_impl(&mut self) {
        use crate::utils;
        
        let search_text = &self.popup_state.search_text;
        utils::detailed_log("SHOW_FOLDER", &format!("Triggered with search text: '{}'", search_text));
        
        // Get the current filtered commands from popup state
        let display_commands = self.popup_state.filtered_commands.clone();
        utils::detailed_log("SHOW_FOLDER", &format!("Found {} filtered commands", display_commands.len()));
        
        if display_commands.is_empty() {
            utils::detailed_log("SHOW_FOLDER", "No filtered commands to show folder for");
            return;
        }
        
        // Get all commands for alias resolution
        let all_commands = self.popup_state.get_commands();
        
        // Log first few commands for debugging
        for (i, cmd) in display_commands.iter().take(3).enumerate() {
            utils::detailed_log("SHOW_FOLDER", &format!("  Command {}: '{}' (action: {}, arg: {})", 
                i, cmd.command, cmd.action, cmd.arg));
        }
        
        // Use the currently selected command and extract its folder
        let selected_idx = self.selected_index();
        if selected_idx >= display_commands.len() {
            utils::detailed_log("SHOW_FOLDER", &format!("Selected index {} is out of bounds ({})", selected_idx, display_commands.len()));
            return;
        }
        
        let cmd = &display_commands[selected_idx];
        utils::detailed_log("SHOW_FOLDER", &format!("Using selected command at index {}: '{}'", selected_idx, cmd.command));
        
        if PopupState::is_separator_command(cmd) {
            utils::detailed_log("SHOW_FOLDER", &format!("Selected command is a separator: '{}'", cmd.command));
            return;
        }
            
        utils::detailed_log("SHOW_FOLDER", &format!("Processing command: '{}' (action: {})", cmd.command, cmd.action));
        
        // Recursively resolve aliases
        let resolved_cmd = self.resolve_aliases_recursively(cmd, &all_commands);
        
        // Extract folder path using the command's built-in method
        let folder_path = if let Some(abs_path) = resolved_cmd.get_absolute_folder_path(&self.popup_state.config) {
            let path_str = abs_path.to_string_lossy().to_string();
            utils::detailed_log("SHOW_FOLDER", &format!("Found {} action, extracted folder: {}", resolved_cmd.action, path_str));
            Some(path_str)
        } else if resolved_cmd.action == "alias" {
            // This shouldn't happen since we recursively resolved aliases above
            utils::detailed_log("SHOW_FOLDER", &format!("Unresolved alias found: '{}'", resolved_cmd.command));
            None
        } else {
            utils::detailed_log("SHOW_FOLDER", &format!("Command '{}' has non-folder action: {}", resolved_cmd.command, resolved_cmd.action));
            None
        };
        
        if let Some(path) = folder_path {
            utils::log(&format!("SHOW_FOLDER: Attempting to launch folder: '{}'", path));
            
            // Launch the folder (popup stays open)
            let folder_action = crate::execute::make_action("folder", &path);
            utils::log(&format!("SHOW_FOLDER: Created action with type='folder', arg='{}'", path));
            // Execute the action
            let _ = crate::execute::execute_on_server(&folder_action);
            utils::log(&format!("SHOW_FOLDER: Sent folder command to server"));
        } else {
            utils::detailed_log("SHOW_FOLDER", &format!("No folder found for selected command: '{}'", cmd.command));
        }
    }
    
    /// Show contact for selected command (strips @ prefix if present)
    fn show_contact_impl(&mut self) {
        use crate::utils;
        
        // Get selected command index
        let selected_index = self.selected_index();
        
        // Get the current filtered commands  
        let display_commands = self.popup_state.filtered_commands.clone();
        
        if display_commands.is_empty() || selected_index >= display_commands.len() {
            utils::detailed_log("SHOW_CONTACT", "No commands available or invalid selection");
            return;
        }
        
        // Get the selected command
        let selected_command = &display_commands[selected_index];
        
        utils::detailed_log("SHOW_CONTACT", &format!("Processing command: {}", selected_command.command));
        
        // Resolve aliases to get the actual command
        let all_commands = self.popup_state.get_commands();
        let resolved_cmd = self.resolve_aliases_recursively(selected_command, &all_commands);
        
        // Strip @ prefix from the resolved command name if present
        let contact_name = if resolved_cmd.command.starts_with("@") {
            resolved_cmd.command[1..].to_string()
        } else {
            resolved_cmd.command.clone()
        };
        
        utils::detailed_log("SHOW_CONTACT", &format!("Contact name after stripping @: {}", contact_name));
        
        // Execute the contact action through the command server
        // Create a command object to send to the server
        let contact_cmd = crate::core::Command {
            patch: String::new(),
            command: format!("Show contact: {}", contact_name),
            action: "contact".to_string(),
            arg: contact_name.clone(),
            flags: String::new(),
        };
        
        utils::detailed_log("SHOW_CONTACT", &format!("SHOW_CONTACT: Opening contact: {}", contact_name));
        
        // Execute via server
        let contact_action = crate::execute::command_to_action(&contact_cmd);
        let _ = crate::execute::execute_on_server(&contact_action);
    }
    
    // COMMENTED OUT: Old tmux_activate function - replaced by activate_tmux below
    /*
    /// Start tmux session for selected anchor
    fn tmux_activate(&mut self) {
        use crate::utils;
        
        // Get selected command index
        let selected_index = self.selected_index();
        
        // Get the current filtered commands  
        let display_commands = self.popup_state.filtered_commands.clone();
        
        if display_commands.is_empty() || selected_index >= display_commands.len() {
            utils::detailed_log("TMUX_ACTIVATE", "No commands available or invalid selection");
            return;
        }
        
        // Get the selected command
        let selected_command = &display_commands[selected_index];
        
        // Resolve aliases to get the actual command
        let all_commands = self.popup_state.get_commands();
        let resolved_cmd = self.resolve_aliases_recursively(selected_command, &all_commands);
        
        // Check if it's an anchor
        if resolved_cmd.action != "anchor" {
            utils::detailed_log("TMUX_ACTIVATE", &format!("Resolved command is not an anchor: {}", resolved_cmd.action));
            return;
        }
        
        utils::detailed_log("TMUX_ACTIVATE", &format!("Processing anchor: {} ({})", resolved_cmd.command, resolved_cmd.arg));
        
        // Create a synthetic command to execute through the launcher
        // This will be recognized as a JavaScript action because tmux_activate is in listed_actions
        let tmux_cmd = crate::core::Command {
            command: format!("TMUX: {}", resolved_cmd.command),
            action: "tmux_activate".to_string(),
            arg: resolved_cmd.arg.clone(),
            patch: resolved_cmd.patch.clone(),
            flags: String::new(),
        };
        
        // Save as last executed for potential alias creation
        use crate::core::state::save_last_executed_command;
        let _ = save_last_executed_command(&tmux_cmd.command);
        
        // Execute using the same path as normal command execution
        // This will go through the launcher which recognizes JavaScript actions
        utils::detailed_log("TMUX_ACTIVATE", &format!("Executing through launcher: {} {}", tmux_cmd.action, tmux_cmd.arg));
        
        // Use the server for consistent execution (like all other commands)
        // Execute the action
        let tmux_action = crate::execute::command_to_action(&tmux_cmd);
        let _ = crate::execute::execute_on_server(&tmux_action);
        utils::detailed_log("TMUX_ACTIVATE", "Executed tmux_activate via server");
        // Close the popup after successful execution
        self.should_exit = true;
    }
    */
    
    /// Activate TMUX session for selected anchor/folder (formerly activate_anchor)
    fn activate_tmux(&mut self) {
        use crate::utils;
        use std::process::Command;
        use std::path::Path;
        
        // Check if we should use JavaScript version (via config option)
        if let Some(settings) = &self.popup_state.config.launcher_settings {
            if let Some(use_js) = &settings.use_javascript_tmux_activation {
                if use_js == "true" {
                    utils::detailed_log("TMUX", &format!("🚀 TMUX: Using JavaScript TMUX implementation"));
                    
                    // Get selected command index
                    let selected_index = self.selected_index();
                    let display_commands = self.popup_state.filtered_commands.clone();
                    
                    if display_commands.is_empty() || selected_index >= display_commands.len() {
                        utils::detailed_log("TMUX", &format!("❌ TMUX: No commands available for JavaScript"));
                        return;
                    }
                    
                    // Get the selected command and resolve aliases
                    let all_commands = self.popup_state.get_commands();
                    let selected_command = &display_commands[selected_index];
                    let resolved_cmd = self.resolve_aliases_recursively(selected_command, &all_commands);
                    
                    utils::detailed_log("TMUX", &format!("📋 TMUX: Selected: '{}', Resolved: '{}'", 
                        selected_command.command, resolved_cmd.command));
                    
                    // Create command to execute JavaScript action
                    let js_cmd = crate::core::Command {
                        command: format!("JS_TMUX: {}", resolved_cmd.command),
                        action: "activate_anchor".to_string(),
                        arg: resolved_cmd.arg.clone(),
                        patch: resolved_cmd.patch.clone(),
                        flags: String::new(),
                    };
                    
                    utils::detailed_log("TMUX", &format!("🎯 TMUX: Executing JavaScript action with arg: {}", js_cmd.arg));
                    let js_action = crate::execute::command_to_action(&js_cmd);
                    let _ = crate::execute::execute_on_server(&js_action);
                    
                    // Request exit after triggering
                    self.should_exit = true;
                    return;
                }
            }
        }
        
        // Original Rust implementation for TMUX session activation
        utils::detailed_log("TMUX", &format!("🦀 TMUX: Using Rust implementation for TMUX session activation"));
        
        // Get selected command index
        let selected_index = self.selected_index();
        
        // Get the current filtered commands  
        let display_commands = self.popup_state.filtered_commands.clone();
        
        if display_commands.is_empty() || selected_index >= display_commands.len() {
            utils::detailed_log("TMUX", &format!("TMUX: No commands available or invalid selection"));
            return;
        }
        
        // Get the selected command and resolve aliases
        let all_commands = self.popup_state.get_commands();
        let selected_command = &display_commands[selected_index];
        
        // DEBUG: Log the selected command details
        utils::detailed_log("TMUX", &format!("TMUX: Selected command: '{}'", selected_command.command));
        utils::detailed_log("TMUX", &format!("TMUX: Selected action: '{}'", selected_command.action));
        utils::detailed_log("TMUX", &format!("TMUX: Selected arg: '{}'", selected_command.arg));
        utils::detailed_log("TMUX", &format!("TMUX: Selected patch: '{}'", selected_command.patch));
        utils::detailed_log("TMUX", &format!("TMUX: Selected flags: '{}'", selected_command.flags));
        
        let resolved_cmd = self.resolve_aliases_recursively(selected_command, &all_commands);
        
        // DEBUG: Log resolved command if different
        if resolved_cmd.command != selected_command.command {
            utils::detailed_log("TMUX", &format!("TMUX: Resolved to: '{}'", resolved_cmd.command));
            utils::detailed_log("TMUX", &format!("TMUX: Resolved action: '{}'", resolved_cmd.action));
            utils::detailed_log("TMUX", &format!("TMUX: Resolved arg: '{}'", resolved_cmd.arg));
        }
        
        // Extract folder path
        utils::detailed_log("TMUX", &format!("TMUX: Attempting to extract folder path..."));
        let folder_path = if let Some(abs_path) = resolved_cmd.get_absolute_folder_path(&self.popup_state.config) {
            abs_path.to_string_lossy().to_string()
        } else {
            utils::detailed_log("TMUX", &format!("TMUX: ERROR - Could not extract folder path from command: '{}'", resolved_cmd.command));
            utils::detailed_log("TMUX", &format!("TMUX: Command details - action: '{}', arg: '{}'", resolved_cmd.action, resolved_cmd.arg));
            return;
        };
        
        utils::detailed_log("TMUX", &format!("TMUX: Successfully extracted folder path: '{}'", folder_path));
        
        // Direct Rust implementation of TMUX activation (temporary - will be replaced with JavaScript)
        // This is based on the working Python anchor script logic
        
        // 1. Open folder in Finder
        //if let Err(e) = Command::new("open").arg(&folder_path).spawn() {
        //    utils::detailed_log("TMUX", &format!("TMUX: Failed to open folder: {}", e));
        //}
        
        // 2. Open in Obsidian if it's in the vault
        //let vault_path = self.popup_state.config.launcher_settings.as_ref()
        //    .and_then(|ls| ls.obsidian_vault_path.as_ref())
        //    .map(|p| utils::expand_tilde(p))
        //    .unwrap_or_else(|| "/Users/oblinger/ob/kmr".to_string());
        //    
        //if Path::new(&folder_path).starts_with(&vault_path) {
        //    if let Some(vault_name) = self.popup_state.config.launcher_settings.as_ref()
        //        .and_then(|ls| ls.obsidian_vault_name.as_ref()) {
        //        // TODO: Open in Obsidian
        //        utils::detailed_log("TMUX", &format!("TMUX: Would open in Obsidian vault: {}", vault_name));
        //    }
        //}
        
        // 3. Check for .tmuxp.yaml and activate TMUX session
        let tmuxp_path = Path::new(&folder_path).join(".tmuxp.yaml");
        if tmuxp_path.exists() {
            utils::detailed_log("TMUX", &format!("TMUX: Found .tmuxp.yaml at {:?}", tmuxp_path));
            
            // Parse the .tmuxp.yaml file to get the actual session name
            utils::detailed_log("TMUX_RUST", &format!("Line 2354: Parsing .tmuxp.yaml to get session_name"));
            let session_name = match std::fs::read_to_string(&tmuxp_path) {
                Ok(yaml_content) => {
                    let mut found_session_name = None;
                    for line in yaml_content.lines() {
                        if line.trim_start().starts_with("session_name:") {
                            // Extract the session name after the colon
                            let parts: Vec<&str> = line.splitn(2, ':').collect();
                            if parts.len() == 2 {
                                let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                                utils::detailed_log("TMUX_RUST", &format!("Line 2360: Found session_name in YAML: '{}'", name));
                                found_session_name = Some(name.to_string());
                                break;
                            }
                        }
                    }
                    
                    if let Some(name) = found_session_name {
                        utils::detailed_log("TMUX", &format!("TMUX: Using session name from .tmuxp.yaml: '{}'", name));
                        name
                    } else {
                        // No session_name found in YAML - this is an error
                        utils::detailed_log("TMUX", &format!("TMUX: ERROR - No session_name found in .tmuxp.yaml"));
                        utils::detailed_log("TMUX_RUST", "Line 2373: ERROR - .tmuxp.yaml missing session_name field");
                        self.show_error_dialog(&format!(
                            "The .tmuxp.yaml file at:\n{}\n\nis missing a 'session_name' field.\n\nPlease add:\nsession_name: <name>\n\nto the file.",
                            tmuxp_path.display()
                        ));
                        return;
                    }
                }
                Err(e) => {
                    utils::detailed_log("TMUX", &format!("TMUX: ERROR - Failed to read .tmuxp.yaml: {}", e));
                    utils::detailed_log("TMUX_RUST", &format!("Line 2382: Failed to read .tmuxp.yaml: {}", e));
                    self.show_error_dialog(&format!(
                        "Failed to read .tmuxp.yaml file:\n{}\n\nError: {}",
                        tmuxp_path.display(),
                        e
                    ));
                    return;
                }
            };
            
            utils::detailed_log("TMUX", &format!("TMUX: Session name from YAML: '{}'", session_name));
            
            // Check if session exists
            let check_session = Command::new("/opt/homebrew/bin/tmux")
                .args(&["has-session", "-t", &session_name])
                .output();
                
            let session_exists = check_session.map(|o| o.status.success()).unwrap_or(false);
            
            if !session_exists {
                utils::detailed_log("TMUX", &format!("TMUX: Creating new tmux session '{}'", session_name));
                utils::detailed_log("TMUX_RUST", &format!("Line 2384: Session '{}' does not exist, creating with tmuxp", session_name));
                
                // First, let's see what sessions exist before we create
                utils::detailed_log("TMUX_RUST", "Line 2386: Checking existing sessions before creation");
                if let Ok(existing) = Command::new("/opt/homebrew/bin/tmux")
                    .args(&["list-sessions", "-F", "#{session_name}"])
                    .output() {
                    let sessions = String::from_utf8_lossy(&existing.stdout);
                    utils::detailed_log("TMUX_RUST", &format!("Line 2390: Existing sessions: {}", sessions.trim()));
                }
                
                // Create session with tmuxp
                // Since we expect the .tmuxp.yaml to have the correct session name (with underscores),
                // we don't use -s flag which seems to cause issues
                // Add /opt/homebrew/bin to PATH so tmuxp can find tmux
                let tmuxp_cmd = format!("/opt/homebrew/bin/tmuxp load '{}' -d", tmuxp_path.to_str().unwrap_or(""));
                utils::detailed_log("TMUX_RUST", &format!("Line 2397: Running command: {}", tmuxp_cmd));
                utils::detailed_log("TMUX_RUST", &format!("Line 2398: Current dir: {}", folder_path));
                
                match Command::new("/opt/homebrew/bin/tmuxp")
                    .args(&["load", tmuxp_path.to_str().unwrap_or(""), "-d"])
                    .current_dir(&folder_path)
                    .env("PATH", format!("/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:{}", 
                        std::env::var("PATH").unwrap_or_default()))
                    .output() {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        
                        utils::detailed_log("TMUX_RUST", &format!("Line 2408: tmuxp exit code: {:?}", output.status.code()));
                        utils::detailed_log("TMUX_RUST", &format!("Line 2409: tmuxp stdout: '{}'", stdout));
                        utils::detailed_log("TMUX_RUST", &format!("Line 2410: tmuxp stderr: '{}'", stderr));
                        
                        if !output.status.success() {
                            utils::detailed_log("TMUX", &format!("TMUX: tmuxp failed with exit code {:?}", output.status.code()));
                            utils::detailed_log("TMUX", &format!("TMUX: tmuxp stderr: {}", stderr));
                            utils::detailed_log("TMUX", &format!("TMUX: tmuxp stdout: {}", stdout));
                            utils::detailed_log("TMUX_RUST", "Line 2415: tmuxp FAILED, returning");
                            return;
                        }
                        
                        // Log output even on success to debug issues
                        utils::detailed_log("TMUX", &format!("TMUX: tmuxp completed with exit code 0"));
                        if !stdout.is_empty() {
                            utils::detailed_log("TMUX", &format!("TMUX: tmuxp stdout: {}", stdout));
                        }
                        if !stderr.is_empty() {
                            utils::detailed_log("TMUX", &format!("TMUX: tmuxp stderr: {}", stderr));
                        }
                        
                        // Check what sessions exist after tmuxp
                        utils::detailed_log("TMUX_RUST", "Line 2428: Checking sessions after tmuxp");
                        if let Ok(after) = Command::new("/opt/homebrew/bin/tmux")
                            .args(&["list-sessions", "-F", "#{session_name}"])
                            .output() {
                            let sessions = String::from_utf8_lossy(&after.stdout);
                            utils::detailed_log("TMUX_RUST", &format!("Line 2432: Sessions after tmuxp: {}", sessions.trim()));
                        }
                        
                        // Verify the session was actually created
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        utils::detailed_log("TMUX_RUST", &format!("Line 2436: Verifying session '{}' exists", session_name));
                        let verify = Command::new("/opt/homebrew/bin/tmux")
                            .args(&["has-session", "-t", &session_name])
                            .output();
                        
                        if let Ok(v) = verify {
                            let verify_stderr = String::from_utf8_lossy(&v.stderr);
                            utils::detailed_log("TMUX_RUST", &format!("Line 2442: Verify exit code: {}", v.status.code().unwrap_or(-1)));
                            utils::detailed_log("TMUX_RUST", &format!("Line 2443: Verify stderr: '{}'", verify_stderr));
                            
                            if v.status.success() {
                                utils::detailed_log("TMUX", &format!("TMUX: Verified session '{}' exists", session_name));
                                
                                // Run claude --continue in the new session
                                // Add a small delay to ensure the window is created
                                std::thread::sleep(std::time::Duration::from_millis(500));
                                utils::detailed_log("TMUX", &format!("TMUX: Running claude --continue in new tmuxp session '{}'", session_name));
                                match Command::new("/opt/homebrew/bin/tmux")
                                    .args(&["send-keys", "-t", &session_name, "claude --continue", "C-m"])
                                    .output() {
                                    Ok(output) => {
                                        if output.status.success() {
                                            utils::detailed_log("TMUX", &format!("TMUX: Successfully sent claude --continue to tmuxp session '{}'", session_name));
                                        } else {
                                            utils::detailed_log("TMUX", &format!("TMUX: Failed to send claude --continue to tmuxp session: {}", String::from_utf8_lossy(&output.stderr)));
                                        }
                                    }
                                    Err(e) => {
                                        utils::detailed_log("TMUX", &format!("TMUX: Error sending claude --continue to tmuxp session: {}", e));
                                    }
                                }
                            } else {
                                // tmuxp said it succeeded but session wasn't created - this is an error
                                utils::detailed_log("TMUX", &format!("TMUX: ERROR - Session '{}' was NOT created despite tmuxp reporting success", session_name));
                                utils::detailed_log("TMUX", &format!("TMUX: tmux has-session stderr: {}", verify_stderr));
                                utils::detailed_log("TMUX_RUST", &format!("Line 2479: CRITICAL ERROR - tmuxp succeeded but session '{}' doesn't exist", session_name));
                                
                                // Let's check what's in the .tmuxp.yaml file
                                utils::detailed_log("TMUX_RUST", &format!("Line 2481: Reading .tmuxp.yaml to check session_name"));
                                if let Ok(yaml_content) = std::fs::read_to_string(&tmuxp_path) {
                                    // Look for session_name in the YAML
                                    for line in yaml_content.lines() {
                                        if line.contains("session_name") {
                                            utils::detailed_log("TMUX_RUST", &format!("Line 2485: Found in .tmuxp.yaml: {}", line.trim()));
                                            break;
                                        }
                                    }
                                }
                                
                                // Try a fallback: check if tmuxp created a session with a different name
                                utils::detailed_log("TMUX_RUST", "Line 2490: Checking if tmuxp created session with different name");
                                if let Ok(after_create) = Command::new("/opt/homebrew/bin/tmux")
                                    .args(&["list-sessions", "-F", "#{session_name}"])
                                    .output() {
                                    let all_sessions = String::from_utf8_lossy(&after_create.stdout);
                                    utils::detailed_log("TMUX_RUST", &format!("Line 2494: All sessions after tmuxp: {}", all_sessions.trim()));
                                    
                                    // Try to find a session that might match our expected session name
                                    for session in all_sessions.lines() {
                                        if session.contains(&session_name) || session_name.contains(session) {
                                            utils::detailed_log("TMUX_RUST", &format!("Line 2498: Found possible match: '{}'", session));
                                        }
                                    }
                                }
                                
                                // Check if tmuxp can't find tmux in PATH
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                if stdout.contains("tmux not found") {
                                    self.show_error_dialog("tmuxp cannot find tmux in PATH.\nEnsure tmux is installed and in your PATH.");
                                } else {
                                    self.show_error_dialog(&format!(
                                        "Failed to create TMUX session '{}' with tmuxp.\nCheck ~/.config/hookanchor/anchor.log for details.",
                                        session_name
                                    ));
                                }
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        utils::detailed_log("TMUX", &format!("TMUX: Failed to run tmuxp: {}", e));
                        return;
                    }
                }
                // Give tmux a moment to stabilize
                std::thread::sleep(std::time::Duration::from_millis(100));
            } else {
                utils::detailed_log("TMUX", &format!("TMUX: Session '{}' already exists", session_name));
            }
            
            // Now we need to switch to the session in an existing TMUX client
            // First check if there's a TMUX client running anywhere
            utils::detailed_log("TMUX", &format!("TMUX: Checking for existing TMUX clients"));
            
            // Check if any tmux clients are attached
            let list_clients = Command::new("/opt/homebrew/bin/tmux")
                .args(&["list-clients"])
                .output();
                
            let has_clients = match list_clients {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let has = output.status.success() && !stdout.trim().is_empty();
                    if has {
                        utils::detailed_log("TMUX", &format!("TMUX: Found TMUX clients: {}", stdout.trim()));
                    } else {
                        utils::detailed_log("TMUX", &format!("TMUX: No TMUX clients found"));
                    }
                    has
                }
                Err(e) => {
                    utils::detailed_log("TMUX", &format!("TMUX: Failed to list clients: {}", e));
                    false
                }
            };
            
            if !has_clients {
                // No TMUX client running - show error to user
                let error_msg = format!(
                    "Cannot switch to TMUX session '{}': No TMUX client is running.\n\n\
                    Please open a terminal and run:\n\
                    tmux attach-session -t '{}'",
                    session_name, session_name
                );
                utils::detailed_log("TMUX", &format!("TMUX: {}", error_msg));
                self.show_error_dialog(&error_msg);
                return;
            }
            
            // There are TMUX clients, so we can switch
            utils::detailed_log("TMUX", &format!("TMUX: Switching to session '{}' in existing client", session_name));
            
            // Use switch-client which will switch in any attached client
            match Command::new("/opt/homebrew/bin/tmux")
                .args(&["switch-client", "-t", &session_name])
                .output() {
                Ok(output) => {
                    if output.status.success() {
                        utils::detailed_log("TMUX", &format!("TMUX: Successfully switched to session '{}'", session_name));
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        utils::detailed_log("TMUX", &format!("TMUX: Failed to switch: {}", stderr));
                        self.show_error_dialog(&format!(
                            "Failed to switch to TMUX session '{}': {}",
                            session_name, stderr
                        ));
                    }
                }
                Err(e) => {
                    utils::detailed_log("TMUX", &format!("TMUX: Failed to run switch-client: {}", e));
                    self.show_error_dialog(&format!(
                        "Failed to switch to TMUX session '{}': {}",
                        session_name, e
                    ));
                }
            }
                
        } else {
            utils::detailed_log("TMUX", &format!("TMUX: No .tmuxp.yaml found - creating basic TMUX session"));
            
            // Extract folder name for session name
            let folder_name = Path::new(&folder_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("session");
            
            // Sanitize the session name for tmux compatibility
            let session_name = folder_name
                .replace(' ', "_")
                .replace(':', "_")
                .replace('.', "_")
                .replace('[', "_")
                .replace(']', "_");
            
            utils::detailed_log("TMUX", &format!("TMUX: Creating basic session '{}' in folder '{}'", session_name, folder_path));
            
            // Check if session already exists
            let check_session = Command::new("/opt/homebrew/bin/tmux")
                .args(&["has-session", "-t", &session_name])
                .output();
            
            let session_exists = check_session.map(|o| o.status.success()).unwrap_or(false);
            
            if !session_exists {
                // Create new basic tmux session (shell will be started, claude command sent after)
                utils::detailed_log("TMUX", &format!("TMUX: Running: tmux new-session -d -s '{}' -c '{}'", 
                    session_name, folder_path));
                
                // First, let's list all existing sessions before creation
                if let Ok(list_before) = Command::new("/opt/homebrew/bin/tmux")
                    .args(&["list-sessions"])
                    .output() {
                    let sessions_before = String::from_utf8_lossy(&list_before.stdout);
                    utils::detailed_log("TMUX", &format!("TMUX: Sessions BEFORE creation:\n{}", 
                        if sessions_before.trim().is_empty() { "(no sessions)" } else { sessions_before.trim() }));
                }
                
                // Create session without an initial command - just start a shell
                // We'll send the claude --continue command after the session is created
                match Command::new("/opt/homebrew/bin/tmux")
                    .args(&["new-session", "-d", "-s", &session_name, "-c", &folder_path])
                    .output() {
                    Ok(output) => {
                        // Log the output regardless of success/failure
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        utils::detailed_log("TMUX", &format!("TMUX: new-session exit code: {:?}", output.status.code()));
                        if !stdout.is_empty() {
                            utils::detailed_log("TMUX", &format!("TMUX: new-session stdout: {}", stdout));
                        }
                        if !stderr.is_empty() {
                            utils::detailed_log("TMUX", &format!("TMUX: new-session stderr: {}", stderr));
                        }
                        
                        // List sessions after attempted creation
                        if let Ok(list_after) = Command::new("/opt/homebrew/bin/tmux")
                            .args(&["list-sessions"])
                            .output() {
                            let sessions_after = String::from_utf8_lossy(&list_after.stdout);
                            utils::detailed_log("TMUX", &format!("TMUX: Sessions AFTER creation:\n{}", 
                                if sessions_after.trim().is_empty() { "(no sessions)" } else { sessions_after.trim() }));
                        }
                        
                        if output.status.success() {
                            utils::detailed_log("TMUX", &format!("TMUX: Basic session '{}' creation reported success", session_name));
                            
                            // Check if session immediately exists without delay
                            if let Ok(immediate_check) = Command::new("/opt/homebrew/bin/tmux")
                                .args(&["has-session", "-t", &session_name])
                                .output() {
                                if immediate_check.status.success() {
                                    utils::detailed_log("TMUX", &format!("TMUX: ✅ Session '{}' exists immediately after creation", session_name));
                                } else {
                                    utils::detailed_log("TMUX", &format!("TMUX: ❌ Session '{}' NOT found immediately after creation", session_name));
                                    let stderr = String::from_utf8_lossy(&immediate_check.stderr);
                                    utils::detailed_log("TMUX", &format!("TMUX: Immediate check stderr: {}", stderr));
                                }
                            }
                            
                            // Give tmux more time to fully create the session
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            
                            // Verify it was created - with more detailed logging
                            utils::detailed_log("TMUX", &format!("TMUX: Verifying session '{}' exists...", session_name));
                            match Command::new("/opt/homebrew/bin/tmux")
                                .args(&["has-session", "-t", &session_name])
                                .output() {
                                Ok(verify) => {
                                    let verify_stdout = String::from_utf8_lossy(&verify.stdout);
                                    let verify_stderr = String::from_utf8_lossy(&verify.stderr);
                                    utils::detailed_log("TMUX", &format!("TMUX: has-session exit code: {:?}", verify.status.code()));
                                    if !verify_stdout.is_empty() {
                                        utils::detailed_log("TMUX", &format!("TMUX: has-session stdout: {}", verify_stdout));
                                    }
                                    if !verify_stderr.is_empty() {
                                        utils::detailed_log("TMUX", &format!("TMUX: has-session stderr: {}", verify_stderr));
                                    }
                                    
                                    if verify.status.success() {
                                        utils::detailed_log("TMUX", &format!("TMUX: ✅ Session '{}' verified!", session_name));
                                        
                                        // Now send the claude --continue command to the session
                                        utils::detailed_log("TMUX", &format!("TMUX: Sending 'claude --continue' to session '{}'", session_name));
                                        match Command::new("/opt/homebrew/bin/tmux")
                                            .args(&["send-keys", "-t", &session_name, "claude --continue", "C-m"])
                                            .output() {
                                            Ok(send_output) => {
                                                if send_output.status.success() {
                                                    utils::detailed_log("TMUX", &format!("TMUX: ✅ Successfully sent claude command to session '{}'", session_name));
                                                } else {
                                                    let send_stderr = String::from_utf8_lossy(&send_output.stderr);
                                                    utils::detailed_log("TMUX", &format!("TMUX: ❌ Failed to send claude command: {}", send_stderr));
                                                }
                                            }
                                            Err(e) => {
                                                utils::detailed_log("TMUX", &format!("TMUX: Error sending claude command: {}", e));
                                            }
                                        }
                                    } else {
                                        // Session doesn't exist despite tmux new-session reporting success
                                        utils::detailed_log("TMUX", &format!("TMUX: ❌ Session '{}' NOT found after creation!", session_name));
                                        
                                        // Check if stderr contains a specific error
                                        if verify_stderr.contains("duplicate session") {
                                            utils::detailed_log("TMUX", &format!("TMUX: ERROR - Duplicate session name detected"));
                                        } else if verify_stderr.contains("server not found") {
                                            utils::detailed_log("TMUX", &format!("TMUX: ERROR - No TMUX server running"));
                                        }
                                        
                                        // Try to get more info about what's wrong
                                        if let Ok(info) = Command::new("/opt/homebrew/bin/tmux")
                                            .args(&["info"])
                                            .output() {
                                            let info_out = String::from_utf8_lossy(&info.stderr);
                                            if !info_out.is_empty() {
                                                utils::detailed_log("TMUX", &format!("TMUX: tmux info stderr: {}", info_out));
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    utils::detailed_log("TMUX", &format!("TMUX: Error running has-session: {}", e));
                                }
                            }
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            utils::detailed_log("TMUX", &format!("TMUX: Failed to create session: {}", stderr));
                            self.show_error_dialog(&format!("Failed to create TMUX session: {}", stderr));
                            return;
                        }
                    }
                    Err(e) => {
                        utils::detailed_log("TMUX", &format!("TMUX: Error executing tmux command: {}", e));
                        self.show_error_dialog(&format!("Failed to execute tmux: {}", e));
                        return;
                    }
                }
            } else {
                utils::detailed_log("TMUX", &format!("TMUX: Session '{}' already exists", session_name));
            }
            
            // Now switch to the session
            utils::detailed_log("TMUX", &format!("TMUX: Checking for existing TMUX clients"));
            
            let list_clients = Command::new("/opt/homebrew/bin/tmux")
                .args(&["list-clients"])
                .output();
            
            let has_clients = match list_clients {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let has = output.status.success() && !stdout.trim().is_empty();
                    if has {
                        utils::detailed_log("TMUX", &format!("TMUX: Found TMUX clients: {}", stdout.trim()));
                    } else {
                        utils::detailed_log("TMUX", &format!("TMUX: No TMUX clients found"));
                    }
                    has
                }
                Err(e) => {
                    utils::detailed_log("TMUX", &format!("TMUX: Failed to list clients: {}", e));
                    false
                }
            };
            
            if has_clients {
                // Switch to the session
                utils::detailed_log("TMUX", &format!("TMUX: Switching to session '{}'", session_name));
                
                match Command::new("/opt/homebrew/bin/tmux")
                    .args(&["switch-client", "-t", &session_name])
                    .output() {
                    Ok(output) => {
                        if output.status.success() {
                            utils::detailed_log("TMUX", &format!("TMUX: Successfully switched to session '{}'", session_name));
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            utils::detailed_log("TMUX", &format!("TMUX: Failed to switch: {}", stderr));
                        }
                    }
                    Err(e) => {
                        utils::detailed_log("TMUX", &format!("TMUX: Error switching: {}", e));
                    }
                }
            } else {
                // No client to switch, just inform user
                let msg = format!(
                    "TMUX session '{}' created.\nOpen a terminal and run:\ntmux attach-session -t '{}'",
                    session_name, session_name
                );
                utils::detailed_log("TMUX", &format!("TMUX: {}", msg));
                self.show_error_dialog(&msg);
            }
            
        }
        
        // Request exit after activation
        self.should_exit = true;
    }
}

fn save_window_position(pos: egui::Pos2) {
    crate::utils::detailed_log("WINDOW_POS", &format!("save_window_position called with pos: ({}, {})", pos.x, pos.y));
    let mut state = load_state();
    let old_pos = state.window_position;
    state.window_position = Some((pos.x, pos.y));
    match save_state(&state) {
        Ok(_) => {
            crate::utils::detailed_log("WINDOW_POS", &format!("Successfully saved position from {:?} to ({}, {})", old_pos, pos.x, pos.y));
        }
        Err(e) => {
            crate::utils::detailed_log("WINDOW_POS", &format!("Failed to save position: {}", e));
        }
    }
}

fn load_window_position() -> Option<egui::Pos2> {
    let state = load_state();
    let result = state.window_position.map(|(x, y)| egui::pos2(x, y));
    crate::utils::detailed_log("WINDOW_POS", &format!("load_window_position returned {:?}", result));
    result
}

fn get_previous_window_location(ctx: &egui::Context, window_size: egui::Vec2) -> egui::Pos2 {
    crate::utils::detailed_log("WINDOW_POS", &format!("get_previous_window_location called with window_size: {:?}", window_size));
    
    // First try to load the previous position
    if let Some(previous_pos) = load_window_position() {
        crate::utils::detailed_log("WINDOW_POS", &format!("Loaded previous position: {:?}", previous_pos));
        // Check if this position is visible on any current display
        if is_position_visible(previous_pos, window_size) {
            crate::utils::detailed_log("WINDOW_POS", "Position is visible, returning it");
            return previous_pos;
        } else {
            crate::utils::detailed_log("WINDOW_POS", "Position NOT visible, will center instead");
        }
    } else {
        crate::utils::detailed_log("WINDOW_POS", "No previous position found in state");
    }
    
    // If no previous position or not visible, center on main display
    let centered = center_on_main_display(ctx, window_size);
    crate::utils::detailed_log("WINDOW_POS", &format!("Centering on main display at: {:?}", centered));
    centered
}

fn is_position_visible(pos: egui::Pos2, window_size: egui::Vec2) -> bool {
    crate::utils::detailed_log("WINDOW_POS", &format!("is_position_visible checking pos: {:?}, window_size: {:?}", pos, window_size));
    
    // Get all available monitors/displays
    // For now, we'll use a basic check - ensure the window isn't completely off-screen
    // This is a simplified version - in a full implementation you'd query actual display bounds
    
    let window_rect = egui::Rect::from_min_size(pos, window_size);
    crate::utils::detailed_log("WINDOW_POS", &format!("Window rect: {:?}", window_rect));
    
    // Basic bounds check - assume main display is at least 1024x768
    // In a real implementation, you'd query actual display information
    let main_display_rect = egui::Rect::from_min_size(
        egui::pos2(0.0, 0.0), 
        egui::vec2(1024.0, 768.0)
    );
    crate::utils::detailed_log("WINDOW_POS", &format!("Main display rect: {:?}", main_display_rect));
    
    // Check if at least part of the window is visible
    // Allow for window to be partially off-screen but require some overlap
    let min_visible_area = window_size.x.min(window_size.y) * 0.3; // 30% of smaller dimension
    
    let intersection = main_display_rect.intersect(window_rect);
    intersection.width() * intersection.height() >= min_visible_area * min_visible_area
}

/// Get the actual screen dimensions (not window dimensions)
fn get_actual_screen_dimensions() -> (f32, f32) {
    #[cfg(target_os = "macos")]
    {
        let display = CGDisplay::main();
        let width = display.pixels_wide() as f32;
        let height = display.pixels_high() as f32;
        crate::utils::detailed_log("SYSTEM", &format!("🔵 SCREEN: Got actual screen dimensions from CoreGraphics: {}x{}", width, height));
        (width, height)
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        crate::utils::detailed_log("SYSTEM", &format!("🔵 SCREEN: Using default screen dimensions (not macOS)"));
        (1920.0, 1080.0)
    }
}

fn center_on_main_display(_ctx: &egui::Context, window_size: egui::Vec2) -> egui::Pos2 {
    // Get actual screen dimensions
    let (screen_width, screen_height) = get_actual_screen_dimensions();
    let display_size = egui::vec2(screen_width, screen_height);
    
    // Center the window on the display
    egui::pos2(
        (display_size.x - window_size.x) / 2.0,
        (display_size.y - window_size.y) / 2.0
    )
}

impl AnchorSelector {
    /// Generate the patch path display string for submenu headers
    /// Returns a string like "FOO > BAR > BAZ > CMD" 
    fn get_patch_path_display(&self, command_name: &str) -> String {
        // If we have submenu info, use that for more accurate breadcrumbs
        if let Some((_, resolved_command)) = self.popup_state.get_submenu_command_info() {
            let sys_data = crate::core::sys_data::get_sys_data();
            let path = get_patch_path(&resolved_command.command, &sys_data.patches);
            
            if path.is_empty() {
                resolved_command.command.clone()
            } else {
                path.join(" > ")
            }
        } else {
            // Fallback to old behavior
            let sys_data = crate::core::sys_data::get_sys_data();
            let path = get_patch_path(command_name, &sys_data.patches);
            
            if path.is_empty() {
                command_name.to_string()
            } else {
                path.join(" > ")
            }
        }
    }
}

impl eframe::App for AnchorSelector {
    /// Return transparent clear color to enable rounded corners
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if we have a pending rebuild to perform
        if self.pending_rebuild {
            self.pending_rebuild = false;
            self.perform_rebuild();
            return; // Exit after rebuild (the process will be replaced)
        }
        
        // Enhanced debugging for visibility issues
        if self.frame_count < 10 || self.frame_count % 60 == 0 {  // Log first 10 frames and then every second
            let viewport_info = ctx.input(|i| {
                format!("Frame {}: focused={}, pos={:?}, is_hidden={}, should_exit={}",
                    self.frame_count,
                    i.focused,
                    i.viewport().outer_rect,
                    self.is_hidden,
                    self.should_exit
                )
            });
            crate::utils::detailed_log("POPUP_UPDATE", &viewport_info);
        }
        
        // Write to debug file to see if update() is being called at all
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
            use std::io::Write;
            let _ = writeln!(file, "📱 update() called - frame: {} loading: {:?}", self.frame_count, self.loading_state);
        }
        
        // Increment frame counter for initial setup only
        if self.frame_count < 20 {
            self.frame_count += 1;
        }
        
        // Start deferred loading on second frame (after UI is shown)
        if self.frame_count == 2 && self.loading_state == LoadingState::NotLoaded {
            crate::utils::detailed_log("POPUP", &format!("POPUP: Starting deferred loading on frame 2"));
            self.start_deferred_loading();
            ctx.request_repaint(); // Ensure we update when loading completes
        }
        
        // On the first few frames, ensure the window is properly activated and positioned
        if self.frame_count <= 3 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            ctx.request_repaint(); // Ensure continuous updates during initialization
            
            // Also ensure proper window positioning on startup
            if self.frame_count == 2 && !self.position_set {
                let config = crate::core::sys_data::get_config();
                let width = config.popup_settings.get_default_window_width() as f32;
                let height = config.popup_settings.get_default_window_height() as f32;
                let window_size = egui::vec2(width, height);
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
        
        // Check for idle timeout - only when visible and not showing dialogs/editor
        if !self.is_hidden && !self.command_editor.visible && !self.dialog.visible {
            let config = crate::core::sys_data::get_config();
            if let Some(timeout_seconds) = config.popup_settings.idle_timeout_seconds {
                let idle_duration = self.last_interaction_time.elapsed();
                if idle_duration.as_secs() >= timeout_seconds {
                    crate::utils::log(&format!("IDLE_TIMEOUT: Hiding popup after {} seconds of inactivity", timeout_seconds));
                    self.exit_or_hide(ctx);
                    // Reset interaction time to prevent immediate re-triggering
                    self.last_interaction_time = std::time::Instant::now();
                }
            }
        }
        
        // Check if exit was requested by user action
        if self.should_exit {
            self.should_exit = false; // Reset flag
            self.exit_or_hide(ctx);
        }
            
        // When hidden, still request occasional repaints to check for show commands
        if self.is_hidden {
            // Log periodically when hidden to verify update loop is running
            if self.frame_count % 120 == 0 { // Every ~2 seconds at 500ms refresh
                crate::utils::log(&format!("[UPDATE_LOOP] Still running while hidden - Frame {}, is_hidden={}", 
                    self.frame_count, self.is_hidden));
            }
            // Request slower repaints when hidden to check for commands
            ctx.request_repaint_after(std::time::Duration::from_millis(500));
        } else {
            // For idle state, request slower repaints to reduce CPU usage
            if !has_input && !has_active_ui && self.frame_count >= 10 {
                ctx.request_repaint_after(std::time::Duration::from_millis(100));
            }
            
            // During countdown, ensure frequent updates for smooth 1-second timing
            if self.grabber_countdown.is_some() {
                ctx.request_repaint_after(std::time::Duration::from_millis(50));
            }
        }
        
        
        // Check for window focus state changes and log for debugging
        // if self.frame_count <= 20 {
        //     let has_focus = ctx.input(|i| i.focused);
        //     if self.frame_count % 5 == 0 { // Log every 5th frame for first 20 frames
        //         crate::utils::detailed_log("FOCUS", &format!("Frame {}: Window focused={}, input focus_set={}", 
        //             self.frame_count, has_focus, self.focus_set));
        //     }
        // }
        
        
        // Set position on first frame after window is created
        if !self.position_set {
            // Use a reasonable default window size for positioning - the actual size will be auto-calculated
            let config = crate::core::sys_data::get_config();
            let width = config.popup_settings.get_default_window_width() as f32;
            let height = config.popup_settings.get_default_window_height() as f32;
            let window_size = egui::vec2(width, height);
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
        if self.grabber_countdown.is_some() {
            crate::utils::verbose_log("GRAB", "Calling update_grabber_countdown from main update");
        }
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
        // Debug log before calling command editor update
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
            use std::io::Write;
            let _ = writeln!(file, "🎯 POPUP: About to call command_editor.update()");
        }
        let editor_result = self.command_editor.update(ctx, &config);
        
        // Check for queued errors and display them
        if crate::utils::error::has_errors() {
            if let Some(error_message) = crate::utils::error::take_next_error() {
                self.show_error_dialog(&error_message);
            }
        }
        
        // Update dialog system
        if self.dialog.update(ctx) {
            // Dialog was closed, request focus on input field
            self.request_focus = true;
            if let Some(result) = self.dialog.take_result() {
                // Check if we have a pending action to execute
                if self.has_pending_action() {
                    // Let the action system handle the dialog result
                    self.complete_action(result);
                } else {
                    // Handle legacy dialog logic (uninstall, etc.)
                    if let Some(button_text) = result.get("exit") {
                        if button_text == "Exit" {
                            // Don't perform scanner check - it blocks
                            self.exit_or_hide(ctx);
                        } else if button_text == "OK" {
                            // Check if this is from the uninstall dialog (has the warning about Karabiner)
                            // by looking at the dialog title or content - for simplicity, assume OK from uninstall dialog
                            
                            // Execute uninstall in background thread
                            std::thread::spawn(|| {
                                match crate::systems::setup_assistant::uninstall_hookanchor() {
                                    Ok(()) => {
                                        // Exit successfully - no stdout message
                                        std::process::exit(0);
                                    },
                                    Err(_e) => {
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
        }
        // Track if command editor was just closed to restore focus
        let mut command_editor_just_closed = false;
        
        match editor_result {
            CommandEditorResult::Cancel => {
                // Write debug to log file
                if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
                    use std::io::Write;
                    let _ = writeln!(file, "🚪 POPUP: Got Cancel from command editor, hiding it");
                }
                self.close_command_editor();
                command_editor_just_closed = true;
                if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
                    use std::io::Write;
                    let _ = writeln!(file, "🚪 POPUP: Command editor is now hidden, back to main popup");
                }
            }
            CommandEditorResult::Save(_new_command, original_command_name) => {
                // Get the save data from command editor
                let (command_to_delete, mut new_command) = self.command_editor.prepare_save_command();
                
                // Check if this is a rename that might have side effects
                let orig_name = &original_command_name;
                crate::utils::log(&format!("RENAME_CHECK: orig_name='{}', new_name='{}', empty={}, command_to_delete={:?}", orig_name, new_command.command, orig_name.is_empty(), command_to_delete));
                
                // Also check if there's a command_to_delete - this indicates editing an existing command
                let effective_old_name = if !orig_name.is_empty() {
                    orig_name.as_str()
                } else if let Some(ref old_cmd) = command_to_delete {
                    old_cmd.as_str()
                } else {
                    ""
                };
                
                if !effective_old_name.is_empty() {
                    if effective_old_name != &new_command.command {
                        crate::utils::log(&format!("RENAME_DETECTED: '{}' -> '{}'", effective_old_name, new_command.command));
                        // Command name changed - check for rename side effects
                        let config = crate::core::sys_data::get_config();
                        
                        // Check if any rename flags are enabled
                        let has_rename_options = config.popup_settings.rename_doc.unwrap_or(false) ||
                                               config.popup_settings.rename_folder.unwrap_or(false) ||
                                               config.popup_settings.rename_patch.unwrap_or(false) ||
                                               config.popup_settings.rename_prefix.unwrap_or(false);
                        
                        if has_rename_options {
                            // Perform dry-run to see what would be renamed
                            use crate::core::commands::rename_associated_data;
                            let mut patches = crate::core::commands::create_patches_hashmap(&self.commands());
                            match rename_associated_data(
                                effective_old_name,
                                &new_command.command,
                                &new_command.arg,
                                &new_command.action,
                                self.commands_mut(),
                                &mut patches,
                                &config,
                                true, // dry_run = true
                            ) {
                                Ok((updated_arg, actions)) => {
                                    crate::utils::log(&format!("RENAME_DRY_RUN: Found {} actions: {:?}", actions.len(), actions));
                                    if !actions.is_empty() {
                                        // There are side effects - show confirmation dialog
                                        let mut context = HashMap::new();
                                        context.insert("operation".to_string(), "rename".to_string());
                                        context.insert("old_name".to_string(), effective_old_name.to_string());
                                        context.insert("new_name".to_string(), new_command.command.clone());
                                        context.insert("current_arg".to_string(), new_command.arg.clone());
                                        context.insert("action".to_string(), new_command.action.clone());
                                        context.insert("original_command_to_delete".to_string(), 
                                            command_to_delete.clone().unwrap_or_default());
                                        
                                        // Store the new command details in context
                                        context.insert("new_command_action".to_string(), new_command.action.clone());
                                        context.insert("new_command_arg".to_string(), new_command.arg.clone());
                                        context.insert("new_command_patch".to_string(), new_command.patch.clone());
                                        context.insert("new_command_flags".to_string(), new_command.flags.clone());
                                        
                                        // Show confirmation dialog with actions
                                        let action_list = actions.join("\n• ");
                                        let dialog_spec = vec![
                                            "=Confirm Rename".to_string(),
                                            format!("'Renaming \"{}\" to \"{}\"", effective_old_name, new_command.command),
                                            format!("&The following changes will be made:\n\n• {}", action_list),
                                            "!OK".to_string(),
                                            "!Cancel".to_string(),
                                        ];
                                        
                                        // Queue the rename action - action callback simplified for UI thread execution
                                        
                                        self.queue_action(context, Box::new(move |final_context| {
                                            let empty_string = String::new();
                                            let exit_button = final_context.get("exit").unwrap_or(&empty_string);
                                            if exit_button == "OK" {
                                                // User confirmed - we need to execute this in the main UI thread
                                                // Store the rename execution details for the main update loop
                                                // This will be handled in complete_action where we have &mut self
                                                Ok(())
                                            } else {
                                                // Cancel - do nothing
                                                Ok(())
                                            }
                                        }));
                                        
                                        self.dialog.show(dialog_spec);
                                        self.close_command_editor();
                                        command_editor_just_closed = true;
                                        return; // Exit early to wait for user confirmation
                                    } else {
                                        // No side effects found by dry-run - proceed with direct rename
                                        crate::utils::log(&format!("RENAME_DRY_RUN: No side effects found, proceeding with normal save for '{}' -> '{}'", effective_old_name, new_command.command));
                                        // Update arg if it was changed by rename logic
                                        new_command.arg = updated_arg;
                                    }
                                }
                                Err(e) => {
                                    self.show_error_dialog(&format!("Error checking rename effects: {}", e));
                                }
                            }
                        }
                    }
                }
                
                // No rename side effects or user not using rename features - proceed with normal save
                
                // Delete original command if needed
                if let Some(cmd_name) = command_to_delete {
                    use crate::core::commands::delete_command;
                    let deleted = delete_command(&cmd_name, self.commands_mut());
                    if deleted.is_err() {
                        crate::utils::log_error(&format!("Original command '{}' not found for deletion", cmd_name));
                    }
                }
                
                // Clone the new command for template processing before adding it
                let saved_command = new_command.clone();
                
                // Add the new command
                use crate::core::commands::{add_command, save_commands_to_file};
                let _ = add_command(new_command, self.commands_mut());
                
                // Save to file
                match save_commands_to_file(&self.commands()) {
                    Ok(_) => {
                        // Process template files if there was a pending template
                        if let (Some(template), Some(context)) = (
                            self.command_editor.pending_template.as_ref(),
                            self.command_editor.template_context.as_ref()
                        ) {
                            crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: Processing template files after save"));
                            if let Err(e) = crate::core::template_creation::process_template_files(
                                template,
                                context,
                                &saved_command
                            ) {
                                self.show_error_dialog(&format!("Failed to create template files: {}", e));
                            } else {
                                crate::utils::detailed_log("TEMPLATE", &format!("TEMPLATE: Successfully created template files"));
                                // Trigger rescan if requested
                                if template.file_rescan {
                                    self.trigger_rescan();
                                }
                            }
                        }
                        
                        // Update the filtered list if we're currently filtering
                        if !self.popup_state.search_text.trim().is_empty() {
                            // Refresh the search with updated commands
                            let current_search = self.popup_state.search_text.clone();
                            self.popup_state.update_search(current_search);
                        }
                    }
                    Err(e) => {
                        crate::utils::log_error(&format!("Error saving commands to file: {}", e));
                    }
                }
                
                // Clear the pending template now that it's been processed
                self.command_editor.pending_template = None;
                self.command_editor.template_context = None;
                
                self.close_command_editor();
                command_editor_just_closed = true;
            }
            CommandEditorResult::Delete(command_name) => {
                // Delete the specified command and save to file
                use crate::core::commands::{delete_command, save_commands_to_file};
                
                let deleted = delete_command(&command_name, self.commands_mut());
                if deleted.is_err() {
                    crate::utils::log_error(&format!("Command '{}' not found for deletion", command_name));
                } else {
                    // Save the updated command list back to commands.txt
                    if let Err(e) = save_commands_to_file(&self.commands()) {
                        crate::utils::log_error(&format!("Error saving commands to file after deletion: {}", e));
                    } else {
                        // Update the filtered list if we're currently filtering
                        if !self.popup_state.search_text.trim().is_empty() {
                            // Refresh the search with updated commands
                            let current_search = self.popup_state.search_text.clone();
                            self.popup_state.update_search(current_search);
                        }
                    }
                }
                self.close_command_editor();
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
                let _keys_processed = self.handle_keys(ctx);
                
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
                // Check for Enter key BEFORE creating the TextEdit widget
                let enter_pressed = ctx.input(|i| i.key_pressed(egui::Key::Enter));
                if enter_pressed {
                }
                
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
                
                // Check if Enter was pressed (checked before TextEdit could consume it)
                if enter_pressed && response.has_focus() && !self.filtered_commands().is_empty() {
                    self.execute_selected_command();
                    // Clear the Enter key from input to prevent double processing
                    ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
                } else if response.changed() {
                    // Always update search when text field is changed
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
                let should_focus = (!self.focus_set && self.frame_count <= 15) || 
                    command_editor_just_closed || 
                    self.request_focus;
                
                if should_focus {
                    // On early frames, also try to activate the window to ensure proper focus
                    if self.frame_count <= 10 && !self.window_activated {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                        self.window_activated = true;
                    }
                    
                    response.request_focus();
                    if response.has_focus() {
                        self.focus_set = true;
                        self.request_focus = false;  // Clear the request
                        crate::utils::detailed_log("FOCUS", &format!("Focus successfully set on frame {}", self.frame_count));
                    } else if self.frame_count % 5 == 0 && self.frame_count <= 15 {
                        // Log focus attempts every 5 frames for debugging
                        crate::utils::detailed_log("FOCUS", &format!("Focus attempt on frame {} - window focused: {}", 
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
                            let config = crate::core::sys_data::get_config();
                            let window_width = config.popup_settings.get_default_window_width() as f32;
                            let required_height = input_height + mid_drag_height + header_height + (display_commands_single.len() as f32 * row_height) + bottom_drag_height + padding;
                            (window_width, required_height)
                        }
                    };
                    
                    // Determine the correct window size based on current mode
                    let (final_width, final_height) = if self.command_editor.visible {
                        // Use calculated editor size when command editor is open
                        let editor_size = self.calculate_editor_size();
                        (editor_size.x, editor_size.y)
                    } else {
                        (window_width, required_height)
                    };
                    
                    // Apply the calculated window size to ensure it fits the content
                    let config = crate::core::sys_data::get_config();
                    let max_width = config.popup_settings.get_max_window_width() as f32;
                    let max_height = config.popup_settings.get_max_window_height() as f32;
                    let default_width = config.popup_settings.get_default_window_width() as f32;
                    
                    // Ensure the calculated size respects our constraints
                    let constrained_width = final_width.max(default_width).min(max_width);
                    let constrained_height = final_height.min(max_height);
                    
                    // Only resize if the size actually changed significantly to avoid constant resizing
                    let current_size = ctx.screen_rect().size();
                    let size_diff_width = (current_size.x - constrained_width).abs();
                    let size_diff_height = (current_size.y - constrained_height).abs();
                    
                    if size_diff_width > 10.0 || size_diff_height > 10.0 {
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(constrained_width, constrained_height)));
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
                                        let patch_path_display = self.get_patch_path_display(prefix);
                                        ui.label(egui::RichText::new(patch_path_display).font(header_font_id));
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
                                            // This is an inside menu command - apply trimming logic using original command name
                                            if let Some((original_command, resolved_command)) = self.popup_state.get_submenu_command_info() {
                                                let original_prefix = &original_command.command;
                                                
                                                // Check if this command should have its prefix trimmed
                                                if cmd.command.len() > original_prefix.len() {
                                                    let prefix_end = original_prefix.len();
                                                    if cmd.command[..prefix_end].eq_ignore_ascii_case(original_prefix) {
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
                                                
                                                // Save the last executed command for add_alias functionality
                                                use crate::core::state::save_last_executed_command;
                                                crate::utils::detailed_log("STATE_SAVE", &format!("POPUP_CLICK: Attempting to save last executed command: '{}'", cmd.command));
                                                match save_last_executed_command(&cmd.command) {
                                                    Ok(_) => crate::utils::detailed_log("STATE_SAVE", "POPUP_CLICK: Successfully saved last executed command"),
                                                    Err(e) => crate::utils::detailed_log("STATE_SAVE", &format!("POPUP_CLICK: Failed to save last executed command: {}", e)),
                                                }
                                                
                                                // Execute command - handles all retries internally
                                                let action = crate::execute::command_to_action(&cmd);
                                                let _ = crate::execute::execute_on_server(&action);
                                                // Don't perform scanner check - it blocks
                                                self.exit_or_hide(ctx);
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
                                    let patch_path_display = self.get_patch_path_display(prefix);
                                    ui.label(egui::RichText::new(patch_path_display).font(header_font_id));
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
                                    // This is an inside menu command - apply trimming logic using original command name
                                    if let Some((original_command, resolved_command)) = self.popup_state.get_submenu_command_info() {
                                        let original_prefix = &original_command.command;
                                        
                                        // Check if this command should have its prefix trimmed
                                        if cmd.command.len() > original_prefix.len() {
                                            let prefix_end = original_prefix.len();
                                            if cmd.command[..prefix_end].eq_ignore_ascii_case(original_prefix) {
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
                                        
                                        // Save the last executed command for add_alias functionality
                                        use crate::core::state::save_last_executed_command;
                                        crate::utils::detailed_log("STATE_SAVE", &format!("POPUP_CLICK2: Attempting to save last executed command: '{}'", cmd.command));
                                        match save_last_executed_command(&cmd.command) {
                                            Ok(_) => crate::utils::detailed_log("STATE_SAVE", "POPUP_CLICK2: Successfully saved last executed command"),
                                            Err(e) => crate::utils::detailed_log("STATE_SAVE", &format!("POPUP_CLICK2: Failed to save last executed command: {}", e)),
                                        }
                                        
                                        // Execute command - handles all retries internally
                                        let action = crate::execute::command_to_action(&cmd);
                                        let _ = crate::execute::execute_on_server(&action);
                                        // Don't perform scanner check - it blocks
                                        self.exit_or_hide(ctx);
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
                crate::utils::detailed_log("WINDOW_POS", &format!("First position detection: {:?}", current_pos));
                // Only save if this position was explicitly set by user, not auto-positioned
                if !self.was_auto_positioned {
                    save_window_position(current_pos);
                }
                self.last_saved_position = Some(current_pos);
            } else if let Some(last_pos) = self.last_saved_position {
                let moved = (current_pos.x - last_pos.x).abs() > 10.0 || (current_pos.y - last_pos.y).abs() > 10.0;
                if moved {
                    crate::utils::detailed_log("WINDOW_POS", &format!("Window moved from {:?} to {:?} (user moved)", last_pos, current_pos));
                    // Only save significant moves that are likely user-initiated
                    save_window_position(current_pos);
                    self.last_saved_position = Some(current_pos);
                    self.was_auto_positioned = false; // User moved it, so it's no longer auto-positioned
                }
            }
        } else {
            crate::utils::detailed_log("WINDOW_POS", "Could not get current position from viewport");
        }
        
    }
}

/// Embedded icon data compiled into the binary at build time
#[allow(dead_code)]
static EMBEDDED_ICON_PNG: &[u8] = include_bytes!("../../resources/icons/popup.png");

/// Cached icon data to avoid repeated PNG decoding
#[allow(dead_code)]
static CACHED_ICON: OnceLock<IconData> = OnceLock::new();

/// Load app icon with compile-time embedded data (no file I/O or ICNS parsing)
#[allow(dead_code)]
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
            crate::utils::detailed_log("ICON", "Failed to decode embedded icon, using fallback");
            create_fallback_icon()
        }
    }).clone()
}


#[allow(dead_code)]
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

#[allow(dead_code)]
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

/// Wrapper that includes popup control socket
struct PopupWithControl {
    popup: AnchorSelector,
    control: crate::systems::popup_server::PopupControl,
}

impl PopupWithControl {
    fn new(initial_prompt: &str) -> Self {
        let control = crate::systems::popup_server::PopupControl::new();
        control.start_listener();
        
        Self {
            popup: AnchorSelector::new_with_prompt(initial_prompt),
            control,
        }
    }
}

impl eframe::App for PopupWithControl {
    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        self.popup.clear_color(visuals)
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Process any control commands first
        if let Some(command) = self.control.process_commands(ctx) {
            match command {
                crate::systems::popup_server::PopupCommand::Show => {
                    crate::utils::log("===== SHOW COMMAND RECEIVED =====");
                    crate::utils::log(&format!("[SHOW] Current frame: {}", self.popup.frame_count));
                    crate::utils::log(&format!("[SHOW] is_hidden={}, should_exit={}", 
                        self.popup.is_hidden, self.popup.should_exit));
                    
                    // Get current window state
                    let viewport_info = ctx.input(|i| {
                        format!("Viewport: focused={}, visible={:?}, position={:?}",
                            i.focused,
                            i.viewport().inner_rect,
                            i.viewport().outer_rect
                        )
                    });
                    crate::utils::log(&format!("[SHOW] Before: {}", viewport_info));
                    
                    // Restore saved position BEFORE making window visible to avoid flashing at (0,0)
                    if let Some(saved_pos) = load_window_position() {
                        crate::utils::log(&format!("[SHOW] Pre-setting saved position: {:?}", saved_pos));
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(saved_pos));
                    }
                    
                    // Make the window visible again if it was hidden
                    crate::utils::detailed_log("SHOW", "Sending ViewportCommand::Visible(true)");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    
                    crate::utils::detailed_log("SHOW", "Sending ViewportCommand::Focus");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                    
                    // Also try to bring to front
                    crate::utils::detailed_log("SHOW", "Sending ViewportCommand::Fullscreen(false) to ensure windowed mode");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
                    
                    crate::utils::detailed_log("SHOW", "Sending ViewportCommand::Minimized(false)");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                    
                    // Use AppleScript to ensure the window comes to front on macOS
                    crate::utils::detailed_log("SYSTEM", &format!("[SHOW] Using AppleScript to activate popup_server"));
                    let activate_script = r#"tell application "System Events"
                        set frontProcess to first process whose unix id is "#.to_string() + &std::process::id().to_string() + r#"
                        set frontmost of frontProcess to true
                    end tell"#;
                    
                    if let Err(e) = std::process::Command::new("osascript")
                        .arg("-e")
                        .arg(&activate_script)
                        .output() {
                        crate::utils::log(&format!("[SHOW] AppleScript activation failed: {}", e));
                    } else {
                        crate::utils::log("[SHOW] AppleScript activation succeeded");
                    }
                    
                    // CRITICAL: Position window on screen if it's off-screen
                    let current_pos = ctx.input(|i| i.viewport().outer_rect);
                    if let Some(rect) = current_pos {
                        // Get actual screen dimensions using CoreGraphics on macOS
                        let (screen_width, screen_height) = get_actual_screen_dimensions();
                        
                        crate::utils::detailed_log("WINDOW_POS", &format!("[SHOW] Actual screen dimensions: {}x{}", 
                            screen_width, screen_height));
                        crate::utils::detailed_log("WINDOW_POS", &format!("[SHOW] Current window rect: {:?}", rect));
                        
                        // Check if window is truly off-screen using actual screen dimensions
                        // Allow some margin (50 pixels) for window decorations
                        let margin = 50.0;
                        let is_offscreen = rect.min.x < -margin || 
                                          rect.min.y < -margin || 
                                          rect.min.x > screen_width - margin ||
                                          rect.min.y > screen_height - margin;
                        
                        crate::utils::detailed_log("WINDOW_POS", &format!("[SHOW] Off-screen check: x={} < {} OR y={} < {} OR x={} > {} OR y={} > {} = {}", 
                            rect.min.x, -margin, rect.min.y, -margin, 
                            rect.min.x, screen_width - margin, rect.min.y, screen_height - margin, is_offscreen));
                        
                        if is_offscreen {
                            crate::utils::log(&format!("[SHOW] Window is off-screen at {:?} (screen: {}x{}), centering on main display", 
                                rect.min, screen_width, screen_height));
                            // Center on main display but ensure it's not too close to bottom
                            let window_size = rect.size();
                            let center_x = (screen_width - window_size.x) / 2.0;
                            let center_y = (screen_height - window_size.y) / 3.0;
                            // Ensure window is at least 100 pixels from bottom
                            let max_y = (screen_height - window_size.y - 100.0).max(0.0);
                            let center_y = center_y.min(max_y);
                            let center_pos = egui::pos2(center_x.max(0.0), center_y.max(0.0));
                            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(center_pos));
                            self.popup.was_auto_positioned = true; // Mark as auto-positioned
                            crate::utils::log(&format!("[SHOW] Repositioned window to {:?} (auto-centered)", center_pos));
                        } else {
                            // Window is on-screen, restore saved position if different
                            if let Some(mut saved_pos) = load_window_position() {
                                // Ensure saved position is not too close to bottom
                                let window_size = rect.size();
                                let max_y = (screen_height - window_size.y - 100.0).max(0.0);
                                if saved_pos.y > max_y {
                                    crate::utils::log(&format!("[SHOW] Adjusting saved position from y={} to y={} (too close to bottom)", saved_pos.y, max_y));
                                    saved_pos.y = max_y;
                                    self.popup.was_auto_positioned = true; // Mark as auto-positioned since we adjusted it
                                }
                                
                                let pos_diff = (rect.min.x - saved_pos.x).abs() + (rect.min.y - saved_pos.y).abs();
                                crate::utils::log(&format!("[SHOW] Current position: {:?}, Saved position: {:?}, Diff: {}", rect.min, saved_pos, pos_diff));
                                if pos_diff > 5.0 {  // Only restore if significantly different
                                    crate::utils::log(&format!("[SHOW] Restoring saved position {:?} (current: {:?})", saved_pos, rect.min));
                                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(saved_pos));
                                    // If we didn't adjust the position, it's user-set, not auto-positioned
                                    if !self.popup.was_auto_positioned {
                                        self.popup.was_auto_positioned = false;
                                    }
                                } else {
                                    crate::utils::log("[SHOW] Position is close to saved, not restoring");
                                }
                            } else {
                                crate::utils::log("[SHOW] No saved position available");
                            }
                        }
                    } else {
                        // No position available, center the window
                        crate::utils::log("[SHOW] No window position available, centering on main display");
                        let config = crate::core::sys_data::get_config();
                        let width = config.popup_settings.get_default_window_width() as f32;
                        let height = config.popup_settings.get_default_window_height() as f32;
                        let window_size = egui::vec2(width, height);
                        // Get actual screen dimensions
                        let (screen_width, screen_height) = get_actual_screen_dimensions();
                        let center_x = (screen_width - window_size.x) / 2.0;
                        let center_y = (screen_height - window_size.y) / 3.0;
                        // Ensure window is at least 100 pixels from bottom
                        let max_y = (screen_height - window_size.y - 100.0).max(0.0);
                        let center_y = center_y.min(max_y);
                        let center_pos = egui::pos2(center_x.max(0.0), center_y.max(0.0));
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(center_pos));
                        crate::utils::log(&format!("[SHOW] Positioned new window to {:?}", center_pos));
                    }
                    
                    // Clear the search input when showing the popup
                    self.popup.popup_state.search_text.clear();
                    self.popup.popup_state.update_search(String::new());
                    // Request focus on the input field and reset focus tracking flags
                    self.popup.request_focus = true;
                    self.popup.focus_set = false;  // Reset so focus logic will trigger again
                    self.popup.window_activated = false;  // Reset so window activation will trigger again
                    self.popup.frame_count = 0;  // Reset frame count so early-frame focus logic will run
                    // Mark window as not hidden and reset interaction time
                    crate::utils::log(&format!("[SHOW] Setting is_hidden=false (was {})", self.popup.is_hidden));
                    self.popup.is_hidden = false;
                    crate::utils::log(&format!("[SHOW] Setting should_exit=false (was {})", self.popup.should_exit));
                    self.popup.should_exit = false;  // Reset exit flag
                    self.popup.last_interaction_time = std::time::Instant::now();
                    
                    // Force a repaint
                    crate::utils::log("[SHOW] Requesting repaint");
                    ctx.request_repaint();
                    
                    crate::utils::log(&format!("[SHOW] After: is_hidden={}, should_exit={}", 
                        self.popup.is_hidden, self.popup.should_exit));
                    crate::utils::log("===== SHOW COMMAND COMPLETE =====");
                }
                crate::systems::popup_server::PopupCommand::Hide => {
                    crate::utils::log("===== HIDE COMMAND RECEIVED =====");
                    // Call exit_or_hide to properly hide the window
                    self.popup.exit_or_hide(ctx);
                    crate::utils::log("===== HIDE COMMAND COMPLETE =====");
                }
                crate::systems::popup_server::PopupCommand::Ping => {
                    // No special handling needed
                }
            }
        }
        
        // Then update the popup
        self.popup.update(ctx, frame);
    }
}

pub fn run_gui_with_prompt(initial_prompt: &str, _app_state: super::ApplicationState) -> Result<(), eframe::Error> {
    // Debug: Log when popup is being opened
    crate::utils::detailed_log("SYSTEM", &format!("===== POPUP GUI STARTING ====="));
    crate::utils::detailed_log("POPUP_OPEN", &format!("Opening popup with initial prompt: '{}'", initial_prompt));
    
    // Capture the prompt for the closure
    let prompt = initial_prompt.to_string();
    
    // Manual window sizing - no auto-sizing constraints
    let config = crate::core::sys_data::get_config();
    let width = config.popup_settings.get_default_window_width() as f32;
    let height = config.popup_settings.get_default_window_height() as f32;
    // Always start visible - we'll hide it later if needed
    let start_visible = true;
    
    crate::utils::detailed_log("SYSTEM", &format!("[POPUP_INIT] Window size: {}x{}, start_visible: {}", width, height, start_visible));
    
    // Load saved position or use centered position
    let initial_position = if let Some(saved_pos) = load_window_position() {
        crate::utils::detailed_log("WINDOW_POS", &format!("[STARTUP] Using saved window position: {:?}", saved_pos));
        [saved_pos.x, saved_pos.y]
    } else {
        // Center on screen
        let screen_width = 1920.0; // Default fallback
        let screen_height = 1080.0; // Default fallback
        let x = (screen_width - width) / 2.0;
        let y = (screen_height - height) / 2.0;
        crate::utils::detailed_log("WINDOW_POS", &format!("[STARTUP] No saved position, centering at: [{}, {}]", x, y));
        [x, y]
    };
    
    let viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size([width, height]) // Initial size - will be dynamically resized
        .with_position(initial_position) // Set initial position from saved state
        .with_resizable(false) // Disable manual resizing - we control size programmatically
        .with_decorations(false) // Remove title bar and window controls
        .with_transparent(true) // Enable transparency for rounded corners
        .with_visible(start_visible) // Start visible only if not running in background
        .with_always_on_top(); // Keep on top so it's visible
        // Skip icon loading for faster startup - can be added later if needed
    
    crate::utils::detailed_log("WINDOW_POS", &format!("[VIEWPORT] ViewportBuilder configured with position: {:?}", initial_position));
    
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
            
            Ok(Box::new(PopupWithControl::new(&prompt)))
        }),
    )
}