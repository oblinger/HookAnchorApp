#!/usr/bin/env python3
"""
Test script for HookAnchor installer components
"""

import os
import sys
import subprocess
import shutil
from pathlib import Path

def print_status(message, status="INFO"):
    """Print status message with color"""
    colors = {
        "INFO": "\033[0;34m",  # Blue
        "SUCCESS": "\033[0;32m",  # Green
        "WARNING": "\033[1;33m",  # Yellow
        "ERROR": "\033[0;31m",  # Red
    }
    reset = "\033[0m"
    print(f"{colors.get(status, '')}{status}: {message}{reset}")

def check_karabiner():
    """Check if Karabiner-Elements is installed"""
    karabiner_cli = "/Library/Application Support/org.pqrs/Karabiner-Elements/bin/karabiner_cli"
    if Path(karabiner_cli).exists():
        print_status("Karabiner-Elements is installed", "SUCCESS")
        return True
    else:
        print_status("Karabiner-Elements is NOT installed", "WARNING")
        return False

def test_setup_assistant():
    """Test the setup assistant by temporarily moving config"""
    config_dir = Path.home() / ".config" / "hookanchor"
    backup_dir = Path.home() / ".config" / "hookanchor_backup_test"
    
    # Backup existing config if it exists
    moved_config = False
    if config_dir.exists():
        print_status("Backing up existing config for test", "INFO")
        shutil.move(str(config_dir), str(backup_dir))
        moved_config = True
    
    try:
        # Build and run HookAnchor (this should trigger setup assistant)
        print_status("Building HookAnchor...", "INFO")
        os.chdir(Path(__file__).parent.parent.parent)
        subprocess.run(['cargo', 'build', '--release'], check=True)
        
        print_status("Testing setup assistant (should run since no config exists)...", "INFO")
        result = subprocess.run(['./target/release/ha'], capture_output=True, text=True)
        
        if result.returncode == 0:
            print_status("Setup assistant completed successfully", "SUCCESS")
        else:
            print_status(f"Setup assistant failed: {result.stderr}", "ERROR")
            
        # Check if config was created
        if config_dir.exists():
            print_status("Configuration directory was created", "SUCCESS")
            if (config_dir / "config.yaml").exists():
                print_status("Default config.yaml was created", "SUCCESS")
            else:
                print_status("config.yaml was NOT created", "ERROR")
        else:
            print_status("Configuration directory was NOT created", "ERROR")
            
    finally:
        # Restore original config
        if moved_config:
            if config_dir.exists():
                shutil.rmtree(config_dir)
            shutil.move(str(backup_dir), str(config_dir))
            print_status("Restored original config", "INFO")

def test_karabiner_integration():
    """Test Karabiner integration setup"""
    karabiner_dir = Path.home() / ".config" / "karabiner" / "assets" / "complex_modifications"
    hookanchor_mod = karabiner_dir / "hookanchor.json"
    
    if hookanchor_mod.exists():
        print_status("HookAnchor Karabiner modification exists", "SUCCESS")
        
        # Check if it's valid JSON
        try:
            import json
            with open(hookanchor_mod, 'r') as f:
                data = json.load(f)
            
            if 'title' in data and data['title'] == 'HookAnchor':
                print_status("Karabiner modification has correct structure", "SUCCESS")
            else:
                print_status("Karabiner modification has incorrect structure", "ERROR")
                
        except json.JSONDecodeError:
            print_status("Karabiner modification is invalid JSON", "ERROR")
    else:
        print_status("HookAnchor Karabiner modification does NOT exist", "WARNING")

def test_dmg_creation():
    """Test DMG creation process"""
    dmg_script = Path(__file__).parent / "create_dmg.py"
    
    if not dmg_script.exists():
        print_status("DMG creation script not found", "ERROR")
        return
    
    print_status("Testing DMG creation (this may take a while)...", "INFO")
    
    try:
        result = subprocess.run([sys.executable, str(dmg_script)], 
                              capture_output=True, text=True, timeout=300)
        
        if result.returncode == 0:
            print_status("DMG creation completed successfully", "SUCCESS")
            
            # Check if DMG was created
            dist_dir = Path(__file__).parent.parent / "dist"
            dmg_files = list(dist_dir.glob("HookAnchor-*.dmg"))
            
            if dmg_files:
                dmg_file = dmg_files[0]
                size_mb = dmg_file.stat().st_size / (1024 * 1024)
                print_status(f"DMG file created: {dmg_file.name} ({size_mb:.1f} MB)", "SUCCESS")
            else:
                print_status("DMG file was not found in dist directory", "ERROR")
        else:
            print_status(f"DMG creation failed: {result.stderr}", "ERROR")
            
    except subprocess.TimeoutExpired:
        print_status("DMG creation timed out", "ERROR")
    except Exception as e:
        print_status(f"DMG creation error: {e}", "ERROR")

def main():
    """Run all installer tests"""
    print_status("Starting HookAnchor Installer Tests", "INFO")
    print("=" * 50)
    
    # Test 1: Check Karabiner installation
    print("\n1. Checking Karabiner-Elements...")
    karabiner_installed = check_karabiner()
    
    # Test 2: Test setup assistant
    print("\n2. Testing Setup Assistant...")
    if "--skip-setup" not in sys.argv:
        test_setup_assistant()
    else:
        print_status("Setup assistant test skipped (--skip-setup)", "INFO")
    
    # Test 3: Test Karabiner integration
    print("\n3. Testing Karabiner Integration...")
    test_karabiner_integration()
    
    # Test 4: Test DMG creation
    print("\n4. Testing DMG Creation...")
    if "--skip-dmg" not in sys.argv:
        test_dmg_creation()
    else:
        print_status("DMG creation test skipped (--skip-dmg)", "INFO")
    
    print("\n" + "=" * 50)
    print_status("Installer tests completed", "INFO")
    
    if not karabiner_installed:
        print_status("Recommendation: Install Karabiner-Elements for full functionality", "WARNING")

if __name__ == "__main__":
    main()