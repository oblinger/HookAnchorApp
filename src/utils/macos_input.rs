//! Native macOS input functions using CGEvent
//!
//! Provides direct keystroke and typing capabilities without relying on osascript.
//! Requires Accessibility permission for the calling process.

use core_graphics::event::{CGEvent, CGEventFlags, CGKeyCode, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use std::thread;
use std::time::Duration;

use crate::prelude::log;

/// Key codes for common keys (from Carbon HIToolbox/Events.h)
pub mod key_codes {
    pub const RETURN: u16 = 0x24;
    pub const TAB: u16 = 0x30;
    pub const SPACE: u16 = 0x31;
    pub const DELETE: u16 = 0x33;  // Backspace
    pub const ESCAPE: u16 = 0x35;
    pub const COMMAND: u16 = 0x37;
    pub const SHIFT: u16 = 0x38;
    pub const CAPS_LOCK: u16 = 0x39;
    pub const OPTION: u16 = 0x3A;
    pub const CONTROL: u16 = 0x3B;
    pub const RIGHT_SHIFT: u16 = 0x3C;
    pub const RIGHT_OPTION: u16 = 0x3D;
    pub const RIGHT_CONTROL: u16 = 0x3E;
    pub const FUNCTION: u16 = 0x3F;
    pub const F1: u16 = 0x7A;
    pub const F2: u16 = 0x78;
    pub const F3: u16 = 0x63;
    pub const F4: u16 = 0x76;
    pub const F5: u16 = 0x60;
    pub const F6: u16 = 0x61;
    pub const F7: u16 = 0x62;
    pub const F8: u16 = 0x64;
    pub const F9: u16 = 0x65;
    pub const F10: u16 = 0x6D;
    pub const F11: u16 = 0x67;
    pub const F12: u16 = 0x6F;
    pub const HOME: u16 = 0x73;
    pub const PAGE_UP: u16 = 0x74;
    pub const FORWARD_DELETE: u16 = 0x75;
    pub const END: u16 = 0x77;
    pub const PAGE_DOWN: u16 = 0x79;
    pub const LEFT_ARROW: u16 = 0x7B;
    pub const RIGHT_ARROW: u16 = 0x7C;
    pub const DOWN_ARROW: u16 = 0x7D;
    pub const UP_ARROW: u16 = 0x7E;

    // Letter keys (ANSI layout) - from Carbon HIToolbox/Events.h
    pub const A: u16 = 0x00;
    pub const S: u16 = 0x01;
    pub const D: u16 = 0x02;
    pub const F: u16 = 0x03;
    pub const H: u16 = 0x04;
    pub const G: u16 = 0x05;
    pub const Z: u16 = 0x06;
    pub const X: u16 = 0x07;
    pub const C: u16 = 0x08;
    pub const V: u16 = 0x09;
    pub const B: u16 = 0x0B;
    pub const Q: u16 = 0x0C;
    pub const W: u16 = 0x0D;
    pub const E: u16 = 0x0E;
    pub const R: u16 = 0x0F;
    pub const Y: u16 = 0x10;
    pub const T: u16 = 0x11;
    pub const O: u16 = 0x1F;  // Fixed: was 0x15 (which is KEY_4)
    pub const U: u16 = 0x20;  // Fixed: was 0x16 (which is KEY_6)
    pub const I: u16 = 0x22;
    pub const P: u16 = 0x23;
    pub const L: u16 = 0x25;
    pub const J: u16 = 0x26;
    pub const K: u16 = 0x28;
    pub const N: u16 = 0x2D;
    pub const M: u16 = 0x2E;

    // Number keys
    pub const KEY_1: u16 = 0x12;
    pub const KEY_2: u16 = 0x13;
    pub const KEY_3: u16 = 0x14;
    pub const KEY_4: u16 = 0x15;
    pub const KEY_5: u16 = 0x17;
    pub const KEY_6: u16 = 0x16;
    pub const KEY_7: u16 = 0x1A;
    pub const KEY_8: u16 = 0x1C;
    pub const KEY_9: u16 = 0x19;
    pub const KEY_0: u16 = 0x1D;
}

/// Modifier flags for use with send_keystroke
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    Command,
    Shift,
    Option,
    Control,
}

