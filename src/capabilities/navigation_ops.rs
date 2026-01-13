//! Navigation Operations
//!
//! Functions for navigating the command hierarchy and managing selection state.
//! These are pure functions that take the necessary state as parameters,
//! making them testable without needing full popup state.

use std::collections::HashMap;
use crate::core::Command;
use crate::core::commands::Patch;

// =============================================================================
// HIERARCHY NAVIGATION
// =============================================================================

/// Get the parent command in the hierarchy for a given command.
///
/// This navigates "up" in the patch hierarchy. For example, if we're in
/// "Project Tasks" submenu, this returns the parent "Project" anchor.
///
/// # Arguments
/// * `command` - The current command
/// * `patches` - HashMap of all patches
///
/// # Returns
/// The parent command if one exists, None if at root level
pub fn get_parent_command<'a>(command: &Command, patches: &'a HashMap<String, Patch>) -> Option<&'a Command> {
    command.get_parent_command(patches)
}

/// Check if a command can navigate "down" (into a submenu).
///
/// A command can navigate down if it's an anchor (has the 'A' flag).
///
/// # Arguments
/// * `command` - The command to check
///
/// # Returns
/// true if the command is an anchor and can be entered
pub fn can_navigate_into(command: &Command) -> bool {
    command.is_anchor()
}

/// Get the patch path from root to a command.
///
/// Returns the hierarchy path as a vector of patch names.
/// For example: ["Root", "Project", "Project Tasks"]
///
/// # Arguments
/// * `command` - The command to get path for
/// * `patches` - HashMap of all patches
///
/// # Returns
/// Vector of patch names from root to the command's patch
pub fn get_hierarchy_path(command: &Command, patches: &HashMap<String, Patch>) -> Vec<String> {
    if command.patch.is_empty() {
        return vec![];
    }

    if let Some(patch) = patches.get(&command.patch) {
        patch.get_path_from_root(patches)
    } else {
        vec![command.patch.clone()]
    }
}

/// Get children commands for an anchor.
///
/// Returns commands that are "inside" the given anchor - commands whose
/// prefix or patch matches the anchor.
///
/// # Arguments
/// * `anchor` - The anchor command
/// * `all_commands` - All available commands
///
/// # Returns
/// Vector of commands that are children of this anchor
pub fn get_anchor_children(anchor: &Command, all_commands: &[Command]) -> Vec<Command> {
    let anchor_name_lower = anchor.command.to_lowercase();

    all_commands.iter()
        .filter(|cmd| {
            // Check if command's patch matches anchor name
            cmd.patch.to_lowercase() == anchor_name_lower ||
            // Or if command name starts with anchor name followed by separator
            cmd.command.to_lowercase().starts_with(&format!("{} ", anchor_name_lower)) ||
            cmd.command.to_lowercase().starts_with(&format!("{}.", anchor_name_lower)) ||
            cmd.command.to_lowercase().starts_with(&format!("{}_", anchor_name_lower))
        })
        .cloned()
        .collect()
}

// =============================================================================
// SELECTION MANAGEMENT
// =============================================================================

/// Calculate new selection index after vertical movement.
///
/// Handles wrapping at list boundaries.
///
/// # Arguments
/// * `current_index` - Current selection index
/// * `delta` - Movement direction (-1 for up, 1 for down)
/// * `list_length` - Total number of items
///
/// # Returns
/// New selection index with wrapping
pub fn calculate_vertical_selection(current_index: usize, delta: i32, list_length: usize) -> usize {
    if list_length == 0 {
        return 0;
    }

    let new_index = current_index as i32 + delta;

    if new_index < 0 {
        list_length - 1  // Wrap to bottom
    } else if new_index >= list_length as i32 {
        0  // Wrap to top
    } else {
        new_index as usize
    }
}

/// Calculate new selection index after horizontal movement in a grid layout.
///
/// # Arguments
/// * `current_index` - Current selection index
/// * `delta` - Movement direction (-1 for left, 1 for right)
/// * `columns` - Number of columns in the grid
/// * `list_length` - Total number of items
///
/// # Returns
/// New selection index
pub fn calculate_horizontal_selection(
    current_index: usize,
    delta: i32,
    columns: usize,
    list_length: usize
) -> usize {
    if list_length == 0 || columns == 0 {
        return 0;
    }

    let current_row = current_index / columns;
    let current_col = current_index % columns;

    let new_col = if delta < 0 {
        if current_col == 0 { columns - 1 } else { current_col - 1 }
    } else {
        if current_col >= columns - 1 { 0 } else { current_col + 1 }
    };

    let new_index = current_row * columns + new_col;

    // Clamp to list bounds
    if new_index >= list_length {
        current_index
    } else {
        new_index
    }
}

/// Find the index of a command in a list by name.
///
/// # Arguments
/// * `command_name` - Name to search for (case-insensitive)
/// * `commands` - List of commands to search
///
/// # Returns
/// Index if found, None otherwise
pub fn find_command_index(command_name: &str, commands: &[Command]) -> Option<usize> {
    let name_lower = command_name.to_lowercase();
    commands.iter().position(|cmd| cmd.command.to_lowercase() == name_lower)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_vertical_selection_down() {
        assert_eq!(calculate_vertical_selection(0, 1, 5), 1);
        assert_eq!(calculate_vertical_selection(3, 1, 5), 4);
    }

    #[test]
    fn test_calculate_vertical_selection_up() {
        assert_eq!(calculate_vertical_selection(2, -1, 5), 1);
        assert_eq!(calculate_vertical_selection(0, -1, 5), 4); // Wrap to bottom
    }

    #[test]
    fn test_calculate_vertical_selection_wrap() {
        assert_eq!(calculate_vertical_selection(4, 1, 5), 0); // Wrap to top
        assert_eq!(calculate_vertical_selection(0, -1, 5), 4); // Wrap to bottom
    }

    #[test]
    fn test_calculate_vertical_selection_empty() {
        assert_eq!(calculate_vertical_selection(0, 1, 0), 0);
    }

    #[test]
    fn test_calculate_horizontal_selection() {
        // 3 columns, moving right
        assert_eq!(calculate_horizontal_selection(0, 1, 3, 9), 1);
        assert_eq!(calculate_horizontal_selection(2, 1, 3, 9), 0); // Wrap in row

        // Moving left
        assert_eq!(calculate_horizontal_selection(1, -1, 3, 9), 0);
        assert_eq!(calculate_horizontal_selection(0, -1, 3, 9), 2); // Wrap in row
    }

    #[test]
    fn test_can_navigate_into() {
        let mut cmd = Command::new("test".to_string(), "Test".to_string(), "url".to_string(), "".to_string(), "".to_string());
        assert!(!can_navigate_into(&cmd));

        cmd.set_anchor(true);
        assert!(can_navigate_into(&cmd));
    }
}
