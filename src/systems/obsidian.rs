//! Vault management utilities for Obsidian integration
//! 
//! This module provides shared logic for determining if files are within
//! the Obsidian vault and converting between absolute and relative paths.
//! 
//! Note: This module was created during the obs-to-markdown refactoring to provide
//! unified Rust-based vault logic. Currently, the JavaScript actions in config.yaml
//! implement their own vault detection. This module remains for future use when
//! we might want to move vault logic from JavaScript to Rust.

use std::path::{Path, PathBuf};
use crate::core::Config;
use crate::utils;

/// Result of vault path analysis
#[derive(Debug, Clone)]
pub struct VaultPathInfo {
    /// The absolute path to the file
    pub absolute_path: PathBuf,
    /// Whether the file is within the configured vault
    pub is_in_vault: bool,
    /// Relative path from vault root (only if is_in_vault is true)
    pub vault_relative_path: Option<String>,
    /// The obsidian:// URL for opening in Obsidian (only if is_in_vault is true)
    pub obsidian_url: Option<String>,
}

/// Analyzes a file path and determines its relationship to the Obsidian vault
pub fn analyze_path(path: &Path, config: &Config) -> VaultPathInfo {
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        // Convert relative path to absolute using current directory
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    };
    
    // Get vault configuration
    let vault_path = get_vault_path(config);
    let vault_name = get_vault_name(config);
    
    // Check if the absolute path is within the vault
    let is_in_vault = absolute_path.starts_with(&vault_path);
    
    let (vault_relative_path, obsidian_url) = if is_in_vault {
        // Calculate relative path from vault root
        let relative_path = absolute_path
            .strip_prefix(&vault_path)
            .unwrap()
            .to_string_lossy()
            .to_string();
        
        // Create obsidian:// URL
        let encoded_path = urlencoding::encode(&relative_path);
        let obsidian_url = format!("obsidian://open?vault={}&file={}", vault_name, encoded_path);
        
        (Some(relative_path), Some(obsidian_url))
    } else {
        (None, None)
    };
    
    VaultPathInfo {
        absolute_path,
        is_in_vault,
        vault_relative_path,
        obsidian_url,
    }
}

/// Get the configured vault path, with tilde expansion
pub fn get_vault_path(config: &Config) -> PathBuf {
    let vault_path_str = config.launcher_settings
        .as_ref()
        .and_then(|s| s.obsidian_vault_path.as_ref())
        .map(|p| utils::expand_tilde(p))
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/Documents", home)
        });
    
    PathBuf::from(vault_path_str)
}

/// Get the configured vault name
pub fn get_vault_name(config: &Config) -> String {
    config.launcher_settings
        .as_ref()
        .and_then(|s| s.obsidian_vault_name.as_ref())
        .cloned()
        .unwrap_or_else(|| "kmr".to_string())
}

/// Determines the appropriate action for opening a markdown file
/// 
/// This function implements the unified logic for both markdown and anchor actions:
/// - If the file is within the vault: use Obsidian
/// - If the file is outside the vault: use system default (doc action)
pub fn get_markdown_action_strategy(path: &Path, config: &Config) -> MarkdownActionStrategy {
    let path_info = analyze_path(path, config);
    
    if path_info.is_in_vault {
        MarkdownActionStrategy::Obsidian {
            obsidian_url: path_info.obsidian_url.unwrap(),
            absolute_path: path_info.absolute_path,
            vault_relative_path: path_info.vault_relative_path.unwrap(),
        }
    } else {
        MarkdownActionStrategy::SystemDefault {
            absolute_path: path_info.absolute_path,
        }
    }
}

/// Strategy for opening markdown files
#[derive(Debug, Clone)]
pub enum MarkdownActionStrategy {
    /// Open in Obsidian (file is within vault)
    Obsidian {
        obsidian_url: String,
        absolute_path: PathBuf,
        vault_relative_path: String,
    },
    /// Open with system default application (file is outside vault)
    SystemDefault {
        absolute_path: PathBuf,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{LauncherSettings, PopupSettings, ScannerSettings};
    
    fn create_test_config(vault_path: &str, vault_name: &str) -> Config {
        Config {
            launcher_settings: Some(LauncherSettings {
                obsidian_vault_path: Some(vault_path.to_string()),
                obsidian_vault_name: Some(vault_name.to_string()),
                ..Default::default()
            }),
            popup_settings: PopupSettings {
                max_rows: 10,
                max_columns: 1,
                verbose_logging: None,
                listed_actions: None,
                merge_similar: false,
                word_separators: " ._-".to_string(),
                scan_interval_seconds: None,
                idle_timeout_seconds: None,
                countdown_seconds: Some(5),
                max_window_size: None,
                default_window_size: None,
                max_log_file_size: None,
                run_in_background: None,
                file_roots: None,
                orphans_path: None,
                skip_directory_patterns: None,
            },
            grabber_rules: None,
            keybindings: None,
            templates: None,
            actions: None,
        }
    }
    
    #[test]
    fn test_vault_path_analysis() {
        let config = create_test_config("/Users/test/vault", "test_vault");
        
        // Test file inside vault
        let inside_path = Path::new("/Users/test/vault/notes/file.md");
        let info = analyze_path(inside_path, &config);
        
        assert!(info.is_in_vault);
        assert_eq!(info.vault_relative_path, Some("notes/file.md".to_string()));
        assert!(info.obsidian_url.as_ref().unwrap().contains("notes%2Ffile.md"));
        
        // Test file outside vault
        let outside_path = Path::new("/Users/test/documents/file.md");
        let info = analyze_path(outside_path, &config);
        
        assert!(!info.is_in_vault);
        assert_eq!(info.vault_relative_path, None);
        assert_eq!(info.obsidian_url, None);
    }
    
    #[test]
    fn test_markdown_action_strategy() {
        let config = create_test_config("/Users/test/vault", "test_vault");
        
        // Test vault file strategy
        let vault_file = Path::new("/Users/test/vault/notes/file.md");
        match get_markdown_action_strategy(vault_file, &config) {
            MarkdownActionStrategy::Obsidian { obsidian_url, .. } => {
                assert!(obsidian_url.contains("obsidian://open"));
                assert!(obsidian_url.contains("test_vault"));
            }
            _ => panic!("Expected Obsidian strategy for vault file"),
        }
        
        // Test non-vault file strategy
        let external_file = Path::new("/Users/test/documents/file.md");
        match get_markdown_action_strategy(external_file, &config) {
            MarkdownActionStrategy::SystemDefault { .. } => {
                // Expected
            }
            _ => panic!("Expected SystemDefault strategy for external file"),
        }
    }
}