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
use crate::utils::{debug_log, verbose_log};

/// Helper function to output to both console and debug log
fn log_and_print(prefix: &str, message: &str) {
    let formatted = format!("{}: {}", prefix, message);
    println!("{}", formatted);
    debug_log(prefix, message);
}

/// Request structure for command execution
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRequest {
    pub command: String,
    pub working_dir: Option<String>,
    pub env_vars: Option<HashMap<String, String>>,
    pub blocking: bool,
}

/// Response structure for command execution
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub error: Option<String>,
}

/// Background server for command execution
pub struct CommandServer {
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
        let base_working_dir = std::env::current_dir()?;
        
        debug_log("CMD_SERVER", &format!("Creating server with socket: {:?}", socket_path));
        debug_log("CMD_SERVER", &format!("Captured {} environment variables", inherited_env.len()));
        debug_log("CMD_SERVER", &format!("Base working directory: {:?}", base_working_dir));
        
        Ok(Self {
            socket_path,
            running: Arc::new(Mutex::new(false)),
            inherited_env,
            base_working_dir,
        })
    }
    
    /// Start the server in a background thread
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = UnixListener::bind(&self.socket_path)?;
        
        // Log version and build info at server startup
        let version = env!("CARGO_PKG_VERSION");
        let state = crate::core::state::load_state();
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
            debug_log("CMD_SERVER", "Server thread started");
            
            for stream in listener.incoming() {
                // Check if we should continue running
                if !*running_clone.lock().unwrap() {
                    debug_log("CMD_SERVER", "Server shutdown requested");
                    break;
                }
                
                match stream {
                    Ok(stream) => {
                        let env = inherited_env.clone();
                        let working_dir = base_working_dir.clone();
                        
                        // Handle each connection in a separate thread
                        thread::spawn(move || {
                            if let Err(e) = handle_client(stream, env, working_dir) {
                                debug_log("CMD_SERVER", &format!("Error handling client: {}", e));
                            }
                        });
                    }
                    Err(e) => {
                        debug_log("CMD_SERVER", &format!("Error accepting connection: {}", e));
                    }
                }
            }
            
            debug_log("CMD_SERVER", "Server thread exiting");
        });
        
        // We can't store the listener since it was moved into the thread
        // But we can store a flag indicating the server is running
        Ok(())
    }
    
    /// Stop the server
    pub fn stop(&mut self) {
        debug_log("CMD_SERVER", "Stopping server");
        *self.running.lock().unwrap() = false;
        
        // Remove socket file
        if self.socket_path.exists() {
            let _ = std::fs::remove_file(&self.socket_path);
        }
    }
    
    /// Check if server is running
    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
    
    /// Execute a command through the server with proper alias resolution
    pub fn execute_command(&self, command: &crate::Command) -> crate::CommandTarget {
        self.execute_command_with_depth(command, 0)
    }
    
    /// Internal function to execute commands with recursion depth tracking
    fn execute_command_with_depth(&self, command: &crate::Command, depth: u32) -> crate::CommandTarget {
        const MAX_ALIAS_DEPTH: u32 = 10; // Prevent infinite recursion
        
        // Handle alias command type: execute the target command instead
        if command.action == "alias" {
            if depth >= MAX_ALIAS_DEPTH {
                crate::utils::debug_log("ALIAS", &format!("Maximum alias depth ({}) exceeded for '{}'", MAX_ALIAS_DEPTH, command.command));
                return crate::CommandTarget::Command(command.clone());
            }
            
            // Load all commands to find the target
            let all_commands = crate::load_commands();
            
            // Filter commands to find the best match for the alias target
            let filtered = crate::filter_commands(&all_commands, &command.arg, 1, false);
            
            if !filtered.is_empty() {
                // Execute the first matching command
                let target_command = &filtered[0];
                crate::utils::debug_log("ALIAS", &format!("Alias '{}' executing target: {} (depth: {})", command.command, target_command.command, depth));
                return self.execute_command_with_depth(target_command, depth + 1); // Recursive call with depth tracking
            } else {
                crate::utils::debug_log("ALIAS", &format!("Alias '{}' target '{}' not found", command.command, command.arg));
                return crate::CommandTarget::Command(command.clone());
            }
        }
        
        let launcher_cmd = format!("{} {}", command.action, command.arg);
        
        // Log command execution in the requested format
        crate::utils::debug_log("EXECUTE", &format!("'{}' AS '{}' ON '{}'", command.command, command.action, command.arg));
        crate::utils::debug_log("EXECUTE_FLOW", "Starting command execution process via server");
        
        // Save the last executed command for add_alias functionality
        use crate::core::state::save_last_executed_command;
        crate::utils::debug_log("STATE_SAVE", &format!("Attempting to save last executed command: '{}'", command.command));
        match save_last_executed_command(&command.command) {
            Ok(_) => crate::utils::debug_log("STATE_SAVE", "Successfully saved last executed command"),
            Err(e) => crate::utils::debug_log("STATE_SAVE", &format!("Failed to save last executed command: {}", e)),
        }
        
        // Execute via server (always use server for consistent execution)
        crate::utils::debug_log("EXECUTE_FLOW", "Sending command to server for execution");
        
        if let Ok(client) = CommandClient::new() {
            if client.is_server_available() {
                crate::utils::debug_log("EXECUTE_FLOW", "Server available, sending launcher command");
                
                // Send the full launcher command to server for execution
                match client.execute_command(&launcher_cmd, None, None, false) {
                    Ok(response) => {
                        crate::utils::debug_log("EXECUTE_FLOW", &format!("Command sent to server successfully: {:?}", response));
                        
                        // Check process health after command execution
                        crate::process_monitor::check_system_health();
                        
                        // For rewrite commands, we need to handle the special case
                        if command.action == "rewrite" {
                            // The rewrite command should execute another command
                            return crate::CommandTarget::Alias(command.arg.clone());
                        } else {
                            return crate::CommandTarget::Command(command.clone());
                        }
                    }
                    Err(e) => {
                        crate::utils::debug_log("EXECUTE_FLOW", &format!("Failed to send command to server: {:?}", e));
                        // Fall through to error handling
                    }
                }
            }
        }
        
        // If we get here, server execution failed
        eprintln!("Failed to execute command via server");
        crate::CommandTarget::Command(command.clone())
    }
}

