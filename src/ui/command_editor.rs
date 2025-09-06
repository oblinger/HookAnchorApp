use eframe::egui;
use crate::core::{Command};
use crate::core::delete_command;
use crate::core::commands::save_commands_to_file;
use crate::core::Config;
use crate::core::template_creation::{Template, TemplateContext};

pub struct CommandEditor {
    pub visible: bool,
    #[allow(dead_code)]
    pub position: egui::Pos2,
    
    // Editable fields
    pub command: String,
    pub action: String,
    pub argument: String,
    pub patch: String,
    pub flags: String,
    pub priority: bool,
    
    // Track the original command for reference
    pub original_command: Option<Command>,
    pub original_command_name: String,
    
    // Track focus state
    focus_requested: bool,
    
    // Commands list for checking existence
    commands: Vec<Command>,
    
    // Track delete button visibility to detect changes
    
    // Store template for post-save processing
    pub(crate) pending_template: Option<Template>,
    pub(crate) template_context: Option<TemplateContext>,
}

impl CommandEditor {
    pub fn new() -> Self {
        Self {
            visible: false,
            position: egui::pos2(0.0, 0.0),
            command: String::new(),
            action: String::new(),
            argument: String::new(),
            patch: String::new(),
            flags: String::new(),
            priority: false,
            original_command: None,
            original_command_name: String::new(),
            focus_requested: false,
            commands: Vec::new(),
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
    
    pub fn update_commands(&mut self, commands: &[Command]) {
        self.commands = commands.to_vec();
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
            self.flags = cmd.flags.clone();
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
            self.priority = false;
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
        self.flags = template_command.flags.clone();
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
        self.flags = command.flags.clone();
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
        
        // Render as a large modal-style window that fills more space
        egui::Window::new("Command Editor")
            .default_size([420.0, 320.0])
            .resizable(false)
            .collapsible(false)
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
                            let can_delete = !self.original_command_name.is_empty() && 
                                self.commands.iter().any(|cmd| cmd.command == self.original_command_name);
                            
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
                            egui::ComboBox::from_id_salt("action_combo")
                                .selected_text(&self.action)
                                .height(400.0) // Make dropdown tall enough to show all options
                                .show_ui(ui, |ui| {
                                    // Get the list of available actions
                                    let actions = super::helpers::get_listed_actions();
                                    for action in &actions {
                                        if ui.selectable_value(&mut self.action, action.clone(), action).clicked() {
                                            // Selection was made, close the combo box by losing focus
                                            ui.memory_mut(|mem| mem.stop_text_input());
                                        }
                                    }
                                });
                            ui.end_row();
                            
                            // Argument row
                            ui.label("Argument:");
                            let arg_response = ui.text_edit_singleline(&mut self.argument);
                            if arg_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                enter_pressed = true;
                            }
                            ui.end_row();
                            
                            // Patch row
                            ui.label("Patch:");
                            let patch_response = ui.text_edit_singleline(&mut self.patch);
                            if patch_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                enter_pressed = true;
                            }
                            ui.end_row();
                            
                            // Flags row
                            ui.label("Flags:");
                            let flags_response = ui.text_edit_singleline(&mut self.flags);
                            if flags_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                enter_pressed = true;
                            }
                            ui.end_row();
                            
                            // Priority row
                            ui.label("Priority:");
                            ui.checkbox(&mut self.priority, "");
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
                                    // Create the new command
                                    let new_command = Command {
                                        patch: self.patch.clone(),
                                        command: self.command.clone(),
                                        action: self.action.clone(),
                                        arg: self.argument.clone(),
                                        flags: self.flags.clone(),
                                    };
                                    result = CommandEditorResult::Save(new_command, self.original_command_name.clone());
                                    
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
            // Create the new command
            let new_command = Command {
                patch: self.patch.clone(),
                command: self.command.clone(),
                action: self.action.clone(),
                arg: self.argument.clone(),
                flags: self.flags.clone(),
            };
            result = CommandEditorResult::Save(new_command, self.original_command_name.clone());
            
            // IMPORTANT: Consume the Enter key to prevent it from being processed by the popup
            ctx.input_mut(|i| {
                i.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
            });
        }
        
        result
    }
    
    pub fn prepare_save_command(&self) -> (Option<String>, Command) {
        // ALWAYS add 'U' flag when a command is edited in the command editor
        // This indicates user-edited and prevents removal during rescan
        let mut flags = self.flags.clone();
        if !flags.contains('U') {
            if !flags.is_empty() {
                flags.push(' ');
            }
            flags.push('U');
        }
        
        // Return the command to delete (if any) and the new command to add
        let new_command = Command {
            patch: self.patch.clone(),
            command: self.command.clone(),
            action: self.action.clone(),
            arg: self.argument.clone(),
            flags,
        };
        
        let command_to_delete = if !self.original_command_name.is_empty() {
            Some(self.original_command_name.clone())
        } else {
            None
        };
        
        (command_to_delete, new_command)
    }
    
    #[allow(dead_code)]
    pub fn delete_original_command(&self, commands: &mut Vec<Command>) -> Result<(), String> {
        // Only delete if there was an original command
        if !self.original_command_name.is_empty() {
            let deleted = delete_command(&self.original_command_name, commands);
            if deleted.is_err() {
                return Err(format!("Command '{}' not found for deletion", self.original_command_name));
            }
            
            // Save the updated command list back to commands.txt
            match save_commands_to_file(commands) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error saving commands to file after deletion: {}", e)),
            }
        } else {
            // Nothing to delete (this was a new command creation dialog)
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandEditorResult {
    None,
    Cancel,
    Save(Command, String), // (new_command, original_command_name)
    Delete(String), // original_command_name to delete
}