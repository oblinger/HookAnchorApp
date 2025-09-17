#!/usr/bin/env python3
"""
HookAnchor DMG Creation Script
Creates a DMG installer for HookAnchor
"""

import os
import sys
import subprocess
import shutil
import tempfile
import json
from pathlib import Path

# ANSI color codes
RED = '\033[0;31m'
GREEN = '\033[0;32m'
YELLOW = '\033[1;33m'
NC = '\033[0m'  # No Color

def print_colored(message, color=NC):
    """Print message with color"""
    print(f"{color}{message}{NC}")

def run_command(cmd, check=True, capture_output=False):
    """Run a shell command and return result"""
    print(f"Running: {' '.join(cmd) if isinstance(cmd, list) else cmd}")
    if capture_output:
        result = subprocess.run(cmd, shell=isinstance(cmd, str), capture_output=True, text=True)
        if check and result.returncode != 0:
            raise RuntimeError(f"Command failed: {result.stderr}")
        return result.stdout.strip()
    else:
        result = subprocess.run(cmd, shell=isinstance(cmd, str))
        if check and result.returncode != 0:
            raise RuntimeError(f"Command failed with code {result.returncode}")
        return result

def get_cargo_version():
    """Extract version from Cargo.toml"""
    cargo_path = Path(__file__).parent / "Cargo.toml"
    with open(cargo_path, 'r') as f:
        for line in f:
            if line.startswith('version'):
                # Extract version between quotes
                return line.split('"')[1]
    return "0.1.0"  # default

