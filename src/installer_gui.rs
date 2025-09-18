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
        installer.detect_installed_components();
        installer
    }

    /// Initialize the list of installable components
    fn initialize_components(&mut self) {
        self.components = vec![
            InstallComponent {
                name: "Karabiner-Elements Application".to_string(),
                description: "Status of Karabiner-Elements installation (required for hotkeys)".to_string(),
                status: InstallStatus::NotInstalled,
                required: false,
            },
            InstallComponent {
                name: "Karabiner-Elements Configuration".to_string(),
                description: "Caps Lock hotkey to launch HookAnchor".to_string(),
                status: InstallStatus::NotInstalled,
                required: false,
            },
            InstallComponent {
                name: "Configuration Files".to_string(),
                description: "Default config.yaml with HookAnchor settings".to_string(),
                status: InstallStatus::NotInstalled,
                required: true,
            },
            InstallComponent {
                name: "JavaScript Configuration".to_string(),
                description: "config.js file for custom JavaScript functions".to_string(),
                status: InstallStatus::NotInstalled,
                required: false,
            },
            InstallComponent {
                name: "Shell Integration".to_string(),
                description: "Zsh functions (ff, fp, fd) for command-line integration".to_string(),
                status: InstallStatus::NotInstalled,
                required: false,
            },
            InstallComponent {
                name: "Initial Commands".to_string(),
                description: "Starter commands and examples in commands.txt".to_string(),
                status: InstallStatus::NotInstalled,
                required: true,
            },
            InstallComponent {
                name: "Uninstaller App".to_string(),
                description: "Double-clickable Uninstaller.app for easy removal".to_string(),
                status: InstallStatus::NotInstalled,
                required: true,
            },
        ];
    }

    /// Detect which components are already installed
    fn detect_installed_components(&mut self) {
        let config_dir = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
            .join("hookanchor");

        // Collect installation status first
        let mut statuses = Vec::new();
        for i in 0..self.components.len() {
            let is_installed = match i {
                0 => {
                    // Karabiner-Elements Application - check if installed
                    let karabiner_status = self.check_karabiner_status();
                    !matches!(karabiner_status, KarabinerStatus::NotInstalled)
                },
                1 => {
                    // Karabiner-Elements Configuration - check if configured
                    let karabiner_status = self.check_karabiner_status();
                    matches!(karabiner_status, KarabinerStatus::InstalledAndConfigured)
                },
                2 => config_dir.join("config.yaml").exists(),
                3 => config_dir.join("config.js").exists(),
                4 => config_dir.join("hook_anchor_zshrc").exists(),
                5 => config_dir.join("commands.txt").exists(),
                6 => config_dir.join("Uninstaller.app").exists(),
                _ => false,
            };
            statuses.push(is_installed);
        }

        // Then update components
        for (i, component) in self.components.iter_mut().enumerate() {
            let is_installed = statuses[i];
            if is_installed {
                component.status = InstallStatus::Installed;
            } else if component.required {
                component.status = InstallStatus::Selected;
            } else {
                component.status = InstallStatus::Selected; // Default to selected for first-time install
            }
        }
    }

    /// Check if Karabiner-Elements is installed and optionally configured
    fn check_karabiner_status(&self) -> KarabinerStatus {
        // First check if Karabiner-Elements is installed
        let app_installed = std::path::Path::new("/Applications/Karabiner-Elements.app").exists();
        let cli_installed = std::path::Path::new("/Library/Application Support/org.pqrs/Karabiner-Elements/bin/karabiner_cli").exists();

        if !app_installed && !cli_installed {
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
                    2 => setup_assistant.create_default_config()
                        .map_err(|e| e.to_string()),
                    3 => setup_assistant.install_config_js()
                        .map_err(|e| e.to_string()),
                    4 => setup_assistant.install_hook_anchor_zshrc()
                        .map_err(|e| e.to_string()),
                    5 => setup_assistant.install_commands_txt()
                        .map_err(|e| e.to_string()),
                    6 => setup_assistant.install_uninstaller_app()
                        .map_err(|e| e.to_string()),
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

        if let Some(component) = self.components.get_mut(index) {
            // Special handling for Karabiner-Elements components (index 0 and 1)
            if let Some(status) = karabiner_status {
                if index == 0 {
                    // Karabiner-Elements Application status
                    Self::render_karabiner_app_status_static(ui, component, status);
                } else {
                    // Karabiner-Elements Configuration
                    Self::render_karabiner_component_static(ui, component, status);
                }
            } else {
                Self::render_regular_component_static(ui, component);
            }
        }
    }

    /// Render Karabiner application status (read-only)
    fn render_karabiner_app_status_static(ui: &mut egui::Ui, component: &mut InstallComponent, karabiner_status: KarabinerStatus) {
        ui.horizontal(|ui| {
            match karabiner_status {
                KarabinerStatus::NotInstalled => {
                    ui.label(egui::RichText::new("❌").color(egui::Color32::RED));
                    ui.label(egui::RichText::new("Karabiner-Elements - NOT INSTALLED")
                        .color(egui::Color32::RED));
                }
                KarabinerStatus::InstalledNotConfigured => {
                    ui.label(egui::RichText::new("✅").color(egui::Color32::GREEN));
                    ui.label("Karabiner-Elements - Now Installed");
                }
                KarabinerStatus::InstalledAndConfigured => {
                    ui.label(egui::RichText::new("✅").color(egui::Color32::GREEN));
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

    /// Render Karabiner component with special status checking
    fn render_karabiner_component_static(ui: &mut egui::Ui, component: &mut InstallComponent, karabiner_status: KarabinerStatus) {

        ui.horizontal(|ui| {
            match karabiner_status {
                KarabinerStatus::NotInstalled => {
                    // Red X - Karabiner not installed
                    ui.label(egui::RichText::new("❌").color(egui::Color32::RED));
                    ui.label(egui::RichText::new("Karabiner-Elements - NOT INSTALLED")
                        .color(egui::Color32::RED));
                }
                KarabinerStatus::InstalledNotConfigured => {
                    // Regular checkbox - can configure it
                    let mut selected = component.status == InstallStatus::Selected;
                    if ui.checkbox(&mut selected, "Karabiner-Elements Configuration").changed() {
                        component.status = if selected {
                            InstallStatus::Selected
                        } else {
                            InstallStatus::NotInstalled
                        };
                    }
                }
                KarabinerStatus::InstalledAndConfigured => {
                    // Handle normal install states when configured
                    match &component.status {
                        InstallStatus::Installing => {
                            ui.spinner();
                            ui.label("Karabiner-Elements Configuration - Installing...");
                        }
                        InstallStatus::InstallSuccess => {
                            ui.label(egui::RichText::new("✅").color(egui::Color32::GREEN));
                            ui.label("Karabiner-Elements Configuration - Installed");
                        }
                        InstallStatus::InstallFailed(error) => {
                            ui.label(egui::RichText::new("❌").color(egui::Color32::RED));
                            ui.label(egui::RichText::new("Karabiner-Elements Configuration - Failed")
                                .color(egui::Color32::RED));
                            if ui.small_button("?").on_hover_text(error).clicked() {}
                        }
                        _ => {
                            ui.label(egui::RichText::new("✅").color(egui::Color32::GREEN));
                            ui.label("Karabiner-Elements Configuration - Now Configured");
                        }
                    }
                }
            }
        });

        // Show description and download link if needed
        ui.indent("component_desc", |ui| {
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
                    ui.label(egui::RichText::new("✅").color(egui::Color32::GREEN));
                    ui.label(format!("{} - Now Installed", component.name));
                }
                InstallStatus::Installing => {
                    // Spinner for components being installed
                    ui.spinner();
                    ui.label(format!("{} - Installing...", component.name));
                }
                InstallStatus::InstallSuccess => {
                    // Green checkmark for just completed
                    ui.label(egui::RichText::new("✅").color(egui::Color32::GREEN));
                    ui.label(format!("{} - Installed", component.name));
                }
                InstallStatus::InstallFailed(error) => {
                    // Red X for failed installations
                    ui.label(egui::RichText::new("❌").color(egui::Color32::RED));
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
            ui.heading("HookAnchor Installer");
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
                    if ui.add_enabled(
                        any_selected,
                        egui::Button::new("Install")
                    ).clicked() {
                        self.start_installation();
                    }

                    ui.add_space(10.0);

                    if ui.button("Cancel").clicked() {
                        std::process::exit(0);
                    }
                }
            });

            ui.add_space(20.0);

            // Status messages
            if !self.status_messages.is_empty() {
                ui.separator();
                ui.label("Installation Status:");
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        for message in &self.status_messages {
                            ui.label(message);
                        }
                    });
            }

            // Close button at the very bottom when installation is complete
            let installation_complete = !self.installation_in_progress &&
                self.components.iter().all(|comp| {
                    matches!(comp.status,
                        InstallStatus::Installed |
                        InstallStatus::InstallSuccess |
                        InstallStatus::InstallFailed(_)
                    )
                });

            if installation_complete {
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
            }
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
            .with_inner_size([500.0, 700.0])
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