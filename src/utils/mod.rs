//! Utilities crate containing shared functionality
//!
//! This module serves as the public API for all utility functions.
//! ALL public exports are explicitly listed here.
//!
//! **IMPORTANT**: All items in submodules should be declared `pub(crate)` and 
//! re-exported here as `pub` if they need external access. This mod.rs file 
//! is the complete specification of what this crate exports.
//!
//! Internal modules:
//! - logging: All logging functionality
//! - files: File-related utilities
//! - utilities: General utility functions
//! - subprocess: Process spawning and monitoring
//! - error: Error queue system for user-facing errors

// Internal module declarations (accessible within crate for internal use)
pub(crate) mod logging;
pub(crate) mod files;
pub(crate) mod utilities;
pub(crate) mod subprocess;
pub(crate) mod error;

// ============================================================================
// PUBLIC API - All external access goes through these re-exports
// ============================================================================

// Logging functions
pub use logging::{
    log, detailed_log, log_error, log_error_module, 
    debug_log, verbose_log,
    clear_debug_log, check_and_clear_oversized_log, clear_log_file
};

// File utilities
pub use files::{
    is_markdown_file, is_anchor_file
};

// General utilities
pub use utilities::{
    init_binary_path, get_binary_path, expand_tilde, 
    launch_app_with_arg, open_url, open_folder, open_with_app,
    execute_shell_command, execute_shell_command_with_env, 
    execute_shell_command_unified, execute_shell_with_options,
    shell_simple, shell_login, shell_hybrid, 
    ShellOptions
};

// Subprocess management
pub use subprocess::{
    register_process, check_system_health, show_process_status,
    ProcessInfo
};

// Error queue system
pub use error::{
    init_error_queue, queue_user_error, take_next_error, 
    has_errors, clear_errors
};