impl Modifier {
    /// Convert to CGEventFlags
    pub fn to_flags(&self) -> CGEventFlags {
        match self {
            Modifier::Command => CGEventFlags::CGEventFlagCommand,
            Modifier::Shift => CGEventFlags::CGEventFlagShift,
            Modifier::Option => CGEventFlags::CGEventFlagAlternate,
            Modifier::Control => CGEventFlags::CGEventFlagControl,
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "cmd" | "command" | "meta" => Some(Modifier::Command),
            "shift" => Some(Modifier::Shift),
            "alt" | "option" | "opt" => Some(Modifier::Option),
            "ctrl" | "control" => Some(Modifier::Control),
            _ => None,
        }
    }
}

/// Combine multiple modifiers into CGEventFlags
fn combine_modifiers(modifiers: &[Modifier]) -> CGEventFlags {
    let mut flags = CGEventFlags::empty();
    for modifier in modifiers {
        flags |= modifier.to_flags();
    }
    flags
}

/// Send a single keystroke with optional modifiers
///
/// # Arguments
/// * `key_code` - The virtual key code to press
/// * `modifiers` - Optional modifier keys (Command, Shift, Option, Control)
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if failed (e.g., no Accessibility permission)
pub fn send_keystroke(key_code: CGKeyCode, modifiers: &[Modifier]) -> Result<(), String> {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|_| "Failed to create CGEventSource - check Accessibility permission")?;

    let flags = combine_modifiers(modifiers);

    // Key down event
    let key_down = CGEvent::new_keyboard_event(source.clone(), key_code, true)
        .map_err(|_| "Failed to create key down event")?;
    key_down.set_flags(flags);
    key_down.post(CGEventTapLocation::HID);

    // Small delay between down and up
    thread::sleep(Duration::from_millis(10));

    // Key up event
    let key_up = CGEvent::new_keyboard_event(source, key_code, false)
        .map_err(|_| "Failed to create key up event")?;
    key_up.set_flags(flags);
    key_up.post(CGEventTapLocation::HID);

    Ok(())
}

