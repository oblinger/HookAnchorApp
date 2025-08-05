use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DialogElement {
    Title(String),
    Label(String),
    Input { key: String, placeholder: String },
    TextBox { content: String },
    Button { text: String },
}

#[derive(Debug, Clone)]
pub struct DialogRow {
    pub elements: Vec<DialogElement>,
}

pub struct Dialog {
    pub visible: bool,
    pub rows: Vec<DialogRow>,
    pub input_values: HashMap<String, String>,
    pub result: Option<HashMap<String, String>>,
    pub title: String,
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            visible: false,
            rows: Vec::new(),
            input_values: HashMap::new(),
            result: None,
            title: "Dialog".to_string(),
        }
    }
    
    #[allow(dead_code)]
    pub fn show(&mut self, spec_strings: Vec<String>) {
        self.visible = true;
        self.rows.clear();
        self.input_values.clear();
        self.result = None;
        self.title = "Dialog".to_string(); // Reset to default
        
        self.parse_spec_strings(spec_strings);
    }
    
    /// Shows an error dialog with the given error message
    pub fn show_error(&mut self, error_message: &str) {
        let spec_strings = vec![
            "=Error".to_string(),
            format!("&{}", error_message), // Use TextBox (&) instead of Label (#) for wrapping
            "!Exit".to_string(),
        ];
        self.show(spec_strings);
    }
    
    #[allow(dead_code)]
    pub fn hide(&mut self) {
        self.visible = false;
        self.result = None;
    }
    
    pub fn take_result(&mut self) -> Option<HashMap<String, String>> {
        self.result.take()
    }
    
    
    #[allow(dead_code)]
    fn parse_spec_strings(&mut self, spec_strings: Vec<String>) {
        let mut current_row = DialogRow { elements: Vec::new() };
        let mut last_was_button = false;
        
        for spec in spec_strings {
            if spec.is_empty() {
                continue;
            }
            
            let control_char = spec.chars().next().unwrap();
            let rest = &spec[1..];
            
            match control_char {
                '=' => {
                    // Sets the dialog window title (doesn't add visible content)
                    self.title = rest.to_string();
                    last_was_button = false;
                }
                '#' => {
                    // Large text display within the dialog - starts a new row
                    if !current_row.elements.is_empty() {
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    current_row.elements.push(DialogElement::Title(rest.to_string()));
                    self.rows.push(current_row);
                    current_row = DialogRow { elements: Vec::new() };
                    last_was_button = false;
                }
                '\'' => {
                    // Label - starts a new row unless last was also a label
                    if !current_row.elements.is_empty() && last_was_button {
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    current_row.elements.push(DialogElement::Label(rest.to_string()));
                    if !current_row.elements.iter().any(|e| matches!(e, DialogElement::Label(_))) || current_row.elements.len() > 1 {
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    last_was_button = false;
                }
                '$' => {
                    // Input field - starts new row
                    if !current_row.elements.is_empty() {
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    
                    let parts: Vec<&str> = rest.splitn(2, ',').collect();
                    if parts.len() == 2 {
                        let key = parts[0].to_string();
                        let placeholder = parts[1].to_string();
                        self.input_values.insert(key.clone(), String::new());
                        current_row.elements.push(DialogElement::Input { 
                            key: key.clone(), 
                            placeholder 
                        });
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    last_was_button = false;
                }
                '&' => {
                    // TextBox - multi-line text display, starts a new row
                    if !current_row.elements.is_empty() {
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    current_row.elements.push(DialogElement::TextBox { content: rest.to_string() });
                    self.rows.push(current_row);
                    current_row = DialogRow { elements: Vec::new() };
                    last_was_button = false;
                }
                '!' => {
                    // Button - can be on same row as previous button
                    if !last_was_button && !current_row.elements.is_empty() {
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    current_row.elements.push(DialogElement::Button { text: rest.to_string() });
                    last_was_button = true;
                }
                _ => {
                    // Unknown control character, treat as label
                    if !current_row.elements.is_empty() {
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    current_row.elements.push(DialogElement::Label(spec));
                    self.rows.push(current_row);
                    current_row = DialogRow { elements: Vec::new() };
                    last_was_button = false;
                }
            }
        }
        
        // Add any remaining elements
        if !current_row.elements.is_empty() {
            self.rows.push(current_row);
        }
    }
    
    /// Calculate the required dialog size using simple estimation (performance optimized)
    pub fn calculate_required_size(&self, _ctx: &egui::Context) -> (f32, f32) {
        let mut max_width = 400.0f32; // reasonable minimum width
        let mut total_height = 30.0f32; // base padding for window chrome and margins
        
        for row in &self.rows {
            let mut row_width = 30.0f32; // left + right padding
            let mut row_height = 6.0f32; // minimum row height for spacing
            
            // Check if this is an empty row (spacing)
            let is_empty_row = row.elements.is_empty() || 
                (row.elements.len() == 1 && 
                 matches!(row.elements[0], DialogElement::Label(ref s) if s.is_empty()));
            
            if is_empty_row {
                // Empty rows still need some height for spacing
                row_height = 12.0;
            } else {
                // Calculate size for each element in the row
                for element in &row.elements {
                    match element {
                        DialogElement::Title(text) => {
                            // Dialog window title (larger font, centered)
                            row_width += text.len() as f32 * 12.0 + 30.0; // larger font + margins
                            row_height = row_height.max(30.0); // extra height for title
                        }
                        DialogElement::Label(text) => {
                            if !text.is_empty() {
                                // Header text like "Available keyboard shortcuts:"
                                row_width += text.len() as f32 * 8.0 + 15.0;
                                row_height = row_height.max(22.0);
                            }
                        }
                        DialogElement::Input { .. } => {
                            // Fixed width for input fields
                            row_width += 200.0;
                            row_height = row_height.max(30.0);
                        }
                        DialogElement::TextBox { content } => {
                            // Calculate textbox dimensions based on content
                            let lines: Vec<&str> = content.lines().collect();
                            let line_count = lines.len().max(1);
                            
                            // Find the longest line for width calculation
                            let max_line_length = lines.iter()
                                .map(|line| line.len())
                                .max()
                                .unwrap_or(0);
                            
                            // Use monospace font estimation: ~7px per character
                            let textbox_width = (max_line_length as f32 * 7.0 + 40.0).max(350.0);
                            let textbox_height = line_count as f32 * 16.0 + 15.0; // 16px line height + padding
                            
                            row_width += textbox_width;
                            row_height = row_height.max(textbox_height);
                        }
                        DialogElement::Button { text } => {
                            // Button with proper sizing
                            let button_width = (text.len() as f32 * 8.0 + 30.0).max(80.0);
                            row_width += button_width + 15.0; // button + spacing
                            row_height = row_height.max(35.0); // button row height
                        }
                    }
                }
            }
            
            // Update maximums
            max_width = max_width.max(row_width);
            total_height += row_height + 6.0; // row height + inter-row spacing
        }
        
        // Add minimal extra spacing for button rows
        let button_rows = self.rows.iter().filter(|row| {
            row.elements.iter().all(|e| matches!(e, DialogElement::Button { .. }))
        }).count();
        total_height += button_rows as f32 * 8.0; // Extra spacing for button rows
        
        // Use configured maximum window sizes
        let config = crate::core::sys_data::get_config();
        let max_width_available = config.popup_settings.get_max_window_width() as f32;
        let max_height_available = config.popup_settings.get_max_window_height() as f32;
        
        // Add window chrome margins (consistent padding around the dialog)
        let pad = 20.0; // Padding around all edges of the dialog
        let final_width_with_margin = max_width + (pad * 2.0);
        let final_height_with_margin = total_height + (pad * 2.0);
        
        // Use calculated size but constrain to configured maximums
        let final_width = final_width_with_margin.max(350.0).min(max_width_available);
        let final_height = final_height_with_margin.max(150.0).min(max_height_available);
        
        (final_width, final_height)
    }

    pub fn update(&mut self, ctx: &egui::Context, exit_dialog_key: Option<&crate::core::key_processing::Keystroke>) -> bool {
        if !self.visible {
            return false;
        }
        
        let mut should_close = false;
        let mut button_pressed = None;
        
        // Simple hardcoded key handling - Escape or Enter to close
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            should_close = true;
            button_pressed = Some(String::new()); // Empty string for escape
            // Consume the key event
            ctx.input_mut(|i| {
                i.consume_key(egui::Modifiers::NONE, egui::Key::Escape);
            });
        } else if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            should_close = true;
            button_pressed = Some("OK".to_string()); // "OK" for enter
            // Consume the key event
            ctx.input_mut(|i| {
                i.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
            });
        }
        
        // Calculate required window size by measuring actual content  
        let (required_width, required_height) = self.calculate_required_size(ctx);
        let pad = 20.0; // Same padding value used in calculate_required_size

        egui::Window::new(&self.title)
            .default_size([required_width, required_height])
            .resizable(true)
            .collapsible(false)
            .show(ctx, |ui| {
                // Add padding around the dialog content
                ui.add_space(pad);
                ui.horizontal(|ui| {
                    ui.add_space(pad);
                    ui.vertical(|ui| {
                    for row in &self.rows {
                        // Check if this row contains only buttons
                        let is_button_row = row.elements.iter().all(|e| matches!(e, DialogElement::Button { .. }));
                        
                        if is_button_row {
                            // Center button rows using centered horizontal layout
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                ui.horizontal(|ui| {
                                    for (i, element) in row.elements.iter().enumerate() {
                                        if i > 0 {
                                            ui.add_space(10.0);
                                        }
                                        if let DialogElement::Button { text } = element {
                                            let button_width = (text.len() as f32 * 8.0).max(80.0);
                                            let button = egui::Button::new(egui::RichText::new(text).size(14.0))
                                                .min_size([button_width, 28.0].into());
                                            if ui.add(button).clicked() {
                                                button_pressed = Some(text.clone());
                                                should_close = true;
                                            }
                                        }
                                    }
                                });
                                // Add extra padding after button rows
                                ui.add_space(12.0);
                            });
                        } else {
                            // Regular layout for non-button rows
                            ui.horizontal(|ui| {
                                for element in &row.elements {
                                    match element {
                                        DialogElement::Title(text) => {
                                            ui.heading(egui::RichText::new(text).size(18.0));
                                        }
                                        DialogElement::Label(text) => {
                                            ui.label(egui::RichText::new(text).size(14.0));
                                        }
                                        DialogElement::Input { key, placeholder } => {
                                            let current_value = self.input_values.get_mut(key).unwrap();
                                            let text_edit = egui::TextEdit::singleline(current_value)
                                                .font(egui::TextStyle::Body)
                                                .text_color(egui::Color32::BLACK)
                                                .desired_width(220.0)
                                                .hint_text(egui::RichText::new(placeholder).size(13.0).color(egui::Color32::LIGHT_GRAY));
                                            ui.add(text_edit);
                                        }
                                        DialogElement::TextBox { content } => {
                                            // Calculate height based on actual content lines
                                            let lines: Vec<&str> = content.lines().collect();
                                            let line_count = lines.len().max(1);
                                            let text_height = line_count as f32 * 16.0 + 15.0; // 16px per line + padding (match sizing calc)
                                            
                                            // Use the calculated height directly without scrolling for better sizing
                                            ui.add_sized(
                                                [ui.available_width() - 20.0, text_height],
                                                egui::TextEdit::multiline(&mut content.clone())
                                                    .font(egui::TextStyle::Monospace)
                                                    .text_color(egui::Color32::BLACK)
                                                    .interactive(false) // Read-only
                                            );
                                        }
                                        DialogElement::Button { text } => {
                                            let button_width = (text.len() as f32 * 8.0).max(80.0);
                                            let button = egui::Button::new(egui::RichText::new(text).size(14.0))
                                                .min_size([button_width, 28.0].into());
                                            if ui.add(button).clicked() {
                                                button_pressed = Some(text.clone());
                                                should_close = true;
                                            }
                                        }
                                    }
                                }
                            });
                        }
                        ui.add_space(8.0);
                    }
                    ui.add_space(pad); // Bottom padding
                });
                ui.add_space(pad); // Right padding
            });
        });
        
        if should_close {
            // Create result hashmap
            let mut result = HashMap::new();
            
            // Add all input values
            for (key, value) in &self.input_values {
                result.insert(key.clone(), value.clone());
            }
            
            // Add special "exit" key with button text or empty for escape
            result.insert("exit".to_string(), button_pressed.unwrap_or_default());
            
            self.result = Some(result);
            self.visible = false;
            return true;
        }
        
        false
    }
}