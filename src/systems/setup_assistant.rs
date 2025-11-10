//! First-run Setup Assistant for HookAnchor
//! 
//! This module handles the initial setup when HookAnchor is launched for the first time,
//! including Karabiner-Elements integration and configuration file creation.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const KARABINER_CLI_PATH: &str = "/Library/Application Support/org.pqrs/Karabiner-Elements/bin/karabiner_cli";

pub struct SetupAssistant {
    config_dir: PathBuf,
    first_run: bool,
}

impl SetupAssistant {
    pub fn new() -> Self {
        let config_dir = crate::core::get_config_dir();

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
        crate::utils::print("\n⚠️  Terminal needs Accessibility permissions for HookAnchor to work properly.");
        crate::utils::print("\n🎯 IMPORTANT: Since HookAnchor v0.12, we run servers through Terminal");
        crate::utils::print("   This allows the grabber to capture URLs from apps like Notion!");
        
        crate::utils::print("\nThis permission is required to:");
        crate::utils::print("• Capture window information from applications");
        crate::utils::print("• Grab URLs from Notion, Obsidian, and browsers");
        crate::utils::print("• Send keyboard shortcuts to applications");
        crate::utils::print("• Enable the grabber feature (+ key in popup)");
        
        crate::utils::print("\n📋 Steps to grant permission:");
        crate::utils::print("1. System Settings will open to Privacy & Security → Accessibility");
        crate::utils::print("2. Click the lock icon 🔒 and enter your password");
        crate::utils::print("3. Look for 'Terminal' in the list");
        crate::utils::print("4. If Terminal is NOT in the list:");
        crate::utils::print("   • Click the '+' button");
        crate::utils::print("   • Navigate to /Applications/Utilities/");
        crate::utils::print("   • Select Terminal.app and click Open");
        crate::utils::print("5. Make sure the checkbox ✓ is enabled next to Terminal");
        
        crate::utils::print("\n💡 Why Terminal?");
        crate::utils::print("   HookAnchor's servers now run in Terminal windows to inherit");
        crate::utils::print("   Apple's special permissions for automation scripts.");
        
        crate::utils::print("\nPress Enter to open System Settings...");
        std::io::stdin().read_line(&mut String::new())?;
        
        // Open System Settings to the Accessibility pane
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn()?;
        
        crate::utils::print("\nAfter enabling Terminal in Accessibility, press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        
        // Check again after user claims to have set permissions
        if self.check_terminal_accessibility() {
            crate::utils::print("\n✅ Great! Terminal has Accessibility permissions.");
            crate::utils::print("   The grabber feature will work correctly!");
        } else {
            crate::utils::print("\n⚠️  Terminal still doesn't have Accessibility permissions.");
            crate::utils::print("   The grabber feature may not work until you grant permissions.");
            crate::utils::print("   You can add Terminal later through:");
            crate::utils::print("   System Settings → Privacy & Security → Accessibility");
        }
        
        Ok(())
    }
    
    /// Prompt user to install Karabiner-Elements
    fn prompt_karabiner_install(&self) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::print("⚠️  Karabiner-Elements is required but not installed.");
        crate::utils::print("\nKarabiner-Elements is needed to set up the keyboard shortcut for HookAnchor.");
        crate::utils::print("\nPlease install it from: https://karabiner-elements.pqrs.org");
        crate::utils::print("\nSteps:");
        crate::utils::print("1. Download Karabiner-Elements from the link above");
        crate::utils::print("2. Install it by opening the downloaded DMG file");
        crate::utils::print("3. Grant the necessary permissions when prompted");
        crate::utils::print("4. Re-run HookAnchor after installation\n");
        
        crate::utils::print("Press Enter to open the download page in your browser...");
        std::io::stdin().read_line(&mut String::new())?;
        
        // Open the Karabiner download page
        Command::new("open")
            .arg("https://karabiner-elements.pqrs.org")
            .spawn()?;
        
        std::process::exit(0);
    }
    
    /// Create the configuration directory structure
    fn create_config_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::print("Creating configuration directory...");
        
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
        
        crate::utils::print("\n🔍 Checking for existing Caps Lock mappings...");
        let conflicts = self.check_caps_lock_conflicts()?;
        
        if !conflicts.is_empty() {
            crate::utils::print("⚠️  Found existing Caps Lock mappings in:");
            for conflict in &conflicts {
                crate::utils::print(&format!("   - {}", conflict));
            }
            crate::utils::print("\nHookAnchor can use different hotkeys to avoid conflicts:");
            crate::utils::print("1. Caps Lock (may conflict with existing mappings)");
            crate::utils::print("2. F12+Cmd+Option (safer, less likely to conflict)");
            crate::utils::print("3. Skip Karabiner setup (configure manually later)");
            
            print!("\nChoose an option (1/2/3): ");
            use std::io::{self, Write};
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();
            
            match choice {
                "1" => {
                    crate::utils::print("⚠️  Installing Caps Lock mapping (may cause conflicts)");
                    crate::utils::print("   You may need to disable conflicting rules manually in Karabiner");
                },
                "2" => {
                    crate::utils::print("✅ Using F12+Cmd+Option hotkey (safer option)");
                    return self.install_karabiner_f12_only();
                },
                "3" => {
                    crate::utils::print("⏭️  Skipping Karabiner setup");
                    crate::utils::print("   You can manually configure hotkeys in Karabiner later");
                    return Ok(());
                },
                _ => {
                    crate::utils::print("Invalid choice, skipping Karabiner setup");
                    return Ok(());
                }
            }
        } else {
            crate::utils::print("✅ No Caps Lock conflicts detected");
        }
        
        let mod_path = karabiner_dir.join("hookanchor.json");
        
