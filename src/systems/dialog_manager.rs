//! Non-blocking Dialog Manager
//!
//! Manages HookAnchorDialog subprocesses for GUI contexts where blocking is not acceptable.
//! For blocking dialog functions (CLI, initialization), see utils/dialog.rs instead.

use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use crate::prelude::*;

/// Handle to an active dialog subprocess
pub struct DialogHandle {
    child: Child,
    _dialog_id: u64,
}

static mut NEXT_DIALOG_ID: u64 = 1;

/// Spawn a non-blocking dialog subprocess
///
/// # Arguments
/// - `spec_strings`: Dialog specification (see dialog_viewer.rs for format)
/// - `position`: Optional (x, y, screen_width) to position dialog relative to parent window
///
/// # Returns
/// DialogHandle to track the subprocess, or None if spawn failed
pub fn spawn_dialog(
    spec_strings: Vec<String>,
    position: Option<(f32, f32, f32)>,
) -> Option<DialogHandle> {
    let exe_dir = crate::utils::get_binary_dir();
    let dialog_path = exe_dir.join("HookAnchorDialog");

    if !dialog_path.exists() {
        log_error(&format!("Dialog binary not found: {:?}", dialog_path));
        return None;
    }

    // Build command with spec strings
    let mut cmd = Command::new(&dialog_path);
    for spec in &spec_strings {
        cmd.arg("--spec");
        cmd.arg(spec);
    }

    // Add position argument if provided
    if let Some((popup_x, popup_y, _popup_width)) = position {
        // Position dialog 25px down and 25px right from popup's top-left corner
        let dialog_x = popup_x + 25.0;
        let dialog_y = popup_y + 25.0;
        log(&format!(
            "DIALOG_POS: offset 25px from popup: popup_x={}, popup_y={}, dialog_x={}, dialog_y={}",
            popup_x, popup_y, dialog_x, dialog_y
        ));
        cmd.arg("--position");
        cmd.arg(format!("{},{}", dialog_x, dialog_y));
    }

    // Configure stdout to be captured so we can read the JSON result
    cmd.stdout(Stdio::piped());

    // Spawn subprocess
    match cmd.spawn() {
        Ok(child) => {
            let dialog_id = unsafe {
                let id = NEXT_DIALOG_ID;
                NEXT_DIALOG_ID += 1;
                id
            };
            log(&format!(
                "DIALOG_MANAGER: Spawned dialog #{} (pid: {:?})",
                dialog_id,
                child.id()
            ));
            Some(DialogHandle {
                child,
                _dialog_id: dialog_id,
            })
        }
        Err(e) => {
            log_error(&format!("Failed to spawn dialog subprocess: {}", e));
            None
        }
    }
}

/// Poll a dialog subprocess to check if it's finished (non-blocking)
///
/// # Returns
/// - `Some(Ok(result))`: Dialog finished successfully, contains button/field values
/// - `Some(Err(error))`: Dialog process failed or output parsing failed
/// - `None`: Dialog still running, call again later
pub fn poll_dialog(handle: &mut DialogHandle) -> Option<Result<HashMap<String, String>, String>> {
    // Non-blocking check if subprocess finished
    match handle.child.try_wait() {
        Ok(Some(status)) => {
            log(&format!(
                "DIALOG_MANAGER: Dialog finished with status: {}",
                status
            ));

            // Read stdout to get JSON result
            if let Some(mut stdout) = handle.child.stdout.take() {
                use std::io::Read;
                let mut json_str = String::new();
                match stdout.read_to_string(&mut json_str) {
                    Ok(_) => {
                        log(&format!(
                            "DIALOG_MANAGER: Read {} bytes from stdout",
                            json_str.len()
                        ));
                        match serde_json::from_str::<HashMap<String, String>>(&json_str) {
                            Ok(result) => {
                                log(&format!(
                                    "DIALOG_MANAGER: Parsed result: {:?}",
                                    result
                                ));
                                Some(Ok(result))
                            }
                            Err(e) => {
                                let error = format!("Failed to parse dialog JSON: {}", e);
                                log_error(&error);
                                Some(Err(error))
                            }
                        }
                    }
                    Err(e) => {
                        let error = format!("Failed to read dialog stdout: {}", e);
                        log_error(&error);
                        Some(Err(error))
                    }
                }
            } else {
                let error = "Dialog stdout not available (not piped?)".to_string();
                log_error(&error);
                Some(Err(error))
            }
        }
        Ok(None) => {
            // Still running
            None
        }
        Err(e) => {
            let error = format!("Error checking dialog subprocess: {}", e);
            log_error(&error);
            Some(Err(error))
        }
    }
}
