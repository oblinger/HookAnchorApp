//! Data layer - centralized access to all persistent state
//!
//! This module provides the ONLY public interface for accessing commands, config,
//! patches, and state. All other modules must use this interface.
//!
//! The data layer ensures:
//! - Single source of truth (sys_data singleton)
//! - Proper history tracking for all modifications
//! - Cache/disk synchronization
//! - Data integrity through validation
//!
//! # PUBLIC INTERFACE (Exported via crate::core::data)
//!
//! ## Data Access Functions
//! - `SysData` - Bundle containing config, commands, and patches
//! - `initialize() -> Result<(), String>` - Load config, commands, patches into singleton
//! - `get_sys_data() -> (SysData, bool)` - Get full SysData bundle
//! - `get_config() -> Config` - Get configuration from singleton
//! - `get_commands() -> Vec<Command>` - Get commands from singleton
//! - `set_commands(Vec<Command>) -> Result<(), Box<dyn Error>>` - Replace all commands (auto-saves)
//! - `add_command(Command) -> Result<(), Box<dyn Error>>` - Add single command (auto-saves)
//! - `delete_command(&str) -> Result<(), Box<dyn Error>>` - Delete command by name (auto-saves)
//! - `get_patches() -> HashMap<String, Patch>` - Get patches from singleton
//! - `get_state() -> AppState` - Get application state (trampoline to state.rs)
//! - `set_state(&AppState) -> Result<(), Box<dyn Error>>` - Save application state (trampoline to state.rs)
//!
//! **State usage pattern**:
//! ```rust
//! // Within library code:
//! let mut state = crate::core::data::get_state();
//! state.last_executed_command = Some("foo".to_string());
//! crate::core::data::set_state(&state)?;
//!
//! // From binary code:
//! let mut state = hookanchor::core::get_state();
//! state.server_pid = Some(1234);
//! hookanchor::core::set_state(&state)?;
//! ```
//!
//! ## Type Definitions (re-exported via crate::core)
//! - `Config`, `PopupSettings`, `LauncherSettings` - Configuration types (from config.rs)
//! - `AppState`, `HistoryViewerState` - State types (from state.rs)

// Internal modules (private to data layer)
mod sys_data;
mod storage;

// These modules are pub(in crate::core) so the core module can re-export them,
// but they're not directly accessible outside of core
pub(in crate::core) mod config;
pub(in crate::core) mod state;
pub(in crate::core) mod history;

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

// Re-export history functions
pub use history::{
    initialize_history_db,
    record_command_created,
    record_command_modified,
    HistoryEntry,
};

// Internal access to CONFIG for same-crate modules only
pub(crate) use sys_data::CONFIG;
