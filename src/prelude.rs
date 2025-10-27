//! Common imports for HookAnchor modules
//!
//! Add `use crate::prelude::*;` to any file to get access to commonly used functions.
//!
//! ## Logging Functions
//! - `log(message)` - Log message to anchor.log
//! - `log_error(message)` - Log error message
//! - `detailed_log(tag, message)` - Detailed logging with tag
//!
//! ## Dialog Functions (use directly from crate::utils)
//! - `crate::utils::error(message)` - Show error dialog (non-blocking)
//! - `crate::utils::warning2(message)` - Show warning dialog (non-blocking)
//! - `crate::utils::info(message)` - Show info dialog (non-blocking)
//! - `crate::utils::fatal_error2(message)` - Show fatal error and terminate
//! - `crate::utils::blocking_dialog(specs)` - Show blocking dialog (CLI only)

pub use crate::utils::{
    // OLD DIALOG SYSTEM - TO BE REMOVED
    // Dialog functions (commented out - no longer exported)
    // fatal_error,
    // warning,
    // dialog,

    // Logging functions
    log,
    log_error,
    detailed_log,
};
