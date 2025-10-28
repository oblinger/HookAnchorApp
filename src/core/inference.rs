//! Patch inference and auto-assignment logic
//! 
//! This module contains all the complex logic for inferring and automatically assigning
//! patches to commands based on various strategies including file paths, command patterns,
//! aliases, and folder hierarchies.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::core::{Command, Patch};
use super::commands::FLAG_USER_EDITED;

// ============================================================================
// PATCH INFERENCE - Main entry points and coordination logic
// ============================================================================
//
// This section contains the main inference functions that coordinate between
// different inference strategies and provide the public API for patch inference.

/// Main patch inference function that tries multiple strategies
///
/// Attempts to infer a patch for a command using various strategies in priority order:
/// 1. Alias target inheritance (highest priority)
/// 2. File/folder-based inference for path commands
/// 3. Progressive word matching
/// 4. Year-based prefix matching
/// 5. Similarity-based fuzzy matching (lowest priority)
pub fn infer_patch(command: &Command, patches: &HashMap<String, Patch>) -> Option<String> {
    // Don't override user-edited commands UNLESS they're in orphans or have no patch
    // If user explicitly set a real patch, respect it. But if it's orphans/empty, try to infer.
    if command.flags.contains(FLAG_USER_EDITED)
        && !command.patch.is_empty()
        && command.patch != "orphans" {
        crate::utils::detailed_log("PATCH_INFERENCE", &format!(
            "Command '{}' -> NO INFERENCE (user-edited with explicit patch '{}')",
            command.command, command.patch
        ));
        return None;
    }

    // Method 1: Alias commands inherit patch from their target (HIGHEST PRIORITY)
    if command.action == "alias" {
        if let Some(target_patch) = infer_patch_from_alias_target(command, patches) {
            // Check for self-assignment (would create a cycle)
            if command.is_anchor() && target_patch.to_lowercase() == command.command.to_lowercase() {
                crate::utils::detailed_log("PATCH_INFERENCE", &format!(
                    "Command '{}' -> NO PATCH (alias would create self-reference cycle with patch '{}')",
                    command.command, target_patch
                ));
                return None;
            }
            crate::utils::detailed_log("PATCH_INFERENCE", &format!(
                "Command '{}' -> PATCH '{}' (inherited from alias target)",
                command.command, target_patch
            ));
            return Some(target_patch);
        }
    }

    // Method 2: File/folder-based inference (HIGH PRIORITY for path-based commands)
    // Check if the command is path-based and extract folder information
    if command.is_path_based() {
        if let Some(inferred_patch) = infer_patch_from_command(command, patches) {
            // Prevent self-assignment for anchors (would break tree hierarchy)
            // Anchors DEFINE patches but should be IN their parent patch, not their own
            if command.is_anchor() && inferred_patch.to_lowercase() == command.command.to_lowercase() {
                crate::utils::detailed_log("PATCH_INFERENCE", &format!(
                    "Command '{}' -> REJECTED self-assignment '{}' (anchors must be in parent patch, not their own)",
                    command.command, inferred_patch
                ));
                // Continue to other inference methods
            } else {
                crate::utils::detailed_log("PATCH_INFERENCE", &format!(
                    "Command '{}' -> PATCH '{}' (inferred from file/folder path: '{}')",
                    command.command, inferred_patch, command.arg
                ));
                return Some(inferred_patch);
            }
        }
    }

    // Method 3: Check for progressive word matches, preferring longer matches
    let command_words = command.command.to_lowercase();
    let mut best_match = None;
    let mut best_score = 0;

    for patch_name in patches.keys() {
        let patch_lower = patch_name.to_lowercase();

        // Prevent self-assignment for anchors
        if command.is_anchor() && patch_lower == command.command.to_lowercase() {
            continue;
        }

        // Check if patch name is contained in command name (as whole words when possible)
        if command_words.contains(&patch_lower) {
            let score = patch_name.len();
            if score > best_score {
                best_match = Some(patch_name.clone());
                best_score = score;
            }
        }
    }

    if let Some(match_patch) = best_match {
        crate::utils::detailed_log("PATCH_INFERENCE", &format!(
            "Command '{}' -> PATCH '{}' (word matching: '{}' contains '{}')",
            command.command, match_patch, command.command, match_patch
        ));
        return Some(match_patch);
    }

    // Method 4: Year-based prefix matching
    if let Some(year_patch) = infer_patch_from_year_prefix(&command.command) {
        // Verify the patch actually exists
        if patches.contains_key(&year_patch.to_lowercase()) {
            crate::utils::detailed_log("PATCH_INFERENCE", &format!(
                "Command '{}' -> PATCH '{}' (year prefix matching)",
                command.command, year_patch
            ));
            return Some(year_patch);
        }
    }

    // Method 5: Similarity-based fuzzy matching (lowest priority)
    let mut best_similarity = 0.0;
    let mut best_patch = None;

    for patch_name in patches.keys() {
        // Prevent self-assignment for anchors
        if command.is_anchor() && patch_name.to_lowercase() == command.command.to_lowercase() {
            continue;
        }

        let similarity = calculate_similarity(&command.command.to_lowercase(), &patch_name.to_lowercase());
        if similarity > best_similarity && similarity >= 0.6 { // Threshold for fuzzy matching
            best_similarity = similarity;
            best_patch = Some(patch_name.clone());
        }
    }

    if let Some(fuzzy_patch) = best_patch {
        crate::utils::detailed_log("PATCH_INFERENCE", &format!(
            "Command '{}' -> PATCH '{}' (fuzzy matching: {:.2} similarity)",
            command.command, fuzzy_patch, best_similarity
        ));
        return Some(fuzzy_patch);
    }

    // No patch could be inferred
    crate::utils::detailed_log("PATCH_INFERENCE", &format!(
        "Command '{}' -> NO PATCH (no inference method succeeded)",
        command.command
    ));
    None
}

