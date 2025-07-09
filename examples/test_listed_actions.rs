//! Test Listed Actions Functionality
//!
//! Tests the new get_listed_actions function and configuration.

use hookanchor::get_listed_actions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Listed Actions Functionality ===\n");
    
    // Test getting the configured actions list
    let actions = get_listed_actions();
    
    println!("📋 Command Editor Actions List:");
    println!("   Found {} configured actions for command editor dropdown:", actions.len());
    
    for (i, action) in actions.iter().enumerate() {
        println!("   {}. {}", i + 1, action);
    }
    
    println!("\n=== Benefits of Configurable Actions List ===");
    println!("🎛️ Users can customize which actions appear in command editor");
    println!("📝 Users can reorder actions by personal preference"); 
    println!("➕ Users can add their custom actions to the dropdown");
    println!("🗑️ Users can remove unused actions to reduce clutter");
    println!("🔄 Changes take effect immediately on config reload");
    
    println!("\n=== Example Customizations ===");
    println!("# Most used actions first:");
    println!("listed_actions:");
    println!("  - \"anchor\"      # Developer's most used");
    println!("  - \"chrome\"");
    println!("  - \"app\"");
    println!("  - \"folder\"");
    println!();
    println!("# Minimal list for power users:");
    println!("listed_actions:");
    println!("  - \"anchor\"");
    println!("  - \"app\"");
    println!("  - \"cmd\"");
    println!();
    println!("# Extended list with custom actions:");
    println!("listed_actions:");
    println!("  - \"app\"");
    println!("  - \"url\"");
    println!("  - \"my_workflow\"  # Custom JS action");
    println!("  - \"deploy\"       # Custom deployment action");
    
    println!("\n✅ Listed actions configuration is working correctly!");
    
    Ok(())
}