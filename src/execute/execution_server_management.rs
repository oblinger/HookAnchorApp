//! Server Management Module
//!
//! Handles persistent command server lifecycle management including:
//! - Starting servers via Terminal for proper environment
//! - PID tracking and process management
//! - Automatic server restart when needed

use std::sync::atomic::{AtomicBool, Ordering};
use crate::prelude::*;

/// Global flag to track if server availability has been checked this session
static SERVER_CHECKED: AtomicBool = AtomicBool::new(false);

/// Check if a process with the given PID is still running
pub(crate) fn is_process_alive(pid: u32) -> bool {
    unsafe {
        // Use kill with signal 0 to test if process exists without actually sending a signal
        libc::kill(pid as i32, 0) == 0
    }
}

/// Start the command server if needed, with fast session-based caching
pub(crate) fn start_server_if_needed() -> Result<(), Box<dyn std::error::Error>> {
    // Fast path - already verified this session
    if SERVER_CHECKED.load(Ordering::Relaxed) {
        return Ok(());
    }

    // Log file size check is done when popup opens, not here

    detailed_log("SERVER_MGR", "Checking if command server is needed");

    // Check for socket file - if it exists, server is likely running
    let socket_path = crate::core::get_config_dir().join("execution_server.sock");
    let socket_exists = socket_path.exists();

    // Check PID from state.json
    let state = crate::core::data::get_state();
    if let Some(pid) = state.server_pid {
        if is_process_alive(pid) && socket_exists {
            // Both PID alive AND socket exists = server is truly running
            detailed_log("SERVER_MGR", &format!("Server running with PID: {} and socket exists", pid));
            SERVER_CHECKED.store(true, Ordering::Relaxed);
            return Ok(());
        } else if is_process_alive(pid) && !socket_exists {
            // PID alive but no socket = zombie server, kill it
            detailed_log("SERVER_MGR", &format!("Server PID {} alive but socket missing, killing zombie", pid));
            unsafe {
                libc::kill(pid as i32, libc::SIGKILL);
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
            // Clear stale PID
            let mut state = crate::core::data::get_state();
            state.server_pid = None;
            let _ = crate::core::data::set_state(&state);
        } else {
            detailed_log("SERVER_MGR", &format!("Server PID {} no longer running", pid));
            // Clear stale PID
            let mut state = crate::core::data::get_state();
            state.server_pid = None;
            let _ = crate::core::data::set_state(&state);
        }
    }
    
    // Start new server via Terminal for proper environment
    detailed_log("SERVER_MGR", "Starting new command server via Terminal");
    start_server_via_terminal()?;
    
    // Wait for startup and verify PID is saved
    detailed_log("SERVER_MGR", "Waiting for server startup");
    let start_time = std::time::Instant::now();
    let max_wait = std::time::Duration::from_secs(5);
    
    // Poll for server to be ready
    loop {
        if start_time.elapsed() > max_wait {
            detailed_log("SERVER_MGR", "Server startup timeout - failed to start in 5 seconds");
            return Err("Server startup timeout".into());
        }
        
        // Check if server PID is now available
        let new_state = crate::core::data::get_state();
        if let Some(pid) = new_state.server_pid {
            if is_process_alive(pid) {
                detailed_log("SERVER_MGR", &format!("Server started successfully with PID: {}", pid));
                SERVER_CHECKED.store(true, Ordering::Relaxed);
                return Ok(());
            }
        }
        
        // Brief sleep before next check
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

/// Start command server - either via Terminal window or headless based on config
pub(crate) fn start_server_via_terminal() -> Result<(), Box<dyn std::error::Error>> {
    // Check for server startup lock to prevent multiple simultaneous starts
    let lock_path = std::path::Path::new("/tmp/hookanchor_server_starting.lock");
    if lock_path.exists() {
        // Check if lock is stale (older than 30 seconds)
        if let Ok(metadata) = std::fs::metadata(lock_path) {
            if let Ok(created) = metadata.created() {
                let elapsed = created.elapsed().unwrap_or(std::time::Duration::from_secs(0));
                if elapsed > std::time::Duration::from_secs(30) {
                    detailed_log("SERVER_MGR", "Removing stale server startup lock");
                    let _ = std::fs::remove_file(lock_path);
                } else {
                    detailed_log("SERVER_MGR", "Server startup already in progress, skipping");
                    return Ok(());
                }
            }
        }
    }

    // Create lock file
    std::fs::write(lock_path, std::process::id().to_string())?;

    // Get the directory of the current binary, then use the 'ha' binary in that directory
    // This ensures we always use 'ha' for the server, regardless of which binary is currently running
    let current_binary = crate::utils::get_binary_path()
        .ok_or("Binary path not initialized")?;
    let binary_dir = current_binary.parent()
        .ok_or("Could not get binary directory")?;
    let ha_path = binary_dir.join("ha");

    // Check config for terminal mode
    let show_terminal = crate::core::data::get_config()
        .popup_settings
        .show_command_server_terminal
        .unwrap_or(false);

    if show_terminal {
        // Terminal mode: AppleScript to start server in Terminal with login shell
        let escaped_path = ha_path.display().to_string().replace("\"", "\\\"");
        let script = format!(
            r#"tell application "Terminal" to do script "cd ~ && \"{}\" --start-server-daemon""#,
            escaped_path
        );

        detailed_log("SERVER_MGR", "Starting server via Terminal with AppleScript");
        detailed_log("SERVER_MGR", &format!("AppleScript command: {}", script));

        let output = std::process::Command::new("osascript")
            .args(["-e", &script])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("AppleScript failed: {}", stderr).into());
        }

        detailed_log("SERVER_MGR", "Terminal server start initiated successfully");
    } else {
        // Headless mode: spawn directly with output redirected to log file
        detailed_log("SERVER_MGR", "Starting server in headless mode");

        // Open log file for appending stdout/stderr
        let log_path = crate::utils::expand_tilde(crate::utils::logging::DEFAULT_LOG_PATH);
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        // Clone for stderr
        let log_file_stderr = log_file.try_clone()?;

        // Spawn the server process with output redirected to log file
        let child = std::process::Command::new(&ha_path)
            .arg("--start-server-daemon")
            .current_dir(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .stdout(std::process::Stdio::from(log_file))
            .stderr(std::process::Stdio::from(log_file_stderr))
            .stdin(std::process::Stdio::null())
            .spawn()?;

        detailed_log("SERVER_MGR", &format!("Headless server spawned with PID: {}", child.id()));
    }

    // Remove lock file after a delay
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(5000));
        let _ = std::fs::remove_file("/tmp/hookanchor_server_starting.lock");
    });

    Ok(())
}

/// Kill existing command server if running
pub fn kill_existing_server() -> Result<(), Box<dyn std::error::Error>> {
    let state = crate::core::data::get_state();
    if let Some(pid) = state.server_pid {
        if is_process_alive(pid) {
            detailed_log("SERVER_MGR", &format!("Killing existing server PID: {}", pid));
            
            unsafe {
                // Send SIGTERM first (graceful shutdown)
                if libc::kill(pid as i32, libc::SIGTERM) == 0 {
                    // Wait a bit for graceful shutdown
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    
                    // If still running, force kill
                    if is_process_alive(pid) {
                        libc::kill(pid as i32, libc::SIGKILL);
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }
                }
            }
        }
        
        // Clear PID from state regardless
        let mut state = crate::core::data::get_state();
        state.server_pid = None;
        crate::core::data::set_state(&state)?;
    }
    
    // Reset the checked flag so next command will verify server
    SERVER_CHECKED.store(false, Ordering::Relaxed);
    Ok(())
}

/// Reset the server checked flag to force re-check on next call
pub(crate) fn reset_server_check() {
    SERVER_CHECKED.store(false, Ordering::Relaxed);
}

