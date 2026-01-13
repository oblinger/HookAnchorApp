//! Build verification system
//!
//! Ensures that running binaries were built with the Just build system and that
//! the source code matches what's currently running. This prevents accidentally
//! running stale binaries or binaries built outside the official build process.
//!
//! Also checks config version compatibility.

use chrono;
use crate::prelude::*;

/// Get the project directory by deriving from the executable location
/// Binary is at <project>/target/release/<binary>, so go up 3 levels
pub fn get_project_dir() -> Option<std::path::PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))  // release/
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))  // target/
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))  // project/
}

/// Get a shell-safe string for the project directory (escapes spaces)
pub fn get_project_dir_shell() -> String {
    get_project_dir()
        .map(|p| p.display().to_string().replace(' ', "\\ "))
        .unwrap_or_else(|| "<project-dir>".to_string())
}

/// Build metadata embedded at compile time by build.rs
pub struct BuildMetadata {
    /// Timestamp when binary was built (UTC epoch seconds)
    pub build_timestamp: i64,
    /// Human-readable build timestamp
    pub build_timestamp_str: &'static str,
    /// Git commit hash (if available)
    pub git_commit: Option<&'static str>,
    /// Git branch name (if available)
    pub git_branch: Option<&'static str>,
    /// Whether binary was built with Just command
    pub built_with_just: bool,
}

impl BuildMetadata {
    /// Get build metadata embedded in this binary
    pub fn get() -> Self {
        Self {
            build_timestamp: option_env!("BUILD_TIMESTAMP")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            build_timestamp_str: option_env!("BUILD_TIMESTAMP_STR").unwrap_or("unknown"),
            git_commit: option_env!("GIT_COMMIT"),
            git_branch: option_env!("GIT_BRANCH"),
            built_with_just: option_env!("BUILT_WITH_JUST")
                .map(|s| s == "true")
                .unwrap_or(false),
        }
    }
}

/// Result of build verification check
#[derive(Debug)]
pub struct VerificationResult {
    /// Whether verification passed
    pub passed: bool,
    /// List of warnings/issues found
    pub warnings: Vec<String>,
    /// Critical errors (verification failures)
    pub errors: Vec<String>,
}

/// Verify build consistency at startup
///
/// This function checks:
/// 1. Binary was built with Just (not cargo directly)
/// 2. Build timestamp is reasonable (not in future, not too old)
/// 3. Git information is available (for source tracking)
///
/// Call this from sys_data::initialize() to ensure running code matches source
pub fn verify_build() -> VerificationResult {
    let metadata = BuildMetadata::get();
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    // Check 1: Was binary built with Just?
    if !metadata.built_with_just {
        errors.push("".to_string());
        errors.push("❌ ❌ ❌ BUILD VERIFICATION FAILED ❌ ❌ ❌".to_string());
        errors.push("".to_string());
        errors.push("   Binary was built with 'cargo build' instead of 'just build'!".to_string());
        errors.push("".to_string());
        errors.push("   ❌ DO NOT USE: cargo build --release".to_string());
        errors.push("   ✅ ALWAYS USE:  just build".to_string());
        errors.push("".to_string());
        errors.push(format!("   Rebuild now with: cd {} && just build", get_project_dir_shell()));
        errors.push("".to_string());
        errors.push("❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌ ❌".to_string());
        errors.push("".to_string());
    }

    // Check 2: Validate build timestamp
    let current_time = chrono::Utc::now().timestamp();

    if metadata.build_timestamp == 0 {
        errors.push("❌ Build timestamp not found in binary".to_string());
        errors.push("   Binary may be corrupted or built incorrectly".to_string());
    } else {
        // Check if build timestamp is in the future (clock skew or corruption)
        if metadata.build_timestamp > current_time + 300 {  // 5 min tolerance
            errors.push(format!(
                "❌ Build timestamp is in the future: {}",
                metadata.build_timestamp_str
            ));
            errors.push("   This indicates clock skew or binary corruption".to_string());
        }

        // Warn if binary is very old (more than 7 days)
        let age_seconds = current_time - metadata.build_timestamp;
        if age_seconds > 604_800 {  // 7 days
            let age_days = age_seconds / 86_400;
            warnings.push(format!(
                "⚠️  Binary is {} days old (built: {})",
                age_days, metadata.build_timestamp_str
            ));
            warnings.push("   Consider rebuilding to get latest changes".to_string());
        }
    }

    // Check 3: Git information available?
    if metadata.git_commit.is_none() {
        warnings.push("⚠️  Git commit hash not embedded in binary".to_string());
        warnings.push("   Source tracking unavailable - was build run outside git repo?".to_string());
    }

    if metadata.git_branch.is_none() {
        warnings.push("⚠️  Git branch not embedded in binary".to_string());
    }

    // Determine overall pass/fail
    let passed = errors.is_empty();

    VerificationResult {
        passed,
        warnings,
        errors,
    }
}

