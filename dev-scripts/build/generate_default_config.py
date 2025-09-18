#!/usr/bin/env python3
"""
Generate default config.yaml from user's personal config.
Replaces personal paths with generic defaults while preserving structure and examples.
"""

import yaml
import sys
import os
from pathlib import Path

def load_personal_config(config_path):
    """Load the personal config file."""
    with open(config_path, 'r') as f:
        return f.read()

def transform_config(config_text):
    """Transform personal config to default config."""
    
    # Replace personal paths with generic defaults
    replacements = [
        # Specific paths first (longer matches before shorter)
        ('/Users/oblinger/ob/kmr/Work/SV/ww', '~/Work'),
        ('/Users/oblinger/ob/kmr/Notes', '~/Notes'),
        ('~/ob/kmr/Notes', '~/Notes'),
        ('~/ob/kmr/MY/Meta', '~/Personal'),
        
        # General vault paths
        ('~/ob/kmr', '~/Documents/Notes'),
        ('/Users/oblinger/ob/kmr', '~/Documents/Notes'),
        
        # Work and project paths
        ('~/ob/prj', '~/Projects'),
        ('/Users/oblinger/ob/prj', '~/Projects'),
        ('~/ob/proj', '~/Projects'),
        ('/Users/oblinger/ob/proj', '~/Projects'),
        
        # Vault name
        ('obsidian_vault_name:    "kmr"', 'obsidian_vault_name:    "MyVault"'),
        ('obsidian_vault_name: "kmr"', 'obsidian_vault_name: "MyVault"'),
    ]
    
    result = config_text
    for old, new in replacements:
        result = result.replace(old, new)
    
    # Add header comment for new users
    header = """# HookAnchor Default Configuration
# 
# This config has a number of example templates.  Please customize as needed.
#
# UNIFIED ACTIONS SYSTEM:
# All behaviors (templates, keyboard bindings, command actions) are now defined
# in the 'actions:' section. Each action has a 'type' that determines its behavior,
# and can optionally be bound to a keyboard key via the 'key' field.

"""
    
    # Ensure the result starts with the header
    if not result.startswith("# HookAnchor Default Configuration"):
        result = header + result
    
    return result

def main():
    # Get paths
    if len(sys.argv) > 2:
        # Config file path provided as second argument
        personal_config_path = sys.argv[2]
        output_path = sys.argv[1]
    elif len(sys.argv) > 1:
        # Only output path provided, use default config location
        personal_config_path = os.path.expanduser("~/.config/hookanchor/config.yaml")
        output_path = sys.argv[1]
    else:
        # No arguments, use defaults
        personal_config_path = os.path.expanduser("~/.config/hookanchor/config.yaml")
        output_path = "default_config.yaml"
    
    # Check if personal config exists
    if not os.path.exists(personal_config_path):
        print(f"Error: Personal config not found at {personal_config_path}")
        sys.exit(1)
    
    # Load and transform
    print(f"Loading personal config from {personal_config_path}")
    config_text = load_personal_config(personal_config_path)
    
    print("Transforming to default config...")
    default_config = transform_config(config_text)
    
    # Write output
    print(f"Writing default config to {output_path}")
    with open(output_path, 'w') as f:
        f.write(default_config)
    
    print("✓ Default config generated successfully")
    
    # Show summary of replacements
    print("\nKey paths replaced:")
    print("  ~/ob/kmr → ~/Documents/Notes")
    print("  ~/ob/prj → ~/Projects")
    print("  Personal orphans → ~/Documents/Orphans")
    print("  Vault 'kmr' → 'MyVault'")
    
    # Check if we have actions section
    if 'actions:' in default_config:
        print("\n✓ Config uses new unified actions system")
    elif 'templates:' in default_config:
        print("\n⚠ Config still uses legacy templates - will be auto-migrated to actions")

if __name__ == "__main__":
    main()