use eframe::egui;
use std::process;
use anchor_selector::{
    Command, filter_commands, execute_command, load_commands, merge_similar_commands, 
    load_config, Config, split_commands, get_current_submenu_prefix,
    load_state, save_state, AppState, scanner
};
use chrono::Local;

mod command_editor;
use command_editor::{CommandEditor, CommandEditorResult};

mod dialog;
use dialog::Dialog;

/// Main application state for the Anchor Selector popup window
pub struct AnchorSelector {
    /// All available commands loaded from commands.txt
    commands: Vec<Command>,
    /// Current search text entered by the user
    search_text: String,
    /// Commands that match the current search (filtered and potentially merged)
    filtered_commands: Vec<Command>,
    /// Index of currently selected command
    /// INVARIANT: This always indexes into display_commands (from get_display_commands()),
    /// NOT into filtered_commands or commands
    selected_index: usize,
    /// Last saved window position for change detection
    last_saved_position: Option<egui::Pos2>,
    /// Whether window position has been set on startup
    position_set: bool,
    /// Command editor dialog state
    command_editor: CommandEditor,
    /// Dialog system for user input
    dialog: Dialog,
    /// Application configuration
    config: Config,
    /// Application state (window position, build time, etc.)
    state: AppState,
    /// Flag to disable automatic window resizing when we want manual control
    manual_resize_mode: bool,
    /// Track the last manual resize request to avoid repeatedly sending commands
    last_manual_size: Option<egui::Vec2>,
}

impl AnchorSelector {
    // =============================================================================
    // Initialization
    // =============================================================================
    
    pub fn new() -> Self {
        let commands = load_commands();
        
        // Perform startup scan check
        let commands = scanner::startup_check(commands);
        
        let filtered_commands = Vec::new(); // Start with empty list
        let config = load_config();
        let state = load_state();
        
        
        Self {
            commands,
            search_text: String::new(),
            filtered_commands,
            selected_index: 0,
            last_saved_position: None,
            position_set: false,
            command_editor: CommandEditor::new(),
            dialog: Dialog::new(),
            config,
            state,
            manual_resize_mode: false,
            last_manual_size: None,
        }
    }
    
    // =============================================================================
    // Build Time and Version Info
    // =============================================================================
    
    /// Calculate seconds since build time
    fn get_seconds_since_build(&self) -> Option<i64> {
        if let Some(build_time) = self.state.build_time {
            let current_time = Local::now().timestamp();
            Some(current_time - build_time)
        } else {
            None
        }
    }
    
    /// Generate hint text with build version info (only for first 10 minutes)
    fn get_hint_text(&self) -> String {
        if let Some(seconds) = self.get_seconds_since_build() {
            // Only show build time info for first 10 minutes (600 seconds)
            if seconds <= 600 {
                format!("Type to search commands... {}s", seconds)
            } else {
                "Type to search commands...".to_string()
            }
        } else {
            "Type to search commands...".to_string()
        }
    }
    
    // =============================================================================
    // Command Filtering and Management
    // =============================================================================
    
    /// Check if the current search text exactly matches an alias command
    /// If so, replace the search text with the alias's argument
    fn check_and_apply_alias(&mut self) {
        // Look for an exact match with an alias command
        if let Some(alias_cmd) = self.commands.iter().find(|cmd| 
            cmd.action == "alias" && cmd.command.to_lowercase() == self.search_text.to_lowercase()
        ) {
            // Replace search text with the alias argument
            self.search_text = alias_cmd.arg.clone();
        }
    }
    