/// Run basic patch inference with degradation prevention and logging
/// This is a simplified version - the main CLI version is in commands.rs
pub fn run_basic_patch_inference(
    commands: &mut [Command],
    patches: &HashMap<String, Patch>,
    skip_degradation_check: bool,
) {
    let start_time = std::time::Instant::now();
    let mut inferred_count = 0;
    let mut skipped_count = 0;
    
    for command in commands.iter_mut() {
        // Skip commands that already have patches unless we're forcing re-inference
        if !command.patch.is_empty() && !skip_degradation_check {
            continue;
        }
        
        let old_patch = command.patch.clone();
        
        if let Some(inferred_patch) = infer_patch(command, patches) {
            // Check for degradation if we have an existing patch
            if !old_patch.is_empty() && !skip_degradation_check {
                if is_patch_degradation(&old_patch, &inferred_patch) {
                    crate::utils::detailed_log("PATCH_INFERENCE", &format!(
                        "Skipping degraded inference for '{}': '{}' -> '{}' (would be degradation)",
                        command.command, old_patch, inferred_patch
                    ));
                    skipped_count += 1;
                    continue;
                }
            }
            
            command.patch = inferred_patch.clone();
            command.update_full_line();
            inferred_count += 1;
            
            crate::utils::detailed_log("PATCH_INFERENCE", &format!(
                "Inferred patch for '{}': '{}' -> '{}'",
                command.command, old_patch, inferred_patch
            ));
        }
    }
    
    let duration = start_time.elapsed();
    crate::utils::log(&format!(
        "Patch inference completed in {:?}: {} patches inferred, {} skipped (degradation)",
        duration, inferred_count, skipped_count
    ));
}

// ============================================================================
// FILE PATH-BASED INFERENCE
// ============================================================================
//
// This section handles inferring patches based on file paths and folder structures.
// It includes logic for walking directory hierarchies and matching folder names
// to existing patch names.

