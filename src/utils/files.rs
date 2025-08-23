//! File-related utility functions
//! 
//! Common file operations and checks used throughout the application

use std::path::Path;

/// Check if a path is a markdown file (has .md extension)
pub fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("md"))
        .unwrap_or(false)
}

/// Check if a path is an anchor file (folder_name/folder_name.md pattern)
/// This check is case-insensitive
pub fn is_anchor_file(path: &Path) -> bool {
    // First check if it's a markdown file
    if !is_markdown_file(path) {
        return false;
    }
    
    // Check if file stem matches parent folder name (case-insensitive)
    if let Some(file_stem) = path.file_stem() {
        if let Some(parent) = path.parent() {
            if let Some(parent_name) = parent.file_name() {
                // Convert both to strings and compare case-insensitively
                if let (Some(file_str), Some(parent_str)) = (file_stem.to_str(), parent_name.to_str()) {
                    return file_str.eq_ignore_ascii_case(parent_str);
                }
            }
        }
    }
    false
}