//! Major subsystems and capabilities
//!
//! This module serves as the public API for all major subsystems.
//! ALL public exports are explicitly listed here.
//!
//! **IMPORTANT**: All items in submodules should be declared `pub(crate)` and 
//! re-exported here as `pub` if they need external access. This mod.rs file 
//! is the complete specification of what this crate exports.
//!
//! Each subsystem provides specific functionality:
//! - scanner: File system scanning for commands
//! - grabber: Context capture from active applications
//! - obsidian: Obsidian vault and markdown file management
//! - setup_assistant: First-run setup and configuration
//! - popup_server: Server for managing popup instances via IPC

// Internal module declarations (accessible within crate)
pub(crate) mod scanner;
pub(crate) mod grabber;
pub(crate) mod obsidian;
pub(crate) mod setup_assistant;
pub(crate) mod popup_server;
pub(crate) mod restart;
pub(crate) mod dialog_manager;

// ============================================================================
// PUBLIC API - All external access goes through these re-exports
// ============================================================================

// Scanner subsystem - file scanning and command discovery
pub use scanner::{
    scan_check,             // Main function for automatic background scanning
    scan_new_files,         // Scan filesystem for NEW files not yet in commands
    scan_modified_files,    // Scan for MODIFIED files and update history
    load_manual_edits,      // Load user's manual edits from commands.txt
    SCANNER_GENERATED_ACTIONS  // Constants for generated action types
};

// Grabber subsystem - context capture from applications
pub use grabber::{
    grab,           // Main function for capturing and matching context
    grab_debug,     // Debug function for CLI testing
    GrabResult,     // Result type for grab operations
    AppContext,     // Context information type
    GrabberRule,    // Rule configuration type
    generate_rule_template_text  // Generate template for new grabber rules
};

// Obsidian subsystem - Obsidian vault management
pub use obsidian::{
    analyze_path, get_vault_path, get_vault_name,
    get_markdown_action_strategy,
    VaultPathInfo, MarkdownActionStrategy
};

// Setup assistant - first run configuration
pub use setup_assistant::{
    SetupAssistant
};

// Popup server - IPC server for popup management
pub use popup_server::{
    is_primary_popup_instance,
    PopupCommand, PopupControl
};

// Restart subsystem - system restart and rebuild operations
pub use restart::{
    full_system_restart,
    start_popup_server    // Start popup_server in background
};

// Dialog manager - non-blocking dialog subprocess management
pub use dialog_manager::{
    spawn_dialog,       // Spawn dialog subprocess
    poll_dialog,        // Poll dialog for completion (non-blocking)
    DialogHandle        // Handle to active dialog subprocess
};

// History subsystem - moved to core::data module
// - data::initialize_history_db() - Initialize SQLite database
// - data::record_command_created() - Record command creation
// - data::record_command_modified() - Record command modification
// - data::HistoryEntry - History entry type

// CommandStore functionality moved to core::sys_data module
// - sys_data::get_commands() - Load commands from singleton
// - sys_data::set_commands() - Save commands with inference
// - sys_data::add_command() - Add single command with history
// - sys_data::delete_command() - Delete command by name