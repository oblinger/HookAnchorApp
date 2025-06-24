use eframe::egui;
use std::process;
use std::path::Path;
use std::fs;
use anchor_selector::{Command, filter_commands, execute_command, load_commands};

mod command_editor;
use command_editor::{CommandEditor, CommandEditorResult};

pub struct AnchorSelector {
    commands: Vec<Command>,
    search_text: String,
    filtered_commands: Vec<Command>,
    selected_index: usize,
    last_saved_position: Option<egui::Pos2>,
    position_set: bool,
    command_editor: CommandEditor,
}

impl AnchorSelector {
    pub fn new() -> Self {
        let commands = load_commands();
        let filtered_commands = Vec::new(); // Start with empty list
        
        Self {
            commands,
            search_text: String::new(),
            filtered_commands,
            selected_index: 0,
            last_saved_position: None,
            position_set: false,
            command_editor: CommandEditor::new(),
        }
    }
    
    fn update_filter(&mut self) {
        if self.search_text.trim().is_empty() {
            self.filtered_commands.clear();
        } else {
            self.filtered_commands = filter_commands(&self.commands, &self.search_text, false);
            self.filtered_commands.truncate(10);  // Limit to 10 results like command-line
        }
        
        // Always reset selection to first item when filter changes
        self.selected_index = 0;
    }
    
}

fn get_position_file_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".anchor_selector_position")
}

fn save_window_position(pos: egui::Pos2) {
    let file_path = get_position_file_path();
    let content = format!("{},{}", pos.x, pos.y);
    let _ = fs::write(file_path, content);
}

fn load_window_position() -> Option<egui::Pos2> {
    let file_path = get_position_file_path();
    if let Ok(content) = fs::read_to_string(&file_path) {
        let parts: Vec<&str> = content.trim().split(',').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                return Some(egui::pos2(x, y));
            }
        }
    }
    None
}

