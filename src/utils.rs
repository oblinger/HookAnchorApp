//! Shared utility functions used across multiple modules
//!
//! This module consolidates commonly used functions to avoid duplication
//! and ensure consistency across the codebase.

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use chrono::Local;

/// Debug logging function used across all modules
/// 
/// Logs messages to the debug file specified in config.popup_settings.debug_log
/// with a timestamp and module identifier.
pub fn debug_log(module: &str, message: &str) {
    let config = crate::load_config();
    if let Some(debug_path) = &config.popup_settings.debug_log {
        let debug_path = expand_tilde(debug_path);
        
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        
        let log_entry = format!("{} {}: {}\n", timestamp, module, message);
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(debug_path) {
            let _ = file.write_all(log_entry.as_bytes());
        }
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
pub fn launch_app_with_arg(app: &str, arg: Option<&str>) -> Result<std::process::Output, std::io::Error> {
    let mut cmd = Command::new("open");
    cmd.arg("-a").arg(app);
    if let Some(arg_val) = arg {
        if !arg_val.is_empty() {
            cmd.arg(arg_val);
        }
    }
    cmd.output()
}

/// Open a URL using macOS `open` command
/// 
/// Consolidates the common pattern of opening URLs
pub fn open_url(url: &str) -> Result<std::process::Output, std::io::Error> {
    Command::new("open").arg(url).output()
}

/// Open a folder using macOS `open` command
/// 
/// Consolidates the common pattern of opening folders in Finder
pub fn open_folder(path: &str) -> Result<std::process::Output, std::io::Error> {
    let expanded_path = expand_tilde(path);
    Command::new("open").arg(&expanded_path).output()
}

/// Execute a shell command using the user's shell with proper environment
/// 
/// Uses the user's login shell and sources their profile to get proper PATH and environment
pub fn execute_shell_command(command: &str) -> Result<std::process::Output, std::io::Error> {
    execute_shell_command_unified(command, true, false)
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
    execute_shell_command_unified(command, true, false)
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
    debug_log("SHELL", &format!("Executing command with options: {:?}", options));
    debug_log("SHELL", &format!("Command: {}", command));
    
    // Detect current user - try multiple methods for reliability
    let current_user = detect_current_user();
    debug_log("SHELL", &format!("Detected user: {}", current_user));
    
    // Build the command based on options
    let final_command = if options.login_shell {
        // Use login shell to get proper environment
        // Escape single quotes in the original command
        let escaped_command = command.replace("'", "'\"'\"'");
        format!("su - {} -c '{}'", current_user, escaped_command)
    } else {
        // Use simple shell execution
        command.to_string()
    };
    
    debug_log("SHELL", &format!("Final command: {}", final_command));
    
    // Execute with appropriate method
    if detached {
        execute_detached(&final_command, options)
    } else if options.blocking {
        execute_blocking(&final_command, options)
    } else {
        execute_non_blocking(&final_command, options)
    }
}

fn detect_current_user() -> String {
    // Try multiple methods to detect the current user
    
    // Method 1: USER environment variable
    if let Ok(user) = std::env::var("USER") {
        if !user.is_empty() {
            debug_log("SHELL", &format!("Found user via USER env var: {}", user));
            return user;
        }
    }
    
    // Method 2: LOGNAME environment variable  
    if let Ok(user) = std::env::var("LOGNAME") {
        if !user.is_empty() {
            debug_log("SHELL", &format!("Found user via LOGNAME env var: {}", user));
            return user;
        }
    }
    
    // Method 3: whoami command
    if let Ok(output) = Command::new("whoami").output() {
        if output.status.success() {
            let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !user.is_empty() {
                debug_log("SHELL", &format!("Found user via whoami: {}", user));
                return user;
            }
        }
    }
    
    // Method 4: Parse from HOME directory
    if let Ok(home) = std::env::var("HOME") {
        if let Some(user) = home.split('/').last() {
            if !user.is_empty() {
                debug_log("SHELL", &format!("Found user via HOME parsing: {}", user));
                return user.to_string();
            }
        }
    }
    
    // Fallback: assume current user from id command
    if let Ok(output) = Command::new("id").arg("-un").output() {
        if output.status.success() {
            let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !user.is_empty() {
                debug_log("SHELL", &format!("Found user via id -un: {}", user));
                return user;
            }
        }
    }
    
    // Final fallback
    debug_log("SHELL", "Could not detect user, using fallback 'user'");
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
            debug_log("SHELL", "Non-blocking command spawned successfully");
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
            debug_log("SHELL", "Detached command spawned successfully");
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
pub fn open_with_app(app: &str, target: &str) -> Result<std::process::Output, std::io::Error> {
    if app.is_empty() {
        Command::new("open").arg(target).output()
    } else {
        Command::new("open").arg("-a").arg(app).arg(target).output()
    }
}