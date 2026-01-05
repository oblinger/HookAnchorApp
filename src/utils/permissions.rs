//! macOS permissions checking
//!
//! This module checks for required macOS permissions and reports any missing ones.

use std::process::Command;

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

    msg.push_str("Add HookAnchor (or the 'ha' binary) to each list above.\n\n");
    msg.push_str("To disable this check, set 'skip_permissions_check: true' in config.yaml");

    msg
}
