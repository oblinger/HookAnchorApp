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
    AppState, load_state, save_state,
    save_state_with_build_time, save_last_executed_command, 
    save_server_pid, clear_server_pid
};

// Application state
pub use application_state::{
    ApplicationState
};

// System data
pub use sys_data::{
    SysData, load_data, get_sys_data, get_config, 
    initialize_config, mark_commands_modified,
    DEFAULT_LOG_PATH, DEFAULT_MAX_LOG_SIZE
};

// Command types and operations
pub use commands::{
    // Core types
    Command, CommandTarget, Patch,
    
    // CRUD operations
    load_commands, load_commands_with_data, load_commands_for_inference,
    save_commands_to_file, parse_command_line,
    
    // Query and filtering
    filter_commands, 
    merge_similar_commands, merge_similar_commands_with_context,
    
    // Patch management  
    get_patch_for_command, get_patch, create_patches_hashmap, get_patch_path,
    run_patch_inference,
    
    // Submenu and navigation (moved to display module)
    
    // Migration and maintenance
    migrate_commands_to_new_format
};

// User-level command operations
pub use command_ops::{
    add_command, delete_command, rename_associated_data
};

// Patch inference system
pub use inference::{
    infer_patch, auto_assign_patches
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
    ActionHandler, Action, TemplateHandler, 
    UninstallTextHandler, ShowKeysTextHandler,
    
    // Utilities
    create_default_key_registry, ascii_to_key_name
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