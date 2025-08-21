//! Command operations module for complex transformations
//! 
//! This module handles operations that transform or merge commands and their
//! associated filesystem structures, such as merging orphaned anchors.

use std::fs;
use std::path::{Path, PathBuf};
use crate::utils::log;

/// Find orphan folders that should be merged with their non-orphan counterparts
/// Returns pairs of (orphan_folder, target_folder) that should be merged
pub fn find_orphan_folder_merges(
    orphans_path: &Path,
    vault_root: &Path,
) -> Vec<(PathBuf, PathBuf)> {
    let mut merges = Vec::new();
    
    // Read all folders in orphans directory
    let orphan_entries = match fs::read_dir(orphans_path) {
        Ok(entries) => entries,
        Err(e) => {
            log(&format!("MERGE: Could not read orphans directory: {}", e));
            return merges;
        }
    };
    
    for entry in orphan_entries.filter_map(Result::ok) {
        let orphan_folder = entry.path();
        if !orphan_folder.is_dir() {
            continue;
        }
        
        // Check if this folder has an anchor file (folder_name.md)
        let folder_name = match orphan_folder.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => continue,
        };
        
        let orphan_anchor = orphan_folder.join(format!("{}.md", folder_name));
        if !orphan_anchor.exists() {
            crate::utils::detailed_log("MERGE", &format!("Skipping {} - no anchor file at {}", folder_name, orphan_anchor.display()));
            continue;
        }
        
        // Search for a matching anchor outside orphans
        crate::utils::detailed_log("MERGE", &format!("Searching for match for orphan anchor: {}", folder_name));
        if let Some(target_folder) = find_matching_anchor(&folder_name, vault_root, orphans_path) {
            // Keep this as regular log since merges are important events
            log(&format!("MERGE: Found merge candidate: {} -> {}", 
                orphan_folder.display(), target_folder.display()));
            merges.push((orphan_folder, target_folder));
        } else {
            crate::utils::detailed_log("MERGE", &format!("No match found for orphan anchor: {}", folder_name));
        }
    }
    
    merges
}

/// Find a matching anchor folder in the vault (excluding orphans)
fn find_matching_anchor(
    anchor_name: &str,
    search_root: &Path,
    orphans_path: &Path,
) -> Option<PathBuf> {
    find_matching_anchor_recursive(anchor_name, search_root, orphans_path)
}

fn find_matching_anchor_recursive(
    anchor_name: &str,
    current_dir: &Path,
    orphans_path: &Path,
) -> Option<PathBuf> {
    // Skip if we're in the orphans directory
    if current_dir.starts_with(orphans_path) {
        return None;
    }
    
    // First check if current directory contains the standard anchor file (folder_name/folder_name.md)
    let potential_anchor = current_dir.join(anchor_name).join(format!("{}.md", anchor_name));
    if potential_anchor.exists() {
        log(&format!("MERGE: Found matching anchor at: {}", potential_anchor.display()));
        return Some(current_dir.join(anchor_name));
    }
    
    // NEW: Also check if any directory contains an anchor file with the target name
    // This handles cases like HookAnchor/HA.md where HA is an alias for HookAnchor
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                // Skip hidden directories
                if let Some(name) = path.file_name() {
                    if name.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }
                
                // Check if this directory contains the anchor file we're looking for
                let alias_anchor = path.join(format!("{}.md", anchor_name));
                if alias_anchor.exists() {
                    // Verify it's actually marked as an anchor by checking if it's in our commands
                    // This prevents false positives from random markdown files
                    log(&format!("MERGE: Found potential alias anchor at: {}", alias_anchor.display()));
                    return Some(path);
                }
            }
        }
    }
    
    // Recursively search subdirectories
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                // Skip hidden directories
                if let Some(name) = path.file_name() {
                    if name.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }
                
                if let Some(found) = find_matching_anchor_recursive(anchor_name, &path, orphans_path) {
                    return Some(found);
                }
            }
        }
    }
    
    None
}

/// Merge one folder into another (source -> destination)
/// This will merge markdown content and move all files from source to destination
pub fn merge_folders(
    source_folder: &Path,
    dest_folder: &Path,
) -> Result<(), String> {
    log(&format!("MERGE: Starting merge of {} -> {}", 
        source_folder.display(), dest_folder.display()));
    
    // Get folder name for anchor file
    let folder_name = source_folder
        .file_name()
        .ok_or_else(|| "Invalid source folder path".to_string())?
        .to_string_lossy()
        .to_string();
    
    // Merge markdown files
    let source_md = source_folder.join(format!("{}.md", folder_name));
    
    // Check for both standard location and alias location
    let dest_folder_name = dest_folder
        .file_name()
        .ok_or_else(|| "Invalid destination folder path".to_string())?
        .to_string_lossy()
        .to_string();
    
    // Try the alias location first (e.g., HookAnchor/HA.md)
    let dest_md_alias = dest_folder.join(format!("{}.md", folder_name));
    // Then try the standard location (e.g., HookAnchor/HookAnchor.md)
    let dest_md_standard = dest_folder.join(format!("{}.md", dest_folder_name));
    
    // Use the alias location if it exists, otherwise use standard
    let dest_md = if dest_md_alias.exists() {
        log(&format!("MERGE: Using alias anchor file: {}", dest_md_alias.display()));
        dest_md_alias.clone()
    } else {
        dest_md_standard
    };
    
    if source_md.exists() && dest_md.exists() {
        merge_markdown_files(&source_md, &dest_md)?;
        
        // Delete the source markdown file after successful merge
        fs::remove_file(&source_md)
            .map_err(|e| format!("Failed to remove source markdown after merge: {}", e))?;
        log(&format!("MERGE: Removed source markdown: {}", source_md.display()));
    } else if source_md.exists() && !dest_md.exists() {
        // If destination doesn't exist, just move the source file
        log(&format!("MERGE: Moving orphan anchor to destination: {} -> {}", 
            source_md.display(), dest_md.display()));
        fs::rename(&source_md, &dest_md)
            .map_err(|e| format!("Failed to move orphan anchor: {}", e))?;
    }
    
    // Migrate all other files
    migrate_files(source_folder, dest_folder, &folder_name)?;
    
    // Remove empty source folder
    cleanup_empty_folder(source_folder)?;
    
    log(&format!("MERGE: Completed merge successfully"));
    Ok(())
}

