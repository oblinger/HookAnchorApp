//! Server restart operations
//!
//! This module provides centralized functions for restarting various HookAnchor servers.
//! All restart code paths should use these functions to avoid duplication.

use std::process::Command;
use std::path::PathBuf;
use std::fs;
use crate::prelude::*;

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
                    detailed_log("RESTART", &format!("Killed processes matching: {}", pattern));
                }
            }
            Err(e) => {
                detailed_log("RESTART", &format!("Failed to pkill {}: {}", pattern, e));
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

    print_and_log(&format!("Spawned popup_server: {}", popup_server_path.display()));
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
            detailed_log("RESTART", "Killed existing popup_server processes");
        }
        Ok(false) => {
            detailed_log("RESTART", "No popup_server processes found");
        }
        Err(e) => {
            log_error(&format!("Failed to kill popup_server: {}", e));
            // Continue anyway - maybe none were running
        }
    }

    // Brief wait for processes to fully terminate
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Start new
    match start_popup_server() {
        Ok(path) => {
            detailed_log("RESTART", &format!("Started popup_server: {}", path.display()));
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
        detailed_log("RESTART", "Removed server startup lock file before restart");
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
    print_and_log("🔄 Step 1/3: Restarting command server...");
    if let Err(e) = restart_command_server() {
        log_error(&format!("Command server restart failed: {}", e));
        return Err(format!("Command server restart failed: {}", e));
    }
    print_and_log("  ✅ Command server restarted");

    // Step 2: Rescan filesystem
    print_and_log("\n📁 Step 2/3: Rescanning filesystem...");
    // Brief wait for server to be ready
    std::thread::sleep(std::time::Duration::from_millis(1000));

    match run_rescan() {
        Ok(true) => {
            print_and_log("  ✅ Filesystem rescan completed");
        }
        Ok(false) => {
            print_and_log("  ⚠️  Rescan command returned non-zero status");
        }
        Err(e) => {
            log_error(&format!("Rescan failed: {}", e));
            return Err(format!("Rescan failed: {}", e));
        }
    }

    // Step 3: Restart popup server
    print_and_log("\n🔄 Step 3/3: Restarting popup server...");
    restart_popup_server()?;
    print_and_log("  ✅ Popup server restarted");

    Ok(())
}

// =============================================================================
// UNIFIED SERVER LIFECYCLE API
// =============================================================================

/// Get the execution server socket path
///
/// Returns the path to the Unix domain socket used by the command execution server.
/// Uses config directory to avoid hardcoded user paths.
pub fn get_execution_server_socket_path() -> PathBuf {
    let config_dir = crate::core::get_config_dir();
    config_dir.join("execution_server.sock")
}

/// Get the popup socket path
///
/// Returns the path to the Unix domain socket used by the popup server.
pub fn get_popup_socket_path() -> PathBuf {
    PathBuf::from("/tmp/hookanchor_popup.sock")
}

/// Stop all HookAnchor servers (popup + command)
///
/// Returns (popup_killed, command_killed) indicating which servers were stopped.
/// This is the safe way to stop all servers before file operations that could
/// cause race conditions.
pub fn stop_all_servers() -> Result<(bool, bool), String> {
    print_and_log("🛑 Stopping all servers...");

    // Stop popup server
    let popup_killed = match kill_popup_servers() {
        Ok(killed) => {
            if killed {
                print_and_log("  ✓ Popup server stopped");
            } else {
                print_and_log("  ℹ No popup server running");
            }
            killed
        }
        Err(e) => {
            log_error(&format!("  ✗ Failed to stop popup server: {}", e));
            false
        }
    };

    // Stop command server
    let command_killed = match crate::execute::kill_existing_server() {
        Ok(()) => {
            print_and_log("  ✓ Command server stopped");
            true
        }
        Err(e) => {
            log_error(&format!("  ✗ Failed to stop command server: {}", e));
            false
        }
    };

    // Wait for processes to fully terminate
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Clean up socket files
    let popup_socket = get_popup_socket_path();
    if popup_socket.exists() {
        if let Err(e) = fs::remove_file(&popup_socket) {
            detailed_log("RESTART", &format!("Failed to remove popup socket: {}", e));
        }
    }

    let command_socket = get_execution_server_socket_path();
    if command_socket.exists() {
        if let Err(e) = fs::remove_file(&command_socket) {
            detailed_log("RESTART", &format!("Failed to remove command socket: {}", e));
        }
    }

    Ok((popup_killed, command_killed))
}

/// Start all HookAnchor servers (popup + command)
///
/// Returns Ok if both servers started successfully, Err if either failed.
/// Verifies that both servers are actually running after startup.
pub fn start_all_servers() -> Result<(), String> {
    print_and_log("🚀 Starting all servers...");

    // Start popup server
    match start_popup_server() {
        Ok(path) => {
            print_and_log(&format!("  ✓ Popup server started: {}", path.display()));
        }
        Err(e) => {
            return Err(format!("Failed to start popup server: {}", e));
        }
    }

    // Start command server (restart=false means just ensure it's running)
    match crate::execute::activate_command_server(false) {
        Ok(()) => {
            print_and_log("  ✓ Command server started");
        }
        Err(e) => {
            return Err(format!("Failed to start command server: {}", e));
        }
    }

    // Verify both servers are running by checking socket files exist
    std::thread::sleep(std::time::Duration::from_millis(500));

    let popup_socket = get_popup_socket_path();
    let command_socket = get_execution_server_socket_path();

    let mut errors = Vec::new();
    if !popup_socket.exists() {
        errors.push("Popup socket not found after startup");
    }
    if !command_socket.exists() {
        errors.push("Command socket not found after startup");
    }

    if !errors.is_empty() {
        return Err(format!("Server verification failed: {}", errors.join(", ")));
    }

    print_and_log("✅ All servers started and verified");
    Ok(())
}

/// Stop and restart all HookAnchor servers
///
/// This is the safest way to restart everything - stops all servers first,
/// waits for clean shutdown, then starts everything fresh.
///
/// Use this when you need to ensure no race conditions during file operations.
pub fn restart_all_servers() -> Result<(), String> {
    print_and_log("🔄 Restarting all servers (including supervisor)...");

    // Stop everything first
    stop_all_servers()?;

    // Brief additional wait to ensure everything is clean
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Restart the HookAnchor supervisor (Swift GUI app)
    restart_supervisor()?;

    // Start Rust servers
    start_all_servers()?;

    print_and_log("✅ All servers restarted successfully");
    Ok(())
}

/// Restart the HookAnchor supervisor (Swift GUI application)
fn restart_supervisor() -> Result<(), String> {
    use std::process::Command;

    print_and_log("🔄 Restarting HookAnchor supervisor...");

    // Kill existing supervisor process
    let kill_result = Command::new("killall")
        .arg("HookAnchor")
        .output();

    match kill_result {
        Ok(output) if output.status.success() => {
            print_and_log("  ✓ Old supervisor stopped");
        }
        Ok(_) => {
            // killall returns non-zero if process not found - that's fine
            print_and_log("  ℹ No supervisor was running");
        }
        Err(e) => {
            log_error(&format!("  ⚠ Failed to kill supervisor: {}", e));
            // Don't fail - maybe it wasn't running
        }
    }

    // Brief wait for process to fully terminate
    std::thread::sleep(std::time::Duration::from_millis(300));

    // Start new supervisor
    let start_result = Command::new("open")
        .arg("-a")
        .arg("HookAnchor")
        .spawn();

    match start_result {
        Ok(_) => {
            // Give it a moment to start
            std::thread::sleep(std::time::Duration::from_millis(500));
            print_and_log("  ✓ Supervisor restarted");
            Ok(())
        }
        Err(e) => {
            Err(format!("Failed to start supervisor: {}", e))
        }
    }
}
