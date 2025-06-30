use anchor_selector::{load_commands, filter_commands, merge_similar_commands, get_current_submenu_prefix};

#[test]
fn test_fin_vs_fina_merging() {
    println!("\n=== Testing 'fin' vs 'fina' candidate validation ===");
    
    // Test with "fin" - should merge
    test_search("fin");
    
    // Test with "fina" - should NOT merge  
    test_search("fina");
}

fn test_search(search_text: &str) {
    println!("\n--- Testing with search: '{}' ---", search_text);
    
    let all_commands = load_commands();
    let filtered = filter_commands(&all_commands, search_text, 100, false);
    
    // Check submenu prefix
    let submenu_prefix = get_current_submenu_prefix(&filtered, search_text);
    println!("Submenu prefix: {:?}", submenu_prefix);
    
    // Apply merging
    let merged = merge_similar_commands(&filtered, search_text);
    
    // Look for FIN Accounts specifically
    println!("\nFIN Accounts-related commands after merging:");
    for cmd in &merged {
        if cmd.command.to_uppercase().contains("ACCOUNT") {
            println!("  - '{}'", cmd.command);
        }
    }
    
    // Check if FIN Accounts got merged
    let has_merged_accounts = merged.iter().any(|cmd| cmd.command == "FIN Accounts...");
    let has_separate_accounts = merged.iter().any(|cmd| cmd.command == "FIN Accounts");
    
    if has_merged_accounts {
        println!("✅ FIN Accounts was MERGED (as expected for '{}')", search_text);
    } else if has_separate_accounts {
        println!("❌ FIN Accounts was NOT merged (split - as expected for '{}')", search_text);
    }
    
    println!("\nTotal commands: {} -> {} after merging", filtered.len(), merged.len());
}

#[test]
fn test_validation_logic_directly() {
    println!("\n=== Direct validation logic test ===");
    
    // Mock the validation logic
    let separators = " ._-";
    
    // Test case 1: "fin" with "FIN Accounts"
    let candidate1 = "FIN Accounts";
    let prefix1 = "fin";
    println!("\nTest: prefix='{}', candidate='{}'", prefix1, candidate1);
    println!("  Match end position: {}", prefix1.len());
    println!("  Check position (one back): {}", prefix1.len() - 1);
    if let Some(ch) = candidate1.chars().nth(prefix1.len() - 1) {
        println!("  Character at check position: '{}'", ch);
        println!("  Is separator? {}", separators.contains(ch));
        if !separators.contains(ch) {
            let remaining = &candidate1[prefix1.len()..];
            let has_separator = remaining.chars().any(|c| separators.contains(c));
            println!("  Remaining text: '{}'", remaining);
            println!("  Has separator after match? {}", has_separator);
            println!("  → Should merge: {}", has_separator);
        }
    }
    
    // Test case 2: "fina" with "FIN Accounts"  
    let candidate2 = "FIN Accounts";
    let prefix2 = "fina";
    println!("\nTest: prefix='{}', candidate='{}'", prefix2, candidate2);
    
    // Check if it even matches
    if candidate2.to_lowercase().starts_with(&prefix2.to_lowercase()) {
        println!("  ❌ Candidate doesn't start with prefix!");
    } else {
        println!("  Candidate would not match this prefix anyway");
    }
    
    // Actually "fina" would be checked against "FIN Accounts" where FIN is the submenu prefix
    let prefix2_corrected = "FIN";
    println!("\nCorrected test: active_prefix='{}', search='{}', candidate='{}'", 
        prefix2_corrected, "fina", candidate2);
    
    // In this case, the active prefix for validation would still be "FIN"
    // but we need to check if the user has typed beyond word boundaries
}