fn get_previous_window_location(ctx: &egui::Context, window_size: egui::Vec2) -> egui::Pos2 {
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

fn is_position_visible(pos: egui::Pos2, window_size: egui::Vec2) -> bool {
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

fn center_on_main_display(ctx: &egui::Context, window_size: egui::Vec2) -> egui::Pos2 {
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
        
        // Set position on first frame after window is created
        if !self.position_set {
            let window_size = egui::vec2(500.0, 400.0); // Match the size from run_gui()
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
        
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(18.0))
                    .fill(egui::Color32::TRANSPARENT) // Transparent frame background
                    .shadow(egui::Shadow::NONE) // Remove shadow to avoid artifacts
            )
            .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Top draggable area (minimal padding to match input box side borders)
                let top_drag = ui.allocate_response(
                    egui::Vec2::new(ui.available_width(), 0.0),
                    egui::Sense::drag()
                );
                if top_drag.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                
                // Handle keyboard input BEFORE text field to ensure it's not consumed
                ctx.input(|i| {
                    if i.key_pressed(egui::Key::Escape) {
                        if self.command_editor.visible {
                            self.command_editor.hide();
                        } else {
                            std::process::exit(0);
                        }
                    }
                    if i.key_pressed(egui::Key::ArrowDown) {
                        if !self.filtered_commands.is_empty() {
                            self.selected_index = (self.selected_index + 1).min(self.filtered_commands.len() - 1);
                        }
                    }
                    if i.key_pressed(egui::Key::ArrowUp) {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                    }
                    if i.key_pressed(egui::Key::ArrowRight) {
                        let command_to_edit = if !self.filtered_commands.is_empty() && self.selected_index < self.filtered_commands.len() {
                            Some(&self.filtered_commands[self.selected_index])
                        } else {
                            None
                        };
                        self.command_editor.edit_command(command_to_edit, &self.search_text);
                    }
                    if i.key_pressed(egui::Key::Equals) || (i.modifiers.shift && i.key_pressed(egui::Key::Equals)) {
                        // = or + key (shift+=): open command editor with blank fields for new command
                        self.command_editor.edit_command(None, &self.search_text);
                    }
                    if i.key_pressed(egui::Key::Enter) {
                        if !self.filtered_commands.is_empty() && self.selected_index < self.filtered_commands.len() {
                            let selected_cmd = &self.filtered_commands[self.selected_index];
                            execute_command(&selected_cmd.command);
                            std::process::exit(0);
                        }
                    }
                });
                
                // Search input with larger, bold font (50% bigger than heading)
                let mut font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                font_id.size *= 1.5; // Make 50% larger
                
                let response = ui.add_enabled(
                    !self.command_editor.visible, // Disable when dialog is open
                    egui::TextEdit::singleline(&mut self.search_text)
                        .desired_width(ui.available_width())
                        .hint_text("Type to search commands...")
                        .font(font_id)
                );
                
                if response.changed() {
                    self.update_filter();
                }
                
                // Focus the text input on startup (only when dialog is not open)
                if self.search_text.is_empty() && !self.command_editor.visible {
                    response.request_focus();
                }
                
                
                // Draggable space between input and list
                let mid_drag = ui.allocate_response(
                    egui::Vec2::new(ui.available_width(), 18.0),
                    egui::Sense::drag()
                );
                if mid_drag.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                
                // Command list - left justified
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for (i, cmd) in self.filtered_commands.iter().enumerate() {
                            let is_selected = i == self.selected_index;
                            
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
                                
                                let response = ui.selectable_label(
                                    is_selected,
                                    egui::RichText::new(&cmd.command).font(list_font_id)
                                );
                                
                                if response.clicked() {
                                    self.selected_index = i;
                                    execute_command(&cmd.command);
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
                    });
                
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
        
        // Update command editor dialog
        self.command_editor.update_commands(&self.commands);
        let editor_result = self.command_editor.update(ctx);
        match editor_result {
            CommandEditorResult::Cancel => {
                self.command_editor.hide();
            }
            CommandEditorResult::Save(_new_command, _original_command_name) => {
                // Use the command editor's save method
                if let Err(e) = self.command_editor.save_command(&mut self.commands) {
                    eprintln!("Error saving command: {}", e);
                } else {
                    // Update the filtered list if we're currently filtering
                    if !self.search_text.trim().is_empty() {
                        self.update_filter();
                    }
                }
                self.command_editor.hide();
            }
            CommandEditorResult::Delete(command_name) => {
                // Delete the specified command and save to file
                use anchor_selector::{delete_command, save_commands_to_file};
                
                let deleted = delete_command(&mut self.commands, &command_name);
                if !deleted {
                    eprintln!("Warning: Command '{}' not found for deletion", command_name);
                } else {
                    // Save the updated command list back to spot_cmds.txt
                    if let Err(e) = save_commands_to_file(&self.commands) {
                        eprintln!("Error saving commands to file after deletion: {}", e);
                    } else {
                        // Update the filtered list if we're currently filtering
                        if !self.search_text.trim().is_empty() {
                            self.update_filter();
                        }
                    }
                }
                self.command_editor.hide();
            }
            CommandEditorResult::None => {
                // Continue normal operation
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    run_gui()
}

fn run_gui() -> Result<(), eframe::Error> {
    let viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size([500.0, 400.0])
        .with_min_inner_size([400.0, 300.0])
        .with_resizable(true)
        .with_decorations(false); // Remove title bar and window controls
        // .with_transparent(true); // DISABLED: May cause hanging
    
    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };
    
    eframe::run_native(
        "Anchor Selector",
        options,
        Box::new(|cc| {
            // Set light theme
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            
            // Set light grey background for corner areas
            if let Some(gl) = cc.gl.as_ref() {
                use eframe::glow::HasContext as _;
                unsafe {
                    // Light grey (200/255 = 0.78) with full opacity for corners
                    gl.clear_color(0.78, 0.78, 0.78, 1.0);
                }
            }
            
            Ok(Box::new(AnchorSelector::new()))
        }),
    )
}