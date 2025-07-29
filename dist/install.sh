#!/bin/bash

echo "Installing HookAnchor..."

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo "Please do not run this installer as root/sudo"
   exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_PATH="$SCRIPT_DIR/HookAnchor.app"

# Check if app exists
if [ ! -d "$APP_PATH" ]; then
    echo "Error: HookAnchor.app not found in the same directory as this script"
    exit 1
fi

# Install to Applications
echo "Installing to /Applications..."
if [ -d "/Applications/HookAnchor.app" ]; then
    echo "Backing up existing installation..."
    mv "/Applications/HookAnchor.app" "/Applications/HookAnchor.app.backup.$(date +%Y%m%d_%H%M%S)"
fi

cp -R "$APP_PATH" "/Applications/"

# Create config directory
echo "Setting up configuration directory..."
mkdir -p "$HOME/.config/hookanchor"

# Check if config exists, if not create default
if [ ! -f "$HOME/.config/hookanchor/config.yaml" ]; then
    echo "Creating default configuration..."
    # Copy the comprehensive default config from the project
    if [ -f "$PROJECT_ROOT/resources/common/default_config.yaml" ]; then
        cp "$PROJECT_ROOT/resources/common/default_config.yaml" "$HOME/.config/hookanchor/config.yaml"
        echo "✅ Installed comprehensive default configuration"
    else
        echo "⚠️  Default config template not found, creating basic config..."
        cat > "$HOME/.config/hookanchor/config.yaml" << 'CONFIG_EOF'
# Basic HookAnchor Configuration
popup_settings:
  max_rows: 15
  max_columns: 3
  debug_log: "~/.config/hookanchor/anchor.log"
  server_log: "~/.config/hookanchor/server.log"
  debug_scanner: false
  merge_similar: true
  word_separators: " ._-"
  scan_interval_seconds: 86400000
  idle_timeout_seconds: 60
  listed_actions: "alias,anchor,app,url,folder,cmd,chrome,safari,brave,firefox,work,notion,obs,obs_url,1pass,rewrite,doc,contact,slack,text,shutdown"

markdown_roots:
 - "~/Documents"
 - "~/Notes"

launcher_settings:
  application_folder: "/Applications/HookAnchor.app"
  default_browser: "Google Chrome"
  work_browser: "Google Chrome Beta"
  timeout_ms: 5000
  obsidian_app_name: "Obsidian"
  obsidian_vault_name: "main"
  obsidian_vault_path: "~/Documents"

scanner_settings:
  orphans_path: "~/Documents/HookAnchor/Orphans"

keybindings:
  exit_app: "Escape"
  navigate_down: "ArrowDown"
  navigate_up: "ArrowUp"
  navigate_left: "ArrowLeft"
  navigate_right: "ArrowRight"
  execute_command: "Enter"
  force_rescan: "Backtick"
  show_folder: "Slash"
  start_grabber: "Plus"
  open_editor: "Equals"
  add_alias: ">"
  edit_active_command: "Semicolon"
  delete_command: "Delete"
  cancel_editor: "Escape"

functions:
  action_app: {fn: launch_app, name: "{{arg}}"}
  action_url: {fn: open_url, url: "{{arg}}"}
  action_folder: {fn: open_folder, path: "{{arg}}"}
  action_cmd: {fn: shell, command: "{{arg}}"}
  action_doc: {fn: open_with, app: "", arg: "{{arg}}"}

CONFIG_EOF
    fi
fi

echo ""
echo "✅ HookAnchor installation complete!"
echo ""
echo "Next steps:"
echo "1. Launch HookAnchor from Applications folder"
echo "2. Grant accessibility permissions if prompted"
echo "3. Customize your configuration in ~/.config/hookanchor/config.yaml"
echo ""
echo "For help and documentation, see README.md"
