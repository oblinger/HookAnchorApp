//! History Viewer - Standalone GUI for browsing command history
//!
//! This is a separate egui application that displays the command history database
//! in a searchable, filterable list.

use eframe::egui;
use hookanchor::core::{get_history_entries, HistoryEntry};
use std::env;

/// Main history viewer application
struct HistoryViewer {
    /// Actual selected patch filter (persisted to state)
    actual_patch_filter: String,
    /// Hover patch filter (temporary, not persisted)
    hover_patch_filter: Option<String>,
    /// Enable peek-on-hover functionality
    peek_on_hover: bool,
    /// Command name search filter (case-insensitive substring match)
    name_filter: String,
    /// Minimum edit size filter (text representation for editing)
    min_edit_size_filter: String,
    /// Selected action types for filtering (HashSet for quick lookup)
    selected_action_types: std::collections::HashSet<String>,
    /// Loaded history entries
    history_entries: Vec<HistoryEntry>,
    /// Filtered history entries (after applying filters)
    filtered_entries: Vec<HistoryEntry>,
    /// Selected row index (None if no selection)
    selected_index: Option<usize>,
    /// Error message to display
    error_message: Option<String>,
    /// Last known window position for tracking changes
    last_window_position: Option<egui::Pos2>,
    /// Last known window size for tracking changes
    last_window_size: Option<egui::Vec2>,
    /// Cached set of descendant patches for the current anchor filter
    anchor_descendant_patches: Option<std::collections::HashSet<String>>,
    /// The resolved anchor name (after alias resolution)
    resolved_anchor_name: Option<String>,
    /// Anchor tree navigation widget
    anchor_tree_navigator: hookanchor::ui::AnchorTreeNavigator,
    /// Cached patches HashMap for navigation
    patches_cache: Option<std::collections::HashMap<String, hookanchor::core::Patch>>,
    /// Anchor search/jump input text
    anchor_search_input: String,
    /// Sidebar width from config (used for consistent sizing)
    sidebar_width: f32,
    /// Minimum sidebar width from config
    sidebar_min_width: f32,
    /// Show current commands instead of history
    show_current_commands: bool,
}

impl HistoryViewer {
    /// Get the currently active patch filter (hover takes precedence if peeking enabled)
    fn get_active_patch_filter(&self) -> &str {
        if self.peek_on_hover {
            if let Some(ref hover) = self.hover_patch_filter {
                return hover;
            }
        }
        &self.actual_patch_filter
    }

    /// Resolve patch filter by looking up command and resolving aliases
    /// Returns the actual command name (not the patch!)
    /// Also handles case-insensitive matching
    fn resolve_patch_filter(input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }

        // Get commands from singleton
        let (sys_data, _) = hookanchor::core::get_sys_data();

        // Try to find a command matching the input (case-insensitive)
        if let Some(cmd) = sys_data.commands.iter().find(|c| c.command.eq_ignore_ascii_case(input)) {
            // Resolve any aliases to get the final command
            let resolved = cmd.resolve_alias(&sys_data.commands);

            // Return the resolved command name (not the patch!)
            return resolved.command;
        }

