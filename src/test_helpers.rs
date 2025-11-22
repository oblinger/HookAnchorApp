//! Test helper functions and types
//!
//! This module provides testing-friendly APIs that wrap internal functionality
//! without exposing private implementation details.
//!
//! This module is only compiled when running tests.

#![cfg(test)]

use crate::core::{Command, Patch, Config};
use std::collections::HashMap;

/// Test-friendly wrapper for loading commands from a string
///
/// This is meant for testing only and bypasses the normal sys_data system.
pub fn parse_test_commands(command_text: &str) -> Vec<Command> {
    // Parse commands from text
    let mut commands = Vec::new();
    for line in command_text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Ok(cmd) = crate::core::parse_command_line(line) {
            commands.push(cmd);
        }
    }
    commands
}

/// Test-friendly wrapper for creating patches from commands
pub fn create_test_patches(commands: &[Command]) -> HashMap<String, Patch> {
    crate::core::create_patches_hashmap(commands)
}

/// Test-friendly wrapper for running patch inference
pub fn run_test_patch_inference(commands: &mut Vec<Command>, patches: &HashMap<String, Patch>) {
    crate::core::run_patch_inference(commands, patches, false, false, false);
}

/// Test-friendly wrapper for filtering commands
pub fn filter_test_commands(
    query: &str,
    commands: &[Command],
    _patches: &HashMap<String, Patch>
) -> Vec<Command> {
    crate::core::filter_commands(commands, query, 100, false)
}

/// Test-friendly wrapper for get_new_display_commands
pub fn get_test_display_commands(
    input: &str,
    commands: &[Command],
    patches: &HashMap<String, Patch>,
    config: &Config
) -> (Vec<Command>, bool, Option<(Command, Command)>, usize, Option<usize>, String) {
    crate::core::get_new_display_commands(input, commands, patches, config)
}
