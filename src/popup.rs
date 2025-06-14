use eframe::egui;
use std::process;
use std::time::Instant;
use crate::{Command, filter_commands, execute_command, load_commands};

pub struct AnchorSelector {
    commands: Vec<Command>,
    search_text: String,
    filtered_commands: Vec<Command>,
    selected_index: usize,
    startup_time: Instant,
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
        }
    }
    
    fn update_filter(&mut self) {
        if self.search_text.trim().is_empty() {
            self.filtered_commands.clear();
        } else {
            self.filtered_commands = filter_commands(&self.commands, &self.search_text, true);
            self.filtered_commands.truncate(10);  // Limit to 10 results like command-line
        }
        
        // Always reset selection to first item when filter changes
        self.selected_index = 0;
    }
}

impl eframe::App for AnchorSelector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Only check focus after 500ms to allow window to fully initialize
        if self.startup_time.elapsed().as_millis() > 500 {
            if !ctx.input(|i| i.focused) {
                std::process::exit(0);
            }
        }
        
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(18.0))
                    .fill(egui::Color32::from_gray(240)) // Light gray background
            )
            .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(18.0);
                
                // Search input with larger, bold font
                
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.search_text)
                        .desired_width(ui.available_width())
                        .hint_text("Type to search commands...")
                        .font(egui::TextStyle::Heading)
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
                
                ui.add_space(18.0);
                
                // Command list - left justified
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for (i, cmd) in self.filtered_commands.iter().enumerate() {
                            let is_selected = i == self.selected_index;
                            
                            // Left-justified selectable label
                            ui.horizontal(|ui| {
                                let response = ui.selectable_label(
                                    is_selected,
                                    &cmd.command
                                );
                                
                                if response.clicked() {
                                    self.selected_index = i;
                                    execute_command(&cmd.command);
                                    process::exit(0);
                                }
                            });
                        }
                    });
            });
        });
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([400.0, 300.0])
            .with_resizable(true),
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