        // If no match, return the input as-is
        input.to_string()
    }

    /// Get all patch names that are descendants of the given anchor
    /// (i.e., all patches whose parent chain includes this anchor)
    /// Special case: "orphans" is the universal root and returns ALL patches
    fn get_descendant_patches(anchor_name: &str) -> std::collections::HashSet<String> {
        let mut descendants = std::collections::HashSet::new();
        let anchor_lower = anchor_name.to_lowercase();

        // Get patches from singleton
        let (sys_data, _) = hookanchor::core::get_sys_data();

        // Special case: "orphans" is the universal root - includes ALL patches
        // This is because patches with no parent are shown under "orphans" in the tree
        if anchor_lower == "orphans" {
            for patch_name in sys_data.patches.keys() {
                descendants.insert(patch_name.to_string());
            }
            descendants.insert("orphans".to_string());
            hookanchor::utils::log(&format!("ANCHOR_FILTER: orphans (universal root) includes ALL {} patches", descendants.len()));
            return descendants;
        }

        hookanchor::utils::log(&format!("ANCHOR_FILTER: Looking for descendants of '{}'", anchor_name));

        // For each patch, check if the anchor is in its parent chain
        for (patch_name, _patch) in &sys_data.patches {
            let patch_path = hookanchor::core::get_patch_path(patch_name, &sys_data.patches);

            // Check if anchor is anywhere in the path (case-insensitive)
            if patch_path.iter().any(|p| p.eq_ignore_ascii_case(anchor_name)) ||
               patch_name.eq_ignore_ascii_case(anchor_name) {
                hookanchor::utils::log(&format!("ANCHOR_FILTER: Found descendant: {} (path: {:?})", patch_name, patch_path));
                descendants.insert(patch_name.to_string());
            }
        }

        // Also include the anchor itself
        descendants.insert(anchor_lower);

        hookanchor::utils::log(&format!("ANCHOR_FILTER: Total {} descendants found", descendants.len()));

        descendants
    }

    /// Check if a history entry matches the anchor filter
    fn matches_anchor_filter(
        entry: &HistoryEntry,
        anchor_patches: &std::collections::HashSet<String>,
        anchor_name: &str,
    ) -> bool {
        // Fast path: explicit patch match
        if !entry.patch.is_empty() {
            let patch_lower = entry.patch.to_lowercase();
            if anchor_patches.contains(&patch_lower) {
                hookanchor::utils::detailed_log("ANCHOR_MATCH", &format!("MATCH via patch: {} (patch: {})", entry.command, entry.patch));
                return true;
            }
        }

        // Medium path: command prefix match (separator-aware)
        // Use two-step approach for efficiency:
        // 1. Fast check: case-insensitive prefix match
        // 2. If passes, verify next char is separator (or exact match)
        let cmd_lower = entry.command.to_lowercase();
        let anchor_lower = anchor_name.to_lowercase();

        if cmd_lower.starts_with(&anchor_lower) {
            // Exact match - allow it
            if cmd_lower.len() == anchor_lower.len() {
                hookanchor::utils::detailed_log("ANCHOR_MATCH", &format!("MATCH via exact command: {}", entry.command));
                return true;
            }

            // Prefix match - check if followed by separator
            // Get separators from config (default: " ._-!")
            let separators = &hookanchor::core::get_config()
                .popup_settings
                .word_separators;

            if let Some(next_char) = entry.command.chars().nth(anchor_name.len()) {
                if separators.contains(next_char) {
                    hookanchor::utils::detailed_log("ANCHOR_MATCH", &format!("MATCH via command prefix+separator: {} (next char: '{}')", entry.command, next_char));
                    return true;
                }
            }
        }

        // Slow path: file path prefix match (separator-aware)
        if let Some(ref file_path) = entry.file_path {
            let path_lower = file_path.to_lowercase();
            let anchor_lower = anchor_name.to_lowercase();

            if path_lower.starts_with(&anchor_lower) {
                // Exact match
                if path_lower.len() == anchor_lower.len() {
                    hookanchor::utils::detailed_log("ANCHOR_MATCH", &format!("MATCH via exact file path: {} (path: {})", entry.command, file_path));
                    return true;
                }

                // Prefix match - check if followed by separator or path separator
                let separators = &hookanchor::core::get_config()
                    .popup_settings
                    .word_separators;

                if let Some(next_char) = file_path.chars().nth(anchor_name.len()) {
                    // Allow word separators OR path separator '/'
                    if separators.contains(next_char) || next_char == '/' {
                        hookanchor::utils::detailed_log("ANCHOR_MATCH", &format!("MATCH via file path prefix+separator: {} (path: {}, next char: '{}')", entry.command, file_path, next_char));
                        return true;
                    }
                }
            }
        }

        // Don't log NO MATCH - creates thousands of log entries that lock up the GUI
        false
    }

    /// Save current viewer state to state.json
    fn save_viewer_state(&self) {
        let mut state = hookanchor::core::get_state();

        // Parse min_edit_size_filter to Option<i64>
        let min_edit_size = if self.min_edit_size_filter.is_empty() {
            None
        } else {
            self.min_edit_size_filter.parse::<i64>().ok()
        };

        // Convert HashSet to Vec for serialization
        let selected_action_types: Vec<String> =
            self.selected_action_types.iter().cloned().collect();

        // Update history viewer state
        state.history_viewer_state = hookanchor::core::HistoryViewerState {
            position: self.last_window_position.map(|pos| (pos.x, pos.y)),
            window_size: self.last_window_size.map(|size| (size.x, size.y)),
            patch_filter: self.actual_patch_filter.clone(),
            name_filter: self.name_filter.clone(),
            min_edit_size,
            selected_action_types,
            sidebar_width: Some(self.sidebar_width),
            show_current_commands: self.show_current_commands,
        };

        // Save to file
        if let Err(e) = hookanchor::core::set_state(&state) {
            hookanchor::utils::log_error(&format!("Failed to save history viewer state: {}", e));
        }
    }

    /// Create new history viewer, loading state from state.json
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load saved state
        let state = hookanchor::core::get_state();
        let viewer_state = &state.history_viewer_state;

        // Convert min_edit_size from Option<i64> to String for text field
        let min_edit_size_filter = viewer_state.min_edit_size
            .map(|n| n.to_string())
            .unwrap_or_default();

        // Convert Vec<String> to HashSet for selected_action_types
        let selected_action_types: std::collections::HashSet<String> =
            viewer_state.selected_action_types.iter().cloned().collect();

        // Build patches cache for navigation from singleton
        let (sys_data, _) = hookanchor::core::get_sys_data();
        let patches_cache = sys_data.patches;

        // Debug: Log patch count
        hookanchor::utils::log(&format!("HISTORY_VIEWER_INIT: Loaded {} patches from singleton", patches_cache.len()));
        hookanchor::utils::log(&format!("HISTORY_VIEWER_INIT: Loaded {} commands from singleton", sys_data.commands.len()));

        // Debug: Count anchor commands
        let anchor_count = sys_data.commands.iter().filter(|c| c.is_anchor()).count();
        hookanchor::utils::log(&format!("HISTORY_VIEWER_INIT: Found {} anchor commands", anchor_count));

        // Get tree sidebar settings from config
        let config = hookanchor::core::get_config();
        let config_tree_width = config.history_viewer.as_ref()
            .and_then(|hv| hv.tree_sidebar_width)
            .unwrap_or(250.0);
        let config_tree_min_width = config.history_viewer.as_ref()
            .and_then(|hv| hv.tree_sidebar_min_width)
            .unwrap_or(50.0);

        // Prefer saved sidebar width over config default (capped at 800px)
        let tree_width = viewer_state.sidebar_width.unwrap_or(config_tree_width).min(800.0);

        let tree_indent = config.history_viewer.as_ref()
            .and_then(|hv| hv.tree_indent_pixels)
            .unwrap_or(10.0);
        let tree_guides = config.history_viewer.as_ref()
            .and_then(|hv| hv.tree_show_guides)
            .unwrap_or(true);
        let peek_on_hover = config.history_viewer.as_ref()
            .and_then(|hv| hv.peek_on_hover)
            .unwrap_or(true);

        // Resolve patch_filter alias to get the actual patch name
        let resolved_patch = if !viewer_state.patch_filter.is_empty() {
            Self::resolve_patch_filter(&viewer_state.patch_filter)
        } else {
            String::new()
        };

        // Initialize breadcrumb navigator with current patch and settings
        let mut anchor_tree_navigator = hookanchor::ui::AnchorTreeNavigator::with_settings(
            tree_width,
            tree_indent,
            tree_guides,
        );
        if !resolved_patch.is_empty() {
            anchor_tree_navigator.set_current_patch(resolved_patch.clone(), &patches_cache);
        }

        let patches_cache = Some(patches_cache);

        let mut viewer = Self {
            actual_patch_filter: resolved_patch.clone(),
            hover_patch_filter: None,
            peek_on_hover,
            name_filter: viewer_state.name_filter.clone(),
            min_edit_size_filter,
            selected_action_types,
            history_entries: Vec::new(),
            filtered_entries: Vec::new(),
            selected_index: None,
            error_message: None,
            last_window_position: viewer_state.position.map(|(x, y)| egui::pos2(x, y)),
            last_window_size: viewer_state.window_size.map(|(w, h)| egui::vec2(w, h)),
            anchor_descendant_patches: None,
            resolved_anchor_name: None,
            anchor_tree_navigator,
            patches_cache,
            anchor_search_input: resolved_patch, // Initialize with current patch
            sidebar_width: tree_width,
            sidebar_min_width: config_tree_min_width,
            show_current_commands: viewer_state.show_current_commands,
        };

        // Load initial history
        viewer.reload_history();

        viewer
    }

    /// Reload patches cache from singleton (call after data changes like rescan)
    fn reload_patches(&mut self) {
        let (sys_data, _) = hookanchor::core::get_sys_data();
        self.patches_cache = Some(sys_data.patches);
    }

    /// Reload history from database with current filters
    fn reload_history(&mut self) {
        // Also reload patches in case they've changed (e.g., after rescan or delete-history)
        self.reload_patches();

        self.history_entries.clear();
        self.filtered_entries.clear();
        self.selected_index = None;

        // Resolve patch filter (handles aliases and case-insensitive matching)
        // Use active filter (hover takes precedence if peeking)
        let active_filter = self.get_active_patch_filter();
        let resolved_patch = Self::resolve_patch_filter(active_filter);

        // Build descendant patches cache if we have an anchor filter
        if !resolved_patch.is_empty() {
            self.resolved_anchor_name = Some(resolved_patch.clone());
            self.anchor_descendant_patches = Some(Self::get_descendant_patches(&resolved_patch));
        } else {
            self.resolved_anchor_name = None;
            self.anchor_descendant_patches = None;
        }

        if self.show_current_commands {
            // Load current commands from sys_data
            let (sys_data, _) = hookanchor::core::get_sys_data();
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            // Convert commands to HistoryEntry format
            self.history_entries = sys_data.commands.iter().map(|cmd| {
                hookanchor::core::HistoryEntry {
                    id: 0,
                    timestamp: current_time,
                    patch: cmd.patch.clone(),
                    command: cmd.command.clone(),
                    action: cmd.action.clone(),
                    arg: if cmd.arg.is_empty() { None } else { Some(cmd.arg.clone()) },
                    flags: if cmd.flags.is_empty() { None } else { Some(cmd.flags.clone()) },
                    file_path: if cmd.arg.starts_with('/') || cmd.arg.starts_with('~') {
                        Some(cmd.arg.clone())
                    } else {
                        None
                    },
                    edit_size: cmd.file_size.map(|s| s as i64),
                }
            }).collect();

            hookanchor::utils::detailed_log("VIEWER", &format!(
                "Loaded {} current commands from sys_data",
                self.history_entries.len()
            ));
        } else {
            // Get limit from config, default to 50000
            let limit = hookanchor::core::get_config()
                .history_viewer
                .as_ref()
                .and_then(|hv| hv.viewable_history_limit)
                .unwrap_or(50000);

            // Load history entries using the new API
            // exclude_deletions=true to hide $DELETED$ entries from the viewer
            match get_history_entries(limit, true) {
                Ok(entries) => {
                    self.history_entries = entries;

                    // Log what we loaded
                    hookanchor::utils::detailed_log("VIEWER", &format!(
                        "Loaded {} total entries from database",
                        self.history_entries.len()
                    ));

                    // Count by action type
                    let mut action_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
                    for entry in &self.history_entries {
                        *action_counts.entry(entry.action.clone()).or_insert(0) += 1;
                    }

                    // Log action type breakdown
                    let mut action_list: Vec<_> = action_counts.iter().collect();
                    action_list.sort_by_key(|(action, _)| *action);
                    for (action, count) in action_list {
                        hookanchor::utils::detailed_log("VIEWER", &format!(
                            "  {} entries with action '{}'",
                            count, action
                        ));
                    }
                }
                Err(e) => {
                    self.error_message = Some(format!("Query error: {}", e));
                }
            }
        }

        // Apply all filters
        self.apply_filters();
    }

    /// Apply all filters to history entries
    fn apply_filters(&mut self) {
        self.filtered_entries.clear();

        let name_filter_lower = self.name_filter.to_lowercase();

        // Parse minimum edit size filter
        let min_edit_size: Option<i64> = self.min_edit_size_filter.trim()
            .parse::<i64>()
            .ok()
            .filter(|&n| n > 0);

        // Log active filters
        hookanchor::utils::detailed_log("VIEWER", &format!(
            "Applying filters - name_filter='{}', min_edit_size={:?}, selected_action_types={:?}, anchor_filter={:?}",
            self.name_filter, min_edit_size, self.selected_action_types, self.resolved_anchor_name
        ));

        for entry in &self.history_entries {
            // Apply anchor filter (if active)
            if let (Some(ref anchor_patches), Some(ref anchor_name)) =
                (&self.anchor_descendant_patches, &self.resolved_anchor_name) {
                if !Self::matches_anchor_filter(entry, anchor_patches, anchor_name) {
                    continue;
                }
            }

            // Apply name filter (case-insensitive substring match against command name AND type)
            if !name_filter_lower.is_empty() {
                let cmd_lower = entry.command.to_lowercase();
                let type_str = Self::get_display_type(entry).to_lowercase();

                // Match if either command name OR type contains the filter text
                if !cmd_lower.contains(&name_filter_lower) && !type_str.contains(&name_filter_lower) {
                    continue;
                }
            }

            // Apply minimum edit size filter (only for file-based entries that have edit_size)
            // Non-file commands (1pass, alias, work, etc.) don't have edit_size and should be allowed through
            if let Some(min_size) = min_edit_size {
                if let Some(edit_size) = entry.edit_size {
                    // Entry has edit_size - apply the filter
                    if edit_size.abs() < min_size {
                        continue;
                    }
                }
                // Entry doesn't have edit_size - allow it through (it's not a file-based command)
            }

            // Apply action type filter
            if !self.selected_action_types.is_empty() {
                if !self.selected_action_types.contains(&entry.action) {
                    continue;
                }
            }

            // Entry passed all filters
            self.filtered_entries.push(entry.clone());
        }

        // Log filtering results
        hookanchor::utils::detailed_log("VIEWER", &format!(
            "After filtering: {} entries passed out of {} total",
            self.filtered_entries.len(), self.history_entries.len()
        ));

        // Count by action type after filtering
        let mut filtered_action_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for entry in &self.filtered_entries {
            *filtered_action_counts.entry(entry.action.clone()).or_insert(0) += 1;
        }

        let mut filtered_action_list: Vec<_> = filtered_action_counts.iter().collect();
        filtered_action_list.sort_by_key(|(action, _)| *action);
        for (action, count) in filtered_action_list {
            hookanchor::utils::detailed_log("VIEWER", &format!(
                "  {} filtered entries with action '{}'",
                count, action
            ));
        }
    }

    /// Execute the selected command
    fn execute_command(&self, entry: &HistoryEntry) {
        // Get the path to the ha binary
        let exe_dir = hookanchor::utils::get_binary_dir();
        let ha_path = exe_dir.join("ha");

        // Execute command using: ha -x "command_name" (execute top match)
        let result = std::process::Command::new(&ha_path)
            .arg("-x")
            .arg(&entry.command)
            .spawn();

        match result {
            Ok(_) => {
                hookanchor::utils::log(&format!("HISTORY_VIEWER: Executed command '{}'", entry.command));
            }
            Err(e) => {
                hookanchor::utils::log_error(&format!("HISTORY_VIEWER: Failed to execute command '{}': {}", entry.command, e));
            }
        }
    }

    /// Edit the currently selected history entry
    /// Opens the popup with the command pre-loaded and executes the edit_selection action
    fn edit_selected_entry(&self) {
        if let Some(idx) = self.selected_index {
            if idx < self.filtered_entries.len() {
                let entry = &self.filtered_entries[idx];

                // Get the path to the ha binary
                let exe_dir = hookanchor::utils::get_binary_dir();
                let ha_path = exe_dir.join("ha");

                // Open popup with command pre-loaded and execute edit_selection action
                // This will: 1) Open popup, 2) Set input to command name, 3) Execute edit_selection template
                let result = std::process::Command::new(&ha_path)
                    .arg("--popup")
                    .arg("--input")
                    .arg(&entry.command)
                    .arg("--action")
                    .arg("edit_selection")
                    .spawn();

                match result {
                    Ok(_) => {
                        hookanchor::utils::log(&format!("HISTORY_VIEWER: Opened editor for command '{}'", entry.command));
                    }
                    Err(e) => {
                        hookanchor::utils::log_error(&format!("HISTORY_VIEWER: Failed to open editor for '{}': {}", entry.command, e));
                    }
                }
            }
        }
    }

    /// Format timestamp as readable date (date only, no time)
    fn format_timestamp(timestamp: i64) -> String {
        use chrono::{Local, TimeZone};
        if let Some(dt) = Local.timestamp_opt(timestamp, 0).single() {
            dt.format("%Y-%m-%d").to_string()
        } else {
            "Invalid date".to_string()
        }
    }

    /// Format edit size using binary prefixes (K/M/G for 1024-based units)
    /// Examples: 15K (15 kilobytes), 2M (2 megabytes), 3G (3 gigabytes)
    fn format_edit_size(size: Option<i64>) -> String {
        match size {
            Some(s) if s > 0 => {
                // >= 1 GiB (1024^3 = 1,073,741,824): show in gigabytes
                if s >= 1_073_741_824 {
                    let gigabytes = (s as f64 / 1_073_741_824.0).round() as i64;
                    format!("{}G", gigabytes)
                }
                // >= 1 MiB (1024^2 = 1,048,576): show in megabytes
                else if s >= 1_048_576 {
                    let megabytes = (s as f64 / 1_048_576.0).round() as i64;
                    format!("{}M", megabytes)
                }
                // >= 1 KiB (1024): show in kilobytes
                else if s >= 1024 {
                    let kilobytes = (s as f64 / 1024.0).round() as i64;
                    format!("{}K", kilobytes)
                }
                // < 1 KiB: show raw bytes or empty for very small values
                else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }

    /// Format argument path for display - replace home directory with ~
    fn format_arg_for_display(arg: &str) -> String {
        if let Some(home) = dirs::home_dir() {
            if let Some(home_str) = home.to_str() {
                if arg.starts_with(home_str) {
                    return arg.replacen(home_str, "~", 1);
                }
            }
        }
        arg.to_string()
    }

    /// Format number with commas for thousands separator
    fn format_number_with_commas(n: usize) -> String {
        let s = n.to_string();
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(*c);
        }

        result
    }

    /// Calculate the display type for an entry
    /// If entry has a file_path, use the file extension with "." prefix
    /// Otherwise, use the action type (no prefix)
    fn get_display_type(entry: &HistoryEntry) -> String {
        if let Some(ref file_path) = entry.file_path {
            // Extract file extension and add "." prefix
            if let Some(extension) = std::path::Path::new(file_path)
                .extension()
                .and_then(|ext| ext.to_str())
            {
                return format!(".{}", extension);
            }
        }
        // No file path or no extension - use action (no prefix)
        entry.action.clone()
    }
}

