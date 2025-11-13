//! Global hotkey registration for macOS
//!
//! This module handles registering and responding to global keyboard shortcuts
//! using macOS CGEventTap API. Requires accessibility permissions.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::prelude::*;

/// Represents a global hotkey registration
pub struct GlobalHotkey {
    callback: Arc<dyn Fn() + Send + Sync>,
    is_registered: Arc<AtomicBool>,
    hotkey_string: String,
}

impl GlobalHotkey {
    /// Create a new global hotkey
    pub fn new(callback: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            callback: Arc::new(callback),
            is_registered: Arc::new(AtomicBool::new(false)),
            hotkey_string: String::new(),
        }
    }

    /// Register the global hotkey
    #[cfg(target_os = "macos")]
    pub fn register(&mut self, hotkey_str: &str) -> Result<(), String> {
        // Parse the hotkey string (e.g., "cmd+z", "cmd+shift+space")
        let (modifiers, keycode) = parse_hotkey(hotkey_str)?;

        // Check if we have accessibility permissions
        let has_permissions = check_accessibility_permission();
        log(&format!("Accessibility permissions check: {}", has_permissions));

        if !has_permissions {
            log_error("Missing accessibility permissions!");
            log_error("Please grant HookAnchor accessibility permissions in:");
            log_error("System Settings > Privacy & Security > Accessibility");
            request_accessibility_permission();
            return Err("Accessibility permissions required. Please grant permissions in System Settings > Privacy & Security > Accessibility".to_string());
        }

        // Store the hotkey string
        self.hotkey_string = hotkey_str.to_string();

        log(&format!("Setting up CGEventTap for hotkey: {} (modifiers: {:?}, keycode: {})", hotkey_str, modifiers, keycode));

        // Use a simpler approach with a static callback
        unsafe {
            GLOBAL_CALLBACK = Some(self.callback.clone());
            GLOBAL_MODIFIERS = modifiers;
            GLOBAL_KEYCODE = keycode;
        }

        // Create the event tap using C bindings
        log("Creating CGEventTap...");
        let tap = create_event_tap()?;
        if tap.is_null() {
            let error = "Failed to create event tap. Make sure accessibility permissions are granted.";
            log_error(error);
            return Err(error.to_string());
        }
        log("✅ CGEventTap created successfully");

        // Create run loop source and add to current run loop
        log("Creating run loop source...");
        let source = create_runloop_source(tap)?;
        if source.is_null() {
            let error = "Failed to create run loop source.";
            log_error(error);
            return Err(error.to_string());
        }
        log("✅ Run loop source created successfully");

        log("Adding source to run loop...");
        add_source_to_runloop(source);
        log("✅ Source added to run loop");

        log("Enabling event tap...");
        enable_event_tap(tap);
        log("✅ Event tap enabled");

        self.is_registered.store(true, Ordering::Relaxed);

        log(&format!("✅ Successfully registered global hotkey: {}", hotkey_str));
        Ok(())
    }

    /// Register the global hotkey (non-macOS stub)
    #[cfg(not(target_os = "macos"))]
    pub fn register(&mut self, hotkey_str: &str) -> Result<(), String> {
        self.hotkey_string = hotkey_str.to_string();
        log(&format!("Global hotkey registration not supported on this platform: {}", hotkey_str));
        self.is_registered.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Unregister the hotkey
    pub fn unregister(&mut self) {
        if !self.is_registered.load(Ordering::Relaxed) {
            return;
        }

        #[cfg(target_os = "macos")]
        unsafe {
            GLOBAL_CALLBACK = None;
        }

        self.is_registered.store(false, Ordering::Relaxed);
        log(&format!("Unregistered global hotkey: {}", self.hotkey_string));
    }

    /// Check if hotkey is registered
    pub fn is_registered(&self) -> bool {
        self.is_registered.load(Ordering::Relaxed)
    }
}

impl Drop for GlobalHotkey {
    fn drop(&mut self) {
        self.unregister();
    }
}

// Global state for C callback
#[cfg(target_os = "macos")]
static mut GLOBAL_CALLBACK: Option<Arc<dyn Fn() + Send + Sync>> = None;
#[cfg(target_os = "macos")]
static mut GLOBAL_MODIFIERS: u64 = 0;
#[cfg(target_os = "macos")]
static mut GLOBAL_KEYCODE: u16 = 0;

// C callback function
#[cfg(target_os = "macos")]
extern "C" fn event_tap_callback(
    _proxy: *mut std::ffi::c_void,
    event_type: u32,
    event: *mut std::ffi::c_void,
    _user_info: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    unsafe {
        // Log every event we receive for debugging
        detailed_log("HOTKEY", &format!("Event received: type={}, event={:p}", event_type, event));

        // Check if this is a key down event
        if event_type != 10 { // kCGEventKeyDown = 10
            return event;
        }

        // Get event flags and keycode
        let flags = CGEventGetFlags(event);
        let keycode = CGEventGetIntegerValueField(event, 9); // kCGKeyboardEventKeycode = 9

        detailed_log("HOTKEY", &format!("Key event: flags={:#x}, keycode={}, target_flags={:#x}, target_keycode={}",
            flags, keycode, GLOBAL_MODIFIERS, GLOBAL_KEYCODE));

        // Check if this matches our hotkey
        let flags_match = (flags & GLOBAL_MODIFIERS) == GLOBAL_MODIFIERS;
        let keycode_match = keycode == GLOBAL_KEYCODE as i64;

        if flags_match && keycode_match {
            log(&format!("🔥 Hotkey triggered! flags: {:#x}, keycode: {}", flags, keycode));

            // Call our callback
            if let Some(ref callback) = GLOBAL_CALLBACK {
                detailed_log("HOTKEY", "Calling popup callback...");
                callback();
                detailed_log("HOTKEY", "Callback completed");
            } else {
                log_error("No callback registered for hotkey");
            }

            // Consume the event
            return std::ptr::null_mut();
        }

        // Pass through other events
        event
    }
}

