//! UI modules for the anchor selector
//! 
//! This module serves as the public API for UI functionality.
//! ALL public exports are explicitly listed here.
//!
//! **IMPORTANT**: All items in submodules should be declared `pub(crate)` and 
//! re-exported here as `pub` if they need external access. This mod.rs file 
//! is the complete specification of what this crate exports.

// Internal module declarations (accessible within crate)
pub(crate) mod layout;
pub(crate) mod popup_state;
pub(crate) mod popup;
pub(crate) mod command_editor;
pub(crate) mod dialog;
mod helpers;  // Only used within UI module

// ============================================================================
// PUBLIC API - All external access goes through these re-exports
// ============================================================================

// Layout and display types
pub use layout::{
    DisplayLayout, LayoutArrangement, Selection, Direction, PrefixMenuInfo
};

// Popup state management
pub use popup_state::{
    PopupState
};

// Main GUI entry point
pub use popup::{
    run_gui_with_prompt, AnchorSelector
};

// Command editor
pub use command_editor::{
    CommandEditor, CommandEditorResult
};

// Dialog system
pub use dialog::{
    Dialog, DialogElement, DialogRow
};

// Re-export ApplicationState from core
pub use crate::core::{
    ApplicationState
};