//! GUI installer for HookAnchor
//!
//! Provides a user-friendly graphical interface for installing HookAnchor components
//! with checkboxes to select which components to install and visual status indicators.

use eframe::egui;
use std::sync::mpsc;
use std::thread;
use hookanchor::systems::SetupAssistant;

/// Installation status for each component
#[derive(Debug, Clone, PartialEq)]
enum InstallStatus {
    NotInstalled,     // Component not installed - show unchecked checkbox
    Installed,        // Component already installed - show green checkmark + "Installed"
    Selected,         // Component selected for installation - show checked checkbox
    Installing,       // Currently being installed - show spinner
    InstallSuccess,   // Just successfully installed - show green checkmark
    InstallFailed(String), // Installation failed - show red X with error
}

/// Karabiner-Elements installation and configuration status
#[derive(Debug, Clone, PartialEq)]
enum KarabinerStatus {
    NotInstalled,           // Karabiner-Elements not installed - show X and download link
    InstalledNotConfigured, // Installed but HookAnchor not configured - can configure
    InstalledAndConfigured, // Installed and HookAnchor configured - show checkmark
}

/// Individual installation component
#[derive(Debug, Clone)]
struct InstallComponent {
    name: String,
    description: String,
    status: InstallStatus,
    required: bool,  // Some components might be required
}

/// Main installer GUI application
struct InstallerGui {
    setup_assistant: SetupAssistant,
    components: Vec<InstallComponent>,
    installation_in_progress: bool,
    install_receiver: Option<mpsc::Receiver<(usize, Result<(), String>)>>,
    status_messages: Vec<String>,
}

impl InstallerGui {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let setup_assistant = SetupAssistant::new();

        let mut installer = Self {
            setup_assistant,
            components: Vec::new(),
            installation_in_progress: false,
            install_receiver: None,
            status_messages: Vec::new(),
        };

