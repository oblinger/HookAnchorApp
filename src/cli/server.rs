//! Server Commands
//!
//! Functions for managing the HookAnchor server processes.

use crate::utils::logging::print;
use crate::prelude::*;

/// Start/restart the command server (--start-server).
pub fn run_start_server() {
    detailed_log("SYSTEM", &format!("Restarting command server..."));

    // Restart the server (kill existing and start new)
    match crate::execute::activate_command_server(true) {
        Ok(()) => {
            print("Command server restart initiated");
            print("The server will start with full shell environment in a few seconds");
        }
        Err(e) => {
            log_error(&format!("Failed to restart server: {}", e));
            print(&format!("Failed to restart server: {}", e));
            std::process::exit(1);
        }
    }
}

/// Internal daemon mode - starts persistent server (--start-server-daemon).
pub fn run_start_server_daemon() {
    // Config will be initialized automatically by get_config() when needed
    // No need to initialize it explicitly here

    detailed_log("SYSTEM", &format!("Starting command server daemon..."));

    // Change to home directory
    if let Ok(home) = std::env::var("HOME") {
        if let Err(e) = std::env::set_current_dir(&home) {
            log_error(&format!("Could not change to home directory: {}", e));
        }
    }

    // Clean up any existing socket file (using centralized path function)
    let socket_path = crate::systems::get_execution_server_socket_path();
    let _ = std::fs::remove_file(&socket_path);

    // Run the command server (this never returns)
    match crate::execute::run_command_server() {
        Ok(_) => {
            // Should never reach here
        }
        Err(e) => {
            log_error(&format!("Failed to start persistent server: {}", e));
            std::process::exit(1);
        }
    }
}

/// Check for hung processes (--process-health).
pub fn run_process_health() {
    print("Checking process health...");
    crate::utils::subprocess::check_system_health();
    print("Health check complete. See debug logs for details.");
}

/// Show detailed process status (--process-status).
pub fn run_process_status() {
    print("Process status:");
    crate::utils::subprocess::show_process_status();
}

/// Restart all HookAnchor processes (--restart).
pub fn run_restart_server() {
    print("🔄 Restarting ALL HookAnchor processes...");

    // Hide popup window if visible (best effort)
    print("  Hiding popup window...");
    use std::os::unix::net::UnixStream;
    use std::io::Write;

    let popup_socket = crate::systems::get_popup_socket_path();
    if let Ok(mut stream) = UnixStream::connect(&popup_socket) {
        let _ = stream.write_all(b"hide");
        print("  ✅ Popup window hidden");
    }

    // BRUTAL KILL: Kill ALL HookAnchor processes including supervisor AND command server
    print("  🔪 Killing all HookAnchor processes...");
    // Kill command server first (uses PID from state.json)
    if let Err(e) = crate::execute::kill_existing_server() {
        print(&format!("  ⚠️  Failed to kill command server: {}", e));
    }
    // Kill popup servers and supervisor
    match crate::systems::restart::kill_popup_servers() {
        Ok(_) => {
            print("  ✅ All processes killed and verified dead");
        }
        Err(e) => {
            print(&format!("  ❌ Kill failed: {}", e));
            std::process::exit(1);
        }
    }

    // Clean up all sockets and lock files
    let config_dir = crate::core::get_config_dir();
    let _ = std::fs::remove_file(config_dir.join("popup.sock"));
    let _ = std::fs::remove_file(config_dir.join("supervisor.sock"));
    let _ = std::fs::remove_file(config_dir.join("execution_server.sock"));
    // Critical: clean up server startup lock file to prevent orphaned locks blocking restart
    let _ = std::fs::remove_file("/tmp/hookanchor_server_starting.lock");
    print("  ✅ Sockets cleaned up");

    // Start fresh supervisor (which will start fresh popup)
    print("  🚀 Starting fresh supervisor...");
    match std::process::Command::new("open")
        .arg("-a")
        .arg("HookAnchor")
        .spawn()
    {
        Ok(_) => {
            // Give supervisor time to start popup
            std::thread::sleep(std::time::Duration::from_millis(1500));
            print("  ✅ Supervisor and popup started");
        }
        Err(e) => {
            print(&format!("  ❌ Failed to start supervisor: {}", e));
            std::process::exit(1);
        }
    }

    // Verify popup is running
    let popup_check = std::process::Command::new("pgrep")
        .arg("-f")
        .arg("popup_server")
        .output();

    match popup_check {
        Ok(output) if output.status.success() && !output.stdout.is_empty() => {
            let pids = String::from_utf8_lossy(&output.stdout);
            print(&format!("  ✅ Popup server running (PID: {})", pids.trim().lines().next().unwrap_or("?")));
        }
        _ => {
            print("  ⚠️  Popup server may not have started yet");
        }
    }

    print("");
    print("✅ All servers restarted with fresh binaries!");
}

/// Send a command to the supervisor (--supervisor).
pub fn run_supervisor_command(args: &[String]) {
    if args.len() < 3 {
        print("Usage: ha --supervisor <action>");
        print("Actions:");
        print("  start   - Ensure servers are running (idempotent)");
        print("  stop    - Stop all servers (idempotent)");
        print("  restart - Restart all servers");
        std::process::exit(1);
    }

    let action = &args[2];

    // Validate action
    if !["start", "stop", "restart"].contains(&action.as_str()) {
        print(&format!("Unknown supervisor action: {}", action));
        print("Valid actions: start, stop, restart");
        std::process::exit(1);
    }

    print(&format!("📡 Sending '{}' command to supervisor...", action));

    match crate::systems::supervisor_command(action) {
        Ok(response) => {
            print(&format!("✅ {}", response));
        }
        Err(e) => {
            print(&format!("❌ Supervisor command failed: {}", e));
            std::process::exit(1);
        }
    }
}
