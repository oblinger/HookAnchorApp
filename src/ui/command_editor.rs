use eframe::egui;
use crate::core::{Command};
use crate::core::commands::{FLAG_USER_EDITED, FLAG_ANCHOR};
use crate::core::Config;
use crate::core::template_creation::{Template, TemplateContext};
use crate::prelude::*;

pub struct CommandEditor {
    pub visible: bool,

    // Editable fields
    pub command: String,
    pub action: String,
    pub argument: String,
    pub patch: String,
    pub flags: String,
    pub parameters: String,  // KEY:=VALUE pairs like "template:=mytemplate priority:=high"
    pub priority: bool,
    pub is_anchor: bool,

    // Track the original command for reference
    pub original_command: Option<Command>,
    pub original_command_name: String,

    // Track focus state
    focus_requested: bool,

    // Track delete button visibility to detect changes

    // Store template for post-save processing
    pub(crate) pending_template: Option<Template>,
    pub(crate) template_context: Option<TemplateContext>,
}

impl CommandEditor {
    pub fn new() -> Self {
        Self {
            visible: false,
            command: String::new(),
            action: String::new(),
            argument: String::new(),
            patch: String::new(),
            flags: String::new(),
            parameters: String::new(),
            priority: false,
            is_anchor: false,
            original_command: None,
            original_command_name: String::new(),
            focus_requested: false,
            pending_template: None,
            template_context: None,
        }
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
        self.original_command = None;
        self.original_command_name = String::new();
        self.focus_requested = false;
        self.pending_template = None;
        self.template_context = None;
    }

    /// Set the pending template for post-save processing
    pub fn set_pending_template(&mut self, template: Template, context: TemplateContext) {
        self.pending_template = Some(template);
        self.template_context = Some(context);
    }
    
    pub fn edit_command(&mut self, command_to_edit: Option<&Command>, search_text: &str) {
        self.visible = true;
        self.focus_requested = false; // Reset focus flag when opening dialog
        
        // Clear any pending template (this is for normal editing, not template processing)
        self.pending_template = None;
        self.template_context = None;
        
        if let Some(cmd) = command_to_edit {
            // Populate with selected command data
            self.command = cmd.command.clone();
            self.action = cmd.action.clone();
            self.argument = cmd.arg.clone();
            self.patch = cmd.patch.clone();

            // Extract anchor state - is_anchor() checks the 'A' flag
            // Then remove 'A' flag from flags string so it doesn't show in the text field
            self.is_anchor = cmd.is_anchor();
            let mut temp_cmd = cmd.clone();
            temp_cmd.remove_flag(FLAG_ANCHOR);
            self.flags = temp_cmd.flags.clone();

            // Load parameters from other_params HashMap
            self.parameters = if let Some(ref params) = cmd.other_params {
                crate::utils::format_kv_pairs(params)
            } else {
                String::new()
            };

            self.priority = false;
            self.original_command_name = cmd.command.clone();
            self.original_command = Some(cmd.clone());

        } else {
            // No command selected - populate with search text as command name and blank other fields
            self.command = search_text.to_string();
            self.action = String::new();
            self.argument = String::new();
            self.patch = String::new();
            self.flags = String::new();
            self.parameters = String::new();
            self.priority = false;
            self.is_anchor = false;
            self.original_command_name = String::new();
            self.original_command = None;
        }
    }
    
    /// Create a new command with pre-populated data from a template
    pub fn create_new_command(&mut self, template_command: &Command, _search_text: &str) {
        self.visible = true;
        self.focus_requested = false;
        
        // Clear any pending template
        self.pending_template = None;
        self.template_context = None;
        
        // Populate with template data, but mark as new command (no original_command_name)
        self.command = template_command.command.clone();
        self.action = template_command.action.clone();
        self.argument = template_command.arg.clone();
        self.patch = template_command.patch.clone();

        // Extract anchor state - is_anchor() checks the 'A' flag
        // Then remove 'A' flag from flags string so it doesn't show in the text field
        self.is_anchor = template_command.is_anchor();
        let mut temp_cmd = template_command.clone();
        temp_cmd.remove_flag(FLAG_ANCHOR);
        self.flags = temp_cmd.flags.clone();

        // Load parameters from other_params HashMap
        self.parameters = if let Some(ref params) = template_command.other_params {
            crate::utils::format_kv_pairs(params)
        } else {
            String::new()
        };

        self.priority = false;

        // IMPORTANT: Mark as new command by leaving original_command_name empty
        self.original_command_name = String::new();
        self.original_command = None;
    }
    
    /// Open the command editor with a pre-filled command (for grabber functionality)
    pub fn open_with_command(&mut self, command: Command) {
        self.visible = true;
        self.focus_requested = true;

        // Fill in the fields from the provided command
        self.command = command.command.clone();
        self.action = command.action.clone();
        self.argument = command.arg.clone();
        self.patch = command.patch.clone();

        // Extract anchor state - is_anchor() checks the 'A' flag
        // Then remove 'A' flag from flags string so it doesn't show in the text field
        self.is_anchor = command.is_anchor();
        let mut temp_cmd = command.clone();
        temp_cmd.remove_flag(FLAG_ANCHOR);
        self.flags = temp_cmd.flags.clone();

        // Load parameters from other_params HashMap
        self.parameters = if let Some(ref params) = command.other_params {
            crate::utils::format_kv_pairs(params)
        } else {
            String::new()
        };

        self.priority = false; // Default to false

        // No original command since this is a new command
        self.original_command = None;
        self.original_command_name = String::new();
    }
    
