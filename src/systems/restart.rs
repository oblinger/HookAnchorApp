//! Server restart operations
//!
//! This module provides centralized functions for restarting various HookAnchor servers.
//! All restart code paths should use these functions to avoid duplication.

use std::process::Command;
use std::path::PathBuf;

/// Kill all popup_server processes
///
/// Returns Ok(true) if processes were killed, Ok(false) if none found, Err on failure
pub fn kill_popup_servers() -> Result<bool, String> {
    // Kill both old and new popup server binary names
    // - popup_server (new binary name)
    // - HookAnchorPopupServer (old binary name)
    let patterns = vec!["popup_server", "HookAnchorPopupServer"];
    let mut any_killed = false;

    for pattern in patterns {
        match Command::new("pkill")
            .arg("-f")
            .arg(pattern)
            .output() {
            Ok(output) => {
                if output.status.success() {
                    any_killed = true;
                    crate::utils::detailed_log("RESTART", &format!("Killed processes matching: {}", pattern));
                }
            }
            Err(e) => {
                crate::utils::detailed_log("RESTART", &format!("Failed to pkill {}: {}", pattern, e));
            }
        }
    }

    Ok(any_killed)
}

/// Start popup_server as background process
///
/// Spawns popup_server directly as a background daemon process.
/// No Terminal tab needed - logs go to anchor.log.
///
/// Returns the path to the popup_server that was started
pub fn start_popup_server() -> Result<PathBuf, String> {
    // Get popup_server path
    let popup_server_path = get_popup_server_path()?;

    if !popup_server_path.exists() {
        return Err(format!("popup_server not found at: {}", popup_server_path.display()));
    }

    // Start directly as background process
    Command::new(&popup_server_path)
        .spawn()
        .map_err(|e| format!("Failed to start popup_server: {}", e))?;

    crate::utils::log(&format!("Spawned popup_server: {}", popup_server_path.display()));
    Ok(popup_server_path)
}

/// Get the path to popup_server binary
pub fn get_popup_server_path() -> Result<PathBuf, String> {
    let binary_path = crate::utils::get_binary_path()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .ok_or("Failed to determine binary directory")?;

    Ok(binary_path.join("popup_server"))
}

/// Restart popup server: kill existing and start new
///
/// This is the standard way to restart the popup server across all code paths.
pub fn restart_popup_server() -> Result<(), String> {
    // Kill existing
    match kill_popup_servers() {
        Ok(true) => {
            crate::utils::detailed_log("RESTART", "Killed existing popup_server processes");
        }
        Ok(false) => {
            crate::utils::detailed_log("RESTART", "No popup_server processes found");
        }
        Err(e) => {
            crate::utils::log_error(&format!("Failed to kill popup_server: {}", e));
            // Continue anyway - maybe none were running
        }
    }

    // Brief wait for processes to fully terminate
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Start new
    match start_popup_server() {
        Ok(path) => {
            crate::utils::detailed_log("RESTART", &format!("Started popup_server: {}", path.display()));
            Ok(())
        }
        Err(e) => Err(format!("Failed to start popup_server: {}", e))
    }
}

/// Restart command server
///
/// Wrapper around activate_command_server for consistency
pub fn restart_command_server() -> Result<(), Box<dyn std::error::Error>> {
    // Clear any stale lock file before restarting
    // This ensures a forced restart (like Cmd+B) always works even if a previous
    // startup attempt left a lock file behind
    let lock_path = std::path::Path::new("/tmp/hookanchor_server_starting.lock");
    if lock_path.exists() {
        let _ = std::fs::remove_file(lock_path);
        crate::utils::detailed_log("RESTART", "Removed server startup lock file before restart");
    }

    crate::execute::activate_command_server(true)
}

/// Run filesystem rescan via ha --rescan
///
/// Returns Ok(true) if scan succeeded, Ok(false) if failed, Err on execution failure
pub fn run_rescan() -> Result<bool, String> {
    let ha_binary = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|dir| dir.join("ha")))
        .unwrap_or_else(|| "ha".into());

    match Command::new(ha_binary)
        .arg("--rescan")
        .status() {
        Ok(status) => Ok(status.success()),
        Err(e) => Err(format!("Failed to execute rescan: {}", e))
    }
}

/// Full system restart: command server + rescan + popup server
///
/// This is what Command+B does - a complete rebuild of everything.
pub fn full_system_restart() -> Result<(), String> {
    // Step 1: Restart command server
    crate::utils::log("🔄 Step 1/3: Restarting command server...");
    if let Err(e) = restart_command_server() {
        crate::utils::log_error(&format!("Command server restart failed: {}", e));
        return Err(format!("Command server restart failed: {}", e));
    }
    crate::utils::log("  ✅ Command server restarted");

    // Step 2: Rescan filesystem
    crate::utils::log("\n📁 Step 2/3: Rescanning filesystem...");
    // Brief wait for server to be ready
    std::thread::sleep(std::time::Duration::from_millis(1000));

    match run_rescan() {
        Ok(true) => {
            crate::utils::log("  ✅ Filesystem rescan completed");
        }
        Ok(false) => {
            crate::utils::log("  ⚠️  Rescan command returned non-zero status");
        }
        Err(e) => {
            crate::utils::log_error(&format!("Rescan failed: {}", e));
            return Err(format!("Rescan failed: {}", e));
        }
    }

    // Step 3: Restart popup server
    crate::utils::log("\n🔄 Step 3/3: Restarting popup server...");
    restart_popup_server()?;
    crate::utils::log("  ✅ Popup server restarted");

    Ok(())
}
