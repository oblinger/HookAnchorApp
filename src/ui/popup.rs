//! Popup interface module for the Anchor Selector
//!
//! This module contains the AnchorSelector struct and all popup-specific UI logic.

use crate::prelude::*;
use eframe::egui::{self, IconData};
use std::sync::OnceLock;
use std::collections::HashMap;
use crate::core::Command;
use crate::core::{
    Config, AppState
};
use crate::core::key_processing::{PopupInterface, KeyRegistry, create_default_key_registry};
use crate::core::commands::{get_patch_path, FLAG_MERGED};

#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;
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
    /// Cached window position for external dialogs (x, y, screen_width)
    cached_window_position: Option<(f32, f32, f32)>,
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
    /// Request cursor position at end of text on next frame
    request_cursor_at_end: bool,
    /// Frame counter to track how many frames have passed since startup
    frame_count: u32,
    /// Track if window activation has been attempted
    window_activated: bool,
    /// Config error to show in dialog if config loading failed
    config_error: Option<String>,
    /// Last user interaction time for idle timeout
    last_interaction_time: std::time::Instant,
    /// Track when show command was received for timing measurements
    show_command_start: Option<std::time::Instant>,
    /// Loading state for deferred initialization
    loading_state: LoadingState,
    /// Buffer for keyboard input captured before full initialization
    pre_init_input_buffer: String,
    /// Flag to indicate search needs to be computed after loading is complete
    search_pending_after_load: bool,
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
    /// Last modification time of state.json file (for efficient polling)
    last_state_mod_time: Option<std::time::SystemTime>,
    /// Action to execute immediately after initialization (from --action flag)
    initial_action_name: Option<String>,
    /// External dialog state (subprocess-based dialog)
    external_dialog_state: Option<ExternalDialogState>,
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
}

/// Action to be executed after user confirms via dialog or other UI component
pub struct PendingAction {
    /// Context data passed to the action
    pub context: HashMap<String, String>,
    /// Callback function to execute with final context (including user input)
    pub callback: Box<dyn FnOnce(&HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>>>,
}

/// External dialog state - tracks active dialog subprocess
struct ExternalDialogState {
    /// Dialog subprocess handle
    handle: crate::systems::DialogHandle,
    /// Context data for the action
    context: HashMap<String, String>,
    /// Callback to execute when dialog completes
    callback: Box<dyn FnOnce(&HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>>>,
}

impl AnchorSelector {
    // =============================================================================
    // Window Size Management
    // =============================================================================

    /// Truncate command name if it exceeds max_characters from config
    fn truncate_command_name(&self, text: String) -> String {
        if let Some(max_len) = self.popup_state.config.popup_settings.max_characters {
            if text.len() > max_len {
                format!("{}...", &text[..max_len.saturating_sub(3)])
            } else {
                text
            }
        } else {
            text
        }
    }

    /// Determine the appropriate window size mode based on current UI state
    fn determine_window_size_mode(&self) -> WindowSizeMode {
        if self.dialog.is_visible() {
            WindowSizeMode::Dialog
        } else if self.command_editor.visible {
            WindowSizeMode::Editor
        } else {
            WindowSizeMode::Normal
        }
    }
    
    /// Calculate the required window size for normal mode (command list)
    fn calculate_normal_size(&self, command_count: usize) -> egui::Vec2 {
        let config = crate::core::data::get_config();
        let default_width = config.popup_settings.get_default_window_width() as f32;
        let max_width = config.popup_settings.get_max_window_width() as f32;
        let max_height = config.popup_settings.get_max_window_height() as f32;
        
        // Start with default width, but allow it to grow based on content
        let mut calculated_width = default_width;
        
        // If we have commands, calculate optimal width based on content
        if command_count > 0 {
            let display_commands = &self.popup_state.display_commands;

            // Estimate the width needed for the longest command text
            let mut max_text_width: f32 = 0.0;
            for cmd in display_commands {
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
        let config = crate::core::data::get_config();
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
        // TODO: Finish window sizing debugging
        // log(&format!("🪟 WINDOW_SIZING: Mode detected={:?}, dialog.visible={}, command_editor.visible={}",
        //     new_mode, self.dialog.visible, self.command_editor.visible));
        
        // Calculate the required size for the new mode
        let required_size = match new_mode {
            WindowSizeMode::Normal => {
                // Use same logic as display: only count commands if user has typed something (not just last anchor text)
                let command_count = if self.loading_state == LoadingState::Loaded
                    && !self.filtered_commands().is_empty()
                    && !self.popup_state.raw_search_text().is_empty() {
                    self.popup_state.display_commands.len()
                } else {
                    0 // No commands to display, so size for empty state
                };
                self.calculate_normal_size(command_count)
            }
            WindowSizeMode::Editor => self.calculate_editor_size(),
            WindowSizeMode::Dialog => {
                // TEST: Force main window to 1000x1000 to verify this code path is working
                detailed_log("WINDOW_SIZING", "Dialog mode - calculating size for dialog");
                let size = egui::vec2(1000.0, 1000.0);
                detailed_log("WINDOW_SIZING", &format!("Dialog mode calculated size: {}x{}", size.x, size.y));
                size
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
        
        // TODO: Finish window sizing debugging
        // log(&format!("🪟 WINDOW_SIZING: Should resize? {}, current_mode={:?}, new_mode={:?}, required_size={}x{}",
        //     should_resize, self.window_size_mode, new_mode, required_size.x, required_size.y));
            
        if should_resize {
            // Get current position BEFORE resizing to maintain top-left anchor
            let current_pos = ctx.input(|i| i.viewport().outer_rect.map(|r| r.min)).unwrap_or(egui::pos2(100.0, 100.0));

            detailed_log("WINDOW_SIZING", &format!("SENDING RESIZE COMMAND! position={:?}, mode={:?}, target_size={}x{}",
                current_pos, new_mode, required_size.x, required_size.y));

            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(required_size));

            // CRITICAL: Re-anchor position after resize to prevent drift
            // This ensures the top-left corner stays in the same place
            detailed_log("POSITION", &format!("Re-anchoring position after resize to {:?} (reason: prevent drift)", current_pos));
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(current_pos));
            
            // Update the mode and cache the size
            self.window_size_mode = new_mode.clone();
            match new_mode {
                WindowSizeMode::Normal => self.last_normal_size = Some(required_size),
                WindowSizeMode::Editor => self.last_editor_size = Some(required_size),
                WindowSizeMode::Dialog => self.last_dialog_size = Some(required_size),
            }
            
            detailed_log("WINDOW_SIZE", &format!(
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
                detailed_log("EXIT_OR_HIDE", "Already hiding, returning early");
                return; // Already hiding, don't call again
            }
            HIDING = true;
        }
        
        // Check if we're in direct mode (set by launcher via environment variable)
        let direct_mode = std::env::var("HOOKANCHOR_DIRECT_MODE").is_ok();
        detailed_log("EXIT_OR_HIDE", &format!("Direct mode: {}", direct_mode));
        
        if !direct_mode {
            // Server mode - hide window using AppleScript
            detailed_log("EXIT_OR_HIDE", "Server mode - hiding window");
            
            // Save current position before hiding
            if let Some(current_pos) = ctx.input(|i| i.viewport().outer_rect.map(|r| r.min)) {
                log(&format!("[EXIT_OR_HIDE] Saving position before hide: {:?}", current_pos));
                save_window_position_with_reason(current_pos, "hiding window");
                self.last_saved_position = Some(current_pos);
            }
            
            // Clear the search input for next time
            self.popup_state.search_text.clear();
            self.popup_state.update_search(String::new());

            // Hide the window using egui's viewport command
            // Note: popup_server is already hidden from Cmd+Tab on startup,
            // so we don't need to hide it here
            detailed_log("EXIT_OR_HIDE", "Sending ViewportCommand::Visible(false)");
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            
            detailed_log("EXIT_OR_HIDE", &format!("Setting is_hidden=true (was {})", self.is_hidden));
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
            detailed_log("EXIT_OR_HIDE", &viewport_info);
            detailed_log("EXIT_OR_HIDE", "Window hidden via viewport commands");
            // Reset the flag after a delay to allow for future hide operations
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(100));
                unsafe { 
                    HIDING = false;
                    detailed_log("EXIT_OR_HIDE", "HIDING flag reset to false");
                }
            });
        } else {
            // Direct mode - exit the application
            log("EXIT: Exiting application (direct mode)");
            std::process::exit(0);
        }
    }
    
