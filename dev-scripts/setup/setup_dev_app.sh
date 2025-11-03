#!/bin/bash

# HookAnchor Development App Bundle Setup
# Creates /Applications/HookAnchor.app with symlinks to development binaries
# This is the primary setup script for the app bundle

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Change to script directory first, then find project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/../.."
PROJECT_DIR="$(pwd)"
RELEASE_DIR="$PROJECT_DIR/target/release"
APP_DIR="/Applications/HookAnchor.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
CONFIG_DIR="$HOME/.config/hookanchor"

echo -e "${YELLOW}📦 Setting up HookAnchor.app in /Applications...${NC}"
echo "Project dir: $PROJECT_DIR"
echo ""

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 1. Check prerequisites
echo "📋 Checking prerequisites..."

if ! command_exists cargo; then
    echo -e "${RED}❌ Rust/Cargo not installed${NC}"
    echo "   Please install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

if ! command_exists swift; then
    echo -e "${RED}❌ Swift not installed${NC}"
    echo "   Please install Xcode command line tools: xcode-select --install"
    exit 1
fi

echo -e "${GREEN}✅ Prerequisites satisfied${NC}"
echo ""

# 2. Build the project if binaries don't exist
echo "🔨 Checking binaries..."

if [ ! -f "$RELEASE_DIR/HookAnchorCommand" ] || [ ! -f "$RELEASE_DIR/HookAnchorPopupServer" ] || [ ! -f "$RELEASE_DIR/HookAnchorPopup" ] || [ ! -f "$RELEASE_DIR/HookAnchorSupervisor" ]; then
    echo "   Building Rust components..."
    cd "$PROJECT_DIR"
    cargo build --release

    echo "   Building Swift supervisor..."
    "$PROJECT_DIR/swift/build_supervisor.sh"

    echo "   Building URL handler..."
    "$PROJECT_DIR/swift/URLHandler/build_url_handler.sh"
else
    echo "   Binaries already exist"
    echo "   (Run 'cargo build --release' to rebuild)"
fi

echo -e "${GREEN}✅ Binaries ready${NC}"
echo ""

# 3. Create config directory and default config if needed
echo "📁 Setting up configuration..."

if [ ! -d "$CONFIG_DIR" ]; then
    mkdir -p "$CONFIG_DIR"
    echo "   Created config directory: $CONFIG_DIR"
fi

# Ensure config.yaml exists with minimal defaults
if [ ! -f "$CONFIG_DIR/config.yaml" ]; then
    echo "   Creating default config.yaml..."
    cat > "$CONFIG_DIR/config.yaml" << 'EOF'
# HookAnchor Configuration
keybindings:
  exit_app:             "Escape"
  navigate_down:        "ArrowDown"
  navigate_up:          "ArrowUp"
  navigate_left:        "ArrowLeft"
  navigate_right:       "ArrowRight"
  execute_command:      "Enter"
  force_rebuild:        "`"
  show_folder:          "/"
  open_editor:          "="
  edit_active_command:  ";"
  show_keys:            "?"

popup_settings:
  max_rows: 25
  max_columns: 4
  run_in_background: true

launcher_settings:
  obsidian_vault_name: "MyVault"
  obsidian_vault_path: "~/Documents/MyVault"

file_roots:
  - "~/Documents"
EOF
    echo "   Created default config"
else
    echo "   Config already exists"
fi

echo -e "${GREEN}✅ Configuration ready${NC}"
echo ""

# 4. Create/Update Application Bundle
echo "📦 Creating HookAnchor.app..."

# Check if we need sudo
if [ -w "/Applications" ]; then
    SUDO=""
else
    SUDO="sudo"
    echo "   Need sudo access to create app in /Applications"
fi

# Remove old app if it exists and recreate
if [ -d "$APP_DIR" ]; then
    echo "   Removing existing app bundle..."
    $SUDO rm -rf "$APP_DIR"
fi

echo "   Creating app bundle structure..."
$SUDO mkdir -p "$MACOS_DIR"
$SUDO mkdir -p "$RESOURCES_DIR"

# Create symlinks to binaries (NEVER COPY!)
echo "   Creating symlinks to binaries..."
$SUDO ln -sf "$RELEASE_DIR/HookAnchorSupervisor" "$MACOS_DIR/HookAnchor"
$SUDO ln -sf "$RELEASE_DIR/HookAnchorPopupServer" "$MACOS_DIR/popup_server"
$SUDO ln -sf "$RELEASE_DIR/HookAnchorCommand" "$MACOS_DIR/ha"
$SUDO ln -sf "$RELEASE_DIR/HookAnchorPopup" "$MACOS_DIR/popup"

