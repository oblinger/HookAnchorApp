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
    force_reinstall: bool,
    installation_in_progress: bool,
    install_receiver: Option<mpsc::Receiver<(usize, Result<(), String>)>>,
    status_messages: Vec<String>,
    show_advanced: bool,
}

impl InstallerGui {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let setup_assistant = SetupAssistant::new();

        let mut installer = Self {
            setup_assistant,
            components: Vec::new(),
            force_reinstall: false,
            installation_in_progress: false,
            install_receiver: None,
            status_messages: Vec::new(),
            show_advanced: false,
        };

        installer.initialize_components();
        installer.detect_installed_components();
        installer
    }

    /// Initialize the list of installable components
    fn initialize_components(&mut self) {
        self.components = vec![
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
                0 => self.is_karabiner_configured(),
                1 => config_dir.join("config.yaml").exists(),
                2 => config_dir.join("config.js").exists(),
                3 => config_dir.join("hook_anchor_zshrc").exists(),
                4 => config_dir.join("commands.txt").exists(),
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

    /// Check if Karabiner is configured with HookAnchor
    fn is_karabiner_configured(&self) -> bool {
        let karabiner_dir = dirs::home_dir()
            .map(|h| h.join(".config/karabiner/assets/complex_modifications"))
            .filter(|p| p.exists());

        if let Some(dir) = karabiner_dir {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.starts_with("hookanchor") && name.ends_with(".json") {
                            return true;
                        }
                    }
                }
            }
        }
        false
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
        let force = self.force_reinstall;

        // Start installation in background thread
        thread::spawn(move || {
            let setup_assistant = SetupAssistant::new();

            for (index, should_install) in selected_components {
                if !should_install {
                    continue;
                }

                let result = match index {
                    0 => setup_assistant.install_karabiner_modification(force)
                        .map_err(|e| e.to_string()),
                    1 => setup_assistant.create_default_config(force)
                        .map_err(|e| e.to_string()),
                    2 => setup_assistant.install_config_js(force)
                        .map_err(|e| e.to_string()),
                    3 => setup_assistant.install_hook_anchor_zshrc(force)
                        .map_err(|e| e.to_string()),
                    4 => setup_assistant.create_initial_commands(force)
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
                            self.status_messages.push(format!("✓ {}", component.name));
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

    /// Render a component checkbox with appropriate status
    fn render_component(&mut self, ui: &mut egui::Ui, index: usize) {
        if let Some(component) = self.components.get_mut(index) {
            ui.horizontal(|ui| {
                match &component.status {
                    InstallStatus::Installed => {
                        // Green checkmark for installed components
                        ui.label(egui::RichText::new("✅").color(egui::Color32::GREEN));
                        ui.label(format!("{} - Installed", component.name));
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

            // Advanced options
            ui.collapsing("Advanced Options", |ui| {
                ui.checkbox(&mut self.force_reinstall, "Force reinstall (overwrite existing files)");
                ui.label("⚠️ This will backup and replace existing configuration files");
            });

            ui.add_space(20.0);

            // Installation button
            ui.horizontal(|ui| {
                let any_selected = self.components.iter()
                    .any(|comp| comp.status == InstallStatus::Selected);

                if ui.add_enabled(
                    any_selected && !self.installation_in_progress,
                    egui::Button::new("Install Selected Components")
                ).clicked() {
                    self.start_installation();
                }

                if self.installation_in_progress {
                    ui.spinner();
                    ui.label("Installing...");
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
            .with_inner_size([500.0, 600.0])
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