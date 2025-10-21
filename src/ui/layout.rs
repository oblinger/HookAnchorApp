//! Display layout and navigation logic
//! 
//! This module handles how commands are arranged for display and how navigation
//! works within that arrangement.

use crate::core::Command;
use crate::core::Config;

/// Represents how commands are arranged for display
#[derive(Debug, Clone)]
pub struct DisplayLayout {
    /// Commands in display order (includes separators)
    pub commands: Vec<Command>,
    /// Visual arrangement information
    pub arrangement: LayoutArrangement,
    /// Prefix menu information if in prefix menu mode
    pub prefix_menu_info: Option<PrefixMenuInfo>,
}

/// Visual arrangement for commands (always multi-column, with cols=1 for single-column display)
#[derive(Debug, Clone)]
pub enum LayoutArrangement {
    /// Multi-column layout with specified dimensions (cols=1 for single-column display)
    MultiColumn {
        rows: usize,
        cols: usize,
    },
}

/// Information about prefix menu display
#[derive(Debug, Clone)]
pub struct PrefixMenuInfo {
    pub prefix: String,
    pub inside_count: usize,
    pub separator_index: Option<usize>,
}

/// Tracks selection position in both visual and logical space
#[derive(Debug, Clone)]
pub struct Selection {
    /// Visual position (row, col)
    pub visual_position: (usize, usize),
    /// Index into display layout commands
    pub command_index: usize,
}

/// Navigation directions
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl DisplayLayout {
    /// Create a new display layout from filtered commands
    pub fn new(commands: Vec<Command>, config: &Config) -> Self {
        let arrangement = Self::calculate_arrangement(&commands, config);
        let prefix_menu_info = Self::detect_prefix_menu(&commands);

        DisplayLayout {
            commands,
            arrangement,
            prefix_menu_info,
        }
    }
    
    /// Calculate the best visual arrangement for the given commands
    fn calculate_arrangement(commands: &[Command], config: &Config) -> LayoutArrangement {
        let max_rows = config.popup_settings.max_rows;
        let max_cols = config.popup_settings.max_columns;

        crate::utils::log(&format!("LAYOUT: calculate_arrangement: commands.len()={}, max_rows={}, max_cols={}",
            commands.len(), max_rows, max_cols));

        // ALWAYS use MultiColumn, even for small lists (cols=1)
        let cols_needed = (commands.len() + max_rows - 1) / max_rows;
        let cols_to_use = cols_needed.min(max_cols).max(1); // At least 1 column
        let rows_per_col = (commands.len() + cols_to_use - 1) / cols_to_use;

        crate::utils::log(&format!("LAYOUT: → Using MultiColumn (rows={}, cols={})", rows_per_col, cols_to_use));

        LayoutArrangement::MultiColumn {
            rows: rows_per_col,
            cols: cols_to_use,
        }
    }
    
    /// Detect if we're in prefix menu mode and extract info
    fn detect_prefix_menu(commands: &[Command]) -> Option<PrefixMenuInfo> {
        // Look for separator command
        if let Some(separator_index) = commands.iter().position(|cmd| cmd.action == "separator") {
            Some(PrefixMenuInfo {
                prefix: "prefix menu".to_string(), // Could be extracted from commands
                inside_count: separator_index,
                separator_index: Some(separator_index),
            })
        } else {
            None
        }
    }
    
    /// Get command at visual position (row, col)
    pub fn get_command_at_position(&self, row: usize, col: usize) -> Option<&Command> {
        match &self.arrangement {
            LayoutArrangement::MultiColumn { rows, .. } => {
                let index = col * rows + row;
                self.commands.get(index)
            }
        }
    }
    
    /// Convert visual position to command index
    pub fn visual_to_index(&self, row: usize, col: usize) -> Option<usize> {
        match &self.arrangement {
            LayoutArrangement::MultiColumn { rows, .. } => {
                let index = col * rows + row;
                if index < self.commands.len() {
                    Some(index)
                } else {
                    None
                }
            }
        }
    }
    
    /// Convert command index to visual position
    pub fn index_to_visual(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.commands.len() {
            return None;
        }

        match &self.arrangement {
            LayoutArrangement::MultiColumn { rows, .. } => {
                let col = index / rows;
                let row = index % rows;
                Some((row, col))
            }
        }
    }
    
    /// Get layout dimensions (rows, cols)
    pub fn get_dimensions(&self) -> (usize, usize) {
        match &self.arrangement {
            LayoutArrangement::MultiColumn { rows, cols } => (*rows, *cols),
        }
    }
}

impl Selection {
    /// Create new selection at the first position
    pub fn new() -> Self {
        Selection {
            visual_position: (0, 0),
            command_index: 0,
        }
    }
    
    /// Create selection from command index
    pub fn from_index(index: usize, layout: &DisplayLayout) -> Self {
        let visual_position = layout.index_to_visual(index).unwrap_or((0, 0));
        Selection {
            visual_position,
            command_index: index,
        }
    }
    
    /// Move selection in the given direction
    pub fn navigate(&mut self, direction: Direction, layout: &DisplayLayout) -> bool {
        let (max_rows, max_cols) = layout.get_dimensions();
        let (current_row, current_col) = self.visual_position;
        
        let (new_row, new_col) = match direction {
            Direction::Up => {
                if current_row > 0 {
                    (current_row - 1, current_col)
                } else {
                    return false; // Can't go up from top
                }
            }
            Direction::Down => {
                if current_row + 1 < max_rows {
                    (current_row + 1, current_col)
                } else {
                    return false; // Can't go down from bottom
                }
            }
            Direction::Left => {
                if current_col > 0 {
                    (current_row, current_col - 1)
                } else {
                    return false; // Can't go left from leftmost
                }
            }
            Direction::Right => {
                if current_col + 1 < max_cols {
                    (current_row, current_col + 1)
                } else {
                    return false; // Can't go right from rightmost
                }
            }
        };
        
        // Check if the new position has a valid command
        if let Some(new_index) = layout.visual_to_index(new_row, new_col) {
            // Skip separator commands
            if layout.commands.get(new_index).map_or(false, |cmd| cmd.action == "separator") {
                // Try to continue in the same direction
                let temp_selection = Selection {
                    visual_position: (new_row, new_col),
                    command_index: new_index,
                };
                let mut temp_copy = temp_selection.clone();
                if temp_copy.navigate(direction, layout) {
                    *self = temp_copy;
                    return true;
                } else {
                    return false; // Hit separator and can't continue
                }
            }
            
            self.visual_position = (new_row, new_col);
            self.command_index = new_index;
            true
        } else {
            false // No valid command at new position
        }
    }
    
    /// Get the currently selected command
    pub fn get_command<'a>(&self, layout: &'a DisplayLayout) -> Option<&'a Command> {
        layout.commands.get(self.command_index)
    }
    
    /// Reset selection to first valid command
    pub fn reset(&mut self, layout: &DisplayLayout) {
        self.visual_position = (0, 0);
        self.command_index = 0;
        
        // Skip to first non-separator command
        while let Some(cmd) = self.get_command(layout) {
            if cmd.action != "separator" {
                break;
            }
            if !self.navigate(Direction::Down, layout) {
                break;
            }
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}