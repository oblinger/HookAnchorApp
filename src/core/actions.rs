//! Action determination logic for files and paths
//!
//! This module provides functions to determine the appropriate action type
//! for a given file path based on file type and naming conventions.

use std::path::Path;
use std::os::unix::fs::PermissionsExt;

/// Determines the appropriate action for a given file path
/// 
/// Returns:
/// - "anchor" for markdown files where filename matches parent directory name
/// - "markdown" for other markdown files (replaces old "obs" action)
/// - "folder" for directories
/// - "cmd" for executable files
/// - "doc" for non-markdown files
pub fn get_action(path: &Path) -> String {
    // Check if it's a directory
    if path.is_dir() {
        return "folder".to_string();
    }
    
    // Check if it's a markdown file
    if let Some(extension) = path.extension() {
        if extension.to_str().unwrap_or("").to_lowercase() == "md" {
            // It's a markdown file - check if it's an anchor
            if is_markdown_anchor(path) {
                return "anchor".to_string();
            } else {
                return "markdown".to_string();
            }
        }
    }
    
    // Check if it's executable
    if is_executable(path) {
        return "cmd".to_string();
    }
    
    // For all other file types, use "doc" action
    "doc".to_string()
}

/// Checks if a markdown file is an "anchor" (base name matches parent folder name)
/// 
/// This function is extracted from the scanner module to be reusable.
/// An anchor file is one where the markdown file's base name (without .md extension)
/// matches the parent directory name (case-insensitive).
/// 
/// Examples:
/// - `/path/to/ProjectName/ProjectName.md` → true (anchor)
/// - `/path/to/ProjectName/readme.md` → false (regular obs file)
/// - `/path/to/projectname/PROJECTNAME.md` → true (case-insensitive match)
pub fn is_markdown_anchor(path: &Path) -> bool {
    // Get the file stem (base name without extension)
    let file_stem = match path.file_stem() {
        Some(stem) => stem.to_str().unwrap_or(""),
        None => return false,
    };
    
    // Get the parent directory name
    let parent_name = match path.parent() {
        Some(parent) => match parent.file_name() {
            Some(name) => name.to_str().unwrap_or(""),
            None => return false,
        },
        None => return false,
    };
    
    // Compare ignoring case
    file_stem.to_lowercase() == parent_name.to_lowercase()
}

/// Checks if a file has executable permissions
/// 
/// Returns true if the file has the executable bit set for owner, group, or other.
pub fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = path.metadata() {
        let permissions = metadata.permissions();
        // Check if any executable bit is set (owner, group, or other)
        permissions.mode() & 0o111 != 0
    } else {
        false
    }
}

/// Determines the appropriate default patch for a given action
/// 
/// Returns:
/// - "Dir" for folder actions
/// - "Cmd" for cmd actions  
/// - "Doc" for doc actions
/// - None for other actions (will use inference)
pub fn get_default_patch_for_action(action: &str) -> Option<String> {
    match action {
        "folder" => Some("Dir".to_string()),
        "cmd" => Some("Cmd".to_string()),
        "doc" => Some("Doc".to_string()),
        _ => None, // Let normal patch inference handle other types
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_action_anchor() {
        let path = PathBuf::from("/home/user/ProjectName/ProjectName.md");
        assert_eq!(get_action(&path), "anchor");
    }

    #[test]
    fn test_get_action_markdown() {
        let path = PathBuf::from("/home/user/ProjectName/readme.md");
        assert_eq!(get_action(&path), "markdown");
    }

    #[test]
    fn test_get_action_doc() {
        let path = PathBuf::from("/home/user/ProjectName/document.txt");
        assert_eq!(get_action(&path), "doc");
    }

    #[test]
    fn test_get_action_case_insensitive() {
        let path = PathBuf::from("/home/user/projectname/PROJECTNAME.md");
        assert_eq!(get_action(&path), "anchor");
    }

    #[test]
    fn test_is_markdown_anchor_true() {
        let path = PathBuf::from("/home/user/ProjectName/ProjectName.md");
        assert!(is_markdown_anchor(&path));
    }

    #[test]
    fn test_is_markdown_anchor_false() {
        let path = PathBuf::from("/home/user/ProjectName/readme.md");
        assert!(!is_markdown_anchor(&path));
    }

    #[test]
    fn test_is_markdown_anchor_case_insensitive() {
        let path = PathBuf::from("/home/user/projectname/PROJECTNAME.md");
        assert!(is_markdown_anchor(&path));
    }
    
    #[test]
    fn test_get_action_folder() {
        // Note: This test may fail in CI since we can't create actual directories
        // but it documents the intended behavior
        let path = PathBuf::from("/tmp");
        if path.is_dir() {
            assert_eq!(get_action(&path), "folder");
        }
    }
    
    #[test]
    fn test_get_default_patch_for_action() {
        assert_eq!(get_default_patch_for_action("folder"), Some("Dir".to_string()));
        assert_eq!(get_default_patch_for_action("cmd"), Some("Cmd".to_string()));
        assert_eq!(get_default_patch_for_action("doc"), Some("Doc".to_string()));
        assert_eq!(get_default_patch_for_action("url"), None);
        assert_eq!(get_default_patch_for_action("markdown"), None);
    }
}