impl Drop for CommandServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Handle a client connection
fn handle_client(
    mut stream: UnixStream,
    inherited_env: HashMap<String, String>,
    base_working_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    verbose_log("CMD_SERVER", "Handling new client connection");
    
    // Read request
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    let bytes_read = reader.read_line(&mut request_line)?;
    
    // Check if we actually read any data
    if bytes_read == 0 || request_line.trim().is_empty() {
        verbose_log("CMD_SERVER", "Client sent empty request, closing connection");
        return Ok(());
    }
    
    verbose_log("CMD_SERVER", &format!("Read {} bytes: '{}'", bytes_read, request_line.trim()));
    
    let request: CommandRequest = serde_json::from_str(&request_line.trim())?;
    verbose_log("CMD_SERVER", &format!("Received request: {:?}", request));
    
    if !request.blocking {
        // For non-blocking requests, spawn a thread to execute the command
        verbose_log("CMD_SERVER", "Non-blocking request: spawning thread for execution");
        let inherited_env_clone = inherited_env.clone();
        let base_working_dir_clone = base_working_dir.clone();
        thread::spawn(move || {
            let response = execute_command_with_env(request, inherited_env_clone, base_working_dir_clone);
            verbose_log("CMD_SERVER", &format!("Non-blocking command completed: success={}", response.success));
        });
        
        // Send immediate success response for non-blocking
        let response = CommandResponse {
            success: true,
            exit_code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
            error: None,
        };
        let response_json = serde_json::to_string(&response)?;
        stream.write_all(response_json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
    } else {
        // Execute command with inherited environment (blocking)
        let response = execute_command_with_env(request, inherited_env, base_working_dir);
        
        // Send response
        let response_json = serde_json::to_string(&response)?;
        stream.write_all(response_json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
    }
    
    verbose_log("CMD_SERVER", "Response sent to client");
    Ok(())
}

/// Execute a command with the inherited environment
fn execute_command_with_env(
    request: CommandRequest,
    inherited_env: HashMap<String, String>,
    base_working_dir: PathBuf,
) -> CommandResponse {
    // Parse command to extract action and arg for clean logging
    let command_parts: Vec<&str> = request.command.trim().split_whitespace().collect();
    let (action, arg) = if command_parts.len() >= 2 {
        (command_parts[0], command_parts[1..].join(" "))
    } else if command_parts.len() == 1 {
        (command_parts[0], String::new())
    } else {
        ("unknown", request.command.clone())
    };
    
    // Clean command execution log
    if arg.is_empty() {
        log_and_print("CMD", &format!("action={}", action));
    } else {
        log_and_print("CMD", &format!("action={} arg={}", action, arg));
    }
    
    // Detailed logging goes to debug only
    verbose_log("CMD_SERVER", &format!("Full command: {}", request.command));
    verbose_log("CMD_SERVER", &format!("Command parts: {:?}", command_parts));
    
    let is_launcher_command = command_parts.len() >= 1 && 
        matches!(
            command_parts[0], 
            "app" | "url" | "cmd" | "chrome" | "safari" | "brave" | "firefox" | "work" | 
            "notion" | "obs" | "obs_url" | "1pass" | "rewrite" | "anchor" | "folder" | 
            "doc" | "markdown" | "text" | "shutdown" | "slack" | "contact"
        );
    
    verbose_log("CMD_SERVER", &format!("Is launcher command: {}", is_launcher_command));
    
    if is_launcher_command {
        // Use launcher for known action types
        verbose_log("CMD_SERVER", &format!("Using launcher for action: {}", action));
        
        // Execute launcher command directly in server context
        match crate::command_launcher::launch(&request.command) {
            Ok(()) => {
                verbose_log("CMD_SERVER", "Launcher execution completed successfully");
                return CommandResponse {
                    success: true,
                    exit_code: Some(0),
                    stdout: String::new(),
                    stderr: String::new(),
                    error: None,
                };
            }
            Err(e) => {
                log_and_print("CMD", &format!("FAILED: {:?}", e));
                return CommandResponse {
                    success: false,
                    exit_code: Some(1),
                    stdout: String::new(),
                    stderr: format!("Launcher error: {:?}", e),
                    error: Some(format!("Launcher error: {:?}", e)),
                };
            }
        }
    }
    
    // Fall back to shell execution for non-launcher commands
    verbose_log("CMD_SERVER", &format!("Using shell execution for: {}", request.command));
    let mut cmd = std::process::Command::new("sh");
    cmd.arg("-c").arg(&request.command);
    
    // Set working directory
    let working_dir = if let Some(ref wd) = request.working_dir {
        PathBuf::from(wd)
    } else {
        base_working_dir
    };
    cmd.current_dir(&working_dir);
    
    // Apply inherited environment
    for (key, value) in &inherited_env {
        cmd.env(key, value);
    }
    
    // Apply any additional environment variables from request
    if let Some(ref env_vars) = request.env_vars {
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
    }
    
    verbose_log("CMD_SERVER", &format!("Working directory: {:?}", working_dir));
    verbose_log("CMD_SERVER", &format!("Environment variables: {}", inherited_env.len()));
    
    // Log the PATH specifically for debugging
    if let Some(path) = inherited_env.get("PATH") {
        verbose_log("CMD_SERVER", &format!("PATH: {}", path));
    } else {
        verbose_log("CMD_SERVER", "PATH: Not found in environment");
    }
    
    // Execute command
    match cmd.output() {
        Ok(output) => {
            let success = output.status.success();
            let exit_code = output.status.code();
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            // Always log stdout/stderr for shell commands with indented format
            if !stdout.is_empty() {
                for line in stdout.lines() {
                    debug_log("", &format!("  | {}", line));
                }
            }
            if !stderr.is_empty() {
                for line in stderr.lines() {
                    debug_log("", &format!("  | {}", line));
                }
            }
            
            CommandResponse {
                success,
                exit_code,
                stdout,
                stderr,
                error: None,
            }
        }
        Err(e) => {
            verbose_log("CMD_SERVER", &format!("Command execution failed: {}", e));
            
            CommandResponse {
                success: false,
                exit_code: None,
                stdout: String::new(),
                stderr: String::new(),
                error: Some(e.to_string()),
            }
        }
    }
}

/// Get the socket path for the command server
fn get_socket_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let socket_path = PathBuf::from(home)
        .join(".config")
        .join("hookanchor")
        .join("command_server.sock");
    
    // Ensure directory exists
    if let Some(parent) = socket_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    Ok(socket_path)
}

/// Client function to send commands to the server
pub struct CommandClient {
    socket_path: PathBuf,
}

impl CommandClient {
    /// Create a new command client
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let socket_path = get_socket_path()?;
        Ok(Self { socket_path })
    }
    
    /// Execute a command via the server
    pub fn execute_command(
        &self,
        command: &str,
        working_dir: Option<&str>,
        env_vars: Option<HashMap<String, String>>,
        blocking: bool,
    ) -> Result<CommandResponse, Box<dyn std::error::Error>> {
        verbose_log("CMD_CLIENT", &format!("Sending command to server: {} (blocking: {})", command, blocking));
        
        // Create request
        let request = CommandRequest {
            command: command.to_string(),
            working_dir: working_dir.map(|s| s.to_string()),
            env_vars,
            blocking,
        };
        
        // Connect to server
        let mut stream = UnixStream::connect(&self.socket_path)?;
        
        // Send request
        let request_json = serde_json::to_string(&request)?;
        stream.write_all(request_json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
        
        if !blocking {
            // For non-blocking mode, don't wait for response
            verbose_log("CMD_CLIENT", "Non-blocking mode: not waiting for response");
            // Return a synthetic success response
            return Ok(CommandResponse {
                success: true,
                exit_code: Some(0),
                stdout: String::new(),
                stderr: String::new(),
                error: None,
            });
        }
        
        // Read response only in blocking mode
        let mut response_data = String::new();
        stream.read_to_string(&mut response_data)?;
        
        let response: CommandResponse = serde_json::from_str(&response_data.trim())?;
        verbose_log("CMD_CLIENT", &format!("Received response: success={}", response.success));
        
        Ok(response)
    }
    
    /// Check if the server is available
    pub fn is_server_available(&self) -> bool {
        UnixStream::connect(&self.socket_path).is_ok()
    }
}

/// Global server instance
static mut GLOBAL_SERVER: Option<CommandServer> = None;
static mut SERVER_INITIALIZED: bool = false;

/// Initialize the global command server
pub fn init_global_server() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if SERVER_INITIALIZED {
            return Ok(());
        }
        
        debug_log("CMD_SERVER", "Initializing global command server");
        
        let mut server = CommandServer::new()?;
        server.start()?;
        
        GLOBAL_SERVER = Some(server);
        SERVER_INITIALIZED = true;
        
        debug_log("CMD_SERVER", "Global command server initialized");
        Ok(())
    }
}

