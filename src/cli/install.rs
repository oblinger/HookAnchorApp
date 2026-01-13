//! Install/Uninstall Commands
//!
//! Functions for installing and uninstalling HookAnchor.

use crate::utils;
use crate::utils::logging::print;

/// Run the GUI installer (--install).
pub fn run_install(args: &[String]) {
    // Check if --force flag is present for future use
    let _force = args.len() > 2 && args[2] == "--force";

    print("🚀 Launching HookAnchor GUI installer...");
    print("");

    // Get the current executable path to find the installer_gui binary
    let exe_dir = utils::get_binary_dir();
    let installer_path = exe_dir.join("installer_gui");

    // Launch the GUI installer
    match std::process::Command::new(&installer_path)
        .spawn()
    {
        Ok(_) => {
            print("✅ GUI installer launched successfully!");
            print("   Complete the installation in the GUI window.");
        }
        Err(e) => {
            print(&format!("❌ Failed to launch GUI installer: {}", e));
            print("");
            print("Troubleshooting:");
            print(&format!("  - Make sure installer_gui binary exists at: {}", installer_path.display()));
            print("  - Try rebuilding with: cargo build --release");
            print("  - Check file permissions");
        }
    }
}

/// Uninstall HookAnchor (--uninstall).
pub fn run_uninstall() {
    // Check if uninstall script exists and use it
    if let Ok(home) = std::env::var("HOME") {
        let uninstall_script = format!("{}/.config/hookanchor/uninstall.sh", home);
        if std::path::Path::new(&uninstall_script).exists() {
            print("🗑️  Found uninstall script - launching...");
            print("");

            // Execute the uninstall script directly (it will handle prompts)
            match std::process::Command::new("bash")
                .arg(&uninstall_script)
                .status()
            {
                Ok(status) => {
                    if status.success() {
                        print("✅ Uninstall completed via script");
                    } else {
                        print("⚠️  Uninstall script completed with warnings");
                    }
                }
                Err(e) => {
                    print(&format!("⚠️  Could not run uninstall script: {}", e));
                    print("Falling back to manual uninstall...");
                    run_manual_uninstall();
                }
            }
            return;
        }
    }

    // Fallback to manual uninstall if script not found
    print("🗑️  Uninstall script not found - using manual method");
    run_manual_uninstall();
}

/// Manual uninstall fallback.
fn run_manual_uninstall() {
    print("🗑️  HookAnchor Uninstall");
    print("========================");
    print("");

    print("This will remove:");
    print("  - Karabiner configuration for HookAnchor");
    print("  - Configuration directory: ~/.config/hookanchor");
    print("  - URL handler registration");
    print("");

    print!("Are you sure you want to uninstall HookAnchor? (y/N): ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        print("");
        print("🗑️  Uninstalling HookAnchor...");

        // Remove configuration directory
        if let Ok(home) = std::env::var("HOME") {
            let config_dir = format!("{}/.config/hookanchor", home);
            if std::path::Path::new(&config_dir).exists() {
                match std::fs::remove_dir_all(&config_dir) {
                    Ok(()) => print("✅ Removed configuration directory"),
                    Err(e) => print(&format!("⚠️  Could not remove config directory: {}", e)),
                }
            }
        }

        // Remove Karabiner configuration
        let setup_assistant = crate::systems::setup_assistant::SetupAssistant::new();
        match setup_assistant.remove_karabiner_config() {
            Ok(()) => print("✅ Removed Karabiner configuration"),
            Err(e) => print(&format!("⚠️  Could not remove Karabiner configuration: {}", e)),
        }

        print("");
        print("✅ HookAnchor uninstalled successfully!");
        print("You may need to manually remove any remaining files:");
        print("  - App bundles in /Applications");
        print("  - Binary files if installed to system paths");
    } else {
        print("Uninstall cancelled.");
    }
}
