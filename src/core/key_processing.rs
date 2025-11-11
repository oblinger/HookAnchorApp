//! Key processing and registry system
//!
//! This module provides unified key processing using the Keystroke system
//! and a registry-based dispatch pattern for clean separation of concerns.

use crate::prelude::*;

/// Modifier state for key events
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
}

impl Modifiers {
    /// Create modifiers with all false
    pub fn none() -> Self {
        Self { ctrl: false, alt: false, shift: false, cmd: false }
    }
    
    /// Create modifiers from egui modifiers
    pub fn from_egui(modifiers: &egui::Modifiers) -> Self {
        Self {
            ctrl: modifiers.ctrl,
            alt: modifiers.alt,
            shift: modifiers.shift,
            cmd: modifiers.command,
        }
    }
}

/// Unified keystroke representation using egui::Key + Modifiers
/// This directly matches egui's event format for efficient comparison
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Keystroke {
    /// The physical key pressed
    pub key: egui::Key,
    /// Modifier keys state  
    pub modifiers: Modifiers,
}

impl Keystroke {
    /// Create a new keystroke
    pub fn new(key: egui::Key, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }
    
    /// Create a keystroke with no modifiers
    pub fn simple(key: egui::Key) -> Self {
        Self { key, modifiers: Modifiers::none() }
    }
    
    /// Convert a key string to a Keystroke
    /// Examples: "@" -> Keystroke(Num2, shift=true), "Escape" -> Keystroke(Escape, none)
    /// Key strings can appear in config files, templates, keybindings, etc.
    pub fn from_key_string(key_str: &str) -> Result<Self, String> {
        Self::parse_key_string(key_str)
    }
    
    /// Check if this keystroke matches an egui event
    pub fn matches_event(&self, event: &egui::Event) -> bool {
        match event {
            egui::Event::Key { key, pressed, modifiers, .. } => {
                // Always log bracket key events
                if *key == egui::Key::OpenBracket || *key == egui::Key::CloseBracket {
                    detailed_log("BRACKET_MATCH", &format!("Testing key={:?} pressed={} modifiers={:?} against keystroke={:?}", 
                        key, pressed, modifiers, self));
                }
                
                detailed_log("KEY_PROCESSING", &format!("Key event: {:?} pressed={} modifiers={:?}", key, pressed, modifiers));
                if !pressed {
                    if *key == egui::Key::OpenBracket || *key == egui::Key::CloseBracket {
                        detailed_log("BRACKET_MATCH", &format!("Key not pressed, returning false"));
                    }
                    return false;
                }
                
                // Log detailed matching for bracket keys
                if *key == egui::Key::OpenBracket || *key == egui::Key::CloseBracket {
                    detailed_log("BRACKET_MATCH", &format!("key={:?} vs self.key={:?}, modifiers={:?} vs self.modifiers={:?}",
                        key, self.key, Modifiers::from_egui(modifiers), self.modifiers));
                }

                // For logical punctuation keys (?, +, |, :), ignore shift in modifier matching
                // egui sends these as separate Key variants (Questionmark, Plus, etc.)
                let is_logical_punctuation = LOGICAL_PUNCTUATION_KEYS.contains(key);

                let modifiers_match = if is_logical_punctuation {
                    // Compare modifiers ignoring shift
                    let incoming = Modifiers::from_egui(modifiers);
                    incoming.ctrl == self.modifiers.ctrl &&
                    incoming.alt == self.modifiers.alt &&
                    incoming.cmd == self.modifiers.cmd
                } else {
                    // Normal comparison including shift
                    Modifiers::from_egui(modifiers) == self.modifiers
                };

                // Key matching
                let key_match = *key == self.key;

                key_match && modifiers_match
            }
            _ => false
        }
    }
    
    /// Check if this keystroke would consume both a KEY event and a TEXT event
    /// Returns (consumes_key_event, consumes_text_event)
    pub fn would_consume_events(&self, key_event: &egui::Event, text_event: &egui::Event) -> (bool, bool) {
        let matches_key = self.matches_event(key_event);
        let matches_text = if matches_key {
            // If the key matches, check if the text event is the expected result
            match text_event {
                egui::Event::Text(text) => self.generates_text(text),
                _ => false
            }
        } else {
            false
        };
        (matches_key, matches_text)
    }
    