/// Shutdown the global command server
pub fn shutdown_global_server() {
    unsafe {
        if let Some(ref mut server) = GLOBAL_SERVER {
            server.stop();
        }
        GLOBAL_SERVER = None;
        SERVER_INITIALIZED = false;
        debug_log("CMD_SERVER", "Global command server shutdown");
    }
}

/// Check if the global server is running
pub fn is_global_server_running() -> bool {
    unsafe {
        if let Some(ref server) = GLOBAL_SERVER {
            server.is_running()
        } else {
            false
        }
    }
}

/// Execute a command using the global server (server required for consistent environment)
pub fn execute_via_server(
    command: &str,
    working_dir: Option<&str>,
    env_vars: Option<HashMap<String, String>>,
    blocking: bool,
) -> Result<CommandResponse, Box<dyn std::error::Error>> {
    // Try to use the server
    if let Ok(client) = CommandClient::new() {
        if client.is_server_available() {
            verbose_log("CMD_SERVER", "Using server for command execution");
            return client.execute_command(command, working_dir, env_vars, blocking);
        }
    }
    
    // Server not available - try to start it
    verbose_log("CMD_SERVER", "Server not available, attempting to start it");
    crate::utils::debug_log("CMD_SERVER", &format!("Starting server for command: {}", command));
    crate::command_server_management::reset_server_check(); // Force re-check of PID
    if let Err(e) = crate::command_server_management::start_server_if_needed() {
        let error_msg = format!("Failed to start command server: {}", e);
        verbose_log("CMD_SERVER", &error_msg);
        return Err(error_msg.into());
    }
    
    // Wait for server to start (reduced from 4s to 100ms for better UX)
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Try again after starting server
    if let Ok(client) = CommandClient::new() {
        if client.is_server_available() {
            verbose_log("CMD_SERVER", "Using newly started server for command execution");
            return client.execute_command(command, working_dir, env_vars, blocking);
        }
    }
    
    // Still not available - this is an error condition
    let error_msg = "Command server not available - cannot execute commands with proper environment";
    verbose_log("CMD_SERVER", error_msg);
    
    Err(error_msg.into())
}

/// Start a persistent command server and return its PID
/// This function starts a server that runs indefinitely until killed
pub fn start_persistent_server() -> Result<u32, Box<dyn std::error::Error>> {
    use crate::utils::debug_log;
    
    debug_log("CMD_SERVER", "Starting persistent command server");
    
    // Create a new server instance
    let socket_path = get_socket_path()?;
    
    // Remove existing socket if present
    if socket_path.exists() {
        std::fs::remove_file(&socket_path)?;
    }
    
    let mut server = CommandServer::new()?;
    
    // Get current process PID (this will be the daemon PID)
    let server_pid = std::process::id();
    debug_log("CMD_SERVER", &format!("Persistent server starting with PID: {}", server_pid));
    
    // Start the server (this starts a background thread)
    server.start()?;
    
    
    // Keep the server alive by preventing it from being dropped
    // We need to transfer ownership to avoid the Drop implementation
    std::mem::forget(server);
    
    Ok(server_pid)
}