// C API bindings for CGEventTap
#[cfg(target_os = "macos")]
extern "C" {
    fn CGEventTapCreate(
        tap: u32,           // CGEventTapLocation
        place: u32,         // CGEventTapPlacement
        options: u32,       // CGEventTapOptions
        events_of_interest: u64, // CGEventMask
        callback: extern "C" fn(*mut std::ffi::c_void, u32, *mut std::ffi::c_void, *mut std::ffi::c_void) -> *mut std::ffi::c_void,
        user_info: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;

    fn CFMachPortCreateRunLoopSource(
        allocator: *mut std::ffi::c_void,
        port: *mut std::ffi::c_void,
        order: i64,
    ) -> *mut std::ffi::c_void;

    fn CFRunLoopGetCurrent() -> *mut std::ffi::c_void;

    fn CFRunLoopAddSource(
        rl: *mut std::ffi::c_void,
        source: *mut std::ffi::c_void,
        mode: *mut std::ffi::c_void,
    );

    fn CGEventTapEnable(tap: *mut std::ffi::c_void, enable: bool);

    fn CGEventGetFlags(event: *mut std::ffi::c_void) -> u64;

    fn CGEventGetIntegerValueField(event: *mut std::ffi::c_void, field: u32) -> i64;

    static kCFRunLoopCommonModes: *mut std::ffi::c_void;
}

#[cfg(target_os = "macos")]
fn create_event_tap() -> Result<*mut std::ffi::c_void, String> {
    unsafe {
        let tap = CGEventTapCreate(
            1,    // kCGHIDEventTap
            0,    // kCGHeadInsertEventTap
            0,    // kCGEventTapOptionDefault
            1 << 10, // kCGEventKeyDown
            event_tap_callback,
            std::ptr::null_mut(),
        );

        if tap.is_null() {
            Err("Failed to create event tap".to_string())
        } else {
            Ok(tap)
        }
    }
}

#[cfg(target_os = "macos")]
fn create_runloop_source(tap: *mut std::ffi::c_void) -> Result<*mut std::ffi::c_void, String> {
    unsafe {
        let source = CFMachPortCreateRunLoopSource(std::ptr::null_mut(), tap, 0);
        if source.is_null() {
            Err("Failed to create run loop source".to_string())
        } else {
            Ok(source)
        }
    }
}

#[cfg(target_os = "macos")]
fn add_source_to_runloop(source: *mut std::ffi::c_void) {
    unsafe {
        let run_loop = CFRunLoopGetCurrent();
        CFRunLoopAddSource(run_loop, source, kCFRunLoopCommonModes);
    }
}

#[cfg(target_os = "macos")]
fn enable_event_tap(tap: *mut std::ffi::c_void) {
    unsafe {
        CGEventTapEnable(tap, true);
    }
}

/// Parse hotkey string into modifiers and keycode
#[cfg(target_os = "macos")]
fn parse_hotkey(hotkey_str: &str) -> Result<(u64, u16), String> {
    let parts: Vec<&str> = hotkey_str.split('+').collect();
    if parts.is_empty() {
        return Err("Invalid hotkey format".to_string());
    }

    let mut modifiers = 0u64;
    let key_part = parts.last().unwrap();

    // Parse modifiers (CGEventFlags values)
    for part in &parts[..parts.len() - 1] {
        match part.to_lowercase().as_str() {
            "cmd" | "command" => modifiers |= 0x100000, // kCGEventFlagMaskCommand
            "ctrl" | "control" => modifiers |= 0x040000, // kCGEventFlagMaskControl
            "alt" | "option" => modifiers |= 0x080000, // kCGEventFlagMaskAlternate
            "shift" => modifiers |= 0x020000, // kCGEventFlagMaskShift
            _ => return Err(format!("Unknown modifier: {}", part)),
        }
    }

    // Parse key (macOS virtual keycodes)
    let keycode = match key_part.to_lowercase().as_str() {
        "a" => 0x00,
        "s" => 0x01,
        "d" => 0x02,
        "f" => 0x03,
        "h" => 0x04,
        "g" => 0x05,
        "z" => 0x06,
        "x" => 0x07,
        "c" => 0x08,
        "v" => 0x09,
        "b" => 0x0B,
        "q" => 0x0C,
        "w" => 0x0D,
        "e" => 0x0E,
        "r" => 0x0F,
        "y" => 0x10,
        "t" => 0x11,
        "space" => 0x31,
        "return" | "enter" => 0x24,
        "tab" => 0x30,
        "escape" | "esc" => 0x35,
        _ => return Err(format!("Unknown key: {}", key_part)),
    };

    Ok((modifiers, keycode))
}

/// Check if app has accessibility permissions
#[cfg(target_os = "macos")]
fn check_accessibility_permission() -> bool {
    extern "C" {
        fn AXIsProcessTrusted() -> bool;
    }
    unsafe { AXIsProcessTrusted() }
}

// Public interface for checking accessibility
pub fn has_accessibility_permission() -> bool {
    #[cfg(target_os = "macos")]
    return check_accessibility_permission();

    #[cfg(not(target_os = "macos"))]
    true
}

// Request accessibility permissions
pub fn request_accessibility_permission() {
    #[cfg(target_os = "macos")]
    {
        // Open System Preferences to the Accessibility pane
        let _ = std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn();

        log("Opening System Preferences > Privacy & Security > Accessibility");
        log("Please add HookAnchor to the list of allowed apps and try again");
    }
}
