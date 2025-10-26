//! User-level command operations
//! 
//! This module contains operations that users directly invoke on the command set,
//! such as adding, deleting, and renaming commands with their associated data.

use std::collections::HashMap;
use crate::core::{Command, Config, Patch};

// ============================================================================
// Simple Command Operations
// ============================================================================

/// Adds a new command to the list and saves
/// Validate an alias command for cycles before adding it
fn validate_alias_command(new_command: &Command, _commands: &[Command]) -> Result<(), String> {
    if new_command.action != "alias" {
        return Ok(());
    }
    
    // Prevent self-referential alias (A -> A) - this is the most critical check
    if new_command.command == new_command.arg {
        return Err(format!("Cannot create self-referential alias: '{}' cannot alias to itself", new_command.command));
    }
    
    // Note: More complex cycle detection would require loading the full command set
    // and doing graph traversal, but the self-referential check catches the most
    // dangerous case that causes immediate stack overflow
    
    Ok(())
}

pub fn add_command(new_command: Command, commands: &mut Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
    // Validate before making any changes
    validate_alias_command(&new_command, commands)?;

    // Use sys_data to add (automatically records history + inference + save)
    crate::core::data::add_command(new_command)?;

    // Reload commands into the provided vec for backward compatibility
    *commands = crate::core::data::get_commands();
    Ok(())
}

/// Deletes a command from the list and saves
pub fn delete_command(command_to_delete: &str, commands: &mut Vec<Command>) -> Result<(), Box<dyn std::error::Error>> {
    // Use sys_data to delete (automatically saves + inference)
    crate::core::data::delete_command(command_to_delete)?;

    // Reload commands into the provided vec for backward compatibility
    *commands = crate::core::data::get_commands();
    Ok(())
}

// ============================================================================
// Complex Command Operations  
// ============================================================================

