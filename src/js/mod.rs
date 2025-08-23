//! JavaScript runtime and scripts module
//! 
//! This module serves as the public API for JavaScript runtime functionality.
//! ALL public exports are explicitly listed here.
//!
//! **IMPORTANT**: All items in submodules should be declared `pub(crate)` and 
//! re-exported here as `pub` if they need external access. This mod.rs file 
//! is the complete specification of what this crate exports.

// Internal module declarations
mod runtime;

// ============================================================================
// PUBLIC API - All external access goes through these re-exports
// ============================================================================

// Runtime creation and management
pub use runtime::{
    execute,        // Main function for executing JavaScript code
    setup_runtime   // Setup function for creating custom runtime contexts (includes config.js)
};