    /// Check if this keystroke would generate the given text
    /// This replaces the old key_generates_text function
    fn generates_text(&self, text: &str) -> bool {
        match (&self.key, &self.modifiers) {
            // Shifted number keys
            (egui::Key::Num1, Modifiers { shift: true, .. }) => text == "!",
            (egui::Key::Num2, Modifiers { shift: true, .. }) => text == "@",
            (egui::Key::Num3, Modifiers { shift: true, .. }) => text == "#",
            (egui::Key::Num4, Modifiers { shift: true, .. }) => text == "$",
            (egui::Key::Num5, Modifiers { shift: true, .. }) => text == "%",
            (egui::Key::Num6, Modifiers { shift: true, .. }) => text == "^",
            (egui::Key::Num7, Modifiers { shift: true, .. }) => text == "&",
            (egui::Key::Num8, Modifiers { shift: true, .. }) => text == "*",
            (egui::Key::Num9, Modifiers { shift: true, .. }) => text == "(",
            (egui::Key::Num0, Modifiers { shift: true, .. }) => text == ")",
            
            // Unshifted number keys
            (egui::Key::Num0, Modifiers { shift: false, .. }) => text == "0",
            (egui::Key::Num1, Modifiers { shift: false, .. }) => text == "1",
            (egui::Key::Num2, Modifiers { shift: false, .. }) => text == "2",
            (egui::Key::Num3, Modifiers { shift: false, .. }) => text == "3",
            (egui::Key::Num4, Modifiers { shift: false, .. }) => text == "4",
            (egui::Key::Num5, Modifiers { shift: false, .. }) => text == "5",
            (egui::Key::Num6, Modifiers { shift: false, .. }) => text == "6",
            (egui::Key::Num7, Modifiers { shift: false, .. }) => text == "7",
            (egui::Key::Num8, Modifiers { shift: false, .. }) => text == "8",
            (egui::Key::Num9, Modifiers { shift: false, .. }) => text == "9",
            
            // Letters - only match if modifiers are just shift or none
            (egui::Key::A, m) if m.ctrl || m.alt || m.cmd => false,
            (egui::Key::A, Modifiers { shift: true, .. }) => text == "A",
            (egui::Key::A, Modifiers { shift: false, .. }) => text == "a",
            // ... we could add all letters but most templates won't use them with modifiers
            
            // Special punctuation keys
            (egui::Key::Minus, Modifiers { shift: true, .. }) => text == "_",
            (egui::Key::Minus, Modifiers { shift: false, .. }) => text == "-",
            (egui::Key::Equals, Modifiers { shift: true, .. }) => text == "+",
            (egui::Key::Equals, Modifiers { shift: false, .. }) => text == "=",
            (egui::Key::Comma, Modifiers { shift: true, .. }) => text == "<",
            (egui::Key::Comma, Modifiers { shift: false, .. }) => text == ",",
            (egui::Key::Period, Modifiers { shift: true, .. }) => text == ">",
            (egui::Key::Period, Modifiers { shift: false, .. }) => text == ".",
            (egui::Key::Slash, Modifiers { shift: true, .. }) => text == "?",
            (egui::Key::Slash, Modifiers { shift: false, .. }) => text == "/",
            (egui::Key::Backslash, Modifiers { shift: true, .. }) => text == "|",
            (egui::Key::Backslash, Modifiers { shift: false, .. }) => text == "\\",
            (egui::Key::Semicolon, Modifiers { shift: true, .. }) => text == ":",
            (egui::Key::Semicolon, Modifiers { shift: false, .. }) => text == ";",
            (egui::Key::Backtick, Modifiers { shift: true, .. }) => text == "~",
            (egui::Key::Backtick, Modifiers { shift: false, .. }) => text == "`",
            (egui::Key::OpenBracket, Modifiers { shift: true, .. }) => text == "{",
            (egui::Key::OpenBracket, Modifiers { shift: false, .. }) => text == "[",
            (egui::Key::CloseBracket, Modifiers { shift: true, .. }) => text == "}",
            (egui::Key::CloseBracket, Modifiers { shift: false, .. }) => text == "]",
            (egui::Key::Space, _) => text == " ",
            
            // Keys that don't generate text events
            (egui::Key::Escape, _) | (egui::Key::Enter, _) | (egui::Key::Tab, _) |
            (egui::Key::ArrowUp, _) | (egui::Key::ArrowDown, _) | 
            (egui::Key::ArrowLeft, _) | (egui::Key::ArrowRight, _) |
            (egui::Key::F1, _) | (egui::Key::F2, _) | (egui::Key::F3, _) | (egui::Key::F4, _) |
            (egui::Key::F5, _) | (egui::Key::F6, _) | (egui::Key::F7, _) | (egui::Key::F8, _) |
            (egui::Key::F9, _) | (egui::Key::F10, _) | (egui::Key::F11, _) | (egui::Key::F12, _) => false,
            
            _ => false
        }
    }
    
    /// Parse a key string into egui::Key + Modifiers
    fn parse_key_string(key_str: &str) -> Result<Self, String> {
        // Special case: if the entire string is just "+", treat it as the plus character
        if key_str == "+" {
            return Self::char_to_keystroke("+");
        }

        // Handle chord notation (contains '+')
        if key_str.contains('+') {
            let parts: Vec<&str> = key_str.split('+').collect();
            if parts.len() < 2 {
                return Err(format!("Invalid chord format: {}", key_str));
            }

            let mut modifiers = Modifiers::none();
            let key_part = parts[parts.len() - 1]; // Last part is the key

            // Parse modifiers
            for modifier in &parts[..parts.len() - 1] {
                match modifier.to_lowercase().as_str() {
                    "cmd" | "command" | "meta" => modifiers.cmd = true,
                    "ctrl" | "control" => modifiers.ctrl = true,
                    "alt" | "option" | "opt" => modifiers.alt = true,
                    "shift" => modifiers.shift = true,
                    _ => return Err(format!("Unknown modifier: {}", modifier)),
                }
            }

            // Parse the key part using the same logic as single characters
            let base_keystroke = Self::char_to_keystroke(key_part)?;

            // Merge the parsed modifiers with any modifiers from the character itself
            let merged_modifiers = Modifiers {
                ctrl: modifiers.ctrl || base_keystroke.modifiers.ctrl,
                alt: modifiers.alt || base_keystroke.modifiers.alt,
                shift: modifiers.shift || base_keystroke.modifiers.shift,
                cmd: modifiers.cmd || base_keystroke.modifiers.cmd,
            };

            return Ok(Self::new(base_keystroke.key, merged_modifiers));
        }

        // Single character - use the helper function
        Self::char_to_keystroke(key_str)
    }

