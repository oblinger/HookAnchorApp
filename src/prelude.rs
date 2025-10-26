//! Common imports for HookAnchor modules
//!
//! Add `use crate::prelude::*;` to any file to get access to commonly used functions.
//!
//! ## Dialog Functions
//! - `fatal_error(message)` - Show fatal error dialog and terminate
//! - `warning(message)` - Show warning dialog and continue
//! - `dialog(specs)` - Show general-purpose dialog and return result
//!
//! ## Logging Functions
//! - `log(message)` - Log message to anchor.log
//! - `log_error(message)` - Log error message
//! - `detailed_log(tag, message)` - Detailed logging with tag

pub use crate::utils::{
    // Dialog functions
    fatal_error,
    warning,
    dialog,

    // Logging functions
    log,
    log_error,
    detailed_log,
};
