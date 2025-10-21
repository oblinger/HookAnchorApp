use std::fs;
use std::path::Path;
use std::env;

fn main() {
    // =========================================================================
    // EMBEDDED BUILD METADATA - Used for runtime verification
    // =========================================================================

    // Embed build timestamp (UTC epoch seconds)
    let build_timestamp = chrono::Utc::now().timestamp();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", build_timestamp);

    // Embed build timestamp as human-readable string
    let build_timestamp_str = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    println!("cargo:rustc-env=BUILD_TIMESTAMP_STR={}", build_timestamp_str);

    // Embed git commit hash if available
    if let Ok(output) = std::process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output() {
        if output.status.success() {
            let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-env=GIT_COMMIT={}", commit);
        }
    }

    // Embed git branch if available
    if let Ok(output) = std::process::Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output() {
        if output.status.success() {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-env=GIT_BRANCH={}", branch);
        }
    }

    // Check if built with Just by looking for JUST environment variable
    // Just sets this when running recipes
    let built_with_just = env::var("JUST").is_ok();
    println!("cargo:rustc-env=BUILT_WITH_JUST={}", built_with_just);

    // =========================================================================
    // Update build time in state.json file (legacy compatibility)
    // =========================================================================

    let state_path = get_state_file_path();

    // Use the same timestamp for state.json
    let build_time = build_timestamp;

    // Load existing state or create default if needed
    let mut state = if let Ok(content) = fs::read_to_string(&state_path) {
        // Try to parse existing state.json
        match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(mut json) => {
                // Update build_time field
                json["build_time"] = serde_json::Value::from(build_time);
                json
            }
            Err(_) => {
                // Invalid JSON, create new state
                serde_json::json!({
                    "window_position": null,
                    "build_time": build_time,
                    "last_scan_time": null,
                    "last_scan_checksum": null
                })
            }
        }
    } else {
        // No existing file, create new state
        serde_json::json!({
            "window_position": null,
            "build_time": build_time,
            "last_scan_time": null,
            "last_scan_checksum": null
        })
    };

    // Ensure build_time is set to current time
    state["build_time"] = serde_json::Value::from(build_time);

    // Ensure config directory exists
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent).ok();
    }

    // Write updated state
    if let Err(e) = fs::write(&state_path, serde_json::to_string_pretty(&state).unwrap_or_default()) {
        println!("cargo:warning=Failed to update build time: {}", e);
    }
    
    // macOS: Set up icon embedding
    #[cfg(target_os = "macos")]
    {
        // Try to find the icon file in the app bundle
        let icon_path = "/Applications/HookAnchor.app/Contents/Resources/applet.icns";
        
        if Path::new(icon_path).exists() {
            println!("cargo:rustc-env=MACOS_ICON_PATH={}", icon_path);
        } else {
            // Fallback to source directory icon if it exists
            let fallback_icon = "resources/applet.icns";
            if Path::new(fallback_icon).exists() {
                println!("cargo:rustc-env=MACOS_ICON_PATH={}", fallback_icon);
            }
        }
    }
}

fn get_state_file_path() -> std::path::PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/hookanchor/state.json")
}