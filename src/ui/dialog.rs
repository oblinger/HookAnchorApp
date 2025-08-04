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
    pub fn calculate_required_size(&self) -> (f32, f32) {
        // Much simpler calculation based on actual content
        let row_count = self.rows.len() as f32;
        
        // Very conservative estimates based on visual inspection
        let estimated_width = 420.0; // Fixed reasonable width
        let estimated_height = 60.0 + (row_count * 25.0); // Base + rows * height per row
        
        (estimated_width, estimated_height)
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
    
    pub fn update(&mut self, ctx: &egui::Context) -> bool {
        if !self.visible {
            return false;
        }
        
        
        let mut should_close = false;
        let mut button_pressed = None;
        
        // Handle escape key
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            should_close = true;
            button_pressed = Some(String::new()); // Empty string for escape
        }
        
        // Calculate required window size
        let mut required_width = 400.0f32; // minimum width
        let mut required_height = 80.0f32; // base height for title bar and padding
        
        for row in &self.rows {
            let mut row_height = 35.0; // default height per row
            
            // Check if this row contains a TextBox and adjust height
            for element in &row.elements {
                if let DialogElement::TextBox { content } = element {
                    // Calculate height based on content lines, min 4, max 15
                    let line_count = content.lines().count().max(4).min(15) as f32;
                    let text_height = line_count * 18.0_f32; // Slightly larger line height
                    row_height = (text_height + 60.0_f32).max(row_height); // More padding for buttons
                }
            }
            
            required_height += row_height;
            
            // Calculate width needed for this row
            let mut row_width = 40.0; // padding
            for element in &row.elements {
                match element {
                    DialogElement::Title(text) => {
                        row_width += text.len() as f32 * 10.0; // rough estimate for title
                    }
                    DialogElement::Label(text) => {
                        row_width += text.len() as f32 * 8.0; // rough estimate for label
                    }
                    DialogElement::Input { .. } => {
                        row_width += 220.0; // input field width + margin
                    }
                    DialogElement::TextBox { content } => {
                        // Calculate based on longest line
                        let max_line_length = content.lines().map(|line| line.len()).max().unwrap_or(0);
                        row_width += (max_line_length as f32 * 7.0).max(400.0); // minimum 400px wide
                    }
                    DialogElement::Button { text } => {
                        row_width += (text.len() as f32 * 8.0).max(80.0) + 15.0; // button width + spacing
                    }
                }
            }
            required_width = required_width.max(row_width);
        }
        
        required_width = required_width.max(500.0).min(800.0); // clamp between 500-800
        required_height = required_height.max(200.0).min(600.0); // clamp between 200-600

        egui::Window::new(&self.title)
            .default_size([required_width, required_height])
            .resizable(true)
            .collapsible(false)
            .show(ctx, |ui| {
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
                                            // Create a text area that's always 15 lines high with scrolling
                                            let text_height = 15.0_f32 * 16.0_f32; // 15 lines * 16px per line for monospace
                                            
                                            egui::ScrollArea::vertical()
                                                .min_scrolled_height(text_height)
                                                .max_height(text_height)
                                                .show(ui, |ui| {
                                                    ui.add_sized(
                                                        [ui.available_width() - 20.0, text_height],
                                                        egui::TextEdit::multiline(&mut content.clone())
                                                            .font(egui::TextStyle::Monospace)
                                                            .text_color(egui::Color32::BLACK)
                                                            .interactive(false) // Read-only
                                                    );
                                                });
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