        installer.initialize_components();
        installer
    }

    /// Initialize the list of installable components
    fn initialize_components(&mut self) {
        self.components = vec![
            InstallComponent {
                name: "Karabiner-Elements Application".to_string(),
                description: "Status of Karabiner-Elements installation (required for hotkeys)".to_string(),
                status: InstallStatus::Selected,  // Default to selected
                required: false,
            },
            InstallComponent {
                name: "Karabiner-Elements Configuration".to_string(),
                description: "Pre-install Caps Lock hotkey configuration for HookAnchor".to_string(),
                status: InstallStatus::Selected,  // Default to selected
                required: false,
            },
            InstallComponent {
                name: "Configuration Files".to_string(),
                description: "All config files: config.yaml, config.js, shell integration, commands.txt, uninstaller, and distribution templates".to_string(),
                status: InstallStatus::Selected,  // Default to selected
                required: true,
            },
            InstallComponent {
                name: "Shell Integration Status".to_string(),
                description: "User added shell integration to ~/.zshrc".to_string(),
                status: InstallStatus::Selected,  // Default to selected
                required: false,
            },
            InstallComponent {
                name: "Accessibility Permissions".to_string(),
                description: "Required for grabber to capture URLs from Obsidian, Notion, etc.".to_string(),
                status: InstallStatus::Selected,  // Default to selected
                required: true,
            },
        ];
    }


    /// Check if shell integration is added to ~/.zshrc
    fn check_shell_integration_status(&self) -> bool {
        // Try multiple ways to find .zshrc
        let paths_to_check = vec![
            // Try dirs::home_dir() first
            dirs::home_dir().map(|h| h.join(".zshrc")),
            // Also try $HOME env var as fallback
            std::env::var("HOME").ok().map(|h| std::path::PathBuf::from(h).join(".zshrc")),
            // Try explicit path as last resort using whoami
            std::env::var("USER").ok().map(|u|
                std::path::PathBuf::from(format!("/Users/{}", u)).join(".zshrc")
            ),
        ];

        for path_option in paths_to_check {
            if let Some(path) = path_option {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    // Look for the exact source line we want users to add
                    // Also check for variations with extra spaces
                    if content.contains("source ~/.config/hookanchor/hook_anchor_zshrc") ||
                       content.contains("source  ~/.config/hookanchor/hook_anchor_zshrc") ||
                       content.contains("source\t~/.config/hookanchor/hook_anchor_zshrc") {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Get the exact line users should add to their ~/.zshrc
    fn get_shell_integration_line(&self) -> &'static str {
        "source ~/.config/hookanchor/hook_anchor_zshrc"
    }

    /// Check if Karabiner-Elements is installed and optionally configured
    fn check_karabiner_status(&self) -> KarabinerStatus {
        // Simply check if the application exists - this is what users expect
        // When they "uninstall" an app on macOS, they delete it from /Applications
        let app_installed = std::path::Path::new("/Applications/Karabiner-Elements.app").exists();

        if !app_installed {
            return KarabinerStatus::NotInstalled;
        }

        // Check if HookAnchor is configured
        let karabiner_dir = dirs::home_dir()
            .map(|h| h.join(".config/karabiner/assets/complex_modifications"))
            .filter(|p| p.exists());

        if let Some(dir) = karabiner_dir {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.starts_with("hookanchor") && name.ends_with(".json") {
                            return KarabinerStatus::InstalledAndConfigured;
                        }
                    }
                }
            }
        }

        KarabinerStatus::InstalledNotConfigured
    }

    /// Start installation of selected components
    fn start_installation(&mut self) {
        if self.installation_in_progress {
            return;
        }

        self.installation_in_progress = true;
        self.status_messages.clear();
        self.status_messages.push("Starting installation...".to_string());

        // Mark selected components as installing
        for component in &mut self.components {
            if component.status == InstallStatus::Selected {
                component.status = InstallStatus::Installing;
            }
        }

        // Create channel for receiving installation results
        let (tx, rx) = mpsc::channel();
        self.install_receiver = Some(rx);

        // Clone data for background thread
        let selected_components: Vec<(usize, bool)> = self.components
            .iter()
            .enumerate()
            .map(|(i, comp)| (i, comp.status == InstallStatus::Installing))
            .collect();

        // Start installation in background thread
        thread::spawn(move || {
            let setup_assistant = SetupAssistant::new();

            for (index, should_install) in selected_components {
                if !should_install {
                    continue;
                }

                let result = match index {
                    0 => {
                        // Karabiner-Elements Application - this is just a status display, not installable
                        Err("Karabiner-Elements must be installed manually from https://karabiner-elements.pqrs.org/".to_string())
                    },
                    1 => setup_assistant.install_karabiner_modification()
                        .map_err(|e| e.to_string()),
                    2 => {
                        // Configuration Files - install ALL config files with their individual logic
                        let mut errors = Vec::new();

                        // Install each config file with its own logic and rules
                        if let Err(e) = setup_assistant.create_default_config() {
                            errors.push(format!("config.yaml: {}", e));
                        }
                        if let Err(e) = setup_assistant.install_config_js() {
                            errors.push(format!("config.js: {}", e));
                        }
                        if let Err(e) = setup_assistant.install_hook_anchor_zshrc() {
                            errors.push(format!("shell integration: {}", e));
                        }
                        if let Err(e) = setup_assistant.install_commands_txt() {
                            errors.push(format!("commands.txt: {}", e));
                        }
                        if let Err(e) = setup_assistant.install_uninstaller_app() {
                            errors.push(format!("uninstaller: {}", e));
                        }
                        if let Err(e) = setup_assistant.install_latest_dist_files() {
                            errors.push(format!("distribution templates: {}", e));
                        }

                        if errors.is_empty() {
                            Ok(())
                        } else {
                            Err(format!("Some config files failed: {}", errors.join(", ")))
                        }
                    },
                    3 => {
                        // Shell Integration Status - this is just a status display, no action needed
                        Ok(())
                    },
                    4 => {
                        // Accessibility Permissions - trigger the permission check
                        match SetupAssistant::ensure_accessibility_permissions() {
                            Ok(true) => Ok(()),
                            Ok(false) => Err("User needs to grant accessibility permissions manually".to_string()),
                            Err(e) => Err(e.to_string()),
                        }
                    },
                    _ => Err("Unknown component".to_string()),
                };

                // Send result back to main thread
                if tx.send((index, result)).is_err() {
                    break; // Main thread disconnected
                }

                // Small delay to show progress
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        });
    }

    /// Check for installation results from background thread
    fn check_installation_progress(&mut self) {
        if let Some(receiver) = &self.install_receiver {
            while let Ok((component_index, result)) = receiver.try_recv() {
                if let Some(component) = self.components.get_mut(component_index) {
                    match result {
                        Ok(_) => {
                            component.status = InstallStatus::InstallSuccess;
                            self.status_messages.push(format!("✓ Added {}", component.name));
                        }
                        Err(error) => {
                            component.status = InstallStatus::InstallFailed(error.clone());
                            self.status_messages.push(format!("✗ {}: {}", component.name, error));
                        }
                    }
                }
            }

            // Check if installation is complete
            let still_installing = self.components.iter()
                .any(|comp| comp.status == InstallStatus::Installing);

            if !still_installing {
                self.installation_in_progress = false;
                self.install_receiver = None;
                self.status_messages.push("Installation complete!".to_string());

                // Remove install marker file - no more installer needed on future runs
                if let Ok(exe_path) = std::env::current_exe() {
                    let marker_path = exe_path
                        .parent() // MacOS
                        .and_then(|p| p.parent()) // Contents
                        .map(|p| p.join("Resources").join("install_pending"));

                    if let Some(marker) = marker_path {
                        if marker.exists() {
                            if let Err(e) = std::fs::remove_file(&marker) {
                                self.status_messages.push(format!("Warning: Could not remove install marker: {}", e));
                            } else {
                                self.status_messages.push("✓ Install marker removed - app will skip installer on future runs".to_string());
                            }
                        }
                    }
                }

                // Convert InstallSuccess to Installed after a moment
                for component in &mut self.components {
                    if component.status == InstallStatus::InstallSuccess {
                        component.status = InstallStatus::Installed;
                    }
                }
            }
        }
    }

    /// Check if installation is complete (all components are installed or failed)
    fn installation_complete(&self) -> bool {
        !self.installation_in_progress &&
        self.components.iter().all(|comp| {
            matches!(comp.status,
                InstallStatus::Installed |
                InstallStatus::InstallSuccess |
                InstallStatus::InstallFailed(_)
            )
        })
    }

    /// Render a component checkbox with appropriate status
    fn render_component(&mut self, ui: &mut egui::Ui, index: usize) {
        // Get karabiner status before borrowing components
        let karabiner_status = if index == 0 || index == 1 {
            Some(self.check_karabiner_status())
        } else {
            None
        };

        // Get shell integration status and command before borrowing components
        let shell_integration_data = if index == 3 {
            Some((self.check_shell_integration_status(), self.get_shell_integration_line()))
        } else {
            None
        };

        if let Some(component) = self.components.get_mut(index) {
            // Special handling for Karabiner-Elements components (index 0 and 1)
            if let Some(status) = karabiner_status {
                if index == 0 {
                    // Karabiner-Elements Application status
                    Self::render_karabiner_app_status_static(ui, component, status);
                } else {
                    // Karabiner-Elements Configuration - treat like regular component
                    Self::render_regular_component_static(ui, component);
                }
            } else if let Some((shell_status, shell_command)) = shell_integration_data {
                // Shell Integration Status (special handling)
                Self::render_shell_integration_status_static(ui, component, shell_status, shell_command);
            } else {
                Self::render_regular_component_static(ui, component);
            }
        }
    }

    /// Render shell integration status (read-only with command line always shown)
    fn render_shell_integration_status_static(ui: &mut egui::Ui, _component: &InstallComponent, shell_integrated: bool, shell_command: &str) {
        ui.horizontal(|ui| {
            if shell_integrated {
                ui.label(egui::RichText::new("✅").size(20.0).strong().color(egui::Color32::GREEN));
                ui.label("User added shell integration to ~/.zshrc");
            } else {
                ui.label(egui::RichText::new("❌").size(20.0).strong().color(egui::Color32::RED));
                ui.label("User can optionally add this line for shell integration (typically to ~/.zshrc):");
            }
        });

        // Always show the command line indented
        ui.indent("shell_command", |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(shell_command)
                    .family(egui::FontFamily::Monospace)
                    .size(11.0)
                    .background_color(egui::Color32::LIGHT_GRAY));
            });
        });

        ui.add_space(8.0);
    }

    /// Render Karabiner application status (read-only)
    fn render_karabiner_app_status_static(ui: &mut egui::Ui, component: &mut InstallComponent, karabiner_status: KarabinerStatus) {
        ui.horizontal(|ui| {
            match karabiner_status {
                KarabinerStatus::NotInstalled => {
                    ui.label(egui::RichText::new("❌").size(20.0).strong().color(egui::Color32::RED));
                    ui.label(egui::RichText::new("Karabiner-Elements - NOT INSTALLED")
                        .color(egui::Color32::RED));
                }
                KarabinerStatus::InstalledNotConfigured => {
                    ui.label(egui::RichText::new("✅").size(20.0).strong().color(egui::Color32::GREEN));
                    ui.label("Karabiner-Elements - Now Installed");
                }
                KarabinerStatus::InstalledAndConfigured => {
                    ui.label(egui::RichText::new("✅").size(20.0).strong().color(egui::Color32::GREEN));
                    ui.label("Karabiner-Elements - Now Installed");
                }
            }
        });

        // Show description and download link if needed
        ui.indent("karabiner_app_desc", |ui| {
            match karabiner_status {
                KarabinerStatus::NotInstalled => {
                    ui.label(egui::RichText::new("Karabiner-Elements is required for hotkey functionality")
                        .size(12.0)
                        .color(egui::Color32::RED));
                    ui.horizontal(|ui| {
                        ui.label("Download from:");
                        if ui.link("https://karabiner-elements.pqrs.org/").clicked() {
                            // Open link in browser
                            let _ = std::process::Command::new("open")
                                .arg("https://karabiner-elements.pqrs.org/")
                                .spawn();
                        }
                    });
                }
                _ => {
                    ui.label(egui::RichText::new(&component.description)
                        .size(12.0)
                        .color(egui::Color32::GRAY));
                }
            }
        });

        ui.add_space(8.0);
    }


    /// Render regular component (non-Karabiner)
    fn render_regular_component_static(ui: &mut egui::Ui, component: &mut InstallComponent) {
        ui.horizontal(|ui| {
            match &component.status {
                InstallStatus::Installed => {
                    // Green checkmark for installed components
                    ui.label(egui::RichText::new("✅").size(20.0).strong().color(egui::Color32::GREEN));
                    ui.label(format!("{} - Now Installed", component.name));
                }
                InstallStatus::Installing => {
                    // Spinner for components being installed
                    ui.spinner();
                    ui.label(format!("{} - Installing...", component.name));
                }
                InstallStatus::InstallSuccess => {
                    // Green checkmark for just completed
                    ui.label(egui::RichText::new("✅").size(20.0).strong().color(egui::Color32::GREEN));
                    ui.label(format!("{} - Installed", component.name));
                }
                InstallStatus::InstallFailed(error) => {
                    // Red X for failed installations
                    ui.label(egui::RichText::new("❌").size(20.0).strong().color(egui::Color32::RED));
                    ui.label(egui::RichText::new(format!("{} - Failed", component.name))
                        .color(egui::Color32::RED));
                    if ui.small_button("?").on_hover_text(error).clicked() {
                        // Show error details on hover
                    }
                }
                _ => {
                    // Regular checkbox for not installed/selected
                    let mut selected = component.status == InstallStatus::Selected;
                    if ui.checkbox(&mut selected, &component.name).changed() {
                        component.status = if selected {
                            InstallStatus::Selected
                        } else {
                            InstallStatus::NotInstalled
                        };
                    }
                }
            }
        });

        // Show description
        ui.indent("component_desc", |ui| {
            ui.label(egui::RichText::new(&component.description)
                .size(12.0)
                .color(egui::Color32::GRAY));
        });

        ui.add_space(8.0);
    }
}

