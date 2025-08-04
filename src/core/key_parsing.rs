//! Key parsing and chord handling utilities
//! 
//! Provides functionality to parse key strings in various formats:
//! - ASCII characters: "a", "+", "="
//! - Named keys: "Enter", "Escape", "ArrowUp"
//! - Chord combinations: "Cmd+C", "Ctrl+Shift+A", "Alt+F4"

use std::collections::HashSet;

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
    
    /// Check if this chord matches the given key event with modifiers
    pub fn matches(&self, key_name: &str, modifiers: &HashSet<String>) -> bool {
        self.key == key_name && self.modifiers == *modifiers
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

/// Check if a key event with modifiers matches a key configuration string
pub fn key_matches(config_key: &str, event_key: &str, event_modifiers: &HashSet<String>) -> bool {
    let chord = parse_key_string(config_key);
    chord.matches(event_key, event_modifiers)
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
        
        // Word names that should pass through unchanged (already correct)
        "Minus" | "Equals" | "Plus" | "Slash" | "Backslash" | "Semicolon" |
        "Backtick" | "Comma" | "Period" | "Space" | "Colon" | "Pipe" |
        "Questionmark" | "Exclamationmark" | "Quote" => key.to_string(),
        
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