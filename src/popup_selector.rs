use eframe::egui;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Command {
    group: String,
    command: String,
    action: String,
    arg: String,
    full_line: String,
}

struct AnchorSelector {
    commands: Vec<Command>,
    search_text: String,
    filtered_commands: Vec<Command>,
    selected_index: usize,
    startup_time: Instant,
}

impl AnchorSelector {
    fn new() -> Self {
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
        if self.search_text.is_empty() {
            self.filtered_commands.clear();
        } else {
            let search_lower = self.search_text.to_lowercase();
            
            // Separate prefix matches from other matches
            let mut prefix_matches: Vec<Command> = Vec::new();
            let mut other_matches: Vec<Command> = Vec::new();
            
            for cmd in &self.commands {
                let cmd_lower = cmd.command.to_lowercase();
                if cmd_lower.starts_with(&search_lower) {
                    prefix_matches.push(cmd.clone());
                } else if cmd_lower.contains(&search_lower) {
                    other_matches.push(cmd.clone());
                }
            }
            
            // Sort prefix matches by length (shortest first)
            prefix_matches.sort_by_key(|cmd| cmd.command.len());
            
            // Sort other matches by length (shortest first)
            other_matches.sort_by_key(|cmd| cmd.command.len());
            
            // Combine: prefix matches first, then other matches
            self.filtered_commands = prefix_matches;
            self.filtered_commands.extend(other_matches);
            
            // Limit to 10 results only when searching
            self.filtered_commands.truncate(10);
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
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                // Search input with larger, bold font and light gray background
                let mut style = (*ctx.style()).clone();
                style.visuals.extreme_bg_color = egui::Color32::from_gray(240);
                ctx.set_style(style);
                
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.search_text)
                        .desired_width(400.0)
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
                
                ui.add_space(10.0);
            });
            
            // Command list - left aligned with margin
            ui.horizontal(|ui| {
                ui.add_space(36.0); // About 1/2 inch margin (36 pixels)
                ui.vertical(|ui| {
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            for (i, cmd) in self.filtered_commands.iter().enumerate() {
                                let is_selected = i == self.selected_index;
                                
                                let response = ui.selectable_label(
                                    is_selected,
                                    &cmd.command
                                );
                                
                                if response.clicked() {
                                    self.selected_index = i;
                                    execute_command(&cmd.command);
                                    process::exit(0);
                                }
                            }
                        });
                });
            });
        });
    }
}

fn load_commands() -> Vec<Command> {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let file_path = Path::new(&home).join("ob/data/spot_cmds/spot_cmds.txt");
    
    let contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading commands file {}: {}", file_path.display(), e);
            process::exit(1);
        }
    };
    
    let commands: Vec<Command> = contents
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            
            // Parse format: GROUP ! COMMAND : ACTION ARG or COMMAND : ACTION ARG
            let (group, command, action_arg) = if line.contains('!') {
                let parts: Vec<&str> = line.splitn(2, '!').collect();
                if parts.len() != 2 {
                    return None;
                }
                let group = parts[0].trim().to_string();
                let rest = parts[1].trim();
                
                let cmd_parts: Vec<&str> = rest.splitn(2, ':').collect();
                if cmd_parts.len() != 2 {
                    return None;
                }
                (group, cmd_parts[0].trim().to_string(), cmd_parts[1].trim())
            } else {
                let cmd_parts: Vec<&str> = line.splitn(2, ':').collect();
                if cmd_parts.len() != 2 {
                    return None;
                }
                (String::new(), cmd_parts[0].trim().to_string(), cmd_parts[1].trim())
            };
            
            let action_parts: Vec<&str> = action_arg.splitn(2, char::is_whitespace).collect();
            let action = action_parts[0].to_string();
            let arg = if action_parts.len() > 1 {
                action_parts[1].to_string()
            } else {
                String::new()
            };
            
            Some(Command {
                group,
                command,
                action,
                arg,
                full_line: line.to_string(),
            })
        })
        .collect();
    
    commands
}

fn execute_command(command: &str) {
    let content = format!("execute {}\n", command);
    
    match std::fs::write("/tmp/cmd_file", content) {
        Ok(_) => {
            // Successfully wrote to file
        }
        Err(e) => {
            eprintln!("Error writing to /tmp/cmd_file: {}", e);
        }
    }
}

fn main() -> Result<(), eframe::Error> {
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
        Box::new(|_cc| Ok(Box::new(AnchorSelector::new()))),
    )
}