impl eframe::App for HistoryViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Track window position and size changes and save to state
        let (position_changed, size_changed) = ctx.input(|i| {
            let mut pos_changed = false;
            let mut size_changed = false;

            // Track position changes
            if let Some(current_pos) = i.viewport().outer_rect.map(|r| r.min) {
                pos_changed = self.last_window_position.map_or(true, |last_pos| {
                    (current_pos.x - last_pos.x).abs() > 5.0 || (current_pos.y - last_pos.y).abs() > 5.0
                });
                if pos_changed {
                    self.last_window_position = Some(current_pos);
                }
            }

            // Track size changes
            if let Some(current_size) = i.viewport().outer_rect.map(|r| r.size()) {
                size_changed = self.last_window_size.map_or(true, |last_size| {
                    (current_size.x - last_size.x).abs() > 5.0 || (current_size.y - last_size.y).abs() > 5.0
                });
                if size_changed {
                    self.last_window_size = Some(current_size);
                }
            }

            (pos_changed, size_changed)
        });

        if position_changed || size_changed {
            self.save_viewer_state();
        }

        // Check for keyboard shortcuts
        ctx.input(|i| {
            // Escape to collapse one level of tree or close window
            if i.key_pressed(egui::Key::Escape) {
                if self.anchor_tree_navigator.is_tree_expanded() {
                    // Collapse one level of the navigation tree
                    self.anchor_tree_navigator.collapse_one_level();
                } else {
                    // No tree expanded, close the window
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }

            // Enter to execute selected command
            if i.key_pressed(egui::Key::Enter) {
                if let Some(idx) = self.selected_index {
                    if idx < self.filtered_entries.len() {
                        self.execute_command(&self.filtered_entries[idx]);
                    }
                }
            }

            // "/" (Slash) to open folder in Finder for selected command
            if i.key_pressed(egui::Key::Slash) {
                if let Some(idx) = self.selected_index {
                    if idx < self.filtered_entries.len() {
                        let entry = &self.filtered_entries[idx];
                        if let Some(ref file_path) = entry.file_path {
                            // Extract folder path from file path
                            if let Some(folder_path) = std::path::Path::new(file_path).parent() {
                                // Open folder in Finder
                                let result = std::process::Command::new("open")
                                    .arg(folder_path)
                                    .spawn();

                                match result {
                                    Ok(_) => {
                                        hookanchor::utils::log(&format!("HISTORY_VIEWER: Opened folder '{}'", folder_path.display()));
                                    }
                                    Err(e) => {
                                        hookanchor::utils::log_error(&format!("HISTORY_VIEWER: Failed to open folder '{}': {}", folder_path.display(), e));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Edit selected command key (configurable via history_viewer.key_bindings.edit_selection)
            // Default: ";" (Semicolon)
            let config = hookanchor::core::get_config();
            let edit_key_string = config.history_viewer
                .as_ref()
                .and_then(|hv| hv.key_bindings.as_ref())
                .and_then(|kb| kb.edit_selection.as_ref())
                .map(|s| s.as_str())
                .unwrap_or(";");

            // Parse the key string and check if it's pressed
            if let Ok(keystroke) = hookanchor::core::Keystroke::from_key_string(edit_key_string) {
                if i.key_pressed(keystroke.key) {
                    self.edit_selected_entry();
                }
            }
        });

        // Show tree navigation sidebar on the left
        egui::SidePanel::left("tree_sidebar")
            .resizable(true)
            .default_width(self.sidebar_width.min(800.0)) // Cap at 800px to leave room for content
            .min_width(self.sidebar_min_width) // Minimum width from config
            .max_width(800.0) // Maximum 800px to ensure content area is visible
            .show(ctx, |ui| {
                // Get actual panel width (may differ from default if user resized)
                let panel_width = ui.available_width();

                // If user resized the panel, save the new width (capped at 800px)
                if (panel_width - self.sidebar_width).abs() > 1.0 {
                    self.sidebar_width = panel_width.min(800.0);
                    self.save_viewer_state();
                }

                // Anchor search/jump input at the top of sidebar (no label)
                let search_response = ui.add_sized(
                    [panel_width - 10.0, 20.0], // Use actual panel width
                    egui::TextEdit::singleline(&mut self.anchor_search_input)
                        .hint_text("anchor name...")
                );

                // When user presses Enter, jump to that anchor (or clear filter if empty)
                if search_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !self.anchor_search_input.is_empty() {
                        // Resolve alias and jump to anchor
                        let resolved = Self::resolve_patch_filter(&self.anchor_search_input);
                        if !resolved.is_empty() {
                            if let Some(ref patches) = self.patches_cache {
                                self.actual_patch_filter = resolved.clone();
                                self.hover_patch_filter = None;  // Clear hover
                                self.anchor_search_input = resolved.clone();
                                self.anchor_tree_navigator.set_current_patch(resolved, patches);
                                self.reload_history();
                                self.save_viewer_state();
                            }
                        }
                    } else {
                        // Empty input - clear filter to show everything
                        self.actual_patch_filter = String::new();
                        self.hover_patch_filter = None;  // Clear hover
                        if let Some(ref patches) = self.patches_cache {
                            self.anchor_tree_navigator.set_current_patch("orphans".to_string(), patches);
                        }
                        self.reload_history();
                        self.save_viewer_state();
                    }
                }

                ui.separator();

                // Tree in the middle with vertical scroll
                let available_height = ui.available_height();
                let entry_count_height = 35.0; // Reserve space for separator + entry count label + padding

                // Set ScrollArea to use full sidebar width
                egui::ScrollArea::vertical()
                    .max_height(available_height - entry_count_height)
                    .auto_shrink([false, false]) // Don't shrink - use full available width
                    .show(ui, |ui| {
                        if let Some(ref patches) = self.patches_cache {
                            let (clicked_patch, hovered_patch) = self.anchor_tree_navigator.show_sidebar(ui, patches);

                            // Handle click - becomes the actual selection
                            if let Some(new_patch) = clicked_patch {
                                // User clicked to navigate to a new patch
                                // If root ("orphans") is selected, clear filter to show everything
                                if new_patch.to_lowercase() == "orphans" {
                                    self.actual_patch_filter = String::new();
                                    self.anchor_search_input = String::new();
                                } else {
                                    self.actual_patch_filter = new_patch.clone();
                                    self.anchor_search_input = new_patch.clone();
                                }
                                self.hover_patch_filter = None;  // Clear hover on click
                                self.anchor_tree_navigator.set_current_patch(new_patch, patches);
                                self.reload_history();
                                self.save_viewer_state();
                            }

                            // Handle hover - temporary peek (if enabled)
                            if self.peek_on_hover {
                                // Check if hover changed
                                let hover_changed = self.hover_patch_filter != hovered_patch;

                                if hover_changed {
                                    self.hover_patch_filter = hovered_patch;
                                    self.reload_history();  // Reload with hover filter
                                    // DON'T save state - this is temporary
                                }
                            }
                        }
                    });

                // Show entry count at the bottom of the sidebar with proper spacing
                ui.add_space(3.0);
                ui.separator();
                ui.add_space(2.0);
                ui.label(format!("{} entries", Self::format_number_with_commas(self.filtered_entries.len())));
            });

        // Main content panel on the right
        egui::CentralPanel::default().show(ctx, |ui| {

            // Top bar with all filters and close button
            ui.horizontal(|ui| {
                ui.add_space(10.0);

                // Action type multi-selector using ComboBox (before magnifying glass)
                // Use the same action list as the command editor (from config listed_actions)
                let action_types = hookanchor::ui::helpers::get_listed_actions();

                // Calculate dropdown height based on item count
                // Each checkbox item needs ~30px, "Clear All" button needs ~30px, separator ~15px
                // Add extra padding for top/bottom and dropdown chrome ~100px (generous for safety)
                let dropdown_height = (action_types.len() as f32 * 30.0) + 30.0 + 15.0 + 100.0;

                egui::ComboBox::new("action_type_selector", "")
                    .selected_text(if self.selected_action_types.is_empty() {
                        "All types".to_string()
                    } else {
                        format!("{} types", self.selected_action_types.len())
                    })
                    .height(dropdown_height)
                    .show_ui(ui, |ui| {
                        // "Clear All" option at the top
                        if ui.button("Clear All").clicked() {
                            self.selected_action_types.clear();
                            self.apply_filters();
                            self.save_viewer_state();
                        }
                        ui.separator();

                        for action_type in action_types {
                            let is_selected = self.selected_action_types.contains(&action_type);
                            let mut selected = is_selected;

                            if ui.checkbox(&mut selected, &action_type).clicked() {
                                if selected {
                                    self.selected_action_types.insert(action_type.clone());
                                } else {
                                    self.selected_action_types.remove(&action_type);
                                }
                                self.apply_filters();
                                self.save_viewer_state();
                            }
                        }
                    });

                ui.add_space(10.0);

                // Search filter with magnifying glass (matches command name and type)
                ui.label("🔍");
                let name_response = ui.add(
                    egui::TextEdit::singleline(&mut self.name_filter)
                        .desired_width(225.0)  // Increased by 50% from 150.0
                );

                if name_response.changed() {
                    self.apply_filters();
                    self.save_viewer_state();
                }

                // Push min edit size and close button to the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Close button on far right
                    if ui.button("❌").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    // Min edit size filter with ≥ symbol
                    let edit_size_response = ui.add(
                        egui::TextEdit::singleline(&mut self.min_edit_size_filter)
                            .desired_width(80.0)
                            .hint_text("min bytes")
                    );

                    if edit_size_response.changed() {
                        self.apply_filters();
                        self.save_viewer_state();
                    }

                    ui.label("≥");

                    // "Now" checkbox to show current commands instead of history
                    if ui.checkbox(&mut self.show_current_commands, "Now").clicked() {
                        self.reload_history();
                        self.save_viewer_state();
                    }
                });
            });

            ui.separator();

            // Show error message if any
            if let Some(ref error) = self.error_message {
                ui.colored_label(egui::Color32::RED, error);
                ui.separator();
            }

            // Scrollable history list with canvas painting for performance
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Configuration
                    let row_height = 18.0;
                    let font_id = egui::FontId::proportional(13.0);
                    let header_font = egui::FontId::proportional(14.0);

                    // Column positions (left edge of each column)
                    let col_date = 5.0;
                    let col_size = 85.0;     // Date column end (+5px for full date display)
                    let col_type = 140.0;    // Update column (edit size with K/M/G suffix, +15px for 999G)
                    let col_command = 220.0; // Command column
                    let col_path = 520.0;    // Path column

                    // Header row (not scrolled) - use absolute positioning to match data rows
                    let available_width = ui.available_width();
                    let (response, painter) = ui.allocate_painter(
                        egui::vec2(available_width, 20.0),
                        egui::Sense::hover()
                    );

                    let header_y = response.rect.min.y;
                    painter.text(
                        egui::pos2(response.rect.min.x + col_date, header_y),
                        egui::Align2::LEFT_TOP,
                        "Date",
                        header_font.clone(),
                        ui.style().visuals.text_color()
                    );
                    painter.text(
                        egui::pos2(response.rect.min.x + col_type - 10.0, header_y),  // Right-align header
                        egui::Align2::RIGHT_TOP,
                        "Update",
                        header_font.clone(),
                        ui.style().visuals.text_color()
                    );
                    painter.text(
                        egui::pos2(response.rect.min.x + col_type, header_y),
                        egui::Align2::LEFT_TOP,
                        "Type",
                        header_font.clone(),
                        ui.style().visuals.text_color()
                    );
                    painter.text(
                        egui::pos2(response.rect.min.x + col_command, header_y),
                        egui::Align2::LEFT_TOP,
                        "Command",
                        header_font.clone(),
                        ui.style().visuals.text_color()
                    );
                    painter.text(
                        egui::pos2(response.rect.min.x + col_path, header_y),
                        egui::Align2::LEFT_TOP,
                        "Path",
                        header_font.clone(),
                        ui.style().visuals.text_color()
                    );

                    ui.separator();

                    // Calculate total content height for scrollbar
                    let total_height = self.filtered_entries.len() as f32 * row_height;

                    // Get available width for the content area
                    let available_width = ui.available_width();

                    // Allocate space and get painter
                    let (response, painter) = ui.allocate_painter(
                        egui::vec2(available_width, total_height),
                        egui::Sense::click()
                    );

                    // Get the visible rect to determine which rows to paint
                    let visible_rect = response.rect;

                    // Calculate which rows are visible
                    let first_visible_row = ((visible_rect.min.y - response.rect.min.y) / row_height).floor().max(0.0) as usize;
                    let last_visible_row = ((visible_rect.max.y - response.rect.min.y) / row_height).ceil() as usize;
                    let last_visible_row = last_visible_row.min(self.filtered_entries.len());

                    // Paint only visible rows
                    for idx in first_visible_row..last_visible_row {
                        let entry = &self.filtered_entries[idx];
                        let is_selected = self.selected_index == Some(idx);

                        // Calculate row Y position
                        let row_y = response.rect.min.y + (idx as f32 * row_height);
                        let row_rect = egui::Rect::from_min_size(
                            egui::pos2(response.rect.min.x, row_y),
                            egui::vec2(available_width, row_height)
                        );

                        // Paint background
                        if is_selected {
                            painter.rect_filled(row_rect, 0.0, egui::Color32::from_rgb(100, 150, 255));
                        } else if idx % 2 == 0 {
                            painter.rect_filled(row_rect, 0.0, egui::Color32::from_gray(250));
                        }

                        // Prepare text content
                        let date_text = Self::format_timestamp(entry.timestamp);
                        let size_text = Self::format_edit_size(entry.edit_size);
                        let type_text = Self::get_display_type(entry);
                        let cmd_text = &entry.command;
                        let path_text = Self::format_arg_for_display(entry.file_path.as_deref().unwrap_or(""));

                        // Text color
                        let text_color = if is_selected {
                            egui::Color32::WHITE
                        } else {
                            ui.style().visuals.text_color()
                        };

                        // Set up clipping rects for each column to prevent overflow
                        let clip_date = egui::Rect::from_min_max(
                            egui::pos2(response.rect.min.x + col_date, row_y),
                            egui::pos2(response.rect.min.x + col_size - 5.0, row_y + row_height)
                        );
                        let clip_size = egui::Rect::from_min_max(
                            egui::pos2(response.rect.min.x + col_size, row_y),
                            egui::pos2(response.rect.min.x + col_type - 5.0, row_y + row_height)
                        );
                        let clip_type = egui::Rect::from_min_max(
                            egui::pos2(response.rect.min.x + col_type, row_y),
                            egui::pos2(response.rect.min.x + col_command - 5.0, row_y + row_height)
                        );
                        let clip_command = egui::Rect::from_min_max(
                            egui::pos2(response.rect.min.x + col_command, row_y),
                            egui::pos2(response.rect.min.x + col_path - 5.0, row_y + row_height)
                        );

                        // Paint text columns with clipping
                        painter.with_clip_rect(clip_date).text(
                            egui::pos2(response.rect.min.x + col_date, row_y),
                            egui::Align2::LEFT_TOP,
                            &date_text,
                            font_id.clone(),
                            text_color
                        );

                        painter.with_clip_rect(clip_size).text(
                            egui::pos2(response.rect.min.x + col_type - 10.0, row_y),  // Right-align before Type column
                            egui::Align2::RIGHT_TOP,
                            &size_text,
                            font_id.clone(),
                            text_color
                        );

                        painter.with_clip_rect(clip_type).text(
                            egui::pos2(response.rect.min.x + col_type, row_y),
                            egui::Align2::LEFT_TOP,
                            type_text,
                            font_id.clone(),
                            text_color
                        );

                        painter.with_clip_rect(clip_command).text(
                            egui::pos2(response.rect.min.x + col_command, row_y),
                            egui::Align2::LEFT_TOP,
                            cmd_text,
                            font_id.clone(),
                            text_color
                        );

                        // Path column doesn't need clipping (it's the last column)
                        painter.text(
                            egui::pos2(response.rect.min.x + col_path, row_y),
                            egui::Align2::LEFT_TOP,
                            &path_text,
                            font_id.clone(),
                            text_color
                        );
                    }

                    // Handle single click - just select
                    if response.clicked() {
                        if let Some(click_pos) = response.interact_pointer_pos() {
                            let clicked_row = ((click_pos.y - response.rect.min.y) / row_height) as usize;
                            if clicked_row < self.filtered_entries.len() {
                                self.selected_index = Some(clicked_row);
                            }
                        }
                    }

                    // Handle double click - execute
                    if response.double_clicked() {
                        if let Some(click_pos) = response.interact_pointer_pos() {
                            let clicked_row = ((click_pos.y - response.rect.min.y) / row_height) as usize;
                            if clicked_row < self.filtered_entries.len() {
                                self.selected_index = Some(clicked_row);
                                self.execute_command(&self.filtered_entries[clicked_row]);
                            }
                        }
                    }
                });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // Initialize sys_data (config + cache) before creating the viewer
    if let Err(e) = hookanchor::core::initialize() {
        hookanchor::utils::log_error(&format!("Failed to initialize sys_data: {}", e));
        // Continue with default config
    }

    // Load saved window position and size from state
    let state = hookanchor::core::get_state();
    let saved_position = state.history_viewer_state.position;
    let saved_size = state.history_viewer_state.window_size;

    let mut viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size(saved_size.map(|(w, h)| [w, h]).unwrap_or([1000.0, 700.0]))
        .with_title("History Viewer")
        .with_resizable(true);

    // Apply saved position if available
    if let Some((x, y)) = saved_position {
        viewport_builder = viewport_builder.with_position([x, y]);
    }

    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };

    eframe::run_native(
        "History Viewer",
        options,
        Box::new(|cc| Ok(Box::new(HistoryViewer::new(cc)))),
    )
}