/// Check if developer mode is enabled based on configuration
///
/// Developer mode logic:
/// - If config value is "true" (case-insensitive), always enable developer mode
/// - If config value is "false" (case-insensitive), always disable developer mode
/// - If config value is any other string, compare it to the current hostname (case-insensitive)
///   - If it matches the hostname, enable developer mode
///   - If it doesn't match, disable developer mode
/// - If config value is None, default to false (disabled)
///
/// # Returns
/// * `true` if developer mode should be enabled
/// * `false` if developer mode should be disabled
pub fn is_developer_mode() -> bool {
    // Get config
    let config = crate::core::data::get_config();

    // Get developer_mode setting from config
    let developer_mode_value = match &config.popup_settings.developer_mode {
        Some(value) => value.trim(),
        None => return false,  // Default: developer mode disabled
    };

    // Check for explicit "true" or "false"
    let lower_value = developer_mode_value.to_lowercase();
    if lower_value == "true" {
        return true;
    }
    if lower_value == "false" {
        return false;
    }

    // Otherwise, treat as hostname and compare to current machine's hostname
    let current_hostname = match std::process::Command::new("hostname")
        .output()
    {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => return false,  // Can't get hostname, default to disabled
    };

    // Case-insensitive hostname comparison
    developer_mode_value.eq_ignore_ascii_case(&current_hostname)
}

/// Log build metadata at startup (informational)
pub fn log_build_info() {
    let metadata = BuildMetadata::get();

    log("═══════════════════════════════════════════════════════");
    log("BUILD INFORMATION");
    log("═══════════════════════════════════════════════════════");

    // Build timestamp
    log(&format!("  Built: {}", metadata.build_timestamp_str));

    // Age of binary
    if metadata.build_timestamp > 0 {
        let age_seconds = chrono::Utc::now().timestamp() - metadata.build_timestamp;
        if age_seconds < 60 {
            log(&format!("  Age: {} seconds (very recent!)", age_seconds));
        } else if age_seconds < 3600 {
            log(&format!("  Age: {} minutes", age_seconds / 60));
        } else if age_seconds < 86400 {
            log(&format!("  Age: {:.1} hours", age_seconds as f64 / 3600.0));
        } else {
            log(&format!("  Age: {:.1} days", age_seconds as f64 / 86400.0));
        }
    }

    // Build method
    if metadata.built_with_just {
        log("  Build method: Just (verified ✓)");
    } else {
        log("  Build method: cargo (unverified ⚠️)");
    }

    // Git information
    if let Some(commit) = metadata.git_commit {
        log(&format!("  Git commit: {}", &commit[..8.min(commit.len())]));
    }
    if let Some(branch) = metadata.git_branch {
        log(&format!("  Git branch: {}", branch));
    }

    log("═══════════════════════════════════════════════════════");
}