/// Infer patch from command's file path or argument
fn infer_patch_from_command(command: &Command, patches: &HashMap<String, Patch>) -> Option<String> {
    // Use get_absolute_folder_path to normalize the path:
    // - For files: returns parent directory (folder containing the file)
    // - For folders: returns the folder itself
    // - For anchors: smart logic for both cases
    //
    // This gives us the folder that this command represents/is in, and we pass
    // that to inference which will check folder_map to find the patch
    let config = crate::core::data::get_config();
    if let Some(folder_path) = command.get_absolute_folder_path(&config) {
        infer_patch_from_file_path(folder_path.to_str()?, patches)
    } else if !command.arg.is_empty() {
        // Fallback to raw arg if get_absolute_folder_path returns None
        infer_patch_from_file_path(&command.arg, patches)
    } else {
        None
    }
}

/// Infer patch from file path, excluding a specific command to prevent self-assignment
fn infer_patch_from_file_path_with_exclusion(file_path: &str, patches: &HashMap<String, Patch>, exclude_command: &str) -> Option<String> {
    // Build folder map from current commands, excluding the specified command
    let (sys_data, _) = crate::core::get_sys_data();
    let mut filtered_commands = sys_data.commands;
    filtered_commands.retain(|cmd| cmd.command.to_lowercase() != exclude_command.to_lowercase());
    
    let folder_map = build_folder_to_patch_map(&filtered_commands);
    
    // Use simple inference with the filtered map
    if let Some(patch) = infer_patch_simple(file_path, &folder_map) {
        // Double-check that the patch exists in our patches map
        if patches.contains_key(&patch.to_lowercase()) {
            return Some(patch);
        }
    }
    
    // Fallback to walking up the directory hierarchy
    let path = Path::new(file_path);
    // FIXED: For folders, start with parent. For files, start with containing directory.
    // Both cases should check the PARENT, not the item itself.
    let mut current_dir = path.parent();

    while let Some(dir) = current_dir {
        if let Some(dir_name) = dir.file_name() {
            if let Some(dir_str) = dir_name.to_str() {
                // Look for a patch that matches this directory name
                for patch_name in patches.keys() {
                    // Exclude the specified command
                    if patch_name.to_lowercase() == exclude_command.to_lowercase() {
                        continue;
                    }

                    if patch_name.to_lowercase() == dir_str.to_lowercase() {
                        crate::utils::detailed_log("PATCH_INFERENCE", &format!(
                            "Found patch '{}' from directory name '{}' for path '{}'",
                            patch_name, dir_str, file_path
                        ));
                        return Some(patch_name.clone());
                    }
                }
            }
        }
        current_dir = dir.parent();
    }

    None
}

/// Core file path-based patch inference
fn infer_patch_from_file_path(file_path: &str, patches: &HashMap<String, Patch>) -> Option<String> {
    // Build folder map from current commands
    let (sys_data, _) = crate::core::get_sys_data();
    let folder_map = build_folder_to_patch_map(&sys_data.commands);

    // Use simple inference first
    if let Some(patch) = infer_patch_simple(file_path, &folder_map) {
        // Double-check that the patch exists in our patches map
        if patches.contains_key(&patch.to_lowercase()) {
            return Some(patch);
        }
    }

    // Fallback to walking up the directory hierarchy
    let path = Path::new(file_path);
    // At this point, file_path should already be a directory (from infer_patch_from_command)
    // Start checking from this directory
    let mut current_dir = Some(path);
    
    while let Some(dir) = current_dir {
        if let Some(dir_name) = dir.file_name() {
            if let Some(dir_str) = dir_name.to_str() {
                // Look for a patch that matches this directory name
                for patch_name in patches.keys() {
                    if patch_name.to_lowercase() == dir_str.to_lowercase() {
                        crate::utils::detailed_log("PATCH_INFERENCE", &format!(
                            "Found patch '{}' from directory name '{}' for path '{}'", 
                            patch_name, dir_str, file_path
                        ));
                        return Some(patch_name.clone());
                    }
                }
            }
        }
        current_dir = dir.parent();
    }
    
    None
}

