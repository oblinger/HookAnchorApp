//! Popup Server Wrapper
//!
//! This module wraps AnchorSelector with PopupControl to handle server
//! communication (show/hide/set_input commands from the popup_server).
//!
//! Architecture:
//! - `AnchorSelector` (in popup.rs) = the core UI and state machine
//! - `PopupWithControl` (this file) = wrapper that handles external control commands
//! - `PopupControl` (in systems/popup_server.rs) = the server communication layer

use crate::prelude::*;
use crate::ui::popup::AnchorSelector;
use crate::ui::window_utils::{load_window_position, get_actual_screen_dimensions};

/// Wrapper that combines the popup UI with server control.
///
/// This struct wraps AnchorSelector and processes commands from the popup server
/// (show, hide, set_input) before delegating to the inner popup.
pub(crate) struct PopupWithControl {
    popup: AnchorSelector,
    control: crate::systems::popup_server::PopupControl,
}

impl PopupWithControl {
    fn new(initial_prompt: &str, initial_action: Option<&str>) -> Self {
        let control = crate::systems::popup_server::PopupControl::new();
        control.start_listener();

        Self {
            popup: AnchorSelector::new_with_prompt_and_action(initial_prompt, initial_action),
            control,
        }
    }
}

impl eframe::App for PopupWithControl {
    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        self.popup.clear_color(visuals)
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Process any control commands first
        if let Some(command) = self.control.process_commands(ctx) {
            match command {
                crate::systems::popup_server::PopupCommand::Show => {
                    self.handle_show_command(ctx);
                }
                crate::systems::popup_server::PopupCommand::ShowWithInput(ref input) => {
                    self.handle_show_with_input_command(ctx, input);
                }
                crate::systems::popup_server::PopupCommand::Hide => {
                    detailed_log("HIDE", "===== HIDE COMMAND RECEIVED =====");
                    self.popup.exit_or_hide(ctx);
                    detailed_log("HIDE", "===== HIDE COMMAND COMPLETE =====");
                }
                crate::systems::popup_server::PopupCommand::Ping => {
                    // No special handling needed
                }
            }
        }

        // Then update the popup
        self.popup.update(ctx, frame);
    }
}

impl PopupWithControl {
    /// Handle the Show command - make window visible and focused
    fn handle_show_command(&mut self, ctx: &egui::Context) {
        let show_start = std::time::Instant::now();
        detailed_log("SHOW", "===== SHOW COMMAND RECEIVED =====");
        detailed_log("SHOW_TIMING", &format!("Command received at {:?}", show_start));
        detailed_log("SHOW", &format!(" Current frame: {}", self.popup.frame_count));
        detailed_log("SHOW", &format!(" is_hidden={}, should_exit={}",
            self.popup.is_hidden, self.popup.should_exit));

        // Store the show start time for measuring total time to focus
        self.popup.show_command_start = Some(show_start);

        // Get current window state
        let viewport_info = ctx.input(|i| {
            format!("Viewport: focused={}, visible={:?}, position={:?}",
                i.focused,
                i.viewport().inner_rect,
                i.viewport().outer_rect
            )
        });
        detailed_log("SHOW", &format!(" Before: {}", viewport_info));

        // Restore saved position BEFORE making window visible to avoid flashing at (0,0)
        if let Some(saved_pos) = load_window_position() {
            detailed_log("SHOW", &format!(" Pre-setting saved position: {:?}", saved_pos));
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(saved_pos));
        }

