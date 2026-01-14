//! Window Positioning Utilities
//!
//! Functions for managing window position persistence and screen positioning.

use eframe::egui;
use crate::prelude::*;

#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;

/// Save window position to persistent state with a reason for debugging.
pub fn save_window_position_with_reason(pos: egui::Pos2, reason: &str) {
    let mut state = crate::core::data::get_state();
    let old_pos = state.window_position;

    // Log the save with reason and check for drift
    let msg = if pos.y > 600.0 {
        format!("[POSITION-SAVE] WARNING: Saving low position ({}, {}) from {:?} - reason: {} - POSITION TOO LOW!",
                pos.x, pos.y, old_pos, reason)
    } else {
        format!("[POSITION-SAVE] Saving position ({}, {}) from {:?} - reason: {}",
                pos.x, pos.y, old_pos, reason)
    };
    log(&msg);

    state.window_position = Some((pos.x, pos.y));
    match crate::core::data::set_state(&state) {
        Ok(_) => {
            detailed_log("WINDOW_POS", "Successfully saved position");
        }
        Err(e) => {
            log(&format!("[POSITION-SAVE] ERROR: Failed to save position: {}", e));
        }
    }
}

/// Load window position from persistent state.
pub fn load_window_position() -> Option<egui::Pos2> {
    let state = crate::core::data::get_state();
    let result = state.window_position.map(|(x, y)| egui::pos2(x, y));

    if let Some(pos) = result {
        if pos.y > 600.0 {
            detailed_log("POSITION-LOAD", &format!("WARNING: Loading low position ({}, {}) - POSITION TOO LOW!", pos.x, pos.y));
        } else {
            detailed_log("POSITION-LOAD", &format!("Loading saved position ({}, {})", pos.x, pos.y));
        }
    } else {
        detailed_log("POSITION-LOAD", "No saved position available");
    }

    result
}

/// Get the previous window location, or center on screen if not available/visible.
pub fn get_previous_window_location(ctx: &egui::Context, window_size: egui::Vec2) -> egui::Pos2 {
    detailed_log("WINDOW_POS", &format!("get_previous_window_location called with window_size: {:?}", window_size));

    // First try to load the previous position
    if let Some(previous_pos) = load_window_position() {
        detailed_log("WINDOW_POS", &format!("Loaded previous position: {:?}", previous_pos));
        // Check if this position is visible on any current display
        if is_position_visible(previous_pos, window_size) {
            detailed_log("WINDOW_POS", "Position is visible, returning it");
            return previous_pos;
        } else {
            detailed_log("WINDOW_POS", "Position NOT visible, will center instead");
        }
    } else {
        detailed_log("WINDOW_POS", "No previous position found in state");
    }

    // If no previous position or not visible, center on main display
    let centered = center_on_main_display(ctx, window_size);
    detailed_log("WINDOW_POS", &format!("Centering on main display at: {:?}", centered));
    centered
}

/// Check if a window position is visible on the current display.
pub fn is_position_visible(pos: egui::Pos2, window_size: egui::Vec2) -> bool {
    detailed_log("WINDOW_POS", &format!("is_position_visible checking pos: {:?}, window_size: {:?}", pos, window_size));

    let window_rect = egui::Rect::from_min_size(pos, window_size);
    detailed_log("WINDOW_POS", &format!("Window rect: {:?}", window_rect));

    // Get actual screen dimensions instead of hardcoded values
    let (screen_width, screen_height) = get_actual_screen_dimensions();
    let main_display_rect = egui::Rect::from_min_size(
        egui::pos2(0.0, 0.0),
        egui::vec2(screen_width, screen_height)
    );
    detailed_log("WINDOW_POS", &format!("Main display rect: {:?} (actual screen: {}x{})", main_display_rect, screen_width, screen_height));

    // Check if at least part of the window is visible
    // Allow for window to be partially off-screen but require some overlap
    let min_visible_area = window_size.x.min(window_size.y) * 0.3; // 30% of smaller dimension

    let intersection = main_display_rect.intersect(window_rect);
    let is_visible = intersection.width() * intersection.height() >= min_visible_area * min_visible_area;
    detailed_log("WINDOW_POS", &format!("Position visible check: {} (intersection: {:?})", is_visible, intersection));
    is_visible
}

/// Get the actual screen dimensions (not window dimensions).
pub fn get_actual_screen_dimensions() -> (f32, f32) {
    #[cfg(target_os = "macos")]
    {
        let display = CGDisplay::main();
        let width = display.pixels_wide() as f32;
        let height = display.pixels_high() as f32;
        detailed_log("SYSTEM", &format!("🔵 SCREEN: Got actual screen dimensions from CoreGraphics: {}x{}", width, height));
        (width, height)
    }

    #[cfg(not(target_os = "macos"))]
    {
        detailed_log("SYSTEM", "🔵 SCREEN: Using default screen dimensions (not macOS)");
        (1920.0, 1080.0)
    }
}

/// Center a window on the main display.
pub fn center_on_main_display(_ctx: &egui::Context, window_size: egui::Vec2) -> egui::Pos2 {
    // Get actual screen dimensions
    let (screen_width, screen_height) = get_actual_screen_dimensions();
    let display_size = egui::vec2(screen_width, screen_height);

    // Center the window on the display
    egui::pos2(
        (display_size.x - window_size.x) / 2.0,
        (display_size.y - window_size.y) / 2.0
    )
}