/// Build a hashmap of absolute folder paths to patch names from anchor commands
/// This creates the folder hierarchy that patch inference will use
pub fn build_folder_to_patch_map(commands: &[Command]) -> HashMap<PathBuf, String> {
    let mut folder_map = HashMap::new();
    let config = crate::core::data::get_config();

    // First pass: Add all anchor commands to the map
    for cmd in commands {
        if cmd.is_anchor() && !cmd.arg.is_empty() {
            // Use the proper accessor that handles both file and folder anchors correctly
            if let Some(folder_path) = cmd.get_absolute_folder_path(&config) {
                // Canonicalize to handle symlinks and relative paths
                if let Ok(canonical_folder) = folder_path.canonicalize() {
                    // Map this folder to the anchor's command name (which becomes the patch for its contents)
                    folder_map.insert(canonical_folder, cmd.command.clone());

                    crate::utils::detailed_log("PATCH_MAP", &format!(
                        "Folder '{}' -> patch '{}' (using proper accessor)",
                        folder_path.display(), cmd.command
                    ));
                }
            }
        }
    }

    folder_map
}

/// Simple patch inference: walk up the folder hierarchy until we find a mapped folder
pub fn infer_patch_simple(file_path: &str, folder_map: &HashMap<PathBuf, String>) -> Option<String> {
    // Skip if not a file path
    if file_path.is_empty() || file_path.starts_with("http") || !file_path.contains('/') {
        return None;
    }

    let path = Path::new(file_path);

    // Determine starting point based on what we're looking at:
    // - Anchor files: start from grandparent to avoid self-reference
    // - Regular files: start from parent directory (the folder containing them)
    // - Directories: start from the directory itself (folder_map has this folder's patch)
    let mut current = if crate::utils::is_anchor_file(path) {
        // Anchor files: start from grandparent to avoid self-reference
        path.parent().and_then(|p| p.parent())
    } else if path.is_dir() {
        // Directories: check the directory itself in folder_map
        Some(path)
    } else {
        // Regular files: check their containing directory
        path.parent()
    };

    // Walk up the directory tree checking folder_map at each level
    while let Some(dir) = current {
        if let Ok(canonical) = dir.canonicalize() {
            if let Some(patch) = folder_map.get(&canonical) {
                crate::utils::detailed_log("FOLDER_MAPPING", &format!(
                    "File '{}' -> mapped folder '{}' -> patch '{}'",
                    file_path, dir.display(), patch
                ));
                return Some(patch.clone());
            }
        }
        current = dir.parent();
    }

    None
}

// ============================================================================
// HIERARCHY-BASED INFERENCE
// ============================================================================
//
// This section handles patch inference based on folder hierarchies,
// particularly for anchor commands that need to find their parent patch
// without creating self-reference cycles.

/// Infer patch for anchor commands by looking at their folder hierarchy
fn infer_patch_from_hierarchy_for_anchor(command: &Command, patches: &HashMap<String, Patch>) -> Option<String> {
    if command.is_anchor() && !command.arg.is_empty() {
        let path = Path::new(&command.arg);
        if let Some(parent_dir) = path.parent() {
            return infer_patch_from_hierarchy(parent_dir.parent()?, patches);
        }
    }
    None
}

/// Walk up directory hierarchy to find matching patch
fn infer_patch_from_hierarchy(dir: &Path, patches: &HashMap<String, Patch>) -> Option<String> {
    let mut current = Some(dir);
    
    while let Some(path) = current {
        if let Some(dir_name) = path.file_name() {
            if let Some(dir_str) = dir_name.to_str() {
                // Look for exact match
                for patch_name in patches.keys() {
                    if patch_name.to_lowercase() == dir_str.to_lowercase() {
                        crate::utils::detailed_log("HIERARCHY_INFERENCE", &format!(
                            "Found hierarchy patch '{}' from directory '{}'", 
                            patch_name, dir_str
                        ));
                        return Some(patch_name.clone());
                    }
                }
            }
        }
        current = path.parent();
    }
    
    None
}

// ============================================================================
// ALIAS AND PATTERN-BASED INFERENCE  
// ============================================================================
//
// This section handles inference for alias commands and pattern-based
// matching including year prefixes and command name patterns.

