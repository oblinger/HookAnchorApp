//! Test Comma-Separated Actions
//!
//! Tests that the comma-separated listed_actions parsing works correctly

use hookanchor::{get_listed_actions, Config};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Comma-Separated Actions ===\n");
    
    // Test 1: Default actions (when config doesn't have listed_actions)
    println!("🔧 Testing default actions...");
    let default_actions = get_listed_actions();
    println!("  Default actions: {:?}", default_actions);
    assert_eq!(default_actions.len(), 6);
    assert!(default_actions.contains(&"app".to_string()));
    assert!(default_actions.contains(&"anchor".to_string()));
    
    // Test 2: Custom comma-separated string
    println!("\n📝 Testing custom comma-separated string...");
    let test_cases = vec![
        ("app,url,folder", vec!["app", "url", "folder"]),
        ("app, url, folder", vec!["app", "url", "folder"]), // with spaces
        ("  app  ,  url  ,  folder  ", vec!["app", "url", "folder"]), // lots of spaces
        ("one", vec!["one"]), // single action
        ("", vec![]), // empty string
        ("a,b,c,d,e", vec!["a", "b", "c", "d", "e"]), // many actions
    ];
    
    for (input, expected) in test_cases {
        let parsed: Vec<String> = input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        let expected_strings: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
        println!("  \"{}\" -> {:?}", input, parsed);
        assert_eq!(parsed, expected_strings);
    }
    
    println!("\n🚀 All comma-separated action tests passed!");
    
    Ok(())
}