/// Get key code for a character (basic ASCII support)
fn char_to_key_code(c: char) -> Option<(CGKeyCode, bool)> {
    // Returns (key_code, needs_shift)
    match c {
        'a' => Some((key_codes::A, false)),
        'b' => Some((key_codes::B, false)),
        'c' => Some((key_codes::C, false)),
        'd' => Some((key_codes::D, false)),
        'e' => Some((key_codes::E, false)),
        'f' => Some((key_codes::F, false)),
        'g' => Some((key_codes::G, false)),
        'h' => Some((key_codes::H, false)),
        'i' => Some((key_codes::I, false)),
        'j' => Some((key_codes::J, false)),
        'k' => Some((key_codes::K, false)),
        'l' => Some((key_codes::L, false)),
        'm' => Some((key_codes::M, false)),
        'n' => Some((key_codes::N, false)),
        'o' => Some((key_codes::O, false)),
        'p' => Some((key_codes::P, false)),
        'q' => Some((key_codes::Q, false)),
        'r' => Some((key_codes::R, false)),
        's' => Some((key_codes::S, false)),
        't' => Some((key_codes::T, false)),
        'u' => Some((key_codes::U, false)),
        'v' => Some((key_codes::V, false)),
        'w' => Some((key_codes::W, false)),
        'x' => Some((key_codes::X, false)),
        'y' => Some((key_codes::Y, false)),
        'z' => Some((key_codes::Z, false)),
        'A' => Some((key_codes::A, true)),
        'B' => Some((key_codes::B, true)),
        'C' => Some((key_codes::C, true)),
        'D' => Some((key_codes::D, true)),
        'E' => Some((key_codes::E, true)),
        'F' => Some((key_codes::F, true)),
        'G' => Some((key_codes::G, true)),
        'H' => Some((key_codes::H, true)),
        'I' => Some((key_codes::I, true)),
        'J' => Some((key_codes::J, true)),
        'K' => Some((key_codes::K, true)),
        'L' => Some((key_codes::L, true)),
        'M' => Some((key_codes::M, true)),
        'N' => Some((key_codes::N, true)),
        'O' => Some((key_codes::O, true)),
        'P' => Some((key_codes::P, true)),
        'Q' => Some((key_codes::Q, true)),
        'R' => Some((key_codes::R, true)),
        'S' => Some((key_codes::S, true)),
        'T' => Some((key_codes::T, true)),
        'U' => Some((key_codes::U, true)),
        'V' => Some((key_codes::V, true)),
        'W' => Some((key_codes::W, true)),
        'X' => Some((key_codes::X, true)),
        'Y' => Some((key_codes::Y, true)),
        'Z' => Some((key_codes::Z, true)),
        '0' => Some((key_codes::KEY_0, false)),
        '1' => Some((key_codes::KEY_1, false)),
        '2' => Some((key_codes::KEY_2, false)),
        '3' => Some((key_codes::KEY_3, false)),
        '4' => Some((key_codes::KEY_4, false)),
        '5' => Some((key_codes::KEY_5, false)),
        '6' => Some((key_codes::KEY_6, false)),
        '7' => Some((key_codes::KEY_7, false)),
        '8' => Some((key_codes::KEY_8, false)),
        '9' => Some((key_codes::KEY_9, false)),
        ' ' => Some((key_codes::SPACE, false)),
        '\n' => Some((key_codes::RETURN, false)),
        '\t' => Some((key_codes::TAB, false)),
        _ => None,
    }
}

/// Type a string by simulating individual keystrokes
///
/// Note: This uses basic key code mapping. For full Unicode support,
/// consider using the CGEventKeyboardSetUnicodeString API.
///
/// # Arguments
/// * `text` - The text to type
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if failed
pub fn type_string(text: &str) -> Result<(), String> {
    log(&format!("MACOS_INPUT: Typing {} characters", text.len()));

    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|_| "Failed to create CGEventSource - check Accessibility permission")?;

    for c in text.chars() {
        if let Some((key_code, needs_shift)) = char_to_key_code(c) {
            let flags = if needs_shift {
                CGEventFlags::CGEventFlagShift
            } else {
                CGEventFlags::empty()
            };

            // Key down
            let key_down = CGEvent::new_keyboard_event(source.clone(), key_code, true)
                .map_err(|_| format!("Failed to create key down event for '{}'", c))?;
            key_down.set_flags(flags);
            key_down.post(CGEventTapLocation::HID);

            thread::sleep(Duration::from_millis(5));

            // Key up
            let key_up = CGEvent::new_keyboard_event(source.clone(), key_code, false)
                .map_err(|_| format!("Failed to create key up event for '{}'", c))?;
            key_up.set_flags(flags);
            key_up.post(CGEventTapLocation::HID);

            thread::sleep(Duration::from_millis(10));
        } else {
            // Skip unsupported characters with a warning
            log(&format!("MACOS_INPUT: Skipping unsupported character: '{}'", c));
        }
    }

    Ok(())
}