def create_app_bundle(dmg_dir, version):
    """Create the macOS app bundle structure"""
    app_name = "HookAnchor"
    app_bundle = dmg_dir / f"{app_name}.app"
    contents = app_bundle / "Contents"
    macos_dir = contents / "MacOS"
    resources_dir = contents / "Resources"
    
    # Create directories
    macos_dir.mkdir(parents=True, exist_ok=True)
    resources_dir.mkdir(parents=True, exist_ok=True)
    
    # Copy dispatcher binary
    dispatcher_src = Path(__file__).parent / "target" / "release" / "ha"
    dispatcher_dst = macos_dir / "ha"
    if not dispatcher_src.exists():
        raise FileNotFoundError(f"Dispatcher binary not found at {dispatcher_src}. Run 'cargo build --release' first.")
    
    shutil.copy2(dispatcher_src, dispatcher_dst)
    os.chmod(dispatcher_dst, 0o755)
    
    # Copy popup binary
    popup_src = Path(__file__).parent / "target" / "release" / "popup"
    popup_dst = macos_dir / "popup"
    if not popup_src.exists():
        raise FileNotFoundError(f"Popup binary not found at {popup_src}. Run 'cargo build --release' first.")
    
    shutil.copy2(popup_src, popup_dst)
    os.chmod(popup_dst, 0o755)
    
    # Copy popup_server binary
    popup_server_src = Path(__file__).parent / "target" / "release" / "popup_server"
    popup_server_dst = macos_dir / "popup_server"
    if not popup_server_src.exists():
        raise FileNotFoundError(f"Popup server binary not found at {popup_server_src}. Run 'cargo build --release' first.")
    
    shutil.copy2(popup_server_src, popup_server_dst)
    os.chmod(popup_server_dst, 0o755)
    
    # Create AppleScript wrapper as main executable (no URL handling)
    wrapper_dst = macos_dir / "hookanchor"
    with open(wrapper_dst, 'w') as f:
        f.write('''#!/usr/bin/osascript

-- HookAnchor AppleScript Wrapper
-- Only handles normal app launches, URL handling is delegated to URLHandler.app

on run
    -- Normal app launch (no URL) - directly launch popup
    set script_dir to (do shell script "dirname " & quoted form of POSIX path of (path to me))
    do shell script "exec '" & script_dir & "/popup'"
end run
''')
    os.chmod(wrapper_dst, 0o755)
    
    # Create URLHandler.app sub-app for URL handling
    print("Creating URLHandler.app sub-app...")
    urlhandler_app = macos_dir / "URLHandler.app"
    urlhandler_contents = urlhandler_app / "Contents"
    urlhandler_macos = urlhandler_contents / "MacOS"
    urlhandler_macos.mkdir(parents=True, exist_ok=True)
    
    # Create URLHandler AppleScript
    urlhandler_script = urlhandler_macos / "URLHandler"
    with open(urlhandler_script, 'w') as f:
        f.write('''#!/usr/bin/osascript

on idle
    return 3600
end idle

on open location url_string
    set ha_path to "/Applications/HookAnchor.app/Contents/MacOS/ha"
    set quoted_url to quoted form of url_string
    do shell script ha_path & " --hook " & quoted_url
    quit
end open location

on run
    -- Do nothing when launched directly
end run

on quit
    continue quit
end quit
''')
    os.chmod(urlhandler_script, 0o755)
    
    # Create URLHandler Info.plist
    urlhandler_info = {
        "CFBundleExecutable": "URLHandler",
        "CFBundleIdentifier": "com.oblinger.HookAnchor.URLHandler",
        "CFBundleName": "HookAnchor URL Handler",
        "CFBundlePackageType": "APPL",
        "CFBundleShortVersionString": version,
        "CFBundleVersion": "1",
        "LSBackgroundOnly": True,
        "LSUIElement": True,
        "CFBundleURLTypes": [
            {
                "CFBundleURLName": "HookAnchor URL",
                "CFBundleURLSchemes": ["hook"]
            }
        ]
    }
    
    # Write URLHandler Info.plist
    urlhandler_plist_path = urlhandler_contents / "Info.plist"
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump(urlhandler_info, f)
        temp_json = f.name
    
    run_command(['plutil', '-convert', 'xml1', '-o', str(urlhandler_plist_path), temp_json])
    os.unlink(temp_json)
    
    # Create Info.plist (without URL handling - delegated to URLHandler.app)
    info_plist = {
        "CFBundleExecutable": "hookanchor",
        "CFBundleIdentifier": "com.hookanchor.app",
        "CFBundleName": app_name,
        "CFBundleDisplayName": app_name,
        "CFBundleVersion": version,
        "CFBundleShortVersionString": version,
        "CFBundlePackageType": "APPL",
        "LSMinimumSystemVersion": "11.0",
        "LSUIElement": True,  # Run as menu bar app
        "NSHighResolutionCapable": True
        # Note: URL handling removed - now handled by URLHandler.app sub-app
    }
    
    # Write Info.plist
    plist_path = contents / "Info.plist"
    # Use plutil to create proper plist format
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump(info_plist, f)
        temp_json = f.name
    
    run_command(['plutil', '-convert', 'xml1', '-o', str(plist_path), temp_json])
    os.unlink(temp_json)
    
    # Copy icon if exists
    icon_src = Path(__file__).parent / "installer" / "resources" / "HookAnchor.icns"
    if icon_src.exists():
        shutil.copy2(icon_src, resources_dir / "HookAnchor.icns")
        # Update plist to include icon
        run_command(['plutil', '-insert', 'CFBundleIconFile', '-string', 'HookAnchor', str(plist_path)])
    
    return app_bundle

def create_dmg_contents(version):
    """Prepare the DMG contents directory"""
    dmg_dir = Path(__file__).parent / "dmg-contents"
    
    # Clean and recreate directory
    if dmg_dir.exists():
        shutil.rmtree(dmg_dir)
    dmg_dir.mkdir(parents=True)
    
    # Create app bundle
    app_bundle = create_app_bundle(dmg_dir, version)
    
    # Create Applications symlink
    apps_link = dmg_dir / "Applications"
    apps_link.symlink_to("/Applications")
    
    # Copy README
    readme_src = Path(__file__).parent / "installer" / "resources" / "README.txt"
    if readme_src.exists():
        shutil.copy2(readme_src, dmg_dir / "README.txt")
    
    return dmg_dir

