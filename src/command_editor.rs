use eframe::egui;
use anchor_selector::{Command, delete_command, add_command, save_commands_to_file};

pub struct CommandEditor {
    pub visible: bool,
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
        }
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
        self.original_command = None;
        self.original_command_name = String::new();
    }
    
    pub fn edit_command(&mut self, command_to_edit: Option<&Command>, search_text: &str) {
        self.visible = true;
        
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
    
    pub fn update(&mut self, ctx: &egui::Context) -> CommandEditorResult {
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
                            // Command row
                            ui.label("Command:");
                            let command_response = ui.text_edit_singleline(&mut self.command);
                            // Request focus on the command field when dialog opens
                            command_response.request_focus();
                            ui.end_row();
                            
                            // Action row (dropdown)
                            ui.label("Action:");
                            egui::ComboBox::from_id_salt("action_combo")
                                .selected_text(&self.action)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.action, "pass".to_string(), "pass");
                                    ui.selectable_value(&mut self.action, "alias".to_string(), "alias");
                                    ui.selectable_value(&mut self.action, "anchor".to_string(), "anchor");
                                    ui.selectable_value(&mut self.action, "app".to_string(), "app");
                                    ui.selectable_value(&mut self.action, "cmd".to_string(), "cmd");
                                    ui.selectable_value(&mut self.action, "folder".to_string(), "folder");
                                    ui.selectable_value(&mut self.action, "doc".to_string(), "doc");
                                    ui.selectable_value(&mut self.action, "notion".to_string(), "notion");
                                    ui.selectable_value(&mut self.action, "obs".to_string(), "obs");
                                    ui.selectable_value(&mut self.action, "obs_url".to_string(), "obs_url");
                                    ui.selectable_value(&mut self.action, "chrome".to_string(), "chrome");
                                    ui.selectable_value(&mut self.action, "safari".to_string(), "safari");
                                    ui.selectable_value(&mut self.action, "brave".to_string(), "brave");
                                    ui.selectable_value(&mut self.action, "firefox".to_string(), "firefox");
                                    ui.selectable_value(&mut self.action, "url".to_string(), "url");
                                    ui.selectable_value(&mut self.action, "work".to_string(), "work");
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
                    
                    // Centered buttons
                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(
                            [ui.available_width(), 30.0].into(),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                // Calculate centering based on whether delete button will be shown
                                let button_group_width = if self.original_command_name.is_empty() {
                                    140.0 // Cancel + Save
                                } else {
                                    180.0 // Cancel + Save + Delete
                                };
                                ui.add_space((ui.available_width() - button_group_width) / 2.0);
                                
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
                                
                                // Only show delete button if editing an existing command
                                if !self.original_command_name.is_empty() {
                                    ui.add_space(10.0);
                                    
                                    if ui.button("🗑️").clicked() {
                                        result = CommandEditorResult::Delete(self.original_command_name.clone());
                                    }
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
        
        // Step 4: Save the updated command list back to spot_cmds.txt
        match save_commands_to_file(commands) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error saving commands to file: {}", e)),
        }
    }
    
    pub fn delete_original_command(&self, commands: &mut Vec<Command>) -> Result<(), String> {
        // Only delete if there was an original command
        if !self.original_command_name.is_empty() {
            let deleted = delete_command(commands, &self.original_command_name);
            if !deleted {
                return Err(format!("Command '{}' not found for deletion", self.original_command_name));
            }
            
            // Save the updated command list back to spot_cmds.txt
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