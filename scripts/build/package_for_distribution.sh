#!/bin/bash

# HookAnchor Distribution Packaging Script
# This script builds and packages HookAnchor for distribution

set -e  # Exit on any error

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Project root is two directories up from scripts/build/
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
APP_NAME="HookAnchor"
APP_BUNDLE="/Applications/$APP_NAME.app"
DIST_DIR="$PROJECT_ROOT/dist"

# Get version from Cargo.toml
VERSION=$(grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | sed 's/version = "\(.*\)"/\1/')

echo "🔨 Building HookAnchor Distribution Package v$VERSION"
echo "================================================="

# Clean and create distribution directory
echo "📁 Setting up distribution directory..."
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Generate default config from personal config
echo "📝 Generating default configuration..."
if python3 scripts/generate_default_config.py; then
    echo "✅ Default config generated successfully"
else
    echo "⚠️  Config generation failed, using existing default"
fi

# Build universal binary for both architectures
echo "🔧 Building universal binary..."
cd "$PROJECT_ROOT"

# Check if Intel target is installed
if ! rustup target list --installed | grep -q "x86_64-apple-darwin"; then
    echo "📥 Installing Intel target..."
    rustup target add x86_64-apple-darwin
fi

# Build for Apple Silicon (ARM64)
echo "🏗️  Building for Apple Silicon (ARM64)..."
cargo build --release --target aarch64-apple-darwin

# Build for Intel (x86_64)
echo "🏗️  Building for Intel (x86_64)..."
cargo build --release --target x86_64-apple-darwin

# Create universal binary
echo "🔗 Creating universal binary..."
lipo -create -output target/release/ha-universal \
    target/aarch64-apple-darwin/release/ha \
    target/x86_64-apple-darwin/release/ha

# Verify universal binary
echo "✅ Verifying universal binary..."
file target/release/ha-universal

# Update the app bundle with universal binary
echo "📦 Updating app bundle..."
cp target/release/ha-universal "$APP_BUNDLE/Contents/MacOS/hookanchor"
cp target/release/ha-universal "$APP_BUNDLE/Contents/MacOS/popup"

# Update icon if source exists
if [ -f "$PROJECT_ROOT/resources/icon.icns" ]; then
    echo "🎨 Updating app bundle icon..."
    cp "$PROJECT_ROOT/resources/icon.icns" "$APP_BUNDLE/Contents/Resources/applet.icns"
    cp "$PROJECT_ROOT/resources/icon.icns" "$APP_BUNDLE/Contents/Resources/popup.icns"
else
    echo "ℹ️  No source icon found - using existing icon"
fi

# Update version in Info.plist
echo "📝 Updating version information..."
/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" "$APP_BUNDLE/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $VERSION" "$APP_BUNDLE/Contents/Info.plist" 2>/dev/null || \
/usr/libexec/PlistBuddy -c "Add :CFBundleVersion string $VERSION" "$APP_BUNDLE/Contents/Info.plist"

# Create a clean copy of the app bundle
echo "📋 Creating distribution copy..."
cp -R "$APP_BUNDLE" "$DIST_DIR/"

# Create a README for users
echo "📖 Creating user documentation..."
cat > "$DIST_DIR/README.md" << 'EOF'
# HookAnchor - Universal Command Launcher

## Installation

1. **Move the app**: Drag `HookAnchor.app` to your `/Applications` folder
2. **First run security**: If you get "This app won't run on this Mac":
   - **Right-click** the app in Applications and select "Open" (don't double-click)
   - Or go to System Preferences → Security & Privacy → General → Click "Open Anyway"
3. **Grant permissions**: You may need to grant accessibility permissions in System Preferences > Security & Privacy > Privacy > Accessibility

## System Requirements

- macOS 11.0 (Big Sur) or later
- Works on both Intel and Apple Silicon Macs

## Configuration

HookAnchor creates its configuration in `~/.config/hookanchor/config.yaml`

## Usage

- **Launch**: Press your configured hotkey (default varies by system)
- **Search**: Type to find commands, files, and applications
- **Execute**: Press Enter to execute the selected command
- **Navigation**: Use arrow keys to navigate results
- **Special Keys**:
  - `Ctrl+C`: Copy markdown link to clipboard
  - `=`: Open command editor
  - `+`: Start grabber mode
  - `~`: Force rescan

## URL Scheme

HookAnchor supports `hook://` URLs for deep linking:
- `hook://command_name` - Execute a specific command

## Support

For issues and documentation, visit the project repository.
EOF

# Create installation script
echo "⚙️  Creating installation script..."
cat > "$DIST_DIR/install.sh" << 'EOF'
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
  open_editor: "Equals"
  edit_active_command: "Semicolon"
  delete_command: "Delete"

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
EOF

chmod +x "$DIST_DIR/install.sh"

# Create a proper DMG (optional)
echo "💿 Creating DMG file..."
DMG_NAME="HookAnchor-$VERSION.dmg"
hdiutil create -size 50m -fs HFS+ -volname "HookAnchor $VERSION" "$DIST_DIR/temp.dmg"
hdiutil attach "$DIST_DIR/temp.dmg" -mountpoint "$DIST_DIR/dmg_mount"

# Copy contents to DMG
cp -R "$DIST_DIR/HookAnchor.app" "$DIST_DIR/dmg_mount/"
cp "$DIST_DIR/README.md" "$DIST_DIR/dmg_mount/"
cp "$DIST_DIR/install.sh" "$DIST_DIR/dmg_mount/"

# Create Applications symlink
ln -s /Applications "$DIST_DIR/dmg_mount/Applications"

# Unmount and compress
hdiutil detach "$DIST_DIR/dmg_mount"
hdiutil convert "$DIST_DIR/temp.dmg" -format UDZO -o "$DIST_DIR/$DMG_NAME"
rm "$DIST_DIR/temp.dmg"

# Create ZIP archive as alternative
echo "🗜️  Creating ZIP archive..."
cd "$DIST_DIR"
zip -r "HookAnchor-$VERSION.zip" HookAnchor.app README.md install.sh

# Create tarball for advanced users
echo "📦 Creating tarball..."
tar -czf "HookAnchor-$VERSION.tar.gz" HookAnchor.app README.md install.sh

echo ""
echo "🎉 Distribution package created successfully!"
echo "================================================="
echo "📁 Location: $DIST_DIR"
echo "📦 Files created:"
echo "   - HookAnchor-$VERSION.dmg (recommended for most users)"
echo "   - HookAnchor-$VERSION.zip (alternative format)"
echo "   - HookAnchor-$VERSION.tar.gz (for advanced users)"
echo "   - HookAnchor.app (standalone app bundle)"
echo "   - install.sh (installation script)"
echo "   - README.md (user documentation)"
echo ""
echo "✅ Ready for distribution!"