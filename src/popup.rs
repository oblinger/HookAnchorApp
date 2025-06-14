use eframe::egui;
use std::process;
use std::time::Instant;
use std::path::Path;
use std::fs;
use anchor_selector::{Command, filter_commands, execute_command, load_commands};

pub struct AnchorSelector {
    commands: Vec<Command>,
    search_text: String,
    filtered_commands: Vec<Command>,
    selected_index: usize,
    startup_time: Instant,
    last_saved_position: Option<egui::Pos2>,
    position_set: bool,
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
            startup_time: Instant::now(),
            last_saved_position: None,
            position_set: false,
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

impl eframe::App for AnchorSelector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set position on first frame after window is created
        if !self.position_set {
            if let Some(pos) = load_window_position() {
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
                self.position_set = true;
            }
        }
        
        // DISABLED: Focus detection causing immediate exit
        // if self.startup_time.elapsed().as_millis() > 500 {
        //     if !ctx.input(|i| i.focused) {
        //         std::process::exit(0);
        //     }
        // }
        
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(18.0))
                    .fill(egui::Color32::from_gray(240)) // Light gray background
            )
            .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Top draggable area (1/8 inch = ~9 pixels)
                let top_drag = ui.allocate_response(
                    egui::Vec2::new(ui.available_width(), 9.0),
                    egui::Sense::drag()
                );
                if top_drag.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                
                // Search input with larger, bold font (50% bigger than heading)
                let mut font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                font_id.size *= 1.5; // Make 50% larger
                
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.search_text)
                        .desired_width(ui.available_width())
                        .hint_text("Type to search commands...")
                        .font(font_id)
                );
                
                if response.changed() {
                    self.update_filter();
                }
                
                // Handle keyboard input
                ctx.input(|i| {
                    if i.key_pressed(egui::Key::Escape) {
                        std::process::exit(0);
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
                    if i.key_pressed(egui::Key::Enter) {
                        if !self.filtered_commands.is_empty() && self.selected_index < self.filtered_commands.len() {
                            let selected_cmd = &self.filtered_commands[self.selected_index];
                            execute_command(&selected_cmd.command);
                            std::process::exit(0);
                        }
                    }
                });
                
                // Focus the text input on startup
                if self.search_text.is_empty() {
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
            Ok(Box::new(AnchorSelector::new()))
        }),
    )
}