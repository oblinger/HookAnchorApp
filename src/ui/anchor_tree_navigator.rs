//! Anchor Tree Navigation Widget
//!
//! A reusable widget for navigating patch (anchor) hierarchies with tree display
//! and breadcrumb path.
//!
//! Features:
//! - Tree view of anchor hierarchy
//! - Breadcrumb path display (orphans → Projects → ML → AI Safety)
//! - Click tree nodes to focus on that patch
//! - Hover over tree nodes to peek at their content (if enabled)
//! - Focus collapses all nodes, then expands path to selected + its children
//! - Escape key to collapse one level at a time

use std::collections::HashMap;
use crate::core::{Patch, get_patch, get_patch_path};

/// State for the anchor tree navigator widget
pub struct AnchorTreeNavigator {
    /// Currently selected patch name (rightmost breadcrumb)
    current_patch: String,

    /// Set of expanded items within the tree (for nested expansion)
    expanded_tree_items: std::collections::HashSet<String>,

    /// Sidebar width in pixels
    sidebar_width: f32,

    /// Indent per tree level in pixels
    indent_pixels: f32,

    /// Show indent guide lines
    show_guides: bool,
}

impl Default for AnchorTreeNavigator {
    fn default() -> Self {
        Self::new()
    }
}

impl AnchorTreeNavigator {
    /// Create a new anchor tree navigator with default settings
    pub fn new() -> Self {
        Self::with_settings(250.0, 10.0, true)
    }

    /// Create a new anchor tree navigator with custom settings
    pub fn with_settings(sidebar_width: f32, indent_pixels: f32, show_guides: bool) -> Self {
        Self {
            current_patch: String::new(),
            expanded_tree_items: std::collections::HashSet::new(),
            sidebar_width,
            indent_pixels,
            show_guides,
        }
    }

    /// Set the current patch and focus on it in the tree
    pub fn set_current_patch(&mut self, patch_name: String, patches: &HashMap<String, Patch>) {
        self.current_patch = patch_name.clone();
        self.focus(&patch_name, patches);
    }

    /// Get the current patch name
    pub fn current_patch(&self) -> &str {
        &self.current_patch
    }

    /// Focus on a patch: collapse everything, then expand path to patch + its children
    pub fn focus(&mut self, patch_name: &str, patches: &HashMap<String, Patch>) {
        // Clear all expansions
        self.expanded_tree_items.clear();

        // Get the full path from orphans to this patch
        let full_path = get_patch_path(patch_name, patches);

        // Expand orphans (root)
        self.expanded_tree_items.insert("orphans".to_string());

        // Expand all nodes in the path to this patch (normalize to lowercase for consistency)
        for path_patch in &full_path {
            self.expanded_tree_items.insert(path_patch.to_lowercase());
        }

        // Expand the patch itself to show its immediate children (normalize to lowercase)
        self.expanded_tree_items.insert(patch_name.to_lowercase());
    }

    /// Collapse one level of tree expansion (for Escape key handling)
    pub fn collapse_one_level(&mut self) {
        // Find the deepest expanded item and collapse it
        // For simplicity, just clear the last expanded item
        if let Some(last) = self.expanded_tree_items.iter().last().cloned() {
            self.expanded_tree_items.remove(&last);
        }
    }

    /// Check if any tree items are expanded beyond the auto-expanded path
    pub fn is_tree_expanded(&self) -> bool {
        !self.expanded_tree_items.is_empty()
    }

