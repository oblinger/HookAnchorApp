use std::fs;
use std::path::Path;
use std::env;

fn main() {
    // Update build time in state.json file
    let state_path = get_state_file_path();
    
    // Get current timestamp
    let build_time = chrono::Local::now().timestamp();
    
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
}

fn get_state_file_path() -> std::path::PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".config/hookanchor/state.json")
}