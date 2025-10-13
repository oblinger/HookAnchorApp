//! First-run Setup Assistant for HookAnchor
//! 
//! This module handles the initial setup when HookAnchor is launched for the first time,
//! including Karabiner-Elements integration and configuration file creation.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const KARABINER_CLI_PATH: &str = "/Library/Application Support/org.pqrs/Karabiner-Elements/bin/karabiner_cli";
#[allow(dead_code)]
const KARABINER_CONFIG_DIR: &str = "~/.config/karabiner/assets/complex_modifications";

pub struct SetupAssistant {
    config_dir: PathBuf,
    first_run: bool,
}

impl SetupAssistant {
    pub fn new() -> Self {
        let config_dir = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
            .join("hookanchor");
        
        let first_run = !config_dir.join("config.yaml").exists();
        
        Self {
            config_dir,
            first_run,
        }
    }
    
    /// Check if this is the first run
    pub fn is_first_run(&self) -> bool {
        self.first_run
    }
    
    
    /// Check if Karabiner-Elements is installed
    fn check_karabiner(&self) -> bool {
        Path::new(KARABINER_CLI_PATH).exists()
    }
    
    /// Check if Terminal has accessibility permissions
    fn check_terminal_accessibility(&self) -> bool {
        // Test if Terminal can send keystrokes via AppleScript
        // This will fail if Terminal doesn't have Accessibility permissions
        let test_script = r#"
            tell application "System Events"
                try
                    -- Try to query if we can send keystrokes (doesn't actually send any)
                    set canSend to exists (keystroke)
                    return "true"
                on error
                    return "false"
                end try
            end tell
        "#;
        
        let output = Command::new("osascript")
            .arg("-e")
            .arg(test_script)
            .output();
        
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                // Check if we got the specific permission error
                if stderr.contains("not allowed to send keystrokes") {
                    false
                } else {
                    // If no permission error, Terminal has access
                    !stderr.contains("1002") && stdout.trim() != "false"
                }
            }
            Err(_) => false,
        }
    }
    
    /// Prompt user to grant accessibility permissions to Terminal
    fn prompt_accessibility_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n⚠️  Terminal needs Accessibility permissions for HookAnchor to work properly.");
        println!("\n🎯 IMPORTANT: Since HookAnchor v0.12, we run servers through Terminal");
        println!("   This allows the grabber to capture URLs from apps like Notion!");
        
        println!("\nThis permission is required to:");
        println!("• Capture window information from applications");
        println!("• Grab URLs from Notion, Obsidian, and browsers");
        println!("• Send keyboard shortcuts to applications");
        println!("• Enable the grabber feature (+ key in popup)");
        
        println!("\n📋 Steps to grant permission:");
        println!("1. System Settings will open to Privacy & Security → Accessibility");
        println!("2. Click the lock icon 🔒 and enter your password");
        println!("3. Look for 'Terminal' in the list");
        println!("4. If Terminal is NOT in the list:");
        println!("   • Click the '+' button");
        println!("   • Navigate to /Applications/Utilities/");
        println!("   • Select Terminal.app and click Open");
        println!("5. Make sure the checkbox ✓ is enabled next to Terminal");
        
        println!("\n💡 Why Terminal?");
        println!("   HookAnchor's servers now run in Terminal windows to inherit");
        println!("   Apple's special permissions for automation scripts.");
        
        println!("\nPress Enter to open System Settings...");
        std::io::stdin().read_line(&mut String::new())?;
        
        // Open System Settings to the Accessibility pane
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn()?;
        
        println!("\nAfter enabling Terminal in Accessibility, press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        
        // Check again after user claims to have set permissions
        if self.check_terminal_accessibility() {
            println!("\n✅ Great! Terminal has Accessibility permissions.");
            println!("   The grabber feature will work correctly!");
        } else {
            println!("\n⚠️  Terminal still doesn't have Accessibility permissions.");
            println!("   The grabber feature may not work until you grant permissions.");
            println!("   You can add Terminal later through:");
            println!("   System Settings → Privacy & Security → Accessibility");
        }
        
        Ok(())
    }
    
    /// Prompt user to install Karabiner-Elements
    fn prompt_karabiner_install(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⚠️  Karabiner-Elements is required but not installed.");
        println!("\nKarabiner-Elements is needed to set up the keyboard shortcut for HookAnchor.");
        println!("\nPlease install it from: https://karabiner-elements.pqrs.org");
        println!("\nSteps:");
        println!("1. Download Karabiner-Elements from the link above");
        println!("2. Install it by opening the downloaded DMG file");
        println!("3. Grant the necessary permissions when prompted");
        println!("4. Re-run HookAnchor after installation\n");
        
        println!("Press Enter to open the download page in your browser...");
        std::io::stdin().read_line(&mut String::new())?;
        
        // Open the Karabiner download page
        Command::new("open")
            .arg("https://karabiner-elements.pqrs.org")
            .spawn()?;
        
        std::process::exit(0);
    }
    
    /// Create the configuration directory structure
    fn create_config_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Creating configuration directory...");
        
        fs::create_dir_all(&self.config_dir)?;
        fs::create_dir_all(self.config_dir.join("backups"))?;
        fs::create_dir_all(self.config_dir.join("logs"))?;
        
        Ok(())
    }
    
    /// Create the default config.yaml file (safely) - GUI-friendly version
    pub fn create_default_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.config_dir.join("config.yaml");

        // Prioritize dist_config.yaml (only install if doesn't exist)
        if let Ok(dist_config_path) = self.find_distribution_file("dist_config.yaml") {
            if !config_path.exists() {
                fs::copy(&dist_config_path, &config_path)?;
            }
            return Ok(());
        }

        // MUST use distribution default_config.yaml - no fallbacks!
        let dist_config_path = self.find_distribution_file("default_config.yaml")?;

        if config_path.exists() {
            // NEVER overwrite existing config.yaml - user settings are too complex and valuable
            return Ok(());
        }

        // Copy distribution default to user config
        fs::copy(&dist_config_path, &config_path)?;
        Ok(())
    }



    /// Install config.js from distribution default
    pub fn install_config_js(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_js_path = self.config_dir.join("config.js");

        // Prioritize dist_config.js (only install if doesn't exist)
        if let Ok(dist_config_js) = self.find_distribution_file("dist_config.js") {
            if !config_js_path.exists() {
                fs::copy(&dist_config_js, &config_js_path)?;
            }
            return Ok(());
        }

        // Fallback to old default_config.js approach (only install if doesn't exist)
        if !config_js_path.exists() {
            let dist_config_js = self.find_distribution_file("default_config.js")?;
            fs::copy(&dist_config_js, &config_js_path)?;
        }
        Ok(())
    }

    /// Install uninstaller script from distribution default
    pub fn install_uninstaller_script(&self) -> Result<(), Box<dyn std::error::Error>> {
        let uninstall_script_path = self.config_dir.join("uninstall.sh");

        // Prioritize dist_uninstall.sh (always install approach)
        if let Ok(dist_uninstall) = self.find_distribution_file("dist_uninstall.sh") {
            if uninstall_script_path.exists() {
                return Ok(());
            }

            // Install the dist_ file
            fs::copy(&dist_uninstall, &uninstall_script_path)?;

            // Make it executable
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&uninstall_script_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&uninstall_script_path, perms)?;
            }

            return Ok(());
        }

        // Fallback to looking for the original script
        let source_script = self.find_distribution_file("uninstall.sh").or_else(|_| -> Result<PathBuf, Box<dyn std::error::Error>> {
            // Look in scripts directory relative to exe
            let exe_path = std::env::current_exe().unwrap_or_default();
            let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
            let script_path = exe_dir.join("../../scripts/uninstall.sh");
            if script_path.exists() {
                Ok(script_path.canonicalize()?)
            } else {
                Err("Uninstall script not found".into())
            }
        })?;

        if uninstall_script_path.exists() {
            return Ok(());
        }

        fs::copy(&source_script, &uninstall_script_path)?;

        // Make it executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&uninstall_script_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&uninstall_script_path, perms)?;
        }

        Ok(())
    }

    /// Install Uninstaller.app from distribution
    pub fn install_uninstaller_app(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;

        let uninstaller_app_path = self.config_dir.join("Uninstaller.app");

        // Look for Uninstaller.app in distribution files
        if let Ok(dist_uninstaller_dir) = self.find_distribution_file("Uninstaller.app") {
            if uninstaller_app_path.exists()  {
                // Remove existing app if forcing update or if it exists
                std::fs::remove_dir_all(&uninstaller_app_path)?;
            }

            // Copy the entire app bundle
            Command::new("cp")
                .arg("-R")
                .arg(&dist_uninstaller_dir)
                .arg(&uninstaller_app_path)
                .output()?;

            return Ok(());
        }

        Err("Uninstaller.app not found in distribution".into())
    }

    /// Install hook_anchor_zshrc from distribution default
    pub fn install_hook_anchor_zshrc(&self) -> Result<(), Box<dyn std::error::Error>> {
        let zshrc_path = self.config_dir.join("hook_anchor_zshrc");

        // Find the distribution default zshrc
        let dist_zshrc = self.find_distribution_file("dist_hook_anchor_zshrc")?;

        if zshrc_path.exists()  {
            // Check if file has changed from default and backup if needed
            if self.file_differs_from_default(&zshrc_path, &dist_zshrc)? {
                self.create_backup(&zshrc_path)?;
            }
        }

        // Copy distribution default to config location
        fs::copy(&dist_zshrc, &zshrc_path)?;
        Ok(())
    }

    /// Find a distribution file relative to the executable
    fn find_distribution_file(&self, filename: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let exe_path = std::env::current_exe().unwrap_or_default();
        let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));

        // Look for distribution files in possible locations
        let possible_paths = vec![
            // In the app bundle Resources directory
            exe_dir.join("../Resources").join(filename),
            // In development mode from target/release
            exe_dir.join("../../dist/HookAnchor.app/Contents/Resources").join(filename),
            // In development mode from target/debug or target/debug/examples
            exe_dir.join("../../../dist/HookAnchor.app/Contents/Resources").join(filename),
            exe_dir.join("../../../../dist/HookAnchor.app/Contents/Resources").join(filename),
            // Direct in exe directory
            exe_dir.join(filename),
        ];

        for path in &possible_paths {
            if path.exists() {
                return Ok(path.canonicalize()?);
            }
        }

        Err(format!("Distribution file '{}' not found in any expected location", filename).into())
    }

    /// Check if a file differs from the distribution default
    fn file_differs_from_default(&self, user_file: &std::path::Path, dist_file: &std::path::Path) -> Result<bool, Box<dyn std::error::Error>> {
        let user_content = fs::read_to_string(user_file)?;
        let dist_content = fs::read_to_string(dist_file)?;
        Ok(user_content != dist_content)
    }

    /// Create a timestamped backup of a file
    fn create_backup(&self, file_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        if !file_path.exists() {
            return Ok(()); // Nothing to backup
        }

        let file_name = file_path.file_name()
            .and_then(|name| name.to_str())
            .ok_or("Invalid file name")?;

        let backup_path = self.config_dir.join(format!("{}.backup.{}",
            file_name,
            chrono::Utc::now().format("%Y%m%d_%H%M%S")));

        fs::copy(file_path, &backup_path)?;
        Ok(())
    }

    /// Check for existing Caps Lock mappings in Karabiner
    fn check_caps_lock_conflicts(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let karabiner_dir = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
            .join("karabiner")
            .join("assets")
            .join("complex_modifications");
        
        let mut conflicts = Vec::new();
        
        if karabiner_dir.exists() {
            for entry in fs::read_dir(&karabiner_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if content.contains("\"caps_lock\"") && !path.file_name().unwrap_or_default().to_str().unwrap_or("").starts_with("hookanchor") {
                            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                                conflicts.push(filename.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        Ok(conflicts)
    }

    /// Install the Karabiner complex modification (safely with user prompts)
    pub fn install_karabiner_modification(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if Karabiner-Elements is installed - only check Applications folder
        // CLI binary can persist after app deletion, causing false positives
        let app_installed = std::path::Path::new("/Applications/Karabiner-Elements.app").exists();

        if !app_installed {
            return Err("Karabiner-Elements is not installed. Please install it from https://karabiner-elements.pqrs.org/ first.".into());
        }

        let karabiner_dir = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
            .join("karabiner")
            .join("assets")
            .join("complex_modifications");

        // Create the directory structure (this is safe even if Karabiner creates it later)
        fs::create_dir_all(&karabiner_dir)?;
        
        // Check for Caps Lock conflicts first
        
        println!("\n🔍 Checking for existing Caps Lock mappings...");
        let conflicts = self.check_caps_lock_conflicts()?;
        
        if !conflicts.is_empty() {
            println!("⚠️  Found existing Caps Lock mappings in:");
            for conflict in &conflicts {
                println!("   - {}", conflict);
            }
            println!("\nHookAnchor can use different hotkeys to avoid conflicts:");
            println!("1. Caps Lock (may conflict with existing mappings)");
            println!("2. F12+Cmd+Option (safer, less likely to conflict)");
            println!("3. Skip Karabiner setup (configure manually later)");
            
            print!("\nChoose an option (1/2/3): ");
            use std::io::{self, Write};
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();
            
            match choice {
                "1" => {
                    println!("⚠️  Installing Caps Lock mapping (may cause conflicts)");
                    println!("   You may need to disable conflicting rules manually in Karabiner");
                },
                "2" => {
                    println!("✅ Using F12+Cmd+Option hotkey (safer option)");
                    return self.install_karabiner_f12_only();
                },
                "3" => {
                    println!("⏭️  Skipping Karabiner setup");
                    println!("   You can manually configure hotkeys in Karabiner later");
                    return Ok(());
                },
                _ => {
                    println!("Invalid choice, skipping Karabiner setup");
                    return Ok(());
                }
            }
        } else {
            println!("✅ No Caps Lock conflicts detected");
        }
        
        let mod_path = karabiner_dir.join("hookanchor.json");
        
        // Check if modification already exists and is current
        if mod_path.exists() {
            // Read existing modification
            if let Ok(existing_content) = fs::read_to_string(&mod_path) {
                // Check if it contains the current app path
                let current_path = "/Applications/HookAnchor.app/Contents/MacOS/hookanchor";
                if existing_content.contains(current_path) {
                    println!("Karabiner modification already exists and is current");
                    return Ok(());
                } else {
                    println!("Updating existing Karabiner modification...");
                    // Backup the existing modification
                    let backup_path = karabiner_dir.join("hookanchor.json.backup");
                    fs::copy(&mod_path, &backup_path)?;
                    println!("  Backed up existing modification to hookanchor.json.backup");
                }
            }
        } else {
            println!("Installing Karabiner complex modification...");
        }
        
        // Copy the static resource file from app bundle Resources
        let exe_path = std::env::current_exe()?;
        let resource_path = exe_path
            .parent() // MacOS
            .and_then(|p| p.parent()) // Contents
            .map(|p| p.join("Resources").join("hookanchor.json"))
            .expect("Could not determine app bundle path");

        if !resource_path.exists() {
            return Err(format!("Resource file not found: {:?}", resource_path).into());
        }

        fs::copy(&resource_path, &mod_path)?;
        
        // Enable the modification using Karabiner CLI
        println!("Enabling Karabiner modification...");
        Command::new(KARABINER_CLI_PATH)
            .args(&["--select-profile", "Default profile"])
            .output()?;
        
        // Note: The user will need to manually enable the complex modification
        // in Karabiner-Elements preferences
        println!("\n📝 Note: You'll need to enable the HookAnchor modification in");
        println!("   Karabiner-Elements → Complex Modifications → Add rule");
        
        Ok(())
    }
    
    /// Install F12+Cmd+Option hotkey only (no Caps Lock conflicts)
    fn install_karabiner_f12_only(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if Karabiner-Elements is installed - only check Applications folder
        // CLI binary can persist after app deletion, causing false positives
        let app_installed = std::path::Path::new("/Applications/Karabiner-Elements.app").exists();

        if !app_installed {
            return Err("Karabiner-Elements is not installed. Please install it from https://karabiner-elements.pqrs.org/ first.".into());
        }

        let karabiner_dir = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
            .join("karabiner")
            .join("assets")
            .join("complex_modifications");

        // Create the directory structure (this is safe even if Karabiner creates it later)
        fs::create_dir_all(&karabiner_dir)?;

        let mod_path = karabiner_dir.join("hookanchor.json");
        
        // Copy the static resource file from app bundle Resources
        let exe_path = std::env::current_exe()?;
        let resource_path = exe_path
            .parent() // MacOS
            .and_then(|p| p.parent()) // Contents
            .map(|p| p.join("Resources").join("hookanchor.json"))
            .expect("Could not determine app bundle path");

        if !resource_path.exists() {
            return Err(format!("Resource file not found: {:?}", resource_path).into());
        }

        fs::copy(&resource_path, &mod_path)?;
        
        println!("✅ Installed F12+Cmd+Option hotkey");
        println!("\n📝 Note: You'll need to enable the HookAnchor modification in");
        println!("   Karabiner-Elements → Complex Modifications → Add rule");
        
        Ok(())
    }
    
    /// Create initial commands.txt file with some starter commands (safely)
    /// Install commands.txt using dist_ approach
    pub fn install_commands_txt(&self) -> Result<(), Box<dyn std::error::Error>> {
        let commands_path = self.config_dir.join("commands.txt");

        // Prioritize dist_commands.txt (always install approach)
        if let Ok(dist_commands) = self.find_distribution_file("dist_commands.txt") {
            if commands_path.exists()  {
                // Check if user file differs from distribution default
                if self.file_differs_from_default(&commands_path, &dist_commands)? {
                    // User has modifications - back up their version
                    self.create_backup(&commands_path)?;
                }
            }

            // Always install the latest dist_ file
            fs::copy(&dist_commands, &commands_path)?;
            return Ok(());
        }

        // Fallback to old create_initial_commands approach
        self.create_initial_commands()
    }

    pub fn create_initial_commands(&self) -> Result<(), Box<dyn std::error::Error>> {
        let commands_path = self.config_dir.join("commands.txt");
        
        if commands_path.exists() {
            // Commands file already exists - don't overwrite
            return Ok(());
        }

        // Create initial commands file
        println!("Creating initial commands file...");
        
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        let initial_commands = format!(r#"# HookAnchor Commands
# Generated on: {}
# Format: patch! command : action flags; argument

# Example commands to get you started:
Apps! Terminal : app; Terminal
Apps! Safari : app; Safari
Apps! Finder : app; Finder

# Add your own commands below:
"#, timestamp);
        
        fs::write(&commands_path, initial_commands)?;
        println!("✓ Initial commands file created with starter examples");
        
        Ok(())
    }
}


impl SetupAssistant {
    /// Remove Karabiner configuration for HookAnchor
    pub fn remove_karabiner_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let karabiner_dir = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
            .join("karabiner")
            .join("assets")
            .join("complex_modifications");

        let mod_path = karabiner_dir.join("hookanchor.json");

        if mod_path.exists() {
            // Disable the modification first if CLI is available
            if Path::new(KARABINER_CLI_PATH).exists() {
                let _result = Command::new(KARABINER_CLI_PATH)
                    .args(&["--remove-rule", "HookAnchor Launcher"])
                    .output();
                // Don't fail if this doesn't work - just continue
            }

            // Remove the JSON file
            fs::remove_file(&mod_path)?;
        }

        Ok(())
    }

    /// Remove configuration directory (with user confirmation for data)
    #[allow(dead_code)]
    fn remove_config_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config_dir.exists() {
            println!("ℹ️  Configuration directory not found");
            return Ok(());
        }
        
        // Check if there are any important files
        let mut has_important_files = false;
        let mut file_count = 0;
        
        if let Ok(entries) = fs::read_dir(&self.config_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    file_count += 1;
                    let file_name = entry.file_name();
                    if let Some(name) = file_name.to_str() {
                        if name.ends_with(".txt") || name.ends_with(".yaml") || name.contains("command") {
                            has_important_files = true;
                        }
                    }
                }
            }
        }
        
        if has_important_files {
            println!("⚠️  Configuration directory contains {} files including commands/config.", file_count);
            print!("Remove all configuration and data? (y/N): ");
            
            use std::io::{self, Write};
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let response = input.trim().to_lowercase();
            
            if response != "y" && response != "yes" {
                println!("📁 Keeping configuration directory: {}", self.config_dir.display());
                return Ok(());
            }
        }
        
        println!("🗑️  Removing configuration directory...");
        fs::remove_dir_all(&self.config_dir)?;
        
        Ok(())
    }
    
    /// Remove configuration directory (silent version without user confirmation)
    #[allow(dead_code)]
    fn remove_config_directory_silent(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config_dir.exists() {
            fs::remove_dir_all(&self.config_dir)?;
        }

        Ok(())
    }

    /// Test if the current application has accessibility permissions
    pub fn test_accessibility_permissions() -> Result<bool, Box<dyn std::error::Error>> {
        crate::utils::log("Testing accessibility permissions...");

        // Simple test script that tries to send a keystroke to System Events
        let test_script = r#"
            tell application "System Events"
                try
                    -- Try to get the frontmost process (requires accessibility)
                    set frontApp to first application process whose frontmost is true
                    set appName to name of frontApp
                    return "SUCCESS: " & appName
                on error errMsg number errNum
                    return "ERROR: " & errMsg & " (Code: " & errNum & ")"
                end try
            end tell
        "#;

        let output = Command::new("osascript")
            .arg("-e")
            .arg(test_script)
            .output()?;

        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr);

        crate::utils::log(&format!("Accessibility test result: '{}'", result));
        if !stderr.is_empty() {
            crate::utils::log(&format!("Accessibility test stderr: '{}'", stderr));
        }

        // Check for permission-related errors
        let has_permission = result.starts_with("SUCCESS:") &&
                           !stderr.contains("not allowed") &&
                           !stderr.contains("1002");

        Ok(has_permission)
    }

    /// Open macOS System Preferences to Accessibility settings
    pub fn open_accessibility_settings() -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::log("Opening System Preferences → Security & Privacy → Accessibility");

        // Try the modern System Settings first (macOS 13+)
        let modern_result = Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .output();

        if let Ok(output) = modern_result {
            if output.status.success() {
                return Ok(());
            }
        }

        // Fallback to legacy System Preferences (macOS 12 and earlier)
        let legacy_result = Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .output();

        if let Ok(output) = legacy_result {
            if output.status.success() {
                return Ok(());
            }
        }

        // Final fallback - just open System Preferences
        Command::new("open")
            .arg("/System/Applications/System Preferences.app")
            .output()?;

        Ok(())
    }

    /// Check accessibility permissions and prompt user if needed
    pub fn ensure_accessibility_permissions() -> Result<bool, Box<dyn std::error::Error>> {
        match Self::test_accessibility_permissions()? {
            true => {
                crate::utils::log("✅ Accessibility permissions are granted");
                Ok(true)
            }
            false => {
                crate::utils::log("❌ Accessibility permissions are missing");

                println!("\n🔒 ACCESSIBILITY PERMISSIONS REQUIRED");
                println!("═══════════════════════════════════════");
                println!("HookAnchor needs accessibility permissions to:");
                println!("• Capture context from applications (Obsidian, Notion, etc.)");
                println!("• Send keystrokes to copy URLs and content");
                println!("• Detect window titles and active documents");
                println!();
                println!("📱 WHAT TO DO:");
                println!("1. Click OK to open System Preferences");
                println!("2. Navigate to Security & Privacy → Privacy → Accessibility");
                println!("3. Click the lock icon and enter your password");
                println!("4. Find and enable the app running HookAnchor:");
                println!("   - Terminal (if running from command line)");
                println!("   - HookAnchor.app (if using the app bundle)");
                println!("   - Your IDE (if running from development environment)");
                println!("5. Come back and test again");
                println!();
                print!("Press Enter to open System Preferences...");

                use std::io::{self, Write};
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                Self::open_accessibility_settings()?;

                println!("\n⏳ After granting permissions, test again with:");
                println!("   ha --test-permissions");

                Ok(false)
            }
        }
    }

    /// Install/update the latest distribution files to user config folder
    /// This ensures users always have the latest templates and config examples
    pub fn install_latest_dist_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::log("Installing latest distribution files to user config...");

        // List of dist files to copy (latest versions from distribution)
        let dist_files = [
            ("dist_config.yaml", "dist_config.yaml"),
            ("dist_config.js", "dist_config.js"),
            ("dist_hook_anchor_zshrc", "dist_hook_anchor_zshrc"),
        ];

        for (source_name, target_name) in &dist_files {
            if let Ok(source_path) = self.find_distribution_file(source_name) {
                let target_path = self.config_dir.join(target_name);

                // Check if target exists and differs from source
                if target_path.exists() {
                    if self.file_differs_from_default(&target_path, &source_path)? {
                        // User has a different version - create backup
                        self.create_backup(&target_path)?;
                        crate::utils::log(&format!("Backed up existing {} due to changes", target_name));
                    }
                }

                // Always copy the latest version
                fs::copy(&source_path, &target_path)?;
                crate::utils::log(&format!("✓ Installed latest {}", target_name));
            } else {
                crate::utils::log(&format!("Warning: Distribution file {} not found", source_name));
            }
        }

        Ok(())
    }
}