/// Infer patch from alias target command
fn infer_patch_from_alias_target(command: &Command, patches: &HashMap<String, Patch>) -> Option<String> {
    if command.action == "alias" && !command.arg.is_empty() {
        // Get all commands from singleton to find the target
        let (sys_data, _) = crate::core::get_sys_data();

        // Find the target command
        for target_cmd in &sys_data.commands {
            if target_cmd.command == command.arg {
                if !target_cmd.patch.is_empty() {
                    crate::utils::detailed_log("ALIAS_INFERENCE", &format!(
                        "Alias '{}' inherits patch '{}' from target '{}'",
                        command.command, target_cmd.patch, target_cmd.command
                    ));
                    return Some(target_cmd.patch.clone());
                }
                // If target doesn't have a patch, try to infer one for it
                if let Some(inferred_patch) = infer_patch(target_cmd, patches) {
                    crate::utils::detailed_log("ALIAS_INFERENCE", &format!(
                        "Alias '{}' inherits inferred patch '{}' from target '{}'",
                        command.command, inferred_patch, target_cmd.command
                    ));
                    return Some(inferred_patch);
                }
                break;
            }
        }
    }
    None
}

/// Infer patch based on year prefix in command name (e.g., "2024 Project" -> "2024")
fn infer_patch_from_year_prefix(command_name: &str) -> Option<String> {
    // Look for 4-digit year at the beginning
    if command_name.len() >= 4 {
        let potential_year = &command_name[0..4];
        if let Ok(year) = potential_year.parse::<u32>() {
            if year >= 2000 && year <= 2030 {
                return Some(potential_year.to_string());
            }
        }
    }
    
    // Look for year after common prefixes
    let prefixes = ["proj ", "project ", "work "];
    for prefix in &prefixes {
        if let Some(rest) = command_name.to_lowercase().strip_prefix(prefix) {
            if rest.len() >= 4 {
                let potential_year = &rest[0..4];
                if let Ok(year) = potential_year.parse::<u32>() {
                    if year >= 2000 && year <= 2030 {
                        return Some(potential_year.to_string());
                    }
                }
            }
        }
    }
    
    None
}

// ============================================================================
// AUTO-ASSIGNMENT AND BATCH OPERATIONS
// ============================================================================
//
// This section handles automatic patch assignment for large batches of commands
// and provides utilities for managing patch assignments across the entire
// command database.

/// Automatically assign patches to commands that don't have them
pub fn auto_assign_patches(commands: &mut Vec<Command>) {
    let patches = crate::core::commands::create_patches_hashmap(commands);
    let start_time = std::time::Instant::now();
    let mut assigned_count = 0;
    
    for command in commands.iter_mut() {
        // Skip commands that already have patches
        if !command.patch.is_empty() {
            continue;
        }
        
        if let Some(patch) = infer_patch(command, &patches) {
            command.patch = patch.clone();
            command.update_full_line();
            assigned_count += 1;
            
            crate::utils::detailed_log("AUTO_ASSIGN", &format!(
                "Auto-assigned patch '{}' to command '{}'", 
                patch, command.command
            ));
        }
    }
    
    let duration = start_time.elapsed();
    if assigned_count > 0 {
        crate::utils::log(&format!(
            "Auto-assigned {} patches in {:?}", 
            assigned_count, duration
        ));
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================
//
// This section contains helper functions for patch inference including
// similarity calculations, degradation detection, and other utilities.

/// Check if a patch change would be a degradation (less specific/useful)
fn is_patch_degradation(current_patch: &str, inferred_patch: &str) -> bool {
    // If current patch is empty, any inference is an improvement
    if current_patch.is_empty() {
        return false;
    }
    
    // If inferred patch is empty, that's definitely a degradation
    if inferred_patch.is_empty() {
        return true;
    }
    
    // Check if current patch contains a year but inferred doesn't
    let current_has_year = current_patch.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .any(|window| {
            let year_str: String = window.iter().collect();
            if let Ok(year) = year_str.parse::<u32>() {
                year >= 2000 && year <= 2030
            } else {
                false
            }
        });
    
    let inferred_has_year = inferred_patch.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .any(|window| {
            let year_str: String = window.iter().collect();
            if let Ok(year) = year_str.parse::<u32>() {
                year >= 2000 && year <= 2030
            } else {
                false
            }
        });
    
    if current_has_year && !inferred_has_year {
        return true;
    }
    
    // If current patch is significantly longer and more specific, prefer it
    if current_patch.len() > inferred_patch.len() + 5 {
        // Only consider it degradation if the current patch has multiple words/parts
        if current_patch.contains(' ') || current_patch.contains('-') || current_patch.contains('_') {
            return true;
        }
    }
    
    // Not a degradation
    false
}

/// Calculate similarity between two strings (simple implementation)
fn calculate_similarity(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }
    
    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }
    
    // Simple substring matching
    let longer = if s1.len() > s2.len() { s1 } else { s2 };
    let shorter = if s1.len() <= s2.len() { s1 } else { s2 };
    
    if longer.contains(shorter) {
        return shorter.len() as f64 / longer.len() as f64;
    }
    
    // Count common characters (very basic)
    let mut common = 0;
    for c in shorter.chars() {
        if longer.contains(c) {
            common += 1;
        }
    }
    
    common as f64 / longer.len() as f64
}

