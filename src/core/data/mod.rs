//! Data layer - centralized access to all persistent state
//!
//! This module provides the ONLY public interface for accessing commands, config,
//! patches, history, and cache. All other modules must use this interface.
//!
//! The data layer ensures:
//! - Single source of truth (sys_data singleton)
//! - Proper history tracking for all modifications
//! - Cache/disk synchronization
//! - Data integrity through validation

// Internal modules (private to data layer)
mod sys_data;
mod storage;

// These modules are pub(in crate::core) so the core module can re-export them,
// but they're not directly accessible outside of core
pub(in crate::core) mod config;
pub(in crate::core) mod state;

// Re-export the public interface from sys_data
// These are the ONLY functions that external code should use
pub use sys_data::{
    // Types
    SysData,
    // Initialization
    initialize,
    // Read operations
    get_commands,
    get_config,
    get_patches,
    get_sys_data,
    get_state,
    // Write operations
    set_commands,
    add_command,
    delete_command,
    set_state,
};

// Internal access to CONFIG for same-crate modules only
pub(crate) use sys_data::CONFIG;
