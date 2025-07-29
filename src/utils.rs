//! Shared utility functions used across multiple modules
//!
//! This module consolidates commonly used functions to avoid duplication
//! and ensure consistency across the codebase.

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use chrono::Local;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Cached login shell environment variables
static LOGIN_ENV_CACHE: OnceLock<Mutex<Option<HashMap<String, String>>>> = OnceLock::new();

/// Debug logging function used across all modules
/// 
/// Logs messages to the debug file specified in config.popup_settings.debug_log
/// with a timestamp and module identifier.
pub fn debug_log(module: &str, message: &str) {
    let config = crate::core::sys_data::get_config();
    if let Some(debug_path) = &config.popup_settings.debug_log {
        let debug_path = expand_tilde(debug_path);
        
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        
        let log_entry = format!("{} {}: {}\n", timestamp, module, message);
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(debug_path) {
            let _ = file.write_all(log_entry.as_bytes());
        } // File handle is dropped here, closing the file
    }
}

/// Verbose debug logging function for detailed debugging
/// 
/// Only logs messages when verbose_logging is enabled in config.
/// Used for shell commands, JavaScript execution, and other detailed debugging.
pub fn verbose_log(module: &str, message: &str) {
    let config = crate::core::sys_data::get_config();
    // Check if verbose logging is enabled (default to false if not set)
    if config.popup_settings.verbose_logging.unwrap_or(false) {
        debug_log(module, message);
    }
}

/// Expands ~ in paths to the home directory
/// 
/// This is a common operation needed across multiple modules for handling
/// user-provided paths that may use the ~ shorthand.
pub fn expand_tilde(path: &str) -> String {
    if path.starts_with('~') {
        if let Ok(home) = std::env::var("HOME") {
            path.replacen('~', &home, 1)
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    }
}

/// Launch an app with optional argument using macOS `open` command
/// 
/// Consolidates the common pattern of launching apps with the `-a` flag
/// Uses non-blocking spawn to prevent UI lockups
pub fn launch_app_with_arg(app: &str, arg: Option<&str>) -> Result<std::process::Output, std::io::Error> {
    debug_log("UTILS", &format!("launch_app_with_arg: app='{}', arg={:?}", app, arg));
    
    let mut cmd = Command::new("open");
    cmd.arg("-a").arg(app);
    if let Some(arg_val) = arg {
        if !arg_val.is_empty() {
            cmd.arg(arg_val);
        }
    }
    
    // Use spawn + detach for non-blocking execution to prevent UI lockups
    debug_log("UTILS", "Spawning non-blocking open command");
    let child = cmd.spawn()?;
    
    // Register the process for monitoring
    let command_str = format!("open -a {} {}", app, arg.unwrap_or(""));
    let process_id = crate::process_monitor::register_process(child, command_str);
    
    debug_log("UTILS", &format!("Process spawned successfully (ID: {}), returning immediate success", process_id));
    
    // For non-blocking execution, we don't wait for the result
    // The application will open independently without blocking the UI
    Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "NON_BLOCKING_SUCCESS: Process spawned successfully"
    ))
}

/// Open a URL using macOS `open` command
/// 
/// Consolidates the common pattern of opening URLs  
/// Uses non-blocking spawn to prevent UI lockups
pub fn open_url(url: &str) -> Result<std::process::Output, std::io::Error> {
    debug_log("UTILS", &format!("open_url: url='{}'", url));
    
    debug_log("UTILS", "Spawning non-blocking open command for URL");
    let child = Command::new("open").arg(url).spawn()?;
    
    // Register the process for monitoring
    let command_str = format!("open {}", url);
    let process_id = crate::process_monitor::register_process(child, command_str);
    
    debug_log("UTILS", &format!("URL open process spawned successfully (ID: {})", process_id));
    
    // For non-blocking execution, we don't wait for the result
    Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "NON_BLOCKING_SUCCESS: URL process spawned successfully"
    ))
}

/// Open a folder using macOS `open` command
/// 
/// Consolidates the common pattern of opening folders in Finder
/// Uses non-blocking spawn to prevent UI lockups
pub fn open_folder(path: &str) -> Result<std::process::Output, std::io::Error> {
    let expanded_path = expand_tilde(path);
    debug_log("UTILS", &format!("open_folder: path='{}' -> '{}'", path, expanded_path));
    
    debug_log("UTILS", "Spawning non-blocking open command for folder");
    let child = Command::new("open").arg(&expanded_path).spawn()?;
    
    // Register the process for monitoring
    let command_str = format!("open {}", expanded_path);
    let process_id = crate::process_monitor::register_process(child, command_str);
    
    debug_log("UTILS", &format!("Folder open process spawned successfully (ID: {})", process_id));
    
    // For non-blocking execution, we don't wait for the result
    Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "NON_BLOCKING_SUCCESS: Folder process spawned successfully"
    ))
}

