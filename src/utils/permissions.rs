//! macOS permissions checking
//!
//! This module checks for required macOS permissions and reports any missing ones.

use std::process::Command;

#[cfg(target_os = "macos")]
use std::ffi::c_void;

#[cfg(target_os = "macos")]
#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrustedWithOptions(options: *const c_void) -> bool;
}

#[cfg(target_os = "macos")]
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFDictionaryCreate(
        allocator: *const c_void,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: isize,
        key_callbacks: *const c_void,
        value_callbacks: *const c_void,
    ) -> *const c_void;
    fn CFRelease(cf: *const c_void);

    static kCFBooleanTrue: *const c_void;
    static kCFTypeDictionaryKeyCallBacks: c_void;
    static kCFTypeDictionaryValueCallBacks: c_void;
}

// This is defined in HIServices but we link via ApplicationServices
#[cfg(target_os = "macos")]
extern "C" {
    static kAXTrustedCheckOptionPrompt: *const c_void;
}

/// Represents a permission that may be required
#[derive(Debug, Clone)]
pub struct PermissionInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub settings_path: &'static str,
    pub is_granted: bool,
}

/// Check if Accessibility permission is granted
/// Uses osascript to try sending a keystroke - if it fails with error 1002, permission is denied
fn check_accessibility_permission() -> bool {
    // Try a harmless keystroke test
    let output = Command::new("osascript")
        .args(["-e", "tell application \"System Events\" to key code 0"])
        .output();

    match output {
        Ok(result) => {
            // Check stderr for the specific error
            let stderr = String::from_utf8_lossy(&result.stderr);
            !stderr.contains("1002") && !stderr.contains("not allowed")
        }
        Err(_) => false,
    }
}

/// Check if Automation permission is granted for System Events
/// This is related to but separate from Accessibility
fn check_automation_permission() -> bool {
    // Try to get the name of the frontmost app - requires Automation permission
    let output = Command::new("osascript")
        .args(["-e", "tell application \"System Events\" to get name of first process whose frontmost is true"])
        .output();

    match output {
        Ok(result) => {
            let stderr = String::from_utf8_lossy(&result.stderr);
            // Error -1743 or "not allowed assistive access" indicates permission denied
            !stderr.contains("-1743") && !stderr.contains("not allowed")
        }
        Err(_) => false,
    }
}

/// Check all required permissions and return a list of results
pub fn check_all_permissions() -> Vec<PermissionInfo> {
    vec![
        PermissionInfo {
            name: "Accessibility",
            description: "Required for sending keystrokes (1Password integration, etc.)",
            settings_path: "System Settings → Privacy & Security → Accessibility",
            is_granted: check_accessibility_permission(),
        },
        PermissionInfo {
            name: "Automation (System Events)",
            description: "Required for controlling other applications",
            settings_path: "System Settings → Privacy & Security → Automation",
            is_granted: check_automation_permission(),
        },
    ]
}

/// Check permissions and return list of missing ones
pub fn get_missing_permissions() -> Vec<PermissionInfo> {
    check_all_permissions()
        .into_iter()
        .filter(|p| !p.is_granted)
        .collect()
}

/// Format missing permissions into a user-friendly message
pub fn format_missing_permissions_message(missing: &[PermissionInfo]) -> String {
    if missing.is_empty() {
        return String::new();
    }

    let mut msg = String::from("HookAnchor is missing the following permissions:\n\n");

    for perm in missing {
        msg.push_str(&format!("• {} - {}\n", perm.name, perm.description));
        msg.push_str(&format!("  → {}\n\n", perm.settings_path));
    }

    // Get the binary directory to show exact paths
    if let Some(binary_path) = crate::utils::get_binary_path() {
        if let Some(binary_dir) = binary_path.parent() {
            msg.push_str("Add these binaries using the + button:\n");
            msg.push_str(&format!("  • {}/HookAnchorPopupServer\n", binary_dir.display()));
            msg.push_str(&format!("  • {}/ha\n\n", binary_dir.display()));
        }
    } else {
        msg.push_str("Add HookAnchor binaries (HookAnchorPopupServer and ha) to the list.\n\n");
    }

    msg.push_str("To disable this check, set 'skip_permissions_check: true' in config.yaml");

    msg
}

/// Format Accessibility permission warning message
/// Used when AXIsProcessTrustedWithOptions returns false
pub fn format_accessibility_warning() -> String {
    let mut msg = String::from("HookAnchor needs Accessibility permission.\n\n");

    msg.push_str("This is required for:\n");
    msg.push_str("• Sending keystrokes (1Password integration, etc.)\n");
    msg.push_str("• Controlling other applications\n\n");

    // Get the binary directory to show exact paths
    if let Some(binary_path) = crate::utils::get_binary_path() {
        if let Some(binary_dir) = binary_path.parent() {
            msg.push_str("Add these binaries in System Settings → Privacy & Security → Accessibility:\n");
            msg.push_str(&format!("  • {}/HookAnchorPopupServer\n", binary_dir.display()));
            msg.push_str(&format!("  • {}/ha\n\n", binary_dir.display()));
        }
    } else {
        msg.push_str("Add HookAnchor binaries in System Settings → Privacy & Security → Accessibility.\n\n");
    }

    msg.push_str("Tip: Drag the binaries from Finder into the Settings list.\n\n");
    msg.push_str("To disable this check, set 'skip_permissions_check: true' in config.yaml");

    msg
}

/// Request Accessibility permission from macOS
/// This triggers the system prompt and adds the app to the Accessibility list
/// Returns true if permission is already granted, false if user needs to grant it
#[cfg(target_os = "macos")]
pub fn request_accessibility_permission() -> bool {
    unsafe {
        // Create options dictionary with kAXTrustedCheckOptionPrompt = true
        // This tells macOS to show the permission prompt if not already granted
        let keys: [*const c_void; 1] = [kAXTrustedCheckOptionPrompt];
        let values: [*const c_void; 1] = [kCFBooleanTrue];

        let options = CFDictionaryCreate(
            std::ptr::null(),
            keys.as_ptr(),
            values.as_ptr(),
            1,
            &kCFTypeDictionaryKeyCallBacks as *const _ as *const c_void,
            &kCFTypeDictionaryValueCallBacks as *const _ as *const c_void,
        );

        let result = AXIsProcessTrustedWithOptions(options);

        if !options.is_null() {
            CFRelease(options);
        }

        result
    }
}

#[cfg(not(target_os = "macos"))]
pub fn request_accessibility_permission() -> bool {
    true // Non-macOS platforms don't need this
}

/// Open the Accessibility settings pane in System Settings
pub fn open_accessibility_settings() {
    let _ = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn();
}

/// Reveal the HookAnchor binaries in Finder for easy adding to Accessibility
pub fn reveal_binaries_in_finder() {
    if let Some(binary_path) = crate::utils::get_binary_path() {
        if let Some(binary_dir) = binary_path.parent() {
            // Reveal the popup server binary
            let popup_server = binary_dir.join("HookAnchorPopupServer");
            if popup_server.exists() {
                let _ = Command::new("open")
                    .args(["-R", &popup_server.to_string_lossy()])
                    .spawn();
            }
        }
    }
}
