//! Background Command Execution Server
//!
//! This module implements a background server that handles command execution with proper
//! environment inheritance. The server starts from the main thread, inheriting the full
//! user environment and GUI permissions, then executes commands in proper subshells.
//!
//! This solves environment inconsistencies between CLI and URL handler execution paths.

use std::collections::HashMap;
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write, BufRead, BufReader};
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use chrono::TimeZone;
use crate::utils::detailed_log;

/// Helper function to output to both console and debug log
fn log_and_print(prefix: &str, message: &str) {
    let formatted = format!("{}: {}", prefix, message);
    // Print to stdout so we can see what the server is doing
    crate::utils::print(&formatted);
    // Also log to file for persistence - use regular log() for command execution
    crate::utils::log(&formatted);
}

/// Response structure for command execution
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CommandResponse {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub error: Option<String>,
}

/// Background server for command execution
pub(crate) struct CommandServer {
    socket_path: PathBuf,
    running: Arc<Mutex<bool>>,
    /// Environment captured from main thread at startup
    inherited_env: HashMap<String, String>,
    /// Working directory at startup
    base_working_dir: PathBuf,
}

impl CommandServer {
    /// Create a new command server
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let socket_path = get_socket_path()?;
        
        // Clean up any existing socket
        if socket_path.exists() {
            std::fs::remove_file(&socket_path)?;
        }
        
        // Capture current environment
        let inherited_env: HashMap<String, String> = std::env::vars().collect();
        
        // Set working directory to ~/.config/hookanchor per PRD
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let base_working_dir = PathBuf::from(home).join(".config/hookanchor");
        
        // Ensure the directory exists
        if !base_working_dir.exists() {
            std::fs::create_dir_all(&base_working_dir)?;
        }
        
        // Change to that directory
        std::env::set_current_dir(&base_working_dir)?;
        
        detailed_log("CMD_SERVER", &format!("Creating server with socket: {:?}", socket_path));
        detailed_log("CMD_SERVER", &format!("Captured {} environment variables", inherited_env.len()));
        detailed_log("CMD_SERVER", &format!("Base working directory: {:?}", base_working_dir));
        
        Ok(Self {
            socket_path,
            running: Arc::new(Mutex::new(false)),
            inherited_env,
            base_working_dir,
        })
    }
    
    /// Start the server in a background thread
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Clean up any stale socket file from previous crashed instances
        if std::path::Path::new(&self.socket_path).exists() {
            if let Err(e) = std::fs::remove_file(&self.socket_path) {
                crate::utils::log_error(&format!("Failed to remove stale command server socket: {}", e));
            } else {
                crate::utils::log("Cleaned up stale command server socket file");
            }
        }

        let listener = UnixListener::bind(&self.socket_path)?;
        
        // Log version and build info at server startup
        let version = env!("CARGO_PKG_VERSION");
        let state = crate::core::data::get_state();
        let build_time_str = if let Some(build_time) = state.build_time {
            let dt = chrono::Local.timestamp_opt(build_time, 0).single()
                .unwrap_or_else(|| chrono::Local::now());
            dt.format("%Y-%m-%d %H:%M:%S").to_string()
        } else {
            "Unknown".to_string()
        };
        
        log_and_print("CMD_SERVER", &format!("HookAnchor Server v{} starting - Build: {}", version, build_time_str));
        log_and_print("CMD_SERVER", &format!("Server listening on: {:?}", self.socket_path));
        
        
        let running = Arc::clone(&self.running);
        *running.lock().unwrap() = true;
        
        let inherited_env = self.inherited_env.clone();
        let base_working_dir = self.base_working_dir.clone();
        let running_clone = Arc::clone(&running);
        
        thread::spawn(move || {
            detailed_log("CMD_SERVER", "Server thread started");
            
            for stream in listener.incoming() {
                // Check if we should continue running
                if !*running_clone.lock().unwrap() {
                    detailed_log("CMD_SERVER", "Server shutdown requested");
                    break;
                }
                
                match stream {
                    Ok(stream) => {
                        let env = inherited_env.clone();
                        let working_dir = base_working_dir.clone();
                        
                        // Handle each connection in a separate thread
                        thread::spawn(move || {
                            if let Err(e) = handle_client(stream, env, working_dir) {
                                detailed_log("CMD_SERVER", &format!("Error handling client: {}", e));
                            }
                        });
                    }
                    Err(e) => {
                        detailed_log("CMD_SERVER", &format!("Error accepting connection: {}", e));
                    }
                }
            }
            
            detailed_log("CMD_SERVER", "Server thread exiting");
        });
        
        // We can't store the listener since it was moved into the thread
        // But we can store a flag indicating the server is running
        Ok(())
    }
    
    /// Stop the server
    pub fn stop(&mut self) {
        detailed_log("CMD_SERVER", "Stopping server");
        *self.running.lock().unwrap() = false;
        
        // Remove socket file
        if self.socket_path.exists() {
            let _ = std::fs::remove_file(&self.socket_path);
        }
    }
    
}