    /// Convert a single character string to a Keystroke (Key + Modifiers)
    /// This is the single source of truth for character-to-key mapping
    fn char_to_keystroke(s: &str) -> Result<Self, String> {
        let (key, modifiers) = match s {
            // Shifted numbers - use base key + shift
            "!" => (egui::Key::Num1, Modifiers { shift: true, ..Modifiers::none() }),
            "@" => (egui::Key::Num2, Modifiers { shift: true, ..Modifiers::none() }),
            "#" => (egui::Key::Num3, Modifiers { shift: true, ..Modifiers::none() }),
            "$" => (egui::Key::Num4, Modifiers { shift: true, ..Modifiers::none() }),
            "%" => (egui::Key::Num5, Modifiers { shift: true, ..Modifiers::none() }),
            "^" => (egui::Key::Num6, Modifiers { shift: true, ..Modifiers::none() }),
            "&" => (egui::Key::Num7, Modifiers { shift: true, ..Modifiers::none() }),
            "*" => (egui::Key::Num8, Modifiers { shift: true, ..Modifiers::none() }),
            "(" => (egui::Key::Num9, Modifiers { shift: true, ..Modifiers::none() }),
            ")" => (egui::Key::Num0, Modifiers { shift: true, ..Modifiers::none() }),

            // Shifted punctuation WITHOUT logical variants - use base key + shift
            "_" => (egui::Key::Minus, Modifiers { shift: true, ..Modifiers::none() }),
            "\"" => (egui::Key::Quote, Modifiers { shift: true, ..Modifiers::none() }),
            "<" => (egui::Key::Comma, Modifiers { shift: true, ..Modifiers::none() }),
            ">" => (egui::Key::Period, Modifiers { shift: true, ..Modifiers::none() }),
            "~" => (egui::Key::Backtick, Modifiers { shift: true, ..Modifiers::none() }),
            "{" => (egui::Key::OpenBracket, Modifiers { shift: true, ..Modifiers::none() }),
            "}" => (egui::Key::CloseBracket, Modifiers { shift: true, ..Modifiers::none() }),

            // Logical punctuation WITH variants - use logical key WITHOUT shift
            // These match egui's behavior where it sends separate Key enum variants
            "?" => (egui::Key::Questionmark, Modifiers::none()),
            "+" => (egui::Key::Plus, Modifiers::none()),
            "|" => (egui::Key::Pipe, Modifiers::none()),
            ":" => (egui::Key::Colon, Modifiers::none()),

            // All other keys (letters, unshifted punctuation, etc.)
            _ => {
                let key = Self::string_to_egui_key(s)?;
                (key, Modifiers::none())
            }
        };

        Ok(Self::new(key, modifiers))
    }
    
    /// Convert a string to egui::Key
    fn string_to_egui_key(s: &str) -> Result<egui::Key, String> {
        match s.to_lowercase().as_str() {
            // Numbers
            "0" => Ok(egui::Key::Num0),
            "1" => Ok(egui::Key::Num1),
            "2" => Ok(egui::Key::Num2),
            "3" => Ok(egui::Key::Num3),
            "4" => Ok(egui::Key::Num4),
            "5" => Ok(egui::Key::Num5),
            "6" => Ok(egui::Key::Num6),
            "7" => Ok(egui::Key::Num7),
            "8" => Ok(egui::Key::Num8),
            "9" => Ok(egui::Key::Num9),
            
            // Letters (case insensitive)
            "a" => Ok(egui::Key::A),
            "b" => Ok(egui::Key::B),
            "c" => Ok(egui::Key::C),
            "d" => Ok(egui::Key::D),
            "e" => Ok(egui::Key::E),
            "f" => Ok(egui::Key::F),
            "g" => Ok(egui::Key::G),
            "h" => Ok(egui::Key::H),
            "i" => Ok(egui::Key::I),
            "j" => Ok(egui::Key::J),
            "k" => Ok(egui::Key::K),
            "l" => Ok(egui::Key::L),
            "m" => Ok(egui::Key::M),
            "n" => Ok(egui::Key::N),
            "o" => Ok(egui::Key::O),
            "p" => Ok(egui::Key::P),
            "q" => Ok(egui::Key::Q),
            "r" => Ok(egui::Key::R),
            "s" => Ok(egui::Key::S),
            "t" => Ok(egui::Key::T),
            "u" => Ok(egui::Key::U),
            "v" => Ok(egui::Key::V),
            "w" => Ok(egui::Key::W),
            "x" => Ok(egui::Key::X),
            "y" => Ok(egui::Key::Y),
            "z" => Ok(egui::Key::Z),
            
            // Special keys
            "escape" => Ok(egui::Key::Escape),
            "enter" => Ok(egui::Key::Enter),
            "space" => Ok(egui::Key::Space),
            "tab" => Ok(egui::Key::Tab),
            "backspace" => Ok(egui::Key::Backspace),
            "delete" => Ok(egui::Key::Delete),
            "arrowup" => Ok(egui::Key::ArrowUp),
            "arrowdown" => Ok(egui::Key::ArrowDown),
            "arrowleft" => Ok(egui::Key::ArrowLeft),
            "arrowright" => Ok(egui::Key::ArrowRight),
            
            // Punctuation (unshifted) - only include keys that exist in egui
            "-" => Ok(egui::Key::Minus),
            "=" => Ok(egui::Key::Equals),
            "\\" => Ok(egui::Key::Backslash),
            ";" => Ok(egui::Key::Semicolon),
            "`" => Ok(egui::Key::Backtick),
            "," => Ok(egui::Key::Comma),
            "." => Ok(egui::Key::Period),
            "/" => Ok(egui::Key::Slash),
            "[" => Ok(egui::Key::OpenBracket),
            "]" => Ok(egui::Key::CloseBracket),
            
            // Function keys
            "f1" => Ok(egui::Key::F1),
            "f2" => Ok(egui::Key::F2),
            "f3" => Ok(egui::Key::F3),
            "f4" => Ok(egui::Key::F4),
            "f5" => Ok(egui::Key::F5),
            "f6" => Ok(egui::Key::F6),
            "f7" => Ok(egui::Key::F7),
            "f8" => Ok(egui::Key::F8),
            "f9" => Ok(egui::Key::F9),
            "f10" => Ok(egui::Key::F10),
            "f11" => Ok(egui::Key::F11),
            "f12" => Ok(egui::Key::F12),
            
            _ => Err(format!("Unknown key: {}", s))
        }
    }
}

// ================================================================================================
// KEY REGISTRY SYSTEM
// ================================================================================================

use std::collections::HashMap;

/// Logical punctuation keys that egui sends when shifted
/// For these keys, we should ignore the shift modifier in matching
const LOGICAL_PUNCTUATION_KEYS: &[egui::Key] = &[
    egui::Key::Questionmark,  // Shift+/
    egui::Key::Plus,          // Shift+=
    egui::Key::Pipe,          // Shift+\
    egui::Key::Colon,         // Shift+;
];

/// Trait for key event handlers
pub trait KeyHandler {
    /// Execute the handler with the given context
    fn execute(&self, context: &mut KeyHandlerContext) -> KeyHandlerResult;
    
    /// Get a description of what this handler does (for debugging)
    fn description(&self) -> &str;
}

