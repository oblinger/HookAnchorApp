//! Tiny launcher for the popup server
//! 
//! This lightweight binary manages the popup server lifecycle and sends commands to it.
//! Designed for minimal startup time when triggered by keyboard shortcuts.

use std::os::unix::net::UnixStream;
use std::io::{Write, Read};
use std::process::{Command, exit};
use std::time::Duration;
use std::thread;
use std::env;

/// Socket path for popup server control
const POPUP_SOCKET: &str = "/tmp/hookanchor_popup.sock";

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
    
    // Start popup server in background
    Command::new(&popup_server_path)
        .spawn()
        .map_err(|e| format!("Failed to start popup server: {}", e))?;
    
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
    let command = if args.len() > 1 {
        args[1].as_str()
    } else {
        "show" // Default action
    };
    
    // First try to send command
    let result = match send_command(command) {
        Ok(response) => {
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
                    println!("Popup server not running");
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