impl eframe::App for InstallerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_installation_progress();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&format!("HookAnchor Installer v{}", env!("CARGO_PKG_VERSION")));
            ui.add_space(10.0);

            ui.label("Select the components you want to install:");
            ui.add_space(10.0);

            // Render each component
            for i in 0..self.components.len() {
                self.render_component(ui, i);
            }

            ui.add_space(20.0);

            // Installation button
            ui.horizontal(|ui| {
                let any_selected = self.components.iter()
                    .any(|comp| comp.status == InstallStatus::Selected);

                // Show different buttons based on installation state
                if self.installation_in_progress {
                    ui.spinner();
                    ui.label("Installing...");
                } else if !self.installation_complete() {
                    // Show Install and Cancel buttons before installation
                    // Install button is always enabled so users can remove pending flag
                    if ui.button("Install").clicked() {
                        self.start_installation();
                    }

                    ui.add_space(10.0);

                    if ui.button("Cancel").clicked() {
                        std::process::exit(0);
                    }
                }
            });

            ui.add_space(20.0);

            // Status messages (always visible with fixed size)
            ui.separator();
            ui.label("Installation Status:");
            ui.add_space(5.0);

            egui::ScrollArea::vertical()
                .max_height(120.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if self.status_messages.is_empty() {
                        // Check if all required components are already installed
                        let all_required_installed = self.components.iter()
                            .filter(|comp| comp.required)
                            .all(|comp| comp.status == InstallStatus::Installed);

                        if all_required_installed {
                            ui.label(egui::RichText::new("Installation complete!")
                                .italics()
                                .color(egui::Color32::DARK_GREEN));
                        } else {
                            ui.label(egui::RichText::new("Ready to install...")
                                .italics()
                                .color(egui::Color32::GRAY));
                        }
                    } else {
                        for message in &self.status_messages {
                            ui.label(message);
                        }
                    }
                });

            // Close button always at the bottom
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                // Center the close button
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Close").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });

        // Request repaint if installation is in progress
        if self.installation_in_progress {
            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 640.0])
            .with_title("HookAnchor Installer")
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "HookAnchor Installer",
        options,
        Box::new(|cc| Ok(Box::new(InstallerGui::new(cc)))),
    )
}