/// Context passed to key handlers
pub struct KeyHandlerContext<'a> {
    /// The egui event that triggered this handler
    pub event: &'a egui::Event,
    /// Mutable reference to the popup state (we'll define this interface)
    pub popup: &'a mut dyn PopupInterface,
    /// The egui context
    pub egui_ctx: &'a egui::Context,
}

/// Result of key handler execution
#[derive(Debug, PartialEq)]
pub enum KeyHandlerResult {
    /// Handler processed the event successfully
    Handled,
    /// Handler couldn't process this event, try next handler
    NotHandled,
    /// Handler processed event but encountered an error
    Error(String),
}

/// Interface that the popup must implement for key handlers
pub trait PopupInterface {
    /// Navigate vertically (positive = down, negative = up)
    fn navigate_vertical(&mut self, delta: i32);
    
    /// Navigate horizontally (positive = right, negative = left)  
    fn navigate_horizontal(&mut self, delta: i32);
    
    /// Trigger a rebuild of the command list
    fn trigger_rebuild(&mut self, ctx: &egui::Context);
    
    /// Start the grabber countdown
    fn start_grabber_countdown(&mut self, ctx: &egui::Context);
    
    /// Show the folder of the selected command
    fn show_folder(&mut self);
    
    /// Show contact for selected command (strips @ prefix)
    fn show_contact(&mut self);
    
    /// Activate TMUX - open folder, tmux session, and Obsidian (formerly activate_anchor)
    fn activate_tmux(&mut self);

    /// Create child command using anchor's template parameter
    fn create_child(&mut self);

    /// Exit the application
    fn perform_exit_scanner_check(&mut self);
    
    /// Request exit (through unified exit pathway)
    fn request_exit(&mut self);
    
    /// Execute the currently selected command
    fn execute_selected_command(&mut self);
    
    /// Open the command editor
    fn open_command_editor(&mut self);
    
    /// Handle add alias action
    fn handle_add_alias(&mut self);
    
    /// Edit the active command
    fn edit_active_command(&mut self);
    
    /// Edit/create command from input text
    fn edit_input_command(&mut self);
    
    /// Handle link to clipboard
    fn handle_link_to_clipboard(&mut self);
    
    /// Show the keys dialog
    fn show_keys_dialog(&mut self);

    /// Show command history viewer
    fn show_history_viewer(&mut self);

    /// Toggle showing files in prefix menu
    fn toggle_show_files(&mut self);

    /// Handle uninstall app
    fn handle_uninstall_app(&mut self);
    
    /// Handle template creation
    fn handle_template_create(&mut self);
    
    /// Handle named template creation
    fn handle_template_create_named(&mut self, template_name: &str);
    
    /// Check if command editor is visible
    fn is_command_editor_visible(&self) -> bool;
    
    /// Check if dialog is visible
    fn is_dialog_visible(&self) -> bool;
    
    /// Close the command editor
    fn close_command_editor(&mut self);
    
    /// Close the dialog
    fn close_dialog(&mut self);
    
    /// Get the current search text
    fn get_search_text(&self) -> &str;
    
    /// Update the search text
    fn update_search(&mut self, text: String);
    
    /// Navigate up hierarchy to parent patch
    fn navigate_up_hierarchy(&mut self);
    
    /// Navigate down hierarchy into selected anchor prefix menu
    fn navigate_down_hierarchy(&mut self);

    /// Set input text and position cursor at the end
    fn set_input(&mut self, text: String);
}

/// Registry for key event handlers
pub struct KeyRegistry {
    /// Map from keystroke to handler
    handlers: HashMap<Keystroke, Box<dyn KeyHandler>>,
    /// Special handlers for text-based commands (like "uninstall hookanchor")
    text_handlers: Vec<Box<dyn TextHandler>>,
}

/// Trait for text-based command handlers
pub trait TextHandler {
    /// Check if this handler can process the given search text
    fn matches(&self, search_text: &str) -> bool;
    
    /// Execute the handler
    fn execute(&self, context: &mut KeyHandlerContext) -> KeyHandlerResult;
    
    /// Get a description of what this handler does
    fn description(&self) -> &str;
}

