//! Dialog2 - New non-blocking dialog system
//!
//! This module provides dialog functions that use the external HookAnchorDialog binary.
//! All functions are non-blocking except `blocking_dialog()` which is for CLI contexts.
//!
//! ## GUI Context Functions (non-blocking)
//!
//! - `fatal_error()`: Shows error dialog then exits app immediately
//! - `warning()`: Shows warning dialog, continues execution
//! - `dialog()`: Shows dialog with callback, requires popup polling loop
//!
//! ## CLI Context Functions (blocking)
//!
//! - `blocking_dialog()`: Shows dialog and blocks until user responds

use std::collections::HashMap;

/// Show fatal error dialog and exit immediately (non-blocking)
///
/// Spawns the error dialog then exits the application. The dialog will show
/// briefly before the app terminates. Use this for unrecoverable errors.
///
/// # Example
/// ```no_run
/// crate::utils::dialog2::fatal_error("Cannot load config file");
/// // App exits here
/// ```
pub fn fatal_error(message: &str) -> ! {
    crate::utils::log_error(&format!("FATAL: {}", message));

    let _ = crate::systems::spawn_dialog(
        vec![
            "=Fatal Error".to_string(),
            "#🛑 Fatal Error".to_string(),
            format!("'{}", message),
            "!Exit HookAnchor".to_string(),
        ],
        None, // No positioning needed
    );

    std::process::exit(1);
}

/// Show warning dialog (non-blocking)
///
/// Spawns a warning dialog and continues execution immediately.
/// The dialog will stay visible until the user dismisses it.
///
/// # Example
/// ```no_run
/// crate::utils::dialog2::warning("Using default value");
/// // Execution continues immediately
/// ```
pub fn warning(message: &str) {
    crate::utils::log(&format!("WARNING: {}", message));

    let _ = crate::systems::spawn_dialog(
        vec![
            "=Warning".to_string(),
            "#⚠️  Warning".to_string(),
            format!("'{}", message),
            "!OK".to_string(),
        ],
        None,
    );
}

/// Spawn a dialog (non-blocking, returns handle for popup to manage)
///
/// **IMPORTANT**: This function requires the popup update loop to be running.
/// The dialog is spawned immediately but you must:
/// 1. Store the DialogHandle in popup's external_dialog_state
/// 2. Poll it via poll_external_dialog() in the update loop
/// 3. Execute your callback when the dialog completes
///
/// Do NOT use this in CLI or non-GUI contexts - use `blocking_dialog()` instead.
///
/// # Arguments
/// - `spec`: Dialog specification strings (see dialog_viewer.rs for format)
/// - `position`: Optional (x, y, screen_width) for positioning
///
/// # Returns
/// DialogHandle that popup will poll, or None if spawn failed
///
/// # Example
/// ```no_run
/// // Spawn the dialog
/// if let Some(handle) = crate::utils::dialog2::dialog(
///     vec!["=Confirm".to_string(), "'Are you sure?".to_string(), "!Yes".to_string()],
///     Some((100.0, 100.0, 1920.0))
/// ) {
///     // Store handle+callback in popup's external_dialog_state
///     self.external_dialog_state = Some(ExternalDialogState {
///         handle,
///         context: my_context,
///         callback: Box::new(|result| { /* ... */ }),
///     });
/// }
/// ```
pub fn dialog(
    spec: Vec<String>,
    position: Option<(f32, f32, f32)>,
) -> Option<crate::systems::DialogHandle> {
    crate::systems::spawn_dialog(spec, position)
}

/// Show dialog and block until user responds (blocking - for CLI only)
///
/// **IMPORTANT**: This function BLOCKS the current thread until the user
/// responds to the dialog. Only use this in CLI contexts where blocking
/// is acceptable. For GUI contexts, use `dialog()` instead.
///
/// # Arguments
/// - `spec`: Dialog specification strings
///
/// # Returns
/// HashMap with dialog results ("exit" key contains button clicked, plus any field values)
///
/// # Example
/// ```no_run
/// let result = crate::utils::dialog2::blocking_dialog(vec![
///     "=Confirm".to_string(),
///     "'Delete this file?".to_string(),
///     "!Yes".to_string(),
///     "!No".to_string()
/// ]);
/// if result.get("exit") == Some(&"Yes".to_string()) {
///     // User confirmed
/// }
/// ```
pub fn blocking_dialog(spec: Vec<String>) -> HashMap<String, String> {
    use std::process::Command;

    let exe_dir = crate::utils::get_binary_dir();
    let dialog_path = exe_dir.join("HookAnchorDialog");

    if !dialog_path.exists() {
        crate::utils::log_error(&format!("Dialog binary not found: {:?}", dialog_path));
        let mut result = HashMap::new();
        result.insert("exit".to_string(), "OK".to_string());
        return result;
    }

    let mut cmd = Command::new(&dialog_path);
    for s in &spec {
        cmd.arg("--spec");
        cmd.arg(s);
    }

    // Block and wait for output
    match cmd.output() {
        Ok(output) => {
            if let Ok(json_str) = String::from_utf8(output.stdout) {
                if let Ok(result) = serde_json::from_str::<HashMap<String, String>>(&json_str) {
                    return result;
                }
            }

            // Fallback
            let mut result = HashMap::new();
            result.insert("exit".to_string(), "OK".to_string());
            result
        }
        Err(e) => {
            crate::utils::log_error(&format!("Failed to run dialog: {}", e));
            let mut result = HashMap::new();
            result.insert("exit".to_string(), "OK".to_string());
            result
        }
    }
}
