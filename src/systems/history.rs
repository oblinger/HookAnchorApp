//! History tracking module
//!
//! This module maintains markdown history files that track changes to markdown files in the vault.
//! History files track both new files and new entries (H1/H2 headings) from markdown files within their scope.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

use crate::core::sys_data::SysData;
use crate::core::commands::Patch;
use crate::core::inference::{build_folder_to_patch_map, infer_patch_simple};
use crate::core::commands::get_patch_path;

/// Cache tracking all markdown files and their modification state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryCache {
    /// Map from file path to file metadata
    pub files: HashMap<PathBuf, FileMetadata>,
}

/// Metadata for a single file in the history cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Current file size in bytes
    pub size: u64,

    /// Last modification time (Unix timestamp)
    pub mtime: i64,

    /// Size when last whole-file history entry was added
    /// Used to track growth for history_increment_size threshold
    pub last_history_size: u64,

    /// Set of heading texts seen in the Log section
    /// Used to detect new log entries
    pub seen_headings: HashSet<String>,
}

/// Represents a single entry to be added to history files
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryEntry {
    /// Date of the entry (from file mtime or H tag)
    pub date: String,  // Format: YYYY-MM-DD

    /// Wiki link to the file or heading
    pub link: String,  // Format: [[filename]] or [[filename#heading]]

    /// Optional additional text after the link
    pub text: Option<String>,
}

impl HistoryCache {
    pub fn new() -> Self {
        HistoryCache {
            files: HashMap::new(),
        }
    }
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

/// Load history cache from disk
fn load_history_cache() -> HistoryCache {
    let config_dir = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".config/hookanchor"))
        .unwrap_or_else(|_| PathBuf::from(".config/hookanchor"));

    let cache_path = config_dir.join("history_cache.json");

    if cache_path.exists() {
        if let Ok(contents) = fs::read_to_string(&cache_path) {
            if let Ok(cache) = serde_json::from_str(&contents) {
                return cache;
            }
        }
    }

    HistoryCache::new()
}

/// Save history cache to disk
fn save_history_cache(cache: &HistoryCache) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".config/hookanchor"))
        .unwrap_or_else(|_| PathBuf::from(".config/hookanchor"));

    fs::create_dir_all(&config_dir)?;

    let cache_path = config_dir.join("history_cache.json");
    let contents = serde_json::to_string_pretty(cache)?;
    fs::write(&cache_path, contents)?;

    Ok(())
}

/// Scan all configured file roots for markdown files
fn scan_markdown_files(sys_data: &SysData) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let file_roots = sys_data.config.popup_settings.file_roots
        .as_ref()
        .ok_or("file_roots not configured")?;

    let skip_patterns = sys_data.config.popup_settings.skip_directory_patterns
        .as_ref()
        .cloned()
        .unwrap_or_default();

    let mut markdown_files = Vec::new();

    crate::utils::log(&format!("HISTORY: Scanning {} file roots", file_roots.len()));
    for (idx, root) in file_roots.iter().enumerate() {
        let root_path = expand_home(root);
        let root_path = PathBuf::from(root_path);

        if !root_path.exists() {
            crate::utils::log(&format!("HISTORY: Skipping non-existent root {}: {}", idx + 1, root));
            continue;
        }

        crate::utils::log(&format!("HISTORY: Scanning root {}/{}: {}", idx + 1, file_roots.len(), root));
        let before_count = markdown_files.len();
        scan_directory(&root_path, &skip_patterns, &mut markdown_files)?;
        let found = markdown_files.len() - before_count;
        crate::utils::log(&format!("HISTORY: Found {} files in root {}", found, root));
    }

    Ok(markdown_files)
}

