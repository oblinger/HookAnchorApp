//! Anchor Selector Library
//! 
//! A command management and filtering library that provides fuzzy matching
//! and prioritized search for command execution.
//!
//! For complete configuration documentation including JavaScript API, built-in functions,
//! and usage examples, see `docs.md` in the project root.

// Core modules
pub mod core;
pub mod execute;

// Capabilities layer - consolidated business logic
pub mod capabilities;

// UI modules  
pub mod ui;

// JavaScript runtime and scripts
pub mod js;

// Command-line interface
pub mod cmd;
pub mod utils;

// Major subsystems
pub mod systems;

// Prelude for common imports
pub mod prelude;

// Test helpers (only available during tests - see test_helpers.rs for #[cfg(test)])
pub mod test_helpers;

// All exports are now controlled through individual mod.rs files
// Access via: core::, execute::, ui::, js::, systems::, utils:: namespaces