impl KeyRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            text_handlers: Vec::new(),
        }
    }
    
    /// Register a keystroke handler
    pub fn register_keystroke(&mut self, keystroke: Keystroke, handler: Box<dyn KeyHandler>) {
        self.handlers.insert(keystroke, handler);
    }
    
    /// Register a text handler
    pub fn register_text_handler(&mut self, handler: Box<dyn TextHandler>) {
        self.text_handlers.push(handler);
    }
    
    /// Process key events using registered handlers  
    pub fn process_events(&self, events: &[egui::Event], popup: &mut dyn PopupInterface, egui_ctx: &egui::Context) -> bool {
        if !events.is_empty() {
            // Create descriptive list of what events are actually being processed
            let key_events: Vec<String> = events.iter().filter_map(|e| match e {
                egui::Event::Key { key, pressed, modifiers, .. } => {
                    Some(format!("{}{}{}",
                        if *pressed { "" } else { "↑" },
                        format!("{:?}", key),
                        if modifiers.any() { format!("+{:?}", modifiers) } else { String::new() }
                    ))
                },
                egui::Event::Text(t) => Some(format!("'{}'", t)),
                _ => None,
            }).collect();

            if !key_events.is_empty() {
                detailed_log("KEY_REGISTRY", &format!("{}", key_events.join(", ")));
            }
        }
        // Log EVERY incoming event
        for event in events {
            match event {
                egui::Event::Key { key, pressed, modifiers, .. } => {
                    if *pressed {
                        detailed_log("KEY_PRESS", &format!("Key pressed: {:?} (modifiers={:?})", key, modifiers));
                    }
                    detailed_log("KEYSTROKE", &format!(
                        "Key event: key={:?}, pressed={}, modifiers={:?}", 
                        key, pressed, modifiers
                    ));
                },
                egui::Event::Text(text) => {
                    detailed_log("KEYSTROKE", &format!("Text event: '{}'", text));
                },
                _ => {
                    // Log other events too for completeness
                    detailed_log("KEYSTROKE", &format!("Other event: {:?}", event));
                }
            }
        }
        
        let mut handled = false;
        
        // First check text-based handlers
        let search_text = popup.get_search_text().to_lowercase();
        for text_handler in &self.text_handlers {
            if text_handler.matches(&search_text) {
                let mut context = KeyHandlerContext {
                    event: &egui::Event::Text(search_text.clone()), // Dummy event for text handlers
                    popup,
                    egui_ctx,
                };
                
                match text_handler.execute(&mut context) {
                    KeyHandlerResult::Handled => {
                        handled = true;
                        break;
                    }
                    KeyHandlerResult::Error(e) => {
                        log_error(&format!("Text handler error: {}", e));
                        handled = true;
                        break;
                    }
                    KeyHandlerResult::NotHandled => continue,
                }
            }
        }
        
        if handled {
            return true;
        }
        
        // Then check keystroke handlers
        for event in events {
            // Log ALL events when detailed logging is enabled
            match event {
                egui::Event::Key { key, pressed, modifiers, .. } => {
                    if *pressed {
                        detailed_log("KEY_PRESS", &format!("Key={:?}, Modifiers={{shift:{}, ctrl:{}, alt:{}, cmd:{}}}", 
                            key, modifiers.shift, modifiers.ctrl, modifiers.alt, modifiers.command));
                        
                        // Special logging for > key (Period with Shift)
                        if *key == egui::Key::Period && modifiers.shift {
                            log(">>> GREATER-THAN KEY DETECTED! Period + Shift pressed");
                        }
                        // Special logging for Cmd+D key
                        if *key == egui::Key::D && modifiers.command {
                            detailed_log("KEY", &format!("📝 CMD+D KEY DETECTED! Cmd+D pressed for DOC template"));
                        }
                    }
                }
                egui::Event::Text(text) => {
                    detailed_log("KEY_TEXT", &format!("Text input: '{}'", text));
                }
                _ => {}
            }
            
            // Debug: log how many handlers we're checking
            if let egui::Event::Key { key, pressed, .. } = event {
                if *pressed && *key == egui::Key::Escape {
                    detailed_log("SYSTEM", &format!("Checking {} handlers for Escape key", self.handlers.len()));
                    for (ks, h) in &self.handlers {
                        detailed_log("KEY_HANDLER", &format!("Handler: {:?} -> {}", ks, h.description()));
                    }
                }
            }
            
            // Try all matching handlers until one succeeds
            for (keystroke, handler) in &self.handlers {
                // Debug logging for specific keys - checking against registry
                if let egui::Event::Key { key, pressed, .. } = event {
                    if *pressed {
                        if *key == egui::Key::Semicolon {
                            detailed_log("REGISTRY_CHECK", &format!("🔸 SEMICOLON: Checking handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::OpenBracket {
                            detailed_log("REGISTRY_CHECK", &format!("🔶 LEFT BRACKET: Checking handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::CloseBracket {
                            detailed_log("REGISTRY_CHECK", &format!("🔷 RIGHT BRACKET: Checking handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::Equals {
                            detailed_log("REGISTRY_CHECK", &format!("🟰 EQUALS: Checking handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        }
                    }
                }
                
                let matches = keystroke.matches_event(event);
                
                // Log the matching result for specific keys
                if let egui::Event::Key { key, pressed, .. } = event {
                    if *pressed {
                        if *key == egui::Key::Semicolon && matches {
                            detailed_log("REGISTRY_MATCH", &format!("🔸 SEMICOLON: ✅ MATCHED handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::Semicolon {
                            detailed_log("REGISTRY_MATCH", &format!("🔸 SEMICOLON: ❌ NO MATCH for handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::OpenBracket && matches {
                            detailed_log("REGISTRY_MATCH", &format!("🔶 LEFT BRACKET: ✅ MATCHED handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::OpenBracket {
                            detailed_log("REGISTRY_MATCH", &format!("🔶 LEFT BRACKET: ❌ NO MATCH for handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::CloseBracket && matches {
                            detailed_log("REGISTRY_MATCH", &format!("🔷 RIGHT BRACKET: ✅ MATCHED handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::CloseBracket {
                            detailed_log("REGISTRY_MATCH", &format!("🔷 RIGHT BRACKET: ❌ NO MATCH for handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::Equals && matches {
                            detailed_log("REGISTRY_MATCH", &format!("🟰 EQUALS: ✅ MATCHED handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::Equals {
                            detailed_log("REGISTRY_MATCH", &format!("🟰 EQUALS: ❌ NO MATCH for handler '{}' (keystroke: {:?})", 
                                handler.description(), keystroke));
                        }
                    }
                }
                
                if matches {
                    detailed_log("KEY_HANDLER", &format!("KEY_HANDLER: Handler matched: {}", handler.description()));
                    
                    // Special logging for Enter key
                    if let egui::Event::Key { key, modifiers, .. } = event {
                        if *key == egui::Key::Enter {
                            detailed_log("KEY", &format!("🔵 ENTER KEY: Handler '{}' matched for Enter key!", handler.description()));
                        }
                        // Also log Backtick key
                        if *key == egui::Key::Backtick {
                            detailed_log("KEY", &format!("⚡ BACKTICK KEY: Handler '{}' matched for Backtick key!", handler.description()));
                        }
                        // Log > key (Period with Shift)
                        if *key == egui::Key::Period && modifiers.shift {
                            detailed_log("KEY", &format!(">>> ALIAS KEY: Handler '{}' matched for > key!", handler.description()));
                        }
                        // Log Cmd+D key
                        if *key == egui::Key::D && modifiers.command {
                            detailed_log("KEY", &format!("📝 DOC KEY: Handler '{}' matched for Cmd+D key!", handler.description()));
                        }
                        // Log bracket keys
                        if *key == egui::Key::OpenBracket {
                            detailed_log("KEY", &format!("🔶 LEFT BRACKET KEY: Handler '{}' MATCHED for [ key!", handler.description()));
                        }
                        if *key == egui::Key::CloseBracket {
                            detailed_log("KEY", &format!("🔷 RIGHT BRACKET KEY: Handler '{}' MATCHED for ] key!", handler.description()));
                        }
                        // Log equals and semicolon for comparison
                        if *key == egui::Key::Equals {
                            detailed_log("KEY", &format!("🟰 EQUALS KEY: Handler '{}' MATCHED for = key!", handler.description()));
                        }
                        if *key == egui::Key::Semicolon {
                            detailed_log("KEY", &format!("🔸 SEMICOLON KEY: Handler '{}' MATCHED for ; key!", handler.description()));
                        }
                    }
                    
                    let mut context = KeyHandlerContext {
                        event,
                        popup,
                        egui_ctx,
                    };
                    
                    match handler.execute(&mut context) {
                        KeyHandlerResult::Handled => {
                            detailed_log("KEY_HANDLER", &format!("KEY_HANDLER: Handler HANDLED: {}", handler.description()));
                            
                            // Special logging for Enter key
                            if let egui::Event::Key { key, .. } = event {
                                if *key == egui::Key::Enter {
                                    detailed_log("KEY", &format!("🔵 ENTER KEY: ✅ Handler successfully executed for Enter key!"));
                                }
                                // Also log Backtick key
                                if *key == egui::Key::Backtick {
                                    detailed_log("KEY", &format!("⚡ BACKTICK KEY: ✅ Handler successfully executed for Backtick key!"));
                                }
                            }
                            
                            handled = true;
                            break; // Exit inner loop for this event
                        }
                        KeyHandlerResult::Error(e) => {
                            detailed_log("KEY", &format!("KEY_HANDLER ERROR: {} - {}", handler.description(), e));
                            handled = true;
                            break; // Exit inner loop for this event
                        }
                        KeyHandlerResult::NotHandled => {
                            detailed_log("KEY_HANDLER", &format!("Handler NOT_HANDLED: {}", handler.description()));
                            continue; // Try next handler
                        }
                    }
                }
            }
            if handled {
                detailed_log("KEY_PROCESSING", "🛑 Event handled, stopping processing");
                break; // Exit outer loop if any handler succeeded
            }
        }
        
        handled
    }
    
    
    /// Get all registered keystrokes (for debugging)
    pub fn get_registered_keystrokes(&self) -> Vec<&Keystroke> {
        self.handlers.keys().collect()
    }
}

impl Default for KeyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================================================
// CONCRETE HANDLER IMPLEMENTATIONS
// ================================================================================================

/// Navigation handler for arrow keys and similar
pub struct NavigationHandler {
    direction: NavigationDirection,
    description: String,
}

#[derive(Debug, Clone)]
pub enum NavigationDirection {
    Up,
    Down,
    Left,
    Right,
}

impl NavigationHandler {
    pub fn new(direction: NavigationDirection) -> Self {
        let description = format!("Navigate {}", match direction {
            NavigationDirection::Up => "up",
            NavigationDirection::Down => "down", 
            NavigationDirection::Left => "left",
            NavigationDirection::Right => "right",
        });
        
        Self { direction, description }
    }
}

impl KeyHandler for NavigationHandler {
    fn execute(&self, context: &mut KeyHandlerContext) -> KeyHandlerResult {
        match self.direction {
            NavigationDirection::Up => context.popup.navigate_vertical(-1),
            NavigationDirection::Down => context.popup.navigate_vertical(1),
            NavigationDirection::Left => context.popup.navigate_horizontal(-1), 
            NavigationDirection::Right => context.popup.navigate_horizontal(1),
        }
        KeyHandlerResult::Handled
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Handler for popup actions - uses action string directly from config
/// This eliminates the need for a hardcoded Action enum
pub struct PopupActionHandler {
    action_name: String,
    description: String,
    close_popup: bool,
}

impl PopupActionHandler {
    pub fn new(action_name: String) -> Self {
        Self::with_close_flag(action_name, false)
    }

    pub fn with_close_flag(action_name: String, close_popup: bool) -> Self {
        // Generate description from action name
        let description = match action_name.as_str() {
            "force_rebuild" => "Rebuild files, command and popup server",
            "start_grabber" => "Start grabber countdown",
            "show_folder" => "Show folder of selected command",
            "show_contact" => "Show contact (strips @ prefix)",
            "exit" => "Exit application",
            "execute_command" => "Execute selected command",
            "open_editor" => "Open command editor",
            "add_alias" => "Add alias for last command",
            "edit_active_command" => "Edit active command",
            "edit_input_command" => "Create new command from input",
            "link_to_clipboard" => "Link to clipboard",
            "show_keys" => "Show key bindings",
            "show_history" | "show_history_viewer" => "Show command history",
            "uninstall_app" => "Uninstall application",
            "template_create" => "Create template",
            "activate_tmux" => "Activate TMUX session for selected command",
            "tmux" => "Activate TMUX session for selected command",
            "create_child" => "Create child command using anchor's template",
            "navigate_up_hierarchy" => "Navigate up to parent patch",
            "navigate_down_hierarchy" => "Navigate into selected anchor prefix menu",
            "toggle_show_files" => "Toggle showing files in prefix menus",
            _ => &action_name, // Fallback to action name itself
        }.to_string();

        Self { action_name, description, close_popup }
    }
}

impl PopupActionHandler {
    /// Execute a built-in popup action by name
    /// These are Rust-implemented actions that need direct access to the popup UI state
    fn execute_popup_builtin(&self, context: &mut KeyHandlerContext) -> KeyHandlerResult {
        let result = match self.action_name.as_str() {
            "exit" => {
                // Only exit if no sub-interfaces are visible
                if !context.popup.is_command_editor_visible() && !context.popup.is_dialog_visible() {
                    context.popup.request_exit();
                    KeyHandlerResult::Handled
                } else {
                    // If sub-interfaces are open, let their handlers take priority
                    KeyHandlerResult::NotHandled
                }
            },
            "force_rebuild" => {
                context.popup.trigger_rebuild(context.egui_ctx);
                KeyHandlerResult::Handled
            },
            "start_grabber" => {
                context.popup.start_grabber_countdown(context.egui_ctx);
                KeyHandlerResult::Handled
            },
            "show_folder" => {
                context.popup.show_folder();
                KeyHandlerResult::Handled
            },
            "show_contact" => {
                context.popup.show_contact();
                KeyHandlerResult::Handled
            },
            "activate_tmux" | "tmux" => {
                context.popup.activate_tmux();
                KeyHandlerResult::Handled
            },
            "create_child" => {
                context.popup.create_child();
                KeyHandlerResult::Handled
            },
            "execute_command" => {
                context.popup.execute_selected_command();
                KeyHandlerResult::Handled
            },
            "open_editor" => {
                context.popup.open_command_editor();
                KeyHandlerResult::Handled
            },
            "add_alias" => {
                context.popup.handle_add_alias();
                KeyHandlerResult::Handled
            },
            "edit_active_command" => {
                context.popup.edit_active_command();
                KeyHandlerResult::Handled
            },
            "edit_input_command" => {
                context.popup.edit_input_command();
                KeyHandlerResult::Handled
            },
            "link_to_clipboard" => {
                context.popup.handle_link_to_clipboard();
                KeyHandlerResult::Handled
            },
            "show_keys" => {
                context.popup.show_keys_dialog();
                KeyHandlerResult::Handled
            },
            "show_history" | "show_history_viewer" => {
                context.popup.show_history_viewer();
                KeyHandlerResult::Handled
            },
            "uninstall_app" => {
                context.popup.handle_uninstall_app();
                KeyHandlerResult::Handled
            },
            "template_create" => {
                context.popup.handle_template_create();
                KeyHandlerResult::Handled
            },
            "navigate_up_hierarchy" => {
                context.popup.navigate_up_hierarchy();
                KeyHandlerResult::Handled
            },
            "navigate_down_hierarchy" => {
                context.popup.navigate_down_hierarchy();
                KeyHandlerResult::Handled
            },
            "toggle_show_files" => {
                context.popup.toggle_show_files();
                KeyHandlerResult::Handled
            },
            _ => {
                // Unknown action - log warning and skip
                log(&format!("⚠️ Unknown popup action: {}", self.action_name));
                KeyHandlerResult::NotHandled
            }
        };

        // Check if we should close popup after executing the action
        if self.close_popup && result == KeyHandlerResult::Handled {
            context.popup.request_exit();
        }

        result
    }
}

impl KeyHandler for PopupActionHandler {
    fn execute(&self, context: &mut KeyHandlerContext) -> KeyHandlerResult {
        self.execute_popup_builtin(context)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Template handler for template creation keystrokes
pub struct TemplateHandler {
    template_name: String,
    description: String,
}

impl TemplateHandler {
    pub fn new(template_name: String) -> Self {
        let description = format!("Create template: {}", template_name);
        Self { template_name, description }
    }
}

impl KeyHandler for TemplateHandler {
    fn execute(&self, context: &mut KeyHandlerContext) -> KeyHandlerResult {
        detailed_log("TEMPLATE", &format!(">>> TEMPLATE_HANDLER: Executing template '{}'", self.template_name));
        context.popup.handle_template_create_named(&self.template_name);
        detailed_log("TEMPLATE", &format!(">>> TEMPLATE_HANDLER: Template '{}' execution completed", self.template_name));
        KeyHandlerResult::Handled
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Text-based uninstall handler
pub struct UninstallTextHandler;

impl TextHandler for UninstallTextHandler {
    fn matches(&self, search_text: &str) -> bool {
        search_text == "uninstall hookanchor" || search_text == "!uninstall"
    }
    
    fn execute(&self, context: &mut KeyHandlerContext) -> KeyHandlerResult {
        context.popup.handle_uninstall_app();
        context.popup.update_search(String::new());
        KeyHandlerResult::Handled
    }
    
    fn description(&self) -> &str {
        "Uninstall application via text command"
    }
}

/// Special handler for the "?" key to show keys
pub struct ShowKeysTextHandler;

impl TextHandler for ShowKeysTextHandler {
    fn matches(&self, _search_text: &str) -> bool {
        false // This will be handled by keystroke instead
    }
    
    fn execute(&self, _context: &mut KeyHandlerContext) -> KeyHandlerResult {
        KeyHandlerResult::NotHandled
    }
    
    fn description(&self) -> &str {
        "Show keys dialog"
    }
}

/// Handler for JavaScript actions that executes a JavaScript function via the command server
pub struct JavaScriptHandler {
    function_name: String,
    description: String,
    action_params: std::collections::HashMap<String, serde_json::Value>,
}

impl JavaScriptHandler {
    pub fn new(function_name: String, action_params: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self {
            description: format!("Execute JavaScript function '{}'", function_name),
            function_name,
            action_params,
        }
    }
}

impl KeyHandler for JavaScriptHandler {
    fn execute(&self, _context: &mut KeyHandlerContext) -> KeyHandlerResult {
        log(&format!("🔧 JavaScript handler executing function: {}", self.function_name));

        // Create an action with all the stored parameters from the config
        let mut action_params = self.action_params.clone();
        // Ensure action_type and function are set (may already be in action_params)
        action_params.insert("action_type".to_string(), serde_json::Value::String("js".to_string()));
        action_params.insert("function".to_string(), serde_json::Value::String(self.function_name.clone()));
        let action = crate::execute::Action { params: action_params };

        // Execute the JavaScript function via the command server
        match crate::execute::execute_on_server(&action, None) {
            Ok(result) => {
                log(&format!("✅ JavaScript function '{}' executed successfully: {}", self.function_name, result));
                KeyHandlerResult::Handled
            },
            Err(e) => {
                log_error(&format!("❌ Failed to execute JavaScript function '{}': {}", self.function_name, e));
                KeyHandlerResult::NotHandled
            }
        }
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Factory function to create a fully configured key registry
pub fn create_default_key_registry(config: &super::Config) -> KeyRegistry {
    detailed_log("KEY_REGISTRY", &format!("KEY_REGISTRY: Creating default key registry"));
    detailed_log("KEY_REGISTRY", &format!("KEY_REGISTRY: Config has {} actions", config.actions.as_ref().map(|a| a.len()).unwrap_or(0)));
    let mut registry = KeyRegistry::new();
    let mut registered_count = 0;
    
    
    // Register handlers for unified actions with keys
    log(&format!("Config has actions field: {}", config.actions.is_some()));
    if let Some(ref actions) = config.actions {
        detailed_log("SYSTEM", &format!("Checking {} unified actions for keyboard shortcuts", actions.len()));
        let mut actions_with_keys = 0;
        for (action_name, action) in actions {
            // Check if this action has a key field
            if let Some(key_str) = action.key() {
                detailed_log("KEY_REGISTRY", &format!("Action '{}' has key '{}'", action_name, key_str));
                // Parse the key string into a keystroke
                match Keystroke::from_key_string(key_str) {
                    Ok(keystroke) => {
                        actions_with_keys += 1;
                        // Only log in detailed mode
                        detailed_log("KEY_REGISTRY", &format!("  Action '{}': key={} -> keystroke={:?}", 
                            action_name, key_str, keystroke));
                        
                        // Register handlers based on action type
                        if action.action_type() == "template" {
                            let handler = Box::new(TemplateHandler::new(action_name.clone()));
                            registry.register_keystroke(keystroke.clone(), handler);
                            registered_count += 1;
                            detailed_log("KEY_REGISTRY", &format!("Registered template '{}' to key '{}'", action_name, key_str));
                        } else if action.action_type() == "js" {
                            // Create a JavaScript handler for js actions
                            let function_name = action.params.get("function")
                                .and_then(|v| v.as_str())
                                .unwrap_or(&format!("action_{}", action.action_type()))
                                .to_string();
                            let handler = Box::new(JavaScriptHandler::new(function_name.clone(), action.params.clone()));
                            registry.register_keystroke(keystroke.clone(), handler);
                            registered_count += 1;
                            log(&format!("✅ Registered JavaScript action '{}' (function: {}) to key '{}'", action_name, function_name, key_str));
                            // Special logging for doc template
                            if action_name == "doc" {
                                detailed_log("SYSTEM", &format!("📝 DOC REGISTRATION: Template 'doc' registered to '{}' key", key_str));
                                log(&format!("📝 DOC REGISTRATION: Keystroke details: {:?}", keystroke));
                            }
                            // Special logging for > key
                            if key_str == ">" {
                                detailed_log("SYSTEM", &format!(">>> ALIAS REGISTRATION: Template '{}' registered to '>' key", action_name));
                                detailed_log("SYSTEM", &format!(">>> ALIAS REGISTRATION: Keystroke details: {:?}", keystroke));
                            }
                        } else if action.action_type() == "popup" {
                        // Handle popup actions (navigation, exit, etc.)
                        if let Some(popup_action) = action.params.get("popup_action")
                            .and_then(|v| v.as_str()) {
                            detailed_log("KEY_REGISTRY", &format!("Processing popup action '{}' with key '{}'", popup_action, key_str));
                            let handler: Box<dyn KeyHandler> = match popup_action {
                                "navigate" => {
                                    // Check dx and dy parameters to determine direction
                                    let dx = action.params.get("dx")
                                        .and_then(|v| v.as_i64())
                                        .unwrap_or(0);
                                    let dy = action.params.get("dy")
                                        .and_then(|v| v.as_i64())
                                        .unwrap_or(0);
                                    
                                    let dir_name = if dy < 0 {
                                        "Up"
                                    } else if dy > 0 {
                                        "Down"
                                    } else if dx < 0 {
                                        "Left"
                                    } else if dx > 0 {
                                        "Right"
                                    } else {
                                        continue; // No direction specified
                                    };
                                    
                                    detailed_log("KEY_REGISTRY", &format!("Registering navigation handler: {} (dx={}, dy={})", 
                                        dir_name, dx, dy));
                                    
                                    if dy < 0 {
                                        Box::new(NavigationHandler::new(NavigationDirection::Up))
                                    } else if dy > 0 {
                                        Box::new(NavigationHandler::new(NavigationDirection::Down))
                                    } else if dx < 0 {
                                        Box::new(NavigationHandler::new(NavigationDirection::Left))
                                    } else if dx > 0 {
                                        Box::new(NavigationHandler::new(NavigationDirection::Right))
                                    } else {
                                        continue; // No direction specified
                                    }
                                }
                                // All popup actions now use PopupActionHandler with the action string directly
                                action_str => {
                                    let close_popup = action.params.get("close_popup")
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false);
                                    Box::new(PopupActionHandler::with_close_flag(action_str.to_string(), close_popup))
                                }
                            };
                            registry.register_keystroke(keystroke, handler);
                            registered_count += 1;
                            detailed_log("KEY_REGISTRATION", &format!("Registered popup action '{}' to key '{}'", popup_action, key_str));
                        }
                    } else {
                        // All other action types are JavaScript actions
                        // They follow the pattern: action_type "foo" calls "action_foo()"
                        let function_name = format!("action_{}", action.action_type());
                        let handler = Box::new(JavaScriptHandler::new(function_name.clone(), action.params.clone()));
                        registry.register_keystroke(keystroke.clone(), handler);
                        registered_count += 1;
                        log(&format!("✅ Registered JavaScript action '{}' (function: {}) to key '{}'", action_name, function_name, key_str));
                    }
                    }
                    Err(e) => {
                        detailed_log("KEY", &format!("❌ KEY_REGISTRY ERROR: Failed to parse key '{}' for action '{}': {}", 
                            key_str, action_name, e));
                    }
                }
            }
        }
        if actions_with_keys > 0 {
            log(&format!("Registered {} actions with keys", actions_with_keys));
        }
    }
    
    
    // Register text handlers
    registry.register_text_handler(Box::new(UninstallTextHandler));
    
    detailed_log("KEY_REGISTRY", &format!("Total handlers registered: {}", registered_count));
    detailed_log("KEY_REGISTRY", &format!("Registry has {} keystroke handlers", registry.handlers.len()));
    
    registry
}