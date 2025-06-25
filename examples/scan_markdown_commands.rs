//! Scan Markdown Commands Example
//!
//! Demonstrates the integrated markdown scanning functionality
//! using JavaScript business logic.

use anchor_selector::business_logic::update_commands_from_markdown;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Markdown Command Scanner ===\n");
    println!("This will scan your markdown files and update the command list.\n");
    
    println!("Scanning directories:");
    println!("  - ~/documents");
    println!("  - ~/notes");
    println!("  - ~/obsidian");
    println!("  - ~/projects");
    println!();
    
    match update_commands_from_markdown() {
        Ok(count) => {
            println!("\n✅ Successfully updated command list!");
            println!("   Commands added/updated: {}", count);
            println!("\nYou can now use these commands in the anchor selector.");
        }
        Err(e) => {
            eprintln!("\n❌ Error scanning markdown files: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("  - Check that directories exist");
            eprintln!("  - Ensure markdown files have proper frontmatter");
            eprintln!("  - Look for error messages in the output above");
        }
    }
    
    Ok(())
}