def create_dmg(dmg_dir, version):
    """Create the final DMG file"""
    app_name = "HookAnchor"
    dmg_name = f"{app_name}-{version}.dmg"
    volume_name = f"{app_name} {version}"
    output_dir = Path(__file__).parent / "target" / "dist"
    output_dir.mkdir(exist_ok=True)
    
    dmg_path = output_dir / dmg_name
    
    # Remove existing DMG
    if dmg_path.exists():
        dmg_path.unlink()
    
    # Create temporary DMG
    temp_dmg = output_dir / f"{dmg_name}.temp.dmg"
    print_colored("Creating DMG...", YELLOW)
    
    run_command([
        'hdiutil', 'create',
        '-volname', volume_name,
        '-srcfolder', str(dmg_dir),
        '-ov',
        '-format', 'UDRW',
        str(temp_dmg)
    ])
    
    # Mount the temporary DMG
    print_colored("Mounting temporary DMG...", YELLOW)
    mount_output = run_command([
        'hdiutil', 'attach',
        '-readwrite',
        '-noverify',
        str(temp_dmg)
    ], capture_output=True)
    
    # Extract device from output
    device = None
    for line in mount_output.splitlines():
        if line.startswith('/dev/'):
            device = line.split()[0]
            break
    
    if not device:
        raise RuntimeError("Failed to mount DMG")
    
    volume_path = f"/Volumes/{volume_name}"
    
    # Wait for volume to mount
    import time
    time.sleep(2)
    
    # Set DMG window properties using AppleScript
    print_colored("Setting DMG window properties...", YELLOW)
    applescript = f"""
    tell application "Finder"
        tell disk "{volume_name}"
            open
            set current view of container window to icon view
            set toolbar visible of container window to false
            set statusbar visible of container window to false
            set the bounds of container window to {{400, 100, 900, 400}}
            set viewOptions to the icon view options of container window
            set arrangement of viewOptions to not arranged
            set icon size of viewOptions to 72
            set position of item "{app_name}.app" of container window to {{150, 150}}
            set position of item "Applications" of container window to {{350, 150}}
            close
            open
            update without registering applications
            delay 2
        end tell
    end tell
    """
    
    run_command(['osascript', '-e', applescript])
    
    # Unmount and convert to compressed DMG
    print_colored("Finalizing DMG...", YELLOW)
    run_command(['hdiutil', 'detach', device])
    
    run_command([
        'hdiutil', 'convert',
        str(temp_dmg),
        '-format', 'UDZO',
        '-imagekey', 'zlib-level=9',
        '-o', str(dmg_path)
    ])
    
    # Remove temporary DMG
    temp_dmg.unlink()
    
    # Calculate file size
    size = dmg_path.stat().st_size / (1024 * 1024)  # Convert to MB
    print_colored(f"✓ DMG created successfully: {dmg_path}", GREEN)
    print_colored(f"  Size: {size:.1f} MB", GREEN)
    
    return dmg_path

def main():
    """Main entry point"""
    try:
        print_colored("Building HookAnchor DMG Installer...", GREEN)
        
        # Get version
        version = get_cargo_version()
        print(f"Version: {version}")
        
        # Build release if needed
        print_colored("Building release binary...", YELLOW)
        os.chdir(Path(__file__).parent)
        run_command(['cargo', 'build', '--release'])
        
        # Create DMG contents
        dmg_dir = create_dmg_contents(version)
        
        # Create DMG
        dmg_path = create_dmg(dmg_dir, version)
        
        # Clean up DMG contents directory to prevent URL registration conflicts
        print_colored("Cleaning up temporary DMG contents...", YELLOW)
        if dmg_dir.exists():
            shutil.rmtree(dmg_dir)
            print_colored("✓ DMG contents directory cleaned up", GREEN)
        
        # Open the output directory
        run_command(['open', str(dmg_path.parent)])
        
    except Exception as e:
        print_colored(f"Error: {e}", RED)
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())