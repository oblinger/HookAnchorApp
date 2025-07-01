//! Shared utility functions used across multiple modules
//!
//! This module consolidates commonly used functions to avoid duplication
//! and ensure consistency across the codebase.

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Debug logging function used across all modules
/// 
/// Logs messages to the debug file specified in config.popup_settings.debug_log
/// with a timestamp and module identifier.
pub fn debug_log(module: &str, message: &str) {
    let config = crate::load_config();
    if let Some(debug_path) = &config.popup_settings.debug_log {
        let debug_path = expand_tilde(debug_path);
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let log_entry = format!("[{}] {}: {}\n", timestamp, module, message);
        
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
    execute_shell_command_with_env(command)
}

/// Core shell command execution with proper user environment
/// 
/// This function is shared between the utils module and JavaScript runtime
/// to ensure consistent shell execution behavior across the application
pub fn execute_shell_command_with_env(command: &str) -> Result<std::process::Output, std::io::Error> {
    // Get the user's shell from SHELL environment variable, fallback to zsh
    let user_shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
    
    // Add debug logging for command execution
    debug_log("SHELL", &format!("Executing command: {}", command));
    debug_log("SHELL", &format!("Using shell: {}", user_shell));
    
    // For commands starting with underscore, try to execute them directly first
    // since zsh might interpret them as completion functions
    if command.starts_with('_') {
        debug_log("SHELL", "Command starts with underscore, trying direct execution");
        
        // Try to find the command in PATH first
        let which_command = format!("which {}", command);
        let wrapped_which = format!(
            "source ~/.zshrc 2>/dev/null || source ~/.bash_profile 2>/dev/null || source ~/.bashrc 2>/dev/null || true; {}",
            which_command
        );
        
        let mut which_cmd = Command::new(&user_shell);
        which_cmd.arg("-c").arg(&wrapped_which);
        if let Ok(current_path) = std::env::var("PATH") {
            which_cmd.env("PATH", current_path);
        }
        
        if let Ok(which_output) = which_cmd.output() {
            if which_output.status.success() {
                let command_path = String::from_utf8_lossy(&which_output.stdout).trim().to_string();
                debug_log("SHELL", &format!("Found command at: {}", command_path));
                
                // Execute the command directly using its full path
                let direct_command = format!("exec '{}'", command_path);
                let wrapped_direct = format!(
                    "source ~/.zshrc 2>/dev/null || source ~/.bash_profile 2>/dev/null || source ~/.bashrc 2>/dev/null || true; {}",
                    direct_command
                );
                
                let mut direct_cmd = Command::new(&user_shell);
                direct_cmd.arg("-c").arg(&wrapped_direct);
                direct_cmd.env("HOME", std::env::var("HOME").unwrap_or_else(|_| "/Users/oblinger".to_string()));
                if let Ok(current_path) = std::env::var("PATH") {
                    direct_cmd.env("PATH", current_path);
                }
                
                debug_log("SHELL", &format!("Executing direct command: {}", direct_command));
                return direct_cmd.output();
            }
        }
    }
    
    // Create a command that sources the user's profile first, then runs the command
    // This ensures we get the user's full PATH and environment
    let wrapped_command = format!(
        "source ~/.zshrc 2>/dev/null || source ~/.bash_profile 2>/dev/null || source ~/.bashrc 2>/dev/null || true; {}",
        command
    );
    
    debug_log("SHELL", &format!("Executing wrapped command: {}", wrapped_command));
    
    let mut cmd = Command::new(&user_shell);
    cmd.arg("-c").arg(&wrapped_command);
    
    // Inherit environment from current process (which should have basic PATH)
    cmd.env("HOME", std::env::var("HOME").unwrap_or_else(|_| "/Users/oblinger".to_string()));
    
    // Set a basic PATH in case the shell profile doesn't load properly
    if let Ok(current_path) = std::env::var("PATH") {
        cmd.env("PATH", current_path);
    }
    
    cmd.output()
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