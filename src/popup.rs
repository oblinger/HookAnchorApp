use eframe::egui;
use std::process;
use std::path::Path;
use std::fs;
use anchor_selector::{Command, filter_commands, execute_command, load_commands, get_submenu_display_positions, get_submenu_prefix, load_config, Config};

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
    config: Config,
}

impl AnchorSelector {
    pub fn new() -> Self {
        let commands = load_commands();
        let filtered_commands = Vec::new(); // Start with empty list
        let config = load_config();
        
        
        Self {
            commands,
            search_text: String::new(),
            filtered_commands,
            selected_index: 0,
            last_saved_position: None,
            position_set: false,
            command_editor: CommandEditor::new(),
            config,
        }
    }
    
    fn update_filter(&mut self) {
        if self.search_text.trim().is_empty() {
            self.filtered_commands.clear();
        } else {
            let total_limit = self.config.popup_settings.max_rows * self.config.popup_settings.max_columns;
            self.filtered_commands = filter_commands(&self.commands, &self.search_text, total_limit, false);
        }
        
        // Always reset selection to first item when filter changes
        self.selected_index = 0;
    }
    
    // Calculate if we should use multi-column layout
    fn should_use_columns(&self) -> bool {
        self.filtered_commands.len() > self.config.popup_settings.max_rows && self.config.popup_settings.max_columns > 1
    }
    
    // Calculate column layout (rows per column, number of columns to use)
    fn get_column_layout(&self) -> (usize, usize) {
        if !self.should_use_columns() {
            return (self.filtered_commands.len(), 1);
        }
        
        let max_rows = self.config.popup_settings.max_rows;
        let max_cols = self.config.popup_settings.max_columns;
        let total_items = self.filtered_commands.len();
        
        // Calculate optimal number of columns needed
        let cols_needed = (total_items + max_rows - 1) / max_rows; // Ceiling division
        let cols_to_use = cols_needed.min(max_cols);
        
        // Calculate rows per column
        let rows_per_col = (total_items + cols_to_use - 1) / cols_to_use; // Ceiling division
        
        (rows_per_col, cols_to_use)
    }
    