/// Recursively scan a directory for markdown files
fn scan_directory(
    dir: &Path,
    skip_patterns: &[String],
    markdown_files: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !dir.is_dir() {
        return Ok(());
    }

    // Skip hidden directories and .app bundles
    if let Some(name) = dir.file_name() {
        let name_str = name.to_string_lossy();
        if name_str.starts_with('.') || name_str.ends_with(".app") {
            return Ok(());
        }
    }

    // Check skip patterns (simple glob matching)
    let dir_str = dir.to_string_lossy();
    for pattern in skip_patterns {
        let pattern_lower = pattern.to_lowercase();
        let dir_lower = dir_str.to_lowercase();

        // Simple glob pattern matching
        if pattern.contains('*') {
            if pattern_lower.starts_with('*') && pattern_lower.ends_with('*') {
                let inner = &pattern_lower[1..pattern_lower.len()-1];
                if dir_lower.contains(inner) {
                    return Ok(());
                }
            } else {
                let cleaned = pattern_lower.replace('*', "");
                if dir_lower.contains(&cleaned) {
                    return Ok(());
                }
            }
        } else if dir_str.eq_ignore_ascii_case(pattern) {
            return Ok(());
        }
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            scan_directory(&path, skip_patterns, markdown_files)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    markdown_files.push(path);
                }
            }
        }
    }

    Ok(())
}

/// Check if a file is new (not in cache)
fn detect_new_file(file_path: &Path, cache: &HistoryCache) -> bool {
    !cache.files.contains_key(file_path)
}

/// Check if a file has grown by more than the threshold since last whole-file entry
fn detect_file_growth(
    file_path: &Path,
    cache: &HistoryCache,
    threshold: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let metadata = fs::metadata(file_path)?;
    let current_size = metadata.len();

    if let Some(file_meta) = cache.files.get(file_path) {
        let growth = if current_size > file_meta.last_history_size {
            current_size - file_meta.last_history_size
        } else {
            0
        };

        Ok(growth >= threshold)
    } else {
        Ok(false) // New files are handled by detect_new_file
    }
}

/// Check if a file has been modified since last scan (for Log section parsing)
fn detect_file_modification(
    file_path: &Path,
    cache: &HistoryCache,
) -> Result<bool, Box<dyn std::error::Error>> {
    let metadata = fs::metadata(file_path)?;
    let current_mtime = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64;

    if let Some(file_meta) = cache.files.get(file_path) {
        Ok(current_mtime != file_meta.mtime)
    } else {
        Ok(true) // New files are considered modified
    }
}

/// Parse a markdown file's Log section for new entries
fn parse_log_section(
    file_path: &Path,
    sys_data: &SysData,
    cache: &HistoryCache,
) -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
    // Log large file reads
    if let Ok(metadata) = fs::metadata(file_path) {
        if metadata.len() > 1_000_000 {  // Log if file > 1MB
            crate::utils::log(&format!("HISTORY: Reading large file ({} bytes): {:?}",
                metadata.len(), file_path.file_name().unwrap_or_default()));
        }
    }

    let contents = fs::read_to_string(file_path)?;

    // Get config settings for history
    let log_indicator = sys_data.config.popup_settings.history_settings
        .as_ref()
        .and_then(|h| h.anchor_log_indicator.as_deref())
        .unwrap_or("# Log");

    let anchor_levels = sys_data.config.popup_settings.history_settings
        .as_ref()
        .and_then(|h| h.anchor_levels.as_ref())
        .cloned()
        .unwrap_or_else(|| vec![1, 2]);

    // Find the Log section - must be at start of a line, alone (only whitespace after)
    let log_section_start = contents.lines().enumerate().find_map(|(line_num, line)| {
        let trimmed = line.trim();
        if trimmed == log_indicator {
            // Found it! Calculate the byte position
            contents.lines().take(line_num).map(|l| l.len() + 1).sum::<usize>()
                .into()
        } else {
            None
        }
    });

    if log_section_start.is_none() {
        return Ok(Vec::new());
    }

    // Skip past the log indicator line itself - only parse headings AFTER "# Log"
    let log_start_pos = log_section_start.unwrap();
    let after_indicator = &contents[log_start_pos..];
    let first_newline = after_indicator.find('\n').unwrap_or(0);
    let log_section = &contents[log_start_pos + first_newline..];

    // Get previously seen headings from cache
    let seen_headings = cache.files.get(file_path)
        .map(|m| &m.seen_headings)
        .cloned()
        .unwrap_or_default();

    let mut entries = Vec::new();
    let metadata = fs::metadata(file_path)?;
    let mtime = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64;

    // Parse headings in the log section
    for line in log_section.lines() {
        for level in &anchor_levels {
            let prefix = "#".repeat(*level as usize);
            let prefix_with_space = format!("{} ", prefix);

            if line.starts_with(&prefix_with_space) {
                let heading_text = line[prefix_with_space.len()..].trim();

                // Skip if we've seen this heading before
                if seen_headings.contains(heading_text) {
                    continue;
                }

                // Create history entry for this new heading
                let entry = create_heading_entry(file_path, heading_text, mtime)?;
                entries.push(entry);
            }
        }
    }

    Ok(entries)
}

