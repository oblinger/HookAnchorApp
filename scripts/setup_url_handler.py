#!/usr/bin/env python3

"""
Setup script to register hook:// URL handler for hookanchor popup on macOS
"""

import os
import subprocess
import json

def setup_url_handler():
    # Get the absolute path to the cmd binary (no GUI)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    cmd_path = os.path.join(script_dir, "target", "release", "cmd")
    
    if not os.path.exists(cmd_path):
        print(f"Error: cmd binary not found at {cmd_path}")
        print("Please build the cmd binary first with: cargo build --release --bin cmd")
        return False
    
    # Create a simple app bundle directory structure
    app_bundle_path = os.path.expanduser("~/Applications/AnchorSelector.app")
    contents_path = os.path.join(app_bundle_path, "Contents")
    macos_path = os.path.join(contents_path, "MacOS")
    
    # Create directories
    os.makedirs(macos_path, exist_ok=True)
    
    # Copy cmd binary to app bundle
    import shutil
    target_binary = os.path.join(macos_path, "AnchorSelector")
    shutil.copy2(cmd_path, target_binary)
    os.chmod(target_binary, 0o755)
    
    # Create Info.plist
    info_plist_content = f"""<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>AnchorSelector</string>
    <key>CFBundleIdentifier</key>
    <string>com.hookanchor.app</string>
    <key>CFBundleName</key>
    <string>AnchorSelector</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLName</key>
            <string>Hook URL</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>hook</string>
            </array>
            <key>CFBundleURLIconFile</key>
            <string></string>
        </dict>
    </array>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
</dict>
</plist>"""
    
    info_plist_path = os.path.join(contents_path, "Info.plist")
    with open(info_plist_path, 'w') as f:
        f.write(info_plist_content)
    
    print(f"Created app bundle at: {app_bundle_path}")
    
    # Register the URL scheme using LSRegister
    try:
        subprocess.run([
            "/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister",
            "-f", app_bundle_path
        ], check=True)
        print("Successfully registered hook:// URL handler!")
        print("")
        print("You can now use URLs like:")
        print("  hook://spot")
        print("  hook://test%20command")
        print("")
        print("To test in Terminal: open 'hook://spot'")
        
        return True
    except subprocess.CalledProcessError as e:
        print(f"Error registering URL handler: {e}")
        return False

if __name__ == "__main__":
    setup_url_handler()