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

/// Clear the debug log file
/// 
/// Removes the debug log file at the hardcoded path ~/.config/hookanchor/anchor.log
/// to start fresh logging. Used before rebuilds and when log exceeds max size.
pub fn clear_debug_log() {
    // Use constant from sys_data
    let debug_path = expand_tilde(crate::core::sys_data::DEFAULT_LOG_PATH);
    
    // Remove the file if it exists  
    if std::path::Path::new(&debug_path).exists() {
        let _ = std::fs::remove_file(&debug_path);
    }
}

/// Check if debug log file exceeds max size and clear if needed
/// 
/// Returns true if log was cleared, false otherwise
pub fn check_and_clear_oversized_log() -> bool {
    // Use constant from sys_data
    let debug_path = expand_tilde(crate::core::sys_data::DEFAULT_LOG_PATH);
    
    // Get max size from config if available, otherwise use default
    let max_size = match crate::core::sys_data::CONFIG.get() {
        Some(cfg) => cfg.popup_settings.max_log_file_size.unwrap_or(1_000_000), // 1MB default
        None => crate::core::sys_data::DEFAULT_MAX_LOG_SIZE,
    };
    
    // Log when we check (to temp file to avoid recursion)
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/hookanchor_log_checks.log") {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(file, "{} Checking log size - max_size configured: {}", timestamp, max_size);
    }
    
    if let Ok(metadata) = std::fs::metadata(&debug_path) {
        let current_size = metadata.len();
        
        if current_size > max_size {
            // First log the truncation event to a separate file to avoid recursion
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("/tmp/hookanchor_log_truncation.log") {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
                let _ = writeln!(file, "{} Log file exceeded {} bytes (was {} bytes), clearing log file at: {}", 
                    timestamp, max_size, current_size, debug_path);
            }
            
            clear_debug_log();
            // Now we can safely log to the fresh file
            log(&format!("LOG_MANAGEMENT: Log file exceeded {} bytes (was {} bytes), cleared for fresh start", max_size, current_size));
            return true;
        }
    }
    false
}

/// Clear the log file unconditionally
/// 
/// This is used when we want to start fresh, such as during a rebuild
pub fn clear_log_file() {
    clear_debug_log();
}

/// Simple logging function that checks if logging is enabled
/// 
/// This is the primary logging function that should be used throughout the codebase.
/// It checks if a debug log path is configured before writing.
pub fn log(message: &str) {
    // Use constant from sys_data for consistency
    let debug_path = expand_tilde(crate::core::sys_data::DEFAULT_LOG_PATH);
    
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("{} {}\n", timestamp, message);
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(debug_path) {
        let _ = file.write_all(log_entry.as_bytes());
    }
}

/// Detailed logging function that only logs when detailed_logging is enabled
/// 
/// This function should be used for verbose logging that would normally be too noisy,
/// such as logging every key press or detailed execution flow.
pub fn detailed_log(module: &str, message: &str) {
    // Check if detailed logging is enabled
    let verbose_enabled = match crate::core::sys_data::CONFIG.get() {
        Some(cfg) => cfg.popup_settings.verbose_logging.unwrap_or(false),
        None => false, // Config not loaded yet, assume verbose is off
    };
    
    if verbose_enabled {
        log(&format!("{}: {}", module, message));
    }
}

/// Error logging function that marks errors clearly in the log
/// 
/// This replaces eprintln! calls for errors that should go to the log file
/// instead of stderr. The error is always logged (not conditional on verbose mode).
pub fn log_error(message: &str) {
    log(&format!("❌ ERROR: {}", message));
}

/// Error logging with module context
pub fn log_error_module(module: &str, message: &str) {
    log(&format!("❌ ERROR [{}]: {}", module, message));
}

/// Legacy debug log function - now just calls log() with formatted message
/// 
/// Kept for backward compatibility. New code should use log() or detailed_log().
pub fn debug_log(module: &str, message: &str) {
    log(&format!("{}: {}", module, message));
}

/// Verbose debug logging function for detailed debugging
/// 
/// This is now an alias for detailed_log. Kept for backward compatibility.
pub fn verbose_log(module: &str, message: &str) {
    detailed_log(module, message);
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
    
    let mut cmd = Command::new("open");
    cmd.arg("-a").arg(app);
    if let Some(arg_val) = arg {
        if !arg_val.is_empty() {
            cmd.arg(arg_val);
        }
    }
    
    // Use spawn + detach for non-blocking execution to prevent UI lockups
    let child = cmd.spawn()?;
    
    // Register the process for monitoring
    let command_str = format!("open -a {} {}", app, arg.unwrap_or(""));
    let _process_id = crate::process_monitor::register_process(child, command_str);
    
    
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
    
    let child = Command::new("open").arg(url).spawn()?;
    
    // Register the process for monitoring
    let command_str = format!("open {}", url);
    let _process_id = crate::process_monitor::register_process(child, command_str);
    
    
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
    
    let child = Command::new("open").arg(&expanded_path).spawn()?;
    
    // Register the process for monitoring
    let command_str = format!("open {}", expanded_path);
    let _process_id = crate::process_monitor::register_process(child, command_str);
    
    
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
    
    // For browsers, add -F flag to bring app to foreground
    let mut cmd = Command::new("open");
    if app.is_empty() {
        cmd.arg(target);
    } else {
        cmd.arg("-F"); // Bring app to foreground
        cmd.arg("-a").arg(app).arg(target);
    }
    
    
    // Add environment info
    
    // Try to spawn the command
    let child = match cmd.spawn() {
        Ok(child) => {
            child
        },
        Err(e) => {
            log_error(&format!("Failed to spawn open command: {}", e));
            return Err(e);
        }
    };
    
    // Register the process for monitoring
    let command_str = if app.is_empty() {
        format!("open {}", target)
    } else {
        format!("open -a {} {}", app, target)
    };
    let _process_id = crate::process_monitor::register_process(child, command_str.clone());
    
    
    // Check if the process is actually running
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // For non-blocking execution, we don't wait for the result
    Err(std::io::Error::new(
        std::io::ErrorKind::Other, 
        "NON_BLOCKING_SUCCESS: Open with app process spawned successfully"
    ))
}