/// Extract date from heading text if it starts with YYYY-MM-DD format
fn parse_heading_date(heading_text: &str) -> Option<String> {
    let parts: Vec<&str> = heading_text.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    let first_part = parts[0];

    // Check if it matches YYYY-MM-DD format
    if first_part.len() == 10 && first_part.chars().nth(4) == Some('-') && first_part.chars().nth(7) == Some('-') {
        if first_part[0..4].parse::<u32>().is_ok()
            && first_part[5..7].parse::<u32>().is_ok()
            && first_part[8..10].parse::<u32>().is_ok() {
            return Some(first_part.to_string());
        }
    }

    None
}

/// Create a history entry for a file
/// - Uses creation time if file is new OR rebuild_all is set
/// - Uses modification time for file growth events
fn create_file_entry(
    file_path: &Path,
    is_new: bool,
    rebuild_all: bool,
    mtime: i64
) -> Result<HistoryEntry, Box<dyn std::error::Error>> {
    let filename = file_path.file_stem()
        .ok_or("Invalid file path")?
        .to_string_lossy()
        .to_string();

    // Use creation time for new files or when rebuilding all
    let timestamp = if is_new || rebuild_all {
        let metadata = fs::metadata(file_path)?;
        metadata.created()?.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64
    } else {
        // Use modification time for file growth
        mtime
    };

    let datetime = DateTime::from_timestamp(timestamp, 0)
        .ok_or("Invalid timestamp")?
        .with_timezone(&Local);
    let date = datetime.format("%Y-%m-%d").to_string();

    // Just [[filename]] - no need for pipe syntax when display text matches link
    Ok(HistoryEntry {
        date,
        link: format!("[[{}]]", filename),
        text: None,
    })
}

/// Create a history entry for a log heading
fn create_heading_entry(
    file_path: &Path,
    heading_text: &str,
    mtime: i64,
) -> Result<HistoryEntry, Box<dyn std::error::Error>> {
    let filename = file_path.file_stem()
        .ok_or("Invalid file path")?
        .to_string_lossy()
        .to_string();

    // Try to extract date from heading
    let (date, remaining_text) = if let Some(heading_date) = parse_heading_date(heading_text) {
        let text = heading_text[heading_date.len()..].trim();
        (heading_date, Some(text.to_string()))
    } else {
        // Use file mtime as date
        let datetime = DateTime::from_timestamp(mtime, 0)
            .ok_or("Invalid timestamp")?
            .with_timezone(&Local);
        (datetime.format("%Y-%m-%d").to_string(), Some(heading_text.to_string()))
    };

    // Create anchor-safe heading ID from the original heading text
    let heading_id = heading_text.to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();

    // Use pipe syntax: [[filename#heading|filename]] so only filename displays
    // Obsidian can find files by name, no need for full paths
    let link = format!("[[{}#{}|{}]]", filename, heading_id, filename);
    Ok(HistoryEntry {
        date,
        link,
        text: remaining_text,
    })
}

/// Collect which history files need which entries
fn collect_history_updates(
    entries: Vec<HistoryEntry>,
    patch_name: &str,
    patches: &HashMap<String, Patch>,
) -> HashMap<PathBuf, Vec<HistoryEntry>> {
    let mut updates: HashMap<PathBuf, Vec<HistoryEntry>> = HashMap::new();

    // Get the patch path (hierarchy) for this patch
    let patch_path = get_patch_path(patch_name, patches);

    // Add entries to all patches in the hierarchy that have history files
    for ancestor_patch_name in std::iter::once(patch_name.to_string()).chain(patch_path) {
        if let Some(patch) = patches.get(&ancestor_patch_name.to_lowercase()) {
            if let Some(ref history_file) = patch.history_file {
                updates.entry(history_file.clone())
                    .or_insert_with(Vec::new)
                    .extend(entries.clone());
            }
        }
    }

    updates
}

