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
pub(crate) mod history;

// ============================================================================
// PUBLIC API - All external access goes through these re-exports
// ============================================================================

// Scanner subsystem - file scanning and command discovery
pub use scanner::{
    scan_check,      // Main function for automatic background scanning
    scan_verbose,    // Manual CLI rescanning with verbose output
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

// History subsystem - command change tracking with SQLite
pub use history::{
    initialize_history_db,
    record_command_created,
    record_command_modified,
    record_command_deleted,
    query_history_by_date_range,
    query_history_by_path_prefix,
    HistoryEntry,
};