    fn update_filter(&mut self) {
        if self.search_text.trim().is_empty() {
            self.filtered_commands.clear();
        } else {
            let total_limit = self.config.popup_settings.max_rows * self.config.popup_settings.max_columns;
            let mut filtered = filter_commands(&self.commands, &self.search_text, total_limit * 2, false); // Get more to account for merging
            
            // Apply merge_similar only if enabled, and use split-before-merge logic
            if self.config.popup_settings.merge_similar {
                // Always check if we're in submenu mode and split before merging
                if let Some(_menu_prefix) = get_current_submenu_prefix(&self.search_text) {
                    // Get submenu commands using split_commands
                    let submenu_commands = split_commands(&filtered, &self.search_text, &self.config.popup_settings.word_separators);
                    
                    // For simplicity, just use the split commands as the filtered result
                    let inside_menu = submenu_commands.clone();
                    let outside_menu: Vec<Command> = Vec::new();
                    
                    // Merge each list separately
                    let merged_inside = merge_similar_commands(inside_menu, &self.config);
                    let merged_outside = merge_similar_commands(outside_menu, &self.config);
                    
                    // Combine the results
                    filtered = merged_inside;
                    filtered.extend(merged_outside);
                } else {
                    // Not in submenu mode, merge normally
                    filtered = merge_similar_commands(filtered, &self.config);
                }
            }
            
            // Truncate to display limit after merging
            filtered.truncate(total_limit);
            
            self.filtered_commands = filtered;
        }
        
        // Always reset selection to first item when filter changes
        self.selected_index = 0;
    }
    
    // =============================================================================
    // Layout and Display Logic
    // =============================================================================
    
    // Calculate if we should use multi-column layout based on display commands
    fn should_use_columns_for_display(&self, display_commands: &[Command]) -> bool {
        display_commands.len() > self.config.popup_settings.max_rows && self.config.popup_settings.max_columns > 1
    }
    
    // Calculate column layout for the actual display commands
    fn get_column_layout_for_display(&self, display_commands: &[Command]) -> (usize, usize) {
        if !self.should_use_columns_for_display(display_commands) {
            return (display_commands.len(), 1);
        }
        
        let max_rows = self.config.popup_settings.max_rows;
        let max_cols = self.config.popup_settings.max_columns;
        let total_items = display_commands.len();
        
        // Calculate optimal number of columns needed
        let cols_needed = (total_items + max_rows - 1) / max_rows; // Ceiling division
        let cols_to_use = cols_needed.min(max_cols);
        
        // Calculate rows per column
        let rows_per_col = (total_items + cols_to_use - 1) / cols_to_use; // Ceiling division
        
        (rows_per_col, cols_to_use)
    }
    
    // Legacy functions for compatibility
    fn should_use_columns(&self) -> bool {
        self.filtered_commands.len() > self.config.popup_settings.max_rows && self.config.popup_settings.max_columns > 1
    }
    
    
    // Check if a command is a separator (for skipping during navigation)
    fn is_separator_command(cmd: &Command) -> bool {
        cmd.command == "---" && cmd.action == "separator"
    }
    
    /// Compute the commands to display based on current menu state
    /// Returns (commands_to_display, is_in_submenu, menu_prefix, inside_count)
    fn get_display_commands(&self) -> (Vec<Command>, bool, Option<String>, usize) {
        if let Some(menu_prefix) = get_current_submenu_prefix(&self.search_text) {
            // Split commands to determine submenu display
            let display_commands = split_commands(&self.filtered_commands, &self.search_text, &self.config.popup_settings.word_separators);
            
            // Find the separator to count inside commands
            let separator_index = display_commands.iter().position(|cmd| Self::is_separator_command(cmd));
            let inside_count = separator_index.unwrap_or(display_commands.len());
            
            (display_commands, true, Some(menu_prefix), inside_count)
        } else {
            // Not in submenu mode, use filtered commands directly
            (self.filtered_commands.clone(), false, None, 0)
        }
    }
    
    // =============================================================================
    // Navigation Logic
    // =============================================================================
    
