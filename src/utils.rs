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

/// Execute a shell command using `/bin/sh -c`
/// 
/// Consolidates the common pattern of shell command execution
pub fn execute_shell_command(command: &str) -> Result<std::process::Output, std::io::Error> {
    Command::new("/bin/sh").arg("-c").arg(command).output()
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