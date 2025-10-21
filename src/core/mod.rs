//! Core business logic modules
//! 
//! This module serves as the public API for core functionality.
//! ALL public exports are explicitly listed here.
//!
//! **IMPORTANT**: All items in submodules should be declared `pub(crate)` and 
//! re-exported here as `pub` if they need external access. This mod.rs file 
//! is the complete specification of what this crate exports.

// Internal module declarations (accessible within crate)
pub(crate) mod config;
pub(crate) mod state;
pub(crate) mod application_state;
pub(crate) mod sys_data;
pub(crate) mod build_verification;  // Build consistency verification
pub(crate) mod template_creation;  // Used by UI and execute modules
pub(crate) mod key_processing;     // Used by UI module
pub(crate) mod commands;           // Many internal functions used across crates
pub(crate) mod command_ops;        // User-level command operations
pub(crate) mod inference;          // Patch inference and auto-assignment logic
pub(crate) mod display;            // Command display and filtering logic

// ============================================================================
// PUBLIC API - All external access goes through these re-exports
// ============================================================================

// Configuration types
pub use config::{
    Config, PopupSettings, LauncherSettings,
    ConfigResult, load_config_with_error, get_config_file_path
};

// State management
pub use state::{
    AppState, HistoryViewerState, load_state, save_state,
    save_state_with_build_time, save_last_executed_command,
    save_server_pid, clear_server_pid
};

// Application state
pub use application_state::{
    ApplicationState
};

// System data
pub use sys_data::{
    // Internal use (used by other core modules and systems)
    SysData, get_sys_data, get_config, initialize_config,
    DEFAULT_LOG_PATH, DEFAULT_MAX_LOG_SIZE,
    // Public API - ONLY these should be used by external code
    initialize, get_commands, get_patches, set_commands
    // add_command/delete_command exported from command_ops for backward compat
};

// Command types and operations
pub use commands::{
    // Core types
    Command, CommandTarget, Patch,

    // CRUD operations (INTERNAL USE ONLY - use sys_data API instead)
    load_commands_for_inference,
    save_commands_to_file, parse_command_line,

    // Query and filtering
    filter_commands,
    merge_similar_commands, merge_similar_commands_with_context,

    // Patch management
    get_patch_for_command, get_patch, get_patch_path,
    run_patch_inference,

    // NOTE: load_commands, load_commands_with_data, create_patches_hashmap are now
    // pub(crate) only. External code MUST use sys_data::get_commands() or get_sys_data()
};

// User-level command operations
pub use command_ops::{
    add_command, delete_command, // Backward compat wrappers around sys_data
    rename_associated_data
};

// Patch validation and repair system
pub use inference::{
    infer_patch, auto_assign_patches,
    validate_and_repair_patches, PatchResolutionResult
};

// Template creation (used by UI)
pub use template_creation::{
    Template, TemplateContext, 
    create_command_from_template, process_template, process_template_files
};

// Key processing system (used primarily by UI)
pub use key_processing::{
    // Core types
    Keystroke, Modifiers, KeyRegistry,
    
    // Handler interfaces
    KeyHandler, KeyHandlerContext, KeyHandlerResult, PopupInterface,

    // Built-in handlers
    TextHandler, NavigationHandler, NavigationDirection,
    PopupActionHandler, TemplateHandler,
    UninstallTextHandler, ShowKeysTextHandler,

    // Utilities
    create_default_key_registry
};

// Display and filtering system
pub use display::{
    // Core filtering functions
    command_matches_query, command_matches_query_with_debug,
    
    // Main display function
    get_new_display_commands,
};

// Command utilities 
pub use commands::{
    // Helper functions
    get_command_prefix,
};