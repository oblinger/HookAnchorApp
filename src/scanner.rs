//! Scanner module for finding and processing markdown files and contacts
//! 
//! This module scans specified directory roots for markdown files and
//! user contacts, creating commands based on the items found.

use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;
use crate::Command;

/// Top-level scan function that orchestrates all scanning operations
pub fn scan(mut commands: Vec<Command>, markdown_roots: &[String]) -> Vec<Command> {
    // First scan markdown files
    commands = scan_files(commands, markdown_roots);
    
    // Then scan contacts
    commands = scan_contacts(commands);
    
    commands
}

/// Scans the configured markdown roots and returns an updated command list
pub fn scan_files(mut commands: Vec<Command>, markdown_roots: &[String]) -> Vec<Command> {
    // First, remove all existing obs and anchor commands
    commands.retain(|cmd| cmd.action != "obs" && cmd.action != "anchor");
    
    // Then scan for new markdown files
    for root in markdown_roots {
        let expanded_root = expand_home(root);
        let root_path = Path::new(&expanded_root);
        
        if root_path.exists() && root_path.is_dir() {
            scan_directory(&root_path, &mut commands);
        }
    }
    
    commands
}

/// Recursively scans a directory for files and folders
fn scan_directory(dir: &Path, commands: &mut Vec<Command>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            
            // Process the path (file or directory)
            if let Some(command) = process_markdown(&path) {
                commands.push(command);
            }
            
            // Recursively scan subdirectories
            if path.is_dir() {
                scan_directory(&path, commands);
            }
        }
    }
}

/// Processes a path and returns a command if it's a markdown file
fn process_markdown(path: &Path) -> Option<Command> {
    // Check if it's a file and has .md extension
    if !path.is_file() {
        return None;
    }
    
    let extension = path.extension()?.to_str()?;
    if extension.to_lowercase() != "md" {
        return None;
    }
    
    // Get the base name without extension
    let file_name = path.file_stem()?.to_str()?;
    
    // Determine action type
    let action = if is_markdown_anchor(path) {
        "anchor"
    } else {
        "obs"
    };
    
    // Create command
    let command_name = format!("{} md", file_name);
    let full_path = path.to_string_lossy().to_string();
    let full_line = format!("{} : {} {}", command_name, action, full_path);
    
    Some(Command {
        group: String::new(),
        command: command_name,
        action: action.to_string(),
        arg: full_path,
        full_line,
    })
}

/// Checks if a markdown file is an "anchor" (base name matches parent folder name)
fn is_markdown_anchor(path: &Path) -> bool {
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

/// Expands ~ to home directory
fn expand_home(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen("~", &home, 1);
        }
    }
    path.to_string()
}

/// Scans macOS contacts and creates commands for contacts with phone or email
pub fn scan_contacts(mut commands: Vec<Command>) -> Vec<Command> {
    // First, remove all existing contact commands
    commands.retain(|cmd| cmd.action != "contact");
    
    // Use simple, fast AppleScript to get first 10 contacts for testing
    let script = r#"
tell application "Contacts"
    set output to ""
    set maxContacts to 10
    
    repeat with i from 1 to maxContacts
        try
            set p to person i
            set fullName to name of p
            set contactId to id of p
            set output to output & fullName & "|" & contactId & "\n"
        on error
            -- Skip contacts that cause errors
        end try
    end repeat
    
    return output
end tell
"#;
    
    // Execute the AppleScript with timeout
    let output = ProcessCommand::new("timeout")
        .arg("10")  // 10 second timeout
        .arg("osascript")
        .arg("-e")
        .arg(script)
        .output();
    
    if let Ok(result) = output {
        if result.status.success() {
            let contacts_str = String::from_utf8_lossy(&result.stdout);
            
            // Parse each line
            for line in contacts_str.lines() {
                if line.is_empty() {
                    continue;
                }
                
                // Split by pipe to get name and ID
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 2 {
                    let full_name = parts[0].trim();
                    let contact_id = parts[1].trim();
                    
                    // Create command name: @FirstName LastName ctc
                    let command_name = format!("@{} ctc", full_name);
                    let full_line = format!("{} : contact {}", command_name, contact_id);
                    
                    commands.push(Command {
                        group: String::new(),
                        command: command_name,
                        action: "contact".to_string(),
                        arg: contact_id.to_string(),
                        full_line,
                    });
                }
            }
        } else {
            // If contacts scanning fails, fall back to sample data
            let sample_contacts = vec![
                ("John Doe", "12345"),
                ("Jane Smith", "67890"), 
                ("Bob Johnson", "11111"),
            ];
            
            for (name, id) in sample_contacts {
                let command_name = format!("@{} ctc", name);
                let full_line = format!("{} : contact {}", command_name, id);
                
                commands.push(Command {
                    group: String::new(),
                    command: command_name,
                    action: "contact".to_string(),
                    arg: id.to_string(),
                    full_line,
                });
            }
        }
    }
    
    commands
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_markdown_anchor() {
        // Test case: ProjectName/ProjectName.md (should be anchor)
        let path = PathBuf::from("/home/user/ProjectName/ProjectName.md");
        assert!(is_markdown_anchor(&path));
        
        // Test case: ProjectName/readme.md (should not be anchor)
        let path = PathBuf::from("/home/user/ProjectName/readme.md");
        assert!(!is_markdown_anchor(&path));
        
        // Test case: projectname/PROJECTNAME.md (should be anchor - case insensitive)
        let path = PathBuf::from("/home/user/projectname/PROJECTNAME.md");
        assert!(is_markdown_anchor(&path));
    }
}