/// Press a named key (e.g., "return", "tab", "escape")
///
/// # Arguments
/// * `key_name` - Name of the key to press
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if the key name is not recognized
pub fn press_key(key_name: &str) -> Result<(), String> {
    let key_code = match key_name.to_lowercase().as_str() {
        "return" | "enter" => key_codes::RETURN,
        "tab" => key_codes::TAB,
        "space" => key_codes::SPACE,
        "delete" | "backspace" => key_codes::DELETE,
        "escape" | "esc" => key_codes::ESCAPE,
        "up" | "uparrow" => key_codes::UP_ARROW,
        "down" | "downarrow" => key_codes::DOWN_ARROW,
        "left" | "leftarrow" => key_codes::LEFT_ARROW,
        "right" | "rightarrow" => key_codes::RIGHT_ARROW,
        "home" => key_codes::HOME,
        "end" => key_codes::END,
        "pageup" => key_codes::PAGE_UP,
        "pagedown" => key_codes::PAGE_DOWN,
        "f1" => key_codes::F1,
        "f2" => key_codes::F2,
        "f3" => key_codes::F3,
        "f4" => key_codes::F4,
        "f5" => key_codes::F5,
        "f6" => key_codes::F6,
        "f7" => key_codes::F7,
        "f8" => key_codes::F8,
        "f9" => key_codes::F9,
        "f10" => key_codes::F10,
        "f11" => key_codes::F11,
        "f12" => key_codes::F12,
        _ => return Err(format!("Unknown key name: {}", key_name)),
    };

    send_keystroke(key_code, &[])
}

/// Send a keystroke with modifiers specified by name
///
/// # Arguments
/// * `key` - Single character or key name
/// * `modifier_names` - List of modifier names (e.g., ["cmd", "shift"])
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if failed
pub fn send_keystroke_by_name(key: &str, modifier_names: &[&str]) -> Result<(), String> {
    let modifiers: Vec<Modifier> = modifier_names
        .iter()
        .filter_map(|name| Modifier::from_str(name))
        .collect();

    // Try to interpret as single character first
    if key.len() == 1 {
        let c = key.chars().next().unwrap();
        if let Some((key_code, needs_shift)) = char_to_key_code(c.to_ascii_lowercase()) {
            let mut mods = modifiers.clone();
            if c.is_ascii_uppercase() || needs_shift {
                mods.push(Modifier::Shift);
            }
            return send_keystroke(key_code, &mods);
        }
    }

    // Try as key name
    let key_code = match key.to_lowercase().as_str() {
        "return" | "enter" => key_codes::RETURN,
        "tab" => key_codes::TAB,
        "space" => key_codes::SPACE,
        "delete" | "backspace" => key_codes::DELETE,
        "escape" | "esc" => key_codes::ESCAPE,
        "up" => key_codes::UP_ARROW,
        "down" => key_codes::DOWN_ARROW,
        "left" => key_codes::LEFT_ARROW,
        "right" => key_codes::RIGHT_ARROW,
        _ => return Err(format!("Unknown key: {}", key)),
    };

    send_keystroke(key_code, &modifiers)
}

/// Test if we have Accessibility permission by trying to create a CGEventSource
///
/// # Returns
/// * `true` if we can create events (have permission)
/// * `false` if we cannot
pub fn test_accessibility_permission() -> bool {
    CGEventSource::new(CGEventSourceStateID::HIDSystemState).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_from_str() {
        assert_eq!(Modifier::from_str("cmd"), Some(Modifier::Command));
        assert_eq!(Modifier::from_str("command"), Some(Modifier::Command));
        assert_eq!(Modifier::from_str("shift"), Some(Modifier::Shift));
        assert_eq!(Modifier::from_str("alt"), Some(Modifier::Option));
        assert_eq!(Modifier::from_str("option"), Some(Modifier::Option));
        assert_eq!(Modifier::from_str("ctrl"), Some(Modifier::Control));
        assert_eq!(Modifier::from_str("unknown"), None);
    }

    #[test]
    fn test_char_to_key_code() {
        assert!(char_to_key_code('a').is_some());
        assert!(char_to_key_code('A').is_some());
        assert!(char_to_key_code(' ').is_some());
        assert!(char_to_key_code('1').is_some());
    }
}
