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
    // Print to stdout so we can see what the server is doing
    println!("{}", formatted);
    // Also log to file for persistence
    debug_log(prefix, message);
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
        
        // Set working directory to ~/.config/hookanchor per PRD
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let base_working_dir = PathBuf::from(home).join(".config/hookanchor");
        
        // Ensure the directory exists
        if !base_working_dir.exists() {
            std::fs::create_dir_all(&base_working_dir)?;
        }
        
        // Change to that directory
        std::env::set_current_dir(&base_working_dir)?;
        
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
        
        // Log command execution in the requested format
        crate::utils::debug_log("EXECUTE", &format!("'{}' AS '{}' ON '{}'", command.command, command.action, command.arg));
        crate::utils::debug_log("EXECUTE_FLOW", "Starting command execution process via server");
        
        // Save the last executed command for add_alias functionality
        use crate::core::state::save_last_executed_command;
        crate::utils::detailed_log("STATE_SAVE", &format!("Attempting to save last executed command: '{}'", command.command));
        match save_last_executed_command(&command.command) {
            Ok(_) => crate::utils::detailed_log("STATE_SAVE", "Successfully saved last executed command"),
            Err(e) => crate::utils::detailed_log("STATE_SAVE", &format!("Failed to save last executed command: {}", e)),
        }
        
        // Execute via server (always use server for consistent execution)
        crate::utils::debug_log("EXECUTE_FLOW", "Sending command to server for execution");
        
        if let Ok(client) = CommandClient::new() {
            if client.is_server_available() {
                crate::utils::debug_log("EXECUTE_FLOW", "Server available, sending command object");
                
                // Send the command object to server for execution
                match client.execute_command(command) {
                    Ok(response) => {
                        crate::utils::debug_log("EXECUTE_FLOW", &format!("Command sent to server successfully: {:?}", response));
                        
                        // Check process health after command execution
                        crate::process_monitor::check_system_health();
                        
                        return crate::CommandTarget::Command(command.clone());
                    }
                    Err(e) => {
                        crate::utils::debug_log("EXECUTE_FLOW", &format!("Failed to send command to server: {:?}", e));
                        // Fall through to error handling
                    }
                }
            }
        }
        
        // If we get here, server execution failed
        crate::utils::log("CMD_SERVER: Failed to execute command via server");
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
    
    // Try to parse as Action first (new protocol), fall back to Command (old protocol)
    let (is_action, command) = if let Ok(action) = serde_json::from_str::<crate::core::unified_actions::Action>(&request_line.trim()) {
        verbose_log("CMD_SERVER", &format!("Received action: type={}", action.action_type()));
        // Convert Action to Command for now (until we fully migrate execute_command_with_env)
        let cmd = crate::Command {
            patch: action.get_string("patch").unwrap_or("").to_string(),
            command: action.get_string("command_name").unwrap_or("").to_string(),
            action: action.action_type().to_string(),
            arg: action.get_string("arg").unwrap_or("").to_string(),
            flags: action.get_string("flags").unwrap_or("").to_string(),
        };
        (true, cmd)
    } else {
        // Fall back to old Command protocol
        let command: crate::Command = serde_json::from_str(&request_line.trim())?;
        verbose_log("CMD_SERVER", &format!("Received command (legacy): {:?}", command));
        (false, command)
    };
    
    if is_action {
        verbose_log("CMD_SERVER", "Using new Action protocol");
    }
    
    // Check if this is a GUI command that needs blocking execution
    // GUI commands have "G" in their flags field
    let needs_blocking = command.flags.contains("G");
    
    if needs_blocking {
        // For blocking commands (shell_sync), execute synchronously and wait for result
        verbose_log("CMD_SERVER", &format!("Executing blocking command: {:?}", command));
        let result = execute_command_with_env(command.clone(), inherited_env.clone(), base_working_dir.clone());
        verbose_log("CMD_SERVER", &format!("Blocking command completed: {:?}", result));
        
        // Send the actual result back
        let response_json = serde_json::to_string(&result)?;
        stream.write_all(response_json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
    } else {
        // Spawn command execution in a separate thread for async commands
        // This allows us to send immediate ACK while command runs
        let command_clone = command.clone();
        let env_clone = inherited_env.clone();
        let dir_clone = base_working_dir.clone();
        
        thread::spawn(move || {
            verbose_log("CMD_SERVER", &format!("Executing command in background: {:?}", command_clone));
            let result = execute_command_with_env(command_clone, env_clone, dir_clone);
            verbose_log("CMD_SERVER", &format!("Command execution result: {:?}", result));
        });
        
        // Send immediate success ACK for async commands
        // The command is now running in background
        verbose_log("CMD_SERVER", "Sending immediate ACK (command spawned in background)");
        let ack_response = CommandResponse {
            success: true,
            exit_code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
            error: None,
        };
        let response_json = serde_json::to_string(&ack_response)?;
        stream.write_all(response_json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
    }
    
    verbose_log("CMD_SERVER", "Response sent to client");
    Ok(())
}

/// Main entry point for executing actions in the server - routes based on action type
pub fn execute(action: &crate::core::unified_actions::Action) -> Result<String, String> {
    verbose_log("CMD_SERVER", &format!("Server execute: type={}", action.action_type()));
    
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
pub fn execute_locally(action: &crate::core::unified_actions::Action) -> Result<String, String> {
    verbose_log("CMD_SERVER", &format!("Executing action locally: type={}", action.action_type()));
    
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
            let cmd = crate::Command {
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

/// Execute a command with the inherited environment
fn execute_command_with_env(
    command: crate::Command,
    inherited_env: HashMap<String, String>,
    base_working_dir: PathBuf,
) -> CommandResponse {
    // Add a separator and clean command execution log
    crate::utils::log("------");
    
    // Format: CMD  name:XXXX  action:XXXX  flags:XXX  arg:XXX
    // (no space after colon, two spaces between fields)
    let mut log_msg = format!("name:{}  action:{}", command.command, command.action);
    
    // Only include flags if not empty
    if !command.flags.is_empty() {
        log_msg.push_str(&format!("  flags:{}", command.flags));
    }
    
    // Include arg if not empty
    if !command.arg.is_empty() {
        log_msg.push_str(&format!("  arg:{}", command.arg));
    }
    
    log_and_print("CMD", &log_msg);
    
    // Detailed logging goes to debug only
    verbose_log("CMD_SERVER", &format!("Command object: {:?}", command));
    
    // Check if we should use the launcher system for this action
    let is_launcher_command = matches!(
        command.action.as_str(),
        "app" | "url" | "cmd" | "chrome" | "safari" | "brave" | "firefox" | "work" | 
        "obs" | "obs_url" | "1pass" | "anchor" | "folder" | 
        "doc" | "markdown" | "text" | "slack" | "contact" | "alias" |
        "activate_anchor" | "tmux_activate"
    );
    
    verbose_log("CMD_SERVER", &format!("Is launcher command: {}", is_launcher_command));
    
    if is_launcher_command {
        // Use unified actions for known action types
        verbose_log("CMD_SERVER", &format!("Using unified actions for action: {}", command.action));
        
        // Check if this is a GUI command that needs blocking execution
        // GUI commands have "G" in their flags field
        let needs_blocking = command.flags.contains("G");
        
        if command.arg.contains("ec") {
            eprintln!("[CMD_SERVER ACTIONS] EC command in actions path");
            eprintln!("[CMD_SERVER ACTIONS] Command object: {:?}", command);
            eprintln!("[CMD_SERVER ACTIONS] needs_blocking: {} (flags: '{}')", needs_blocking, command.flags);
        }
        
        // Look up the action in config
        let full_config = crate::core::sys_data::get_config();
        if let Some(actions) = &full_config.actions {
            if let Some(action_def) = actions.get(&command.action) {
                verbose_log("CMD_SERVER", &format!("Found action '{}' in config (type: {})", command.action, action_def.action_type));
                
                if needs_blocking {
                    // Execute action synchronously for GUI commands
                    verbose_log("CMD_SERVER", "Executing action synchronously (blocking)");
                    match crate::core::unified_actions::execute_action(
                        action_def,
                        Some(&command.arg),
                        None
                    ) {
                        Ok(result) => {
                            verbose_log("CMD_SERVER", &format!("Action completed: {}", result));
                            log_and_print("CMD", &format!("✓ Completed: {}", command.action));
                            return CommandResponse {
                                success: true,
                                exit_code: Some(0),
                                stdout: String::new(),
                                stderr: String::new(),
                                error: None,
                            };
                        }
                        Err(e) => {
                            let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
                            crate::utils::log_error(&format!("[{}] Action failed: {}", timestamp, e));
                            return CommandResponse {
                                success: false,
                                exit_code: Some(1),
                                stdout: String::new(),
                                stderr: format!("Action failed: {}", e),
                                error: Some(e.to_string()),
                            };
                        }
                    }
                } else {
                    // Spawn action execution in a separate thread for non-blocking commands
                    let action_clone = action_def.clone();
                    let arg_clone = command.arg.clone();
                    let action_name = command.action.clone();
                    thread::spawn(move || {
                        match crate::core::unified_actions::execute_action(
                            &action_clone,
                            Some(&arg_clone),
                            None
                        ) {
                            Ok(result) => {
                                verbose_log("CMD_SERVER", &format!("Action completed: {}", result));
                                crate::utils::log(&format!("✓ Action completed: {}", action_name));
                            }
                            Err(e) => {
                                let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
                                crate::utils::log_error(&format!("[{}] Action failed: {}", timestamp, e));
                            }
                        }
                    });
                    
                    // Return immediate success for spawning the action
                    log_and_print("CMD", &format!("✓ Spawned: {}", command.action));
                    return CommandResponse {
                        success: true,
                        exit_code: Some(0),
                        stdout: String::new(),
                        stderr: String::new(),
                        error: None,
                    };
                }
            } else {
                // Action not found in config, fall through to shell execution
                verbose_log("CMD_SERVER", &format!("Action '{}' not found in config, falling back to shell", command.action));
            }
        }
    }
    
    // Fall back to shell execution for non-launcher commands
    // This shouldn't happen with the new Command object approach, but keep as fallback
    verbose_log("CMD_SERVER", &format!("Non-launcher command, using shell execution for: {} {}", command.action, command.arg));
    
    // Check if this is a GUI command that needs blocking execution
    let needs_blocking = command.flags.contains("G");
    
    // For shell execution, just use the arg as the command
    let shell_command = &command.arg;
    
    if needs_blocking {
        verbose_log("CMD_SERVER", &format!("Using blocking execution (flags='{}')", command.flags));
    } else {
        verbose_log("CMD_SERVER", "Using non-blocking execution (spawn)");
    }
    
    let mut cmd = std::process::Command::new("sh");
    cmd.arg("-c").arg(shell_command);
    
    // Set working directory
    cmd.current_dir(&base_working_dir);
    
    // Apply inherited environment
    for (key, value) in &inherited_env {
        cmd.env(key, value);
    }
    
    verbose_log("CMD_SERVER", &format!("Working directory: {:?}", base_working_dir));
    verbose_log("CMD_SERVER", &format!("Environment variables: {}", inherited_env.len()));
    
    // Log the PATH specifically for debugging
    if let Some(path) = inherited_env.get("PATH") {
        verbose_log("CMD_SERVER", &format!("PATH: {}", path));
    } else {
        verbose_log("CMD_SERVER", "PATH: Not found in environment");
    }
    
    // Always spawn commands - GUI or not
    // The difference is that GUI commands (G flag) get reported as "blocking" 
    // so the popup knows to wait/hide, but we still spawn them
    verbose_log("CMD_SERVER", &format!("Spawning command (GUI={})", needs_blocking));
    match cmd.spawn() {
        Ok(mut child) => {
            verbose_log("CMD_SERVER", &format!("Command spawned with PID: {:?}", child.id()));
            
            // Return success immediately for all spawned commands
            CommandResponse {
                success: true,
                exit_code: Some(0),
                stdout: String::new(),
                stderr: String::new(),
                error: None,
            }
        }
        Err(e) => {
            verbose_log("CMD_SERVER", &format!("Command spawn failed: {}", e));
            
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
    
    /// Execute a command via the server with timeout and ACK verification
    pub fn execute_command(
        &self,
        command: &crate::Command,
    ) -> Result<CommandResponse, Box<dyn std::error::Error>> {
        verbose_log("CMD_CLIENT", &format!("Sending command to server: {:?}", command));
        
        // Connect to server with timeout
        let mut stream = UnixStream::connect(&self.socket_path)?;
        
        // Set read timeout for ACK verification
        stream.set_read_timeout(Some(std::time::Duration::from_millis(1000)))?;
        
        // Send command as JSON
        let command_json = serde_json::to_string(&command)?;
        stream.write_all(command_json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
        
        // Always wait for ACK/response (server sends immediate response for both modes)
        let mut response_data = String::new();
        match stream.read_to_string(&mut response_data) {
            Ok(_) => {
                let response: CommandResponse = serde_json::from_str(&response_data.trim())?;
                verbose_log("CMD_CLIENT", &format!("Received ACK/response: success={}", response.success));
                Ok(response)
            }
            Err(e) => {
                // Timeout or connection error - server is not responding
                verbose_log("CMD_CLIENT", &format!("Failed to receive ACK from server: {}", e));
                Err(format!("Server not responding: {}", e).into())
            }
        }
    }
    
    /// Send raw JSON to the server (for Actions)
    pub fn send_json(&self, json: &str) -> Result<CommandResponse, Box<dyn std::error::Error>> {
        verbose_log("CMD_CLIENT", &format!("Sending JSON to server: {}", json));
        
        // Connect to server
        let mut stream = UnixStream::connect(&self.socket_path)?;
        
        // Set read timeout for response
        stream.set_read_timeout(Some(std::time::Duration::from_millis(1000)))?;
        
        // Send JSON
        stream.write_all(json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
        
        // Wait for response
        let mut response_data = String::new();
        match stream.read_to_string(&mut response_data) {
            Ok(_) => {
                let response: CommandResponse = serde_json::from_str(&response_data.trim())?;
                verbose_log("CMD_CLIENT", &format!("Received response: success={}", response.success));
                Ok(response)
            }
            Err(e) => {
                verbose_log("CMD_CLIENT", &format!("Failed to receive response from server: {}", e));
                Err(format!("Server not responding: {}", e).into())
            }
        }
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

/// Simple helper to create a Command from action and arg (covers 90% of cases)
pub fn make_command(action: &str, arg: &str) -> crate::Command {
    crate::Command {
        patch: String::new(),
        command: if arg.is_empty() { action.to_string() } else { format!("{}: {}", action, arg) },
        action: action.to_string(),
        arg: arg.to_string(),
        flags: String::new(),
    }
}

/// Helper for shell commands with optional flags
pub fn shell_command(cmd: &str, flags: &str) -> crate::Command {
    make_command("shell", cmd).with_flags(flags)
}

// Extension trait to add flags to a Command
impl crate::Command {
    pub fn with_flags(mut self, flags: &str) -> Self {
        self.flags = flags.to_string();
        self
    }
}

/// Execute a command using the global server with retry and restart logic
/// This function NEVER fails from the caller's perspective - it handles all retries internally
/// If the server can't be started after MAX_RETRIES attempts:
///   - Shows error dialog to user
///   - Exits popup if in GUI mode
/// Otherwise guarantees command delivery
pub fn execute_via_server(command: &crate::Command) {
    const MAX_RETRIES: u32 = 5;  // Increased for better resilience
    const ACK_TIMEOUT_MS: u64 = 1000;
    
    // Convert Command to Action for the new protocol
    let action = crate::core::unified_actions::command_to_action(command);
    let action_json = serde_json::to_string(&action).unwrap_or_else(|e| {
        verbose_log("CMD_SERVER", &format!("Failed to serialize action: {}", e));
        // Fall back to old protocol
        serde_json::to_string(command).unwrap()
    });
    
    for attempt in 1..=MAX_RETRIES {
        verbose_log("CMD_SERVER", &format!("Command execution attempt {} of {}", attempt, MAX_RETRIES));
        
        // Try to send action and get ACK
        if let Ok(client) = CommandClient::new() {
            match client.send_json(&action_json) {
                Ok(_response) => {
                    // Success! Command was delivered
                    verbose_log("CMD_SERVER", &format!("Action executed successfully on attempt {}", attempt));
                    return;  // Done - command delivered
                }
                Err(e) => {
                    verbose_log("CMD_SERVER", &format!("Action failed on attempt {}: {}", attempt, e));
                }
            }
        }
        
        // If we've exhausted all retries, show error and exit
        if attempt == MAX_RETRIES {
            let error_msg = format!(
                "Critical Error: Command server is completely unresponsive after {} attempts.\n\n\
                The server cannot be started or is experiencing critical issues.\n\
                Please restart HookAnchor manually.",
                MAX_RETRIES
            );
            
            // Show error to user
            crate::error_display::queue_user_error(&error_msg);
            crate::utils::log_error(&error_msg);
            
            // If we're in popup mode, exit gracefully
            #[cfg(feature = "gui")]
            {
                std::thread::sleep(std::time::Duration::from_secs(2)); // Let user see the error
                std::process::exit(1);
            }
            
            return;  // Give up - server is truly broken
        }
        
        // Server not responding - restart it for next attempt
        verbose_log("CMD_SERVER", &format!("Restarting server for attempt {}", attempt + 1));
        crate::utils::debug_log("CMD_SERVER", &format!("Restarting server for command: {}", command.command));
        
        // Kill existing server and reset check
        let _ = crate::command_server_management::kill_existing_server();
        crate::command_server_management::reset_server_check();
        
        // Start new server
        if let Err(e) = crate::command_server_management::start_server_if_needed() {
            verbose_log("CMD_SERVER", &format!("Failed to restart server: {}", e));
            // Continue trying - don't give up yet
        }
        
        // Brief delay before retry
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
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