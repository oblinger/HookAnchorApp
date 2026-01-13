//! CLI Module
//!
//! This module contains the command-line interface functionality,
//! organized into focused submodules.

pub mod help;

// Re-export commonly used items
pub use help::{print_help, print_help_vars, print_help_config, print_help_fns};
