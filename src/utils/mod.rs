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
//! - build_verification: Build verification and metadata
//! - dialog2: Dialog helper functions (new non-blocking external system)

// Internal module declarations (accessible within crate for internal use)
pub(crate) mod logging;
pub(crate) mod files;
pub(crate) mod utilities;
pub(crate) mod subprocess;
pub(crate) mod error;
pub(crate) mod build_verification;
// OLD DIALOG SYSTEM - TO BE REMOVED
// pub(crate) mod dialog_old;
pub(crate) mod dialog2;
pub(crate) mod params;

// ============================================================================
// PUBLIC API - All external access goes through these re-exports
// ============================================================================

// Logging functions
pub use logging::{
    log, print, detailed_log, log_error, log_error_module,
    clear_debug_log, check_and_clear_oversized_log, clear_log_file
};

// File utilities
pub use files::{
    is_markdown_file, is_anchor_file
};

// General utilities
pub use utilities::{
    init_binary_path, get_binary_path, get_binary_dir, expand_tilde,
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

// Build verification
pub use build_verification::{
    verify_and_log as verify_build,
    verify_config_version_or_exit,
    check_config_js_version
};

// OLD DIALOG SYSTEM - TO BE REMOVED
// Dialog helper functions (old blocking system)
// pub use dialog_old::{
//     fatal_error, warning, dialog
// };

// Dialog2 helper functions (new non-blocking system)
pub use dialog2::{
    fatal_error as fatal_error2,
    warning as warning2,
    error,                      // Non-blocking error dialog
    info,                       // Non-blocking info dialog
    dialog as dialog2,
    blocking_dialog
};

// Parameter parsing utilities
pub use params::{
    parse_kv_pairs, format_kv_pairs,
    PARAM_TEMPLATE, PARAM_PRIORITY, PARAM_FLAGS, PARAM_ARG
};