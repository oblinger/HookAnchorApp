//! Key processing and registry system
//! 
//! This module provides unified key processing using the Keystroke system
//! and a registry-based dispatch pattern for clean separation of concerns.


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
                if !pressed {
                    return false;
                }
                
                let modifiers_match = Modifiers::from_egui(modifiers) == self.modifiers;
                
                // Special cases for shifted punctuation characters
                // Some keyboards/systems send different key codes for these
                // egui only has certain special key variants available
                
                // First check if the incoming key is one of the special variants
                // and map it back to the base key + shift
                if modifiers_match {
                    // If we receive Questionmark, check if we're looking for Slash+Shift
                    if *key == egui::Key::Questionmark && self.key == egui::Key::Slash && self.modifiers.shift {
                        return true;
                    }
                    // If we receive Plus, check if we're looking for Equals+Shift
                    if *key == egui::Key::Plus && self.key == egui::Key::Equals && self.modifiers.shift {
                        return true;
                    }
                    // If we receive Pipe, check if we're looking for Backslash+Shift
                    if *key == egui::Key::Pipe && self.key == egui::Key::Backslash && self.modifiers.shift {
                        return true;
                    }
                    // If we receive Colon, check if we're looking for Semicolon+Shift
                    if *key == egui::Key::Colon && self.key == egui::Key::Semicolon && self.modifiers.shift {
                        return true;
                    }
                }
                
                if self.modifiers.shift && modifiers_match {
                    match self.key {
                        // Plus key: can come through as Plus or Equals+Shift
                        egui::Key::Equals => {
                            return *key == egui::Key::Equals || *key == egui::Key::Plus;
                        }
                        // Question mark: can come through as Questionmark or Slash+Shift
                        egui::Key::Slash => {
                            return *key == egui::Key::Slash || *key == egui::Key::Questionmark;
                        }
                        // Pipe: can come through as Pipe or Backslash+Shift
                        egui::Key::Backslash => {
                            return *key == egui::Key::Backslash || *key == egui::Key::Pipe;
                        }
                        // Colon: can come through as Colon or Semicolon+Shift
                        egui::Key::Semicolon => {
                            return *key == egui::Key::Semicolon || *key == egui::Key::Colon;
                        }
                        // For other shifted keys, just use normal matching
                        // The numbered keys and other punctuation don't have special variants in egui
                        _ => {}
                    }
                }
                
                // Normal key matching
                *key == self.key && modifiers_match
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
        // (Shift+Equals). Without this, it would be parsed as a chord with empty parts.
        if key_str == "+" {
            return Ok(Self::new(egui::Key::Equals, Modifiers { shift: true, ..Modifiers::none() }));
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
            
            let key = Self::string_to_egui_key(key_part)?;
            return Ok(Self::new(key, modifiers));
        }
        
        // Handle shifted characters that should map to base key + shift
        let (key, extra_modifiers) = match key_str {
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
            "_" => (egui::Key::Minus, Modifiers { shift: true, ..Modifiers::none() }),
            "+" => (egui::Key::Equals, Modifiers { shift: true, ..Modifiers::none() }),
            // Note: [ and ] brackets with shift may not be available directly
            // We'll add them later if needed
            "|" => (egui::Key::Backslash, Modifiers { shift: true, ..Modifiers::none() }),
            ":" => (egui::Key::Semicolon, Modifiers { shift: true, ..Modifiers::none() }),
            "\"" => (egui::Key::Quote, Modifiers { shift: true, ..Modifiers::none() }),
            "<" => (egui::Key::Comma, Modifiers { shift: true, ..Modifiers::none() }),
            ">" => (egui::Key::Period, Modifiers { shift: true, ..Modifiers::none() }),
            "?" => (egui::Key::Slash, Modifiers { shift: true, ..Modifiers::none() }),
            "~" => (egui::Key::Backtick, Modifiers { shift: true, ..Modifiers::none() }),
            _ => {
                // Single key without modifiers
                let key = Self::string_to_egui_key(key_str)?;
                (key, Modifiers::none())
            }
        };
        
        Ok(Self::new(key, extra_modifiers))
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

/// Compatibility function for legacy key->text mapping
/// TODO: Remove this once all legacy code is cleaned up
pub fn ascii_to_key_name(ascii_char: &str) -> String {
    ascii_char.to_string() // Simplified - just return the character
}

// ================================================================================================
// KEY REGISTRY SYSTEM
// ================================================================================================

