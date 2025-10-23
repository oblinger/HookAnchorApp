/// Protected version with loop detection and depth limiting
/// NEW TWO-PASS ALGORITHM:
/// 1. Read directory once into Vec<PathBuf>
/// 2. Pass 1: Find and create anchor files first, add to folder_map
/// 3. Pass 2: Process all files (assign patches using folder_map) and recurse into subdirectories
fn scan_directory_with_root_protected(dir: &Path, vault_root: &Path, commands: &mut Vec<Command>, existing_commands: &mut HashSet<String>, handled_files: &mut HashSet<String>, found_folders: &mut Vec<PathBuf>, folder_map: &mut std::collections::HashMap<PathBuf, String>, config: &Config, visited: &mut HashSet<PathBuf>, depth: usize) {
    // Prevent infinite loops with depth limit
    if depth > 20 {
        return;
    }

    // Get canonical path to detect symbolic link loops
    let canonical_dir = match dir.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            return;
        }
    };

    // Check if we've already visited this directory (prevents infinite loops)
    if visited.contains(&canonical_dir) {
        return;
    }
    visited.insert(canonical_dir.clone());

    // NEW TWO-PASS ALGORITHM
    // Step 1: Read directory once into Vec<PathBuf>
    let entries: Vec<PathBuf> = if let Ok(read_dir) = fs::read_dir(dir) {
        read_dir
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect()
    } else {
        return; // Can't read directory
    };

    // Step 2: PASS 1 - Find and create anchor files FIRST in this directory
    for path in &entries {
        // Skip hidden files
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                if name_str.starts_with('.') {
                    continue;
                }
            }
        }

        // Only process files in Pass 1 (directories are handled in Pass 2)
        if !path.is_file() {
            continue;
        }

        // Check if this is an anchor file (filename matches parent folder name)
        if crate::utils::is_anchor_file(path) {
            // Create anchor command
            if let Some(mut anchor_cmd) = process_markdown_with_root(path, vault_root, commands, existing_commands, handled_files, folder_map) {
                // The process_markdown_with_root will have set 'a' flag for anchors
                if !handled_files.contains(&anchor_cmd.arg) {
                    // Infer patch from folder_map BEFORE adding to commands
                    anchor_cmd.patch = infer_patch_from_folder_map(path, folder_map);

                    crate::utils::log(&format!("🏷️  PASS1 Creating anchor: '{}' patch: '{}'", anchor_cmd.command, anchor_cmd.patch));

                    // Add this anchor to the folder_map for its folder
                    if let Some(parent) = path.parent() {
                        if let Ok(canonical_parent) = parent.canonicalize() {
                            folder_map.insert(canonical_parent.clone(), anchor_cmd.command.clone());
                            crate::utils::log(&format!("   Added to folder_map: '{}' -> '{}'", canonical_parent.display(), anchor_cmd.command));
                        }
                    }

                    existing_commands.insert(anchor_cmd.command.to_lowercase());
                    handled_files.insert(anchor_cmd.arg.clone());
                    commands.push(anchor_cmd);
                }
            }
        }
    }

    // Step 3: PASS 2 - Process all entries (files get patches, directories recurse)
    for path in &entries {
        // Skip hidden files and directories
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                if name_str.starts_with('.') {
                    continue;
                }

                // Skip directories based on config patterns
                if path.is_dir() && should_skip_directory(&name_str, config) {
                    continue;
                }
            } else {
                crate::utils::detailed_log("SCANNER", &format!("Skipping non-UTF8 filename: {:?}", path));
                continue;
            }
        }

        // Check metadata for symlink detection
        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        if path.is_dir() {
            // Skip symlinked directories to prevent infinite loops
            if metadata.file_type().is_symlink() {
                continue;
            }

            // Check if this is a .app bundle
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.ends_with(".app") {
                        // Create APP command with patch from folder_map
                        if let Some(mut app_cmd) = process_app_bundle(path, existing_commands, folder_map) {
                            if !handled_files.contains(&app_cmd.arg) {
                                // Infer patch from folder_map
                                app_cmd.patch = infer_patch_from_folder_map(path, folder_map);

                                crate::utils::log(&format!("📱 Found app: '{}' patch: '{}'", app_cmd.command, app_cmd.patch));

                                existing_commands.insert(app_cmd.command.to_lowercase());
                                handled_files.insert(app_cmd.arg.clone());
                                commands.push(app_cmd);
                            }
                        }
                        // Don't scan inside .app bundles
                        continue;
                    }
                }
            }

            // Recursively scan subdirectories (depth-first)
            scan_directory_with_root_protected(path, vault_root, commands, existing_commands, handled_files, found_folders, folder_map, config, visited, depth + 1);
        } else {
            // Process files (skip anchors - already processed in Pass 1)
            if crate::utils::is_anchor_file(path) {
                continue; // Already created in Pass 1
            }

            // Try to process as markdown file
            if let Some(mut cmd) = process_markdown_with_root(path, vault_root, commands, existing_commands, handled_files, folder_map) {
                if !handled_files.contains(&cmd.arg) {
                    // Infer patch from folder_map
                    cmd.patch = infer_patch_from_folder_map(path, folder_map);

                    crate::utils::log(&format!("📄 Found markdown: '{}' patch: '{}'", cmd.command, cmd.patch));

                    existing_commands.insert(cmd.command.to_lowercase());
                    handled_files.insert(cmd.arg.clone());
                    commands.push(cmd);
                }
            }
            // Try to process as DOC file
            else if let Some(mut cmd) = process_doc_file(path, existing_commands, folder_map, config) {
                if !handled_files.contains(&cmd.arg) {
                    // Infer patch from folder_map
                    cmd.patch = infer_patch_from_folder_map(path, folder_map);

                    crate::utils::log(&format!("📄 Found doc: '{}' patch: '{}'", cmd.command, cmd.patch));

                    existing_commands.insert(cmd.command.to_lowercase());
                    handled_files.insert(cmd.arg.clone());
                    commands.push(cmd);
                }
            }
        }
    }
}

/// Infer patch for a file by walking up its directory tree and looking in folder_map
fn infer_patch_from_folder_map(file_path: &Path, folder_map: &std::collections::HashMap<PathBuf, String>) -> String {
    let mut current = file_path.parent();

    while let Some(dir) = current {
        if let Ok(canonical) = dir.canonicalize() {
            if let Some(patch) = folder_map.get(&canonical) {
                crate::utils::detailed_log("PATCH_INFER", &format!("Inferred patch '{}' for '{}' from folder '{}'",
                    patch, file_path.display(), canonical.display()));
                return patch.clone();
            }
        }
        current = dir.parent();
    }

    // No parent folder found in map - assign to orphans
    crate::utils::detailed_log("PATCH_INFER", &format!("No folder match for '{}' - assigning to orphans", file_path.display()));
    "orphans".to_string()
}