    // Convert linear index to (column, row) coordinates
    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        let (rows_per_col, _) = self.get_column_layout();
        let col = index / rows_per_col;
        let row = index % rows_per_col;
        (col, row)
    }
    
    // Convert (column, row) coordinates to linear index
    fn coords_to_index(&self, col: usize, row: usize) -> usize {
        let (rows_per_col, _) = self.get_column_layout();
        col * rows_per_col + row
    }
    
    // Navigate up/down in the multi-column layout
    fn navigate_vertical(&mut self, direction: i32) {
        if self.filtered_commands.is_empty() {
            return;
        }
        
        let (rows_per_col, cols_to_use) = self.get_column_layout();
        let (current_col, current_row) = self.index_to_coords(self.selected_index);
        
        let new_row = if direction > 0 {
            // Down: move to next row, wrapping to next column if needed
            if current_row + 1 < rows_per_col {
                // Stay in same column, move down
                let new_index = self.coords_to_index(current_col, current_row + 1);
                if new_index < self.filtered_commands.len() {
                    current_row + 1
                } else {
                    current_row // Don't move if target doesn't exist
                }
            } else if current_col + 1 < cols_to_use {
                // Move to top of next column
                0
            } else {
                current_row // Stay at bottom of last column
            }
        } else {
            // Up: move to previous row, wrapping to previous column if needed
            if current_row > 0 {
                current_row - 1
            } else if current_col > 0 {
                // Move to bottom of previous column
                let prev_col_size = if current_col == cols_to_use - 1 && self.filtered_commands.len() % rows_per_col != 0 {
                    // Last column might be shorter
                    (self.filtered_commands.len() - 1) % rows_per_col
                } else {
                    rows_per_col - 1
                };
                prev_col_size
            } else {
                current_row // Stay at top of first column
            }
        };
        
        let new_col = if direction > 0 && current_row + 1 >= rows_per_col && current_col + 1 < cols_to_use {
            current_col + 1
        } else if direction < 0 && current_row == 0 && current_col > 0 {
            current_col - 1
        } else {
            current_col
        };
        
        let new_index = self.coords_to_index(new_col, new_row);
        if new_index < self.filtered_commands.len() {
            self.selected_index = new_index;
        }
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
            // Use a reasonable default window size for positioning - the actual size will be auto-calculated
            let window_size = egui::vec2(500.0, 300.0);
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
                        self.navigate_vertical(1);
                    }
                    if i.key_pressed(egui::Key::ArrowUp) {
                        self.navigate_vertical(-1);
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
                
                // Command list - check for submenu and display accordingly  
                // No scroll area - window will size to accommodate max_rows
                if !self.filtered_commands.is_empty() {
                    // Calculate required window dimensions
                    let row_height = 28.0; // Slightly larger to account for font size and spacing
                    let input_height = 60.0; // Height for search input (accounting for larger font)
                    let padding = 50.0; // Top and bottom margins (more generous)
                    
                    let (window_width, required_height) = if self.should_use_columns() {
                        let (rows_per_col, cols_to_use) = self.get_column_layout();
                        let column_width = 250.0; // Width per column
                        let total_width = (cols_to_use as f32 * column_width) + 50.0; // Add some padding
                        let total_height = input_height + padding + (rows_per_col as f32 * row_height);
                        (total_width, total_height)
                    } else {
                        let window_width = 500.0;
                        let required_height = input_height + padding + (self.filtered_commands.len() as f32 * row_height);
                        (window_width, required_height)
                    };
                    
                    // Resize window to accommodate content
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(window_width, required_height)));
                    
                    let submenu_positions = get_submenu_display_positions(&self.filtered_commands, &self.search_text);
                    
                    if self.should_use_columns() {
                        // Multi-column display
                        let (rows_per_col, cols_to_use) = self.get_column_layout();
                        
                        // Show submenu header if applicable (even in multi-column mode)
                        if !submenu_positions.is_empty() {
                            if let Some(prefix) = get_submenu_prefix(&self.filtered_commands, &self.search_text) {
                                let mut header_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                                header_font_id.size *= 1.3;
                                
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(format!("{} ->", prefix)).font(header_font_id));
                                });
                                
                                ui.add_space(8.0);
                            }
                        }
                        
                        ui.horizontal(|ui| {
                            for col in 0..cols_to_use {
                                ui.vertical(|ui| {
                                    for row in 0..rows_per_col {
                                        let i = col * rows_per_col + row;
                                        if i >= self.filtered_commands.len() {
                                            break;
                                        }
                                        
                                        let cmd = &self.filtered_commands[i];
                                        let is_selected = i == self.selected_index;
                                        
                                        // Determine display text based on submenu positions
                                        let display_text = if !submenu_positions.is_empty() && i < submenu_positions.len() {
                                            let pos = submenu_positions[i];
                                            if pos == 0 {
                                                cmd.command.clone()
                                            } else if pos < cmd.command.len() {
                                                cmd.command[pos..].to_string()
                                            } else {
                                                cmd.command.clone()
                                            }
                                        } else {
                                            cmd.command.clone()
                                        };
                                        
                                        // Use larger font for command list (50% bigger than body)
                                        let mut list_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                                        list_font_id.size *= 1.5; // Make 50% larger
                                        
                                        let response = ui.selectable_label(
                                            is_selected,
                                            egui::RichText::new(&display_text).font(list_font_id)
                                        );
                                        
                                        if response.clicked() {
                                            self.selected_index = i;
                                            execute_command(&cmd.command);
                                            process::exit(0);
                                        }
                                    }
                                });
                                
                                // Add space between columns (except after last column)
                                if col < cols_to_use - 1 {
                                    ui.add_space(10.0);
                                }
                            }
                        });
                    } else if submenu_positions.is_empty() {
                        // Single-column regular display - show full command names
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
                    } else {
                        // Single-column submenu display
                        
                        // First show the submenu header (prefix + arrow) - left aligned with input
                        if let Some(prefix) = get_submenu_prefix(&self.filtered_commands, &self.search_text) {
                            // Submenu header with larger font and left alignment
                            let mut header_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                            header_font_id.size *= 1.3; // Make larger than regular commands
                            
                            ui.horizontal(|ui| {
                                // No left margin for header - align with input box
                                ui.label(egui::RichText::new(format!("{} ->", prefix)).font(header_font_id));
                            });
                            
                            ui.add_space(8.0); // Small gap between header and commands
                        }
                        
                        // Then show the command suffixes with indentation
                        for (i, (cmd, &pos)) in self.filtered_commands.iter().zip(submenu_positions.iter()).enumerate() {
                            let is_selected = i == self.selected_index;
                            
                            // Determine display text based on position
                            let display_text = if pos == 0 {
                                // Show full command name
                                cmd.command.clone()
                            } else {
                                // Show suffix starting from position
                                if pos < cmd.command.len() {
                                    cmd.command[pos..].to_string()
                                } else {
                                    cmd.command.clone()
                                }
                            };
                            
                            // Indented selectable label with draggable margins
                            ui.horizontal(|ui| {
                                // Left margin draggable area - larger for indentation
                                let left_drag = ui.allocate_response(
                                    egui::Vec2::new(20.0, ui.text_style_height(&egui::TextStyle::Body)), // Increased from 10.0 to 20.0 for indentation
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
                                    egui::RichText::new(&display_text).font(list_font_id)
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
                    }
                } else {
                    // No commands - resize to minimal height (just input field)
                    let input_height = 60.0;
                    let padding = 50.0;
                    let base_height = input_height + padding;
                    let window_width = 500.0;
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(window_width, base_height)));
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
    // Manual window sizing - no auto-sizing constraints
    let viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size([500.0, 120.0]) // Initial size - will be dynamically resized
        .with_resizable(false) // Disable manual resizing - we control size programmatically
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