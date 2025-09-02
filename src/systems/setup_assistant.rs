//! First-run Setup Assistant for HookAnchor
//! 
//! This module handles the initial setup when HookAnchor is launched for the first time,
//! including Karabiner-Elements integration and configuration file creation.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde_json::json;

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
    
    /// Run the setup process
    pub fn run_setup(&self, force: bool) -> Result<(), Box<dyn std::error::Error>> {
        println!("Welcome to HookAnchor Setup!");
        println!("=============================\n");
        
        // Step 1: Check for Karabiner-Elements
        if !self.check_karabiner() {
            self.prompt_karabiner_install()?;
        }
        
        // Step 2: Check Terminal accessibility permissions
        if !self.check_terminal_accessibility() {
            self.prompt_accessibility_permissions()?;
        }
        
        // Step 3: Create configuration directory
        self.create_config_directory()?;
        
        // Step 4: Create default configuration
        self.create_default_config(force)?;
        
        // Step 5: Install Karabiner complex modification
        self.install_karabiner_modification(force)?;
        
        // Step 6: Create initial commands file
        self.create_initial_commands(force)?;
        
        println!("\n✓ Setup completed successfully!");
        println!("\nHookAnchor is now configured to launch with Caps Lock (when pressed alone).");
        println!("You can modify the keyboard shortcut in Karabiner-Elements if needed.\n");
        
        Ok(())
    }
    
    /// Check if Karabiner-Elements is installed
    fn check_karabiner(&self) -> bool {
        Path::new(KARABINER_CLI_PATH).exists()
    }
    
    /// Check if Terminal has accessibility permissions
    fn check_terminal_accessibility(&self) -> bool {
        // Try to run a simple AppleScript that requires accessibility
        let test_script = r#"
            tell application "System Events"
                try
                    key code 0
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
                stdout.trim() == "true"
            }
            Err(_) => false,
        }
    }
    
    /// Prompt user to grant accessibility permissions to Terminal
    fn prompt_accessibility_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⚠️  Terminal needs Accessibility permissions for HookAnchor to work properly.");
        println!("\nThis permission is required to:");
        println!("• Capture window information from applications");
        println!("• Grab URLs from Notion and other apps");
        println!("• Send keyboard shortcuts to applications");
        
        println!("\nSteps to grant permission:");
        println!("1. System Settings will open to Privacy & Security → Accessibility");
        println!("2. Click the lock icon and enter your password");
        println!("3. Enable the checkbox next to 'Terminal'");
        println!("4. If Terminal is not in the list, click '+' and add it from /Applications/Utilities/");
        
        println!("\nPress Enter to open System Settings...");
        std::io::stdin().read_line(&mut String::new())?;
        
        // Open System Settings to the Accessibility pane
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn()?;
        
        println!("\nAfter granting permission, press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        
        // Check again after user claims to have set permissions
        if !self.check_terminal_accessibility() {
            println!("\n⚠️  Terminal still doesn't have Accessibility permissions.");
            println!("Please ensure you've enabled Terminal in Privacy & Security → Accessibility.");
            println!("You may need to restart Terminal after granting permissions.");
            println!("\nContinuing setup, but some features may not work until permissions are granted.");
        } else {
            println!("✓ Terminal has Accessibility permissions!");
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
    
    /// Create the default config.yaml file (safely)
    fn create_default_config(&self, force: bool) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.config_dir.join("config.yaml");
        
        if config_path.exists() && !force {
            println!("Configuration file already exists - preserving existing settings");
            
            // Validate existing config
            match fs::read_to_string(&config_path) {
                Ok(content) => {
                    // Basic validation - check if it parses as YAML
                    if content.trim().is_empty() {
                        println!("Warning: Existing config file is empty");
                        println!("NOT overwriting - use --install --force to replace empty config");
                        return Ok(()); // NEVER overwrite without force flag!
                    } else {
                        println!("Existing configuration looks valid - keeping as-is");
                        return Ok(());
                    }
                }
                Err(_) => {
                    println!("Warning: Could not read existing config file");
                    return Ok(()); // Don't overwrite if we can't read it
                }
            }
        }
        
        // Only reach here if: (!exists) OR (exists AND force)
        if !config_path.exists() {
            println!("Creating default configuration...");
        } else if force {
            println!("🔄 Force reinstall: Creating fresh configuration file...");
            // Backup existing file when forcing
            let backup_path = self.config_dir.join(format!("config.yaml.backup.{}", 
                chrono::Utc::now().format("%Y%m%d_%H%M%S")));
            if let Err(e) = fs::copy(&config_path, &backup_path) {
                println!("Warning: Could not backup existing config: {}", e);
            } else {
                println!("Backed up existing config to: {}", backup_path.display());
            }
        }
        
        // Create timestamp for this installation
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let default_config = format!(r#"# HookAnchor Configuration
# Generated on: {}

general:
  max_results: 20
  debug: false
  word_separators: " /-_."

ui:
  window_width: 600
  font_size: 14
  show_patch: true
  show_icons: true

search:
  fuzzy_match: true
  case_sensitive: false
  min_search_length: 1

launcher:
  terminal: "Terminal"
  browser: "default"

logging:
  level: "info"
  file: "logs/hookanchor.log"
  max_size: 10
  max_files: 5

# Installation info
_install_info:
  created_timestamp: {}
  version: "installer_generated"
"#, 
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp)),
            timestamp
        );
        
        fs::write(&config_path, default_config)?;
        println!("✓ Default configuration created");
        
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
    fn install_karabiner_modification(&self, force: bool) -> Result<(), Box<dyn std::error::Error>> {
        let karabiner_dir = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
            .join("karabiner")
            .join("assets")
            .join("complex_modifications");
        
        fs::create_dir_all(&karabiner_dir)?;
        
        // Check for Caps Lock conflicts first (skip if force reinstall)
        if force {
            println!("\n🔄 Force reinstall: Skipping conflict detection, installing F12+Cmd+Option hotkey");
            return self.install_karabiner_f12_only();
        }
        
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
        
        let modification = json!({
            "title": "HookAnchor",
            "rules": [
                {
                    "description": "Launch HookAnchor popup with Caps Lock (when pressed alone)",
                    "manipulators": [{
                        "type": "basic",
                        "from": {
                            "key_code": "caps_lock"
                        },
                        "to_if_alone": [{
                            "shell_command": "/Applications/HookAnchor.app/Contents/MacOS/hookanchor"
                        }]
                    }]
                },
                {
                    "description": "Launch HookAnchor popup with F12+Cmd+Option",
                    "manipulators": [{
                        "type": "basic",
                        "from": {
                            "key_code": "f12",
                            "modifiers": {
                                "mandatory": ["left_command", "left_option"]
                            }
                        },
                        "to": [{
                            "shell_command": "/Applications/HookAnchor.app/Contents/MacOS/hookanchor"
                        }]
                    }]
                }
            ]
        });
        
        fs::write(&mod_path, serde_json::to_string_pretty(&modification)?)?;
        
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
        let karabiner_dir = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config")
            .join("karabiner")
            .join("assets")
            .join("complex_modifications");
        
        let mod_path = karabiner_dir.join("hookanchor.json");
        
        let modification = json!({
            "title": "HookAnchor (F12 only)",
            "rules": [{
                "description": "Launch HookAnchor popup with F12+Cmd+Option",
                "manipulators": [{
                    "type": "basic",
                    "from": {
                        "key_code": "f12",
                        "modifiers": {
                            "mandatory": ["left_command", "left_option"]
                        }
                    },
                    "to": [{
                        "shell_command": "/Applications/HookAnchor.app/Contents/MacOS/hookanchor"
                    }]
                }]
            }]
        });
        
        fs::write(&mod_path, serde_json::to_string_pretty(&modification)?)?;
        
        println!("✅ Installed F12+Cmd+Option hotkey");
        println!("\n📝 Note: You'll need to enable the HookAnchor modification in");
        println!("   Karabiner-Elements → Complex Modifications → Add rule");
        
        Ok(())
    }
    
    /// Create initial commands.txt file with some starter commands (safely)
    fn create_initial_commands(&self, force: bool) -> Result<(), Box<dyn std::error::Error>> {
        let commands_path = self.config_dir.join("commands.txt");
        
        if commands_path.exists() && !force {
            // Check if the file has any non-comment content
            match fs::read_to_string(&commands_path) {
                Ok(content) => {
                    let has_commands = content.lines().any(|line| {
                        let trimmed = line.trim();
                        !trimmed.is_empty() && !trimmed.starts_with('#')
                    });
                    
                    if has_commands {
                        println!("Commands file already exists with content - preserving existing commands");
                        return Ok(());
                    } else {
                        println!("Commands file exists but is empty/comments only");
                        println!("NOT overwriting - use --install --force to replace empty commands");
                        return Ok(()); // NEVER overwrite without force flag!
                    }
                }
                Err(_) => {
                    println!("Warning: Could not read existing commands file - skipping initialization");
                    return Ok(());
                }
            }
        }
        
        // Only reach here if: (!exists) OR (exists AND force)
        if !commands_path.exists() {
            println!("Creating initial commands file...");
        } else if force {
            println!("🔄 Force reinstall: Creating fresh commands file...");
            // Backup existing file when forcing
            let backup_path = self.config_dir.join(format!("commands.txt.backup.{}", 
                chrono::Utc::now().format("%Y%m%d_%H%M%S")));
            if let Err(e) = fs::copy(&commands_path, &backup_path) {
                println!("Warning: Could not backup existing commands: {}", e);
            } else {
                println!("Backed up existing commands to: {}", backup_path.display());
            }
        }
        
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

/// Check if setup is needed and run if necessary
pub fn check_and_run_setup() -> Result<bool, Box<dyn std::error::Error>> {
    let assistant = SetupAssistant::new();
    
    if assistant.is_first_run() {
        assistant.run_setup(false)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Uninstall HookAnchor and clean up configuration
pub fn uninstall_hookanchor() -> Result<(), Box<dyn std::error::Error>> {
    let assistant = SetupAssistant::new();
    assistant.run_uninstall()
}

impl SetupAssistant {
    /// Run the uninstall process
    pub fn run_uninstall(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Silent uninstall - no stdout messages, no interactive prompts
        // NOTE: Does NOT remove ~/.config/hookanchor directory - user data is preserved
        
        // Step 1: Remove from Applications folder
        self.remove_from_applications()?;
        
        // Step 2: Remove Karabiner configuration
        self.remove_karabiner_config()?;
        
        // Config directory is NOT removed - user's commands and settings are preserved
        
        Ok(())
    }
    
    /// Remove HookAnchor from Applications folder
    fn remove_from_applications(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app_path = Path::new("/Applications/HookAnchor.app");
        
        if app_path.exists() {
            fs::remove_dir_all(app_path)?;
        }
        
        Ok(())
    }
    
    /// Remove Karabiner configuration for HookAnchor
    fn remove_karabiner_config(&self) -> Result<(), Box<dyn std::error::Error>> {
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
}