/// Rename associated data when a command name is changed
/// 
/// This function handles renaming of associated files, folders, patches, and prefixes
/// when a command name is edited in the command editor.
/// 
/// Algorithm:
/// 1. Document Renaming (rename_doc flag): Renames physical document files that match the command name
/// 2. Folder Renaming (rename_folder flag): Renames folders for anchor commands where folder matches command name  
/// 3. Patch Renaming (rename_patch flag): Updates patch names and all commands referencing that patch
/// 4. Prefix Renaming (rename_prefix flag): Updates command names that have the old name as a prefix
/// 
/// Parameters:
/// - dry_run: If true, only returns descriptions of what would be done without making changes
/// 
/// Returns: Tuple of (updated_arg, list of action descriptions)
pub fn rename_associated_data(
    old_name: &str,
    new_name: &str,
    current_arg: &str,
    action: &str,
    commands: &mut [Command],
    patches: &mut HashMap<String, Patch>,
    config: &Config,
    dry_run: bool,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    
    let mut updated_arg = current_arg.to_string();
    let mut actions = Vec::new();
    
    // Helper to normalize names for comparison (lowercase, remove spaces)
    let normalize_for_match = |s: &str| -> String {
        s.to_lowercase().replace(' ', "").replace('_', "").replace('-', "")
    };
    
    // 1. Document Renaming
    if config.popup_settings.rename_doc.unwrap_or(false) {
        // Check if this is a document action
        if matches!(action, "markdown" | "text" | "doc" | "anchor") {
            let path = Path::new(current_arg);
            if path.exists() && path.is_file() {
                // Check if the file basename matches the old command name
                if let Some(file_stem) = path.file_stem() {
                    if let Some(file_stem_str) = file_stem.to_str() {
                        if normalize_for_match(file_stem_str) == normalize_for_match(old_name) {
                            // Build new file path with new name
                            let extension = path.extension().unwrap_or_default();
                            let new_file_name = if extension.is_empty() {
                                new_name.to_string()
                            } else {
                                format!("{}.{}", new_name, extension.to_str().unwrap())
                            };
                            
                            let new_path = path.parent()
                                .map(|p| p.join(&new_file_name))
                                .unwrap_or_else(|| PathBuf::from(new_file_name));
                            
                            // Rename the file
                            if !new_path.exists() {
                                let file_name = path.file_name().unwrap().to_string_lossy();
                                let new_file = new_path.file_name().unwrap().to_string_lossy();
                                actions.push(format!("DOC -- Rename file: {} → {}", file_name, new_file));
                                
                                if !dry_run {
                                    crate::utils::log(&format!("RENAME: Renaming document {} -> {}", 
                                        path.display(), new_path.display()));
                                    fs::rename(path, &new_path)?;
                                    updated_arg = new_path.to_string_lossy().to_string();
                                }
                            } else if dry_run {
                                actions.push(format!("WARNING -- Cannot rename file (target exists): {}",
                                    new_path.file_name().unwrap().to_string_lossy()));
                            }
                        }
                    }
                }
            }
        }
    }
    
    // 2. Folder Renaming (for anchor commands)
    if config.popup_settings.rename_folder.unwrap_or(false) && action == "anchor" {
        let path = Path::new(&updated_arg);
        
        // For anchor files, check if folder name matches old command name
        if let Some(parent) = path.parent() {
            if let Some(folder_name) = parent.file_name() {
                if let Some(folder_name_str) = folder_name.to_str() {
                    if normalize_for_match(folder_name_str) == normalize_for_match(old_name) {
                        // Build new folder path
                        let new_folder_path = parent.parent()
                            .map(|p| p.join(new_name))
                            .unwrap_or_else(|| PathBuf::from(new_name));
                        
                        if !new_folder_path.exists() && parent.exists() {
                            // Add action descriptions
                            actions.push(format!("FOLDER -- Rename folder: {} → {}",
                                folder_name_str, new_name));
                            actions.push(format!("ANCHOR -- Update anchor command: {} → {}",
                                folder_name_str, new_name));
                            
                            // Perform rename if not dry run
                            if !dry_run {
                                crate::utils::log(&format!("RENAME: Renaming folder {} -> {}", 
                                    parent.display(), new_folder_path.display()));
                                fs::rename(parent, &new_folder_path)?;
                                
                                // Update the arg to point to the new location
                                // Rebuild the file path within the renamed folder
                                if let Some(file_name) = path.file_name() {
                                    // Also rename the anchor file itself to match
                                    let new_anchor_file = format!("{}.md", new_name);
                                    let new_full_path = new_folder_path.join(&new_anchor_file);
                                    let old_file_in_new_folder = new_folder_path.join(file_name);
                                    
                                    // Rename the anchor file to match the folder
                                    if old_file_in_new_folder.exists() && !new_full_path.exists() {
                                        fs::rename(&old_file_in_new_folder, &new_full_path)?;
                                        updated_arg = new_full_path.to_string_lossy().to_string();
                                    } else {
                                        updated_arg = old_file_in_new_folder.to_string_lossy().to_string();
                                    }
                                }
                            }
                        } else if new_folder_path.exists() && dry_run {
                            actions.push(format!("WARNING -- Cannot rename folder (target exists): {}", new_name));
                        }
                    }
                }
            }
        }
    }
    
    // 3. Patch Renaming
    if config.popup_settings.rename_patch.unwrap_or(false) {
        // Check if there's a patch with the old name
        let old_name_lower = old_name.to_lowercase();
        if patches.contains_key(&old_name_lower) {
            // Count affected commands
            let mut affected_commands = Vec::new();
            for cmd in commands.iter() {
                if cmd.patch.to_lowercase() == old_name_lower ||
                   cmd.patch.to_lowercase().starts_with(&format!("{}!", old_name_lower)) {
                    affected_commands.push(cmd.command.clone());
                }
            }
            
            if !affected_commands.is_empty() {
                // Add action description with complete command listing
                let mut action_desc = format!("PATCH -- Update patch from {} to {} for the following commands:", old_name, new_name);
                
                // Add all affected commands, 4 spaces indented, wrapping lines as needed
                let mut current_line = String::from("    ");
                for (i, cmd) in affected_commands.iter().enumerate() {
                    let cmd_text = if i == affected_commands.len() - 1 {
                        cmd.clone() // Last command, no comma
                    } else {
                        format!("{}, ", cmd) // Add comma and space
                    };
                    
                    // Check if adding this command would make the line too long (>70 chars)
                    if current_line.len() + cmd_text.len() > 70 && current_line.len() > 4 {
                        // Add the current line and start a new one
                        action_desc.push('\n');
                        action_desc.push_str(&current_line);
                        current_line = format!("    {}", cmd_text);
                    } else {
                        current_line.push_str(&cmd_text);
                    }
                }
                
                // Add the final line if it has content
                if current_line.len() > 4 {
                    action_desc.push('\n');
                    action_desc.push_str(&current_line);
                }
                
                actions.push(action_desc);
                
                // Perform update if not dry run
                if !dry_run {
                    crate::utils::log(&format!("RENAME: Renaming patch '{}' -> '{}'", old_name, new_name));
                    
                    // Update the patch in the patches map
                    if let Some(mut patch) = patches.remove(&old_name_lower) {
                        patch.name = new_name.to_string();
                        patches.insert(new_name.to_lowercase(), patch);
                    }
                    
                    // Update all commands that have this patch
                    for cmd in commands.iter_mut() {
                        if cmd.patch.to_lowercase() == old_name_lower {
                            cmd.patch = new_name.to_string();
                            cmd.update_full_line();
                        } else if cmd.patch.to_lowercase().starts_with(&format!("{}!", old_name_lower)) {
                            cmd.patch = format!("{}!", new_name);
                            cmd.update_full_line();
                        }
                    }
                    crate::utils::log(&format!("RENAME: Updated {} commands with new patch name", 
                        affected_commands.len()));
                }
            }
        }
    }
    
    // 4. Prefix Renaming  
    if config.popup_settings.rename_prefix.unwrap_or(false) {
        let separators = &config.popup_settings.word_separators;
        
        // Find affected commands
        let mut affected_commands = Vec::new();
        for cmd in commands.iter() {
            // Skip the command being renamed
            if cmd.command == old_name {
                continue;
            }
            
            // Check if this command starts with old_name followed by a separator
            let old_name_chars: Vec<char> = old_name.chars().collect();
            let cmd_chars: Vec<char> = cmd.command.chars().collect();
            
            if cmd_chars.len() > old_name_chars.len() {
                // Extract prefix characters
                let prefix_chars: String = cmd_chars[..old_name_chars.len()].iter().collect();
                let next_char = cmd_chars[old_name_chars.len()];
                
                // Check if prefix matches (case-insensitive) and next char is a separator
                if prefix_chars.to_lowercase() == old_name.to_lowercase() && 
                   separators.contains(next_char) {
                    affected_commands.push(cmd.command.clone());
                }
            }
        }
        
        if !affected_commands.is_empty() {
            // Add action description with complete command listing
            let mut action_desc = format!("PREFIX -- Update prefix from {} to {} for the following commands:", old_name, new_name);
            
            // Add all affected commands, 4 spaces indented, wrapping lines as needed
            let mut current_line = String::from("    ");
            for (i, cmd) in affected_commands.iter().enumerate() {
                let cmd_text = if i == affected_commands.len() - 1 {
                    cmd.clone() // Last command, no comma
                } else {
                    format!("{}, ", cmd) // Add comma and space
                };
                
                // Check if adding this command would make the line too long (>80 chars)
                if current_line.len() + cmd_text.len() > 80 && current_line.len() > 4 {
                    // Add the current line and start a new one
                    action_desc.push('\n');
                    action_desc.push_str(&current_line);
                    current_line = format!("    {}", cmd_text);
                } else {
                    current_line.push_str(&cmd_text);
                }
            }
            
            // Add the final line if it has content
            if current_line.len() > 4 {
                action_desc.push('\n');
                action_desc.push_str(&current_line);
            }
            
            actions.push(action_desc);
            
            // Perform update if not dry run
            if !dry_run {
                for cmd in commands.iter_mut() {
                    // Skip the command being renamed
                    if cmd.command == old_name {
                        continue;
                    }
                    
                    // Check if this command starts with old_name followed by a separator
                    let old_name_chars: Vec<char> = old_name.chars().collect();
                    let cmd_chars: Vec<char> = cmd.command.chars().collect();
                    
                    if cmd_chars.len() > old_name_chars.len() {
                        // Extract prefix characters
                        let prefix_chars: String = cmd_chars[..old_name_chars.len()].iter().collect();
                        let next_char = cmd_chars[old_name_chars.len()];
                        
                        // Check if prefix matches (case-insensitive) and next char is a separator
                        if prefix_chars.to_lowercase() == old_name.to_lowercase() && 
                           separators.contains(next_char) {
                            // Replace the prefix with the new name
                            let remainder_chars: String = cmd_chars[old_name_chars.len()..].iter().collect();
                            let old_cmd_name = cmd.command.clone();
                            cmd.command = format!("{}{}", new_name, remainder_chars);
                            cmd.update_full_line();
                            crate::utils::log(&format!("RENAME: Updated prefix '{}' -> '{}'", 
                                old_cmd_name, cmd.command));
                        }
                    }
                }
                crate::utils::log(&format!("RENAME: Updated {} commands with new prefix", 
                    affected_commands.len()));
            }
        }
    }
    
    Ok((updated_arg, actions))
}