/// Show error dialog and terminate the application
///
/// This is called when build verification fails and terminate_on_failure is true
fn show_error_dialog_and_exit(errors: &[String], warnings: &[String]) -> ! {
    let metadata = BuildMetadata::get();

    // Build clean, focused error message
    let mut message = String::from("BUILD VERIFICATION FAILED\n\n");
    message.push_str("Use:\n");
    message.push_str(&format!("  cd {} && just build\n\n\n", get_project_dir_shell()));
    message.push_str("Build details:\n");
    message.push_str(&format!("  Binary timestamp: {}\n", metadata.build_timestamp_str));
    message.push_str(&format!("  Current time: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    if let Some(commit) = metadata.git_commit {
        message.push_str(&format!("  Git commit: {}\n", &commit[..8.min(commit.len())]));
    }
    if let Some(branch) = metadata.git_branch {
        message.push_str(&format!("  Git branch: {}\n", branch));
    }

    // Show native dialog
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"display dialog "{}" buttons {{"OK"}} default button "OK" with icon stop with title "Build Verification Failed""#,
                message.replace('\n', "\\n").replace('"', "\\\"")
            ))
            .output();
    }

    // Log detailed error information
    let mut log_message = String::from("❌ BUILD VERIFICATION FAILED ❌\n\n");
    log_message.push_str("The binary was not built correctly.\n\n");

    if !errors.is_empty() {
        log_message.push_str("ERRORS:\n");
        for error in errors {
            log_message.push_str("  ");
            log_message.push_str(error);
            log_message.push_str("\n");
        }
        log_message.push_str("\n");
    }

    if !warnings.is_empty() {
        log_message.push_str("WARNINGS:\n");
        for warning in warnings {
            log_message.push_str("  ");
            log_message.push_str(warning);
            log_message.push_str("\n");
        }
        log_message.push_str("\n");
    }

    log_message.push_str("SOLUTION:\n");
    log_message.push_str(&format!("  Rebuild with: cd {} && just build\n", get_project_dir_shell()));

    log_error(&log_message);

    // Terminate
    std::process::exit(1);
}

/// Verify build and log results
///
/// This is the main entry point called from sys_data::initialize()
///
/// # Arguments
/// * `terminate_on_failure` - If true, show error dialog and terminate on verification failure
///
/// # Returns
/// * `true` if verification passed
/// * `false` if verification failed (only returned when terminate_on_failure is false)
///
/// # Panics
/// Calls std::process::exit(1) if verification fails and terminate_on_failure is true
pub fn verify_and_log(terminate_on_failure: bool) -> bool {
    // Always log build information
    log_build_info();

    // Check if we're in developer mode
    if !is_developer_mode() {
        log("Developer mode: DISABLED (skipping build verification)");
        log(""); // Blank line for separation
        return true;  // Skip verification, return success
    }

    log("Developer mode: ENABLED (performing build verification)");

    // Simple log message indicating we're checking
    log("Checking build validity...");

    // Run verification
    let result = verify_build();

    // Log warnings
    for warning in &result.warnings {
        log(warning);
    }

    // Log errors (both to file and stdout so they're visible)
    for error in &result.errors {
        log_error(error);
        crate::utils::print(error);  // Also print to stdout
    }

    // Summary
    if result.passed {
        log("Build validity check: SUCCESS");
    } else {
        log_error("Build validity check: FAILED");
        crate::utils::print("Build validity check: FAILED");  // Also print to stdout

        // If terminate_on_failure is true, show dialog and exit
        if terminate_on_failure {
            show_error_dialog_and_exit(&result.errors, &result.warnings);
        }
    }

    log(""); // Blank line for separation

    result.passed
}

/// Minimum required config version
/// Config files older than this version are incompatible with this build
const MIN_CONFIG_VERSION: &str = "0.19.0";

/// Parse version string to comparable tuple (major, minor, patch)
fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    let major = parts[0].parse().ok()?;
    let minor = parts[1].parse().ok()?;
    let patch = parts[2].parse().ok()?;

    Some((major, minor, patch))
}

/// Compare two version strings
/// Returns true if version1 >= version2
fn version_gte(version1: &str, version2: &str) -> bool {
    let v1 = match parse_version(version1) {
        Some(v) => v,
        None => return false,
    };

    let v2 = match parse_version(version2) {
        Some(v) => v,
        None => return false,
    };

    v1 >= v2
}

