use anchor_selector::load_config;

fn main() {
    let config = load_config();
    println!("Config loaded:");
    println!("  max_rows: {}", config.settings.max_rows);
    
    // Also print the expected file path
    use std::env;
    use std::path::Path;
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let config_path = Path::new(&home).join(".config/anchor_selector/config.yaml");
    println!("  Expected path: {}", config_path.display());
    
    // Check if file exists
    if config_path.exists() {
        println!("  File exists: YES");
        if let Ok(contents) = std::fs::read_to_string(&config_path) {
            println!("  File contents:\n{}", contents);
        }
    } else {
        println!("  File exists: NO");
    }
}