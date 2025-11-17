//! Tiny launcher for the popup server
//!
//! This lightweight binary manages the popup server lifecycle and sends commands to it.
//! Designed for minimal startup time when triggered by keyboard shortcuts.

use hookanchor::prelude::*;
use hookanchor::utils::print;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::io::{Write, Read};
use std::process::{Command, exit};
use std::time::Duration;
use std::thread;
use std::env;

/// Socket path for popup server control
const POPUP_SOCKET: &str = "/tmp/hookanchor_popup.sock";

/// Check if run_in_background is enabled in config
/// Optimized for speed - does minimal file reading
fn check_run_in_background() -> bool {
    // Cache the result using a simple check file
    // This avoids parsing YAML on every launch
    let cache_marker = "/tmp/hookanchor_server_mode";
    
    // If cache file exists, we're in server mode
    if std::path::Path::new(cache_marker).exists() {
        return true;
    }
    
    // Otherwise default to direct mode (faster startup)
    false
}

/// Send a command to the popup server
fn send_command(command: &str) -> Result<String, String> {
    match UnixStream::connect(POPUP_SOCKET) {
        Ok(mut stream) => {
            // Set timeout for operations
            stream.set_read_timeout(Some(Duration::from_secs(2)))
                .map_err(|e| format!("Failed to set timeout: {}", e))?;
            
            // Send command
            stream.write_all(command.as_bytes())
                .map_err(|e| format!("Failed to send command: {}", e))?;
            
            // Read response
            let mut response = String::new();
            stream.read_to_string(&mut response)
                .map_err(|e| format!("Failed to read response: {}", e))?;
            
            Ok(response)
        }
        Err(_) => Err("Popup server not running".to_string())
    }
}