/// Check config version compatibility
///
/// Returns:
/// - Ok(()) if config version is compatible or missing (dev machine)
/// - Err(message) if config version is too old
pub fn check_config_version() -> Result<(), String> {
    // Get config
    let config = crate::core::data::get_config();

    // If no config_version field, silently succeed (dev machine or old config)
    let config_version = match &config.config_version {
        Some(v) => v.trim(),
        None => {
            log("Config version: not specified (assuming dev machine)");
            return Ok(());
        }
    };

    log(&format!("Config version: {}", config_version));
    log(&format!("Minimum required: {}", MIN_CONFIG_VERSION));

    // Check if config version is recent enough
    if !version_gte(config_version, MIN_CONFIG_VERSION) {
        let config_dir = dirs::home_dir()
            .map(|h| h.join(".config/hookanchor"))
            .and_then(|p| p.to_str().map(String::from))
            .unwrap_or_else(|| "~/.config/hookanchor".to_string());

        let error_msg = format!(
            "Config version {} in {}/config.yaml is too old.\n\
             Minimum required: {}\n\n\
             Please copy configuration from {}/dist_config.yaml\n\
             or run the installer and check 'Update config.yaml'.",
            config_version,
            config_dir,
            MIN_CONFIG_VERSION,
            config_dir
        );

        return Err(error_msg);
    }

    log("Config version check: PASSED");
    Ok(())
}

/// Check config.js version (similar to config.yaml check)
pub fn check_config_js_version() -> Result<(), String> {
    let config_dir = dirs::home_dir()
        .map(|h| h.join(".config/hookanchor"))
        .ok_or("Cannot determine home directory")?;

    let config_js_path = config_dir.join("config.js");

    // If config.js doesn't exist, skip check (not required)
    if !config_js_path.exists() {
        log("config.js: not found (optional)");
        return Ok(());
    }

    // Read first 500 bytes to find version comment
    let content = std::fs::read_to_string(&config_js_path)
        .map_err(|e| format!("Failed to read config.js: {}", e))?;

    // Look for version comment in first few lines: // Version: 0.19.8
    let version_regex = regex::Regex::new(r"//\s*[Vv]ersion:\s*([0-9]+\.[0-9]+\.[0-9]+)")
        .map_err(|e| format!("Regex error: {}", e))?;

    let config_js_version = if let Some(captures) = version_regex.captures(&content) {
        captures.get(1).map(|m| m.as_str()).unwrap_or("")
    } else {
        // No version found - assume dev machine
        log("config.js version: not specified (assuming dev machine)");
        return Ok(());
    };

    log(&format!("config.js version: {}", config_js_version));
    log(&format!("Minimum required: {}", MIN_CONFIG_VERSION));

    // Check if config.js version is recent enough
    if !version_gte(config_js_version, MIN_CONFIG_VERSION) {
        let error_msg = format!(
            "config.js version {} in {}/config.js is too old.\n\
             Minimum required: {}\n\n\
             Please run the installer and check 'Force overwrite config files'.",
            config_js_version,
            config_dir.display(),
            MIN_CONFIG_VERSION
        );

        return Err(error_msg);
    }

    log("config.js version check: PASSED");
    Ok(())
}

/// Show config version error dialog and terminate
fn show_config_error_dialog_and_exit(error: &str) -> ! {
    // Show native dialog
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"display dialog "{}" buttons {{"OK"}} default button "OK" with icon stop with title "Config Version Too Old""#,
                error.replace('\n', "\\n").replace('"', "\\\"")
            ))
            .output();
    }

    // Log error
    log_error(&format!("❌ CONFIG VERSION CHECK FAILED\n\n{}", error));

    // Terminate
    std::process::exit(1);
}

/// Check config version and show error if too old
///
/// This should be called at startup after config is loaded.
/// If config version is too old, shows error dialog and terminates.
pub fn verify_config_version_or_exit() {
    // Check config.yaml version
    match check_config_version() {
        Ok(()) => {
            // Config version is OK
        }
        Err(error) => {
            // Config version is too old - show error and exit
            show_config_error_dialog_and_exit(&error);
        }
    }

    // Check config.js version
    match check_config_js_version() {
        Ok(()) => {
            // Config.js version is OK (or not present)
        }
        Err(error) => {
            // Config.js version is too old - show error and exit
            show_config_error_dialog_and_exit(&error);
        }
    }
}
