//! Dialog Viewer - Standalone binary for displaying dialogs
//!
//! This binary displays dialogs independent of the main popup application.
//! It accepts spec strings via command-line arguments and returns results via JSON stdout.
//!
//! ## Command-Line Usage
//!
//! ```bash
//! # General purpose dialog using spec strings
//! HookAnchorDialog --spec "=Title" --spec "#Heading" --spec "!OK"
//!
//! # Convenience wrappers
//! HookAnchorDialog --error "Something failed"
//! HookAnchorDialog --warning "Using default value"
//! HookAnchorDialog --fatal "Critical error"
//! ```
//!
//! ## Output Format
//!
//! Results are returned as JSON to stdout:
//! ```json
//! {"exit": "OK", "field1": "value1"}
//! ```

use eframe::egui;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

struct DialogApp {
    rows: Vec<DialogRow>,
    input_values: HashMap<String, String>,
    result_storage: Arc<Mutex<Option<HashMap<String, String>>>>,
    title: String,
    should_close: bool,
    button_pressed: Option<String>,
}

impl DialogApp {
    fn new(spec_strings: Vec<String>, result_storage: Arc<Mutex<Option<HashMap<String, String>>>>) -> Self {
        let mut app = Self {
            rows: Vec::new(),
            input_values: HashMap::new(),
            result_storage,
            title: "Dialog".to_string(),
            should_close: false,
            button_pressed: None,
        };

        app.parse_spec_strings(spec_strings);
        app
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
                    // Sets the dialog window title
                    self.title = rest.to_string();
                    last_was_button = false;
                }
                '#' => {
                    // Large text display - starts a new row
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
                    // TextBox - multi-line text display (auto-sized)
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
                    // ScrollableTextBox - multi-line text display with scrolling
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
}

impl eframe::App for DialogApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts - only if we haven't already decided to close
        if !self.should_close {
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.should_close = true;
                self.button_pressed = Some(String::new()); // Empty string for escape
                ctx.input_mut(|i| {
                    i.consume_key(egui::Modifiers::NONE, egui::Key::Escape);
                });
            } else if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.should_close = true;
                self.button_pressed = Some("OK".to_string());
                ctx.input_mut(|i| {
                    i.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
                });
            }
        }

        // Fixed window size
        let window_width = 800.0;
        let window_height = 600.0;

        egui::Window::new(&self.title)
            .fixed_size([window_width, window_height])
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                let pad = 10.0;
                let content_width = window_width - (pad * 2.0) - 20.0; // Account for padding and margins

                ui.add_space(pad);
                ui.horizontal(|ui| {
                    ui.add_space(pad);
                    ui.vertical(|ui| {
                        ui.set_width(content_width);
                        // Render content rows
                        for row in &self.rows {
                            let is_button_row = row.elements.iter().all(|e| matches!(e, DialogElement::Button { .. }));
                            let has_scrollable_textbox = row.elements.iter().any(|e| matches!(e, DialogElement::ScrollableTextBox { .. }));

                            if is_button_row {
                                continue; // Render buttons at bottom
                            } else if has_scrollable_textbox {
                                for element in &row.elements {
                                    if let DialogElement::ScrollableTextBox { content } = element {
                                        egui::ScrollArea::vertical()
                                            .max_height(480.0)
                                            .auto_shrink([false, false])
                                            .show(ui, |ui| {
                                                let lines: Vec<&str> = content.lines().collect();
                                                let line_count = lines.len().max(1);
                                                let text_height = line_count as f32 * 16.0 + 10.0;

                                                ui.add_sized(
                                                    [content_width - 10.0, text_height],
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
                                // Regular content rows
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
                                                let lines: Vec<&str> = content.lines().collect();
                                                let line_count = lines.len().max(1);
                                                let text_height = line_count as f32 * 16.0 + 10.0;

                                                ui.add_sized(
                                                    [content_width, text_height],
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

                        // Render button rows at bottom
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
                                                    self.button_pressed = Some(text.clone());
                                                    self.should_close = true;
                                                }
                                            }
                                        }
                                    });
                                });
                                ui.add_space(3.0);
                            }
                        }

                        ui.add_space(pad);
                    });
                    ui.add_space(pad);
                });
            });

        if self.should_close {
            // Create result hashmap
            let mut result = HashMap::new();

            // Add all input values
            for (key, value) in &self.input_values {
                result.insert(key.clone(), value.clone());
            }

            // Add special "exit" key with button text
            result.insert("exit".to_string(), self.button_pressed.clone().unwrap_or_default());

            // Store result in shared storage
            if let Ok(mut storage) = self.result_storage.lock() {
                *storage = Some(result);
            }

            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse command-line arguments to build spec strings
    let spec_strings = parse_args(&args);

    if spec_strings.is_empty() {
        eprintln!("Usage: HookAnchorDialog --spec <spec> [--spec <spec> ...]");
        eprintln!("   or: HookAnchorDialog --error <message>");
        eprintln!("   or: HookAnchorDialog --warning <message>");
        eprintln!("   or: HookAnchorDialog --fatal <message>");
        std::process::exit(1);
    }

    // Create shared storage for result
    let result_storage = Arc::new(Mutex::new(None));
    let result_storage_clone = Arc::clone(&result_storage);

    // Create dialog app
    let app = DialogApp::new(spec_strings, result_storage_clone);

    // Run egui application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(false)
            .with_always_on_top(),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Dialog",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    );

    // After dialog closes, print result as JSON to stdout
    if let Ok(storage) = result_storage.lock() {
        if let Some(result) = &*storage {
            // Output result as JSON
            if let Ok(json) = serde_json::to_string(result) {
                println!("{}", json);
            }
        }
    };
}

fn parse_args(args: &[String]) -> Vec<String> {
    let mut spec_strings = Vec::new();
    let mut i = 1; // Skip program name

    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                if i + 1 < args.len() {
                    spec_strings.push(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("--spec requires an argument");
                    i += 1;
                }
            }
            "--error" => {
                if i + 1 < args.len() {
                    spec_strings.extend(vec![
                        "=Error".to_string(),
                        "#⚠️  Error".to_string(),
                        format!("'{}", args[i + 1]),
                        "!OK".to_string(),
                    ]);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--warning" => {
                if i + 1 < args.len() {
                    spec_strings.extend(vec![
                        "=Warning".to_string(),
                        "#⚡ Warning".to_string(),
                        format!("'{}", args[i + 1]),
                        "!OK".to_string(),
                    ]);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--fatal" => {
                if i + 1 < args.len() {
                    spec_strings.extend(vec![
                        "=Fatal Error".to_string(),
                        "#🛑 Fatal Error".to_string(),
                        format!("'{}", args[i + 1]),
                        "!Exit".to_string(),
                    ]);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                i += 1;
            }
        }
    }

    spec_strings
}