    // Navigate left/right in the multi-column layout
    fn navigate_horizontal(&mut self, direction: i32) {
        let (display_commands, _, _, _) = self.get_display_commands();
        if display_commands.is_empty() {
            return;
        }
        
        // Check if we should use columns for these display commands
        if !self.should_use_columns_for_display(&display_commands) {
            return; // No horizontal navigation in single column mode
        }
        
        let (rows_per_col, cols_to_use) = self.get_column_layout_for_display(&display_commands);
        
        // Get current visual position based on actual display commands
        let current_visual_col = self.selected_index / rows_per_col;
        let current_visual_row = self.selected_index % rows_per_col;
        
        // Calculate new visual column
        let new_visual_col = if direction > 0 {
            // Moving right
            if current_visual_col + 1 < cols_to_use {
                current_visual_col + 1
            } else {
                return; // Already at rightmost column
            }
        } else {
            // Moving left
            if current_visual_col > 0 {
                current_visual_col - 1
            } else {
                return; // Already at leftmost column
            }
        };
        
        // Calculate new index maintaining the same visual row
        let new_index = new_visual_col * rows_per_col + current_visual_row;
        
        // Make sure we're not going past the end of the list
        if new_index >= display_commands.len() {
            return; // Can't move to non-existent item
        }
        
        // Skip separator if we land on one
        if Self::is_separator_command(&display_commands[new_index]) {
            return; // Don't move to separator
        }
        
        self.selected_index = new_index;
    }
    
    fn navigate_vertical(&mut self, direction: i32) {
        let (display_commands, _, _, _) = self.get_display_commands();
        if display_commands.is_empty() {
            return;
        }
        
        let max_index = display_commands.len() - 1;
        let new_index;
        
        if self.should_use_columns_for_display(&display_commands) {
            // Multi-column navigation
            let (rows_per_col, cols_to_use) = self.get_column_layout_for_display(&display_commands);
            
            // Get current visual position
            let current_col = self.selected_index / rows_per_col;
            let current_row = self.selected_index % rows_per_col;
            
            if direction > 0 {
                // Moving down
                if current_row + 1 < rows_per_col {
                    // Stay in same column, move down one row
                    let test_index = current_col * rows_per_col + (current_row + 1);
                    if test_index < display_commands.len() {
                        new_index = test_index;
                    } else {
                        return; // Can't move down, at bottom of column
                    }
                } else if current_col + 1 < cols_to_use {
                    // Move to top of next column
                    new_index = (current_col + 1) * rows_per_col;
                    if new_index >= display_commands.len() {
                        return; // Next column doesn't exist
                    }
                } else {
                    return; // Already at bottom of last column
                }
            } else {
                // Moving up
                if current_row > 0 {
                    // Stay in same column, move up one row
                    new_index = current_col * rows_per_col + (current_row - 1);
                } else if current_col > 0 {
                    // Move to bottom of previous column
                    let prev_col_end = current_col * rows_per_col - 1;
                    new_index = prev_col_end.min(max_index);
                } else {
                    return; // Already at top of first column
                }
            }
        } else {
            // Single column navigation
            if direction > 0 {
                if self.selected_index < max_index {
                    new_index = self.selected_index + 1;
                } else {
                    return; // Already at last item
                }
            } else {
                if self.selected_index > 0 {
                    new_index = self.selected_index - 1;
                } else {
                    return; // Already at first item
                }
            }
        }
        
        // Skip separator when navigating
        if Self::is_separator_command(&display_commands[new_index]) {
            if direction > 0 && new_index + 1 <= max_index {
                self.selected_index = new_index + 1;
            } else if direction < 0 && new_index > 0 {
                self.selected_index = new_index - 1;
            } else {
                return; // Can't skip separator
            }
        } else {
            self.selected_index = new_index;
        }
    }
    
}

fn save_window_position(pos: egui::Pos2) {
    let mut state = load_state();
    state.window_position = Some((pos.x, pos.y));
    let _ = save_state(&state);
}

