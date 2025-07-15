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
    cat > "$HOME/.config/hookanchor/config.yaml" << 'CONFIG_EOF'
# HookAnchor Configuration
# This file is automatically created on first install

popup_settings:
  max_rows: 15
  max_columns: 3
  use_new_launcher: true
  debug_log: "~/.anchor.log"
  verbose_logging: false
  debug_scanner: false
  merge_similar: true
  word_separators: "_-. "
  scan_root: "~/Documents"
  scan_interval_seconds: 3600
  listed_actions: "alias,anchor,app,url,folder,cmd,chrome,safari,brave,firefox,work,notion,obs,obs_url,1pass,rewrite,doc,contact,slack,shutdown"

markdown_roots:
  - "~/Documents"

launcher_settings:
  application_folder: "/Applications/HookAnchor.app"
  default_browser: "Google Chrome"
  work_browser: "Google Chrome Beta"
  timeout_ms: 5000
  obsidian_app_name: "Obsidian"
  obsidian_vault_name: "main"
  obsidian_vault_path: "~/Documents"

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
  link_to_clipboard: "ctrl+c"

functions:
  action_app: {fn: launch_app, name: "{{arg}}"}
  action_url: {fn: open_url, url: "{{arg}}"}
  action_folder: {fn: open_folder, path: "{{arg}}"}
  action_cmd: {fn: shell, command: "{{arg}}"}
  action_doc: {fn: open_with, app: "", arg: "{{arg}}"}
  action_chrome: {fn: open_with, app: "Google Chrome", arg: "{{arg}}"}
  action_safari: {fn: open_with, app: "Safari", arg: "{{arg}}"}
  action_brave: {fn: open_with, app: "Brave Browser", arg: "{{arg}}"}
  action_firefox: {fn: open_with, app: "Firefox", arg: "{{arg}}"}
  action_work: {fn: open_with, app: "Google Chrome Beta", arg: "{{arg}}"}
  action_notion: {fn: open_with, app: "Notion", arg: "{{arg}}"}
  action_obs_url: {fn: open_with, app: "Obsidian", arg: "{{arg}}"}
  action_obs: {fn: shell_sync, command: "open -a Obsidian && sleep 0.1 && open 'obsidian://open?vault=main&file={{arg}}'"}
  action_contact: {fn: open_with, app: "Contacts", arg: "addressbook://{{arg}}"}
  
  action_link_to_clipboard: |
    const cmdName = "{{arg}}";
    const wordSeparators = "_-. ";
    let firstSeparator = null;
    for (let i = 0; i < wordSeparators.length; i++) {
      const char = wordSeparators[i];
      if (char !== ' ') {
        firstSeparator = char;
        break;
      }
    }
    if (!firstSeparator) {
      firstSeparator = '_';
    }
    const urlName = cmdName.replace(/ /g, firstSeparator);
    const markdownLink = `[${cmdName}](hook://${urlName}) `;
    shell_sync(`echo '${markdownLink}' | pbcopy`);
    log(`LINK_TO_CLIPBOARD: Copied to clipboard: ${markdownLink}`);

CONFIG_EOF
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
