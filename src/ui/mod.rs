//! UI modules for the anchor selector
//! 
//! This module contains UI-specific logic separated from business logic.

pub mod layout;
pub mod popup_state;

// Re-export commonly used types
pub use layout::{DisplayLayout, LayoutArrangement, Selection, Direction};
pub use popup_state::PopupState;