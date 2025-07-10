//! Core business logic modules
//! 
//! This module contains the core application logic separated from UI concerns.

pub mod actions;
pub mod commands;
pub mod config;
pub mod state;
pub mod application_state;

// Re-export commonly used types
pub use actions::{get_action, is_markdown_anchor};
pub use commands::{Command, CommandTarget, filter_commands, merge_similar_commands};
pub use config::{Config, PopupSettings, LauncherSettings};
pub use state::{AppState, load_state, save_state};
pub use application_state::ApplicationState;