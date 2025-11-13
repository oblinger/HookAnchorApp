//! OLD DIALOG SYSTEM - TO BE REMOVED
//!
//! Dialog Helper Functions (OLD BLOCKING SYSTEM)
//!
//! This module provides helper functions to launch the standalone HookAnchorDialog binary.
//! All functions are blocking - they wait for user interaction before returning/exiting.
//!
//! **DEPRECATED**: This is the old blocking dialog system. Use dialog2.rs instead.

use std::collections::HashMap;
use std::process::Command;

/// Show fatal error dialog and terminate (never returns)
/// Blocks until user clicks Exit, then panics
///
/// # Example
/// ```no_run
/// use crate::prelude::*;
/// fatal_error(&format!("Cannot load config: {}", path));
/// ```
pub fn fatal_error(message: &str) -> ! {
    log_error(&format!("FATAL: {}", message));

    launch_dialog_blocking(&[
        "=Fatal Error",
        "#🛑 Fatal Error",
        &format!("'{}", message),
        "!Exit"
    ]);

    panic!("Fatal error: {}", message);
}

/// Show warning dialog (blocking)
/// Blocks until user clicks OK, then continues execution
///
/// # Example
/// ```no_run
/// use crate::prelude::*;
/// warning(&format!("Using default value for invalid setting: {}", value));
/// ```
pub fn warning(message: &str) {
    log(&format!("WARNING: {}", message));

    launch_dialog_blocking(&[
        "=Warning",
        "#⚡ Warning",
        &format!("'{}", message),
        "!OK"
    ]);
}

/// Show general-purpose dialog (blocking)
/// Returns HashMap with "exit" key for button clicked, plus any input field values
///
/// # Example
/// ```no_run
/// use crate::prelude::*;
/// let result = dialog(&[
///     "=Confirm Delete",
///     "#Are you sure?",
///     "'This cannot be undone",
///     "!Yes",
///     "!No"
/// ]);
/// if result.get("exit") == Some(&"Yes".to_string()) {
///     // User confirmed
/// }
/// ```
pub fn dialog(specs: &[&str]) -> HashMap<String, String> {
    launch_dialog_blocking(specs)
}

/// Internal function to launch dialog binary and wait for result
fn launch_dialog_blocking(specs: &[&str]) -> HashMap<String, String> {
    let exe_dir = crate::utils::get_binary_dir();
    let dialog_path = exe_dir.join("HookAnchorDialog");

    if !dialog_path.exists() {
        log_error(&format!("Dialog binary not found: {:?}", dialog_path));

        // Return default result
        let mut result = HashMap::new();
        result.insert("exit".to_string(), "OK".to_string());
        return result;
    }

    let mut cmd = Command::new(&dialog_path);

    // Pass spec strings as multiple --spec arguments
    for spec in specs {
        cmd.arg("--spec");
        cmd.arg(spec);
    }

    // Wait for output (blocking)
    match cmd.output() {
        Ok(output) => {
            // Parse JSON result from stdout
            if let Ok(json_str) = String::from_utf8(output.stdout) {
                if let Ok(result) = serde_json::from_str::<HashMap<String, String>>(&json_str) {
                    return result;
                }
            }

            // Fallback: just return OK
            let mut result = HashMap::new();
            result.insert("exit".to_string(), "OK".to_string());
            result
        }
        Err(e) => {
            log_error(&format!("Failed to run dialog: {}", e));

            // Return default result
            let mut result = HashMap::new();
            result.insert("exit".to_string(), "OK".to_string());
            result
        }
    }
}
