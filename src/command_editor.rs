use eframe::egui;
use anchor_selector::Command;

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
        }
    }
    
    pub fn show_for_command(&mut self, cmd: &Command, main_window_pos: egui::Pos2) {
        println!("show_for_command called");
        self.visible = true;
        println!("set visible to true");
        
        // SIMPLIFIED: Just set position, skip all data copying for now
        self.position = egui::pos2(
            main_window_pos.x + 50.0,  // Offset right
            main_window_pos.y - 20.0   // Offset up a little
        );
        println!("position set");
        
        // Skip all string operations for now
        // self.command = cmd.command.clone();
        // self.action = cmd.action.clone();
        // self.argument = cmd.arg.clone();
        // self.group = cmd.group.clone();
        // self.priority = false;
        // self.original_command = Some(cmd.clone());
        
        println!("show_for_command completed");
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
        self.original_command = None;
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
                            ui.text_edit_singleline(&mut self.command);
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
                                ui.add_space((ui.available_width() - 140.0) / 2.0); // Center the button group
                                
                                if ui.button("Cancel").clicked() {
                                    result = CommandEditorResult::Cancel;
                                }
                                
                                ui.add_space(20.0);
                                
                                if ui.button("Save").clicked() {
                                    result = CommandEditorResult::Save;
                                }
                            }
                        );
                    });
                });
            });
        
        result
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandEditorResult {
    None,
    Cancel,
    Save,
}