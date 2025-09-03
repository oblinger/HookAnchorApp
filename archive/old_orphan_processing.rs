// Old Orphan Processing Code
// Archived before revamping the orphan processing system
// This file contains the original implementation of orphan anchor detection and merging

// ============================================================================
// From src/systems/scanner.rs
// ============================================================================

/// Process orphan merges during rescanning
/// Called from rescan_commands() around line 137-190
fn process_orphan_merges(sys_data: &mut SysData) {
    // Process orphan merges after scanning files
    if let Some(orphans_path_str) = &sys_data.config.popup_settings.orphans_path {
        let expanded_orphans_path = expand_home(orphans_path_str);
        let orphans_path = std::path::Path::new(&expanded_orphans_path);
        
        for root in &sys_data.config.scan_roots {
            let expanded_root = expand_home(root);
            let vault_root = std::path::Path::new(&expanded_root);
            
            // Skip orphan merge checking for /Applications and other system directories
            // Orphan merging only makes sense for markdown vault directories
            if expanded_root.starts_with("/Applications") 
                || expanded_root.starts_with("/System")
                || expanded_root.starts_with("/Library")
                || !vault_root.exists() {
                continue;
            }
            
            println!("\n🔄 Checking for orphan anchors to merge in {}...", expanded_root);
            
            // Find orphan folders that should be merged
            let merges = crate::core::commands::find_orphan_folder_merges(orphans_path, vault_root);
            
            if !merges.is_empty() {
                println!("   Found {} orphan anchors to merge", merges.len());
                
                for (source, dest) in merges {
                    println!("   📁 Merging: {} -> {}", 
                        source.file_name().unwrap_or_default().to_string_lossy(),
                        dest.display());
                    
                    if let Err(e) = crate::core::commands::merge_orphan_folder(&source, &dest) {
                        crate::utils::log_error(&format!("Failed to merge orphan: {}", e));
                    } else {
                        println!("      ✅ Merge completed");
                    }
                }
                
                // After merging, we need to rescan to pick up the changes
                println!("   Rescanning after merges...");
                *sys_data = rescan_all(&sys_data.config, false);
            } else {
                println!("   No orphan anchors found to merge");
            }
        }
    }
}

// ============================================================================
// From src/core/commands.rs (lines 2667-3018)
// ============================================================================

/// Find orphan folders that should be merged with their non-orphan counterparts
/// Returns pairs of (orphan_folder, target_folder) that should be merged
pub(crate) fn find_orphan_folder_merges(
    orphans_path: &Path,
    vault_root: &Path,
) -> Vec<(PathBuf, PathBuf)> {
    let mut merges = Vec::new();
    
    // Read all folders in orphans directory
    let orphan_entries = match fs::read_dir(orphans_path) {
        Ok(entries) => entries,
        Err(e) => {
            crate::utils::log(&format!("MERGE: Could not read orphans directory: {}", e));
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
            crate::utils::log(&format!("MERGE: Found merge candidate: {} -> {}", 
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
    
    // Skip .app bundles and other system directories to prevent deep recursion
    if let Some(dir_name) = current_dir.file_name() {
        let name_str = dir_name.to_string_lossy();
        if name_str.ends_with(".app") || 
           name_str.ends_with(".framework") || 
           name_str.ends_with(".bundle") ||
           name_str.ends_with(".plugin") {
            return None;
        }
    }
    
    // First check if current directory contains the standard anchor file (folder_name/folder_name.md)
    let potential_anchor = current_dir.join(anchor_name).join(format!("{}.md", anchor_name));
    if potential_anchor.exists() {
        crate::utils::log(&format!("MERGE: Found matching anchor at: {}", potential_anchor.display()));
        return Some(current_dir.join(anchor_name));
    }
    
    // NEW: Also check if any directory contains an anchor file with the target name
    // This handles cases like HookAnchor/HA.md where HA is an alias for HookAnchor
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                // Skip hidden directories and .app bundles
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with('.') ||
                       name_str.ends_with(".app") ||
                       name_str.ends_with(".framework") ||
                       name_str.ends_with(".bundle") ||
                       name_str.ends_with(".plugin") {
                        continue;
                    }
                }
                
                // Check for alias anchor file in this directory
                let alias_anchor = path.join(format!("{}.md", anchor_name));
                if alias_anchor.exists() {
                    crate::utils::log(&format!("MERGE: Found alias anchor at: {}", alias_anchor.display()));
                    return Some(path);
                }
            }
        }
    }
    
    // Then recurse into subdirectories
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
                
                if let Some(result) = find_matching_anchor_recursive(anchor_name, &path, orphans_path) {
                    return Some(result);
                }
            }
        }
    }
    
    None
}

