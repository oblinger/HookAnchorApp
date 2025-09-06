//! Tiny launcher for the popup server
//! 
//! This lightweight binary manages the popup server lifecycle and sends commands to it.
//! Designed for minimal startup time when triggered by keyboard shortcuts.

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
    // Find popup_server binary in same directory as launcher
    let exe_path = env::current_exe()
        .map_err(|e| format!("Failed to get current exe: {}", e))?;
    let exe_dir = exe_path.parent()
        .ok_or("Failed to get exe directory")?;
    let popup_server_path = exe_dir.join("popup_server");
    
    // Check if we should use Terminal for better permissions (macOS)
    #[cfg(target_os = "macos")]
    {
        // Start popup server via Terminal to inherit Accessibility permissions
        // This allows osascript subprocesses to send keystrokes (needed for grabber)
        let escaped_path = popup_server_path.display().to_string().replace("\"", "\\\"");
        let script = format!(
            r#"tell application "Terminal" to do script "cd ~ && \"{}\" 2>&1""#,
            escaped_path
        );
        
        eprintln!("Starting popup_server via Terminal for proper permissions...");
        eprintln!("This allows the grabber to send keystrokes to applications.");
        
        let output = Command::new("osascript")
            .args(["-e", &script])
            .output()
            .map_err(|e| format!("Failed to execute AppleScript: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("AppleScript error: {}", stderr);
            // Fall back to direct spawn if Terminal approach fails
            Command::new(&popup_server_path)
                .spawn()
                .map_err(|e| format!("Failed to start popup server directly: {}", e))?;
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        // Non-macOS: start normally
        Command::new(&popup_server_path)
            .spawn()
            .map_err(|e| format!("Failed to start popup server: {}", e))?;
    }
    
    // Wait for server to be ready (max 2 seconds)
    for _ in 0..20 {
        if send_command("ping").is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    
    Err("Popup server failed to start".to_string())
}

fn main() {
    // Get command from args
    let args: Vec<String> = env::args().collect();
    
    // Check for --server flag (used by Swift supervisor)
    if args.len() > 1 && args[1] == "--server" {
        // Run popup_server directly in server mode
        let exe_path = env::current_exe().unwrap_or_else(|e| {
            eprintln!("Failed to get current exe: {}", e);
            exit(1);
        });
        let exe_dir = exe_path.parent().unwrap_or_else(|| {
            eprintln!("Failed to get exe directory");
            exit(1);
        });
        let popup_server_path = exe_dir.join("popup_server");
        
        // Execute popup_server, replacing this process
        let err = Command::new(&popup_server_path)
            .env("HOOKANCHOR_SERVER_MODE", "1")
            .exec();
        
        // If exec fails, fall back to spawn
        eprintln!("Failed to exec popup_server: {:?}, trying spawn", err);
        let status = Command::new(&popup_server_path)
            .env("HOOKANCHOR_SERVER_MODE", "1")
            .status();
        
        match status {
            Ok(s) => exit(s.code().unwrap_or(1)),
            Err(e) => {
                eprintln!("Failed to launch popup_server: {}", e);
                exit(1);
            }
        }
    }
    
    // Check if we should use server mode
    let use_server_mode = check_run_in_background();
    
    let command = if args.len() > 1 {
        args[1].as_str()
    } else {
        "show" // Default action
    };
    
    if !use_server_mode {
        // Direct mode - but first check if a server is still running from before
        if command == "status" {
            // Check if server is running even though we're in direct mode
            if send_command("ping").is_ok() {
                println!("Config: run_in_background = false");
                println!("Mode: Direct mode");
                println!("WARNING: Popup server IS running (leftover from server mode?)");
                println!("         Consider running 'popup delete' to stop the server");
                // Also get the server's status
                if let Ok(response) = send_command("status") {
                    println!("Server reports: {}", response);
                }
            } else {
                println!("Config: run_in_background = false");
                println!("Mode: Direct mode");
                println!("Popup server: Not running");
                println!("(This is normal for direct mode)");
            }
            exit(0);
        }
        
        // Direct mode - just launch popup_server directly (it will run once and exit)
        if command == "show" || command == "" {
            // Find popup_server binary in same directory as launcher
            let exe_path = env::current_exe().unwrap_or_else(|e| {
                eprintln!("Failed to get current exe: {}", e);
                exit(1);
            });
            let exe_dir = exe_path.parent().unwrap_or_else(|| {
                eprintln!("Failed to get exe directory");
                exit(1);
            });
            let popup_server_path = exe_dir.join("popup_server");
            
            // Launch popup_server directly (not as daemon)
            let status = Command::new(&popup_server_path)
                .env("HOOKANCHOR_DIRECT_MODE", "1")  // Signal to popup_server to run in direct mode
                .status();
            
            match status {
                Ok(s) => {
                    if !s.success() {
                        exit(s.code().unwrap_or(1));
                    }
                }
                Err(e) => {
                    eprintln!("Failed to launch popup: {}", e);
                    exit(1);
                }
            }
        } else {
            println!("Direct mode - only 'show' command supported");
        }
        exit(0);
    }
    
    // Server mode - use socket communication
    // First try to send command
    let _result = match send_command(command) {
        Ok(response) => {
            // For status command, add mode information
            if command == "status" {
                let config_says_background = check_run_in_background();
                println!("Config: run_in_background = {}", config_says_background);
                println!("Mode: {}", if config_says_background { "Server mode" } else { "Direct mode" });
                if !config_says_background {
                    println!("WARNING: Server is running but config says direct mode!");
                    println!("         Consider running 'popup delete' to stop the server");
                }
            }
            println!("{}", response);
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
                                    println!("{}", response);
                                    exit(0);
                                }
                                Err(e) => {
                                    eprintln!("Failed to show popup: {}", e);
                                    exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to start popup server: {}", e);
                            exit(1);
                        }
                    }
                }
                "status" => {
                    // Check config setting
                    let config_says_background = check_run_in_background();
                    println!("Config: run_in_background = {}", config_says_background);
                    println!("Mode: {}", if config_says_background { "Server mode" } else { "Direct mode" });
                    println!("Popup server: Not running");
                    
                    // In direct mode, that's expected
                    if !config_says_background {
                        println!("(This is normal for direct mode)");
                    }
                    exit(0);
                }
                _ => {
                    eprintln!("Popup server not running");
                    exit(1);
                }
            }
        }
    };
}