    pub fn update(&mut self, ctx: &egui::Context, _config: &Config) -> CommandEditorResult {
        if !self.visible {
            return CommandEditorResult::None;
        }
        
        let mut result = CommandEditorResult::None;
        let mut enter_pressed = false;
        
        // Simple hardcoded key handling - just Escape and Enter
        // Check for Escape key to cancel
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            result = CommandEditorResult::Cancel;
            
            // IMPORTANT: Consume the key event to prevent it from being processed by the popup
            ctx.input_mut(|i| {
                i.consume_key(egui::Modifiers::NONE, egui::Key::Escape);
            });
        }
        
        // Check for Enter key press globally (will work when no text field has focus)
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            enter_pressed = true;
        }
        
        // Get the config to determine window width
        let config = crate::core::data::get_config();
        let window_width = config.popup_settings.get_default_window_width() as f32;

        egui::Window::new("Command Editor")
            .resizable(false)
            .collapsible(false)
            .fixed_size([window_width, 290.0])  // Increased height for Parameters row
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    
                    // Two-column layout with aligned labels and values
                    egui::Grid::new("command_editor_grid")
                        .num_columns(2)
                        .spacing([20.0, 8.0])
                        .show(ui, |ui| {
                            // Command row with optional delete button
                            ui.label("Command:");
                            
                            // Delete button should be enabled only when editing an existing command
                            // (i.e., original_command_name is not empty and still exists in the system)
                            // Fetch fresh commands from singleton for accurate existence check
                            let current_commands = crate::core::data::get_commands();
                            let can_delete = !self.original_command_name.is_empty() &&
                                current_commands.iter().any(|cmd| cmd.command == self.original_command_name);
                            
                            // Always use the same horizontal layout to prevent focus issues
                            ui.horizontal(|ui| {
                                let available_width = ui.available_width();
                                let command_width = (available_width - 80.0).max(100.0); // Leave space for delete button
                                
                                let command_response = ui.add_sized(
                                    [command_width, 20.0],
                                    egui::TextEdit::singleline(&mut self.command)
                                );
                                
                                // Check for Enter key in this text field
                                if command_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    enter_pressed = true;
                                }
                                
                                // Request focus only when dialog first opens
                                if !self.focus_requested {
                                    command_response.request_focus();
                                    self.focus_requested = true;
                                }
                                
                                // Always show delete button, but enable/disable based on whether we're editing an existing command
                                ui.add_enabled_ui(can_delete, |ui| {
                                    if ui.button("Delete").clicked() {
                                        // Use original command name for deletion, not current text
                                        let command_to_delete = if !self.original_command_name.is_empty() {
                                            self.original_command_name.clone()
                                        } else {
                                            self.command.clone()
                                        };
                                        result = CommandEditorResult::Delete(command_to_delete);
                                    }
                                });
                            });
                            ui.end_row();
                            
                            // Action row (dropdown)
                            ui.label("Action:");
                            let _combo_id = ui.id().with("action_combo");
                            egui::ComboBox::from_id_salt("action_combo")
                                .selected_text(&self.action)
                                .height(400.0) // Make dropdown tall enough to show all options
                                .show_ui(ui, |ui| {
                                    // Get the list of available actions
                                    let actions = super::helpers::get_listed_actions();
                                    for action in &actions {
                                        if ui.selectable_value(&mut self.action, action.clone(), action).clicked() {
                                            // Close the combo box dropdown by closing its popup
                                            ui.memory_mut(|mem| mem.close_popup());
                                        }
                                    }
                                });
                            ui.end_row();
                            
                            // Argument row
                            ui.label("Argument:");
                            let arg_response = ui.add_sized([500.0, 20.0], egui::TextEdit::singleline(&mut self.argument));
                            if arg_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                enter_pressed = true;
                            }
                            ui.end_row();
                            
                            // Patch row
                            ui.label("Patch:");
                            let patch_response = ui.add_sized([500.0, 20.0], egui::TextEdit::singleline(&mut self.patch));
                            if patch_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                enter_pressed = true;
                            }
                            ui.end_row();
                            
                            // Flags row
                            ui.label("Flags:");
                            let flags_response = ui.add_sized([500.0, 20.0], egui::TextEdit::singleline(&mut self.flags));
                            if flags_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                enter_pressed = true;
                            }
                            ui.end_row();

                            // Parameters row
                            ui.label("Parameters:");
                            let params_response = ui.add_sized([500.0, 20.0], egui::TextEdit::singleline(&mut self.parameters)
                                .hint_text("template:=mytemplate priority:=high"));
                            if params_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                enter_pressed = true;
                            }
                            ui.end_row();

                            // Anchor row
                            ui.label("Anchor:");
                            ui.checkbox(&mut self.is_anchor, "");
                            ui.end_row();
                        });
                    
                    ui.add_space(20.0);
                    
                    // Centered buttons and delete icon
                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(
                            [ui.available_width(), 30.0].into(),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                // Center Cancel and Save buttons only
                                ui.add_space((ui.available_width() - 140.0) / 2.0);
                                
                                if ui.button("Cancel").clicked() {
                                    result = CommandEditorResult::Cancel;
                                }
                                
                                ui.add_space(20.0);
                                
                                if ui.button("Save").clicked() {
                                    // Parse parameters from the text field
                                    let parsed_params = if !self.parameters.trim().is_empty() {
                                        let params_map = crate::utils::parse_kv_pairs(&self.parameters);
                                        if params_map.is_empty() {
                                            None
                                        } else {
                                            Some(params_map)
                                        }
                                    } else {
                                        None
                                    };

                                    // Create the new command
                                    let mut new_command = Command {
                                        patch: self.patch.clone(),
                                        command: self.command.clone(),
                                        action: self.action.clone(),
                                        arg: self.argument.clone(),
                                        flags: self.flags.clone(),
                                        other_params: parsed_params,
                                        last_update: 0,
                                        file_size: None,
                                    };

                                    // Apply anchor flag based on toggle state
                                    new_command.set_anchor(self.is_anchor);

                                    // Early validation for self-referential aliases to prevent crashes
                                    if new_command.action == "alias" && new_command.command == new_command.arg {
                                        // Show error immediately without trying to save
                                        // This prevents the system from getting into a bad state
                                        log_error(&format!("ERROR: Cannot create self-referential alias: '{}' cannot alias to itself", new_command.command));
                                        // Don't set result, keep editor open
                                    } else {
                                        result = CommandEditorResult::Save(new_command, self.original_command_name.clone());
                                    }
                                    
                                    // Consume any pending Enter key to prevent command execution
                                    ctx.input_mut(|i| {
                                        i.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
                                    });
                                }
                            }
                        );
                    });
                });
            });
        
        // Handle Enter key press from any text field or global context
        if enter_pressed && result == CommandEditorResult::None {
            // Parse parameters from the text field
            let parsed_params = if !self.parameters.trim().is_empty() {
                let params_map = crate::utils::parse_kv_pairs(&self.parameters);
                if params_map.is_empty() {
                    None
                } else {
                    Some(params_map)
                }
            } else {
                None
            };

            // Create the new command
            let mut new_command = Command {
                patch: self.patch.clone(),
                command: self.command.clone(),
                action: self.action.clone(),
                arg: self.argument.clone(),
                flags: self.flags.clone(),
                other_params: parsed_params,
                last_update: 0,
                file_size: None,
            };

            // Apply anchor flag based on toggle state
            new_command.set_anchor(self.is_anchor);

            // Early validation for self-referential aliases to prevent crashes
            if new_command.action == "alias" && new_command.command == new_command.arg {
                log_error(&format!("ERROR: Cannot create self-referential alias: '{}' cannot alias to itself", new_command.command));
                // Don't set result, keep editor open
            } else {
                result = CommandEditorResult::Save(new_command, self.original_command_name.clone());
            }
            
            // IMPORTANT: Consume the Enter key to prevent it from being processed by the popup
            ctx.input_mut(|i| {
                i.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
            });
        }
        
        result
    }
    
    pub fn prepare_save_command(&self) -> (Option<String>, Command) {
        // Parse parameters from the text field
        let parsed_params = if !self.parameters.trim().is_empty() {
            let params_map = crate::utils::parse_kv_pairs(&self.parameters);
            if params_map.is_empty() {
                None
            } else {
                Some(params_map)
            }
        } else {
            None
        };

        // Return the command to delete (if any) and the new command to add
        let mut new_command = Command {
            patch: self.patch.clone(),
            command: self.command.clone(),
            action: self.action.clone(),
            arg: self.argument.clone(),
            flags: self.flags.clone(),
            other_params: parsed_params,
            last_update: 0,
            file_size: None,
        };

        // Apply anchor flag based on checkbox state
        new_command.set_anchor(self.is_anchor);

        // Add 'U' flag ONLY when editing an existing command (not creating new)
        // This indicates user-edited and prevents removal during rescan
        // New commands from templates/grabber should not get U flag so scanner can manage them
        if !self.original_command_name.is_empty() {
            if !new_command.flags.contains(FLAG_USER_EDITED) {
                new_command.set_flag(FLAG_USER_EDITED, "");
            }
        }

        let command_to_delete = if !self.original_command_name.is_empty() {
            Some(self.original_command_name.clone())
        } else {
            None
        };

        (command_to_delete, new_command)
    }
    
}

#[derive(Debug, PartialEq)]
pub enum CommandEditorResult {
    None,
    Cancel,
    Save(Command, String), // (new_command, original_command_name)
    Delete(String), // original_command_name to delete
}
