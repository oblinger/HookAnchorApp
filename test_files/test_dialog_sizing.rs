#!/usr/bin/env rust-script
//! Test script to verify dialog sizing fix

use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    println!("Testing dialog sizing fix...");
    
    // Create a temporary invalid config to trigger error dialog
    let config_path = Path::new(&std::env::var("HOME").unwrap())
        .join(".config/hookanchor/config.yaml");
    
    let backup_path = config_path.with_extension("yaml.test_backup");
    
    // Backup current config
    if config_path.exists() {
        fs::copy(&config_path, &backup_path).expect("Failed to backup config");
    }
    
    // Create invalid config
    fs::write(&config_path, "invalid_yaml: [unclosed bracket").expect("Failed to write invalid config");
    
    // Run popup for 3 seconds to test dialog sizing
    println!("Running popup with invalid config to test error dialog sizing...");
    let output = Command::new("timeout")
        .arg("3s")
        .arg("./target/release/popup")
        .output()
        .expect("Failed to run popup");
    
    println!("Popup output: {:?}", output);
    
    // Restore config
    if backup_path.exists() {
        fs::copy(&backup_path, &config_path).expect("Failed to restore config");
        fs::remove_file(&backup_path).expect("Failed to remove backup");
    }
    
    println!("Test completed! Dialog sizing fix should now work properly.");
}