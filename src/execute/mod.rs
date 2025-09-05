//! Execute module - Contains all execution-related functionality
//! 
//! This module encapsulates command and action execution logic.
//! ALL public exports are explicitly listed here.
//!
//! **IMPORTANT**: All items in submodules should be declared `pub(crate)` and 
//! re-exported here as `pub` if they need external access. This mod.rs file 
//! is the complete specification of what this crate exports.

// Internal modules - only accessible within execute folder
mod actions;
mod execute;
mod execution_server;
mod execution_server_management;

// ============================================================================
// PUBLIC API - All external access goes through these re-exports
// ============================================================================

// From actions module  
pub use self::actions::{
    Action,
    ActionContext,
    // Helper functions that need to be public
    get_action,
    get_default_patch_for_action,
    expand_string,
    make_action,
    get_action_for_arg,
};

// From execute module - main execution interface
pub use self::execute::{
    command_to_action,
    execute_on_server,
    execute_on_server_with_parameters,
    activate_command_server,
    run_command_server
};

// From execution_server module - UI interface
// Note: execute_locally is currently not used outside this module
// pub use self::execution_server::execute_locally;