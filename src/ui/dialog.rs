//! Dialog System - Modal dialogs for user interaction
//!
//! ## Dialog (struct) -- Template driven dialog manager.  Public Methods:
//! - `new() -> Self` - Creates new empty dialog
//! - `show(&mut self, spec_strings: Vec<String>)` - Shows dialog with given spec strings
//! - `update(&mut self, ctx: &egui::Context) -> bool` - Renders dialog and handles interactions (returns true when dialog closes)
//! - `take_result(&mut self) -> Option<HashMap<String, String>>` - Retrieves result after dialog closes
//! - `is_visible(&self) -> bool` - Check if dialog is currently shown
//!
//! ## Error Handling
//! For error dialogs, use `crate::utils::queue_user_error()` which queues errors for automatic display.
//!
//! ## Spec String Format
//! Each spec string starts with a control character that defines the element type:
//! - `=Title` - Sets dialog window title (not visible content)
//! - `#Text` - Large heading text (starts new row)
//! - `'Text` - Label text (normal size)
//! - `$key,placeholder` - Input field with key and placeholder text
//! - `&Text` - Multi-line text box (auto-sized, read-only)
//! - `^Text` - Multi-line text box with scrolling (for long content)
//! - `!Text` - Button (buttons on same row if consecutive)
//!
//! ## Result Format
//! When a dialog closes, `take_result()` returns a HashMap with:
//! - `"exit"` key contains the button text that was clicked ("OK", "Cancel", etc.)
//! - Other keys contain input field values (keyed by the name from `$key,placeholder`)
//!
//! ## Example Usage
//! ```rust
//! use std::collections::HashMap;
//! use crate::ui::dialog::Dialog;
//!
//! let mut dialog = Dialog::new();
//!
//! // Show a confirmation dialog
//! dialog.show(vec![
//!     "=Confirm Action".to_string(),
//!     "#Please Confirm".to_string(),
//!     "'Are you sure you want to proceed?".to_string(),
//!     "$reason,Enter reason here".to_string(),
//!     "!OK".to_string(),
//!     "!Cancel".to_string(),
//! ]);
//!
//! // In your main update loop
//! loop {
//!     // Check if dialog is open and skip other input handling
//!     if dialog.is_visible() {
//!         // Dialog handles its own input
//!     }
//!
//!     // Update dialog (renders and handles input)
//!     if dialog.update(ctx) {
//!         // Dialog was closed - retrieve result
//!         if let Some(result) = dialog.take_result() {
//!             let button_pressed = result.get("exit").unwrap();
//!             if button_pressed == "OK" {
//!                 let user_reason = result.get("reason").unwrap();
//!                 // Process confirmation...
//!             }
//!         }
//!     }
//! }
//! ```

use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum DialogElement {
    Title(String),
    Label(String),
    Input { key: String, placeholder: String },
    TextBox { content: String },
    ScrollableTextBox { content: String },
    Button { text: String },
}

#[derive(Debug, Clone)]
struct DialogRow {
    elements: Vec<DialogElement>,
}

pub struct Dialog {
    visible: bool,
    rows: Vec<DialogRow>,
    input_values: HashMap<String, String>,
    result: Option<HashMap<String, String>>,
    title: String,
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

    pub fn show(&mut self, spec_strings: Vec<String>) {
        crate::utils::detailed_log("DIALOG", "Dialog opened");
        self.visible = true;
        self.rows.clear();
        self.input_values.clear();
        self.result = None;
        self.title = "Dialog".to_string(); // Reset to default
        
        self.parse_spec_strings(spec_strings);
    }

    pub fn take_result(&mut self) -> Option<HashMap<String, String>> {
        self.result.take()
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

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
                    // TextBox - multi-line text display (auto-sized), starts a new row
                    if !current_row.elements.is_empty() {
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    current_row.elements.push(DialogElement::TextBox { content: rest.to_string() });
                    self.rows.push(current_row);
                    current_row = DialogRow { elements: Vec::new() };
                    last_was_button = false;
                }
                '^' => {
                    // ScrollableTextBox - multi-line text display with scrolling, starts a new row
                    if !current_row.elements.is_empty() {
                        self.rows.push(current_row);
                        current_row = DialogRow { elements: Vec::new() };
                    }
                    current_row.elements.push(DialogElement::ScrollableTextBox { content: rest.to_string() });
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
        
        // Use hard-coded size for the dialog window to allow scrolling
        let window_width = 800.0;
        let window_height = 600.0;

        egui::Window::new(&self.title)
            .fixed_size([window_width, window_height])
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                let pad = 5.0;
                // Force the window to be the size we want by allocating space
                ui.set_min_width(window_width - 10.0);
                ui.set_min_height(window_height - 10.0);

                // Add padding around the dialog content
                ui.add_space(pad);
                ui.horizontal(|ui| {
                    ui.add_space(pad);
                    ui.vertical(|ui| {
                    // Process each row and handle textbox elements specially
                    for row in &self.rows {
                        // Check row type
                        let is_button_row = row.elements.iter().all(|e| matches!(e, DialogElement::Button { .. }));
                        let has_scrollable_textbox = row.elements.iter().any(|e| matches!(e, DialogElement::ScrollableTextBox { .. }));

                        if is_button_row {
                            // Render buttons at bottom (outside scroll area)
                            continue;
                        } else if has_scrollable_textbox {
                            // Wrap ONLY the scrollable textbox content in a scroll area
                            for element in &row.elements {
                                if let DialogElement::ScrollableTextBox { content } = element {
                                    // Put scrollable textbox inside scroll area
                                    egui::ScrollArea::vertical()
                                        .min_scrolled_height(550.0)
                                        .auto_shrink([false, false])
                                        .show(ui, |ui| {
                                            // Calculate height for content
                                            let lines: Vec<&str> = content.lines().collect();
                                            let line_count = lines.len().max(1);
                                            let text_height = line_count as f32 * 16.0 + 10.0;

                                            ui.add_sized(
                                                [ui.available_width() - 20.0, text_height],
                                                egui::TextEdit::multiline(&mut content.clone())
                                                    .font(egui::TextStyle::Monospace)
                                                    .text_color(egui::Color32::BLACK)
                                                    .interactive(false)
                                            );
                                        });
                                }
                            }
                            ui.add_space(2.0);
                        } else {
                            // Regular content rows (titles, labels, inputs, textbox) - no scroll
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
                                            // Auto-sized textbox (no scrolling) - restored original behavior
                                            let lines: Vec<&str> = content.lines().collect();
                                            let line_count = lines.len().max(1);
                                            let text_height = line_count as f32 * 16.0 + 10.0;

                                            ui.add_sized(
                                                [ui.available_width() - 20.0, text_height],
                                                egui::TextEdit::multiline(&mut content.clone())
                                                    .font(egui::TextStyle::Monospace)
                                                    .text_color(egui::Color32::BLACK)
                                                    .interactive(false)
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                            });
                            ui.add_space(2.0);
                        }
                    }

                    // Render button rows at the bottom (outside scroll area)
                    ui.add_space(10.0);
                    for row in &self.rows {
                        let is_button_row = row.elements.iter().all(|e| matches!(e, DialogElement::Button { .. }));
                        if is_button_row {
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
                            ui.add_space(3.0);
                        }
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
            crate::utils::detailed_log("DIALOG", "Dialog closed");
            self.visible = false;
            return true;
        }
        
        false
    }
}