# Create Info.plist for main app with URL handling
echo "   Creating Info.plist with URL registration..."

# Copy icon if available
if [ -f "$PROJECT_DIR/resources/HookAnchor.icns" ]; then
    $SUDO cp "$PROJECT_DIR/resources/HookAnchor.icns" "$RESOURCES_DIR/AppIcon.icns"
elif [ -f "$PROJECT_DIR/resources/icon.icns" ]; then
    $SUDO cp "$PROJECT_DIR/resources/icon.icns" "$RESOURCES_DIR/AppIcon.icns"
fi

# Create symlinks to HTML documentation (in Obsidian vault)
DOCS_SOURCE_DIR="/Users/oblinger/ob/kmr/prj/binproj/Hook Anchor/docs/User Docs"
if [ -d "$DOCS_SOURCE_DIR" ]; then
    for html_file in "$DOCS_SOURCE_DIR"/*.html; do
        if [ -f "$html_file" ]; then
            filename=$(basename "$html_file")
            $SUDO ln -sf "$html_file" "$RESOURCES_DIR/$filename"
            echo "   Linked: $filename"
        fi
    done
    echo "   ✓ Documentation symlinks created"
else
    echo "   ⚠️  Warning: Docs directory not found at $DOCS_SOURCE_DIR"
fi

$SUDO tee "$CONTENTS_DIR/Info.plist" > /dev/null << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>HookAnchor</string>
    <key>CFBundleIdentifier</key>
    <string>com.hookanchor.app</string>
    <key>CFBundleName</key>
    <string>HookAnchor</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>LSUIElement</key>
    <true/>
    <key>LSMultipleInstancesProhibited</key>
    <true/>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLName</key>
            <string>HookAnchor URL</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>hook</string>
            </array>
        </dict>
    </array>
</dict>
</plist>
PLIST

# Register the app for hook:// URLs
echo "   Registering HookAnchor.app for hook:// URLs..."
/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister -f "$APP_DIR" 2>/dev/null || true

echo -e "${GREEN}✅ App bundle configured with URL handling${NC}"
echo ""

# 6. Create convenience symlink in ~/bin
echo "🔗 Setting up command line access..."

if [ ! -d "$HOME/bin" ]; then
    mkdir -p "$HOME/bin"
    echo "   Created ~/bin directory"
fi

if [ ! -L "$HOME/bin/ha" ] || [ "$(readlink "$HOME/bin/ha")" != "$RELEASE_DIR/HookAnchorCommand" ]; then
    ln -sf "$RELEASE_DIR/HookAnchorCommand" "$HOME/bin/ha"
    echo "   Created symlink: ~/bin/ha -> $RELEASE_DIR/HookAnchorCommand"
else
    echo "   Symlink ~/bin/ha already correct"
fi

# Check if ~/bin is in PATH
if [[ ":$PATH:" != *":$HOME/bin:"* ]]; then
    echo -e "${YELLOW}   ⚠️  ~/bin is not in your PATH${NC}"
    echo "   Add this to your shell profile (.zshrc or .bash_profile):"
    echo "   export PATH=\"\$HOME/bin:\$PATH\""
fi

echo -e "${GREEN}✅ Command line access configured${NC}"
echo ""

# 7. Clean up any stale sockets
echo "🧹 Cleaning up..."

if [ -S "/tmp/hookanchor_popup.sock" ]; then
    rm -f "/tmp/hookanchor_popup.sock"
    echo "   Removed stale popup socket"
fi

if [ -S "$HOME/.config/hookanchor/command_server.sock" ]; then
    rm -f "$HOME/.config/hookanchor/command_server.sock"
    echo "   Removed stale command server socket"
fi

echo -e "${GREEN}✅ Cleanup complete${NC}"
echo ""

# Done!
echo "=================================="
echo -e "${GREEN}✅ Setup Complete!${NC}"
echo ""
echo "HookAnchor.app installed at: /Applications/HookAnchor.app"
echo "Command line tool: ~/bin/ha"
echo ""
echo "Next steps:"
echo "1. Configure Karabiner-Elements to trigger HookAnchor.app with caps lock"
echo "2. Test by pressing caps lock key"
echo "3. Run 'ha --help' for command line options"
echo "4. Test URL handling with: open 'hook://test'"
echo ""
echo "To test the popup:"
echo "  ~/bin/ha -m test"