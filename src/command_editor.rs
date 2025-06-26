use eframe::egui;
use anchor_selector::{Command, delete_command, add_command, save_commands_to_file, Config};

pub struct CommandEditor {
    pub visible: bool,
    #[allow(dead_code)]
    pub position: egui::Pos2,
    
    // Editable fields
    pub command: String,
    pub action: String,
    pub argument: String,
    pub group: String,
    pub priority: bool,
    
    // Track the original command for reference
    pub original_command: Option<Command>,
    pub original_command_name: String,
    
    // Track focus state
    focus_requested: bool,
    
    // Commands list for checking existence
    commands: Vec<Command>,
    
    // Track delete button visibility to detect changes
    delete_button_was_visible: bool,
}

impl CommandEditor {
    pub fn new() -> Self {
        Self {
            visible: false,
            position: egui::pos2(0.0, 0.0),
            command: String::new(),
            action: String::new(),
            argument: String::new(),
            group: String::new(),
            priority: false,
            original_command: None,
            original_command_name: String::new(),
            focus_requested: false,
            commands: Vec::new(),
            delete_button_was_visible: false,
        }
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
        self.original_command = None;
        self.original_command_name = String::new();
        self.focus_requested = false;
        self.delete_button_was_visible = false;
    }
    
    pub fn update_commands(&mut self, commands: &[Command]) {
        self.commands = commands.to_vec();
    }
    
    pub fn edit_command(&mut self, command_to_edit: Option<&Command>, search_text: &str) {
        self.visible = true;
        self.focus_requested = false; // Reset focus flag when opening dialog
        
        if let Some(cmd) = command_to_edit {
            // Populate with selected command data
            self.command = cmd.command.clone();
            self.action = cmd.action.clone();
            self.argument = cmd.arg.clone();
            self.group = cmd.group.clone();
            self.priority = false;
            self.original_command_name = cmd.command.clone();
            self.original_command = Some(cmd.clone());
            
        } else {
            // No command selected - populate with search text as command name and blank other fields
            self.command = search_text.to_string();
            self.action = String::new();
            self.argument = String::new();
            self.group = String::new();
            self.priority = false;
            self.original_command_name = String::new();
            self.original_command = None;
        }
    }
    
    pub fn update(&mut self, ctx: &egui::Context, config: &Config) -> CommandEditorResult {
        if !self.visible {
            return CommandEditorResult::None;
        }
        
        let mut result = CommandEditorResult::None;
        
        egui::Window::new("Command Editor")
            .fixed_pos([300.0, 200.0])
            .fixed_size([400.0, 300.0])
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                // Handle escape key
                if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                    result = CommandEditorResult::Cancel;
                }
                
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    
                    // Two-column layout with aligned labels and values
                    egui::Grid::new("command_editor_grid")
                        .num_columns(2)
                        .spacing([20.0, 8.0])
                        .show(ui, |ui| {
                            // Command row with optional delete button
                            ui.label("Command:");
                            
                            // Check if current command text matches an existing command
                            let command_exists = !self.command.is_empty() && 
                                self.commands.iter().any(|cmd| cmd.command == self.command);
                            
                            // Check if delete button visibility changed and request focus if so
                            let delete_button_visibility_changed = command_exists != self.delete_button_was_visible;
                            self.delete_button_was_visible = command_exists;
                            
                            if command_exists {
                                // When delete button exists, use horizontal layout with sized text field
                                ui.horizontal(|ui| {
                                    let available_width = ui.available_width();
                                    let command_width = available_width - 80.0; // Leave space for delete button
                                    
                                    let command_response = ui.add_sized(
                                        [command_width, 20.0],
                                        egui::TextEdit::singleline(&mut self.command)
                                    );
                                    
                                    // Request focus when dialog opens or when delete button visibility changes
                                    if !self.focus_requested || delete_button_visibility_changed {
                                        command_response.request_focus();
                                        self.focus_requested = true;
                                    }
                                    
                                    // Show delete button
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
                            } else {
                                // When no delete button, use regular text edit that matches other fields
                                let command_response = ui.text_edit_singleline(&mut self.command);
                                
                                // Request focus when dialog opens or when delete button visibility changes
                                if !self.focus_requested || delete_button_visibility_changed {
                                    command_response.request_focus();
                                    self.focus_requested = true;
                                }
                            }
                            ui.end_row();
                            
                            // Action row (dropdown)
                            ui.label("Action:");
                            egui::ComboBox::from_id_salt("action_combo")
                                .selected_text(&self.action)
                                .height(400.0) // Make dropdown tall enough to show all options
                                .show_ui(ui, |ui| {
                                    // Parse comma-separated actions from config or use defaults
                                    let actions = match &config.popup_settings.listed_actions {
                                        Some(actions_str) => {
                                            // Split by comma and trim whitespace
                                            actions_str
                                                .split(',')
                                                .map(|s| s.trim().to_string())
                                                .filter(|s| !s.is_empty())
                                                .collect()
                                        },
                                        None => vec![
                                            "app".to_string(),
                                            "url".to_string(),
                                            "folder".to_string(),
                                            "cmd".to_string(),
                                            "chrome".to_string(),
                                            "anchor".to_string(),
                                        ],
                                    };
                                    for action in &actions {
                                        ui.selectable_value(&mut self.action, action.clone(), action);
                                    }
                                });
                            ui.end_row();
                            
                            // Argument row
                            ui.label("Argument:");
                            ui.text_edit_singleline(&mut self.argument);
                            ui.end_row();
                            
                            // Group row
                            ui.label("Group:");
                            ui.text_edit_singleline(&mut self.group);
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
                                        group: self.group.clone(),
                                        command: self.command.clone(),
                                        action: self.action.clone(),
                                        arg: self.argument.clone(),
                                        full_line: self.format_command_line(),
                                    };
                                    result = CommandEditorResult::Save(new_command, self.original_command_name.clone());
                                }
                            }
                        );
                    });
                });
            });
        
        result
    }
    
    fn format_command_line(&self) -> String {
        let action_arg = if self.argument.is_empty() {
            self.action.clone()
        } else {
            format!("{} {}", self.action, self.argument)
        };
        
        if self.group.is_empty() {
            format!("{} : {}", self.command, action_arg)
        } else {
            format!("{} ! {} : {}", self.group, self.command, action_arg)
        }
    }
    
    pub fn save_command(&self, commands: &mut Vec<Command>) -> Result<(), String> {
        // Step 1: Delete the original command if it exists
        if !self.original_command_name.is_empty() {
            let deleted = delete_command(commands, &self.original_command_name);
            if !deleted {
                eprintln!("Warning: Original command '{}' not found for deletion", self.original_command_name);
            }
        }
        
        // Step 2: Create the new command
        let new_command = Command {
            group: self.group.clone(),
            command: self.command.clone(),
            action: self.action.clone(),
            arg: self.argument.clone(),
            full_line: self.format_command_line(),
        };
        
        // Step 3: Add the new command to the list
        add_command(commands, new_command);
        
        // Step 4: Save the updated command list back to commands.txt
        match save_commands_to_file(commands) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error saving commands to file: {}", e)),
        }
    }
    
    #[allow(dead_code)]
    pub fn delete_original_command(&self, commands: &mut Vec<Command>) -> Result<(), String> {
        // Only delete if there was an original command
        if !self.original_command_name.is_empty() {
            let deleted = delete_command(commands, &self.original_command_name);
            if !deleted {
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