    /// Render the breadcrumb and sidebar navigation widget
    /// Returns Some(patch_name) if user navigated to a new patch
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        patches: &HashMap<String, Patch>,
    ) -> Option<String> {
        let mut navigate_to: Option<String> = None;

        // Get the breadcrumb path for current patch
        let breadcrumb_path = get_patch_path(&self.current_patch, patches);

        // Always show "orphans" as the root
        let mut full_path = vec!["orphans".to_string()];
        full_path.extend(breadcrumb_path.clone());

        // Render breadcrumb row
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            for (i, breadcrumb_name) in full_path.iter().enumerate() {
                // Add arrow separator (except before first item)
                if i > 0 {
                    ui.label("→");
                }

                // Determine if this is the current patch
                let is_current = i == full_path.len() - 1;

                // Just display breadcrumbs (no interaction needed - use tree to navigate)
                if is_current {
                    ui.strong(breadcrumb_name);
                } else {
                    ui.label(breadcrumb_name);
                }
            }
        });

        navigate_to
    }

    /// Render the tree sidebar (should be called in a side panel)
    /// Returns (clicked_patch, hovered_patch) tuple
    /// - clicked_patch: Some(patch) if user clicked to navigate
    /// - hovered_patch: Some(patch) if user is hovering over a tree item
    pub fn show_sidebar(
        &mut self,
        ui: &mut egui::Ui,
        patches: &HashMap<String, Patch>,
    ) -> (Option<String>, Option<String>) {
        let mut navigate_to: Option<String> = None;
        let mut hovered: Option<String> = None;

        // Get the full path to current patch (from orphans to current)
        let full_path = get_patch_path(&self.current_patch, patches);
        let full_path_set: std::collections::HashSet<String> = full_path.iter().cloned().collect();

        // Build breadcrumb list: orphans -> parent1 -> parent2 -> current
        let mut breadcrumbs = vec!["orphans".to_string()];
        breadcrumbs.extend(full_path.clone());

        // Render vertical breadcrumb list (all parents with up arrows)
        for (i, breadcrumb) in breadcrumbs.iter().enumerate() {
            let is_current = i == breadcrumbs.len() - 1;

            if !is_current {
                // Parent nodes: show up arrow (using simple caret ^)
                ui.horizontal(|ui| {
                    let label = format!("^ {}", breadcrumb);
                    let response = ui.selectable_label(false, label);
                    if response.clicked() {
                        navigate_to = Some(breadcrumb.clone());
                    }
                    if response.hovered() {
                        hovered = Some(breadcrumb.clone());
                    }
                    // Fill remaining space
                    ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                });
            } else {
                // Current node: show with expand/collapse triangle
                let children = if breadcrumb.to_lowercase() == "orphans" {
                    // Orphans root shows all patches whose parent is "orphans"
                    let mut root_patches = Vec::new();
                    for (_, patch) in patches {
                        if let Some(parent) = patch.parent_patch_name() {
                            if parent.to_lowercase() == "orphans" {
                                root_patches.push(patch.name.clone());
                            }
                        }
                    }
                    root_patches.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                    root_patches
                } else if let Some(patch) = get_patch(breadcrumb, patches) {
                    patch.children_patch_names(patches)
                } else {
                    Vec::new()
                };

                let has_children = !children.is_empty();
                let is_expanded = self.expanded_tree_items.contains(&breadcrumb.to_lowercase());

                // Show current node with triangle
                ui.horizontal(|ui| {
                    let arrow = if has_children {
                        if is_expanded { "⏷" } else { "⏵" }
                    } else {
                        ""
                    };
                    let label = if arrow.is_empty() {
                        breadcrumb.to_string()
                    } else {
                        format!("{} {}", arrow, breadcrumb)
                    };

                    let response = ui.selectable_label(true, label);
                    if response.clicked() && has_children {
                        // Toggle expansion (normalize to lowercase)
                        let breadcrumb_lower = breadcrumb.to_lowercase();
                        if is_expanded {
                            self.expanded_tree_items.remove(&breadcrumb_lower);
                        } else {
                            self.expanded_tree_items.insert(breadcrumb_lower);
                        }
                    }
                    if response.hovered() {
                        hovered = Some(breadcrumb.clone());
                    }
                    // Fill remaining space
                    ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                });

                // Show children if expanded (OUTSIDE horizontal block)
                if is_expanded && has_children {
                    for child_name in children {
                        let (nav, child_hover) = self.show_tree_node(
                            ui,
                            &child_name,
                            &full_path_set,
                            patches,
                            1, // Indent children one level
                        );
                        if let Some(nav_patch) = nav {
                            navigate_to = Some(nav_patch);
                        }
                        if child_hover.is_some() {
                            hovered = child_hover;
                        }
                    }
                }
            }
        }

        (navigate_to, hovered)
    }

    /// Render a single tree node with simple click-to-navigate behavior
    /// Returns (clicked_patch, hovered_patch) tuple
    fn show_tree_node(
        &mut self,
        ui: &mut egui::Ui,
        node_name: &str,
        full_path_set: &std::collections::HashSet<String>,
        patches: &HashMap<String, Patch>,
        depth: usize,
    ) -> (Option<String>, Option<String>) {
        let mut navigate_to: Option<String> = None;
        let mut hovered: Option<String> = None;

        // Get children for this node
        let children = if node_name.to_lowercase() == "orphans" {
            // Special case: orphans root shows all patches whose parent is "orphans"
            let mut root_patches = Vec::new();
            for (_, patch) in patches {
                if let Some(parent) = patch.parent_patch_name() {
                    if parent.to_lowercase() == "orphans" {
                        root_patches.push(patch.name.clone());
                    }
                }
            }
            root_patches.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            root_patches
        } else if let Some(patch) = get_patch(node_name, patches) {
            patch.children_patch_names(patches)
        } else {
            Vec::new()
        };

        let has_children = !children.is_empty();
        let is_expanded = self.expanded_tree_items.contains(&node_name.to_lowercase());
        let is_in_path = full_path_set.contains(node_name) || node_name.to_lowercase() == "orphans";
        let is_current = node_name == self.current_patch;

        // Render the node - use full available width
        ui.horizontal(|ui| {
            // Add indentation for tree depth
            if depth > 0 {
                let indent_width = self.indent_pixels * depth as f32;
                ui.add_space(indent_width);
            }

            // Build label text with arrow prefix if has children
            let label_text = if has_children {
                // Arrow is part of the text to keep alignment consistent
                let arrow = if is_expanded { "⏷" } else { "⏵" };
                format!("{} {}", arrow, node_name)
            } else {
                // No children - no arrow, no prefix, aligns with sibling arrows
                node_name.to_string()
            };

            // Create selectable label that fills available width but left-aligns text
            let item_response = ui.selectable_label(is_current, label_text);

            // Click on item toggles if it's already current, otherwise navigates to it
            if item_response.clicked() {
                if is_current && has_children {
                    // Already current - toggle expansion (normalize to lowercase)
                    let node_name_lower = node_name.to_lowercase();
                    if is_expanded {
                        self.expanded_tree_items.remove(&node_name_lower);
                    } else {
                        self.expanded_tree_items.insert(node_name_lower);
                    }
                } else {
                    // Not current - navigate to it (focus)
                    navigate_to = Some(node_name.to_string());
                }
            }

            // Capture hover state
            if item_response.hovered() {
                hovered = Some(node_name.to_string());
            }
        });

        // Show children if expanded (entire tree structure, just collapsed/expanded)
        if is_expanded && has_children {
            for child_name in children {
                let (nav, child_hover) = self.show_tree_node(
                    ui,
                    &child_name,
                    full_path_set,
                    patches,
                    depth + 1,
                );
                if let Some(nav_patch) = nav {
                    navigate_to = Some(nav_patch);
                }
                if child_hover.is_some() {
                    hovered = child_hover;
                }
            }
        }

        (navigate_to, hovered)
    }
}