/// Merge an orphan folder with its non-orphan counterpart
pub(crate) fn merge_orphan_folder(
    source_folder: &Path,
    dest_folder: &Path,
) -> Result<(), String> {
    crate::utils::log(&format!("MERGE: Starting merge: {} -> {}", 
        source_folder.display(), dest_folder.display()));
    
    // Get the folder name for the anchor file
    let folder_name = match source_folder.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err("Invalid source folder path".to_string()),
    };
    
    let source_md = source_folder.join(format!("{}.md", folder_name));
    
    // Check for both standard and alias anchor files in destination
    let dest_md_standard = dest_folder.join(format!("{}.md", folder_name));
    
    // For alias anchor, use the destination folder's actual name
    let dest_folder_name = match dest_folder.file_name() {
        Some(name) => name.to_string_lossy().to_string(), 
        None => return Err("Invalid destination folder path".to_string()),
    };
    let dest_md_alias = dest_folder.join(format!("{}.md", folder_name));
    
    // Use the alias location if it exists, otherwise use standard
    let dest_md = if dest_md_alias.exists() {
        crate::utils::log(&format!("MERGE: Using alias anchor file: {}", dest_md_alias.display()));
        dest_md_alias.clone()
    } else {
        dest_md_standard
    };
    
    if source_md.exists() && dest_md.exists() {
        merge_markdown_files(&source_md, &dest_md)?;
        
        // Delete the source markdown file after successful merge
        fs::remove_file(&source_md)
            .map_err(|e| format!("Failed to remove source markdown after merge: {}", e))?;
        crate::utils::log(&format!("MERGE: Removed source markdown: {}", source_md.display()));
    } else if source_md.exists() && !dest_md.exists() {
        // If destination doesn't exist, just move the source file
        crate::utils::log(&format!("MERGE: Moving orphan anchor to destination: {} -> {}", 
            source_md.display(), dest_md.display()));
        fs::rename(&source_md, &dest_md)
            .map_err(|e| format!("Failed to move orphan anchor: {}", e))?;
    }
    
    // Migrate all other files
    migrate_files(source_folder, dest_folder, &folder_name)?;
    
    // Remove empty source folder
    cleanup_empty_folder(source_folder)?;
    
    crate::utils::log(&format!("MERGE: Completed merge successfully"));
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
        crate::utils::log(&format!("MERGE: Skipping merge of auto-generated content into {}", 
            dest_md.display()));
        return Ok(());
    }
    
    // Otherwise, merge with a separator
    let merged_content = format!("{}\n\n---\n\n{}", dest_content, source_content);
    
    fs::write(dest_md, merged_content)
        .map_err(|e| format!("Failed to write merged markdown: {}", e))?;
    
    crate::utils::log(&format!("MERGE: Merged markdown content ({} + {} -> {})", 
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
            crate::utils::log(&format!("MERGE: Skipping subdirectory: {}", source_path.display()));
            continue;
        }
        
        // Generate unique filename if needed
        let dest_path = generate_unique_path(dest_folder, &filename);
        
        // Move the file
        fs::rename(&source_path, &dest_path)
            .map_err(|e| format!("Failed to move file {}: {}", source_path.display(), e))?;
        
        if dest_path.file_name().unwrap().to_string_lossy() != filename {
            crate::utils::log(&format!("MERGE: Moved file: {} -> {} (resolved naming conflict)", 
                source_path.display(), dest_path.display()));
        } else {
            crate::utils::log(&format!("MERGE: Moved file: {} -> {}", 
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
    let (base, ext) = if let Some(dot_pos) = filename.rfind('.') {
        (&filename[..dot_pos], &filename[dot_pos..])
    } else {
        (filename, "")
    };
    
    // Try adding _1, _2, etc.
    for counter in 1..100 {
        candidate = folder.join(format!("{}_{}{}", base, counter, ext));
        if !candidate.exists() {
            return candidate;
        }
    }
    
    // If we somehow have 100+ conflicts, use timestamp
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    folder.join(format!("{}_{}_t{}{}", base, 100, timestamp, ext))
}

/// Remove an empty folder
fn cleanup_empty_folder(folder: &Path) -> Result<(), String> {
    // Check if folder is truly empty
    let is_empty = fs::read_dir(folder)
        .map_err(|e| format!("Failed to read folder: {}", e))?
        .next()
        .is_none();
    
    if is_empty {
        fs::remove_dir(folder)
            .map_err(|e| format!("Failed to remove empty folder: {}", e))?;
        crate::utils::log(&format!("MERGE: Removed empty folder: {}", folder.display()));
    } else {
        crate::utils::log(&format!("MERGE: Folder not empty, keeping: {}", folder.display()));
    }
    
    Ok(())
}

// ============================================================================
// Configuration
// ============================================================================

// From src/core/config.rs
pub struct PopupSettings {
    pub orphans_path: Option<String>,
    // ... other fields
}

// The orphans_path is configured in config.yaml as:
// popup_settings:
//   orphans_path: "~/ob/kmr/MY/orphans"
//
// This directory contains folders that were created but don't have a proper home yet.
// The orphan processing system looks for matching anchors in the main vault and merges them.

// ============================================================================
// How the System Works
// ============================================================================

// 1. During rescanning (triggered by --rescan or automatically), the system checks
//    for orphan folders in the configured orphans_path
//
// 2. For each orphan folder, it looks for a matching anchor file (folder_name/folder_name.md)
//
// 3. It searches the main vault (scan_roots) for a corresponding anchor, handling:
//    - Standard anchors: FolderName/FolderName.md
//    - Alias anchors: DifferentFolder/FolderName.md (where FolderName.md is an alias)
//
// 4. When a match is found, it:
//    - Merges the markdown content (appending with separator)
//    - Migrates all other files from the orphan to the target
//    - Cleans up the empty orphan folder
//
// 5. After merging, it triggers a rescan to pick up the changes

// ============================================================================
// Known Issues with Current Implementation
// ============================================================================

// 1. Deep recursion in large directory trees (.app bundles are filtered but still costly)
// 2. No handling of nested orphan folders
// 3. File conflicts resolved with simple counter suffix
// 4. Auto-generated anchor content detection is brittle (string matching)
// 5. No rollback on partial merge failures
// 6. Rescanning entire vault after each merge is expensive