/// Start the popup server
fn start_popup_server() -> Result<(), String> {
    print_and_log("Starting popup_server...");

    // Use centralized start function (no Terminal tab needed)
    hookanchor::systems::start_popup_server()?;

    // Wait for server to be ready (max 2 seconds)
    for _ in 0..20 {
        if send_command("ping").is_ok() {
            print_and_log("✅ Popup server is ready");
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }

    Err("Popup server failed to start".to_string())
}

fn main() {
    // Get command from args
    let args: Vec<String> = env::args().collect();

    // Special case: if first arg is --popup, just forward all args to popup_server directly
    if args.len() > 1 && args[1] == "--popup" {
        // Forward everything to popup_server
        let exe_path = env::current_exe().unwrap_or_else(|e| {
            print(&format!("Failed to get current exe path: {}", e));
            std::path::PathBuf::from("popup")
        });

        let exe_dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let popup_server_path = exe_dir.join("popup_server");

        // Replace current process with popup_server, passing all args except the first (program name)
        let err = Command::new(&popup_server_path)
            .args(&args[1..])  // Forward all args starting from --popup
            .exec();  // Replace process

        // exec() never returns on success, so this is an error
        print(&format!("Failed to launch popup_server: {}", err));
        std::process::exit(1);
    }

    // Parse optional --input and --action flags, and extract non-flag command
    let mut input_text = None;
    let mut action_name = None;
    let mut other_command = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" if i + 1 < args.len() => {
                input_text = Some(args[i + 1].clone());
                i += 2;
            }
            "--action" if i + 1 < args.len() => {
                action_name = Some(args[i + 1].clone());
                i += 2;
            }
            arg if !arg.starts_with("--") && other_command.is_none() => {
                other_command = Some(arg.to_string());
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    // Check for --server flag (used by Swift supervisor)
    if args.len() > 1 && args[1] == "--server" {
        // Run popup_server directly in server mode
        let exe_path = env::current_exe().unwrap_or_else(|e| {
            log_error(&format!("Failed to get current exe: {}", e));
            exit(1);
        });
        let exe_dir = exe_path.parent().unwrap_or_else(|| {
            log_error("Failed to get exe directory");
            exit(1);
        });
        let popup_server_path = exe_dir.join("popup_server");
        
        // Execute popup_server, replacing this process
        let err = Command::new(&popup_server_path)
            .env("HOOKANCHOR_SERVER_MODE", "1")
            .exec();

        // If exec fails, fall back to spawn
        log_error(&format!("Failed to exec popup_server: {:?}, trying spawn", err));
        let status = Command::new(&popup_server_path)
            .env("HOOKANCHOR_SERVER_MODE", "1")
            .status();

        match status {
            Ok(s) => exit(s.code().unwrap_or(1)),
            Err(e) => {
                log_error(&format!("Failed to launch popup_server: {}", e));
                exit(1);
            }
        }
    }
    
    // Check if we should use server mode
    let use_server_mode = check_run_in_background();

    // Determine the command - default to "show" if no command provided
    let command = other_command.as_deref().unwrap_or("show");
    
    if !use_server_mode {
        // Direct mode - but first check if a server is still running from before
        if command == "status" {
            // Check if server is running even though we're in direct mode
            if send_command("ping").is_ok() {
                print_and_log("Config: run_in_background = false");
                print_and_log("Mode: Direct mode");
                print_and_log("WARNING: Popup server IS running (leftover from server mode?)");
                print_and_log("         Consider running 'popup delete' to stop the server");
                // Also get the server's status
                if let Ok(response) = send_command("status") {
                    print_and_log(&format!("Server reports: {}", response));
                }
            } else {
                print_and_log("Config: run_in_background = false");
                print_and_log("Mode: Direct mode");
                print_and_log("Popup server: Not running");
                print_and_log("(This is normal for direct mode)");
            }
            exit(0);
        }
        
        // Direct mode - just launch popup_server directly (it will run once and exit)
        if command == "show" || command == "" {
            // Find popup_server binary in same directory as launcher
            let exe_path = env::current_exe().unwrap_or_else(|e| {
                log_error(&format!("Failed to get current exe: {}", e));
                exit(1);
            });
            let exe_dir = exe_path.parent().unwrap_or_else(|| {
                log_error("Failed to get exe directory");
                exit(1);
            });
            let popup_server_path = exe_dir.join("popup_server");

            // Build command with optional arguments
            let mut cmd = Command::new(&popup_server_path);
            cmd.env("HOOKANCHOR_DIRECT_MODE", "1");  // Signal to popup_server to run in direct mode

            // Pass --input and --action if provided
            if let Some(ref text) = input_text {
                cmd.arg("--input").arg(text);
            }
            if let Some(ref action) = action_name {
                cmd.arg("--action").arg(action);
            }

            // Launch popup_server directly (not as daemon)
            let status = cmd.status();
            
            match status {
                Ok(s) => {
                    if !s.success() {
                        exit(s.code().unwrap_or(1));
                    }
                }
                Err(e) => {
                    log_error(&format!("Failed to launch popup: {}", e));
                    exit(1);
                }
            }
        } else {
            print("Direct mode - only 'show' command supported");
        }
        exit(0);
    }
    
    // Server mode - use socket communication
    // Build command with optional args
    let full_command = if input_text.is_some() || action_name.is_some() {
        let mut cmd_parts = vec![command.to_string()];
        if let Some(text) = input_text {
            cmd_parts.push(format!("--input {}", text));
        }
        if let Some(action) = action_name {
            cmd_parts.push(format!("--action {}", action));
        }
        cmd_parts.join(" ")
    } else {
        command.to_string()
    };

    // First try to send command
    let _result = match send_command(&full_command) {
        Ok(response) => {
            // For status command, add mode information
            if command == "status" {
                let config_says_background = check_run_in_background();
                print_and_log(&format!("Config: run_in_background = {}", config_says_background));
                print_and_log(&format!("Mode: {}", if config_says_background { "Server mode" } else { "Direct mode" }));
                if !config_says_background {
                    print_and_log("WARNING: Server is running but config says direct mode!");
                    print_and_log("         Consider running 'popup delete' to stop the server");
                }
            }
            print(&response);
            exit(0);
        }
        Err(_) => {
            // Server not running, handle based on command
            match command {
                "show" => {
                    // Start server and show
                    match start_popup_server() {
                        Ok(_) => {
                            // Server started, send show command
                            match send_command("show") {
                                Ok(response) => {
                                    print(&response);
                                    exit(0);
                                }
                                Err(e) => {
                                    log_error(&format!("Failed to show popup: {}", e));
                                    exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            log_error(&format!("Failed to start popup server: {}", e));
                            exit(1);
                        }
                    }
                }
                "status" => {
                    // Check config setting
                    let config_says_background = check_run_in_background();
                    print_and_log(&format!("Config: run_in_background = {}", config_says_background));
                    print_and_log(&format!("Mode: {}", if config_says_background { "Server mode" } else { "Direct mode" }));
                    print_and_log("Popup server: Not running");

                    // In direct mode, that's expected
                    if !config_says_background {
                        print_and_log("(This is normal for direct mode)");
                    }
                    exit(0);
                }
                _ => {
                    log_error("Popup server not running");
                    exit(1);
                }
            }
        }
    };
}