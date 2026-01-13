//! Capabilities Layer
//!
//! This module provides purpose-built operation modules that consolidate
//! business logic from the UI and CLI layers. Each capability module
//! is the single source of truth for its domain.
//!
//! Design principles:
//! - Thin UI layers: popup.rs and cmd.rs call into these modules
//! - Single implementation: both popup and CLI use the same code paths
//! - Testable: capability modules can be unit tested without UI
//!
//! See docs/CODE_ORGANIZATION.md for the full architecture plan.

pub mod anchor_ops;
pub mod template_ops;
pub mod navigation_ops;
pub mod display_ops;
pub mod command_ops;
pub mod edit_ops;