use std::collections::HashMap;

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
#[derive(Debug)]
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
    fn trigger_rebuild(&mut self);
    
    /// Start the grabber countdown
    fn start_grabber_countdown(&mut self, ctx: &egui::Context);
    
    /// Show the folder of the selected command
    fn show_folder(&mut self);
    
    /// Start tmux session in selected anchor folder
    fn tmux_activate(&mut self);
    
    /// Activate anchor - open folder, tmux session, and Obsidian
    fn activate_anchor(&mut self);
    
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
    
    /// Handle link to clipboard
    fn handle_link_to_clipboard(&mut self);
    
    /// Show the keys dialog
    fn show_keys_dialog(&mut self);
    
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
        // Log EVERY incoming event
        for event in events {
            match event {
                egui::Event::Key { key, pressed, modifiers, .. } => {
                    crate::utils::detailed_log("KEYSTROKE", &format!(
                        "Key event: key={:?}, pressed={}, modifiers={:?}", 
                        key, pressed, modifiers
                    ));
                },
                egui::Event::Text(text) => {
                    crate::utils::detailed_log("KEYSTROKE", &format!("Text event: '{}'", text));
                },
                _ => {
                    // Log other events too for completeness
                    crate::utils::detailed_log("KEYSTROKE", &format!("Other event: {:?}", event));
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
                        crate::utils::log_error(&format!("Text handler error: {}", e));
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
                        crate::utils::log(&format!("KEY_PRESS: Key={:?}, Modifiers={{shift:{}, ctrl:{}, alt:{}, cmd:{}}}", 
                            key, modifiers.shift, modifiers.ctrl, modifiers.alt, modifiers.command));
                    }
                }
                egui::Event::Text(text) => {
                    crate::utils::detailed_log("KEY_TEXT", &format!("Text input: '{}'", text));
                }
                _ => {}
            }
            
            // Debug: log how many handlers we're checking
            if let egui::Event::Key { key, pressed, .. } = event {
                if *pressed && *key == egui::Key::Escape {
                    crate::utils::log(&format!("Checking {} handlers for Escape key", self.handlers.len()));
                    for (ks, h) in &self.handlers {
                        crate::utils::log(&format!("  Handler: {:?} -> {}", ks, h.description()));
                    }
                }
            }
            
            // Try all matching handlers until one succeeds
            for (keystroke, handler) in &self.handlers {
                // Debug logging for specific keys
                if let egui::Event::Key { key, pressed, .. } = event {
                    if *pressed {
                        if *key == egui::Key::Enter {
                            crate::utils::log(&format!("🔵 ENTER KEY: Checking handler: {} (keystroke: {:?})", 
                                handler.description(), keystroke));
                        } else if *key == egui::Key::Backtick {
                            crate::utils::log(&format!("⚡ BACKTICK KEY: Checking handler: {} (keystroke: {:?})", 
                                handler.description(), keystroke));
                        }
                    }
                }
                
                if keystroke.matches_event(event) {
                    crate::utils::log(&format!("KEY_HANDLER: Handler matched: {}", handler.description()));
                    
                    // Special logging for Enter key
                    if let egui::Event::Key { key, .. } = event {
                        if *key == egui::Key::Enter {
                            crate::utils::log(&format!("🔵 ENTER KEY: Handler '{}' matched for Enter key!", handler.description()));
                        }
                        // Also log Backtick key
                        if *key == egui::Key::Backtick {
                            crate::utils::log(&format!("⚡ BACKTICK KEY: Handler '{}' matched for Backtick key!", handler.description()));
                        }
                    }
                    
                    let mut context = KeyHandlerContext {
                        event,
                        popup,
                        egui_ctx,
                    };
                    
                    match handler.execute(&mut context) {
                        KeyHandlerResult::Handled => {
                            crate::utils::log(&format!("KEY_HANDLER: Handler HANDLED: {}", handler.description()));
                            
                            // Special logging for Enter key
                            if let egui::Event::Key { key, .. } = event {
                                if *key == egui::Key::Enter {
                                    crate::utils::log("🔵 ENTER KEY: ✅ Handler successfully executed for Enter key!");
                                }
                                // Also log Backtick key
                                if *key == egui::Key::Backtick {
                                    crate::utils::log("⚡ BACKTICK KEY: ✅ Handler successfully executed for Backtick key!");
                                }
                            }
                            
                            handled = true;
                            break; // Exit inner loop for this event
                        }
                        KeyHandlerResult::Error(e) => {
                            crate::utils::log(&format!("KEY_HANDLER ERROR: {} - {}", handler.description(), e));
                            handled = true;
                            break; // Exit inner loop for this event
                        }
                        KeyHandlerResult::NotHandled => {
                            crate::utils::detailed_log("KEY_HANDLER", &format!("Handler NOT_HANDLED: {}", handler.description()));
                            continue; // Try next handler
                        }
                    }
                }
            }
            if handled {
                crate::utils::detailed_log("KEY_PROCESSING", "🛑 Event handled, stopping processing");
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

/// Simple action handler for single-action commands
pub struct ActionHandler {
    action: Action,
    description: String,
}

#[derive(Debug, Clone)]
pub enum Action {
    ForceRebuild,
    StartGrabber,
    ShowFolder,
    ExitApp,
    ExecuteCommand,
    OpenEditor,
    AddAlias,
    EditActiveCommand,
    LinkToClipboard,
    ShowKeys,
    UninstallApp,
    TemplateCreate,
    TmuxActivate,
    ActivateAnchor,
}

impl ActionHandler {
    pub fn new(action: Action) -> Self {
        let description = match action {
            Action::ForceRebuild => "Force rebuild command list",
            Action::StartGrabber => "Start grabber countdown",
            Action::ShowFolder => "Show folder of selected command",
            Action::TmuxActivate => "Start tmux session in anchor folder",
            Action::ExitApp => "Exit application",
            Action::ExecuteCommand => "Execute selected command",
            Action::OpenEditor => "Open command editor",
            Action::AddAlias => "Add alias for last command",
            Action::EditActiveCommand => "Edit active command",
            Action::LinkToClipboard => "Link to clipboard",
            Action::ShowKeys => "Show key bindings",
            Action::UninstallApp => "Uninstall application",
            Action::TemplateCreate => "Create template",
            Action::ActivateAnchor => "Activate anchor (folder, tmux, Obsidian)",
        }.to_string();
        
        Self { action, description }
    }
}

impl KeyHandler for ActionHandler {
    fn execute(&self, context: &mut KeyHandlerContext) -> KeyHandlerResult {
        match self.action {
            Action::ExitApp => {
                // Only exit if no sub-interfaces are visible
                if !context.popup.is_command_editor_visible() && !context.popup.is_dialog_visible() {
                    context.popup.request_exit();
                    KeyHandlerResult::Handled
                } else {
                    // If sub-interfaces are open, let their handlers take priority
                    KeyHandlerResult::NotHandled
                }
            },
            // All other actions are always handled
            Action::ForceRebuild => {
                context.popup.trigger_rebuild();
                KeyHandlerResult::Handled
            },
            Action::StartGrabber => {
                context.popup.start_grabber_countdown(context.egui_ctx);
                KeyHandlerResult::Handled
            },
            Action::ShowFolder => {
                context.popup.show_folder();
                KeyHandlerResult::Handled
            },
            Action::TmuxActivate => {
                context.popup.tmux_activate();
                KeyHandlerResult::Handled
            },
            Action::ActivateAnchor => {
                context.popup.activate_anchor();
                KeyHandlerResult::Handled
            },
            Action::ExecuteCommand => {
                context.popup.execute_selected_command();
                KeyHandlerResult::Handled
            },
            Action::OpenEditor => {
                context.popup.open_command_editor();
                KeyHandlerResult::Handled
            },
            Action::AddAlias => {
                context.popup.handle_add_alias();
                KeyHandlerResult::Handled
            },
            Action::EditActiveCommand => {
                context.popup.edit_active_command();
                KeyHandlerResult::Handled
            },
            Action::LinkToClipboard => {
                context.popup.handle_link_to_clipboard();
                KeyHandlerResult::Handled
            },
            Action::ShowKeys => {
                context.popup.show_keys_dialog();
                KeyHandlerResult::Handled
            },
            Action::UninstallApp => {
                context.popup.handle_uninstall_app();
                KeyHandlerResult::Handled
            },
            Action::TemplateCreate => {
                context.popup.handle_template_create();
                KeyHandlerResult::Handled
            },
        }
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
        context.popup.handle_template_create_named(&self.template_name);
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

/// Factory function to create a fully configured key registry
pub fn create_default_key_registry(config: &crate::Config) -> KeyRegistry {
    crate::utils::log("KEY_REGISTRY: Creating default key registry");
    crate::utils::log(&format!("KEY_REGISTRY: Config has {} actions", config.actions.as_ref().map(|a| a.len()).unwrap_or(0)));
    let mut registry = KeyRegistry::new();
    
    // Register keybinding-based handlers
    if let Some(ref keybindings) = config.keybindings {
        for (action_name, key_str) in keybindings {
            if let Ok(keystroke) = Keystroke::from_key_string(key_str) {
                let handler: Box<dyn KeyHandler> = match action_name.as_str() {
                    "navigate_up" => Box::new(NavigationHandler::new(NavigationDirection::Up)),
                    "navigate_down" => Box::new(NavigationHandler::new(NavigationDirection::Down)),
                    "navigate_left" => Box::new(NavigationHandler::new(NavigationDirection::Left)),
                    "navigate_right" => Box::new(NavigationHandler::new(NavigationDirection::Right)),
                    "force_rebuild" => Box::new(ActionHandler::new(Action::ForceRebuild)),
                    "start_grabber" => Box::new(ActionHandler::new(Action::StartGrabber)),
                    "show_folder" => Box::new(ActionHandler::new(Action::ShowFolder)),
                    "tmux_activate" => Box::new(ActionHandler::new(Action::TmuxActivate)),
                    "exit_app" => Box::new(ActionHandler::new(Action::ExitApp)),
                    "execute_command" => Box::new(ActionHandler::new(Action::ExecuteCommand)),
                    "open_editor" => Box::new(ActionHandler::new(Action::OpenEditor)),
                    "add_alias" => Box::new(ActionHandler::new(Action::AddAlias)),
                    "edit_active_command" => Box::new(ActionHandler::new(Action::EditActiveCommand)),
                    "link_to_clipboard" => Box::new(ActionHandler::new(Action::LinkToClipboard)),
                    "show_keys" => Box::new(ActionHandler::new(Action::ShowKeys)),
                    "uninstall_app" => Box::new(ActionHandler::new(Action::UninstallApp)),
                    "template_create" => Box::new(ActionHandler::new(Action::TemplateCreate)),
                    _ => continue, // Skip unknown actions
                };
                
                registry.register_keystroke(keystroke, handler);
            }
        }
    }
    
    // Register handlers for unified actions with keys
    crate::utils::log(&format!("Config has actions field: {}", config.actions.is_some()));
    if let Some(ref actions) = config.actions {
        crate::utils::log(&format!("Checking {} unified actions for keyboard shortcuts", actions.len()));
        let mut actions_with_keys = 0;
        for (action_name, action) in actions {
            // Check if this action has a key field
            if let Some(ref key_str) = action.key {
                crate::utils::log(&format!("KEY_REGISTRY: Action '{}' has key '{}'", action_name, key_str));
                // Parse the key string into a keystroke
                if let Ok(keystroke) = Keystroke::from_key_string(key_str) {
                    actions_with_keys += 1;
                    // Only log in detailed mode
                    crate::utils::detailed_log("KEY_REGISTRY", &format!("  Action '{}': key={} -> keystroke={:?}", 
                        action_name, key_str, keystroke));
                    
                    // For template actions, use TemplateHandler
                    if action.action_type == "template" {
                        let handler = Box::new(TemplateHandler::new(action_name.clone()));
                        registry.register_keystroke(keystroke, handler);
                    } else if action.action_type == "popup" {
                        // Handle popup actions (navigation, exit, etc.)
                        if let Some(popup_action) = action.params.get("popup_action")
                            .and_then(|v| v.as_str()) {
                            crate::utils::log(&format!("KEY_REGISTRY: Processing popup action '{}' with key '{}'", popup_action, key_str));
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
                                    
                                    crate::utils::log(&format!("  Registering navigation handler: {} (dx={}, dy={})", 
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
                                "exit" => Box::new(ActionHandler::new(Action::ExitApp)),
                                "execute_command" => Box::new(ActionHandler::new(Action::ExecuteCommand)),
                                "force_rebuild" => Box::new(ActionHandler::new(Action::ForceRebuild)),
                                "show_folder" => Box::new(ActionHandler::new(Action::ShowFolder)),
                                "show_keys" => Box::new(ActionHandler::new(Action::ShowKeys)),
                                "edit_active_command" => Box::new(ActionHandler::new(Action::EditActiveCommand)),
                                "open_editor" => Box::new(ActionHandler::new(Action::OpenEditor)),
                                "add_alias" => Box::new(ActionHandler::new(Action::AddAlias)),
                                "tmux_activate" => Box::new(ActionHandler::new(Action::TmuxActivate)),
                                "activate_anchor" => Box::new(ActionHandler::new(Action::ActivateAnchor)),
                                _ => continue, // Skip unknown popup actions
                            };
                            registry.register_keystroke(keystroke, handler);
                        }
                    }
                    // Add handlers for other action types as needed
                }
            }
        }
        if actions_with_keys > 0 {
            crate::utils::log(&format!("Registered {} actions with keys", actions_with_keys));
        }
    }
    
    // Keep backward compatibility: check if old templates field still exists
    if let Some(ref templates) = config.templates {
        crate::utils::log(&format!("Registering {} legacy templates", templates.len()));
        for (template_name, template) in templates {
            if let Some(ref keystroke) = template.keystroke {
                crate::utils::log(&format!("  Template '{}': key={:?} -> keystroke={:?}", 
                    template_name, template.key, keystroke));
                let handler = Box::new(TemplateHandler::new(template_name.clone()));
                registry.register_keystroke(keystroke.clone(), handler);
            } else {
                crate::utils::log(&format!("  Template '{}': NO KEYSTROKE (key={:?})", 
                    template_name, template.key));
            }
        }
    } else {
        crate::utils::log("No templates configured (checking unified actions instead)");
    }
    
    // Register text handlers
    registry.register_text_handler(Box::new(UninstallTextHandler));
    
    registry
}