/// Format a history entry as a markdown line
fn format_history_entry(entry: &HistoryEntry) -> String {
    if let Some(ref text) = entry.text {
        format!("{} \t{}\t{}", entry.date, entry.link, text)
    } else {
        format!("{} \t{}", entry.date, entry.link)
    }
}

/// Write updated entries to history files
fn update_history_files(
    updates: HashMap<PathBuf, Vec<HistoryEntry>>,
    rebuild_all: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for (history_path, mut entries) in updates {
        // Sort entries in reverse chronological order (newest first)
        entries.sort_by(|a, b| b.date.cmp(&a.date));

        // Remove duplicates
        entries.dedup();

        // Format entries
        let new_lines: Vec<String> = entries.iter()
            .map(format_history_entry)
            .collect();

        if rebuild_all {
            // Overwrite file with new entries
            fs::write(&history_path, new_lines.join("\n") + "\n")?;
        } else {
            // Prepend new entries to existing content
            let existing_content = if history_path.exists() {
                fs::read_to_string(&history_path)?
            } else {
                String::new()
            };

            let new_content = if existing_content.is_empty() {
                new_lines.join("\n") + "\n"
            } else {
                new_lines.join("\n") + "\n" + &existing_content
            };

            fs::write(&history_path, new_content)?;
        }
    }

    Ok(())
}