fn load_window_position() -> Option<egui::Pos2> {
    let state = load_state();
    state.window_position.map(|(x, y)| egui::pos2(x, y))
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
        
        // Update command editor dialog BEFORE the main UI so it renders as a top-level window
        self.command_editor.update_commands(&self.commands);
        let editor_result = self.command_editor.update(ctx, &self.config);
        let mut editor_handled_escape = false;
        
        // Update dialog system
        if self.dialog.update(ctx) {
            if let Some(_result) = self.dialog.take_result() {
                // Resize back to normal when dialog closes and re-enable automatic resizing
                self.manual_resize_mode = false;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([500.0, 300.0].into()));
            }
        }
        // Track if command editor was just closed to restore focus
        let mut command_editor_just_closed = false;
        
        match editor_result {
            CommandEditorResult::Cancel => {
                self.command_editor.hide();
                editor_handled_escape = true;
                command_editor_just_closed = true;
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
                command_editor_just_closed = true;
            }
            CommandEditorResult::Delete(command_name) => {
                // Delete the specified command and save to file
                use anchor_selector::{delete_command, save_commands_to_file};
                
                let deleted = delete_command(&command_name, &mut self.commands);
                if deleted.is_err() {
                    eprintln!("Warning: Command '{}' not found for deletion", command_name);
                } else {
                    // Save the updated command list back to commands.txt
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
                command_editor_just_closed = true;
            }
            CommandEditorResult::None => {
                // Continue normal operation
            }
        }
        
        // FIRST: Handle navigation keys and filter them out to prevent TextEdit from seeing them
        // This must happen before any widgets are created
        ctx.input_mut(|input| {
            // Handle all four arrow keys for navigation
            if input.key_pressed(egui::Key::ArrowUp) {
                self.navigate_vertical(-1);
            }
            if input.key_pressed(egui::Key::ArrowDown) {
                self.navigate_vertical(1);
            }
            if input.key_pressed(egui::Key::ArrowLeft) {
                self.navigate_horizontal(-1);
            }
            if input.key_pressed(egui::Key::ArrowRight) {
                self.navigate_horizontal(1);
            }
            
            // Remove ALL arrow key events completely so TextEdit never sees them
            input.events.retain(|event| {
                !matches!(event, 
                    egui::Event::Key { key: egui::Key::ArrowUp, .. } |
                    egui::Event::Key { key: egui::Key::ArrowDown, .. } |
                    egui::Event::Key { key: egui::Key::ArrowLeft, .. } |
                    egui::Event::Key { key: egui::Key::ArrowRight, .. }
                )
            });
            
            // Also clear the pressed state for all arrow keys
            input.keys_down.remove(&egui::Key::ArrowUp);
            input.keys_down.remove(&egui::Key::ArrowDown);
            input.keys_down.remove(&egui::Key::ArrowLeft);
            input.keys_down.remove(&egui::Key::ArrowRight);
        });
        
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
                
                // Handle keyboard input - process some keys first
                ctx.input(|i| {
                    if i.key_pressed(egui::Key::Escape) {
                        if !editor_handled_escape {
                            std::process::exit(0);
                        }
                    }
                    if i.key_pressed(egui::Key::Enter) {
                        if !self.filtered_commands.is_empty() {
                            // Get display commands for execution
                            let (display_commands, _is_submenu, _menu_prefix, _inside_count) = self.get_display_commands();
                            
                            if self.selected_index < display_commands.len() {
                                let selected_cmd = &display_commands[self.selected_index];
                                
                                // Don't execute if it's a separator
                                if !Self::is_separator_command(selected_cmd) {
                                    // For merged commands (ending with "..."), use the action and arg directly
                                    // instead of looking up by command name
                                    if selected_cmd.command.ends_with("...") {
                                        // Execute using the stored action and arg from the merged command
                                        let launcher_command = if selected_cmd.arg.is_empty() {
                                            selected_cmd.action.clone()
                                        } else {
                                            format!("{} {}", selected_cmd.action, selected_cmd.arg)
                                        };
                                        
                                        // Use the same execution logic as execute_command
                                        let config = load_config();
                                        if config.popup_settings.use_new_launcher {
                                            use anchor_selector::launcher::launch;
                                            if let Err(e) = launch(&launcher_command) {
                                                eprintln!("Error executing command with new launcher: {:?}", e);
                                                std::process::exit(1);
                                            }
                                        } else {
                                            // Use old temp file method for backward compatibility
                                            use std::fs;
                                            let content = format!("execute {}\n", launcher_command);
                                            if let Err(e) = fs::write("/tmp/cmd_file", content) {
                                                eprintln!("Error writing to /tmp/cmd_file: {}", e);
                                                std::process::exit(1);
                                            }
                                        }
                                    } else {
                                        // For non-merged commands, use the existing lookup method
                                        execute_command(&selected_cmd);
                                    }
                                    std::process::exit(0);
                                }
                            }
                        }
                    }
                    if i.key_pressed(egui::Key::Equals) || (i.modifiers.shift && i.key_pressed(egui::Key::Equals)) {
                        // Command Editor: = or + key (shift+=): open command editor
                        // Check if there's an exact match (case-insensitive) for the search text
                        let exact_match = self.commands.iter().find(|cmd| 
                            cmd.command.to_lowercase() == self.search_text.to_lowercase()
                        );
                        
                        if let Some(matching_command) = exact_match {
                            // Found exact match - edit the existing command
                            self.command_editor.edit_command(Some(matching_command), &self.search_text);
                        } else {
                            // No exact match - create new command with search text as name
                            self.command_editor.edit_command(None, &self.search_text);
                        }
                    }
                });
                
                // Search input with larger, bold font (50% bigger than heading)
                let mut font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                font_id.size *= 1.5; // Make 50% larger
                
                // Calculate hint text before mutable borrow
                let hint_text = self.get_hint_text();
                
                let response = ui.add_enabled(
                    !self.command_editor.visible, // Disable when dialog is open
                    egui::TextEdit::singleline(&mut self.search_text)
                        .desired_width(ui.available_width())
                        .hint_text(hint_text)
                        .font(font_id)
                );
                
                if response.changed() {
                    // Check if user typed slash to open command editor
                    if self.search_text.ends_with('/') {
                        // Remove the slash from search text
                        self.search_text.pop();
                        
                        // Open command editor for selected command
                        let (display_commands, _, _, _) = self.get_display_commands();
                        let command_to_edit = if !display_commands.is_empty() && self.selected_index < display_commands.len() {
                            let cmd = &display_commands[self.selected_index];
                            // Don't edit separators
                            if !Self::is_separator_command(cmd) {
                                Some(cmd)
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        self.command_editor.edit_command(command_to_edit, &self.search_text);
                    } else {
                        // First check if the text matches an alias and replace if so
                        self.check_and_apply_alias();
                        self.update_filter();
                    }
                }
                
                // Focus the text input on startup or when command editor closes
                if (self.search_text.is_empty() && !self.command_editor.visible) || command_editor_just_closed {
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
                    // Get the display commands using our new method
                    let (display_commands, is_submenu, menu_prefix, inside_count) = self.get_display_commands();
                    
                    // Calculate required window dimensions based on final display commands
                    // Use actual font metrics for precise sizing
                    let mut input_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                    input_font_id.size *= 1.5; // Same scaling as used for input
                    let input_height = ui.fonts(|f| f.row_height(&input_font_id)) + 20.0; // Add padding for input box
                    
                    let mut list_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                    list_font_id.size *= 1.5; // Same scaling as used for command list
                    let row_height = ui.fonts(|f| f.row_height(&list_font_id)) + 4.0; // Add more spacing between rows
                    
                    let header_height = if is_submenu {
                        let mut header_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                        header_font_id.size *= 1.3; // Same scaling as used for header
                        ui.fonts(|f| f.row_height(&header_font_id)) + 8.0 // Add the gap after header
                    } else {
                        0.0
                    };
                    
                    let padding = 60.0; // Top and bottom margins - increased for safety
                    let bottom_drag_height = 20.0; // Height of bottom draggable area
                    let mid_drag_height = 18.0; // Height between input and list
                    
                    let (window_width, required_height) = if self.should_use_columns() {
                        // Recalculate layout based on actual display command count
                        let total_items = display_commands.len();
                        let max_rows = self.config.popup_settings.max_rows;
                        let max_cols = self.config.popup_settings.max_columns;
                        let cols_needed = (total_items + max_rows - 1) / max_rows;
                        let cols_to_use = cols_needed.min(max_cols);
                        let rows_per_col = (total_items + cols_to_use - 1) / cols_to_use;
                        
                        let column_width = 250.0; // Width per column
                        let total_width = (cols_to_use as f32 * column_width) + 50.0; // Add some padding
                        let total_height = input_height + mid_drag_height + header_height + (rows_per_col as f32 * row_height) + bottom_drag_height + padding;
                        (total_width, total_height)
                    } else {
                        let window_width = 500.0;
                        let required_height = input_height + mid_drag_height + header_height + (display_commands.len() as f32 * row_height) + bottom_drag_height + padding;
                        (window_width, required_height)
                    };
                    
                    // Adjust window size if command editor is visible - make it larger to accommodate the dialog
                    let (final_width, final_height) = if self.command_editor.visible {
                        // Make window significantly larger when command editor is open
                        let editor_width = 500.0f32.max(window_width); // At least 500px wide
                        let editor_height = 400.0f32.max(required_height); // At least 400px tall
                        (editor_width, editor_height)
                    } else {
                        (window_width, required_height)
                    };
                    
                    // Resize window to accommodate content
                    if self.manual_resize_mode {
                        // Use manual size if set (works even when dialog is open)
                        if let Some(manual_size) = self.last_manual_size {
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(manual_size));
                        }
                    } else if !self.dialog.visible {
                        // Use automatic size only when dialog is not open
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(final_width, final_height)));
                    }
                    
                    if self.should_use_columns() {
                        // Multi-column display
                        
                        // Show submenu header if applicable (even in multi-column mode)
                        if is_submenu {
                            if let Some(ref prefix) = menu_prefix {
                                let mut header_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                                header_font_id.size *= 1.3;
                                
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(format!("{} ->", prefix)).font(header_font_id));
                                });
                                
                                ui.add_space(8.0);
                            }
                        }
                        
                        // Use the same layout calculation as above
                        let total_items = display_commands.len();
                        let max_rows = self.config.popup_settings.max_rows;
                        let max_cols = self.config.popup_settings.max_columns;
                        let cols_needed = (total_items + max_rows - 1) / max_rows;
                        let cols_to_use = cols_needed.min(max_cols);
                        let rows_per_col = (total_items + cols_to_use - 1) / cols_to_use;
                        
                        ui.horizontal(|ui| {
                            for col in 0..cols_to_use {
                                ui.vertical(|ui| {
                                    for row in 0..rows_per_col {
                                        let i = col * rows_per_col + row;
                                        if i >= display_commands.len() {
                                            break;
                                        }
                                        
                                        let cmd = &display_commands[i];
                                        let is_selected = i == self.selected_index;
                                        let is_separator = Self::is_separator_command(cmd);
                                        
                                        // Determine display text based on submenu mode and position
                                        let display_text = if is_submenu && !is_separator && i < inside_count {
                                            // This is an inside menu command - apply trimming logic
                                            if let Some(ref prefix) = menu_prefix {
                                                // Check if this command should have its prefix trimmed
                                                if cmd.command.len() > prefix.len() {
                                                    let prefix_end = prefix.len();
                                                    if cmd.command[..prefix_end].eq_ignore_ascii_case(prefix) {
                                                        // Check if there's a separator right after the prefix
                                                        if let Some(ch) = cmd.command.chars().nth(prefix_end) {
                                                            if ch == ' ' || ch == '.' || ch == '_' {
                                                                // Trim the prefix and separator
                                                                cmd.command[prefix_end + 1..].to_string()
                                                            } else {
                                                                cmd.command.clone()
                                                            }
                                                        } else {
                                                            cmd.command.clone()
                                                        }
                                                    } else {
                                                        cmd.command.clone()
                                                    }
                                                } else {
                                                    cmd.command.clone()
                                                }
                                            } else {
                                                cmd.command.clone()
                                            }
                                        } else {
                                            // This is either not in submenu mode OR it's an outside menu command
                                            // Always show full command name
                                            cmd.command.clone()
                                        };
                                        
                                        if is_separator {
                                            // Separator - not selectable, different styling
                                            let mut separator_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                                            separator_font_id.size *= 1.2;
                                            ui.label(egui::RichText::new("---").font(separator_font_id).color(egui::Color32::GRAY));
                                        } else {
                                            // Regular command
                                            let mut list_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                                            list_font_id.size *= 1.5; // Make 50% larger
                                            
                                            // Make merged entries (ending with "...") bold
                                            let text = if display_text.ends_with("...") {
                                                egui::RichText::new(&display_text).font(list_font_id.clone()).strong()
                                            } else {
                                                egui::RichText::new(&display_text).font(list_font_id)
                                            };
                                            
                                            let response = ui.selectable_label(
                                                is_selected,
                                                text
                                            );
                                            
                                            if response.clicked() {
                                                self.selected_index = i;
                                                execute_command(&cmd);
                                                process::exit(0);
                                            }
                                        }
                                    }
                                });
                                
                                // Add space between columns (except after last column)
                                if col < cols_to_use - 1 {
                                    ui.add_space(10.0);
                                }
                            }
                        });
                    } else {
                        // Single-column display
                        
                        // Show submenu header if applicable
                        if is_submenu {
                            if let Some(ref prefix) = menu_prefix {
                                let mut header_font_id = ui.style().text_styles.get(&egui::TextStyle::Heading).unwrap().clone();
                                header_font_id.size *= 1.3;
                                
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(format!("{} ->", prefix)).font(header_font_id));
                                });
                                
                                ui.add_space(8.0);
                            }
                        }
                        
                        for (i, cmd) in display_commands.iter().enumerate() {
                            let is_selected = i == self.selected_index;
                            let is_separator = Self::is_separator_command(cmd);
                            
                            if is_separator {
                                // Separator - not selectable, different styling
                                let mut separator_font_id = ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone();
                                separator_font_id.size *= 1.2;
                                ui.label(egui::RichText::new("---").font(separator_font_id).color(egui::Color32::GRAY));
                            } else {
                                // Determine display text based on submenu mode and position
                                let display_text = if is_submenu && i < inside_count {
                                    // This is an inside menu command - apply trimming logic
                                    if let Some(ref prefix) = menu_prefix {
                                        // Check if this command should have its prefix trimmed
                                        if cmd.command.len() > prefix.len() {
                                            let prefix_end = prefix.len();
                                            if cmd.command[..prefix_end].eq_ignore_ascii_case(prefix) {
                                                // Check if there's a separator right after the prefix
                                                if let Some(ch) = cmd.command.chars().nth(prefix_end) {
                                                    if ch == ' ' || ch == '.' || ch == '_' {
                                                        // Trim the prefix and separator
                                                        cmd.command[prefix_end + 1..].to_string()
                                                    } else {
                                                        cmd.command.clone()
                                                    }
                                                } else {
                                                    cmd.command.clone()
                                                }
                                            } else {
                                                cmd.command.clone()
                                            }
                                        } else {
                                            cmd.command.clone()
                                        }
                                    } else {
                                        cmd.command.clone()
                                    }
                                } else {
                                    // This is either not in submenu mode OR it's an outside menu command
                                    // Always show full command name
                                    cmd.command.clone()
                                };
                                
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
                                    
                                    // Make merged entries (ending with "...") bold
                                    let text = if display_text.ends_with("...") {
                                        egui::RichText::new(&display_text).font(list_font_id.clone()).strong()
                                    } else {
                                        egui::RichText::new(&display_text).font(list_font_id)
                                    };
                                    
                                    let response = ui.selectable_label(
                                        is_selected,
                                        text
                                    );
                                    
                                    if response.clicked() {
                                        self.selected_index = i;
                                        execute_command(&cmd);
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
                    }
                } else {
                    // No commands - resize to minimal height (just input field), but expand if command editor is open
                    let input_height = 60.0;
                    let padding = 50.0;
                    let base_height = input_height + padding;
                    
                    let (final_width, final_height) = if self.command_editor.visible {
                        // Make window larger when command editor is open, even with no commands
                        (500.0, 400.0f32.max(base_height))
                    } else {
                        (500.0, base_height)
                    };
                    
                    // Resize window (same logic as when there are commands)
                    if self.manual_resize_mode {
                        // Use manual size if set (works even when dialog is open)
                        if let Some(manual_size) = self.last_manual_size {
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(manual_size));
                        }
                    } else if !self.dialog.visible {
                        // Use automatic size only when dialog is not open
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(final_width, final_height)));
                    }
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