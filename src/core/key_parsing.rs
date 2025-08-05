//! Key parsing and chord handling utilities
//! 
//! Provides functionality to parse key strings in various formats:
//! - ASCII characters: "a", "+", "="
//! - Named keys: "Enter", "Escape", "ArrowUp"
//! - Chord combinations: "Cmd+C", "Ctrl+Shift+A", "Alt+F4"

use std::collections::HashSet;

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
    
    /// Convert a config string to a Keystroke
    /// Examples: "@" -> Keystroke(Num2, shift=true), "Escape" -> Keystroke(Escape, none)
    pub fn from_config_string(config_str: &str) -> Result<Self, String> {
        Self::parse_config_string(config_str)
    }
    
    /// Check if this keystroke matches an egui event
    pub fn matches_event(&self, event: &egui::Event) -> bool {
        match event {
            egui::Event::Key { key, pressed, modifiers, .. } => {
                *pressed && *key == self.key && Modifiers::from_egui(modifiers) == self.modifiers
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
    
    /// Parse a config string into egui::Key + Modifiers
    fn parse_config_string(config_str: &str) -> Result<Self, String> {
        // Handle chord notation (contains '+')
        if config_str.contains('+') {
            let parts: Vec<&str> = config_str.split('+').collect();
            if parts.len() < 2 {
                return Err(format!("Invalid chord format: {}", config_str));
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
        let (key, extra_modifiers) = match config_str {
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
                let key = Self::string_to_egui_key(config_str)?;
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


/// Represents a parsed key combination
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyChord {
    /// The main key (without modifiers)
    pub key: String,
    /// Set of modifier keys
    pub modifiers: HashSet<String>,
}

impl KeyChord {
    /// Create a new KeyChord from a key and modifiers
    pub fn new(key: String, modifiers: HashSet<String>) -> Self {
        Self { key, modifiers }
    }
    
    /// Create a simple KeyChord with no modifiers
    pub fn simple(key: String) -> Self {
        Self {
            key,
            modifiers: HashSet::new(),
        }
    }
    
    /// Check if this chord matches the given key event with modifiers (using new Modifiers struct)
    pub fn matches_with_modifiers(&self, key_name: &str, event_modifiers: &Modifiers) -> bool {
        // For direct text matching (when no modifiers), also try character-based matching
        if !event_modifiers.ctrl && !event_modifiers.alt && !event_modifiers.shift && !event_modifiers.cmd {
            // First try direct key name matching
            if self.key == key_name && self.modifiers.is_empty() {
                return true;
            }
            
            // For text events, try matching the original config key against the event text
            // This handles cases like config_key="!" matching event_key="!" even though
            // the parsed chord is key="1" modifiers={"Shift"}
            if key_name.len() == 1 {
                // Check if this is a shifted character that should match directly
                let shifted_chars = [
                    ("!", "1"), ("@", "2"), ("#", "3"), ("$", "4"), 
                    ("%", "5"), ("^", "6"), ("&", "7"), ("*", "8"),
                    ("(", "9"), (")", "0"), ("_", "-"), ("+", "="),
                    ("{", "["), ("}", "]"), ("|", "\\"), (":", ";"),
                    ("\"", "'"), ("<", ","), (">", "."), ("?", "/"),
                    ("~", "`")
                ];
                
                for (shifted_char, base_key) in &shifted_chars {
                    if key_name == *shifted_char && self.key == *base_key && self.modifiers.contains("Shift") {
                        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
                            use std::io::Write;
                            let _ = writeln!(file, "    ✨ TEXT MATCH: '{}' matches config chord '{}' + Shift", shifted_char, base_key);
                        }
                        return true;
                    }
                }
            }
        }
        
        // Standard matching - compare key and convert chord modifiers to Modifiers struct
        if self.key != key_name {
            return false;
        }
        
        let chord_modifiers = Modifiers {
            ctrl: self.modifiers.contains("Ctrl"),
            alt: self.modifiers.contains("Alt"),
            shift: self.modifiers.contains("Shift"),
            cmd: self.modifiers.contains("Cmd"),
        };
        
        chord_modifiers == *event_modifiers
    }
    
    /// Check if this chord matches the given key event with modifiers (legacy HashSet version)
    pub fn matches(&self, key_name: &str, modifiers: &HashSet<String>) -> bool {
        // Convert HashSet to Modifiers struct and use new method
        let event_modifiers = Modifiers {
            ctrl: modifiers.contains("Ctrl"),
            alt: modifiers.contains("Alt"),
            shift: modifiers.contains("Shift"),
            cmd: modifiers.contains("Cmd"),
        };
        self.matches_with_modifiers(key_name, &event_modifiers)
    }
    
    /// Convert back to string representation
    pub fn to_string(&self) -> String {
        if self.modifiers.is_empty() {
            self.key.clone()
        } else {
            let mut parts: Vec<&str> = self.modifiers.iter().map(|s| s.as_str()).collect();
            parts.sort(); // Consistent ordering
            parts.push(&self.key);
            parts.join("+")
        }
    }
}

/// Parse a key string into a KeyChord
/// 
/// Supports formats:
/// - ASCII: "a", "+", "="
/// - Named: "Enter", "Escape" 
/// - Chords: "Cmd+C", "Ctrl+Shift+A", "Cmd+=", "Cmd+Equals"
/// - Special shifted chars: ">" becomes "Shift+Period", "<" becomes "Shift+Comma"
pub fn parse_key_string(key_str: &str) -> KeyChord {
    // Handle special shifted characters first
    match key_str {
        ">" => {
            let mut modifiers = HashSet::new();
            modifiers.insert("Shift".to_string());
            return KeyChord::new("Period".to_string(), modifiers);
        },
        "<" => {
            let mut modifiers = HashSet::new();
            modifiers.insert("Shift".to_string());
            return KeyChord::new("Comma".to_string(), modifiers);
        },
        "!" => {
            let mut modifiers = HashSet::new();
            modifiers.insert("Shift".to_string());
            return KeyChord::new("1".to_string(), modifiers);
        },
        "@" => {
            let mut modifiers = HashSet::new();
            modifiers.insert("Shift".to_string());
            return KeyChord::new("2".to_string(), modifiers);
        },
        "#" => {
            let mut modifiers = HashSet::new();
            modifiers.insert("Shift".to_string());
            return KeyChord::new("3".to_string(), modifiers);
        },
        "^" => {
            let mut modifiers = HashSet::new();
            modifiers.insert("Shift".to_string());
            return KeyChord::new("6".to_string(), modifiers);
        },
        "&" => {
            let mut modifiers = HashSet::new();
            modifiers.insert("Shift".to_string());
            return KeyChord::new("7".to_string(), modifiers);
        },
        _ => {}
    }
    
    // Handle chord notation (contains '+')
    if key_str.contains('+') {
        let parts: Vec<&str> = key_str.split('+').collect();
        if parts.len() < 2 {
            // Malformed chord, treat as simple key
            return KeyChord::simple(normalize_key_name(key_str));
        }
        
        let mut modifiers = HashSet::new();
        let key_part = parts[parts.len() - 1]; // Last part is the key
        
        // All but last part are modifiers
        for modifier in &parts[..parts.len() - 1] {
            let normalized_modifier = normalize_modifier(modifier);
            modifiers.insert(normalized_modifier);
        }
        
        // Handle special case where key part is a shifted character
        let (final_key, additional_modifiers) = match key_part {
            ">" => ("Period".to_string(), vec!["Shift".to_string()]),
            "<" => ("Comma".to_string(), vec!["Shift".to_string()]),
            "!" => ("1".to_string(), vec!["Shift".to_string()]),
            "@" => ("2".to_string(), vec!["Shift".to_string()]),
            "#" => ("3".to_string(), vec!["Shift".to_string()]),
            "^" => ("6".to_string(), vec!["Shift".to_string()]),
            "&" => ("7".to_string(), vec!["Shift".to_string()]),
            _ => (normalize_key_name(key_part), vec![]),
        };
        
        // Add any additional modifiers from shifted characters
        for additional_mod in additional_modifiers {
            modifiers.insert(additional_mod);
        }
        
        KeyChord::new(final_key, modifiers)
    } else {
        // Simple key (no modifiers) - normalize ASCII characters
        KeyChord::simple(normalize_key_name(key_str))
    }
}

/// Normalize modifier key names to standard forms
fn normalize_modifier(modifier: &str) -> String {
    match modifier.to_lowercase().as_str() {
        // Command key (Mac)
        "cmd" | "command" | "meta" => "Cmd".to_string(),
        // Control key
        "ctrl" | "control" => "Ctrl".to_string(),
        // Alt/Option key
        "alt" | "option" | "opt" => "Alt".to_string(),
        // Shift key
        "shift" => "Shift".to_string(),
        // Pass through unknown modifiers as-is
        _ => modifier.to_string(),
    }
}

/// Check if a key event with modifiers matches a key configuration string (using new Modifiers struct)
pub fn key_matches_with_modifiers(config_key: &str, event_key: &str, event_modifiers: &Modifiers) -> bool {
    // Debug logging for key matching
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
        use std::io::Write;
        let _ = writeln!(file, "    🔍 key_matches_with_modifiers: config_key='{}' event_key='{}' event_modifiers={:?}", 
            config_key, event_key, event_modifiers);
    }
    
    let chord = parse_key_string(config_key);
    
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
        use std::io::Write;
        let _ = writeln!(file, "    📊 Parsed chord: key='{}' modifiers={:?}", chord.key, chord.modifiers);
    }
    
    let result = chord.matches_with_modifiers(event_key, event_modifiers);
    
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/hookanchor_debug.log") {
        use std::io::Write;
        let _ = writeln!(file, "    🎯 Match result: {}", result);
    }
    
    result
}

/// Check if a key event with modifiers matches a key configuration string (legacy HashSet version)
pub fn key_matches(config_key: &str, event_key: &str, event_modifiers: &HashSet<String>) -> bool {
    // Convert HashSet to Modifiers struct and use new function
    let modifiers = Modifiers {
        ctrl: event_modifiers.contains("Ctrl"),
        alt: event_modifiers.contains("Alt"),
        shift: event_modifiers.contains("Shift"),
        cmd: event_modifiers.contains("Cmd"),
    };
    key_matches_with_modifiers(config_key, event_key, &modifiers)
}

/// Normalize key names to handle both ASCII characters and word names
/// 
/// This allows users to write either "/" or "Slash", "=" or "Equals", etc.
/// Both forms will be converted to the egui standard name.
pub fn normalize_key_name(key: &str) -> String {
    match key {
        // ASCII characters -> egui key names
        "-" => "Minus".to_string(),
        "=" => "Equals".to_string(),
        "+" => "Plus".to_string(),
        "/" => "Slash".to_string(),
        "\\" => "Backslash".to_string(),
        ";" => "Semicolon".to_string(),
        "`" => "Backtick".to_string(),
        "," => "Comma".to_string(),
        "." => "Period".to_string(),
        " " => "Space".to_string(),
        ":" => "Colon".to_string(),
        "|" => "Pipe".to_string(),
        "?" => "Questionmark".to_string(),
        "!" => "Exclamationmark".to_string(),
        "'" => "Quote".to_string(),
        "@" => "At".to_string(),
        "#" => "Hash".to_string(),
        "^" => "Caret".to_string(),
        "&" => "Ampersand".to_string(),
        
        // Word names that should pass through unchanged (already correct)
        "Minus" | "Equals" | "Plus" | "Slash" | "Backslash" | "Semicolon" |
        "Backtick" | "Comma" | "Period" | "Space" | "Colon" | "Pipe" |
        "Questionmark" | "Exclamationmark" | "Quote" | "At" | "Hash" | "Caret" | "Ampersand" => key.to_string(),
        
        // Arrow keys
        "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" => key.to_string(),
        
        // Control keys
        "Escape" | "Enter" | "Tab" | "Backspace" | "Delete" | "Insert" |
        "Home" | "End" | "PageUp" | "PageDown" => key.to_string(),
        
        // Function keys
        key if key.starts_with('F') && key[1..].parse::<u8>().is_ok() => key.to_string(),
        
        // Letters and numbers - uppercase for consistency
        c if c.len() == 1 && c.chars().next().unwrap().is_ascii_alphabetic() => c.to_uppercase(),
        c if c.len() == 1 && c.chars().next().unwrap().is_ascii_digit() => c.to_string(),
        
        // Everything else passes through unchanged
        _ => key.to_string(),
    }
}

/// Convert ASCII character to standard key name for compatibility
/// 
/// This is now a wrapper around normalize_key_name for backwards compatibility
pub fn ascii_to_key_name(ascii_char: &str) -> String {
    normalize_key_name(ascii_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_key() {
        let chord = parse_key_string("a");
        assert_eq!(chord.key, "A"); // Now normalized to uppercase
        assert!(chord.modifiers.is_empty());
    }

    #[test]
    fn test_parse_ascii_symbols() {
        let chord = parse_key_string("-");
        assert_eq!(chord.key, "Minus"); // Now normalized to word name
        assert!(chord.modifiers.is_empty());
        
        let chord = parse_key_string("=");
        assert_eq!(chord.key, "Equals"); // Now normalized to word name
        assert!(chord.modifiers.is_empty());
    }

    #[test]
    fn test_parse_chord_keys() {
        let chord = parse_key_string("Cmd+C");
        assert_eq!(chord.key, "C");
        assert!(chord.modifiers.contains("Cmd"));
        assert_eq!(chord.modifiers.len(), 1);
    }

    #[test]
    fn test_parse_multiple_modifiers() {
        let chord = parse_key_string("Ctrl+Shift+A");
        assert_eq!(chord.key, "A");
        assert!(chord.modifiers.contains("Ctrl"));
        assert!(chord.modifiers.contains("Shift"));
        assert_eq!(chord.modifiers.len(), 2);
    }

    #[test]
    fn test_normalize_modifiers() {
        assert_eq!(normalize_modifier("cmd"), "Cmd");
        assert_eq!(normalize_modifier("CTRL"), "Ctrl");
        assert_eq!(normalize_modifier("alt"), "Alt");
        assert_eq!(normalize_modifier("shift"), "Shift");
    }

    #[test]
    fn test_key_matches() {
        let mut modifiers = HashSet::new();
        modifiers.insert("Cmd".to_string());
        
        assert!(key_matches("Cmd+C", "C", &modifiers));
        assert!(!key_matches("Cmd+C", "C", &HashSet::new()));
        assert!(key_matches("a", "A", &HashSet::new())); // Key gets normalized to uppercase
    }

    #[test]
    fn test_chord_to_string() {
        let mut modifiers = HashSet::new();
        modifiers.insert("Cmd".to_string());
        modifiers.insert("Shift".to_string());
        
        let chord = KeyChord::new("A".to_string(), modifiers);
        let string_repr = chord.to_string();
        
        // Should contain all parts (order may vary due to HashSet)
        assert!(string_repr.contains("Cmd"));
        assert!(string_repr.contains("Shift"));
        assert!(string_repr.contains("A"));
        assert!(string_repr.contains("+"));
    }

    #[test]
    fn test_normalize_key_name() {
        // ASCII characters should be normalized to word names
        assert_eq!(normalize_key_name("-"), "Minus");
        assert_eq!(normalize_key_name("="), "Equals");
        assert_eq!(normalize_key_name("+"), "Plus");
        assert_eq!(normalize_key_name("/"), "Slash");
        assert_eq!(normalize_key_name(";"), "Semicolon");
        assert_eq!(normalize_key_name("`"), "Backtick");
        
        // Word names should pass through unchanged
        assert_eq!(normalize_key_name("Minus"), "Minus");
        assert_eq!(normalize_key_name("Equals"), "Equals");
        assert_eq!(normalize_key_name("ArrowUp"), "ArrowUp");
        assert_eq!(normalize_key_name("Enter"), "Enter");
        
        // Letters should be uppercase
        assert_eq!(normalize_key_name("a"), "A");
        assert_eq!(normalize_key_name("z"), "Z");
        
        // Numbers should pass through
        assert_eq!(normalize_key_name("1"), "1");
        assert_eq!(normalize_key_name("0"), "0");
    }

    #[test]
    fn test_parse_ascii_keys_in_chords() {
        // Test ASCII characters in chord combinations
        let chord = parse_key_string("Cmd+=");
        assert_eq!(chord.key, "Equals");
        assert!(chord.modifiers.contains("Cmd"));
        
        let chord = parse_key_string("Ctrl+/");
        assert_eq!(chord.key, "Slash");
        assert!(chord.modifiers.contains("Ctrl"));
        
        let chord = parse_key_string("Alt+-");
        assert_eq!(chord.key, "Minus");
        assert!(chord.modifiers.contains("Alt"));
    }

    #[test]
    fn test_parse_word_keys_in_chords() {
        // Test word names in chord combinations
        let chord = parse_key_string("Cmd+Equals");
        assert_eq!(chord.key, "Equals");
        assert!(chord.modifiers.contains("Cmd"));
        
        let chord = parse_key_string("Ctrl+Slash");
        assert_eq!(chord.key, "Slash");
        assert!(chord.modifiers.contains("Ctrl"));
    }

    #[test]
    fn test_parse_special_shifted_characters() {
        // Test that ">" becomes "Shift+Period"
        let chord = parse_key_string(">");
        assert_eq!(chord.key, "Period");
        assert!(chord.modifiers.contains("Shift"));
        assert_eq!(chord.modifiers.len(), 1);
        
        // Test that "<" becomes "Shift+Comma"
        let chord = parse_key_string("<");
        assert_eq!(chord.key, "Comma");
        assert!(chord.modifiers.contains("Shift"));
        assert_eq!(chord.modifiers.len(), 1);
        
        // Test that "!" becomes "Shift+1"
        let chord = parse_key_string("!");
        assert_eq!(chord.key, "1");
        assert!(chord.modifiers.contains("Shift"));
        assert_eq!(chord.modifiers.len(), 1);
        
        // Test that "Cmd+>" becomes "Cmd+Shift+Period"
        let chord = parse_key_string("Cmd+>");
        assert_eq!(chord.key, "Period");
        assert!(chord.modifiers.contains("Cmd"));
        assert!(chord.modifiers.contains("Shift"));
        assert_eq!(chord.modifiers.len(), 2);
    }

    #[test]
    fn test_key_matches_with_ascii_and_word_names() {
        let mut modifiers = HashSet::new();
        modifiers.insert("Cmd".to_string());
        
        // Both ASCII and word name should match
        assert!(key_matches("Cmd+=", "Equals", &modifiers));
        assert!(key_matches("Cmd+Equals", "Equals", &modifiers));
        
        // Test simple keys
        assert!(key_matches("/", "Slash", &HashSet::new()));
        assert!(key_matches("Slash", "Slash", &HashSet::new()));
    }
}