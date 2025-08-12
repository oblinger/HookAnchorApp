//! Core business logic modules
//! 
//! This module contains the core application logic separated from UI concerns.

pub mod actions;
pub mod commands;
pub mod config;
pub mod state;
pub mod application_state;
pub mod sys_data;
pub mod template_creation;
pub mod key_processing;
pub mod unified_actions;

// Re-export commonly used types
pub use actions::{get_action, is_markdown_anchor, get_default_patch_for_action};
pub use commands::{Command, CommandTarget, Patch, filter_commands, merge_similar_commands, load_commands_with_data, infer_patch};
pub use unified_actions::{Action, ActionContext, expand_string, execute_action};
pub use config::{Config, PopupSettings, LauncherSettings};
pub use state::{AppState, load_state, save_state};
pub use application_state::ApplicationState;
pub use sys_data::{SysData, load_data, get_sys_data, get_config, clear_sys_data};