// ============================================================================
// PATCH RESOLUTION - High-level orchestration
// ============================================================================

/// Result of patch resolution operation
pub struct PatchResolutionResult {
    pub patches: HashMap<String, Patch>,
    pub changes_made: bool,
}

/// Validates and repairs the patch structure to ensure data integrity before saving
///
/// This is the main entry point for patch validation and repair. It guarantees that
/// after this function runs, all commands have valid patches and the patch hierarchy
/// is sound (no cycles, all anchors have parents, all patches exist).
///
/// Phases:
/// 1. Build patches hashmap from anchor commands
/// 2. Detect and break cycles in patch hierarchy
/// 3. Create virtual anchors for commands with undefined patches
/// 4. Run patch inference to assign patches to commands without them
/// 5. Set remaining empty-patch anchors to "orphans" (inference failed)
/// 6. Normalize patch case to match anchor commands
///
/// Returns:
/// - patches: The complete patches hashmap
/// - changes_made: Whether any commands were modified (to determine if save is needed)
pub fn validate_and_repair_patches(
    commands: &mut Vec<Command>,
    verbose: bool
) -> PatchResolutionResult {
    // Phase 1: Create patches hashmap from anchor commands
    if verbose {
        crate::utils::log("🏷️  Phase 1: Creating patches hashmap...");
    }
    let mut patches = crate::core::commands::create_patches_hashmap(commands);
    if verbose {
        crate::utils::log(&format!("   Found {} patches from anchor commands", patches.len()));
    }

    // Phase 2: Detect and break cycles in patch hierarchy
    if verbose {
        crate::utils::log("🔄 Phase 2: Detecting cycles in patch hierarchy...");
    }
    let mut cycles_fixed = 0;
    let mut cycles_detected = Vec::new(); // Track which patches were in cycles

    // For each patch, walk up the parent tree and detect cycles
    for (patch_name, patch) in &patches {
        let mut visited = std::collections::HashSet::new();
        let mut current_patch = patch_name.clone();

        // Walk up the parent tree
        loop {
            // Add current patch to visited set
            if visited.contains(&current_patch) {
                // Cycle detected! Current patch is already in our path
                if verbose {
                    crate::utils::log(&format!("   ⚠️  Cycle detected at patch '{}' (visited={:?})", current_patch, visited));
                }
                cycles_detected.push(current_patch.clone());
                break;
            }
            visited.insert(current_patch.clone());

            // Get parent of current patch
            if let Some(parent_patch) = patches.get(&current_patch).and_then(|p| p.parent_patch_name()) {
                let parent_lower = parent_patch.to_lowercase();

                // If we reached orphans, we're done (no cycle)
                if parent_lower == "orphans" {
                    break;
                }

                // Move to parent
                current_patch = parent_lower;
            } else {
                // No parent - this means the patch has no parent or parent doesn't exist
                break;
            }
        }
    }

    // Fix cycles by setting the cycle-creating patch's parent to "orphans"
    if !cycles_detected.is_empty() {
        if verbose {
            crate::utils::log(&format!("   Found {} cycles to fix", cycles_detected.len()));
        }

        for cycle_patch_name in cycles_detected {
            // Find the anchor command for this patch and set its parent to orphans
            for cmd in commands.iter_mut() {
                if cmd.is_anchor() && cmd.command.to_lowercase() == cycle_patch_name {
                    if verbose {
                        crate::utils::log(&format!("   Breaking cycle: Setting parent of '{}' to 'orphans' (was '{}')", cmd.command, cmd.patch));
                    }
                    cmd.patch = "orphans".to_string();
                    cycles_fixed += 1;
                    break; // Only fix the first (primary) anchor for this patch
                }
            }
        }

        // Rebuild patches hashmap after fixing cycles
        if cycles_fixed > 0 {
            if verbose {
                crate::utils::log(&format!("   Rebuilding patches hashmap after fixing {} cycles", cycles_fixed));
            }
            patches = crate::core::commands::create_patches_hashmap(commands);
        }
    }

    if verbose && cycles_fixed == 0 {
        crate::utils::log("   ✅ No cycles detected");
    }

    // Phase 3: Create virtual anchors for commands with undefined patches
    if verbose {
        crate::utils::log("🔧 Phase 3: Creating virtual anchors for orphaned commands...");
    }
    let mut virtual_anchors_created = 0;

    // Build set of existing patch names (lowercase for comparison)
    let existing_patches: std::collections::HashSet<String> = patches.keys().cloned().collect();

    // Find all patches that are referenced by commands but don't exist
    let mut missing_patches = std::collections::HashSet::new();
    for cmd in commands.iter() {
        if !cmd.patch.is_empty() && cmd.patch != "orphans" {
            let patch_lower = cmd.patch.to_lowercase();
            if !existing_patches.contains(&patch_lower) {
                missing_patches.insert(cmd.patch.clone()); // Use original case
            }
        }
    }

    // Create virtual anchor for each missing patch
    for patch_name in missing_patches {
        // Check if there's ANY command (anchor or not) with this name
        // We prefer to reuse existing commands rather than creating orphans
        let existing_cmd_idx = commands.iter().position(|cmd|
            cmd.command.eq_ignore_ascii_case(&patch_name)
        );

        if let Some(idx) = existing_cmd_idx {
            // Found existing command with this name
            if commands[idx].is_anchor() {
                // Already an anchor - nothing to do
                if verbose {
                    crate::utils::log(&format!("   Skipping '{}' - already has anchor flag", patch_name));
                }
                continue;
            } else {
                // Not an anchor yet - set the anchor flag on existing command
                commands[idx].set_anchor(true);
                if verbose {
                    crate::utils::log(&format!("   Set anchor flag on existing command '{}' (patch: {})", patch_name, commands[idx].patch));
                }

                // Update the patches hashmap to include this command as an anchor
                let patch_lower = commands[idx].patch.to_lowercase();
                if let Some(patch) = patches.get_mut(&patch_lower) {
                    patch.anchor_commands.push(commands[idx].clone());
                }

                virtual_anchors_created += 1;
                continue;
            }
        }

        // No existing command found - create new orphan anchor
        if verbose {
            crate::utils::log(&format!("   Creating virtual anchor for undefined patch: '{}'", patch_name));
        }

        let config = crate::core::data::get_config();
        let virtual_anchor = Command {
            command: patch_name.clone(),
            action: String::new(),  // Virtual anchor - blank action (non-executable)
            arg: String::new(), // Virtual anchor - no file
            patch: "orphans".to_string(),
            flags: "A".to_string(), // Anchor flag (no U flag - system-generated)
            other_params: None,
            last_update: 0,
            file_size: None,
        };

        // Add to commands list
        commands.push(virtual_anchor.clone());
        crate::utils::log(&format!("PATCH: Added virtual anchor '{}' to commands (patch parent: {})", patch_name, virtual_anchor.patch));

        // Add to patches hashmap
        patches.insert(patch_name.to_lowercase(), Patch {
            name: patch_name.clone(),
            anchor_commands: vec![virtual_anchor],
            include_commands: Vec::new(),
            history_file: None,
        });

        virtual_anchors_created += 1;
    }

    if verbose {
        if virtual_anchors_created > 0 {
            crate::utils::log(&format!("   Created {} virtual anchors for orphaned commands", virtual_anchors_created));
        } else {
            crate::utils::log("   ✅ All command patches are defined");
        }
    }

    // Phase 4: Run patch inference for commands without patches
    if verbose {
        crate::utils::log("🧩 Phase 4: Running patch inference for commands without patches...");
    }
    let (patches_assigned, new_patches_to_add) = crate::core::commands::run_patch_inference(
        commands,
        &patches,
        true,  // apply_changes = true (normal operation)
        verbose, // print_to_stdout = verbose
        false  // overwrite_patch = false (only fill empty patches)
    );
    if verbose {
        crate::utils::log(&format!("   Assigned patches to {} commands", patches_assigned));
        crate::utils::log(&format!("   Need to add {} new patches", new_patches_to_add.len()));
    }

    // Step 4: Add new patches to hashmap
    for patch_name in new_patches_to_add {
        let patch_key = patch_name.to_lowercase();
        if !patches.contains_key(&patch_key) {
            // Find the first command whose name matches this patch name (case-insensitive)
            let matching_command = commands.iter().find(|cmd| {
                cmd.command.to_lowercase() == patch_key
            });

            patches.insert(patch_key, Patch {
                name: patch_name.clone(), // Store original case
                anchor_commands: if let Some(cmd) = matching_command.cloned() { vec![cmd] } else { vec![] },
                include_commands: Vec::new(),
                history_file: None,
            });
        }
    }

    // Phase 5: Set remaining empty-patch anchors to "orphans" (inference failed for these)
    if verbose {
        crate::utils::log("🔍 Phase 5: Setting remaining empty-patch anchors to 'orphans'...");
    }
    let mut orphaned_anchors_fixed = 0;
    for cmd in commands.iter_mut() {
        if cmd.is_anchor() && cmd.patch.is_empty() {
            if verbose {
                crate::utils::log(&format!("   Setting parent for anchor '{}' to 'orphans' (inference failed)", cmd.command));
            }
            cmd.patch = "orphans".to_string();
            orphaned_anchors_fixed += 1;
        }
    }
    if verbose && orphaned_anchors_fixed > 0 {
        crate::utils::log(&format!("   Set {} anchor commands to 'orphans' (inference failed)", orphaned_anchors_fixed));
    }

    // Phase 6: Normalize patch case to match anchor commands
    if verbose {
        crate::utils::log("🔤 Phase 6: Normalizing patch case to match anchor commands...");
    }
    let normalized_patches = crate::core::commands::normalize_patch_case(commands, &patches);
    if verbose {
        if normalized_patches > 0 {
            crate::utils::log(&format!("   Normalized case for {} patch references", normalized_patches));
        } else {
            crate::utils::log("   No case normalization needed");
        }
    }

    let changes_made = orphaned_anchors_fixed > 0 || cycles_fixed > 0 || patches_assigned > 0 || virtual_anchors_created > 0 || normalized_patches > 0;

    if verbose {
        if changes_made {
            crate::utils::log(&format!("   ✅ Patch resolution complete with {} changes",
                orphaned_anchors_fixed + cycles_fixed + patches_assigned + virtual_anchors_created + normalized_patches));
        } else {
            crate::utils::log("   ⏭️  No changes needed");
        }
    }

    PatchResolutionResult {
        patches,
        changes_made,
    }
}