/// Rename a folder and update all command ARGs that reference it
///
/// This handles the special case when renaming an anchor file (where filename matches folder name).
/// It renames the containing folder and updates all commands whose ARG paths reference the old folder.
///
/// Parameters:
/// - old_folder_path: The current path to the folder
/// - new_folder_name: The new name for the folder (just the name, not full path)
/// - commands: Mutable reference to all commands
/// - dry_run: If true, only returns descriptions without making changes
///
/// Returns: Vec<String> of action descriptions
pub fn rename_folder(
    old_folder_path: &str,
    new_folder_name: &str,
    commands: &mut [Command],
    dry_run: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    let mut actions = Vec::new();
    let old_path = Path::new(old_folder_path);

    // Validate that the old folder exists
    if !old_path.exists() || !old_path.is_dir() {
        return Err(format!("Folder does not exist: {}", old_folder_path).into());
    }

    // Build new folder path (same parent, new name)
    let new_folder_path = old_path.parent()
        .ok_or("Cannot determine parent directory")?
        .join(new_folder_name);

    // Check if target already exists
    if new_folder_path.exists() {
        return Err(format!("Target folder already exists: {}", new_folder_path.display()).into());
    }

    // Find all commands with ARGs that reference this folder
    let mut affected_commands = Vec::new();
    let old_path_str = old_folder_path.to_string();

    for cmd in commands.iter() {
        // Check if the arg starts with the old folder path
        if cmd.arg.starts_with(&old_path_str) {
            affected_commands.push((cmd.command.clone(), cmd.arg.clone()));
        }
    }

    // Build action description
    actions.push(format!("FOLDER -- Rename folder: {} → {}",
        old_folder_path, new_folder_name));

    // Add indented line for path updates if any commands are affected
    if !affected_commands.is_empty() {
        actions.push(format!("  Update file paths in {} command{} to match new folder",
            affected_commands.len(),
            if affected_commands.len() == 1 { "" } else { "s" }));
    }

    // Execute if not dry run
    if !dry_run {
        crate::utils::log(&format!("RENAME_FOLDER: Renaming {} -> {}",
            old_path.display(), new_folder_path.display()));

        // Rename the folder
        fs::rename(old_path, &new_folder_path)?;

        // Update all affected command ARGs
        let new_path_str = new_folder_path.to_string_lossy().to_string();
        for cmd in commands.iter_mut() {
            if cmd.arg.starts_with(&old_path_str) {
                let old_cmd_arg = cmd.arg.clone();
                cmd.arg = cmd.arg.replacen(&old_path_str, &new_path_str, 1);
                cmd.update_full_line();

                crate::utils::log(&format!("RENAME_FOLDER: Updated command '{}' arg: {} -> {}",
                    cmd.command, old_cmd_arg, cmd.arg));
            }
        }

        crate::utils::log(&format!("RENAME_FOLDER: Updated {} command ARGs",
            affected_commands.len()));
    }

    Ok(actions)
}