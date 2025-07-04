use anchor_selector::{load_config, core::config::get_config_file_path};
use std::fs;

fn main() {
    let config_path = get_config_file_path();
    println!("Loading config from: {:?}", config_path);
    println!("Config file exists: {}", config_path.exists());
    
    // Read and parse YAML directly
    if let Ok(contents) = fs::read_to_string(&config_path) {
        println!("Config file content length: {} bytes", contents.len());
        
        // Try to parse as raw YAML first
        match serde_yaml::from_str::<serde_yaml::Value>(&contents) {
            Ok(yaml) => {
                println!("YAML parsed successfully");
                if let Some(markdown_roots_yaml) = yaml.get("markdown_roots") {
                    println!("Raw markdown_roots YAML: {:?}", markdown_roots_yaml);
                    
                    // Try parsing as Vec<String>
                    match serde_yaml::from_value::<Vec<String>>(markdown_roots_yaml.clone()) {
                        Ok(roots) => println!("Parsed {} roots: {:?}", roots.len(), roots),
                        Err(e) => println!("Failed to parse markdown_roots: {}", e),
                    }
                } else {
                    println!("No markdown_roots found in YAML!");
                }
            }
            Err(e) => println!("Failed to parse YAML: {}", e),
        }
    }
    
    let config = load_config();
    
    match &config.markdown_roots {
        Some(roots) => {
            println!("Found {} markdown roots:", roots.len());
            for (i, root) in roots.iter().enumerate() {
                println!("  {}: {}", i+1, root);
            }
        }
        None => {
            println!("No markdown_roots found in config!");
        }
    }
}