        // Make the window visible again if it was hidden
        detailed_log("SHOW", "Sending ViewportCommand::Visible(true)");
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));

        detailed_log("SHOW", "Sending ViewportCommand::Focus");
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);

        // Also try to bring to front
        detailed_log("SHOW", "Sending ViewportCommand::Fullscreen(false) to ensure windowed mode");
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));

        detailed_log("SHOW", "Sending ViewportCommand::Minimized(false)");
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));

        detailed_log("SHOW", "Relying on ViewportCommand::Focus (AppleScript removed for speed)");

        // Position window on screen if it's off-screen
        self.ensure_window_on_screen(ctx);

        // Clear the search input when showing the popup
        self.popup.popup_state.search_text.clear();
        self.popup.popup_state.update_search(String::new());

        // Request focus on the input field and reset focus tracking flags
        self.popup.request_focus = true;
        self.popup.focus_set = false;
        self.popup.window_activated = false;
        self.popup.frame_count = 0;

        // Mark window as not hidden and reset interaction time
        detailed_log("SHOW", &format!(" Setting is_hidden=false (was {})", self.popup.is_hidden));
        self.popup.is_hidden = false;
        detailed_log("SHOW", &format!(" Setting should_exit=false (was {})", self.popup.should_exit));
        self.popup.should_exit = false;
        self.popup.last_interaction_time = std::time::Instant::now();

        // Force a repaint
        detailed_log("SHOW", "Requesting repaint");
        ctx.request_repaint();

        detailed_log("SHOW", &format!(" After: is_hidden={}, should_exit={}",
            self.popup.is_hidden, self.popup.should_exit));
        detailed_log("SHOW_TIMING", &format!("Command processing complete in {:?}", show_start.elapsed()));
        detailed_log("SHOW", "===== SHOW COMMAND COMPLETE =====");
    }

    /// Handle the ShowWithInput command - show window with pre-filled input
    fn handle_show_with_input_command(&mut self, ctx: &egui::Context, input: &str) {
        let show_start = std::time::Instant::now();
        detailed_log("SHOW", &format!("===== SHOW_WITH_INPUT COMMAND RECEIVED: '{}' =====", input));

        self.popup.show_command_start = Some(show_start);

        // Restore saved position BEFORE making window visible
        if let Some(saved_pos) = load_window_position() {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(saved_pos));
        }

        // Make the window visible
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));

        // SET the search input to the provided text (instead of clearing it)
        self.popup.popup_state.search_text = input.to_string();
        self.popup.popup_state.update_search(input.to_string());

        // Request focus on the input field and reset focus tracking flags
        self.popup.request_focus = true;
        self.popup.focus_set = false;
        self.popup.window_activated = false;
        self.popup.frame_count = 0;

        // Mark window as not hidden
        self.popup.is_hidden = false;
        self.popup.should_exit = false;
        self.popup.last_interaction_time = std::time::Instant::now();

        ctx.request_repaint();
        detailed_log("SHOW", &format!("===== SHOW_WITH_INPUT COMMAND COMPLETE in {:?} =====", show_start.elapsed()));
    }

    /// Ensure the window is positioned on-screen, repositioning if necessary
    fn ensure_window_on_screen(&mut self, ctx: &egui::Context) {
        let current_pos = ctx.input(|i| i.viewport().outer_rect);
        if let Some(rect) = current_pos {
            let (screen_width, screen_height) = get_actual_screen_dimensions();

            detailed_log("WINDOW_POS", &format!("[SHOW] Actual screen dimensions: {}x{}",
                screen_width, screen_height));
            detailed_log("WINDOW_POS", &format!("[SHOW] Current window rect: {:?}", rect));

            // Check if window is truly off-screen
            let margin = 50.0;
            let is_offscreen = rect.min.x < -margin ||
                              rect.min.y < -margin ||
                              rect.min.x > screen_width - margin ||
                              rect.min.y > screen_height - margin;

            detailed_log("WINDOW_POS", &format!("[SHOW] Off-screen check: {}", is_offscreen));

            if is_offscreen {
                detailed_log("SHOW", &format!(" Window is off-screen at {:?}, centering", rect.min));
                let window_size = rect.size();
                let center_x = (screen_width - window_size.x) / 2.0;
                let center_y = (screen_height - window_size.y) / 3.0;
                let max_y = (screen_height - window_size.y - 100.0).max(0.0);
                let center_y = center_y.min(max_y);
                let center_pos = egui::pos2(center_x.max(0.0), center_y.max(0.0));
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(center_pos));
                self.popup.was_auto_positioned = true;
                detailed_log("SHOW", &format!(" Repositioned window to {:?}", center_pos));
            } else {
                // Window is on-screen, restore saved position if different
                if let Some(mut saved_pos) = load_window_position() {
                    let window_size = rect.size();
                    let max_y = (screen_height - window_size.y - 100.0).max(0.0);
                    if saved_pos.y > max_y {
                        detailed_log("SHOW", &format!(" Adjusting saved position from y={} to y={}", saved_pos.y, max_y));
                        saved_pos.y = max_y;
                        self.popup.was_auto_positioned = true;
                    }

                    let pos_diff = (rect.min.x - saved_pos.x).abs() + (rect.min.y - saved_pos.y).abs();
                    if pos_diff > 5.0 {
                        detailed_log("SHOW", &format!(" Restoring saved position {:?}", saved_pos));
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(saved_pos));
                        if !self.popup.was_auto_positioned {
                            self.popup.was_auto_positioned = false;
                        }
                    }
                }
            }
        } else {
            // No position available, center the window
            detailed_log("SHOW", "No window position available, centering on main display");
            let config = crate::core::data::get_config();
            let width = config.popup_settings.get_default_window_width() as f32;
            let height = config.popup_settings.get_default_window_height() as f32;
            let window_size = egui::vec2(width, height);
            let (screen_width, screen_height) = get_actual_screen_dimensions();
            let center_x = (screen_width - window_size.x) / 2.0;
            let center_y = (screen_height - window_size.y) / 3.0;
            let max_y = (screen_height - window_size.y - 100.0).max(0.0);
            let center_y = center_y.min(max_y);
            let center_pos = egui::pos2(center_x.max(0.0), center_y.max(0.0));
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(center_pos));
            detailed_log("SHOW", &format!(" Positioned new window to {:?}", center_pos));
        }
    }
}