    /// Perform scanner check before exiting to update commands for next launch
    fn perform_exit_scanner_check(&mut self) {
        if self.scanner_check_pending {
            self.scanner_check_pending = false;
            let current_commands = crate::core::data::get_commands();
            let updated_commands = crate::systems::scan_check(current_commands.clone());
            if updated_commands.len() != current_commands.len() {
                // Commands have changed, save them back to singleton
                // No need to update popup_state - it fetches fresh from singleton
                let _ = crate::core::data::set_commands(updated_commands);
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
        // 🔑 DEBUG: Log EVERY key event that comes in, regardless of handling
        ctx.input(|input| {
            if !input.events.is_empty() {
                detailed_log("KEY_DEBUG", &format!("🔑 ALL EVENTS: {} total events detected", input.events.len()));
                for (i, event) in input.events.iter().enumerate() {
                    detailed_log("KEY_DEBUG", &format!("🔑 EVENT[{}]: {:?}", i, event));

                    // Special handling for key events
                    if let egui::Event::Key { key, pressed, modifiers, .. } = event {
                        detailed_log("KEY_DEBUG", &format!("🔑 KEY_EVENT: key={:?}, pressed={}, modifiers={{ctrl:{}, alt:{}, shift:{}, cmd:{}}}",
                            key, pressed, modifiers.ctrl, modifiers.alt, modifiers.shift, modifiers.command));
                    }
                }
            }

            // Also log current key states
            if !input.keys_down.is_empty() {
                let keys: Vec<String> = input.keys_down.iter().map(|k| format!("{:?}", k)).collect();
                detailed_log("KEY_DEBUG", &format!("🔑 KEYS_DOWN: [{}]", keys.join(", ")));
            }
        });

        // Check which interface is active first
        if self.command_editor.visible {
            // When command editor is visible, we don't process ANY keys here
            // The command editor will handle its own keys in its update() method
            return false; // No keys processed here
        }
        
        if self.dialog.is_visible() {
            // When dialog is visible, we don't process ANY keys here
            // The dialog will handle its own keys in its update() method
            return false; // No keys processed here
        }
        
        // Neither command editor nor dialog is open, handle popup keys
        // Extract events - only needed when we're actually going to process them
        let mut events_to_process = Vec::new();
        ctx.input_mut(|input| {
            events_to_process = input.events.clone();
            // TODO: Remove DEBUG - Commented out mouse/pointer event logging to reduce noise
            // Only log key events, not mouse/pointer events
            // if !input.events.is_empty() {
            //     log(&format!("🔍 RAW_INPUT: Captured {} total events", input.events.len()));
            //     for event in &input.events {
            //         log(&format!("🔍 RAW_EVENT: {:?}", event));
            //     }
            // }
        });
        
        return self.handle_popup_keys_with_events(ctx, events_to_process);
    }
    
    /// Handle keys for the main popup interface using registry system
    /// Returns true if any keys were processed
    fn handle_popup_keys_with_events(&mut self, ctx: &egui::Context, events_to_process: Vec<egui::Event>) -> bool {
        // If still loading, only handle escape key
        if self.loading_state != LoadingState::Loaded {
            detailed_log("KEY_REGISTRY", &format!("Cannot process keys, loading_state = {:?}", self.loading_state));
            return false; // No keys processed during loading
        }
        
        // Check if we have registry and process events
        if self.key_registry.is_some() {
            // Take ownership of the registry temporarily to avoid borrowing issues
            let registry = self.key_registry.take().unwrap();
            
            // Log all key events for debugging
            for event in &events_to_process {
                if let egui::Event::Key { key, pressed, modifiers, .. } = event {
                    if *pressed {
                        detailed_log("KEY_DEBUG", &format!("Processing key event: {:?} + {:?}", key, modifiers));
                    }
                }
            }
            
            // TODO: Remove DEBUG - Log exactly what events are being sent to key registry
            if !events_to_process.is_empty() {
                detailed_log("KEY_REGISTRY", &format!("🔑 SENDING {} events to registry.process_events()", events_to_process.len()));
                for (i, event) in events_to_process.iter().enumerate() {
                    detailed_log("KEY_REGISTRY", &format!("🔑 REGISTRY_INPUT[{}]: {:?}", i, event));
                }
            }

            let handled = registry.process_events(&events_to_process, self, ctx);

            // TODO: Remove DEBUG - Log what the registry returned
            if !events_to_process.is_empty() {
                detailed_log("KEY_REGISTRY", &format!("🔑 REGISTRY RETURNED: handled={}", handled));
            }
            
            // Put the registry back
            self.key_registry = Some(registry);
            
            // Log handling results
            if !events_to_process.is_empty() {
                detailed_log("KEY_DEBUG", &format!("Processed {} events, handled={}", events_to_process.len(), handled));
            }
            
            if handled {
                ctx.input_mut(|input| {
                    // Clear all processed events from input
                    input.events.clear();
                    // Also clear key states to prevent stuck keys
                    input.keys_down.clear();
                });
            } else if !events_to_process.is_empty() {
                // Log when events were not handled
                let key_events: Vec<String> = events_to_process.iter()
                    .filter_map(|event| {
                        if let egui::Event::Key { key, pressed, modifiers, .. } = event {
                            if *pressed {
                                Some(format!("{:?}+{:?}", key, modifiers))
                            } else { None }
                        } else { None }
                    })
                    .collect();
                if !key_events.is_empty() {
                    detailed_log("KEY_DEBUG", &format!("UNHANDLED KEY EVENTS: {}", key_events.join(", ")));
                }
            }
            
            handled // Return whether any keys were actually handled
        } else {
            // Fallback: registry not initialized yet
            log_error("KEY_REGISTRY: Registry not initialized, cannot process keys");
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
    fn activate_tmux(&mut self) {
        self.activate_tmux();
    }

    fn activate_anchor(&mut self) {
        // Get the current input text to check if it's an anchor name
        let input_text = self.popup_state.search_text.clone();

        if input_text.is_empty() {
            // If input is empty, just consume the comma key and do nothing
            return;
        }

        // Get all commands
        let commands = crate::core::data::get_commands();

        // Find a command that matches the input text (case-insensitive)
        if let Some(cmd) = commands.iter().find(|c| c.command.to_lowercase() == input_text.to_lowercase()) {
            // Resolve any aliases to get the final target command
            let resolved_cmd = cmd.resolve_alias(&commands);

            // Check if the resolved command is an anchor
            let is_anchor = resolved_cmd.action == "anchor" || resolved_cmd.flags.contains('A');

            if is_anchor {
                // Found an anchor (either directly or via alias) - set it as active and clear the input
                match crate::core::data::set_active_anchor(resolved_cmd.command.clone(), None) {
                    Ok(()) => {
                        log(&format!("✅ Activated anchor: '{}'", resolved_cmd.command));
                        // Clear the search text after successful activation
                        self.popup_state.search_text.clear();
                        self.popup_state.update_search(String::new());
                    },
                    Err(e) => {
                        log_error(&format!("Failed to activate anchor '{}': {}", resolved_cmd.command, e));
                    }
                }
            } else {
                // Command exists but resolves to something that's not an anchor
                detailed_log("ANCHOR_ACTIVATE", &format!("'{}' resolves to '{}' which is not an anchor, input left unchanged", input_text, resolved_cmd.command));
            }
        } else {
            // No matching command found - just consume the comma key, leave input text as-is
            // User can edit and try again
            detailed_log("ANCHOR_ACTIVATE", &format!("'{}' is not a valid command name, input left unchanged", input_text));
        }
    }

    fn create_child(&mut self) {
        self.create_child();
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

    fn show_history_viewer(&mut self) {
        // Call the real implementation
        self.show_history_viewer_impl();
    }

    fn toggle_show_files(&mut self) {
        // Toggle the show_files flag
        self.popup_state.show_files = !self.popup_state.show_files;

        // Update app_state and save to disk
        self.popup_state.app_state.show_files = self.popup_state.show_files;
        if let Err(e) = crate::core::data::set_state(&self.popup_state.app_state) {
            log_error(&format!("Failed to save show_files state: {}", e));
        }

        log(&format!("Show files toggled to: {}", self.popup_state.show_files));

        // Force display update by recomputing display commands and layout
        self.popup_state.update_display_commands();
        self.popup_state.update_display_layout();
        self.popup_state.selection.reset(&self.popup_state.display_layout);
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
        self.dialog.is_visible()
    }
    
    fn close_command_editor(&mut self) {
        self.command_editor.visible = false;
        // Reset window size when closing editor
        self.window_size_mode = WindowSizeMode::Normal;
        // Request focus on input field
        self.request_focus = true;
    }

    fn close_dialog(&mut self) {
        // Dialog closes automatically via update() when user clicks button
        // This trait method exists for API compatibility but isn't used
        log("DIALOG: close_dialog() called but dialogs close automatically");
    }

    fn get_search_text(&self) -> &str {
        &self.popup_state.search_text
    }
    
    fn update_search(&mut self, text: String) {
        self.popup_state.update_search(text);
    }
    
    fn navigate_up_hierarchy(&mut self) {
        self.handle_navigate_up_hierarchy_impl();
    }
    
    fn navigate_down_hierarchy(&mut self) {
        self.handle_navigate_down_hierarchy_impl();
    }

    fn set_input(&mut self, text: String) {
        log(&format!("SET_INPUT_DEBUG: Called with text: '{}' (length: {})", text, text.len()));
        self.popup_state.update_search(text.clone());
        // Request focus and cursor positioning at the end
        self.request_focus = true;
        self.request_cursor_at_end = true;
        log(&format!("SET_INPUT_DEBUG: Set request_focus=true, request_cursor_at_end=true for text: '{}'", text));
    }
}

impl AnchorSelector {
    
    // OLD DIALOG SYSTEM - TO BE REMOVED
    
    // =============================================================================
    // Unified Action Execution System
    // =============================================================================

    /// Execute a command or action
    fn execute(&mut self, input: &crate::core::Command) {
        let search_text = self.popup_state.raw_search_text();
        detailed_log("POPUP_EXECUTE", &format!("🎯 POPUP execute() called with: '{}', search_text: '{}'", input.command, search_text));

        // Save the last executed command for add_alias functionality
        let mut state = crate::core::data::get_state();
        state.last_executed_command = Some(input.command.clone());
        let _ = crate::core::data::set_state(&state);

        // Save last anchor for commands that resolve to anchors
        let all_commands = self.commands();
        let resolved_command = input.resolve_alias(&all_commands);
        detailed_log("LAST_ANCHOR", &format!("Input: '{}' (action: {}), Resolved to: '{}' (action: {})",
            input.command, input.action, resolved_command.command, resolved_command.action));

        // If the resolved command is an anchor, save it as the last anchor
        if resolved_command.is_anchor() {
            detailed_log("LAST_ANCHOR", &format!("Saving last anchor: '{}'", resolved_command.command));
            let mut state = crate::core::data::get_state();
            state.anchor_name = Some(resolved_command.command.clone());
            state.anchor_timestamp = Some(chrono::Local::now().timestamp());
            let _ = crate::core::data::set_state(&state);
        }

        // Handle "file" action - convert to "doc" action for execution
        let actual_input = if input.action == "file" {
            let mut file_cmd = input.clone();
            file_cmd.action = "doc".to_string();
            detailed_log("FILE_EXEC", &format!("Converting file action to doc action for: {}", input.arg));
            file_cmd
        } else {
            input.clone()
        };

        // Convert command to action
        use crate::execute::command_to_action;
        let mut action = command_to_action(&actual_input);

        // Create template context and expand all action parameters
        let mut template_context = self.create_template_context();

        // Add command's arg to template context for expansion (fixes {{arg}} template expansion)
        if !actual_input.arg.is_empty() {
            template_context.add_variable("arg".to_string(), actual_input.arg.clone());
            detailed_log("POPUP_EXEC", &format!("Added command arg to template context: '{}'", actual_input.arg));
        }

        // Check if any grabber functionality should be executed
        if let Some(grab_value) = action.get("grab") {
            if let Some(grab_seconds) = grab_value.as_u64() {
                detailed_log("POPUP_EXEC", &format!("Action has grab parameter: {} seconds - performing grab", grab_seconds));
                // Perform grab and add variables to template_context
                if let Err(e) = template_context.perform_grab_and_add_variables(grab_seconds) {
                    detailed_log("POPUP_EXEC", &format!("Grab operation failed: {}", e));
                }
            }
        }

        // Expand all action parameters using template context
        template_context.expand_action_parameters(&mut action);

        detailed_log("POPUP_EXEC", &format!("Executing action type: {} (after expansion)", action.action_type()));

        // Execute the action - will route to server or local as appropriate
        match action.action_type() {
            // UI actions need special handling in popup
            "template" | "popup" => {
                detailed_log("POPUP_EXEC", "Executing UI action in popup");
                self.execute_in_popup(&action);
            }
            // All other actions go to server
            _ => {
                // Since template expansion was already done, we can pass minimal context
                // The arg from the command should be available for any remaining expansions
                let mut variables = std::collections::HashMap::new();
                if !actual_input.arg.is_empty() {
                    variables.insert("arg".to_string(), actual_input.arg.clone());
                }
                let _ = crate::execute::execute_on_server(&action, Some(variables));
                detailed_log("POPUP_EXEC", "Action sent to server");
            }
        }
    }

    
    /// Execute an action in the popup context (for UI-requiring actions)
    fn execute_in_popup(&mut self, action: &crate::execute::Action) {
        detailed_log("POPUP_LOCAL", &format!("Executing {} locally", action.action_type()));
        
        // For template actions, we need special UI handling
        if action.action_type() == "template" {
            // Get template name from action or use default
            let template_name = action.get_string("name").unwrap_or("default");
            detailed_log("POPUP_LOCAL", &format!("Processing template UI: {}", template_name));
            
            // Use existing template UI handling
            self.handle_template_create_named_impl(template_name);
        } else if action.action_type() == "popup" {
            // Handle popup control actions
            detailed_log("POPUP_LOCAL", "Handling popup control action");
            // TODO: Implement popup control (show/hide/etc)
        } else {
            // This shouldn't happen - non-UI actions should go to server
            log_error(&format!("Unexpected action type in execute_in_popup: {}", action.action_type()));
        }
    }
    
    /// Execute the currently selected command
    fn execute_selected_command_impl(&mut self) {


        if !self.filtered_commands().is_empty() {
            let selected_idx = self.selected_index();
            let display_commands = self.popup_state.display_commands.clone();

            if selected_idx < display_commands.len() {
                let selected_cmd = &display_commands[selected_idx];

                // Don't execute if it's a separator
                if !PopupState::is_separator_command(selected_cmd) {
                    // Use unified execution with state saving
                    self.execute(selected_cmd);
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
        let state = crate::core::data::get_state();
        detailed_log("ADD_ALIAS", &format!("=== ADD ALIAS TRIGGERED ==="));
        detailed_log("ADD_ALIAS", &format!("Last executed command from state: {:?}", state.last_executed_command));
        if let Some(last_command) = state.last_executed_command {
            let search_text = self.popup_state.search_text.clone();
            
            // Create a new command with alias action
            let alias_command = crate::core::Command {
                patch: String::new(),
                command: search_text.clone(),
                action: "alias".to_string(),
                arg: last_command,
                flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
            };
            
            // Open command editor with the pre-filled alias command
            self.command_editor.open_with_command(alias_command);
        } else {
            // If no last command available, show error
            crate::utils::error("No last executed command available for alias creation.");
        }
    }

    /// Execute a popup action by name (for --action flag)
    /// These are the direct popup actions like "open_editor", "exit", etc.
    /// Note: This requires an egui context, so it can't be called from here
    /// Instead, we'll set a flag that the update loop will process
    fn execute_popup_action(&mut self, popup_action: &str) {
        log(&format!("Executing popup action: {}", popup_action));
        match popup_action {
            "open_editor" => self.open_command_editor(),
            "edit_active_command" => self.edit_active_command(),
            "edit_input_command" => self.edit_input_command(),
            "add_alias" => self.handle_add_alias(),
            "execute_command" => self.execute_selected_command(),
            "exit" => self.should_exit = true,
            "show_folder" => self.show_folder(),
            "activate_tmux" => self.activate_tmux(),
            "show_history_viewer" => self.show_history_viewer(),
            "toggle_show_files" => self.toggle_show_files(),
            _ => log_error(&format!("Unknown popup action: {}", popup_action)),
        }
    }

    /// Execute a JavaScript action by function name (for --action flag)
    fn execute_js_action(&mut self, function_name: &str) {
        log_error(&format!(
            "DEPRECATED: execute_js_action called with '{}' - this should not happen! \
            JavaScript functions should be executed via the launcher system, not this legacy path.",
            function_name
        ));
    }

    fn edit_active_command_impl(&mut self) {
        if !self.filtered_commands().is_empty() {
            let display_commands = &self.popup_state.display_commands;

            if self.selected_index() < display_commands.len() {
                let selected_cmd = &display_commands[self.selected_index()];
                
                // Don't edit if it's a separator or a merged command
                if !PopupState::is_separator_command(selected_cmd) && selected_cmd.get_flag(FLAG_MERGED).is_none() {
                    // Edit the selected command, ignoring the search text
                    self.command_editor.edit_command(Some(selected_cmd), &selected_cmd.command);
                }
            }
        }
    }
    
    fn edit_input_command_impl(&mut self) {
        // Create a new blank command with the input text as the name
        let mut input_text = self.popup_state.search_text.clone();
        
        // If we're in submenu mode, transform input to use expanded alias
        if let Some((original_command, resolved_command)) = self.popup_state.get_prefix_menu_command_info() {
            // Replace the alias prefix with the expanded prefix
            let original_name = &original_command.command;
            let resolved_name = &resolved_command.command;
            
            if input_text.to_lowercase().starts_with(&original_name.to_lowercase()) {
                // Replace the prefix: "FB ZZZ" -> "fireball ZZZ"
                let remaining = &input_text[original_name.len()..];
                input_text = format!("{}{}", resolved_name, remaining);
            }
        }
        
        if !input_text.is_empty() {
            // Create a new command with the transformed input as the name
            let new_command = crate::core::Command {
                command: input_text.clone(),
                action: String::new(),  // Blank action for user to fill
                arg: String::new(),     // Blank arg for user to fill
                patch: String::new(),   // Blank patch
                flags: String::new(),   // No flags
                other_params: None,
                last_update: 0,
                file_size: None,
            };
            
            // Open the command editor with this new command
            self.command_editor.create_new_command(&new_command, &input_text);
        }
    }
    
    fn handle_link_to_clipboard_impl(&mut self) {
        if !self.filtered_commands().is_empty() {
            let display_commands = &self.popup_state.display_commands;

            if self.selected_index() < display_commands.len() {
                let selected_cmd = &display_commands[self.selected_index()];
                
                // Don't copy link if it's a separator
                if !PopupState::is_separator_command(selected_cmd) {
                    // Use launcher to execute the link_to_clipboard action
                    let link_action = crate::execute::make_action("link_to_clipboard", &selected_cmd.command);
                    // Execute the action
                    let mut variables = std::collections::HashMap::new();
                    variables.insert("arg".to_string(), selected_cmd.command.clone());
                    let _ = crate::execute::execute_on_server(&link_action, Some(variables));
                }
            }
        }
    }
    
    /// Handle uninstall app request
    fn handle_uninstall_app_impl(&mut self) {
        // Show confirmation dialog with OK/Cancel using external dialog system
        let spec_strings = vec![
            "=Uninstall HookAnchor".to_string(),
            "#This will remove HookAnchor app and Karabiner config.".to_string(),
            "'⚠️  Your commands and settings will be preserved.".to_string(),
            "!OK".to_string(),
            "!Cancel".to_string(),
        ];

        self.show_external_dialog(
            spec_strings,
            HashMap::new(),
            Box::new(|result| {
                if result.get("exit") == Some(&"OK".to_string()) {
                    // Execute uninstall using the shell script
                    std::thread::spawn(|| {
                        let config_dir = crate::core::get_config_dir();
                        let uninstall_script = config_dir.join("uninstall.sh");

                        if uninstall_script.exists() {
                            match std::process::Command::new("bash")
                                .arg(&uninstall_script)
                                .spawn() {
                                Ok(_) => {
                                    std::process::exit(0);
                                },
                                Err(_) => {
                                    // Silent failure - uninstall script will log errors
                                }
                            }
                        }
                    });
                }
                Ok(())
            }),
            self.cached_window_position
        );
    }
    
    /// Show all available key bindings in a dialog
    fn show_keys_dialog_impl(&mut self) {
        let config = crate::core::data::get_config();
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
        // OLD BLOCKING DIALOG (kept for reference):
        // self.dialog.show(key_lines);

        // NEW NON-BLOCKING DIALOG:
        let _ = crate::utils::dialog2(key_lines, None);
    }

    /// Launch the history viewer with the current anchor as the default filter
    fn show_history_viewer_impl(&mut self) {
        // Try to resolve current search text to an anchor command
        let anchor_filter = if !self.popup_state.search_text.is_empty() {
            // Find if search text matches an anchor command
            let commands = crate::core::data::get_commands();
            if let Some(cmd) = commands.iter().find(|c| c.command == self.popup_state.search_text && c.is_anchor()) {
                cmd.command.clone()
            } else {
                // Fallback to current anchor from app_state
                self.popup_state.app_state.anchor_name.clone()
                    .unwrap_or_else(|| String::new())
            }
        } else {
            // No search text, use current anchor from app_state
            self.popup_state.app_state.anchor_name.clone()
                .unwrap_or_else(|| String::new())
        };

        // Get path to history_viewer binary
        if let Ok(exe_path) = std::env::current_exe() {
            let resolved = std::fs::canonicalize(&exe_path).unwrap_or(exe_path);
            let exe_dir = resolved.parent().unwrap();
            let viewer_path = exe_dir.join("HookAnchorHistoryViewer");

            // Launch history viewer with anchor filter as argument
            match std::process::Command::new(&viewer_path)
                .arg(&anchor_filter)
                .spawn()
            {
                Ok(_) => {
                    log(&format!("Launched history viewer with filter: '{}'", anchor_filter));
                }
                Err(e) => {
                    crate::utils::error(&format!("Could not launch history viewer: {}", e));
                }
            }
        } else {
            crate::utils::error("Could not determine executable path");
        }
    }

    fn handle_template_create_impl(&mut self) {
        self.handle_template_create_named("default");
    }

    /// Create a TemplateContext from current popup state
    /// This consolidates all the context gathering logic in one place
    fn create_template_context(&self) -> crate::core::template_creation::TemplateContext {
        use crate::core::template_creation::TemplateContext;

        // Get the current input text
        let raw_input = self.popup_state.search_text.clone();
        let mut input = raw_input.clone();

        // If we're in submenu mode, transform input to use expanded alias
        if let Some((original_command, resolved_command)) = self.popup_state.get_prefix_menu_command_info() {
            // Replace the alias prefix with the expanded prefix
            let original_name = &original_command.command;
            let resolved_name = &resolved_command.command;

            if input.to_lowercase().starts_with(&original_name.to_lowercase()) {
                // Replace the prefix: "FB ZZZ" -> "fireball ZZZ"
                let remaining = &input[original_name.len()..];
                let transformed_input = format!("{}{}", resolved_name, remaining);
                detailed_log("TEMPLATE_CONTEXT", &format!("Transformed input from '{}' to '{}'", input, transformed_input));
                input = transformed_input;
            }
        }

        // Create basic template context with expanded input
        let mut context = TemplateContext::create_basic_template(&input);

        // Add raw_input variable with the original typed text
        context.variables.insert("raw_input".to_string(), raw_input);

        // Add popup-specific variables (selected command, previous command, etc.)
        let selected_command = if !self.filtered_commands().is_empty() {
            let display_commands = &self.popup_state.display_commands;
            if self.selected_index() < display_commands.len() {
                Some(display_commands[self.selected_index()].clone())
            } else {
                None
            }
        } else {
            None
        };

        context.add_popup_variables(selected_command.as_ref());

        context
    }

    fn handle_template_create_named_impl(&mut self, template_name: &str) {
        detailed_log("TEMPLATE", &format!("TEMPLATE: === ENTER handle_template_create_named_impl('{}') ===", template_name));
        log(&format!("TEMPLATE: Processing template '{}' with search text '{}'", template_name, self.popup_state.search_text));
        
        // Special validation for edit_selection template - only prevent editing invalid commands
        if template_name == "edit_selection" {
            // Only validate if we actually have commands to validate
            if !self.filtered_commands().is_empty() {
                let display_commands = &self.popup_state.display_commands;
                let selected_idx = self.selected_index();
                detailed_log("TEMPLATE", &format!("TEMPLATE: edit_selection - selected_index={}, display_commands.len()={}", selected_idx, display_commands.len()));
                
                if selected_idx < display_commands.len() {
                    let selected_cmd = &display_commands[selected_idx];
                    detailed_log("TEMPLATE", &format!("TEMPLATE: Selected command for editing: '{}'", selected_cmd.command));
                    
                    // Don't edit separator commands or merged commands
                    if PopupState::is_separator_command(selected_cmd) || selected_cmd.get_flag(FLAG_MERGED).is_some() {
                        detailed_log("TEMPLATE", "TEMPLATE: Cannot edit separator or merged command");
                        crate::utils::error("Cannot edit separator or merged commands.");
                        return;
                    }
                }
            }
        }
        
        // Create template context using helper function
        detailed_log("TEMPLATE", &format!("TEMPLATE: Creating context using helper function"));
        let context = self.create_template_context();
        
        detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 2 - TemplateContext created, about to get config"));
        // Get the specified template/action
        let config = crate::core::data::get_config();
        
        detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 3 - Config retrieved successfully"));
        // First try to find in unified actions
        detailed_log("TEMPLATE", &format!("TEMPLATE: Looking for action '{}'", template_name));
        let found_action = if let Some(ref actions) = config.actions {
            detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 4 - config.actions is Some, Found {} actions in config", actions.len()));
            // Debug: log all action names
            let action_names: Vec<String> = actions.keys().cloned().collect();
            detailed_log("TEMPLATE", &format!("TEMPLATE: Available actions: {:?}", action_names));
            if let Some(action) = actions.get(template_name) {
                detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 5 - Found action '{}', type='{}'", template_name, action.action_type()));
                if action.action_type() == "template" {
                    detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 6 - Action type is 'template'"));
                    // Extract grab parameter if present
                    detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 7 - About to extract grab parameter"));
                    let grab_seconds = action.params.get("grab")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32);
                    
                    detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 8 - grab_seconds = {:?}", grab_seconds));
                    if let Some(grab_secs) = grab_seconds {
                        detailed_log("TEMPLATE", &format!("TEMPLATE: DEBUG 9 - grab_secs = {}", grab_secs));
                        // Store template info for after grab completes
                        detailed_log("GRAB", &format!("GRAB: Setting pending_template to '{}' with context", template_name));
                        self.pending_template = Some((template_name.to_string(), context));
                        detailed_log("GRAB", &format!("GRAB: pending_template is now Some"));
                        
                        // Start countdown
                        self.grabber_countdown = Some(grab_secs as u8);
                        self.countdown_last_update = Some(std::time::Instant::now());
                        detailed_log("GRAB", &format!("GRAB: Started countdown for template '{}' with {} seconds", template_name, grab_secs));
                        return; // Early return for grab templates
                    }
                    
                    // For non-grab template actions, we need to convert to old Template format
                    // This is a temporary compatibility layer
                    true
                } else {
                    let error_msg = format!("ERROR: Action '{}' exists but is not a template (type='{}').\nTo use this key binding, rename your template action to avoid name collision.", 
                        template_name, action.action_type());
                    detailed_log("TEMPLATE", &format!("TEMPLATE: {}", error_msg));
                    crate::utils::error(&error_msg);
                    false
                }
            } else {
                detailed_log("TEMPLATE", &format!("TEMPLATE: Action '{}' not found in actions", template_name));
                // Don't show error here - it might be in old templates
                false
            }
        } else {
            false
        };
        
        if !found_action {
            let error_msg = format!("Template/Action '{}' not found in configuration.\n\nAvailable templates:", template_name);
            let mut full_error = error_msg.clone();
            if let Some(ref actions) = config.actions {
                let template_actions: Vec<String> = actions.iter()
                    .filter(|(_, action)| action.action_type() == "template")
                    .map(|(name, _)| name.clone())
                    .collect();
                if !template_actions.is_empty() {
                    full_error.push_str(&format!("\n- {}", template_actions.join("\n- ")));
                } else {
                    full_error.push_str("\n(No templates configured)");
                }
            }
            log_error(&full_error);
            crate::utils::error(&full_error);
        } else {
            // Process the unified action as a template
            detailed_log("SYSTEM", &format!("Processing unified action template: {}", template_name));

            // Convert unified action to template and process it
            if let Some(ref actions) = config.actions {
                if let Some(action) = actions.get(template_name) {
                    if let Some(template) = action.to_template() {
                        detailed_log("TEMPLATE", &format!("TEMPLATE: Converted action to template, edit={}", template.edit));

                        // Check for existing command if use_existing is true
                        // For edit_selection template, use the actual selected command (not a name search)
                        // to ensure we get the exact command the user selected (handles case-sensitive duplicates)
                        let existing_command = if template.use_existing {
                            if template_name == "edit_selection" {
                                // Use the selected command directly from context (already populated above)
                                if !self.filtered_commands().is_empty() {
                                    let display_commands = &self.popup_state.display_commands;
                                    if self.selected_index() < display_commands.len() {
                                        Some(display_commands[self.selected_index()].clone())
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                // For other templates, search by name (case-insensitive for backward compatibility)
                                // IMPORTANT: Get fresh commands from sys_data, not cached popup_state.commands
                                let expanded_name = context.expand(&template.name);
                                let fresh_commands = crate::core::data::get_commands();
                                fresh_commands.iter().find(|cmd|
                                    cmd.command.eq_ignore_ascii_case(&expanded_name)
                                ).cloned()
                            }
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
                                        detailed_log("TEMPLATE", &format!("TEMPLATE: Opening command editor for sub_anchor template"));
                                        // Open command editor with the prefilled command
                                        self.command_editor.create_new_command(&new_command, &self.popup_state.search_text);
                                        // Store template for post-save file creation
                                        self.command_editor.set_pending_template(template.clone(), context.clone());
                                        // Don't clear search text when opening editor - preserve input
                                } else {
                                    // Add the new command directly
                                    match crate::core::add_command(new_command) {
                                        Ok(_) => {
                                            // Command already saved by add_command (via sys_data::add_command)
                                            // Commands will be fetched fresh from singleton by update_search()

                                            // Clear search and update display
                                            self.popup_state.search_text.clear();
                                            self.popup_state.update_search(String::new());

                                            // Trigger rescan if requested
                                            if template.file_rescan {
                                                self.trigger_rescan();
                                            }
                                        }
                                        Err(e) => {
                                            crate::utils::error(&format!("Failed to add command: {}", e));
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                crate::utils::error(&format!("Failed to create command from '{}' template: {}", template_name, e));
                            }
                        }
                        }
                    } else {
                        // Failed to convert action to template format
                        crate::utils::error(&format!("Failed to convert action '{}' to template format. Check your template configuration.", template_name));
                    }
                }
            }
        }
    }
    
    fn handle_navigate_up_hierarchy_impl(&mut self) {
        detailed_log("HIERARCHY", "Navigating up hierarchy");
        detailed_log("HIERARCHY", &format!("Current search text: '{}'", self.popup_state.search_text));
        detailed_log("HIERARCHY", &format!("is_in_prefix_menu: {}", self.popup_state.is_in_prefix_menu));

        // Check if we're currently in a submenu
        if let Some((_original_command, resolved_command)) = self.popup_state.get_prefix_menu_command_info() {
            let (sys_data, _) = crate::core::data::get_sys_data();

            detailed_log("HIERARCHY", &format!("In prefix menu, resolved_command: '{}'", resolved_command.command));

            // Use get_parent_command() to walk up the hierarchy
            if let Some(parent_command) = resolved_command.get_parent_command(&sys_data.patches) {
                detailed_log("HIERARCHY", &format!("Navigating up from '{}' to parent: '{}'",
                    resolved_command.command, parent_command.command));
                self.popup_state.update_search(parent_command.command.clone());
            } else {
                detailed_log("HIERARCHY", &format!("Already at root level (command: '{}')", resolved_command.command));
            }
        } else {
            detailed_log("HIERARCHY", &format!("Not in submenu, cannot navigate up (is_in_prefix_menu={}, prefix_menu_info={})",
                self.popup_state.is_in_prefix_menu,
                if self.popup_state.prefix_menu_info.is_some() { "Some" } else { "None" }));
        }
    }
    
    fn handle_navigate_down_hierarchy_impl(&mut self) {
        detailed_log("HIERARCHY", "Navigating down hierarchy");

        // Get the currently selected command
        if let Some(selected_command) = self.popup_state.get_selected_command() {
            // Resolve aliases to get the actual command
            let commands = crate::core::data::get_commands_arc();
            let resolved_command = selected_command.resolve_alias(&commands);

            // Check if the resolved command is an anchor
            if resolved_command.is_anchor() {
                let anchor_name = &resolved_command.command;
                detailed_log("HIERARCHY", &format!("Navigating into anchor: {} (resolved from '{}')",
                    anchor_name, selected_command.command));

                // Set the search text to the anchor name with a trailing space and position cursor at end
                self.set_input(format!("{} ", anchor_name));
            } else {
                detailed_log("HIERARCHY", &format!("Selected command '{}' is not an anchor (action='{}', resolved='{}')",
                    selected_command.command, selected_command.action, resolved_command.command));
            }
        } else {
            detailed_log("HIERARCHY", "No command selected");
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
            if let Some(_home_dir) = dirs::home_dir() {
                let config_dir = crate::core::get_config_dir();
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
        let state = crate::core::data::get_state();

        // Create minimal popup state for immediate UI display
        let mut popup_state = PopupState::new_minimal();
        popup_state.app_state = state; // Use loaded state for window position
        
        // Set initial prompt if provided
        if !initial_prompt.is_empty() {
            popup_state.set_search_text_during_init(initial_prompt.to_string());
        }
        
        let result = Self {
            popup_state,
            last_saved_position: None,
            cached_window_position: None,
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
            request_cursor_at_end: false,
            frame_count: 0,
            window_activated: false,
            config_error: None,
            last_interaction_time: std::time::Instant::now(),
            show_command_start: None,
            loading_state: LoadingState::NotLoaded,
            pre_init_input_buffer: initial_prompt.to_string(),
            search_pending_after_load: !initial_prompt.is_empty(),
            pending_template: None,
            key_registry: None, // Will be initialized when config is loaded
            exit_app_key: None, // Will be populated from config
            should_exit: false,
            is_hidden: false,
            pending_rebuild: false,
            pending_action: None,
            last_state_mod_time: None,
            initial_action_name: None,
            external_dialog_state: None,
        };

        result
    }

    pub fn new_with_prompt_and_action(initial_prompt: &str, initial_action: Option<&str>) -> Self {
        let _startup_time = std::time::Instant::now();

        // Redirect stdout/stderr to anchor log for centralized debugging
        Self::setup_log_redirection();

        // Load only app state for window positioning - this is fast
        let state = crate::core::data::get_state();

        // Create minimal popup state for immediate UI display
        let mut popup_state = PopupState::new_minimal();
        popup_state.app_state = state; // Use loaded state for window position

        // Set initial prompt if provided
        if !initial_prompt.is_empty() {
            popup_state.set_search_text_during_init(initial_prompt.to_string());
        }

        let mut result = Self {
            popup_state,
            last_saved_position: None,
            cached_window_position: None,
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
            request_cursor_at_end: false,
            frame_count: 0,
            window_activated: false,
            config_error: None,
            last_interaction_time: std::time::Instant::now(),
            show_command_start: None,
            loading_state: LoadingState::NotLoaded,
            pre_init_input_buffer: initial_prompt.to_string(),
            search_pending_after_load: !initial_prompt.is_empty(),
            pending_template: None,
            key_registry: None, // Will be initialized when config is loaded
            exit_app_key: None, // Will be populated from config
            should_exit: false,
            is_hidden: false,
            pending_rebuild: false,
            pending_action: None,
            last_state_mod_time: None,
            initial_action_name: initial_action.map(|s| s.to_string()),
            external_dialog_state: None,
        };

        result
    }

    /// Start loading data in the background after UI is shown
    fn start_deferred_loading(&mut self) {
        if self.loading_state != LoadingState::NotLoaded {
            return; // Already loading or loaded
        }

        log("⏱️ DEFERRED_LOADING: Starting data load...");
        self.loading_state = LoadingState::Loading;
        let start_time = std::time::Instant::now();

        // Initialize sys_data NOW (deferred from main for faster window display)
        let init_start = std::time::Instant::now();
        match crate::core::initialize() {
            Ok(()) => {
                log(&format!("⏱️ DEFERRED_LOADING: Data loaded in {:?}", init_start.elapsed()));
            }
            Err(init_error) => {
                log_error(&format!("Failed to initialize sys_data: {}", init_error));
                // Continue with default config
            }
        }

        // Get data from singleton (now initialized)
        let config = crate::core::data::get_config();

        // Note: Config errors are handled during initialize() - we just use whatever is loaded
        let config_error: Option<String> = None;

        // Initialize key registry with the loaded config
        self.key_registry = Some(create_default_key_registry(&config));


        // Create new popup state with loaded data but preserve app_state
        // Commands will be loaded from singleton when needed (not cached in PopupState)
        let app_state = self.popup_state.app_state.clone();
        self.popup_state = PopupState::new(config, app_state);
        
        // Apply any buffered input - but avoid recompute during deferred loading
        if !self.pre_init_input_buffer.is_empty() {
            self.popup_state.search_text = self.pre_init_input_buffer.clone();
            // Don't call update_search() here - it triggers expensive recompute_filtered_commands()
            // Set flag to trigger search computation on next frame
            self.search_pending_after_load = true;
            self.pre_init_input_buffer.clear();
        }
        
        // Show config error if any
        if let Some(error) = config_error {
            self.config_error = Some(error.clone());
            crate::utils::error(&error);
        }
        
        self.loading_state = LoadingState::Loaded;
        log(&format!("⏱️ DEFERRED_LOADING: Total time: {:?}", start_time.elapsed()));
        
    }
    
    
    // =============================================================================
    // Helper Properties for Backward Compatibility
    // =============================================================================
    
    /// Backward compatibility: access to commands (fetches from singleton)
    fn commands(&self) -> Vec<Command> {
        crate::core::data::get_commands()
    }

    /// Backward compatibility: get commands for modification
    /// Returns a Vec that can be modified and then saved back via set_commands
    fn commands_mut_get(&mut self) -> Vec<Command> {
        crate::core::data::get_commands()
    }

    /// Backward compatibility: save modified commands back
    fn commands_mut_save(&mut self, commands: Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
        crate::core::data::set_commands(commands)
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
        log(&format!("ACTION: Queueing action with {} context keys", context.len()));
        self.pending_action = Some(PendingAction { context, callback });
    }

    /// Complete the pending action by merging user input and executing the callback
    pub fn complete_action(&mut self, user_input: HashMap<String, String>) {
        if let Some(action) = self.pending_action.take() {
            log(&format!("ACTION: Completing action with {} user inputs", user_input.len()));
            
            // Merge action context with user input
            let mut final_context = action.context;
            final_context.extend(user_input);
            
            // Check if this is a rename operation that needs special handling
            if final_context.get("operation") == Some(&"rename".to_string()) {
                let empty_string = String::new();
                let exit_button = final_context.get("exit").unwrap_or(&empty_string);
                if exit_button == "OK" {
                    // User confirmed full rename with all side effects - execute it with access to self
                    match self.execute_rename_with_ui_update(&final_context) {
                        Ok(()) => {
                            log("ACTION: Rename action executed successfully");
                        }
                        Err(e) => {
                            let error_msg = format!("Rename action failed: {}", e);
                            log_error(&error_msg);
                            crate::utils::error(&error_msg);
                        }
                    }
                } else if exit_button == "Only Change CMD" {
                    // User wants to rename command only, without file/folder/prefix changes
                    match self.execute_command_only_rename(&final_context) {
                        Ok(()) => {
                            log("ACTION: Command-only rename executed successfully");
                        }
                        Err(e) => {
                            let error_msg = format!("Command-only rename failed: {}", e);
                            log_error(&error_msg);
                            crate::utils::error(&error_msg);
                        }
                    }
                }
                // If Cancel or Escape, do nothing
            } else {
                // Execute the regular callback with merged context
                match (action.callback)(&final_context) {
                    Ok(()) => {
                        log("ACTION: Action executed successfully");
                    }
                    Err(e) => {
                        let error_msg = format!("Action failed: {}", e);
                        log_error(&error_msg);
                        crate::utils::error(&error_msg);
                    }
                }
            }
        }
    }

    /// Cancel the pending action without executing it
    pub fn cancel_action(&mut self) {
        if self.pending_action.is_some() {
            log("ACTION: Cancelling pending action");
            self.pending_action = None;
        }
    }

    /// Check if there is a pending action waiting for user input
    pub fn has_pending_action(&self) -> bool {
        self.pending_action.is_some()
    }
    /// Save a command atomically - handles both new commands and edits
    /// If old_command_name is Some, this is an edit/rename - delete the old one first
    /// This ensures patch inference only runs once after both delete+add are done
    fn save_command_atomic(&mut self, new_command: Command, old_command_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        // Get current commands from singleton
        let mut commands = crate::core::data::get_commands();

        // Delete original command if this is an edit/rename
        if let Some(old_name) = old_command_name {
            if !old_name.is_empty() {
                log(&format!("SAVE: Deleting original command '{}' from UI", old_name));
                commands.retain(|c| c.command != old_name);
            }
        }

        // Add the new command
        log(&format!("SAVE: Adding command '{}' to UI (action: {}, patch: {})",
            new_command.command, new_command.action, new_command.patch));
        commands.push(new_command);

        // Save all commands to disk - patch inference runs once here
        crate::core::data::set_commands(commands)?;
        log("SAVE: Saved all commands to disk");

        Ok(())
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
        let _new_command_arg = context.get("new_command_arg").ok_or("Missing new_command_arg")?;
        let new_command_patch = context.get("new_command_patch").ok_or("Missing new_command_patch")?;
        let new_command_flags = context.get("new_command_flags").ok_or("Missing new_command_flags")?;

        let config = crate::core::data::get_config();

        // Get current commands for modification
        let mut commands = crate::core::data::get_commands();

        // Execute the rename with dry_run = false
        use crate::core::rename_associated_data;
        let (sys_data, _) = crate::core::get_sys_data();
        let mut patches = sys_data.patches;

        log(&format!("RENAME: About to execute rename_associated_data: '{}' -> '{}'", old_name, new_name));
        let (mut updated_arg, _actions) = rename_associated_data(
            old_name,
            new_name,
            current_arg,
            action,
            &mut commands,
            &mut patches,
            &config,
            false, // dry_run = false
        )?;
        log(&format!("RENAME: rename_associated_data completed, updated_arg: {}", updated_arg));

        // Check if this is an anchor file rename where filename matches folder name
        // Check if the command has the anchor flag ('A')
        if new_command_flags.contains('A') {
            use std::path::Path;
            let arg_path = Path::new(current_arg);

            // Check if the file stem (without extension) matches the old command name
            if let Some(file_stem) = arg_path.file_stem() {
                if let Some(file_stem_str) = file_stem.to_str() {
                    if file_stem_str == old_name {
                        // This is an anchor file - check if folder name also matches
                        if let Some(parent) = arg_path.parent() {
                            if let Some(folder_name) = parent.file_name() {
                                if let Some(folder_name_str) = folder_name.to_str() {
                                    if folder_name_str == old_name {
                                        // Folder name matches - rename the folder too
                                        use crate::core::command_ops::rename_folder;
                                        log(&format!("RENAME_FOLDER: Executing folder rename: '{}' -> '{}'", folder_name_str, new_name));
                                        match rename_folder(
                                            parent.to_str().unwrap_or(""),
                                            new_name,
                                            &mut commands,
                                            false, // dry_run = false
                                        ) {
                                            Ok(_) => {
                                                log("RENAME_FOLDER: Folder rename completed successfully");

                                                // Update the arg path to reflect the new folder name
                                                let new_folder_path = parent.parent()
                                                    .map(|p| p.join(new_name))
                                                    .unwrap_or_else(|| std::path::PathBuf::from(new_name));
                                                let new_file_name = format!("{}.md", new_name);
                                                let new_full_path = new_folder_path.join(&new_file_name);
                                                updated_arg = new_full_path.to_string_lossy().to_string();
                                                log(&format!("RENAME_FOLDER: Updated arg to: {}", updated_arg));
                                            }
                                            Err(e) => {
                                                return Err(format!("Failed to rename folder: {}", e).into());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Delete original command if needed
        if let Some(cmd_name) = original_command_to_delete {
            if !cmd_name.is_empty() {
                log(&format!("RENAME: Deleting original command '{}'", cmd_name));
                commands.retain(|c| &c.command != cmd_name);
            }
        }

        // Create the new command with potentially updated arg
        let new_command = crate::core::Command {
            command: new_name.clone(),
            action: new_command_action.clone(),
            arg: updated_arg, // Use the updated arg from rename operation
            patch: new_command_patch.clone(),
            flags: new_command_flags.clone(),
        other_params: None,
        last_update: 0,
        file_size: None,
        };

        // Add the new command
        log(&format!("RENAME: Adding new command '{}'", new_command.command));
        commands.push(new_command);

        // Save all commands back to sys_data because rename_associated_data modified
        // patches and prefixes on many commands (not just the renamed command)
        crate::core::data::set_commands(commands)?;
        log("RENAME: Saved all command changes (patches/prefixes) to sys_data");

        // Update the filtered list if we're currently filtering
        if !self.popup_state.search_text.trim().is_empty() {
            // Refresh the search with updated commands
            let current_search = self.popup_state.search_text.clone();
            self.popup_state.update_search(current_search);
        }

        // Mark commands as modified to trigger automatic reload

        // Update input field with the renamed command name for symmetric feedback
        // This ensures that after the rename dialog, the input field shows the command that was just renamed
        log(&format!("RENAME_FEEDBACK: Setting input field to renamed command: '{}'", new_name));
        self.set_input(new_name.to_string());

        log(&format!("RENAME: Successfully renamed '{}' to '{}' with side effects and UI update", old_name, new_name));
        Ok(())
    }

    /// Execute a command-only rename - changes ONLY the command name, no file/folder/prefix changes
    fn execute_command_only_rename(&mut self, context: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let old_name = context.get("old_name").ok_or("Missing old_name")?;
        let new_name = context.get("new_name").ok_or("Missing new_name")?;
        let original_command_to_delete = context.get("original_command_to_delete");

        // Get the new command details from context
        let new_command_action = context.get("new_command_action").ok_or("Missing new_command_action")?;
        let new_command_arg = context.get("new_command_arg").ok_or("Missing new_command_arg")?;
        let new_command_patch = context.get("new_command_patch").ok_or("Missing new_command_patch")?;
        let new_command_flags = context.get("new_command_flags").ok_or("Missing new_command_flags")?;

        log(&format!("CMD_ONLY_RENAME: Renaming command '{}' -> '{}' without side effects", old_name, new_name));

        // Get current commands for modification
        let mut commands = crate::core::data::get_commands();

        // Delete original command if needed
        if let Some(cmd_name) = original_command_to_delete {
            if !cmd_name.is_empty() {
                log(&format!("CMD_ONLY_RENAME: Deleting original command '{}'", cmd_name));
                commands.retain(|c| &c.command != cmd_name);
            }
        }

        // Create the new command with ORIGINAL arg (no rename_associated_data modifications)
        let new_command = crate::core::Command {
            command: new_name.clone(),
            action: new_command_action.clone(),
            arg: new_command_arg.clone(), // Use original arg, not renamed
            patch: new_command_patch.clone(),
            flags: new_command_flags.clone(),
            other_params: None,
            last_update: 0,
            file_size: None,
        };

        // Add the new command
        log(&format!("CMD_ONLY_RENAME: Adding new command '{}'", new_command.command));
        commands.push(new_command);

        // Save commands
        crate::core::data::set_commands(commands)?;
        log("CMD_ONLY_RENAME: Saved command changes to sys_data");

        // Update the filtered list if we're currently filtering
        if !self.popup_state.search_text.trim().is_empty() {
            // Refresh the search with updated commands
            let current_search = self.popup_state.search_text.clone();
            self.popup_state.update_search(current_search);
        }

        // Update input field with the renamed command name
        log(&format!("CMD_ONLY_RENAME: Setting input field to renamed command: '{}'", new_name));
        self.set_input(new_name.to_string());

        log(&format!("CMD_ONLY_RENAME: Successfully renamed command '{}' to '{}' (command only, no side effects)", old_name, new_name));
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
        // Just set it directly - no validation needed since we're already checking against display_commands
        // in the rendering loop
        self.popup_state.selection.command_index = index;

        // Update visual position based on layout (column-first layout)
        let (rows, _cols) = self.popup_state.get_layout_dimensions();
        if rows > 0 {
            let col = index / rows;  // Column-first: which column
            let row = index % rows;  // Column-first: position within column
            self.popup_state.selection.visual_position = (row, col);

            // Log with limited backtrace to see WHO called this
            let backtrace = std::backtrace::Backtrace::capture();
            let bt_string = format!("{}", backtrace);
            let caller = bt_string.lines()
                .skip_while(|line| line.contains("set_selected_index") || line.contains("backtrace"))
                .take(3)
                .collect::<Vec<_>>()
                .join(" | ");

            log(&format!("SET_SELECTED_INDEX: index={} -> position=({}, {}) | CALLER: {}",
                index, row, col, caller));
        }
    }

    // =============================================================================
    // External Dialog System (subprocess-based)
    // =============================================================================

    /// Spawn an external dialog subprocess with callback
    pub fn show_external_dialog(
        &mut self,
        spec_strings: Vec<String>,
        context: HashMap<String, String>,
        callback: Box<dyn FnOnce(&HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>>>,
        window_position: Option<(f32, f32, f32)>, // (x, y, width)
    ) {
        // Spawn dialog using dialog2
        match crate::utils::dialog2(spec_strings, window_position) {
            Some(handle) => {
                self.external_dialog_state = Some(ExternalDialogState {
                    handle,
                    context,
                    callback,
                });
            }
            None => {
                crate::utils::error("Failed to show dialog");
            }
        }
    }

    /// Poll external dialog subprocess (call every frame)
    fn poll_external_dialog(&mut self) {
        if let Some(mut state) = self.external_dialog_state.take() {
            // Poll dialog using dialog_manager
            match crate::systems::poll_dialog(&mut state.handle) {
                Some(Ok(mut result)) => {
                    // Dialog finished successfully
                    log(&format!("EXTERNAL_DIALOG: Got result: {:?}", result));

                    // Merge context with result
                    result.extend(state.context);

                    // Execute callback
                    match (state.callback)(&result) {
                        Ok(()) => {
                            log("EXTERNAL_DIALOG: Callback executed successfully");

                            // Check if this is a rename operation that needs special handling
                            if result.get("operation") == Some(&"rename".to_string()) {
                                let empty_string = String::new();
                                let exit_button = result.get("exit").unwrap_or(&empty_string);
                                if exit_button == "OK" {
                                    // User confirmed full rename with all side effects - execute it with access to self
                                    match self.execute_rename_with_ui_update(&result) {
                                        Ok(()) => {
                                            log("EXTERNAL_DIALOG: Rename action executed successfully");
                                        }
                                        Err(e) => {
                                            let error_msg = format!("Rename action failed: {}", e);
                                            log_error(&error_msg);
                                            crate::utils::error(&error_msg);
                                        }
                                    }
                                } else if exit_button == "Only Change CMD" {
                                    // User wants to rename command only, without file/folder/prefix changes
                                    match self.execute_command_only_rename(&result) {
                                        Ok(()) => {
                                            log("EXTERNAL_DIALOG: Command-only rename executed successfully");
                                        }
                                        Err(e) => {
                                            let error_msg = format!("Command-only rename failed: {}", e);
                                            log_error(&error_msg);
                                            crate::utils::error(&error_msg);
                                        }
                                    }
                                }
                                // If Cancel or Escape, do nothing

                                // Close command editor now that dialog is complete
                                self.close_command_editor();
                            }
                        }
                        Err(e) => {
                            let error_msg = format!("External dialog callback failed: {}", e);
                            log_error(&error_msg);
                            crate::utils::error(&error_msg);
                            // Close command editor even on error
                            self.close_command_editor();
                        }
                    }

                    // Dialog finished - state is dropped
                }
                Some(Err(error)) => {
                    // Dialog failed
                    log_error(&format!("EXTERNAL_DIALOG: {}", error));
                    crate::utils::error(&format!("Dialog error: {}", error));
                    // Drop state on error
                }
                None => {
                    // Still running - put state back
                    self.external_dialog_state = Some(state);
                }
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

    /// Try to expand the current input to a full command name
    /// If the input matches a command (especially an alias), expand it to the full/resolved name
    /// Returns true if expansion occurred, false otherwise
    fn try_expand_input(&mut self) -> bool {
        let current_input = self.popup_state.search_text.trim().to_string();

        if current_input.is_empty() {
            return false;
        }

        let all_commands = self.commands();

        // Look for exact match first
        if let Some(matching_cmd) = all_commands.iter().find(|cmd|
            cmd.command.eq_ignore_ascii_case(&current_input)
        ) {
            // If it's an alias, resolve it
            let resolved = matching_cmd.resolve_alias(&all_commands);
            let expanded_name = &resolved.command;

            // Only expand if the name actually changed
            if expanded_name != &current_input {
                log(&format!("🔄 Expanding '{}' -> '{}'", current_input, expanded_name));

                // Update the search text with the expanded/resolved name
                self.popup_state.update_search(expanded_name.clone());
                self.request_cursor_at_end = true;

                return true;
            } else {
                // Name didn't change - no expansion needed
                return false;
            }
        }

        // If no exact match, try prefix match with the first filtered result
        let filtered = self.filtered_commands();
        if !filtered.is_empty() {
            let first_match = &filtered[0];
            // Only expand if the input is a clear prefix of the first match
            if first_match.command.to_lowercase().starts_with(&current_input.to_lowercase())
                && first_match.command.len() > current_input.len() {

                let resolved = first_match.resolve_alias(&all_commands);
                let expanded_name = &resolved.command;

                log(&format!("🔄 Expanding prefix '{}' -> '{}'", current_input, expanded_name));

                self.popup_state.update_search(expanded_name.clone());
                self.request_cursor_at_end = true;

                return true;
            }
        }

        false
    }


    // =============================================================================
    // Layout and Display Logic
    // =============================================================================
    
    // =============================================================================
    // Navigation Logic
    // =============================================================================
    
    // Navigate left/right in the multi-column layout
    fn navigate_horizontal(&mut self, direction: i32) {
        // Use the display_layout that was already computed in PopupState
        self.popup_state.navigate_horizontal_with_layout(direction, &self.popup_state.display_layout.clone());
    }

    fn navigate_vertical(&mut self, direction: i32) {
        // Use the display_layout that was already computed in PopupState
        self.popup_state.navigate_vertical_with_layout(direction, &self.popup_state.display_layout.clone());
    }
    
    /// Start the grabber countdown
    fn start_grabber_countdown(&mut self, _ctx: &egui::Context) {
        let config = crate::core::data::get_config();
        let countdown_seconds = config.popup_settings.countdown_seconds.unwrap_or(5);
        let flip_focus = config.launcher_settings.as_ref().and_then(|ls| ls.flip_focus).unwrap_or(false);
        
        detailed_log("GRAB", &format!("Starting grabber countdown from {} (flip_focus={})", 
            countdown_seconds, flip_focus));
            
        self.grabber_countdown = Some(countdown_seconds);
        self.countdown_last_update = Some(std::time::Instant::now());
        // Note: Don't call ctx.request_repaint_after() as it causes UI lockup
        // The UI update loop handles repaints automatically
    }
    
    /// Update countdown and handle grabber logic
    fn update_grabber_countdown(&mut self, ctx: &egui::Context) {
        if let Some(count) = self.grabber_countdown {
            detailed_log("GRAB", &format!("update_grabber_countdown: count={}", count));
            if let Some(last_update) = self.countdown_last_update {
                let elapsed = last_update.elapsed();
                detailed_log("GRAB", &format!("Elapsed: {}ms", elapsed.as_millis()));
                if elapsed.as_secs() >= 1 {
                    if count > 1 {
                        // Continue countdown
                        let new_count = count - 1;
                        self.grabber_countdown = Some(new_count);
                        self.countdown_last_update = Some(std::time::Instant::now());
                        
                        // Handle focus flipping during countdown if enabled
                        let config = crate::core::data::get_config();
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
                        log("GRAB: Countdown reached 0, calling execute_grab");
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
            log_error(&format!("Failed to flip focus away: {}", e));
        }
    }
    
    /// Execute the grab operation - simplified synchronous version
    fn execute_grab(&mut self, _ctx: &egui::Context) {
        log("GRAB: execute_grab() called");
        let config = crate::core::data::get_config();
        
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
        detailed_log("GRAB", &format!("GRAB: Calling crate::systems::grabber::grab, pending_template={:?}", 
            self.pending_template.as_ref().map(|(name, _)| name)));
        // Execute grab on command server instead of locally
        let mut grab_action_params = std::collections::HashMap::new();
        grab_action_params.insert("action_type".to_string(), serde_json::Value::String("grab".to_string()));
        grab_action_params.insert("flags".to_string(), serde_json::Value::String("G".to_string()));
        let grab_action = crate::execute::Action { params: grab_action_params };

        match crate::execute::execute_on_server(&grab_action, None) {
            Ok(grab_output) => {
                log(&format!("GRAB: Command server returned: '{}'", grab_output));

                // Check if server returned NoRuleMatched error
                if grab_output.starts_with("No grabber rule matched for:") {
                    // Parse app info from error message
                    let lines: Vec<&str> = grab_output.lines().collect();
                    let app_name = lines.iter()
                        .find(|l| l.contains("App:"))
                        .and_then(|l| l.split("App:").nth(1))
                        .map(|s| s.trim().to_string())
                        .unwrap_or_else(|| "Unknown App".to_string());
                    let bundle_id = lines.iter()
                        .find(|l| l.contains("Bundle ID:"))
                        .and_then(|l| l.split("Bundle ID:").nth(1))
                        .map(|s| s.trim().to_string())
                        .unwrap_or_else(|| "unknown.bundle".to_string());

                    log(&format!("🔴 GRAB NoRuleMatched for app: {}", app_name));

                    // Clear any pending template
                    self.pending_template.take();

                    // Generate the template text
                    let template_text = format!("# Grabber Rule Template\n\n- name: {}\n  matcher: 'bundleId === \"{}\"'\n  action: doc\n  group: Apps", app_name, bundle_id);

                    // Show template as info
                    crate::utils::info(&format!("Grabber Rule Template for {}:\n\n{}", app_name, template_text));
                    return;
                }

                // Parse grab output and create grab_result
                let parts: Vec<&str> = grab_output.trim().split_whitespace().collect();
                let grab_result = if parts.len() >= 2 {
                    let action = parts[0];

                    // Check if the output includes rule name and flags (format: "action arg RULE:rule_name FLAGS:suffix")
                    let mut rule_name = "Server Grab".to_string();
                    let mut flags = String::new();
                    let mut url_parts = Vec::new();
                    let mut end_index = parts.len();

                    // Find RULE: and FLAGS: markers
                    for (i, part) in parts[1..].iter().enumerate() {
                        if part.starts_with("RULE:") {
                            rule_name = part[5..].to_string(); // Remove "RULE:" prefix
                            end_index = 1 + i; // Everything before this is URL
                        } else if part.starts_with("FLAGS:") {
                            flags = part[6..].to_string(); // Remove "FLAGS:" prefix
                            end_index = 1 + i; // Everything before this is URL
                        }
                    }

                    // Take everything before the markers as the URL
                    url_parts = parts[1..end_index].to_vec();
                    let url = url_parts.join(" ");

                    crate::systems::grabber::GrabResult::RuleMatched(
                        rule_name.clone(),
                        crate::core::Command {
                            patch: String::new(),
                            command: format!("{} {}", action, url),
                            action: action.to_string(),
                            arg: url,
                            flags: flags, // Use the detected suffix from the grabber
                            other_params: None,
                            last_update: 0,
                            file_size: None,
                        }
                    )
                } else {
                    crate::systems::grabber::GrabResult::NoRuleMatched(
                        crate::systems::grabber::AppContext {
                            app_name: "Unknown App".to_string(),
                            bundle_id: "unknown.bundle".to_string(),
                            app_path: "/Unknown/Path".to_string(),
                            window_title: "Unknown Window".to_string(),
                            properties: serde_json::json!({}),
                        }
                    )
                };

                match grab_result {
                    crate::systems::grabber::GrabResult::RuleMatched(rule_name, mut command) => {
                        log(&format!("🟢 GRAB: RuleMatched - rule='{}', command.action='{}', command.arg='{}'", rule_name, command.action, command.arg));
                        
                        // Check if we have a pending template
                        if let Some((template_name, mut context)) = self.pending_template.take() {
                            detailed_log("GRAB", &format!("GRAB: Found pending template '{}' - will process it", template_name));
                            
                            // Add grabbed variables to template context
                            context.add_variable("grabbed_action".to_string(), command.action.clone());
                            context.add_variable("grabbed_arg".to_string(), command.arg.clone());
                            context.add_variable("grabbed_rule".to_string(), rule_name.clone());

                            // Add suffix from command.flags with leading space if not empty
                            let grabbed_suffix = if command.flags.is_empty() {
                                String::new()
                            } else {
                                format!(" {}", command.flags)
                            };
                            context.add_variable("grabbed_suffix".to_string(), grabbed_suffix);
                            
                            // Process the template with grabbed context
                            let config = crate::core::data::get_config();
                            
                            // Try to find template in unified actions first
                            let _template_found = if let Some(ref actions) = config.actions {
                                if let Some(action) = actions.get(&template_name) {
                                    detailed_log("GRAB", &format!("GRAB: Found action '{}' in unified actions", template_name));
                                    if action.action_type() == "template" {
                                        detailed_log("GRAB", &format!("GRAB: Action type is 'template'"));
                                        // Convert action to Template for processing
                                        if let Some(template) = action.to_template() {
                                            detailed_log("GRAB", &format!("GRAB: Successfully converted action to template"));
                                            detailed_log("GRAB", &format!("GRAB: Calling process_template"));
                                            match crate::core::template_creation::process_template(&template, &context, &config) {
                                                Ok(new_command) => {
                                                    detailed_log("GRAB", &format!("GRAB: process_template succeeded, command='{}'\n", new_command.command));
                                                    // Check edit flag from action params
                                                    let should_edit = action.params.get("edit")
                                                        .and_then(|v| v.as_bool())
                                                        .unwrap_or(false);
                                                    
                                                    detailed_log("GRAB", &format!("GRAB: Template created command '{}', should_edit={}", 
                                                        new_command.command, should_edit));
                                                    
                                                    if should_edit {
                                                        detailed_log("GRAB", &format!("GRAB: Opening command editor for template (should_edit=true)"));
                                                        log(&format!("GRAB: command_editor.visible before={}", self.command_editor.visible));
                                                        self.command_editor.create_new_command(&new_command, &self.popup_state.search_text);
                                                        log(&format!("GRAB: command_editor.visible after={}", self.command_editor.visible));
                                                    } else {
                                                        detailed_log("GRAB", &format!("GRAB: Not opening editor (should_edit=false)"));
                                                        // Add command directly
                                                        match crate::core::add_command(new_command) {
                                                            Ok(_) => {
                                                                // Command already saved by add_command (via sys_data::add_command)
                                                                // Commands will be fetched fresh from singleton by update_search()

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
                                                                crate::utils::error(&format!("Failed to add command: {}", e));
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    detailed_log("GRAB", &format!("GRAB: process_template failed: {}", e));
                                                    crate::utils::error(&format!("Failed to create command from template: {}", e));
                                                }
                                            }
                                            true
                                        } else {
                                            detailed_log("GRAB", &format!("GRAB: Failed to convert action '{}' to template", template_name));
                                            crate::utils::error(&format!("Failed to convert grab action '{}' to template format. Check your template configuration.", template_name));
                                            false
                                        }
                                    } else {
                                        detailed_log("GRAB", &format!("GRAB: Action '{}' is not a template type (type={})", template_name, action.action_type()));
                                        false
                                    }
                                } else {
                                    detailed_log("GRAB", &format!("GRAB: Action '{}' not found in unified actions", template_name));
                                    false
                                }
                            } else {
                                log("GRAB: No unified actions configured");
                                false
                            };
                            
                        } else {
                            detailed_log("GRAB", &format!("GRAB: No pending template - using normal grab behavior"));
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
                        log(&format!("🔴 GRAB NoRuleMatched for app: {}", grab_context.app_name));
                        // TEMPORARILY DISABLED: pending template fallback creates "app" action
                        // We want to test the template info dialog instead
                        /*
                        // Check if we have a pending template
                        if let Some((template_name, mut context)) = self.pending_template.take() {

                            // Add generic grabbed variables when no rule matched
                            context.add_variable("grabbed_action".to_string(), "app".to_string());
                            context.add_variable("grabbed_arg".to_string(), grab_context.app_name.clone());
                            context.add_variable("grabbed_rule".to_string(), "NoRuleMatched".to_string());
                            context.add_variable("grabbed_suffix".to_string(), String::new());

                            // Process the template
                            let config = crate::core::data::get_config();

                            // Try to find template in unified actions first
                            let _template_found = if let Some(ref actions) = config.actions {
                                if let Some(action) = actions.get(&template_name) {
                                    if action.action_type() == "template" {
                                        detailed_log("GRAB", &format!("GRAB: Action type is 'template'"));
                                        // Convert action to Template for processing
                                        if let Some(template) = action.to_template() {
                                            detailed_log("GRAB", &format!("GRAB: Successfully converted action to template"));
                                            detailed_log("GRAB", &format!("GRAB: Calling process_template"));
                                            match crate::core::template_creation::process_template(&template, &context, &config) {
                                                Ok(new_command) => {
                                                    detailed_log("GRAB", &format!("GRAB: process_template succeeded, command='{}'\n", new_command.command));
                                                    // Check edit flag from action params
                                                    let should_edit = action.params.get("edit")
                                                        .and_then(|v| v.as_bool())
                                                        .unwrap_or(false);

                                                    detailed_log("GRAB", &format!("GRAB: Template created command '{}', should_edit={}",
                                                        new_command.command, should_edit));

                                                    if should_edit {
                                                        detailed_log("GRAB", &format!("GRAB: Opening command editor for template (should_edit=true)"));
                                                        log(&format!("GRAB: command_editor.visible before={}", self.command_editor.visible));
                                                        self.command_editor.create_new_command(&new_command, &self.popup_state.search_text);
                                                        log(&format!("GRAB: command_editor.visible after={}", self.command_editor.visible));
                                                    } else {
                                                        detailed_log("GRAB", &format!("GRAB: Not opening editor (should_edit=false)"));
                                                        // Add command directly
                                                        match crate::core::add_command(new_command) {
                                                            Ok(_) => {
                                                                // Command already saved by add_command (via sys_data::add_command)
                                                                // Commands will be fetched fresh from singleton by update_search()

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
                                                                crate::utils::error(&format!("Failed to add command: {}", e));
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    detailed_log("GRAB", &format!("GRAB: process_template failed: {}", e));
                                                    crate::utils::error(&format!("Failed to create command from template: {}", e));
                                                }
                                            }
                                            true
                                        } else {
                                            detailed_log("GRAB", &format!("GRAB: Failed to convert action '{}' to template", template_name));
                                            crate::utils::error(&format!("Failed to convert grab action '{}' to template format. Check your template configuration.", template_name));
                                            false
                                        }
                                    } else {
                                        detailed_log("GRAB", &format!("GRAB: Action '{}' is not a template type (type={})", template_name, action.action_type()));
                                        false
                                    }
                                } else {
                                    detailed_log("GRAB", &format!("GRAB: Action '{}' not found in unified actions", template_name));
                                    false
                                }
                            } else {
                                log("GRAB: No unified actions configured");
                                false
                            };

                        } else {
                        */
                        // Clear any pending template (commented out code above would have taken it)
                        self.pending_template.take();

                        // Normal grab behavior (no template)
                        // Generate the template text
                        let template_text = crate::systems::grabber::generate_rule_template_text(&grab_context);

                        // Show template as info
                        crate::utils::info(&format!("Grabber Rule Template for {}:\n\n{}", grab_context.app_name, template_text));
                        // }  // Commented out closing brace for the if-let
                    }
                }
            }
            Err(err) => {
                log_error(&format!("GRAB: Failed to execute grab on command server: {}", err));
                crate::utils::queue_user_error(&format!("Grab operation failed: {}", err));
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
                log_error(&format!("Failed to restore HookAnchor: {}", e));
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
                log_error(&format!("Failed to regain focus: {}", e));
            }
        } else {
        }
    }
    
    
    /// Regain focus back to anchor selector after grab operation
    fn regain_focus(&self) -> Result<(), String> {
        // Call the regain_focus function from config.js
        let js_code = "regain_focus({})";
        match crate::js::execute_with_context(js_code, "REGAIN_FOCUS") {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to execute regain_focus function: {}", e))
        }
    }
    
    /// Clean up the debug log file before starting a rescan
    
    /// Trigger rebuild: restart server and rescan filesystem (full reset)
    fn trigger_rebuild(&mut self, ctx: &egui::Context) {
        // Set the search text to show rebuilding status using standardized accessor
        self.set_input("Rebuilding...".to_string());
        
        // Force immediate repaint to show the "Rebuilding..." text
        ctx.request_repaint();
        
        // Schedule the actual rebuild work for the next frame
        // This allows the UI to update with "Rebuilding..." before we block
        self.pending_rebuild = true;
        return;
    }
    
    /// Perform the actual rebuild work (called after UI update)
    fn perform_rebuild(&mut self) {
        // Use centralized restart function
        match crate::systems::full_system_restart() {
            Ok(()) => {
                log("
🎉 Rebuild completed successfully!");
                // Exit this instance - the new popup has been started
                std::process::exit(0);
            }
            Err(e) => {
                log_error(&format!("Rebuild failed: {}", e));
                log("
⚠️  Rebuild encountered errors - see log for details");
            }
        }
    }
    
    
    /// Trigger filesystem rescan
    fn trigger_rescan(&mut self) {
        log("Triggering filesystem rescan...");
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
                    detailed_log("RESCAN", "Filesystem rescan completed successfully");
                } else {
                    log_error("Filesystem rescan failed");
                }
            }
            Err(e) => {
                log_error(&format!("Failed to start rescan: {}", e));
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
                detailed_log("SHOW_FOLDER", &format!("Circular alias detected for '{}', stopping resolution", current_cmd.command));
                break;
            }
            visited.insert(current_cmd.command.clone());
            
            detailed_log("SHOW_FOLDER", &format!("Resolving alias '{}' to target '{}'", current_cmd.command, current_cmd.arg));
            
            // Find the target command
            if let Some(target) = all_commands.iter().find(|c| c.command == current_cmd.arg) {
                detailed_log("SHOW_FOLDER", &format!("Alias resolved to: '{}' (action: {}, arg: {})", 
                    target.command, target.action, target.arg));
                current_cmd = target;
            } else {
                detailed_log("SHOW_FOLDER", &format!("Failed to resolve alias '{}' - target '{}' not found", current_cmd.command, current_cmd.arg));
                break;
            }
        }
        
        current_cmd
    }

    /// Show folder functionality - launches the first folder matching current search
    fn show_folder_impl(&mut self) {
        use crate::utils;
        
        let search_text = &self.popup_state.search_text;
        detailed_log("SHOW_FOLDER", &format!("Triggered with search text: '{}'", search_text));
        
        // Get the current filtered commands from popup state
        let display_commands = self.popup_state.filtered_commands.clone();
        detailed_log("SHOW_FOLDER", &format!("Found {} filtered commands", display_commands.len()));
        
        if display_commands.is_empty() {
            detailed_log("SHOW_FOLDER", "No filtered commands to show folder for");
            return;
        }
        
        // Get all commands for alias resolution
        let all_commands = crate::core::data::get_commands();
        
        // Log first few commands for debugging
        for (i, cmd) in display_commands.iter().take(3).enumerate() {
            detailed_log("SHOW_FOLDER", &format!("  Command {}: '{}' (action: {}, arg: {})", 
                i, cmd.command, cmd.action, cmd.arg));
        }
        
        // Use the currently selected command and extract its folder
        let selected_idx = self.selected_index();
        if selected_idx >= display_commands.len() {
            detailed_log("SHOW_FOLDER", &format!("Selected index {} is out of bounds ({})", selected_idx, display_commands.len()));
            return;
        }
        
        let cmd = &display_commands[selected_idx];
        detailed_log("SHOW_FOLDER", &format!("Using selected command at index {}: '{}'", selected_idx, cmd.command));
        
        if PopupState::is_separator_command(cmd) {
            detailed_log("SHOW_FOLDER", &format!("Selected command is a separator: '{}'", cmd.command));
            return;
        }
            
        detailed_log("SHOW_FOLDER", &format!("Processing command: '{}' (action: {})", cmd.command, cmd.action));
        
        // Recursively resolve aliases
        let resolved_cmd = self.resolve_aliases_recursively(cmd, &all_commands);
        
        // Extract folder path using the command's built-in method
        let folder_path = if let Some(abs_path) = resolved_cmd.get_absolute_folder_path(&self.popup_state.config) {
            let path_str = abs_path.to_string_lossy().to_string();
            detailed_log("SHOW_FOLDER", &format!("Found {} action, extracted folder: {}", resolved_cmd.action, path_str));
            Some(path_str)
        } else if resolved_cmd.action == "alias" {
            // This shouldn't happen since we recursively resolved aliases above
            detailed_log("SHOW_FOLDER", &format!("Unresolved alias found: '{}'", resolved_cmd.command));
            None
        } else {
            detailed_log("SHOW_FOLDER", &format!("Command '{}' has non-folder action: {}", resolved_cmd.command, resolved_cmd.action));
            None
        };
        
        if let Some(path) = folder_path {
            log(&format!("SHOW_FOLDER: Attempting to launch folder: '{}'", path));
            
            // Launch the folder (popup stays open)
            let folder_action = crate::execute::make_action("folder", &path);
            log(&format!("SHOW_FOLDER: Created action with type='folder', arg='{}'", path));
            // Execute the action
            let mut variables = std::collections::HashMap::new();
            variables.insert("arg".to_string(), path.clone());
            let _ = crate::execute::execute_on_server(&folder_action, Some(variables));
            log(&format!("SHOW_FOLDER: Sent folder command to server"));
        } else {
            detailed_log("SHOW_FOLDER", &format!("No folder found for selected command: '{}'", cmd.command));
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
            detailed_log("SHOW_CONTACT", "No commands available or invalid selection");
            return;
        }
        
        // Get the selected command
        let selected_command = &display_commands[selected_index];
        
        detailed_log("SHOW_CONTACT", &format!("Processing command: {}", selected_command.command));
        
        // Resolve aliases to get the actual command
        let all_commands = crate::core::data::get_commands();
        let resolved_cmd = self.resolve_aliases_recursively(selected_command, &all_commands);
        
        // Strip @ prefix from the resolved command name if present
        let contact_name = if resolved_cmd.command.starts_with("@") {
            resolved_cmd.command[1..].to_string()
        } else {
            resolved_cmd.command.clone()
        };
        
        detailed_log("SHOW_CONTACT", &format!("Contact name after stripping @: {}", contact_name));
        
        // Execute the contact action through the command server
        // Create a command object to send to the server
        let contact_cmd = crate::core::Command {
            patch: String::new(),
            command: format!("Show contact: {}", contact_name),
            action: "contact".to_string(),
            arg: contact_name.clone(),
            flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
        };
        
        detailed_log("SHOW_CONTACT", &format!("SHOW_CONTACT: Opening contact: {}", contact_name));
        
        // Execute via server
        let contact_action = crate::execute::command_to_action(&contact_cmd);
        let mut variables = std::collections::HashMap::new();
        variables.insert("arg".to_string(), contact_cmd.arg.clone());
        let _ = crate::execute::execute_on_server(&contact_action, Some(variables));
    }
    
    
    /// Activate TMUX session for selected anchor/folder (formerly activate_anchor)
    fn activate_tmux(&mut self) {
        use crate::utils;
        use std::process::Command;
        use std::path::Path;
        
        // Check if we should use JavaScript version (via config option)
        if let Some(settings) = &self.popup_state.config.launcher_settings {
            if let Some(use_js) = &settings.use_javascript_tmux_activation {
                if use_js == "true" {
                    detailed_log("TMUX", &format!("🚀 TMUX: Using JavaScript TMUX implementation"));
                    
                    // Get selected command index
                    let selected_index = self.selected_index();
                    let display_commands = self.popup_state.filtered_commands.clone();
                    
                    if display_commands.is_empty() || selected_index >= display_commands.len() {
                        detailed_log("TMUX", &format!("❌ TMUX: No commands available for JavaScript"));
                        return;
                    }
                    
                    // Get the selected command and resolve aliases
                    let all_commands = crate::core::data::get_commands();
                    let selected_command = &display_commands[selected_index];
                    let resolved_cmd = self.resolve_aliases_recursively(selected_command, &all_commands);
                    
                    detailed_log("TMUX", &format!("📋 TMUX: Selected: '{}', Resolved: '{}'", 
                        selected_command.command, resolved_cmd.command));
                    
                    // Create command to execute JavaScript action
                    // Use "activate_tmux_js" which is the actual JS action in config.yaml
                    let js_cmd = crate::core::Command {
                        command: resolved_cmd.command.clone(),
                        action: "activate_tmux_js".to_string(),
                        arg: resolved_cmd.arg.clone(),
                        patch: resolved_cmd.patch.clone(),
                        flags: String::new(),
        other_params: None,
        last_update: 0,
        file_size: None,
                    };
                    
                    detailed_log("TMUX", &format!("🎯 TMUX: Executing JavaScript action with arg: {}", js_cmd.arg));
                    let js_action = crate::execute::command_to_action(&js_cmd);
                    let mut variables = std::collections::HashMap::new();
                    variables.insert("arg".to_string(), js_cmd.arg.clone());
                    let _ = crate::execute::execute_on_server(&js_action, Some(variables));
                    return;
                }
            }
        }
        
        // Original Rust implementation for TMUX session activation
        detailed_log("TMUX", &format!("🦀 TMUX: Using Rust implementation for TMUX session activation"));
        
        // Get selected command index
        let selected_index = self.selected_index();
        
        // Get the current filtered commands  
        let display_commands = self.popup_state.filtered_commands.clone();
        
        if display_commands.is_empty() || selected_index >= display_commands.len() {
            detailed_log("TMUX", &format!("TMUX: No commands available or invalid selection"));
            return;
        }
        
        // Get the selected command and resolve aliases
        let all_commands = crate::core::data::get_commands();
        let selected_command = &display_commands[selected_index];
        
        // DEBUG: Log the selected command details
        detailed_log("TMUX", &format!("TMUX: Selected command: '{}'", selected_command.command));
        detailed_log("TMUX", &format!("TMUX: Selected action: '{}'", selected_command.action));
        detailed_log("TMUX", &format!("TMUX: Selected arg: '{}'", selected_command.arg));
        detailed_log("TMUX", &format!("TMUX: Selected patch: '{}'", selected_command.patch));
        detailed_log("TMUX", &format!("TMUX: Selected flags: '{}'", selected_command.flags));
        
        let resolved_cmd = self.resolve_aliases_recursively(selected_command, &all_commands);
        
        // DEBUG: Log resolved command if different
        if resolved_cmd.command != selected_command.command {
            detailed_log("TMUX", &format!("TMUX: Resolved to: '{}'", resolved_cmd.command));
            detailed_log("TMUX", &format!("TMUX: Resolved action: '{}'", resolved_cmd.action));
            detailed_log("TMUX", &format!("TMUX: Resolved arg: '{}'", resolved_cmd.arg));
        }
        
        // Extract folder path
        detailed_log("TMUX", &format!("TMUX: Attempting to extract folder path..."));
        let folder_path = if let Some(abs_path) = resolved_cmd.get_absolute_folder_path(&self.popup_state.config) {
            abs_path.to_string_lossy().to_string()
        } else {
            detailed_log("TMUX", &format!("TMUX: ERROR - Could not extract folder path from command: '{}'", resolved_cmd.command));
            detailed_log("TMUX", &format!("TMUX: Command details - action: '{}', arg: '{}'", resolved_cmd.action, resolved_cmd.arg));
            return;
        };
        
        detailed_log("TMUX", &format!("TMUX: Successfully extracted folder path: '{}'", folder_path));
        
        // Direct Rust implementation of TMUX activation (temporary - will be replaced with JavaScript)
        // This is based on the working Python anchor script logic
        
        // 1. Open folder in Finder
        //if let Err(e) = Command::new("open").arg(&folder_path).spawn() {
        //    detailed_log("TMUX", &format!("TMUX: Failed to open folder: {}", e));
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
        //        detailed_log("TMUX", &format!("TMUX: Would open in Obsidian vault: {}", vault_name));
        //    }
        //}
        
        // 3. Check for .tmuxp.yaml and activate TMUX session
        let tmuxp_path = Path::new(&folder_path).join(".tmuxp.yaml");
        if tmuxp_path.exists() {
            detailed_log("TMUX", &format!("TMUX: Found .tmuxp.yaml at {:?}", tmuxp_path));
            
            // Parse the .tmuxp.yaml file to get the actual session name
            detailed_log("TMUX_RUST", &format!("Line 2354: Parsing .tmuxp.yaml to get session_name"));
            let session_name = match std::fs::read_to_string(&tmuxp_path) {
                Ok(yaml_content) => {
                    let mut found_session_name = None;
                    for line in yaml_content.lines() {
                        if line.trim_start().starts_with("session_name:") {
                            // Extract the session name after the colon
                            let parts: Vec<&str> = line.splitn(2, ':').collect();
                            if parts.len() == 2 {
                                let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                                detailed_log("TMUX_RUST", &format!("Line 2360: Found session_name in YAML: '{}'", name));
                                found_session_name = Some(name.to_string());
                                break;
                            }
                        }
                    }
                    
                    if let Some(name) = found_session_name {
                        detailed_log("TMUX", &format!("TMUX: Using session name from .tmuxp.yaml: '{}'", name));
                        name
                    } else {
                        // No session_name found in YAML - this is an error
                        detailed_log("TMUX", &format!("TMUX: ERROR - No session_name found in .tmuxp.yaml"));
                        detailed_log("TMUX_RUST", "Line 2373: ERROR - .tmuxp.yaml missing session_name field");
                        crate::utils::error(&format!(
                            "The .tmuxp.yaml file at:\n{}\n\nis missing a 'session_name' field.\n\nPlease add:\nsession_name: <name>\n\nto the file.",
                            tmuxp_path.display()
                        ));
                        return;
                    }
                }
                Err(e) => {
                    detailed_log("TMUX", &format!("TMUX: ERROR - Failed to read .tmuxp.yaml: {}", e));
                    detailed_log("TMUX_RUST", &format!("Line 2382: Failed to read .tmuxp.yaml: {}", e));
                    crate::utils::error(&format!(
                        "Failed to read .tmuxp.yaml file:\n{}\n\nError: {}",
                        tmuxp_path.display(),
                        e
                    ));
                    return;
                }
            };
            
            detailed_log("TMUX", &format!("TMUX: Session name from YAML: '{}'", session_name));
            
            // Check if session exists
            let check_session = Command::new("/opt/homebrew/bin/tmux")
                .args(&["has-session", "-t", &session_name])
                .output();
                
            let session_exists = check_session.map(|o| o.status.success()).unwrap_or(false);
            
            if !session_exists {
                detailed_log("TMUX", &format!("TMUX: Creating new tmux session '{}'", session_name));
                detailed_log("TMUX_RUST", &format!("Line 2384: Session '{}' does not exist, creating with tmuxp", session_name));
                
                // First, let's see what sessions exist before we create
                detailed_log("TMUX_RUST", "Line 2386: Checking existing sessions before creation");
                if let Ok(existing) = Command::new("/opt/homebrew/bin/tmux")
                    .args(&["list-sessions", "-F", "#{session_name}"])
                    .output() {
                    let sessions = String::from_utf8_lossy(&existing.stdout);
                    detailed_log("TMUX_RUST", &format!("Line 2390: Existing sessions: {}", sessions.trim()));
                }
                
                // Create session with tmuxp
                // Since we expect the .tmuxp.yaml to have the correct session name (with underscores),
                // we don't use -s flag which seems to cause issues
                // Add /opt/homebrew/bin to PATH so tmuxp can find tmux
                let tmuxp_cmd = format!("/opt/homebrew/bin/tmuxp load '{}' -d", tmuxp_path.to_str().unwrap_or(""));
                detailed_log("TMUX_RUST", &format!("Line 2397: Running command: {}", tmuxp_cmd));
                detailed_log("TMUX_RUST", &format!("Line 2398: Current dir: {}", folder_path));
                
                match Command::new("/opt/homebrew/bin/tmuxp")
                    .args(&["load", tmuxp_path.to_str().unwrap_or(""), "-d"])
                    .current_dir(&folder_path)
                    .env("PATH", format!("/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:{}", 
                        std::env::var("PATH").unwrap_or_default()))
                    .output() {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        
                        detailed_log("TMUX_RUST", &format!("Line 2408: tmuxp exit code: {:?}", output.status.code()));
                        detailed_log("TMUX_RUST", &format!("Line 2409: tmuxp stdout: '{}'", stdout));
                        detailed_log("TMUX_RUST", &format!("Line 2410: tmuxp stderr: '{}'", stderr));
                        
                        if !output.status.success() {
                            detailed_log("TMUX", &format!("TMUX: tmuxp failed with exit code {:?}", output.status.code()));
                            detailed_log("TMUX", &format!("TMUX: tmuxp stderr: {}", stderr));
                            detailed_log("TMUX", &format!("TMUX: tmuxp stdout: {}", stdout));
                            detailed_log("TMUX_RUST", "Line 2415: tmuxp FAILED, returning");
                            return;
                        }
                        
                        // Log output even on success to debug issues
                        detailed_log("TMUX", &format!("TMUX: tmuxp completed with exit code 0"));
                        if !stdout.is_empty() {
                            detailed_log("TMUX", &format!("TMUX: tmuxp stdout: {}", stdout));
                        }
                        if !stderr.is_empty() {
                            detailed_log("TMUX", &format!("TMUX: tmuxp stderr: {}", stderr));
                        }
                        
                        // Check what sessions exist after tmuxp
                        detailed_log("TMUX_RUST", "Line 2428: Checking sessions after tmuxp");
                        if let Ok(after) = Command::new("/opt/homebrew/bin/tmux")
                            .args(&["list-sessions", "-F", "#{session_name}"])
                            .output() {
                            let sessions = String::from_utf8_lossy(&after.stdout);
                            detailed_log("TMUX_RUST", &format!("Line 2432: Sessions after tmuxp: {}", sessions.trim()));
                        }
                        
                        // Verify the session was actually created
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        detailed_log("TMUX_RUST", &format!("Line 2436: Verifying session '{}' exists", session_name));
                        let verify = Command::new("/opt/homebrew/bin/tmux")
                            .args(&["has-session", "-t", &session_name])
                            .output();
                        
                        if let Ok(v) = verify {
                            let verify_stderr = String::from_utf8_lossy(&v.stderr);
                            detailed_log("TMUX_RUST", &format!("Line 2442: Verify exit code: {}", v.status.code().unwrap_or(-1)));
                            detailed_log("TMUX_RUST", &format!("Line 2443: Verify stderr: '{}'", verify_stderr));
                            
                            if v.status.success() {
                                detailed_log("TMUX", &format!("TMUX: Verified session '{}' exists", session_name));
                                
                                // Run claude --continue in the new session
                                // Add a small delay to ensure the window is created
                                std::thread::sleep(std::time::Duration::from_millis(500));
                                detailed_log("TMUX", &format!("TMUX: Running claude --continue in new tmuxp session '{}'", session_name));
                                match Command::new("/opt/homebrew/bin/tmux")
                                    .args(&["send-keys", "-t", &session_name, "claude --continue", "C-m"])
                                    .output() {
                                    Ok(output) => {
                                        if output.status.success() {
                                            detailed_log("TMUX", &format!("TMUX: Successfully sent claude --continue to tmuxp session '{}'", session_name));
                                        } else {
                                            detailed_log("TMUX", &format!("TMUX: Failed to send claude --continue to tmuxp session: {}", String::from_utf8_lossy(&output.stderr)));
                                        }
                                    }
                                    Err(e) => {
                                        detailed_log("TMUX", &format!("TMUX: Error sending claude --continue to tmuxp session: {}", e));
                                    }
                                }
                            } else {
                                // tmuxp said it succeeded but session wasn't created - this is an error
                                detailed_log("TMUX", &format!("TMUX: ERROR - Session '{}' was NOT created despite tmuxp reporting success", session_name));
                                detailed_log("TMUX", &format!("TMUX: tmux has-session stderr: {}", verify_stderr));
                                detailed_log("TMUX_RUST", &format!("Line 2479: CRITICAL ERROR - tmuxp succeeded but session '{}' doesn't exist", session_name));
                                
                                // Let's check what's in the .tmuxp.yaml file
                                detailed_log("TMUX_RUST", &format!("Line 2481: Reading .tmuxp.yaml to check session_name"));
                                if let Ok(yaml_content) = std::fs::read_to_string(&tmuxp_path) {
                                    // Look for session_name in the YAML
                                    for line in yaml_content.lines() {
                                        if line.contains("session_name") {
                                            detailed_log("TMUX_RUST", &format!("Line 2485: Found in .tmuxp.yaml: {}", line.trim()));
                                            break;
                                        }
                                    }
                                }
                                
                                // Try a fallback: check if tmuxp created a session with a different name
                                detailed_log("TMUX_RUST", "Line 2490: Checking if tmuxp created session with different name");
                                if let Ok(after_create) = Command::new("/opt/homebrew/bin/tmux")
                                    .args(&["list-sessions", "-F", "#{session_name}"])
                                    .output() {
                                    let all_sessions = String::from_utf8_lossy(&after_create.stdout);
                                    detailed_log("TMUX_RUST", &format!("Line 2494: All sessions after tmuxp: {}", all_sessions.trim()));
                                    
                                    // Try to find a session that might match our expected session name
                                    for session in all_sessions.lines() {
                                        if session.contains(&session_name) || session_name.contains(session) {
                                            detailed_log("TMUX_RUST", &format!("Line 2498: Found possible match: '{}'", session));
                                        }
                                    }
                                }
                                
                                // Check if tmuxp can't find tmux in PATH
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                if stdout.contains("tmux not found") {
                                    crate::utils::error("tmuxp cannot find tmux in PATH.\nEnsure tmux is installed and in your PATH.");
                                } else {
                                    crate::utils::error(&format!(
                                        "Failed to create TMUX session '{}' with tmuxp.\nCheck ~/.config/hookanchor/anchor.log for details.",
                                        session_name
                                    ));
                                }
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        detailed_log("TMUX", &format!("TMUX: Failed to run tmuxp: {}", e));
                        return;
                    }
                }
                // Give tmux a moment to stabilize
                std::thread::sleep(std::time::Duration::from_millis(100));
            } else {
                detailed_log("TMUX", &format!("TMUX: Session '{}' already exists", session_name));
            }
            
            // Now we need to switch to the session in an existing TMUX client
            // First check if there's a TMUX client running anywhere
            detailed_log("TMUX", &format!("TMUX: Checking for existing TMUX clients"));
            
            // Check if any tmux clients are attached
            let list_clients = Command::new("/opt/homebrew/bin/tmux")
                .args(&["list-clients"])
                .output();
                
            let has_clients = match list_clients {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let has = output.status.success() && !stdout.trim().is_empty();
                    if has {
                        detailed_log("TMUX", &format!("TMUX: Found TMUX clients: {}", stdout.trim()));
                    } else {
                        detailed_log("TMUX", &format!("TMUX: No TMUX clients found"));
                    }
                    has
                }
                Err(e) => {
                    detailed_log("TMUX", &format!("TMUX: Failed to list clients: {}", e));
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
                detailed_log("TMUX", &format!("TMUX: {}", error_msg));
                crate::utils::error(&error_msg);
                return;
            }
            
            // There are TMUX clients, so we can switch
            detailed_log("TMUX", &format!("TMUX: Switching to session '{}' in existing client", session_name));
            
            // Use switch-client which will switch in any attached client
            match Command::new("/opt/homebrew/bin/tmux")
                .args(&["switch-client", "-t", &session_name])
                .output() {
                Ok(output) => {
                    if output.status.success() {
                        detailed_log("TMUX", &format!("TMUX: Successfully switched to session '{}'", session_name));
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        detailed_log("TMUX", &format!("TMUX: Failed to switch: {}", stderr));
                        crate::utils::error(&format!(
                            "Failed to switch to TMUX session '{}': {}",
                            session_name, stderr
                        ));
                    }
                }
                Err(e) => {
                    detailed_log("TMUX", &format!("TMUX: Failed to run switch-client: {}", e));
                    crate::utils::error(&format!(
                        "Failed to switch to TMUX session '{}': {}",
                        session_name, e
                    ));
                }
            }
                
        } else {
            detailed_log("TMUX", &format!("TMUX: No .tmuxp.yaml found - creating basic TMUX session"));
            
            // Extract folder name for session name
            let folder_name = Path::new(&folder_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("session");
            
            // Sanitize the session name for tmux compatibility
            // tmux session names can only contain: alphanumeric, underscore, hyphen, dot
            // Replace all other special characters with underscore
            let session_name = folder_name
                .replace(' ', "_")
                .replace('@', "_")
                .replace(':', "_")
                .replace('.', "_")
                .replace('[', "_")
                .replace(']', "_")
                .replace('(', "_")
                .replace(')', "_")
                .replace('$', "_")
                .replace('#', "_")
                .replace('&', "_")
                .replace('*', "_")
                .replace('!', "_")
                .replace('?', "_")
                .replace('/', "_")
                .replace('\\', "_")
                .replace('\'', "_")
                .replace('"', "_");
            
            detailed_log("TMUX", &format!("TMUX: Creating basic session '{}' in folder '{}'", session_name, folder_path));
            
            // Check if session already exists
            let check_session = Command::new("/opt/homebrew/bin/tmux")
                .args(&["has-session", "-t", &session_name])
                .output();
            
            let session_exists = check_session.map(|o| o.status.success()).unwrap_or(false);
            
            if !session_exists {
                // Create new basic tmux session (shell will be started, claude command sent after)
                detailed_log("TMUX", &format!("TMUX: Running: tmux new-session -d -s '{}' -c '{}'", 
                    session_name, folder_path));
                
                // First, let's list all existing sessions before creation
                if let Ok(list_before) = Command::new("/opt/homebrew/bin/tmux")
                    .args(&["list-sessions"])
                    .output() {
                    let sessions_before = String::from_utf8_lossy(&list_before.stdout);
                    detailed_log("TMUX", &format!("TMUX: Sessions BEFORE creation:\n{}", 
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
                        detailed_log("TMUX", &format!("TMUX: new-session exit code: {:?}", output.status.code()));
                        if !stdout.is_empty() {
                            detailed_log("TMUX", &format!("TMUX: new-session stdout: {}", stdout));
                        }
                        if !stderr.is_empty() {
                            detailed_log("TMUX", &format!("TMUX: new-session stderr: {}", stderr));
                        }
                        
                        // List sessions after attempted creation
                        if let Ok(list_after) = Command::new("/opt/homebrew/bin/tmux")
                            .args(&["list-sessions"])
                            .output() {
                            let sessions_after = String::from_utf8_lossy(&list_after.stdout);
                            detailed_log("TMUX", &format!("TMUX: Sessions AFTER creation:\n{}", 
                                if sessions_after.trim().is_empty() { "(no sessions)" } else { sessions_after.trim() }));
                        }
                        
                        if output.status.success() {
                            detailed_log("TMUX", &format!("TMUX: Basic session '{}' creation reported success", session_name));
                            
                            // Check if session immediately exists without delay
                            if let Ok(immediate_check) = Command::new("/opt/homebrew/bin/tmux")
                                .args(&["has-session", "-t", &session_name])
                                .output() {
                                if immediate_check.status.success() {
                                    detailed_log("TMUX", &format!("TMUX: ✅ Session '{}' exists immediately after creation", session_name));
                                } else {
                                    detailed_log("TMUX", &format!("TMUX: ❌ Session '{}' NOT found immediately after creation", session_name));
                                    let stderr = String::from_utf8_lossy(&immediate_check.stderr);
                                    detailed_log("TMUX", &format!("TMUX: Immediate check stderr: {}", stderr));
                                }
                            }
                            
                            // Give tmux more time to fully create the session
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            
                            // Verify it was created - with more detailed logging
                            detailed_log("TMUX", &format!("TMUX: Verifying session '{}' exists...", session_name));
                            match Command::new("/opt/homebrew/bin/tmux")
                                .args(&["has-session", "-t", &session_name])
                                .output() {
                                Ok(verify) => {
                                    let verify_stdout = String::from_utf8_lossy(&verify.stdout);
                                    let verify_stderr = String::from_utf8_lossy(&verify.stderr);
                                    detailed_log("TMUX", &format!("TMUX: has-session exit code: {:?}", verify.status.code()));
                                    if !verify_stdout.is_empty() {
                                        detailed_log("TMUX", &format!("TMUX: has-session stdout: {}", verify_stdout));
                                    }
                                    if !verify_stderr.is_empty() {
                                        detailed_log("TMUX", &format!("TMUX: has-session stderr: {}", verify_stderr));
                                    }
                                    
                                    if verify.status.success() {
                                        detailed_log("TMUX", &format!("TMUX: ✅ Session '{}' verified!", session_name));
                                        
                                        // Now send the claude --continue command to the session
                                        detailed_log("TMUX", &format!("TMUX: Sending 'claude --continue' to session '{}'", session_name));
                                        match Command::new("/opt/homebrew/bin/tmux")
                                            .args(&["send-keys", "-t", &session_name, "claude --continue", "C-m"])
                                            .output() {
                                            Ok(send_output) => {
                                                if send_output.status.success() {
                                                    detailed_log("TMUX", &format!("TMUX: ✅ Successfully sent claude command to session '{}'", session_name));
                                                } else {
                                                    let send_stderr = String::from_utf8_lossy(&send_output.stderr);
                                                    detailed_log("TMUX", &format!("TMUX: ❌ Failed to send claude command: {}", send_stderr));
                                                }
                                            }
                                            Err(e) => {
                                                detailed_log("TMUX", &format!("TMUX: Error sending claude command: {}", e));
                                            }
                                        }
                                    } else {
                                        // Session doesn't exist despite tmux new-session reporting success
                                        detailed_log("TMUX", &format!("TMUX: ❌ Session '{}' NOT found after creation!", session_name));
                                        
                                        // Check if stderr contains a specific error
                                        if verify_stderr.contains("duplicate session") {
                                            detailed_log("TMUX", &format!("TMUX: ERROR - Duplicate session name detected"));
                                        } else if verify_stderr.contains("server not found") {
                                            detailed_log("TMUX", &format!("TMUX: ERROR - No TMUX server running"));
                                        }
                                        
                                        // Try to get more info about what's wrong
                                        if let Ok(info) = Command::new("/opt/homebrew/bin/tmux")
                                            .args(&["info"])
                                            .output() {
                                            let info_out = String::from_utf8_lossy(&info.stderr);
                                            if !info_out.is_empty() {
                                                detailed_log("TMUX", &format!("TMUX: tmux info stderr: {}", info_out));
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    detailed_log("TMUX", &format!("TMUX: Error running has-session: {}", e));
                                }
                            }
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            detailed_log("TMUX", &format!("TMUX: Failed to create session: {}", stderr));
                            crate::utils::error(&format!("Failed to create TMUX session: {}", stderr));
                            return;
                        }
                    }
                    Err(e) => {
                        detailed_log("TMUX", &format!("TMUX: Error executing tmux command: {}", e));
                        crate::utils::error(&format!("Failed to execute tmux: {}", e));
                        return;
                    }
                }
            } else {
                detailed_log("TMUX", &format!("TMUX: Session '{}' already exists", session_name));
            }
            
            // Now switch to the session
            detailed_log("TMUX", &format!("TMUX: Checking for existing TMUX clients"));
            
            let list_clients = Command::new("/opt/homebrew/bin/tmux")
                .args(&["list-clients"])
                .output();
            
            let has_clients = match list_clients {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let has = output.status.success() && !stdout.trim().is_empty();
                    if has {
                        detailed_log("TMUX", &format!("TMUX: Found TMUX clients: {}", stdout.trim()));
                    } else {
                        detailed_log("TMUX", &format!("TMUX: No TMUX clients found"));
                    }
                    has
                }
                Err(e) => {
                    detailed_log("TMUX", &format!("TMUX: Failed to list clients: {}", e));
                    false
                }
            };
            
            if has_clients {
                // Switch to the session
                detailed_log("TMUX", &format!("TMUX: Switching to session '{}'", session_name));
                
                match Command::new("/opt/homebrew/bin/tmux")
                    .args(&["switch-client", "-t", &session_name])
                    .output() {
                    Ok(output) => {
                        if output.status.success() {
                            detailed_log("TMUX", &format!("TMUX: Successfully switched to session '{}'", session_name));
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            detailed_log("TMUX", &format!("TMUX: Failed to switch: {}", stderr));
                        }
                    }
                    Err(e) => {
                        detailed_log("TMUX", &format!("TMUX: Error switching: {}", e));
                    }
                }
            } else {
                // No client to switch, just inform user
                let msg = format!(
                    "TMUX session '{}' created.\nOpen a terminal and run:\ntmux attach-session -t '{}'",
                    session_name, session_name
                );
                detailed_log("TMUX", &format!("TMUX: {}", msg));
                crate::utils::error(&msg);
            }

        }
    }

    /// Create a child command using template from active anchor's patch hierarchy
    fn create_child(&mut self) {
        use crate::utils;

        // Get the input text (will become the child name)
        let child_name = self.popup_state.search_text.clone();
        if child_name.trim().is_empty() {
            log("CREATE_CHILD: No input text for child name");
            return;
        }

        // Get the active anchor from state
        let state = crate::core::data::get_state();
        let config = crate::core::data::get_config();

        let template_name = match &state.anchor_name {
            Some(anchor_name) => {
                log(&format!("CREATE_CHILD: Using active anchor '{}'", anchor_name));

                // Find the anchor command to get its patch
                let all_commands = crate::core::data::get_commands();
                let anchor_cmd = all_commands.iter().find(|cmd| cmd.command == *anchor_name);

                match anchor_cmd {
                    Some(cmd) => {
                        if cmd.patch.is_empty() {
                            log(&format!("CREATE_CHILD: Anchor '{}' has no patch", anchor_name));
                            // Use default_anchor_template
                            config.template_settings
                                .as_ref()
                                .and_then(|ts| ts.default_anchor_template.clone())
                                .unwrap_or_else(|| "sub_markdown".to_string())
                        } else {
                            // Build patches hashmap for hierarchy walking
                            let patches = crate::core::commands::create_patches_hashmap(&all_commands);

                            // Walk up the hierarchy looking for a template parameter
                            let mut found_template = None;
                            for patch in crate::core::commands::walk_patch_hierarchy(&cmd.patch, &patches, 100) {
                                if let Some(anchor) = patch.primary_anchor() {
                                    if let Some(params) = &anchor.other_params {
                                        if let Some(template) = params.get(crate::utils::PARAM_TEMPLATE) {
                                            log(&format!("CREATE_CHILD: Found template '{}' on anchor '{}' in hierarchy",
                                                template, anchor.command));
                                            found_template = Some(template.clone());
                                            break;
                                        }
                                    }
                                }
                            }

                            found_template.unwrap_or_else(|| {
                                // No template found in hierarchy - use default_anchor_template
                                log("CREATE_CHILD: No template in hierarchy, using default_anchor_template");
                                config.template_settings
                                    .as_ref()
                                    .and_then(|ts| ts.default_anchor_template.clone())
                                    .unwrap_or_else(|| "sub_markdown".to_string())
                            })
                        }
                    }
                    None => {
                        log(&format!("CREATE_CHILD: Anchor '{}' not found", anchor_name));
                        crate::utils::error(&format!("Anchor command '{}' not found.", anchor_name));
                        return;
                    }
                }
            }
            None => {
                // No active anchor - use default_no_anchor_template
                log("CREATE_CHILD: No active anchor, using default_no_anchor_template");
                config.template_settings
                    .as_ref()
                    .and_then(|ts| ts.default_no_anchor_template.clone())
                    .unwrap_or_else(|| "sub_markdown".to_string())
            }
        };

        log(&format!("CREATE_CHILD: Using template '{}' to create child '{}'",
            template_name, child_name));

        // Temporarily replace search text with child name so template expansion uses it
        let original_search = self.popup_state.search_text.clone();
        self.popup_state.search_text = child_name.clone();
        self.popup_state.update_search(child_name.clone());

        // Use the same code path as keyboard-triggered templates
        self.handle_template_create_named_impl(&template_name);

        // Restore original search text
        self.popup_state.search_text = original_search.clone();
        self.popup_state.update_search(original_search);
    }
}


fn save_window_position_with_reason(pos: egui::Pos2, reason: &str) {
    let mut state = crate::core::data::get_state();
    let old_pos = state.window_position;

    // Log the save with reason and check for drift
    let msg = if pos.y > 600.0 {
        format!("[POSITION-SAVE] WARNING: Saving low position ({}, {}) from {:?} - reason: {} - POSITION TOO LOW!",
                pos.x, pos.y, old_pos, reason)
    } else {
        format!("[POSITION-SAVE] Saving position ({}, {}) from {:?} - reason: {}",
                pos.x, pos.y, old_pos, reason)
    };
    log(&msg);

    state.window_position = Some((pos.x, pos.y));
    match crate::core::data::set_state(&state) {
        Ok(_) => {
            detailed_log("WINDOW_POS", &format!("Successfully saved position"));
        }
        Err(e) => {
            log(&format!("[POSITION-SAVE] ERROR: Failed to save position: {}", e));
        }
    }
}

fn load_window_position() -> Option<egui::Pos2> {
    let state = crate::core::data::get_state();
    let result = state.window_position.map(|(x, y)| egui::pos2(x, y));
    
    if let Some(pos) = result {
        if pos.y > 600.0 {
            detailed_log("POSITION-LOAD", &format!("WARNING: Loading low position ({}, {}) - POSITION TOO LOW!", pos.x, pos.y));
        } else {
            detailed_log("POSITION-LOAD", &format!("Loading saved position ({}, {})", pos.x, pos.y));
        }
    } else {
        detailed_log("POSITION-LOAD", "No saved position available");
    }
    
    result
}

fn get_previous_window_location(ctx: &egui::Context, window_size: egui::Vec2) -> egui::Pos2 {
    detailed_log("WINDOW_POS", &format!("get_previous_window_location called with window_size: {:?}", window_size));
    
    // First try to load the previous position
    if let Some(previous_pos) = load_window_position() {
        detailed_log("WINDOW_POS", &format!("Loaded previous position: {:?}", previous_pos));
        // Check if this position is visible on any current display
        if is_position_visible(previous_pos, window_size) {
            detailed_log("WINDOW_POS", "Position is visible, returning it");
            return previous_pos;
        } else {
            detailed_log("WINDOW_POS", "Position NOT visible, will center instead");
        }
    } else {
        detailed_log("WINDOW_POS", "No previous position found in state");
    }
    
    // If no previous position or not visible, center on main display
    let centered = center_on_main_display(ctx, window_size);
    detailed_log("WINDOW_POS", &format!("Centering on main display at: {:?}", centered));
    centered
}

fn is_position_visible(pos: egui::Pos2, window_size: egui::Vec2) -> bool {
    detailed_log("WINDOW_POS", &format!("is_position_visible checking pos: {:?}, window_size: {:?}", pos, window_size));

    let window_rect = egui::Rect::from_min_size(pos, window_size);
    detailed_log("WINDOW_POS", &format!("Window rect: {:?}", window_rect));

    // Get actual screen dimensions instead of hardcoded values
    let (screen_width, screen_height) = get_actual_screen_dimensions();
    let main_display_rect = egui::Rect::from_min_size(
        egui::pos2(0.0, 0.0),
        egui::vec2(screen_width, screen_height)
    );
    detailed_log("WINDOW_POS", &format!("Main display rect: {:?} (actual screen: {}x{})", main_display_rect, screen_width, screen_height));

    // Check if at least part of the window is visible
    // Allow for window to be partially off-screen but require some overlap
    let min_visible_area = window_size.x.min(window_size.y) * 0.3; // 30% of smaller dimension

    let intersection = main_display_rect.intersect(window_rect);
    let is_visible = intersection.width() * intersection.height() >= min_visible_area * min_visible_area;
    detailed_log("WINDOW_POS", &format!("Position visible check: {} (intersection: {:?})", is_visible, intersection));
    is_visible
}

/// Get the actual screen dimensions (not window dimensions)
fn get_actual_screen_dimensions() -> (f32, f32) {
    #[cfg(target_os = "macos")]
    {
        let display = CGDisplay::main();
        let width = display.pixels_wide() as f32;
        let height = display.pixels_high() as f32;
        detailed_log("SYSTEM", &format!("🔵 SCREEN: Got actual screen dimensions from CoreGraphics: {}x{}", width, height));
        (width, height)
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        detailed_log("SYSTEM", &format!("🔵 SCREEN: Using default screen dimensions (not macOS)"));
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
        if let Some((_, resolved_command)) = self.popup_state.get_prefix_menu_command_info() {
            let (sys_data, _) = crate::core::data::get_sys_data();
            let path = get_patch_path(&resolved_command.command, &sys_data.patches);
            
            if path.is_empty() {
                resolved_command.command.clone()
            } else {
                path.join(" > ")
            }
        } else {
            // Fallback to old behavior
            let (sys_data, _) = crate::core::data::get_sys_data();
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
        // ⏱️ Track if window just became visible (for timing analysis)
        let was_hidden_last_frame = self.is_hidden;

        // 🔑 UPSTREAM OS EVENT LOGGING - Log ALL events from operating system first
        ctx.input(|input| {
            if !input.events.is_empty() {
                detailed_log("OS_EVENTS", &format!("🔑 OS delivered {} events to application", input.events.len()));
                for (i, event) in input.events.iter().enumerate() {
                    detailed_log("OS_EVENTS", &format!("🔑 OS_EVENT[{}]: {:?}", i, event));

                    // Special detailed logging for key events
                    if let egui::Event::Key { key, pressed, modifiers, .. } = event {
                        detailed_log("OS_EVENTS", &format!("🔑 OS_KEY: key={:?}, pressed={}, ctrl={}, alt={}, shift={}, cmd={}",
                            key, pressed, modifiers.ctrl, modifiers.alt, modifiers.shift, modifiers.command));
                    }
                }
            }
        });

        // ⏱️ Log when window first becomes visible after being hidden
        if was_hidden_last_frame && !self.is_hidden {
            log(&format!("⏱️ [VISIBILITY_TIMING] First frame after becoming visible - frame_count={}, focus_set={}",
                self.frame_count, self.focus_set));

            // 🔍 Verify we're running the latest binary (dev machines only)
            // This catches cases where code was rebuilt but server wasn't restarted
            // Only check if fully loaded to avoid accessing uninitialized SysData
            if self.loading_state == LoadingState::Loaded {
                crate::utils::verify_build(true);
            }
        }

        // Check if we have a pending rebuild to perform
        if self.pending_rebuild {
            self.pending_rebuild = false;
            self.perform_rebuild();
            return; // Exit after rebuild (the process will be replaced)
        }
        
        // Check if commands were modified and refresh display if needed
        if self.popup_state.check_for_reload() {
            ctx.request_repaint();
        }

        // Poll external dialog subprocess (non-blocking)
        self.poll_external_dialog();

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
            detailed_log("POPUP_UPDATE", &viewport_info);
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

        // 🖱️ MOUSE MOTION THRESHOLD: Track mouse position and detect motion
        // Initialize lock position if not set (first frame)
        if self.popup_state.mouse_lock_pos.is_none() {
            let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
            if let Some(pos) = mouse_pos {
                self.popup_state.mouse_lock_pos = Some((pos.x, pos.y));
                self.popup_state.mouse_unlocked = false;
                detailed_log("MOUSE_LOCK", &format!("🖱️ Mouse locked at ({:.1}, {:.1})", pos.x, pos.y));
            }
        }

        // Check if mouse has moved beyond 20 pixel threshold
        if !self.popup_state.mouse_unlocked {
            let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
            if let (Some(current), Some(locked)) = (mouse_pos, self.popup_state.mouse_lock_pos) {
                let dx = current.x - locked.0;
                let dy = current.y - locked.1;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance > 20.0 {
                    self.popup_state.mouse_unlocked = true;
                    detailed_log("MOUSE_UNLOCK", &format!("🖱️ Mouse unlocked - moved {:.1} pixels from locked position", distance));
                }
            }
        }

        // Start deferred loading on second frame (after UI is shown)
        if self.frame_count == 2 && self.loading_state == LoadingState::NotLoaded {
            detailed_log("POPUP", &format!("POPUP: Starting deferred loading on frame 2"));
            self.start_deferred_loading();
            ctx.request_repaint(); // Ensure we update when loading completes
        }

        // Check if state.json has been modified (efficient polling via mod time)
        // This ensures last anchor changes from the server are picked up
        if self.loading_state == LoadingState::Loaded {
            let state_path = std::path::Path::new(&std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join(".config/hookanchor/state.json");

            if let Ok(metadata) = std::fs::metadata(&state_path) {
                if let Ok(mod_time) = metadata.modified() {
                    // Check if file has changed since last check
                    let should_reload = match self.last_state_mod_time {
                        None => true, // First time - always load
                        Some(last_time) => mod_time > last_time, // Only reload if file changed
                    };

                    if should_reload {
                        // File changed or first load - reload state from disk
                        let current_state = crate::core::data::get_state();
                        self.popup_state.app_state = current_state;
                        self.last_state_mod_time = Some(mod_time);
                    }
                }
            }
        }
        
        // If loading just completed and we have pending search, trigger it now
        if self.loading_state == LoadingState::Loaded && self.search_pending_after_load {
            detailed_log("POPUP", &format!("POPUP: Triggering pending search after load"));
            let current_search = self.popup_state.search_text.clone();
            if !current_search.trim().is_empty() {
                self.popup_state.update_search(current_search);
            }
            self.search_pending_after_load = false;
            ctx.request_repaint(); // Ensure UI updates with search results
        }

        // If loading just completed and we have an initial action to execute, trigger it now
        if self.loading_state == LoadingState::Loaded && self.initial_action_name.is_some() {
            let action_name = self.initial_action_name.take().unwrap();

            // Look up action from config (not a command!)
            let config = crate::core::get_config();
            if let Some(ref actions) = config.actions {
                if let Some(action) = actions.get(&action_name) {
                    log(&format!("Executing config action '{}' (type: {})", action_name, action.action_type()));

                    // Execute based on action type
                    match action.action_type() {
                        "template" => {
                            // Template actions like edit_selection
                            self.handle_template_create_named(&action_name);
                        },
                        "popup" => {
                            // Direct popup actions (exit, navigate, etc.)
                            if let Some(popup_action) = action.get_string("action") {
                                self.execute_popup_action(&popup_action);
                            } else {
                                log_error(&format!("Popup action '{}' missing 'action' field", action_name));
                            }
                        },
                        _ => {
                            // All other action types are JavaScript functions
                            // Call action_<type>() - e.g., action_type "clear_log" calls "action_clear_log()"
                            let function_name = format!("action_{}", action.action_type());
                            self.execute_js_action(&function_name);
                        }
                    }
                } else {
                    log_error(&format!("Action '{}' not found in config", action_name));
                }
            } else {
                log_error("No actions defined in config");
            }
        }

        // On the first few frames, ensure the window is properly activated and positioned
        if self.frame_count <= 3 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            ctx.request_repaint(); // Ensure continuous updates during initialization
            
            // Also ensure proper window positioning on startup
            if self.frame_count == 2 && !self.position_set {
                let config = crate::core::data::get_config();
                let width = config.popup_settings.get_default_window_width() as f32;
                let height = config.popup_settings.get_default_window_height() as f32;
                let window_size = egui::vec2(width, height);
                let position = center_on_main_display(ctx, window_size);
                log(&format!("[POSITION] Setting position on first frames to {:?} (reason: initial positioning)", position));
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(position));
                self.position_set = true;
            }
        }
        
        // Check if we need active updates (animations, countdowns, etc.)
        let has_input = ctx.input(|i| !i.events.is_empty() || i.pointer.any_down() || i.keys_down.len() > 0);
        let has_active_ui = self.command_editor.visible
            || self.dialog.is_visible()
            || self.grabber_countdown.is_some();

        // Update interaction time if there's user input
        if has_input {
            self.last_interaction_time = std::time::Instant::now();
        }

        // Check for idle timeout - only when visible and not showing dialogs/editor
        if !self.is_hidden && !self.command_editor.visible && !self.dialog.is_visible() {
            let config = crate::core::data::get_config();
            if let Some(timeout_seconds) = config.popup_settings.idle_timeout_seconds {
                let idle_duration = self.last_interaction_time.elapsed();
                if idle_duration.as_secs() >= timeout_seconds {
                    log(&format!("IDLE_TIMEOUT: Hiding popup after {} seconds of inactivity", timeout_seconds));
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
                log(&format!("[UPDATE_LOOP] Still running while hidden - Frame {}, is_hidden={}", 
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
        //         detailed_log("FOCUS", &format!("Frame {}: Window focused={}, input focus_set={}", 
        //             self.frame_count, has_focus, self.focus_set));
        //     }
        // }
        
        
        // Set position on first frame after window is created
        if !self.position_set {
            // Use a reasonable default window size for positioning - the actual size will be auto-calculated
            let config = crate::core::data::get_config();
            let width = config.popup_settings.get_default_window_width() as f32;
            let height = config.popup_settings.get_default_window_height() as f32;
            let window_size = egui::vec2(width, height);
            let pos = get_previous_window_location(ctx, window_size);
            log(&format!("[POSITION] Setting initial position to {:?} (reason: first-time window setup)", pos));
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
            detailed_log("GRAB", "Calling update_grabber_countdown from main update");
        }
        self.update_grabber_countdown(ctx);
        
        // Update window size based on current UI state
        self.update_window_size(ctx);
        
        // Draw custom rounded background with heavy shadow
        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(egui::LayerId::background());

        // Get window position for positioning external dialogs
        let window_position = ctx.input(|i| {
            // Try outer_rect first, fallback to calculating from inner_rect
            if let Some(outer_rect) = i.viewport().outer_rect {
                Some((outer_rect.left(), outer_rect.top(), screen_rect.width()))
            } else if let Some(inner_rect) = i.viewport().inner_rect {
                Some((inner_rect.left(), inner_rect.top(), screen_rect.width()))
            } else {
                None
            }
        });

        // Cache window position for external dialogs called outside update loop
        self.cached_window_position = window_position;

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
        let config = self.config().clone();
        // Debug log before calling command editor update
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
            use std::io::Write;
            let _ = writeln!(file, "🎯 POPUP: About to call command_editor.update()");
        }
        let editor_result = self.command_editor.update(ctx, &config);
        
        // Check for queued errors and display them
        if crate::utils::error::has_errors() {
            if let Some(error_message) = crate::utils::error::take_next_error() {
                crate::utils::error(&error_message);
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
                            
                            // Execute uninstall using the shell script
                            std::thread::spawn(|| {
                                // Find the uninstall script in the config directory
                                let config_dir = crate::core::get_config_dir();
                                let uninstall_script = config_dir.join("uninstall.sh");

                                if uninstall_script.exists() {
                                    match std::process::Command::new("bash")
                                        .arg(&uninstall_script)
                                        .spawn() {
                                        Ok(_) => {
                                            // Script launched successfully
                                            std::process::exit(0);
                                        },
                                        Err(_e) => {
                                            std::process::exit(1);
                                        }
                                    }
                                } else {
                                    // Fallback: try to find uninstall script in the same directory as the binary
                                    let current_exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("ha"));
                                    let exe_dir = current_exe.parent().unwrap_or_else(|| std::path::Path::new("."));
                                    let script_path = exe_dir.join("../scripts/uninstall.sh");

                                    if script_path.exists() {
                                        match std::process::Command::new("bash")
                                            .arg(&script_path)
                                            .spawn() {
                                            Ok(_) => std::process::exit(0),
                                            Err(_) => std::process::exit(1),
                                        }
                                    } else {
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

                // TILDE EXPANSION: Expand tilde in arg field after user confirms save
                // This allows user to see and edit tilde while editing, but saves expanded path
                new_command.arg = crate::utils::expand_tilde(&new_command.arg);

                // Check if this is a rename that might have side effects
                let orig_name = &original_command_name;
                log(&format!("RENAME_CHECK: orig_name='{}', new_name='{}', empty={}, command_to_delete={:?}", orig_name, new_command.command, orig_name.is_empty(), command_to_delete));
                
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
                        log(&format!("RENAME_DETECTED: '{}' -> '{}'", effective_old_name, new_command.command));
                        // Command name changed - check for rename side effects
                        let config = crate::core::data::get_config();
                        
                        // Check if any rename flags are enabled
                        let has_rename_options = config.popup_settings.rename_doc.unwrap_or(false) ||
                                               config.popup_settings.rename_folder.unwrap_or(false) ||
                                               config.popup_settings.rename_patch.unwrap_or(false) ||
                                               config.popup_settings.rename_prefix.unwrap_or(false);
                        
                        if has_rename_options {
                            // Perform dry-run to see what would be renamed
                            use crate::core::rename_associated_data;
                            let (sys_data, _) = crate::core::get_sys_data();
                            let mut patches = sys_data.patches;
                            let mut commands = crate::core::data::get_commands();
                            match rename_associated_data(
                                effective_old_name,
                                &new_command.command,
                                &new_command.arg,
                                &new_command.action,
                                &mut commands,
                                &mut patches,
                                &config,
                                true, // dry_run = true
                            ) {
                                Ok((updated_arg, mut actions)) => {
                                    log(&format!("RENAME_DRY_RUN: Found {} actions: {:?}", actions.len(), actions));

                                    // Check if this is an anchor file rename where filename matches folder name
                                    // Check if the command has the anchor flag ('A')
                                    log(&format!("FOLDER_CHECK: Checking for folder rename. action={}, arg={}, old_name={}, is_anchor={}",
                                        new_command.action, new_command.arg, effective_old_name, new_command.is_anchor()));

                                    if new_command.is_anchor() {
                                        use std::path::Path;
                                        let arg_path = Path::new(&new_command.arg);

                                        // Check if the file stem (without extension) matches the old command name
                                        if let Some(file_stem) = arg_path.file_stem() {
                                            if let Some(file_stem_str) = file_stem.to_str() {
                                                log(&format!("FOLDER_CHECK: file_stem='{}', comparing to old_name='{}', match={}",
                                                    file_stem_str, effective_old_name, file_stem_str == effective_old_name));
                                                if file_stem_str == effective_old_name {
                                                    // This is an anchor file - check if folder name also matches
                                                    if let Some(parent) = arg_path.parent() {
                                                        log(&format!("FOLDER_CHECK: parent path exists: {}", parent.display()));
                                                        if let Some(folder_name) = parent.file_name() {
                                                            if let Some(folder_name_str) = folder_name.to_str() {
                                                                log(&format!("FOLDER_CHECK: folder_name='{}', comparing to old_name='{}', match={}",
                                                                    folder_name_str, effective_old_name, folder_name_str == effective_old_name));
                                                                if folder_name_str == effective_old_name {
                                                                    // Folder name matches - rename the folder too
                                                                    use crate::core::command_ops::rename_folder;
                                                                    match rename_folder(
                                                                        parent.to_str().unwrap_or(""),
                                                                        &new_command.command,
                                                                        &mut commands,
                                                                        true, // dry_run = true
                                                                    ) {
                                                                        Ok(folder_actions) => {
                                                                            log(&format!("RENAME_FOLDER_DRY_RUN: Found {} folder actions", folder_actions.len()));
                                                                            actions.extend(folder_actions);
                                                                        }
                                                                        Err(e) => {
                                                                            log(&format!("RENAME_FOLDER_DRY_RUN: Error: {}", e));
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

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

                                        // Mark this as a rename operation so poll_external_dialog knows to call execute_rename_with_ui_update
                                        context.insert("operation".to_string(), "rename".to_string());

                                        // Close command editor NOW (before showing dialog) so window is stable for positioning
                                        self.close_command_editor();
                                        command_editor_just_closed = true;

                                        // Use window position from beginning of update() - this is the main popup's top-left corner
                                        // which doesn't move (size may change but position stays fixed)
                                        if let Some((x, y, w)) = window_position {
                                            log(&format!("RENAME_DIALOG_POS: Using popup position: x={}, y={}, w={}", x, y, w));
                                        }

                                        // Show confirmation dialog with actions (actions already have bullets)
                                        let action_list = actions.join("\n");
                                        let dialog_spec = vec![
                                            "=Confirm Rename".to_string(),
                                            format!("'Renaming \"{}\" to \"{}\"", effective_old_name, new_command.command),
                                            format!("^The following changes will be made:\n\n{}", action_list),
                                            "!OK".to_string(),
                                            "!Only Change CMD".to_string(),
                                            "!Cancel".to_string(),
                                        ];

                                        // Show EXTERNAL dialog (subprocess-based) with callback
                                        // The actual rename execution happens in poll_external_dialog via execute_rename_with_ui_update
                                        self.show_external_dialog(dialog_spec, context, Box::new(move |final_context| {
                                            let empty_string = String::new();
                                            let exit_button = final_context.get("exit").unwrap_or(&empty_string);
                                            if exit_button == "OK" {
                                                // User confirmed - actual execution happens in poll_external_dialog
                                                log("EXTERNAL_DIALOG: User confirmed rename");
                                                Ok(())
                                            } else {
                                                // Cancel - do nothing
                                                log("EXTERNAL_DIALOG: User cancelled rename");
                                                Ok(())
                                            }
                                        }), window_position);

                                        return; // Exit early to wait for user confirmation
                                    } else {
                                        // No side effects found by dry-run - proceed with direct rename
                                        log(&format!("RENAME_DRY_RUN: No side effects found, proceeding with normal save for '{}' -> '{}'", effective_old_name, new_command.command));
                                        // Update arg if it was changed by rename logic
                                        new_command.arg = updated_arg;
                                    }
                                }
                                Err(e) => {
                                    crate::utils::error(&format!("Error checking rename effects: {}", e));
                                }
                            }
                        }
                    }
                }
                
                // No rename side effects or user not using rename features - proceed with normal save

                // Clone the new command for template processing before saving it
                let saved_command = new_command.clone();

                // TEMPORARY TEST: Trigger different error types based on command name
                let test_result = if new_command.command == "TESTERROR" {
                    Err("Test error triggered - command named TESTERROR".into())
                } else if new_command.command == "FATALERROR" {
                    // Trigger fatal error (will exit app)
                    crate::utils::fatal_error2("Test fatal error - command named FATALERROR");
                } else {
                    // Use atomic save to prevent patch inference from running twice
                    log(&format!("SAVE: Using atomic save for command: '{}' (action: {}, arg: {})",
                        new_command.command, new_command.action, new_command.arg));
                    self.save_command_atomic(new_command, command_to_delete.as_deref())
                };

                match test_result {
                    Ok(_) => {
                        log(&format!("SAVE: Successfully saved command atomically"));

                        // Commands will be fetched fresh from singleton by update_search()

                        // Process template files if there was a pending template
                        if let (Some(template), Some(context)) = (
                            self.command_editor.pending_template.as_ref(),
                            self.command_editor.template_context.as_ref()
                        ) {
                            detailed_log("TEMPLATE", &format!("TEMPLATE: Processing template files after save"));
                            if let Err(e) = crate::core::template_creation::process_template_files(
                                template,
                                context,
                                &saved_command
                            ) {
                                crate::utils::error(&format!("Failed to create template files: {}", e));
                            } else {
                                detailed_log("TEMPLATE", &format!("TEMPLATE: Successfully created template files"));
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
                        log_error(&format!("SAVE_DEBUG: Failed to add command: {}", e));

                        // Show error using external dialog system
                        let dialog_spec = vec![
                            "=Error".to_string(),
                            format!("'Failed to add command: {}", e),
                            "!OK".to_string(),
                        ];
                        self.show_external_dialog(
                            dialog_spec,
                            HashMap::new(),
                            Box::new(|_| Ok(())),
                            self.cached_window_position
                        );

                        self.close_command_editor();
                        command_editor_just_closed = true;
                        return; // Exit early on error
                    }
                }
                
                // Clear the pending template now that it's been processed
                self.command_editor.pending_template = None;
                self.command_editor.template_context = None;
                
                // Update input field with the renamed command name for symmetric feedback
                // This ensures that after closing the editor, the input field shows the command that was just saved/renamed
                log(&format!("RENAME_FEEDBACK: Setting input field to renamed command: '{}'", saved_command.command));
                self.set_input(saved_command.command.clone());
                
                self.close_command_editor();
                command_editor_just_closed = true;
            }
            CommandEditorResult::Delete(command_name) => {
                // Delete the specified command (auto-saves via commandstore)
                use crate::core::delete_command;

                let deleted = delete_command(&command_name);
                if deleted.is_err() {
                    log_error(&format!("Command '{}' not found for deletion", command_name));
                } else {
                    // Command already saved by delete_command (via sys_data::delete_command)
                    // Commands will be fetched fresh from singleton by update_search()

                    // Update the filtered list if we're currently filtering
                    if !self.popup_state.search_text.trim().is_empty() {
                        // Refresh the search with updated commands
                        let current_search = self.popup_state.search_text.clone();
                        self.popup_state.update_search(current_search);
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
                // TODO: Consider refactoring this to use centralized key handling
                // Check for Enter key BEFORE creating the TextEdit widget
                let enter_pressed = ctx.input(|i| {
                    let pressed = i.key_pressed(egui::Key::Enter);
                    if pressed {
                        detailed_log("KEY_DEBUG", "🔑 ALTERNATE_PATH: Enter key pressed (direct check)");
                    }
                    pressed
                });

                // Check for Space key to accept last anchor
                let space_pressed = ctx.input(|i| {
                    i.key_pressed(egui::Key::Space)
                });

                // Handle space key last anchor acceptance
                if space_pressed {
                    log("🔑 SPACE KEY: Space key detected");
                    let current_input = if self.loading_state == LoadingState::Loaded {
                        &self.popup_state.search_text
                    } else {
                        &self.pre_init_input_buffer
                    };

                    // Check conditions: empty input + anchor name exists
                    let anchor_name = &self.popup_state.app_state.anchor_name;
                    log(&format!("🔑 SPACE KEY: current_input='{}' (empty={}), anchor_name={:?}",
                        current_input, current_input.is_empty(), anchor_name));

                    if current_input.is_empty() && anchor_name.is_some() {
                        if let Some(anchor_text) = anchor_name {
                            log(&format!("🔑 SPACE KEY: Accepting anchor name '{}' + space", anchor_text));

                            // Consume the space key FIRST to prevent TextEdit from processing it
                            ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Space));
                            log("🔑 SPACE KEY: Consumed space key before TextEdit");

                            // Set input to anchor text + space (we control both)
                            let new_text = format!("{} ", anchor_text);
                            if self.loading_state == LoadingState::Loaded {
                                self.popup_state.update_search(new_text.clone());
                                log(&format!("🔑 SPACE KEY: Set search_text to '{}' (len={})",
                                    self.popup_state.search_text, self.popup_state.search_text.len()));
                            } else {
                                self.pre_init_input_buffer = new_text.clone();
                                log(&format!("🔑 SPACE KEY: Set pre_init_buffer to '{}' (len={})",
                                    self.pre_init_input_buffer, self.pre_init_input_buffer.len()));
                            }

                            // Request cursor be positioned at the end of the text
                            self.request_cursor_at_end = true;
                            log("🔑 SPACE KEY: Requested cursor at end of input");
                        }
                    }
                }

                // Check for Left Arrow key - consume it when input is empty so TextEdit doesn't capture it
                let left_arrow_pressed = ctx.input(|i| {
                    i.key_pressed(egui::Key::ArrowLeft)
                });

                if left_arrow_pressed {
                    let current_input = if self.loading_state == LoadingState::Loaded {
                        &self.popup_state.search_text
                    } else {
                        &self.pre_init_input_buffer
                    };

                    // If input is empty, consume the key so TextEdit doesn't get it
                    // This allows the key registry to handle navigation
                    if current_input.is_empty() {
                        ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft));
                        detailed_log("KEY_DEBUG", "🔑 LEFT ARROW: Consumed before TextEdit (empty input)");
                    }
                }

                // Check for Left Bracket key - consume it when input is empty so TextEdit doesn't capture it
                let left_bracket_pressed = ctx.input(|i| {
                    i.key_pressed(egui::Key::OpenBracket)
                });

                if left_bracket_pressed {
                    let current_input = if self.loading_state == LoadingState::Loaded {
                        &self.popup_state.search_text
                    } else {
                        &self.pre_init_input_buffer
                    };

                    // If input is empty, consume the key so TextEdit doesn't get it
                    // This allows the key registry to handle navigation
                    if current_input.is_empty() {
                        ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::OpenBracket));
                        detailed_log("KEY_DEBUG", "🔑 LEFT BRACKET: Consumed before TextEdit (empty input)");
                    }
                }

                if enter_pressed {
                }

                let response = {
                    // Temporarily modify style for more rounded text input corners
                    let mut style = ui.style().as_ref().clone();
                    style.visuals.widgets.inactive.rounding = egui::Rounding::same(6.0); // Half the window corner radius
                    style.visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
                    style.visuals.widgets.active.rounding = egui::Rounding::same(6.0);
                    ui.set_style(style);
                    
                    // Get text length before creating TextEdit to avoid borrowing conflicts
                    let text_len = if self.loading_state == LoadingState::Loaded {
                        self.popup_state.search_text.len()
                    } else {
                        self.pre_init_input_buffer.len()
                    };

                    let mut text_edit = if self.loading_state == LoadingState::Loaded {
                        egui::TextEdit::singleline(&mut self.popup_state.search_text)
                            .desired_width(ui.available_width())
                            .hint_text(hint_text)
                            .font(font_id)
                    } else {
                        egui::TextEdit::singleline(&mut self.pre_init_input_buffer)
                            .desired_width(ui.available_width())
                            .hint_text(hint_text)
                            .font(font_id)
                    };

                    let text_edit_response = ui.add_enabled(
                        !self.command_editor.visible && self.grabber_countdown.is_none(),
                        text_edit
                    );

                    // Handle cursor positioning AFTER rendering the TextEdit
                    if self.request_cursor_at_end {
                        // Check if TextEdit added an unwanted space at the beginning
                        let actual_text = if self.loading_state == LoadingState::Loaded {
                            &self.popup_state.search_text
                        } else {
                            &self.pre_init_input_buffer
                        };

                        log(&format!("CURSOR_DEBUG: text_len={}, actual_text='{}' (len={})",
                            text_len, actual_text, actual_text.len()));

                        // If TextEdit added a space at the beginning despite our consumption, remove it
                        if actual_text.starts_with(' ') && actual_text.len() > text_len {
                            log("CURSOR_DEBUG: Detected unwanted leading space, removing it");
                            let fixed_text = actual_text.trim_start().to_string();
                            if self.loading_state == LoadingState::Loaded {
                                self.popup_state.search_text = fixed_text;
                            } else {
                                self.pre_init_input_buffer = fixed_text;
                            }
                        }

                        if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_response.id) {
                            // Manually set cursor to end position
                            let end_pos = egui::text::CCursor::new(text_len);
                            state.cursor.set_char_range(Some(egui::text::CCursorRange::two(end_pos, end_pos)));
                            state.store(ui.ctx(), text_edit_response.id);
                            log(&format!("CURSOR_DEBUG: Manually set cursor to position {}", text_len));
                        } else {
                            log(&format!("CURSOR_DEBUG: Could not load TextEdit state to set cursor"));
                        }

                        self.request_cursor_at_end = false;
                    }

                    // Check actual cursor position after our manual positioning
                    if let Some(state) = egui::TextEdit::load_state(ui.ctx(), text_edit_response.id) {
                        // TODO: Finish cursor debugging
                        // if let Some(cursor_range) = state.cursor.char_range() {
                        //     log(&format!("CURSOR_DEBUG: Final cursor range: {:?} (text length: {})", cursor_range, text_len));
                        // } else {
                        //     log(&format!("CURSOR_DEBUG: No cursor range available (text length: {})", text_len));
                        // }
                    } else {
                        log(&format!("CURSOR_DEBUG: No TextEdit state available (text length: {})", text_len));
                    }

                    text_edit_response
                };


                // Check if Enter was pressed (checked before TextEdit could consume it)
                if enter_pressed && response.has_focus() {
                    // Try to expand alias/command name first
                    let expanded = self.try_expand_input();

                    if expanded {
                        // Input was expanded - don't execute, just update filter
                        detailed_log("KEY_DEBUG", "🔑 Enter key expanded input text");
                        ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
                    } else if !self.filtered_commands().is_empty() {
                        // No expansion - execute selected command
                        self.execute_selected_command();
                        // TODO: Consider refactoring this to use centralized key handling
                        // Clear the Enter key from input to prevent double processing
                        detailed_log("KEY_DEBUG", "🔑 ALTERNATE_PATH: Consuming Enter key to prevent double processing");
                        ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
                    }
                } else if response.changed() {
                    // TODO: Consider refactoring this to use centralized key handling
                    detailed_log("KEY_DEBUG", "🔑 ALTERNATE_PATH: TextEdit changed (text input events consumed by widget)");
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

                        // Calculate total time from show command to focus ready
                        let total_time_msg = if let Some(start) = self.show_command_start {
                            format!(" (total time from show command: {:?})", start.elapsed())
                        } else {
                            String::new()
                        };

                        detailed_log("FOCUS_TIMING", &format!("Focus successfully set on frame {} - INPUT READY FOR USER{}",
                            self.frame_count, total_time_msg));
                        detailed_log("FOCUS", &format!("Focus successfully set on frame {}", self.frame_count));

                        // Clear the timer
                        self.show_command_start = None;
                    } else if self.frame_count % 5 == 0 && self.frame_count <= 15 {
                        // Log focus attempts every 5 frames for debugging
                        detailed_log("FOCUS", &format!("Focus attempt on frame {} - window focused: {}", 
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
                // Show commands if fully loaded and there are filtered commands (includes prefix menus from last anchor)
                if self.loading_state == LoadingState::Loaded && !self.filtered_commands().is_empty() {
                    // Use the display commands and layout that PopupState already computed
                    // Clone to avoid borrow conflicts in the rendering closure
                    let display_commands = self.popup_state.display_commands.clone();
                    let is_submenu = self.popup_state.is_in_prefix_menu;
                    let menu_prefix = self.popup_state.prefix_menu_info.as_ref().map(|(_, resolved)| resolved.command.clone());
                    let inside_count = self.popup_state.prefix_menu_count;
                    let actual_layout = &self.popup_state.display_layout;

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

                    let (window_width, required_height) = match &actual_layout.arrangement {
                        LayoutArrangement::MultiColumn { rows, cols } => {
                            let rows_per_col = *rows;
                            let cols_to_use = *cols;
                            
                            // Calculate actual column widths by measuring the text
                            let mut column_widths = vec![0.0f32; cols_to_use];

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
                                    let mut display_text = if is_submenu && !is_separator && i < inside_count {
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

                                    // Truncate if too long (but not separators)
                                    if !is_separator {
                                        display_text = self.truncate_command_name(display_text);
                                    }

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
                    };
                    
                    // Skip content-based window sizing when dialog is visible to avoid competing with dialog sizing
                    if !self.dialog.is_visible() {
                        // Determine the correct window size based on current mode
                        let (final_width, final_height) = if self.command_editor.visible {
                            // Use calculated editor size when command editor is open
                            let editor_size = self.calculate_editor_size();
                            (editor_size.x, editor_size.y)
                        } else {
                            (window_width, required_height)
                        };
                        
                        // Apply the calculated window size to ensure it fits the content
                        let config = crate::core::data::get_config();
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
                            detailed_log("CONTENT_RESIZE", &format!("Sending content-based resize command! {}x{} (dialog not visible)",
                                constrained_width, constrained_height));
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(constrained_width, constrained_height)));
                        }
                    } else {
                        detailed_log("CONTENT_RESIZE", "Dialog is visible, allowing dialog sizing to control window");
                    }

                    // Use the layout and display_commands that were already calculated above for window sizing
                    // This ensures window sizing and rendering use the exact same layout
                    detailed_log("RENDER", &format!("Using layout: {:?} (calculated from {} commands)",
                        &actual_layout.arrangement, display_commands.len()));
                    match &actual_layout.arrangement {
                        LayoutArrangement::MultiColumn { rows, cols } => {
                            // Multi-column display
                            detailed_log("RENDER", &format!("Using MultiColumn branch (rows={}, cols={})", rows, cols));
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
                                            if let Some((original_command, _resolved_command)) = self.popup_state.get_prefix_menu_command_info() {
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

                                        // Truncate if too long (but not separators)
                                        let display_text = if !is_separator {
                                            self.truncate_command_name(display_text)
                                        } else {
                                            display_text
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
                                                egui::RichText::new(&display_text).font(list_font_id.clone())
                                            };

                                            let response = ui.selectable_label(is_selected, text);

                                            // Update selection on hover - but only if mouse has moved beyond threshold
                                            // This prevents false hover detection when popup opens with mouse already under it
                                            if self.popup_state.mouse_unlocked && response.hovered() {
                                                let hover_pos = ctx.input(|i| i.pointer.hover_pos());
                                                // Validate pointer is actually inside button (not stale coordinates from different display)
                                                if hover_pos.map_or(false, |pos| response.rect.contains(pos)) {
                                                    if self.selected_index() != i {
                                                        self.set_selected_index(i);
                                                        ui.ctx().request_repaint();
                                                    }
                                                }
                                            }

                                            if response.clicked() {
                                                self.set_selected_index(i);

                                                // Use unified execution with state saving
                                                self.execute(&cmd);
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
                detailed_log("WINDOW_POS", &format!("First position detection: {:?}", current_pos));
                // Only save if this position was explicitly set by user, not auto-positioned
                if !self.was_auto_positioned {
                    save_window_position_with_reason(current_pos, "window closed by user");
                }
                self.last_saved_position = Some(current_pos);
            } else if let Some(last_pos) = self.last_saved_position {
                let moved = (current_pos.x - last_pos.x).abs() > 10.0 || (current_pos.y - last_pos.y).abs() > 10.0;
                if moved {
                    detailed_log("WINDOW_POS", &format!("Window moved from {:?} to {:?} (user moved)", last_pos, current_pos));
                    // Only save significant moves that are likely user-initiated
                    save_window_position_with_reason(current_pos, "user dragged window");
                    self.last_saved_position = Some(current_pos);
                    self.was_auto_positioned = false; // User moved it, so it's no longer auto-positioned
                }
            }
        } else {
            detailed_log("WINDOW_POS", "Could not get current position from viewport");
        }
        
    }
}


/// Wrapper that includes popup control socket
struct PopupWithControl {
    popup: AnchorSelector,
    control: crate::systems::popup_server::PopupControl,
}

impl PopupWithControl {
    fn new(initial_prompt: &str, initial_action: Option<&str>) -> Self {
        let control = crate::systems::popup_server::PopupControl::new();
        control.start_listener();

        Self {
            popup: AnchorSelector::new_with_prompt_and_action(initial_prompt, initial_action),
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
                    let show_start = std::time::Instant::now();
                    detailed_log("SHOW", "===== SHOW COMMAND RECEIVED =====");
                    detailed_log("SHOW_TIMING", &format!("Command received at {:?}", show_start));
                    detailed_log("SHOW", &format!(" Current frame: {}", self.popup.frame_count));
                    detailed_log("SHOW", &format!(" is_hidden={}, should_exit={}",
                        self.popup.is_hidden, self.popup.should_exit));

                    // Store the show start time for measuring total time to focus
                    self.popup.show_command_start = Some(show_start);
                    
                    // Get current window state
                    let viewport_info = ctx.input(|i| {
                        format!("Viewport: focused={}, visible={:?}, position={:?}",
                            i.focused,
                            i.viewport().inner_rect,
                            i.viewport().outer_rect
                        )
                    });
                    detailed_log("SHOW", &format!(" Before: {}", viewport_info));
                    
                    // Restore saved position BEFORE making window visible to avoid flashing at (0,0)
                    if let Some(saved_pos) = load_window_position() {
                        detailed_log("SHOW", &format!(" Pre-setting saved position: {:?}", saved_pos));
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(saved_pos));
                    }
                    
                    // Make the window visible again if it was hidden
                    detailed_log("SHOW", "Sending ViewportCommand::Visible(true)");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    
                    detailed_log("SHOW", "Sending ViewportCommand::Focus");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                    
                    // Also try to bring to front
                    detailed_log("SHOW", "Sending ViewportCommand::Fullscreen(false) to ensure windowed mode");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
                    
                    detailed_log("SHOW", "Sending ViewportCommand::Minimized(false)");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));

                    // REMOVED: AppleScript activation (was taking 176-368ms!)
                    // The ViewportCommand::Focus above should be sufficient for egui/eframe windows
                    // If focus issues occur, we may need a faster native solution
                    detailed_log("SHOW", "Relying on ViewportCommand::Focus (AppleScript removed for speed)");
                    
                    // CRITICAL: Position window on screen if it's off-screen
                    let current_pos = ctx.input(|i| i.viewport().outer_rect);
                    if let Some(rect) = current_pos {
                        // Get actual screen dimensions using CoreGraphics on macOS
                        let (screen_width, screen_height) = get_actual_screen_dimensions();
                        
                        detailed_log("WINDOW_POS", &format!("[SHOW] Actual screen dimensions: {}x{}", 
                            screen_width, screen_height));
                        detailed_log("WINDOW_POS", &format!("[SHOW] Current window rect: {:?}", rect));
                        
                        // Check if window is truly off-screen using actual screen dimensions
                        // Allow some margin (50 pixels) for window decorations
                        let margin = 50.0;
                        let is_offscreen = rect.min.x < -margin || 
                                          rect.min.y < -margin || 
                                          rect.min.x > screen_width - margin ||
                                          rect.min.y > screen_height - margin;
                        
                        detailed_log("WINDOW_POS", &format!("[SHOW] Off-screen check: x={} < {} OR y={} < {} OR x={} > {} OR y={} > {} = {}", 
                            rect.min.x, -margin, rect.min.y, -margin, 
                            rect.min.x, screen_width - margin, rect.min.y, screen_height - margin, is_offscreen));
                        
                        if is_offscreen {
                            detailed_log("SHOW", &format!(" Window is off-screen at {:?} (screen: {}x{}), centering on main display", 
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
                            detailed_log("SHOW", &format!(" Repositioned window to {:?} (auto-centered)", center_pos));
                        } else {
                            // Window is on-screen, restore saved position if different
                            if let Some(mut saved_pos) = load_window_position() {
                                // Ensure saved position is not too close to bottom
                                let window_size = rect.size();
                                let max_y = (screen_height - window_size.y - 100.0).max(0.0);
                                if saved_pos.y > max_y {
                                    detailed_log("SHOW", &format!(" Adjusting saved position from y={} to y={} (too close to bottom)", saved_pos.y, max_y));
                                    saved_pos.y = max_y;
                                    self.popup.was_auto_positioned = true; // Mark as auto-positioned since we adjusted it
                                }
                                
                                let pos_diff = (rect.min.x - saved_pos.x).abs() + (rect.min.y - saved_pos.y).abs();
                                detailed_log("SHOW", &format!(" Current position: {:?}, Saved position: {:?}, Diff: {}", rect.min, saved_pos, pos_diff));
                                if pos_diff > 5.0 {  // Only restore if significantly different
                                    detailed_log("SHOW", &format!(" Restoring saved position {:?} (current: {:?})", saved_pos, rect.min));
                                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(saved_pos));
                                    // If we didn't adjust the position, it's user-set, not auto-positioned
                                    if !self.popup.was_auto_positioned {
                                        self.popup.was_auto_positioned = false;
                                    }
                                } else {
                                    detailed_log("SHOW", "Position is close to saved, not restoring");
                                }
                            } else {
                                detailed_log("SHOW", "No saved position available");
                            }
                        }
                    } else {
                        // No position available, center the window
                        detailed_log("SHOW", "No window position available, centering on main display");
                        let config = crate::core::data::get_config();
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
                        detailed_log("SHOW", &format!(" Positioned new window to {:?}", center_pos));
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
                    detailed_log("SHOW", &format!(" Setting is_hidden=false (was {})", self.popup.is_hidden));
                    self.popup.is_hidden = false;
                    detailed_log("SHOW", &format!(" Setting should_exit=false (was {})", self.popup.should_exit));
                    self.popup.should_exit = false;  // Reset exit flag
                    self.popup.last_interaction_time = std::time::Instant::now();
                    
                    // Force a repaint
                    detailed_log("SHOW", "Requesting repaint");
                    ctx.request_repaint();

                    detailed_log("SHOW", &format!(" After: is_hidden={}, should_exit={}",
                        self.popup.is_hidden, self.popup.should_exit));
                    detailed_log("SHOW_TIMING", &format!("Command processing complete in {:?}", show_start.elapsed()));
                    detailed_log("SHOW", "===== SHOW COMMAND COMPLETE =====");
                }
                crate::systems::popup_server::PopupCommand::Hide => {
                    detailed_log("HIDE", "===== HIDE COMMAND RECEIVED =====");
                    // Call exit_or_hide to properly hide the window
                    self.popup.exit_or_hide(ctx);
                    detailed_log("HIDE", "===== HIDE COMMAND COMPLETE =====");
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

pub fn run_gui_with_prompt(initial_prompt: &str, initial_action: Option<&str>, _app_state: super::ApplicationState) -> Result<(), eframe::Error> {
    // Debug: Log when popup is being opened
    detailed_log("SYSTEM", &format!("===== POPUP GUI STARTING ====="));
    detailed_log("POPUP_OPEN", &format!("Opening popup with initial prompt: '{}', action: {:?}", initial_prompt, initial_action));

    // NOTE: Activation policy is set at process start in main.rs to ensure
    // popup_server is hidden from Cmd+Tab/Dock from the very beginning

    // Capture the prompt and action for the closure
    let prompt = initial_prompt.to_string();
    let action = initial_action.map(|s| s.to_string());
    
    // Manual window sizing - no auto-sizing constraints
    let config = crate::core::data::get_config();
    let width = config.popup_settings.get_default_window_width() as f32;
    let height = config.popup_settings.get_default_window_height() as f32;
    // Always start visible - we'll hide it later if needed
    let start_visible = true;
    
    detailed_log("SYSTEM", &format!("[POPUP_INIT] Window size: {}x{}, start_visible: {}", width, height, start_visible));
    
    // Load saved position or use centered position
    let initial_position = if let Some(saved_pos) = load_window_position() {
        detailed_log("WINDOW_POS", &format!("[STARTUP] Using saved window position: {:?}", saved_pos));
        [saved_pos.x, saved_pos.y]
    } else {
        // Center on screen
        let screen_width = 1920.0; // Default fallback
        let screen_height = 1080.0; // Default fallback
        let x = (screen_width - width) / 2.0;
        let y = (screen_height - height) / 2.0;
        detailed_log("WINDOW_POS", &format!("[STARTUP] No saved position, centering at: [{}, {}]", x, y));
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
    
    detailed_log("WINDOW_POS", &format!("[VIEWPORT] ViewportBuilder configured with position: {:?}", initial_position));
    
    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        renderer: eframe::Renderer::Glow,
        run_and_return: false,
        vsync: true, // Re-enable vsync - disabling it causes unlimited FPS
        ..Default::default()
    };

    // Set activation policy again right before running - in case winit changed it
    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSApplication, NSApplicationActivationPolicy};
        use cocoa::base::nil;

        unsafe {
            let app = NSApplication::sharedApplication(nil);
            app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
            detailed_log("STARTUP", "Re-applied Accessory activation policy before eframe::run_native");
        }
    }

    eframe::run_native(
        "Anchor Selector",
        options,
        Box::new(move |cc| {
            // Set light theme
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            
            // Configure for minimal CPU usage
            let mut style = (*cc.egui_ctx.style()).clone();
            style.animation_time = 0.0; // Disable animations

            // Disable hover visuals completely - we control selection explicitly with blue highlight
            // Copy inactive state to all interaction states to prevent any hover/click effects
            style.visuals.widgets.hovered = style.visuals.widgets.inactive.clone();
            style.visuals.widgets.active = style.visuals.widgets.inactive.clone();
            style.visuals.widgets.open = style.visuals.widgets.inactive.clone();

            // Also disable the background fill for all widget states to prevent gray highlighting
            style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.open.weak_bg_fill = egui::Color32::TRANSPARENT;

            style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.hovered.bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.active.bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.open.bg_fill = egui::Color32::TRANSPARENT;

            cc.egui_ctx.set_style(style);
            
            
            // Set transparent background for corner areas
            if let Some(gl) = cc.gl.as_ref() {
                use eframe::glow::HasContext as _;
                unsafe {
                    // Transparent background to allow rounded corners
                    gl.clear_color(0.0, 0.0, 0.0, 0.0);
                }
            }
            
            Ok(Box::new(PopupWithControl::new(&prompt, action.as_deref())))
        }),
    )
}
