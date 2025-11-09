#!/usr/bin/env python3
"""
Generate default config.js from user's personal config.js
Replaces personal paths and values with generic defaults.
"""

import sys
import os
import re
from pathlib import Path

def get_version_from_cargo():
    """Extract version from Cargo.toml"""
    cargo_path = Path(__file__).parent.parent.parent / "Cargo.toml"
    if cargo_path.exists():
        with open(cargo_path, 'r') as f:
            for line in f:
                if line.startswith('version = '):
                    # Extract version like '0.19.8' from 'version = "0.19.8"'
                    version = line.split('=')[1].strip().strip('"')
                    return version
    return "0.0.0"

def sanitize_js_config(personal_js_path, output_path):
    """Generate default config.js from personal config by sanitizing personal values."""

    # Get version from Cargo.toml
    version = get_version_from_cargo()

    with open(personal_js_path, 'r') as f:
        js_content = f.read()
    
    # Replace personal paths with generic placeholders
    replacements = [
        # Specific personal paths (longest first)
        (r'/Users/oblinger/ob/kmr/SYS/Closet/Orphans', '~/Documents/Orphans'),
        (r'/Users/oblinger/ob/kmr/Work/SV/ww', '~/Work'),
        (r'/Users/oblinger/ob/kmr/Notes', '~/Notes'),
        (r'/Users/oblinger/ob/kmr', '~/Documents/Notes'),
        (r'/Users/oblinger/ob/prj', '~/Projects'),  
        (r'/Users/oblinger/ob/proj', '~/Projects'),
        (r'/Users/oblinger/\.config', '~/.config'),
        (r'/Users/oblinger/Desktop', '~/Desktop'),
        (r'/Users/oblinger/Downloads', '~/Downloads'),
        
        # Shorter patterns
        (r'~/ob/kmr', '~/Documents/Notes'),
        (r'~/ob/prj', '~/Projects'),
        (r'~/ob/proj', '~/Projects'),
        
        # Personal username
        (r'\boblinger\b', 'username'),
        (r'\bOblinger\b', 'Username'),
        
        # Personal vault name
        (r'"kmr"', '"MyVault"'),
        (r"'kmr'", "'MyVault'"),
    ]
    
    for pattern, replacement in replacements:
        js_content = re.sub(pattern, replacement, js_content)
    
    # Strip any existing version comment from the top
    lines = js_content.split('\n')
    if lines and lines[0].strip().startswith('// Version:'):
        # Remove old version line
        js_content = '\n'.join(lines[1:])

    # Add version comment at the very top
    js_content = f"// Version: {version}\n{js_content}"
    
    with open(output_path, 'w') as f:
        f.write(js_content)
    
    print(f"Generated default config.js: {output_path}")

def main():
    # Get paths
    if len(sys.argv) > 2:
        # Config file path provided as second argument
        personal_js_path = sys.argv[2]
        output_path = sys.argv[1]
    elif len(sys.argv) > 1:
        # Only output path provided, use default config location
        personal_js_path = os.path.expanduser("~/.config/hookanchor/config.js")
        output_path = sys.argv[1]
    else:
        # No arguments, use defaults
        personal_js_path = os.path.expanduser("~/.config/hookanchor/config.js")
        output_path = "default_config.js"
    
    # Check if personal config exists
    if not os.path.exists(personal_js_path):
        print(f"Warning: Personal JS config not found at {personal_js_path}")
        print("Skipping JavaScript config generation")
        return

    print(f"Loading personal JS config from {personal_js_path}")
    sanitize_js_config(personal_js_path, output_path)
    
    print("✓ Default JavaScript config generated successfully")
    print("\nKey replacements:")
    print("  Personal paths → Generic paths (~/, ~/Documents, etc.)")
    print("  'oblinger' → 'username'")
    print("  'kmr' vault → 'MyVault'")

if __name__ == "__main__":
    main()