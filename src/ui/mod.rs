//! UI modules for the anchor selector
//! 
//! This module contains UI-specific logic separated from business logic.

pub mod layout;
pub mod popup_state;
pub mod popup;
pub mod command_editor;
pub mod dialog;

// Re-export commonly used types
pub use layout::{DisplayLayout, LayoutArrangement, Selection, Direction};
pub use popup_state::PopupState;
pub use popup::run_gui_with_prompt;
pub use command_editor::{CommandEditor, CommandEditorResult};
pub use dialog::Dialog;

// Re-export ApplicationState from library root
pub use crate::ApplicationState;