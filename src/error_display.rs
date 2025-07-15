//! Error Display System
//!
//! This module provides a global error queue system that allows any part of the application
//! to queue errors for display to the user via the GUI dialog system.
//!
//! This is especially useful for background operations, shell commands, and other
//! non-UI components that need to communicate errors to the user.

use std::sync::{Mutex, OnceLock};
use std::collections::VecDeque;

/// Global error queue for displaying errors to the user
static ERROR_QUEUE: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();

/// Initialize the global error queue
pub fn init_error_queue() {
    let _ = ERROR_QUEUE.set(Mutex::new(VecDeque::new()));
}

/// Queue an error for display to the user
/// This can be called from any part of the application, including non-UI components
pub fn queue_user_error(error_message: &str) {
    if let Some(queue) = ERROR_QUEUE.get() {
        if let Ok(mut queue_guard) = queue.lock() {
            queue_guard.push_back(error_message.to_string());
            crate::utils::debug_log("ERROR_QUEUE", &format!("Queued error: {}", error_message));
        }
    }
}

/// Take the next error from the queue for display
/// Returns None if no errors are queued
pub fn take_next_error() -> Option<String> {
    if let Some(queue) = ERROR_QUEUE.get() {
        if let Ok(mut queue_guard) = queue.lock() {
            return queue_guard.pop_front();
        }
    }
    None
}

/// Check if there are any errors in the queue
pub fn has_errors() -> bool {
    if let Some(queue) = ERROR_QUEUE.get() {
        if let Ok(queue_guard) = queue.lock() {
            return !queue_guard.is_empty();
        }
    }
    false
}

/// Clear all errors from the queue
pub fn clear_errors() {
    if let Some(queue) = ERROR_QUEUE.get() {
        if let Ok(mut queue_guard) = queue.lock() {
            queue_guard.clear();
        }
    }
}