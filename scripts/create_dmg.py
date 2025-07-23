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
    cargo_path = Path(__file__).parent.parent / "Cargo.toml"
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
    
    # Copy binary
    binary_src = Path(__file__).parent.parent / "target" / "release" / "ha"
    binary_dst = macos_dir / "hookanchor"
    if not binary_src.exists():
        raise FileNotFoundError(f"Binary not found at {binary_src}. Run 'cargo build --release' first.")
    
    shutil.copy2(binary_src, binary_dst)
    os.chmod(binary_dst, 0o755)
    
    # Create Info.plist
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
    icon_src = Path(__file__).parent.parent / "installer" / "resources" / "HookAnchor.icns"
    if icon_src.exists():
        shutil.copy2(icon_src, resources_dir / "HookAnchor.icns")
        # Update plist to include icon
        run_command(['plutil', '-insert', 'CFBundleIconFile', '-string', 'HookAnchor', str(plist_path)])
    
    return app_bundle

def create_dmg_contents(version):
    """Prepare the DMG contents directory"""
    dmg_dir = Path(__file__).parent.parent / "dmg-contents"
    
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
    readme_src = Path(__file__).parent.parent / "installer" / "resources" / "README.txt"
    if readme_src.exists():
        shutil.copy2(readme_src, dmg_dir / "README.txt")
    
    return dmg_dir

def create_dmg(dmg_dir, version):
    """Create the final DMG file"""
    app_name = "HookAnchor"
    dmg_name = f"{app_name}-{version}.dmg"
    volume_name = f"{app_name} {version}"
    output_dir = Path(__file__).parent.parent / "target" / "dist"
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
        os.chdir(Path(__file__).parent.parent)
        run_command(['cargo', 'build', '--release'])
        
        # Create DMG contents
        dmg_dir = create_dmg_contents(version)
        
        # Create DMG
        dmg_path = create_dmg(dmg_dir, version)
        
        # Open the output directory
        run_command(['open', str(dmg_path.parent)])
        
    except Exception as e:
        print_colored(f"Error: {e}", RED)
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())