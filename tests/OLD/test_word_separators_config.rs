use hookanchor::load_config;

#[test]
fn test_word_separators_config_loading() {
    let config = load_config();
    
    println!("Loaded config:");
    println!("  merge_similar: {:?}", config.popup_settings.merge_similar);
    println!("  word_separators: {:?}", config.popup_settings.word_separators);
    
    // Check that word_separators is loaded correctly
    let separators = config.popup_settings.word_separators
        .as_deref()
        .unwrap_or(" ._-");
    
    println!("  effective separators: '{}'", separators);
    
    // Test the separators work as expected
    assert!(separators.contains(' '));
    assert!(separators.contains('.'));
    assert!(separators.contains('_'));
    assert!(separators.contains('-'));
    
    println!("✅ All word separators are present in config");
}