/// Run the popup GUI with an initial prompt and optional action.
///
/// This is the main entry point for starting the popup window.
pub fn run_gui_with_prompt(initial_prompt: &str, initial_action: Option<&str>, _app_state: super::ApplicationState) -> Result<(), eframe::Error> {
    detailed_log("SYSTEM", "===== POPUP GUI STARTING =====");
    detailed_log("POPUP_OPEN", &format!("Opening popup with initial prompt: '{}', action: {:?}", initial_prompt, initial_action));

    let prompt = initial_prompt.to_string();
    let action = initial_action.map(|s| s.to_string());

    // Manual window sizing
    let config = crate::core::data::get_config();
    let width = config.popup_settings.get_default_window_width() as f32;
    let height = config.popup_settings.get_default_window_height() as f32;
    let start_visible = true;

    detailed_log("SYSTEM", &format!("[POPUP_INIT] Window size: {}x{}, start_visible: {}", width, height, start_visible));

    // Load saved position or use centered position
    let initial_position = if let Some(saved_pos) = load_window_position() {
        detailed_log("WINDOW_POS", &format!("[STARTUP] Using saved window position: {:?}", saved_pos));
        [saved_pos.x, saved_pos.y]
    } else {
        let screen_width = 1920.0;
        let screen_height = 1080.0;
        let x = (screen_width - width) / 2.0;
        let y = (screen_height - height) / 2.0;
        detailed_log("WINDOW_POS", &format!("[STARTUP] No saved position, centering at: [{}, {}]", x, y));
        [x, y]
    };

    let viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size([width, height])
        .with_position(initial_position)
        .with_resizable(false)
        .with_decorations(false)
        .with_transparent(true)
        .with_visible(start_visible)
        .with_always_on_top();

    detailed_log("WINDOW_POS", &format!("[VIEWPORT] ViewportBuilder configured with position: {:?}", initial_position));

    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        renderer: eframe::Renderer::Glow,
        run_and_return: false,
        vsync: true,
        ..Default::default()
    };

    // Set activation policy right before running
    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSApplication, NSApplicationActivationPolicy};
        use cocoa::base::nil;

        unsafe {
            let app = NSApplication::sharedApplication(nil);
            app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
            detailed_log("STARTUP", "Re-applied Accessory activation policy before eframe::run_native");
        }
    }

    eframe::run_native(
        "Anchor Selector",
        options,
        Box::new(move |cc| {
            // Set light theme
            cc.egui_ctx.set_visuals(egui::Visuals::light());

            // Configure for minimal CPU usage
            let mut style = (*cc.egui_ctx.style()).clone();
            style.animation_time = 0.0;

            // Disable hover visuals completely
            style.visuals.widgets.hovered = style.visuals.widgets.inactive.clone();
            style.visuals.widgets.active = style.visuals.widgets.inactive.clone();
            style.visuals.widgets.open = style.visuals.widgets.inactive.clone();

            // Disable background fill for all widget states
            style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.open.weak_bg_fill = egui::Color32::TRANSPARENT;

            style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.hovered.bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.active.bg_fill = egui::Color32::TRANSPARENT;
            style.visuals.widgets.open.bg_fill = egui::Color32::TRANSPARENT;

            cc.egui_ctx.set_style(style);

            // Set transparent background for corner areas
            if let Some(gl) = cc.gl.as_ref() {
                use eframe::glow::HasContext as _;
                unsafe {
                    gl.clear_color(0.0, 0.0, 0.0, 0.0);
                }
            }

            Ok(Box::new(PopupWithControl::new(&prompt, action.as_deref())))
        }),
    )
}
