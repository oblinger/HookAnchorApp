//! Logging utilities for HookAnchor
//!
//! This module provides all logging functionality including debug logs,
//! error logs, and verbose logging with file management.

use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

/// Clear the debug log file
/// 
/// Removes the debug log file at the hardcoded path ~/.config/hookanchor/anchor.log
/// to start fresh logging. Used before rebuilds and when log exceeds max size.
pub fn clear_debug_log() {
    // Use constant from sys_data
    let debug_path = super::utilities::expand_tilde(crate::core::sys_data::DEFAULT_LOG_PATH);
    
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
    let debug_path = super::utilities::expand_tilde(crate::core::sys_data::DEFAULT_LOG_PATH);
    
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
    let debug_path = super::utilities::expand_tilde(crate::core::sys_data::DEFAULT_LOG_PATH);
    
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