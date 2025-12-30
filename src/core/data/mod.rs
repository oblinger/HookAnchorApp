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
    initialize_config,  // Lightweight config-only initialization
    initialize_minimal, // Minimal initialization with empty data (GUI startup)
    // Read operations
    get_commands,
    get_commands_arc,  // Fast Arc-based access for hot paths
    get_config,
    get_patches,
    get_patch_for_folder,  // Query folder->patch mapping
    get_sys_data,
    get_state,
    get_history_entries,
    // Write operations
    set_commands,
    add_command,
    delete_command,
    set_state,
    set_active_anchor,
    // Data layer maintenance
    clear_commands,              // Clear singleton + delete files (for delete-history)
    reload_commands,             // Reload from disk into singleton (after manual file restore)
    backup_commands,             // Create timestamped backup, returns path for display
    restore_commands_from_file,  // Restore from external file path (user provides)
    // History management
    delete_history,
};

// Re-export storage functions
// load_commands_raw is pub(crate) for use by scanner and actions
pub(crate) use storage::load_commands_raw;
// These are internal to core/
pub(in crate::core) use storage::{
    save_commands_to_file,
    get_commands_file_path,
};

// Re-export config directory path function (for non-data files only)
pub use storage::get_config_dir;

// Re-export deduplication utilities for consistent duplicate detection
pub use storage::{
    command_dedup_key,
    build_command_map,
};

// Re-export history read function (via trampoline in sys_data)
// History writing is automatic and happens only inside sys_data
pub use history::HistoryEntry;

// Internal access to CONFIG for same-crate modules only
pub(crate) use sys_data::CONFIG;