/// Identify which patches have history files
fn identify_history_files(patches: &mut HashMap<String, Patch>) -> HashMap<String, PathBuf> {
    let mut history_map = HashMap::new();

    for (patch_name, patch) in patches.iter_mut() {
        // Check each anchor command for a history file
        for anchor_cmd in &patch.anchor_commands {
            // Get the directory of the anchor
            if let Some(anchor_path) = get_anchor_file_path(&anchor_cmd.arg) {
                if let Some(anchor_dir) = anchor_path.parent() {
                    // Look for files ending in "history.md" in the same directory
                    if let Ok(entries) = fs::read_dir(anchor_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(filename) = path.file_name() {
                                    let filename_str = filename.to_string_lossy().to_lowercase();
                                    if filename_str.ends_with("history.md") {
                                        // Found a history file for this patch
                                        patch.history_file = Some(path.clone());
                                        history_map.insert(patch_name.clone(), path);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    history_map
}

/// Extract file path from anchor argument (handles markdown: and other prefixes)
fn get_anchor_file_path(arg: &str) -> Option<PathBuf> {
    let path_str = if arg.starts_with("markdown:") {
        &arg[9..]
    } else if arg.starts_with("doc:") {
        &arg[4..]
    } else {
        arg
    };

    let expanded = expand_home(path_str);
    Some(PathBuf::from(expanded))
}

/// Main entry point - update all history files based on changes
pub fn update_histories(
    sys_data: &mut SysData,
    rebuild_all: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    crate::utils::log("HISTORY: Starting history update...");

    // Identify which patches have history files
    identify_history_files(&mut sys_data.patches);

    // Build folder→patch map for file lookups
    let folder_map = build_folder_to_patch_map(&sys_data.commands);

    // Load history cache
    let mut cache = load_history_cache();

    // Get history configuration
    let history_increment_size = sys_data.config.popup_settings.history_settings
        .as_ref()
        .and_then(|h| h.history_increment_size)
        .unwrap_or(1000);

    // Scan all markdown files
    crate::utils::log("HISTORY: Scanning markdown files...");
    let markdown_files = scan_markdown_files(sys_data)?;
    crate::utils::log(&format!("HISTORY: Found {} markdown files to process", markdown_files.len()));

    // Collect all history updates
    let mut all_updates: HashMap<PathBuf, Vec<HistoryEntry>> = HashMap::new();
    let mut processed_count = 0;
    let total_files = markdown_files.len();

    for file_path in markdown_files {
        processed_count += 1;

        // Log progress every 100 files
        if processed_count % 100 == 0 {
            crate::utils::log(&format!("HISTORY: Processing file {}/{}", processed_count, total_files));
        }

        // Skip history files themselves
        if let Some(filename) = file_path.file_name() {
            let filename_str = filename.to_string_lossy().to_lowercase();
            if filename_str.ends_with("history.md") {
                continue;
            }
        }

        let mut entries = Vec::new();
        let mut update_cache = false;

        // Get current file metadata
        let metadata = match fs::metadata(&file_path) {
            Ok(m) => m,
            Err(_) => continue, // File might have been deleted
        };

        let current_size = metadata.len();
        let current_mtime = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64;

        let is_new = detect_new_file(&file_path, &cache);

        // Check for new file
        if rebuild_all || is_new {
            if let Ok(entry) = create_file_entry(&file_path, is_new, rebuild_all, current_mtime) {
                entries.push(entry);
                update_cache = true;
            }
        }
        // Check for file growth
        else if detect_file_growth(&file_path, &cache, history_increment_size)? {
            if let Ok(entry) = create_file_entry(&file_path, false, rebuild_all, current_mtime) {
                entries.push(entry);
                update_cache = true;
            }
        }

        // Check for modified files (for Log section)
        if rebuild_all || detect_file_modification(&file_path, &cache)? {
            if let Ok(log_entries) = parse_log_section(&file_path, sys_data, &cache) {
                entries.extend(log_entries.clone());

                // Update seen headings in cache
                if !log_entries.is_empty() {
                    update_cache = true;

                    // Parse all headings from Log section to update seen set
                    if let Ok(contents) = fs::read_to_string(&file_path) {
                        let log_indicator = sys_data.config.popup_settings.history_settings
                            .as_ref()
                            .and_then(|h| h.anchor_log_indicator.as_deref())
                            .unwrap_or("# Log");

                        // Find the Log section - must be at start of a line, alone (only whitespace after)
                        let log_section_start = contents.lines().enumerate().find_map(|(line_num, line)| {
                            let trimmed = line.trim();
                            if trimmed == log_indicator {
                                // Found it! Calculate the byte position
                                contents.lines().take(line_num).map(|l| l.len() + 1).sum::<usize>()
                                    .into()
                            } else {
                                None
                            }
                        });

                        if let Some(log_start_pos) = log_section_start {
                            // Skip past the log indicator line itself
                            let after_indicator = &contents[log_start_pos..];
                            let first_newline = after_indicator.find('\n').unwrap_or(0);
                            let log_section = &contents[log_start_pos + first_newline..];

                            let anchor_levels = sys_data.config.popup_settings.history_settings
                                .as_ref()
                                .and_then(|h| h.anchor_levels.as_ref())
                                .cloned()
                                .unwrap_or_else(|| vec![1, 2]);

                            let mut seen = cache.files.get(&file_path)
                                .map(|m| m.seen_headings.clone())
                                .unwrap_or_default();

                            for line in log_section.lines() {
                                for level in &anchor_levels {
                                    let prefix = format!("{} ", "#".repeat(*level as usize));
                                    if line.starts_with(&prefix) {
                                        let heading_text = line[prefix.len()..].trim();
                                        seen.insert(heading_text.to_string());
                                    }
                                }
                            }

                            cache.files.entry(file_path.clone())
                                .or_insert(FileMetadata {
                                    size: current_size,
                                    mtime: current_mtime,
                                    last_history_size: current_size,
                                    seen_headings: HashSet::new(),
                                })
                                .seen_headings = seen;
                        }
                    }
                }
            }
        }

        // Update cache if needed
        if update_cache {
            cache.files.insert(file_path.clone(), FileMetadata {
                size: current_size,
                mtime: current_mtime,
                last_history_size: current_size, // Update last_history_size when entry is added
                seen_headings: cache.files.get(&file_path)
                    .map(|m| m.seen_headings.clone())
                    .unwrap_or_default(),
            });
        }

        // If we have entries, collect history updates
        if !entries.is_empty() {
            // Find which patch contains this file
            if let Some(patch_name) = infer_patch_simple(&file_path.to_string_lossy(), &folder_map) {
                let updates = collect_history_updates(entries, &patch_name, &sys_data.patches);

                // Merge into all_updates
                for (history_path, new_entries) in updates {
                    all_updates.entry(history_path)
                        .or_insert_with(Vec::new)
                        .extend(new_entries);
                }
            }
        }
    }

    // Write all history file updates
    if !all_updates.is_empty() {
        crate::utils::log(&format!("HISTORY: Updating {} history files", all_updates.len()));
        update_history_files(all_updates, rebuild_all)?;
    } else {
        crate::utils::log("HISTORY: No history updates needed");
    }

    // Save cache
    save_history_cache(&cache)?;
    crate::utils::log("HISTORY: History update complete");

    Ok(())
}