        // Check if modification already exists and is current
        if mod_path.exists() {
            // Read existing modification
            if let Ok(existing_content) = fs::read_to_string(&mod_path) {
                // Check if it contains the current app path
                let current_path = "/Applications/HookAnchor.app/Contents/MacOS/hookanchor";
                if existing_content.contains(current_path) {
                    crate::utils::print("Karabiner modification already exists and is current");
                    return Ok(());
                } else {
                    crate::utils::print("Updating existing Karabiner modification...");
                    // Backup the existing modification
                    let backup_path = karabiner_dir.join("hookanchor.json.backup");
                    fs::copy(&mod_path, &backup_path)?;
                    crate::utils::print("  Backed up existing modification to hookanchor.json.backup");
                }
            }
        } else {
            crate::utils::print("Installing Karabiner complex modification...");
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
        crate::utils::print("Enabling Karabiner modification...");
        Command::new(KARABINER_CLI_PATH)
            .args(&["--select-profile", "Default profile"])
            .output()?;
        
        // Note: The user will need to manually enable the complex modification
        // in Karabiner-Elements preferences
        crate::utils::print("\n📝 Note: You'll need to enable the HookAnchor modification in");
        crate::utils::print("   Karabiner-Elements → Complex Modifications → Add rule");
        
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
        
        crate::utils::print("✅ Installed F12+Cmd+Option hotkey");
        crate::utils::print("\n📝 Note: You'll need to enable the HookAnchor modification in");
        crate::utils::print("   Karabiner-Elements → Complex Modifications → Add rule");
        
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
        crate::utils::print("Creating initial commands file...");
        
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
        crate::utils::print("✓ Initial commands file created with starter examples");
        
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

                crate::utils::print("\n🔒 ACCESSIBILITY PERMISSIONS REQUIRED");
                crate::utils::print("═══════════════════════════════════════");
                crate::utils::print("HookAnchor needs accessibility permissions to:");
                crate::utils::print("• Capture context from applications (Obsidian, Notion, etc.)");
                crate::utils::print("• Send keystrokes to copy URLs and content");
                crate::utils::print("• Detect window titles and active documents");
                crate::utils::print("");
                crate::utils::print("📱 WHAT TO DO:");
                crate::utils::print("1. Click OK to open System Preferences");
                crate::utils::print("2. Navigate to Security & Privacy → Privacy → Accessibility");
                crate::utils::print("3. Click the lock icon and enter your password");
                crate::utils::print("4. Find and enable the app running HookAnchor:");
                crate::utils::print("   - Terminal (if running from command line)");
                crate::utils::print("   - HookAnchor.app (if using the app bundle)");
                crate::utils::print("   - Your IDE (if running from development environment)");
                crate::utils::print("5. Come back and test again");
                crate::utils::print("");
                print!("Press Enter to open System Preferences...");

                use std::io::{self, Write};
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                Self::open_accessibility_settings()?;

                crate::utils::print("\n⏳ After granting permissions, test again with:");
                crate::utils::print("   ha --test-permissions");

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

    /// Update config.yaml with latest version, backing up existing file
    pub fn update_config_yaml_with_backup(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.config_dir.join("config.yaml");

        // Backup existing config.yaml if it exists
        if config_path.exists() {
            let backup_path = self.config_dir.join("config.yaml.backup");

            // If backup already exists, overwrite it
            if backup_path.exists() {
                fs::remove_file(&backup_path)?;
            }

            fs::rename(&config_path, &backup_path)?;
            crate::utils::log(&format!("✓ Backed up config.yaml to config.yaml.backup"));
        }

        // Find and copy distribution config
        // Try dist_config.yaml first, then default_config.yaml
        if let Ok(dist_config_path) = self.find_distribution_file("dist_config.yaml") {
            fs::copy(&dist_config_path, &config_path)?;
            crate::utils::log("✓ Installed latest dist_config.yaml as config.yaml");
            return Ok(());
        }

        let dist_config_path = self.find_distribution_file("default_config.yaml")?;
        fs::copy(&dist_config_path, &config_path)?;
        crate::utils::log("✓ Installed latest default_config.yaml as config.yaml");

        Ok(())
    }

    /// Update config.js with backup (same as config.yaml update)
    pub fn update_config_js_with_backup(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.config_dir.join("config.js");

        // Backup existing config.js if it exists
        if config_path.exists() {
            let backup_path = self.config_dir.join("config.js.backup");

            // If backup already exists, overwrite it
            if backup_path.exists() {
                fs::remove_file(&backup_path)?;
            }

            fs::rename(&config_path, &backup_path)?;
            crate::utils::log(&format!("✓ Backed up config.js to config.js.backup"));
        }

        // Find and copy distribution config.js
        // Try dist_config.js first, then default_config.js
        if let Ok(dist_config_path) = self.find_distribution_file("dist_config.js") {
            fs::copy(&dist_config_path, &config_path)?;
            crate::utils::log("✓ Installed latest dist_config.js as config.js");
            return Ok(());
        }

        let dist_config_path = self.find_distribution_file("default_config.js")?;
        fs::copy(&dist_config_path, &config_path)?;
        crate::utils::log("✓ Installed latest default_config.js as config.js");

        Ok(())
    }

    /// Add HookAnchor to login items using osascript
    pub fn add_login_item(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the path to HookAnchor.app
        let app_path = "/Applications/HookAnchor.app";

        // Check if app exists
        if !Path::new(app_path).exists() {
            return Err("HookAnchor.app not found in /Applications".into());
        }

        // Use osascript to add login item
        let script = format!(
            r#"tell application "System Events"
                make login item at end with properties {{path:"{}", hidden:false}}
            end tell"#,
            app_path
        );

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to add login item: {}", error).into());
        }

        crate::utils::log("✓ Added HookAnchor to login items");
        Ok(())
    }
}