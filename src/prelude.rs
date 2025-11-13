//! Common imports for HookAnchor modules
//!
//! Add `use crate::prelude::*;` to any file to get access to commonly used functions.
//!
//! ## Logging Functions
//! - `log(message)` - Log message to anchor.log only
//! - `log_error(message)` - Log error message
//! - `detailed_log(tag, message)` - Detailed logging with tag
//! - `print_and_log(message)` - Print to console AND log to file (for CLI commands)
//!
//! ## Dialog Functions (use directly from crate::utils)
//! - `crate::utils::error(message)` - Show error dialog (non-blocking)
//! - `crate::utils::warning2(message)` - Show warning dialog (non-blocking)
//! - `crate::utils::info(message)` - Show info dialog (non-blocking)
//! - `crate::utils::fatal_error2(message)` - Show fatal error and terminate
//! - `crate::utils::blocking_dialog(specs)` - Show blocking dialog (CLI only)

// Import logging functions directly from the logging module (not re-exported from utils)
pub use crate::utils::logging::{
    log,
    log_error,
    detailed_log,
    print_and_log,
};
