//! Core business logic modules
//! 
//! This module contains the core application logic separated from UI concerns.

pub mod commands;
pub mod config;
pub mod state;

// Re-export commonly used types
pub use commands::{Command, CommandTarget, filter_commands, merge_similar_commands};
pub use config::{Config, PopupSettings, LauncherSettings};
pub use state::{AppState, load_state, save_state};