/// Capture environment variables from a login shell (cached)
fn get_login_environment() -> Result<HashMap<String, String>, std::io::Error> {
    let cache = LOGIN_ENV_CACHE.get_or_init(|| Mutex::new(None));
    let mut cache_guard = cache.lock().unwrap();
    
    if let Some(ref env) = *cache_guard {
        verbose_log("SHELL", "Using cached login environment");
        return Ok(env.clone());
    }
    
    verbose_log("SHELL", "Capturing login shell environment for first time");
    
    // Get the user's shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
    
    verbose_log("SHELL", &format!("Using shell: {}", shell));
    
    // Run the login shell directly to capture environment
    let output = Command::new(&shell)
        .arg("-l")
        .arg("-c")
        .arg("env")
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to capture login environment: {}", stderr)
        ));
    }
    
    let env_output = String::from_utf8_lossy(&output.stdout);
    let mut env_map = HashMap::new();
    
    for line in env_output.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env_map.insert(key.to_string(), value.to_string());
        }
    }
    
    verbose_log("SHELL", &format!("Captured {} environment variables", env_map.len()));
    *cache_guard = Some(env_map.clone());
    
    Ok(env_map)
}

/// Shell execution with simple current environment (original approach)
pub fn shell_simple(command: &str, blocking: bool) -> Result<std::process::Output, std::io::Error> {
    verbose_log("SHELL", &format!("SIMPLE: {}", command));
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    
    if blocking {
        cmd.output()
    } else {
        match cmd.spawn() {
            Ok(_) => {
                // Return dummy success output for non-blocking
                // Use a command that always succeeds to get a proper ExitStatus
                let dummy_output = Command::new("true").output()?;
                Ok(std::process::Output {
                    status: dummy_output.status,
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            },
            Err(e) => Err(e),
        }
    }
}

/// Shell execution with full login shell environment (su - approach)
pub fn shell_login(command: &str, blocking: bool) -> Result<std::process::Output, std::io::Error> {
    verbose_log("SHELL", &format!("LOGIN: {}", command));
    
    let current_user = detect_current_user();
    let escaped_command = command.replace("'", "'\"'\"'");
    let final_command = format!("su - {} -c '{}'", current_user, escaped_command);
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(&final_command);
    
    if blocking {
        cmd.output()
    } else {
        match cmd.spawn() {
            Ok(_) => {
                let dummy_output = Command::new("true").output()?;
                Ok(std::process::Output {
                    status: dummy_output.status,
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            },
            Err(e) => Err(e),
        }
    }
}

/// Shell execution with hybrid approach: current GUI environment + login shell env vars
pub fn shell_hybrid(command: &str, blocking: bool) -> Result<std::process::Output, std::io::Error> {
    verbose_log("SHELL", &format!("HYBRID: {}", command));
    
    // Check if tmux is requested
    if command.contains("tmux") {
        verbose_log("SHELL", "tmux command detected, checking availability");
    }
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    
    // Try to get login environment, but fall back to simple execution if it fails
    match get_login_environment() {
        Ok(login_env) => {
            verbose_log("SHELL", &format!("Applying {} login environment variables", login_env.len()));
            
            // Debug: Log PATH from login environment
            if let Some(path) = login_env.get("PATH") {
                verbose_log("SHELL", &format!("Login PATH: {}", path));
            }
            
            // Apply login environment variables to current process environment
            for (key, value) in &login_env {
                cmd.env(key, value);
            }
        },
        Err(e) => {
            verbose_log("SHELL", &format!("Failed to get login environment ({}), falling back to simple execution", e));
            // Continue with current environment
        }
    }
    
    if blocking {
        cmd.output()
    } else {
        match cmd.spawn() {
            Ok(_) => {
                let dummy_output = Command::new("true").output()?;
                Ok(std::process::Output {
                    status: dummy_output.status,
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            },
            Err(e) => Err(e),
        }
    }
}

/// Execute a shell command using the user's shell with proper environment
/// 
/// Uses the hybrid approach by default (current GUI environment + login shell env vars)
pub fn execute_shell_command(command: &str) -> Result<std::process::Output, std::io::Error> {
    shell_hybrid(command, true)
}

#[derive(Debug, Clone)]
pub struct ShellOptions {
    pub blocking: bool,      // true = wait for completion, false = spawn and return
    pub login_shell: bool,   // true = use login shell with full environment
    pub inherit_env: bool,   // true = inherit current environment variables
}

impl Default for ShellOptions {
    fn default() -> Self {
        Self {
            blocking: true,
            login_shell: true,
            inherit_env: true,
        }
    }
}

/// Core shell command execution with proper user environment
/// 
/// This function is shared between the utils module and JavaScript runtime
/// to ensure consistent shell execution behavior across the application
pub fn execute_shell_command_with_env(command: &str) -> Result<std::process::Output, std::io::Error> {
    shell_hybrid(command, true)
}

/// Unified shell command execution with configurable options
/// 
/// This replaces all the various shell execution functions with a single, 
/// configurable approach that properly handles user environments
pub fn execute_shell_command_unified(command: &str, blocking: bool, detached: bool) -> Result<std::process::Output, std::io::Error> {
    let options = ShellOptions {
        blocking,
        login_shell: true,
        inherit_env: true,
    };
    execute_shell_with_options(command, options, detached)
}

pub fn execute_shell_with_options(command: &str, options: ShellOptions, detached: bool) -> Result<std::process::Output, std::io::Error> {
    verbose_log("SHELL", &format!("Executing command with options: {:?}", options));
    verbose_log("SHELL", &format!("Command: {}", command));
    
    // Detect current user - try multiple methods for reliability
    let current_user = detect_current_user();
    verbose_log("SHELL", &format!("Detected user: {}", current_user));
    
    // Check if this is a GUI command that needs direct execution
    let is_gui_command = is_gui_command(command);
    verbose_log("SHELL", &format!("Is GUI command: {}", is_gui_command));
    
    // Build the command based on options
    let final_command = if options.login_shell && !is_gui_command {
        // Use login shell to get proper environment, but not for GUI commands
        // Escape single quotes in the original command
        let escaped_command = command.replace("'", "'\"'\"'");
        format!("su - {} -c '{}'", current_user, escaped_command)
    } else {
        // Use simple shell execution for GUI commands or when login_shell is false
        command.to_string()
    };
    
    verbose_log("SHELL", &format!("Final command: {}", final_command));
    
    // Execute with appropriate method
    if detached {
        execute_detached(&final_command, options)
    } else if options.blocking {
        execute_blocking(&final_command, options)
    } else {
        execute_non_blocking(&final_command, options)
    }
}

/// Detects if a command is a GUI command that needs direct execution (not through su -)
fn is_gui_command(command: &str) -> bool {
    // List of common GUI commands that need direct execution
    let gui_commands = [
        "open",           // macOS open command
        "osascript",      // AppleScript execution
        "automator",      // Automator workflows
        "say",            // Text-to-speech
        "afplay",         // Audio playback
        "qlmanage",       // Quick Look
        "screencapture",  // Screen capture
        "caffeinate",     // Keep system awake
    ];
    
    // Extract the first word (command name) from the full command
    let command_name = command.trim().split_whitespace().next().unwrap_or("");
    
    // Check if it's a known GUI command
    gui_commands.iter().any(|&gui_cmd| command_name == gui_cmd)
}

fn detect_current_user() -> String {
    // Try multiple methods to detect the current user
    
    // Method 1: USER environment variable
    if let Ok(user) = std::env::var("USER") {
        if !user.is_empty() {
            verbose_log("SHELL", &format!("Found user via USER env var: {}", user));
            return user;
        }
    }
    
    // Method 2: LOGNAME environment variable  
    if let Ok(user) = std::env::var("LOGNAME") {
        if !user.is_empty() {
            verbose_log("SHELL", &format!("Found user via LOGNAME env var: {}", user));
            return user;
        }
    }
    
    // Method 3: whoami command
    if let Ok(output) = Command::new("whoami").output() {
        if output.status.success() {
            let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !user.is_empty() {
                verbose_log("SHELL", &format!("Found user via whoami: {}", user));
                return user;
            }
        }
    }
    
    // Method 4: Parse from HOME directory
    if let Ok(home) = std::env::var("HOME") {
        if let Some(user) = home.split('/').last() {
            if !user.is_empty() {
                verbose_log("SHELL", &format!("Found user via HOME parsing: {}", user));
                return user.to_string();
            }
        }
    }
    
    // Fallback: assume current user from id command
    if let Ok(output) = Command::new("id").arg("-un").output() {
        if output.status.success() {
            let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !user.is_empty() {
                verbose_log("SHELL", &format!("Found user via id -un: {}", user));
                return user;
            }
        }
    }
    
    // Final fallback
    verbose_log("SHELL", "Could not detect user, using fallback 'user'");
    "user".to_string()
}

fn execute_blocking(command: &str, options: ShellOptions) -> Result<std::process::Output, std::io::Error> {
    use std::process::Command;
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    
    if options.inherit_env {
        // Inherit minimal environment
        if let Ok(home) = std::env::var("HOME") {
            cmd.env("HOME", home);
        }
        if let Ok(path) = std::env::var("PATH") {
            cmd.env("PATH", path);
        }
    }
    
    cmd.output()
}

fn execute_non_blocking(command: &str, options: ShellOptions) -> Result<std::process::Output, std::io::Error> {
    use std::process::{Command, Stdio};
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
       .arg(command)
       .stdin(Stdio::null())
       .stdout(Stdio::null())
       .stderr(Stdio::null());
    
    if options.inherit_env {
        if let Ok(home) = std::env::var("HOME") {
            cmd.env("HOME", home);
        }
        if let Ok(path) = std::env::var("PATH") {
            cmd.env("PATH", path);
        }
    }
    
    // For non-blocking, we spawn and return immediately
    match cmd.spawn() {
        Ok(_) => {
            verbose_log("SHELL", "Non-blocking command spawned successfully");
            // For non-blocking commands, just run a quick successful command to get a real ExitStatus
            let dummy_output = Command::new("true").output().unwrap_or_else(|_| {
                std::process::Output {
                    status: std::process::ExitStatus::default(),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                }
            });
            Ok(dummy_output)
        }
        Err(e) => Err(e)
    }
}

fn execute_detached(command: &str, options: ShellOptions) -> Result<std::process::Output, std::io::Error> {
    use std::process::{Command, Stdio};
    
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
       .arg(command)
       .stdin(Stdio::null())
       .stdout(Stdio::null()) 
       .stderr(Stdio::null());
       
    if options.inherit_env {
        if let Ok(home) = std::env::var("HOME") {
            cmd.env("HOME", home);
        }
        if let Ok(path) = std::env::var("PATH") {
            cmd.env("PATH", path);
        }
    }
    
    match cmd.spawn() {
        Ok(_) => {
            verbose_log("SHELL", "Detached command spawned successfully");
            // For detached commands, just run a quick successful command to get a real ExitStatus
            let dummy_output = Command::new("true").output().unwrap_or_else(|_| {
                std::process::Output {
                    status: std::process::ExitStatus::default(),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                }
            });
            Ok(dummy_output)
        }
        Err(e) => Err(e)
    }
}

/// Open a file/URL with a specific app using macOS `open` command
/// 
/// Consolidates the open-with pattern used throughout the codebase
/// Uses non-blocking spawn to prevent UI lockups
pub fn open_with_app(app: &str, target: &str) -> Result<std::process::Output, std::io::Error> {
    debug_log("UTILS", &format!("open_with_app: app='{}', target='{}'", app, target));
    
    // For browsers, add -F flag to bring app to foreground
    let mut cmd = Command::new("open");
    if app.is_empty() {
        cmd.arg(target);
    } else {
        cmd.arg("-F"); // Bring app to foreground
        cmd.arg("-a").arg(app).arg(target);
    }
    
    debug_log("UTILS", &format!("Spawning non-blocking open command: open -a \"{}\" \"{}\"", app, target));
    
    // Add environment info
    debug_log("UTILS", &format!("Current working directory: {:?}", std::env::current_dir().ok()));
    debug_log("UTILS", &format!("USER env var: {:?}", std::env::var("USER").ok()));
    debug_log("UTILS", &format!("HOME env var: {:?}", std::env::var("HOME").ok()));
    
    // Try to spawn the command
    let child = match cmd.spawn() {
        Ok(child) => {
            debug_log("UTILS", &format!("Successfully spawned process with PID: {:?}", child.id()));
            child
        },
        Err(e) => {
            debug_log("UTILS", &format!("Failed to spawn open command: {}", e));
            return Err(e);
        }
    };
    
    // Register the process for monitoring
    let command_str = if app.is_empty() {
        format!("open {}", target)
    } else {
        format!("open -a {} {}", app, target)
    };
    let process_id = crate::process_monitor::register_process(child, command_str.clone());
    
    debug_log("UTILS", &format!("Open with app process spawned successfully (ID: {})", process_id));
    
    // Check if the process is actually running
    std::thread::sleep(std::time::Duration::from_millis(100));
    debug_log("UTILS", &format!("Checking if '{}' process started...", command_str));
    
    // For non-blocking execution, we don't wait for the result
    Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "NON_BLOCKING_SUCCESS: Open with app process spawned successfully"
    ))
}