/// Merge two markdown files by appending source to destination
fn merge_markdown_files(
    source_md: &Path,
    dest_md: &Path,
) -> Result<(), String> {
    let source_content = fs::read_to_string(source_md)
        .map_err(|e| format!("Failed to read source markdown: {}", e))?;
    
    let dest_content = fs::read_to_string(dest_md)
        .map_err(|e| format!("Failed to read destination markdown: {}", e))?;
    
    // Check if source content is just auto-generated template
    let is_auto_generated = source_content.contains("This is an auto-generated anchor file");
    
    // Check if destination already contains the auto-generated text
    let dest_has_auto_generated = dest_content.contains("This is an auto-generated anchor file");
    
    // If both source and destination have auto-generated content, or if source is just
    // auto-generated and destination already has content, skip the merge
    if is_auto_generated && (dest_has_auto_generated || !dest_content.trim().is_empty()) {
        log(&format!("MERGE: Skipping merge of auto-generated content into {}", 
            dest_md.display()));
        return Ok(());
    }
    
    // Otherwise, merge with a separator
    let merged_content = format!("{}\n\n---\n\n{}", dest_content, source_content);
    
    fs::write(dest_md, merged_content)
        .map_err(|e| format!("Failed to write merged markdown: {}", e))?;
    
    log(&format!("MERGE: Merged markdown content ({} + {} -> {})", 
        source_md.display(), dest_md.display(), dest_md.display()));
    
    Ok(())
}

/// Migrate all files from source folder to destination folder
/// Skip the anchor markdown file itself as it's already merged
fn migrate_files(
    source_folder: &Path,
    dest_folder: &Path,
    anchor_name: &str,
) -> Result<(), String> {
    let anchor_filename = format!("{}.md", anchor_name);
    
    let entries = fs::read_dir(source_folder)
        .map_err(|e| format!("Failed to read source folder: {}", e))?;
    
    for entry in entries.filter_map(Result::ok) {
        let source_path = entry.path();
        let filename = match source_path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => continue,
        };
        
        // Skip the anchor markdown file
        if filename == anchor_filename {
            continue;
        }
        
        // Skip directories for now (could be enhanced to handle subdirs)
        if source_path.is_dir() {
            log(&format!("MERGE: Skipping subdirectory: {}", source_path.display()));
            continue;
        }
        
        // Generate unique filename if needed
        let dest_path = generate_unique_path(dest_folder, &filename);
        
        // Move the file
        fs::rename(&source_path, &dest_path)
            .map_err(|e| format!("Failed to move file {}: {}", source_path.display(), e))?;
        
        if dest_path.file_name().unwrap().to_string_lossy() != filename {
            log(&format!("MERGE: Moved file: {} -> {} (resolved naming conflict)", 
                source_path.display(), dest_path.display()));
        } else {
            log(&format!("MERGE: Moved file: {} -> {}", 
                source_path.display(), dest_path.display()));
        }
    }
    
    Ok(())
}

/// Generate a unique path by adding _1, _2, etc. suffix if needed
fn generate_unique_path(folder: &Path, filename: &str) -> PathBuf {
    let mut candidate = folder.join(filename);
    
    if !candidate.exists() {
        return candidate;
    }
    
    // Split filename into base and extension
    let (base, ext) = match filename.rfind('.') {
        Some(pos) => (&filename[..pos], &filename[pos..]),
        None => (filename, ""),
    };
    
    // Try suffixes until we find an available name
    let mut counter = 1;
    loop {
        candidate = folder.join(format!("{}_{}{}", base, counter, ext));
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
        
        if counter > 1000 {
            // Safety check to prevent infinite loop
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            candidate = folder.join(format!("{}_{}_t{}{}", base, counter, timestamp, ext));
            return candidate;
        }
    }
}

/// Remove an empty folder
fn cleanup_empty_folder(folder: &Path) -> Result<(), String> {
    // Check if folder is truly empty
    let is_empty = fs::read_dir(folder)
        .map_err(|e| format!("Failed to check folder contents: {}", e))?
        .next()
        .is_none();
    
    if is_empty {
        fs::remove_dir(folder)
            .map_err(|e| format!("Failed to remove empty folder: {}", e))?;
        log(&format!("MERGE: Removed empty folder: {}", folder.display()));
    } else {
        log(&format!("MERGE: Folder not empty, keeping: {}", folder.display()));
    }
    
    Ok(())
}