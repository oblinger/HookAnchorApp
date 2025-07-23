#!/usr/bin/env python3
"""
Activate Tmux - Standalone Tmux Session Manager

This program extracts the tmux-related functionality from the anchor tool.
It will activate a tmux session for the current directory if a .tmuxp.yaml file exists.

Usage:
  activate_tmux.py [directory_path]

If no directory_path is provided, uses the current working directory.
"""

import subprocess
import os
import sys
import time
from pathlib import Path
from typing import Optional


def get_tmux_sessions() -> list[str]:
    """Get list of active tmux session names."""
    try:
        result = subprocess.run(['/opt/homebrew/bin/tmux', 'ls'], capture_output=True, text=True)
        session_names = []
        if result.returncode == 0:
            for line in result.stdout.strip().split('\n'):
                if line and ':' in line:
                    session_name = line.split(':')[0].strip()
                    session_names.append(session_name)
        return session_names
    except FileNotFoundError:
        print("Error: tmux not found at /opt/homebrew/bin/tmux")
        return []


def attach_to_tmux_session(session_name: str) -> None:
    """Attach to tmux session, trying switch-client first, then attach-session."""
    try:
        # Check if we're inside tmux by looking for TMUX environment variable
        if os.environ.get('TMUX'):
            print(f"Inside tmux, switching to session '{session_name}'...")
            # We're inside tmux - use switch-client
            switch_result = subprocess.run(['/opt/homebrew/bin/tmux', 'switch-client', '-t', session_name])
            if switch_result.returncode == 0:
                print(f"Successfully switched to session '{session_name}'")
            else:
                print(f"Failed to switch to session '{session_name}'")
        else:
            print(f"Outside tmux, attaching to session '{session_name}'...")
            # We're outside tmux - use attach-session
            subprocess.run(['/opt/homebrew/bin/tmux', 'attach-session', '-t', session_name])
    except FileNotFoundError:
        print("Error: tmux not found at /opt/homebrew/bin/tmux")
        sys.exit(1)


def activate_tmux(anchor_path: Path) -> bool:
    """Activate tmux session if local .tmuxp.yaml exists"""
    folder_name = anchor_path.name
    tmuxp_profile = anchor_path / ".tmuxp.yaml"
    
    print(f"Checking for tmux profile at: {tmuxp_profile}")
    
    if not tmuxp_profile.exists():
        print(f"No .tmuxp.yaml file found in {anchor_path}")
        return False

    print(f"Found .tmuxp.yaml file, managing session '{folder_name}'")
    
    try:
        session_names = get_tmux_sessions()
        print(f"Active tmux sessions: {session_names}")
        
        if folder_name in session_names:
            print(f"Session '{folder_name}' already exists, attaching to it")
        else:
            print(f"Creating new tmux session '{folder_name}' from profile")
            # Create session in detached mode using tmuxp
            result = subprocess.run(['/opt/homebrew/bin/tmuxp', 'load', str(tmuxp_profile), '-d'], 
                                  capture_output=True, text=True)
            if result.returncode != 0:
                print(f"Error creating tmux session: {result.stderr}")
                return False
            
            # Give tmux a moment to fully create the session
            time.sleep(0.2)
            print(f"Session '{folder_name}' created successfully")

        # Activate iTerm2 before attaching to session
        print("Activating iTerm2...")
        try:
            subprocess.run(['/usr/bin/osascript', '-e', 'tell application "iTerm2" to activate'], 
                         capture_output=True)
            time.sleep(0.1)  # Brief delay to ensure activation
        except Exception as e:
            print(f"Warning: Could not activate iTerm2: {e}")
        
        # Attach to the session
        print(f"Attaching to tmux session '{folder_name}'...")
        attach_to_tmux_session(folder_name)
        return True

    except FileNotFoundError as e:
        print(f"Error: Required tool not found - {e}")
        print("Make sure tmux and tmuxp are installed and available at /opt/homebrew/bin/")
        return False
    except subprocess.CalledProcessError as e:
        print(f"Error managing tmux session: {e}")
        return False


def main() -> None:
    """Main function to activate tmux for the specified or current directory."""
    # Determine target directory
    if len(sys.argv) > 1:
        arg_path = Path(sys.argv[1]).resolve()
        
        # If the argument is a file, use its parent directory
        if arg_path.is_file():
            target_path = arg_path.parent
            print(f"File provided, using parent directory: {target_path}")
        else:
            target_path = arg_path
    else:
        target_path = Path.cwd()
    
    print(f"Activating tmux for directory: {target_path}")
    
    if not target_path.exists():
        print(f"Error: Directory does not exist: {target_path}")
        sys.exit(1)
    
    if not target_path.is_dir():
        print(f"Error: Path is not a directory: {target_path}")
        sys.exit(1)
    
    # Change to the target directory
    os.chdir(str(target_path))
    print(f"Changed to directory: {target_path}")
    
    # Try to activate tmux (this is the only function of this script now)
    success = activate_tmux(target_path)
    
    if success:
        print("✓ Tmux session activated successfully!")
    else:
        print("✗ No tmux session was activated")
        # Don't exit with error - let JavaScript handle fallbacks


if __name__ == "__main__":
    main()