//! Build verification system
//!
//! Ensures that running binaries were built with the Just build system and that
//! the source code matches what's currently running. This prevents accidentally
//! running stale binaries or binaries built outside the official build process.

use chrono::{Local, TimeZone};

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
        warnings.push(
            "⚠️  Binary was built with 'cargo build' instead of 'just build'".to_string()
        );
        warnings.push(
            "   This bypasses build verification. Use 'just build' for tracked builds.".to_string()
        );
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

/// Log build metadata at startup (informational)
pub fn log_build_info() {
    let metadata = BuildMetadata::get();

    crate::utils::log("═══════════════════════════════════════════════════════");
    crate::utils::log("BUILD INFORMATION");
    crate::utils::log("═══════════════════════════════════════════════════════");

    // Build timestamp
    crate::utils::log(&format!("  Built: {}", metadata.build_timestamp_str));

    // Age of binary
    if metadata.build_timestamp > 0 {
        let age_seconds = chrono::Utc::now().timestamp() - metadata.build_timestamp;
        if age_seconds < 60 {
            crate::utils::log(&format!("  Age: {} seconds (very recent!)", age_seconds));
        } else if age_seconds < 3600 {
            crate::utils::log(&format!("  Age: {} minutes", age_seconds / 60));
        } else if age_seconds < 86400 {
            crate::utils::log(&format!("  Age: {:.1} hours", age_seconds as f64 / 3600.0));
        } else {
            crate::utils::log(&format!("  Age: {:.1} days", age_seconds as f64 / 86400.0));
        }
    }

    // Build method
    if metadata.built_with_just {
        crate::utils::log("  Build method: Just (verified ✓)");
    } else {
        crate::utils::log("  Build method: cargo (unverified ⚠️)");
    }

    // Git information
    if let Some(commit) = metadata.git_commit {
        crate::utils::log(&format!("  Git commit: {}", &commit[..8.min(commit.len())]));
    }
    if let Some(branch) = metadata.git_branch {
        crate::utils::log(&format!("  Git branch: {}", branch));
    }

    crate::utils::log("═══════════════════════════════════════════════════════");
}

/// Verify build and log results
///
/// This is the main entry point called from sys_data::initialize()
/// Returns true if verification passed, false otherwise
pub fn verify_and_log() -> bool {
    // Always log build information
    log_build_info();

    // Run verification
    let result = verify_build();

    // Log warnings
    for warning in &result.warnings {
        crate::utils::log(warning);
    }

    // Log errors
    for error in &result.errors {
        crate::utils::log_error(error);
    }

    // Summary
    if result.passed {
        if result.warnings.is_empty() {
            crate::utils::log("✅ Build verification PASSED - all checks OK");
        } else {
            crate::utils::log(&format!(
                "✅ Build verification PASSED with {} warnings",
                result.warnings.len()
            ));
        }
    } else {
        crate::utils::log_error(&format!(
            "❌ Build verification FAILED - {} errors found",
            result.errors.len()
        ));
        crate::utils::log_error("   Running binary may be corrupted or improperly built");
    }

    crate::utils::log(""); // Blank line for separation

    result.passed
}