impl Drop for CommandServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Handle a client connection - simplified version
fn handle_client(
    mut stream: UnixStream,
    _inherited_env: HashMap<String, String>,
    _base_working_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    detailed_log("CMD_SERVER", "Handling new client connection");
    
    // Read the JSON action from the client
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    let bytes_read = reader.read_line(&mut request_line)?;
    
    // Check if we actually read any data
    if bytes_read == 0 || request_line.trim().is_empty() {
        detailed_log("CMD_SERVER", "Client sent empty request, closing connection");
        return Ok(());
    }
    
    detailed_log("CMD_SERVER", &format!("Read {} bytes", bytes_read));
    
    // Deserialize the Action
    let action: crate::execute::Action = serde_json::from_str(&request_line.trim())?;
    detailed_log("CMD_SERVER", &format!("Received action: type={}", action.action_type()));
    
    // Print one line to console showing what command is being executed
    // Extract a meaningful description of what's being executed
    let action_desc = if action.action_type() == "shell" || action.action_type() == "cmd" {
        // For shell/cmd, show the actual command
        action.get_string("arg")
            .map(|s| s.to_string())
            .unwrap_or_else(|| action.action_type().to_string())
    } else {
        // For other actions, show type and primary arg if available
        action.get_string("arg")
            .map(|arg| format!("{}: {}", action.action_type(), arg))
            .unwrap_or_else(|| action.action_type().to_string())
    };
    detailed_log("CMD_SERVER", &format!("Executing: {}", action_desc));
    
    // Check if client needs a response (blocking calls)
    let needs_response = action.get_string("flags").unwrap_or("").contains("G");
    
    if needs_response {
        // Execute synchronously and send result back
        detailed_log("CMD_SERVER", "Executing action synchronously (client waiting for response)");
        
        let result = match super::actions::execute_locally(&action, None, None) {
            Ok(output) => CommandResponse {
                success: true,
                exit_code: Some(0),
                stdout: output,
                stderr: String::new(),
                error: None,
            },
            Err(e) => {
                // Log error to file and console
                log_and_print("CMD_SERVER_ERROR", &format!("{}", e));
                CommandResponse {
                    success: false,
                    exit_code: Some(1),
                    stdout: String::new(),
                    stderr: String::new(),
                    error: Some(e.to_string()),
                }
            }
        };
        
        // Send the result back to client
        let response_json = serde_json::to_string(&result)?;
        stream.write_all(response_json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
    } else {
        // Execute asynchronously in background thread
        thread::spawn(move || {
            detailed_log("CMD_SERVER", "Executing action in background");
            match super::actions::execute_locally(&action, None, None) {
                Ok(_) => detailed_log("CMD_SERVER", "Action executed successfully"),
                Err(e) => {
                    // Log error to file and console
                    log_and_print("CMD_SERVER_ERROR", &format!("{}", e));
                    crate::utils::log_error(&format!("Action failed: {}", e));
                }
            }
        });
        
        // Send simple ACK to client
        let ack = CommandResponse {
            success: true,
            exit_code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
            error: None,
        };
        let response_json = serde_json::to_string(&ack)?;
        stream.write_all(response_json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
    }
    
    detailed_log("CMD_SERVER", "Response sent to client");
    Ok(())
}

// ARCHIVED: These execute functions were part of the old execution path before refactoring.
// They may have become disconnected during the module reorganization.
// Keeping them commented for reference in case we need to understand the old flow.
/*
/// Main entry point for executing actions in the server - routes based on action type
pub(crate) fn execute(action: &crate::execute::Action) -> Result<String, String> {
    detailed_log("CMD_SERVER", &format!("Server execute: type={}", action.action_type()));
    
    match action.action_type() {
        // Server-only actions (or actions that should always run in server context)
        "template" | "popup" => {
            // These should not reach the server
            Err(format!("{} actions must be executed in popup", action.action_type()))
        }
        _ => {
            // All other actions execute locally in server
            execute_locally(action)
        }
    }
}
/// Execute an action locally within the server process
pub fn execute_locally(action: &crate::execute::Action) -> Result<String, String> {
    detailed_log("CMD_SERVER", &format!("Executing action locally: type={}", action.action_type()));
    
    match action.action_type() {
        "template" => {
            // Templates are popup-only, shouldn't reach here
            Err("Template actions must be executed in popup".to_string())
        }
        "popup" => {
            // Popup actions are popup-only, shouldn't reach here
            Err("Popup actions must be executed in popup".to_string())
        }
        _ => {
            // For other actions, convert to command and execute
            // This is a temporary measure until we fully migrate to Actions
            let cmd = crate::core::Command {
                patch: action.get_string("patch").unwrap_or("").to_string(),
                command: action.get_string("command_name").unwrap_or("").to_string(),
                action: action.action_type().to_string(),
                arg: action.get_string("arg").unwrap_or("").to_string(),
                flags: action.get_string("flags").unwrap_or("").to_string(),
            };
            
            // Get environment from server
            let env = std::env::vars().collect();
            let working_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
            
            let response = execute_command_with_env(cmd, env, working_dir);
            if response.success {
                Ok(format!("Action executed successfully"))
            } else {
                Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
            }
        }
    }
}
*/


/// Get the socket path for the command server
fn get_socket_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = crate::core::get_config_dir();
    let socket_path = config_dir.join("execution_server.sock");

    // Ensure directory exists
    std::fs::create_dir_all(&config_dir)?;

    Ok(socket_path)
}



/// Send an action to the execution server for execution
/// This is the main public entry point for executing actions via the server
/// Returns CommandResponse with either ACK (non-blocking) or execution results (G flag/GUI)
pub(crate) fn send_for_execution(action: &crate::execute::Action) -> Result<CommandResponse, Box<dyn std::error::Error>> {
    use crate::utils::detailed_log;
    
    // Serialize action to JSON
    let action_json = serde_json::to_string(action)?;
    detailed_log("EXEC_SERVER", &format!("Sending action for execution: {}", action_json));
    
    // Send to server and get response
    let client = CommandClient::new()?;
    let response = client.send_json(&action_json)?;
    
    detailed_log("EXEC_SERVER", &format!("Action sent successfully, got response: success={}", response.success));
    Ok(response)
}

/// Client function to send commands to the server (internal use only)
struct CommandClient {
    socket_path: PathBuf,
}

impl CommandClient {
    /// Create a new command client
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let socket_path = get_socket_path()?;
        Ok(Self { socket_path })
    }
    
    /// Send raw JSON to the server (for Actions)
    fn send_json(&self, json: &str) -> Result<CommandResponse, Box<dyn std::error::Error>> {
        detailed_log("CMD_CLIENT", &format!("Sending JSON to server: {}", json));
        
        // Connect to server
        let mut stream = UnixStream::connect(&self.socket_path)?;
        
        // Set read timeout for response (5 seconds for operations like grab)
        stream.set_read_timeout(Some(std::time::Duration::from_millis(5000)))?;
        
        // Send JSON
        stream.write_all(json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
        
        // Wait for response (read line-by-line since server sends response with newline)
        let mut reader = std::io::BufReader::new(stream);
        let mut response_data = String::new();
        match reader.read_line(&mut response_data) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    detailed_log("CMD_CLIENT", "Server closed connection without sending response");
                    return Err("Server closed connection without sending response".into());
                }
                let response: CommandResponse = serde_json::from_str(&response_data.trim())?;
                detailed_log("CMD_CLIENT", &format!("Received response: success={}", response.success));
                Ok(response)
            }
            Err(e) => {
                detailed_log("CMD_CLIENT", &format!("Failed to receive response from server: {}", e));
                Err(format!("Server not responding: {}", e).into())
            }
        }
    }
    
}



/// Start a persistent command server and return its PID
/// This function starts a server that runs indefinitely until killed
pub(crate) fn start_persistent_server() -> Result<u32, Box<dyn std::error::Error>> {
    use crate::utils::detailed_log;
    
    detailed_log("CMD_SERVER", "Starting persistent command server");
    
    // Create a new server instance
    let socket_path = get_socket_path()?;
    
    // Remove existing socket if present
    if socket_path.exists() {
        std::fs::remove_file(&socket_path)?;
    }
    
    let mut server = CommandServer::new()?;
    
    // Get current process PID (this will be the daemon PID)
    let server_pid = std::process::id();
    detailed_log("CMD_SERVER", &format!("Persistent server starting with PID: {}", server_pid));
    
    // Start the server (this starts a background thread)
    server.start()?;
    
    
    // Keep the server alive by preventing it from being dropped
    // We need to transfer ownership to avoid the Drop implementation
    std::mem::forget(server);
    
    Ok(server_pid)
}