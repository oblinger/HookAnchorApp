#!/bin/bash

# HookAnchor Distribution Build Script
# Builds a complete distribution package with proper URL handler architecture
# CRITICAL: Ensures no temporary apps remain that could register URLs

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get script location and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DIST_DIR="$PROJECT_ROOT/dist"
TEMP_BUILD_DIR="$PROJECT_ROOT/temp_build"

# Get version from Cargo.toml
VERSION=$(grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')

echo -e "${BLUE}🚀 HookAnchor Distribution Build v$VERSION${NC}"
echo "================================================="
echo ""

# Function to clean up any temporary apps
cleanup_temp_apps() {
    echo -e "${YELLOW}🧹 Cleaning up temporary apps...${NC}"
    
    # Remove any temporary build directories
    if [ -d "$TEMP_BUILD_DIR" ]; then
        rm -rf "$TEMP_BUILD_DIR"
        echo "   Removed temp build directory"
    fi
    
    # CRITICAL: Remove any apps that might register URLs
    # Use rm -rf, NOT trash, to ensure complete removal
    if [ -d "/tmp/HookAnchor.app" ]; then
        rm -rf "/tmp/HookAnchor.app"
        echo "   Removed temporary app from /tmp"
    fi
    
    if [ -d "/tmp/URLHandler.app" ]; then
        rm -rf "/tmp/URLHandler.app"
        echo "   Removed temporary URL handler from /tmp"
    fi
    
    # Check for any stray URL handlers in trash and warn
    if ls ~/.Trash/*URLHandler* 2>/dev/null | grep -q .; then
        echo -e "${RED}   ⚠️  WARNING: Found URL handlers in Trash - these can still be registered!${NC}"
        echo "   Permanently deleting from Trash..."
        rm -rf ~/.Trash/*URLHandler* 2>/dev/null || true
    fi
}

# Ensure cleanup happens even on error
trap cleanup_temp_apps EXIT

# Initial cleanup
cleanup_temp_apps

# 1. Clean and create distribution directory
echo -e "${BLUE}📁 Setting up distribution directory...${NC}"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"
mkdir -p "$TEMP_BUILD_DIR"

# 2. Build Rust components
echo -e "${BLUE}🔨 Building Rust components...${NC}"
cd "$PROJECT_ROOT"

# Check for both architectures
echo "   Checking Rust targets..."
if ! rustup target list --installed | grep -q "x86_64-apple-darwin"; then
    echo "   Installing Intel target..."
    rustup target add x86_64-apple-darwin
fi

if ! rustup target list --installed | grep -q "aarch64-apple-darwin"; then
    echo "   Installing Apple Silicon target..."
    rustup target add aarch64-apple-darwin
fi

# Build for both architectures
echo "   Building for Apple Silicon (ARM64)..."
cargo build --release --target aarch64-apple-darwin

echo "   Building for Intel (x86_64)..."
cargo build --release --target x86_64-apple-darwin

# 3. Build Swift components
echo -e "${BLUE}🔨 Building Swift components...${NC}"

# Build supervisor
echo "   Building Swift supervisor..."
"$PROJECT_ROOT/swift/build_supervisor.sh"

# Build URL handler
echo "   Building URL handler..."
"$PROJECT_ROOT/swift/URLHandler/build_url_handler.sh"

# 4. Create universal binaries
echo -e "${BLUE}🔗 Creating universal binaries...${NC}"
mkdir -p "$TEMP_BUILD_DIR/universal"

# Create universal binary for each component
for BINARY in HookAnchorCommand HookAnchorPopup HookAnchorPopupServer HookAnchorInstaller; do
    echo "   Creating universal binary for $BINARY..."
    lipo -create -output "$TEMP_BUILD_DIR/universal/$BINARY" \
        "target/aarch64-apple-darwin/release/$BINARY" \
        "target/x86_64-apple-darwin/release/$BINARY"
done

# URL launcher might be Swift (single arch) or could be made universal
echo "   Copying url_launcher..."
cp "target/release/url_launcher" "$TEMP_BUILD_DIR/universal/url_launcher"

# Copy supervisor (Swift, single arch for now)
echo "   Copying HookAnchor supervisor..."
cp "target/release/HookAnchorSupervisor" "$TEMP_BUILD_DIR/universal/HookAnchor"

# 5. Create app bundle structure
echo -e "${BLUE}📦 Creating HookAnchor.app bundle...${NC}"
APP_DIR="$TEMP_BUILD_DIR/HookAnchor.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
URL_HANDLER_DIR="$RESOURCES_DIR/URLHandler.app"

mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Copy main binaries (with descriptive process names for Activity Monitor)
echo "   Installing main binaries..."
cp "$TEMP_BUILD_DIR/universal/HookAnchor" "$MACOS_DIR/HookAnchor"
cp "$TEMP_BUILD_DIR/universal/HookAnchorPopupServer" "$MACOS_DIR/HookAnchor Popup Server"
cp "$TEMP_BUILD_DIR/universal/HookAnchorCommand" "$MACOS_DIR/HookAnchor Command"
cp "$TEMP_BUILD_DIR/universal/HookAnchorPopup" "$MACOS_DIR/HookAnchor Popup"
cp "$TEMP_BUILD_DIR/universal/HookAnchorInstaller" "$MACOS_DIR/HookAnchor Installer"

# Create symlinks with original names for compatibility
echo "   Creating compatibility symlinks..."
cd "$MACOS_DIR"
ln -sf "HookAnchor Popup Server" popup_server
ln -sf "HookAnchor Command" ha
ln -sf "HookAnchor Popup" popup
ln -sf "HookAnchor Installer" installer_gui
# Also create snake_case symlinks for any existing references
ln -sf "HookAnchor Popup Server" hookanchor_popup_server
ln -sf "HookAnchor Command" hookanchor_cmd
ln -sf "HookAnchor Popup" hookanchor_popup
ln -sf "HookAnchor Installer" hookanchor_installer
# Create CamelCase symlinks for direct binary access
ln -sf "HookAnchor Popup Server" HookAnchorPopupServer
ln -sf "HookAnchor Command" HookAnchorCommand
ln -sf "HookAnchor Popup" HookAnchorPopup
ln -sf "HookAnchor Installer" HookAnchorInstaller
cd - > /dev/null

# 6. Create embedded URLHandler.app
echo -e "${BLUE}🔗 Creating embedded URLHandler.app...${NC}"
mkdir -p "$URL_HANDLER_DIR/Contents/MacOS"

# Copy url_launcher binary
cp "$TEMP_BUILD_DIR/universal/url_launcher" "$URL_HANDLER_DIR/Contents/MacOS/url_launcher"

# Create URLHandler Info.plist (registers hook:// URLs)
cat > "$URL_HANDLER_DIR/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>url_launcher</string>
    <key>CFBundleIdentifier</key>
    <string>com.hookanchor.urlhandler</string>
    <key>CFBundleName</key>
    <string>HookAnchor URL Handler</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>VERSION_PLACEHOLDER</string>
    <key>LSBackgroundOnly</key>
    <true/>
    <key>LSUIElement</key>
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
EOF

# Replace version placeholder
sed -i '' "s/VERSION_PLACEHOLDER/$VERSION/g" "$URL_HANDLER_DIR/Contents/Info.plist"

# 7. Create main app Info.plist (NO URL registration!)
echo "   Creating main app Info.plist..."
cat > "$CONTENTS_DIR/Info.plist" << EOF
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
    <key>CFBundleDisplayName</key>
    <string>HookAnchor</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>LSUIElement</key>
    <true/>
    <key>LSMultipleInstancesProhibited</key>
    <true/>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
</dict>
</plist>
EOF

# 8. Copy resources
echo -e "${BLUE}📦 Copying resources...${NC}"

# Copy icon if it exists (check multiple locations)
if [ -f "$PROJECT_ROOT/installer/resources/HookAnchor.icns" ]; then
    cp "$PROJECT_ROOT/installer/resources/HookAnchor.icns" "$RESOURCES_DIR/AppIcon.icns"
    echo "   ✓ Copied HookAnchor.icns from installer/resources"
elif [ -f "$PROJECT_ROOT/resources/HookAnchor.icns" ]; then
    cp "$PROJECT_ROOT/resources/HookAnchor.icns" "$RESOURCES_DIR/AppIcon.icns"
    echo "   ✓ Copied HookAnchor.icns from resources"
elif [ -f "$PROJECT_ROOT/resources/icons/icon.icns" ]; then
    cp "$PROJECT_ROOT/resources/icons/icon.icns" "$RESOURCES_DIR/AppIcon.icns"
    echo "   ✓ Copied icon.icns from resources/icons"
else
    echo "   ⚠️  No icon file found"
fi

# Generate default configs from personal configs
echo "   Generating default configs from personal configs..."

# Generate YAML config from master config in repo
if [ -f "$PROJECT_ROOT/config/config.yaml" ]; then
    python3 "$PROJECT_ROOT/dev-scripts/build/generate_default_config.py" "$RESOURCES_DIR/default_config.yaml" "$PROJECT_ROOT/config/config.yaml"
    # Also create dist_config.yaml (same content, different name for installer)
    cp "$RESOURCES_DIR/default_config.yaml" "$RESOURCES_DIR/dist_config.yaml"
else
    echo "   Warning: Personal YAML config not found, creating minimal default"
    cat > "$RESOURCES_DIR/default_config.yaml" << 'EOF'
# HookAnchor Default Configuration
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
fi

# Generate JavaScript config from master config in repo
if [ -f "$PROJECT_ROOT/config/config.js" ]; then
    python3 "$PROJECT_ROOT/dev-scripts/build/generate_default_config_js.py" "$RESOURCES_DIR/default_config.js" "$PROJECT_ROOT/config/config.js"
    # Also create dist_config.js (same content, different name for installer)
    cp "$RESOURCES_DIR/default_config.js" "$RESOURCES_DIR/dist_config.js"
else
    echo "   Warning: Personal JavaScript config not found, creating minimal default"
    cat > "$RESOURCES_DIR/default_config.js" << 'EOF'
// HookAnchor Default JavaScript Configuration
// Copy this to ~/.config/hookanchor/config.js and customize as needed

module.exports = {
  // Example JavaScript function for 1Password integration
  action_1pass: function(ctx) {
    const searchTerm = ctx.arg || '';
    try {
      // Use 1Password Quick Access (Cmd+Shift+Space)
      shell("osascript -e 'tell application \"System Events\" to keystroke \" \" using {shift down, command down}'");
      shell_sync("/bin/sleep 0.5");
      shell_sync(`osascript -e 'tell application "System Events" to keystroke "${searchTerm}"'`);
      shell_sync("/bin/sleep 0.3");
      shell("osascript -e 'tell application \"System Events\" to key code 36'");
    } catch (e) {
      log("Error executing 1Password action: " + e.message);
    }
  }
};
EOF
fi

# Create dist_uninstall.sh from the source
echo "   Creating distribution uninstall script..."
cp "$PROJECT_ROOT/dev-scripts/uninstall.sh" "$RESOURCES_DIR/dist_uninstall.sh"

# Create dist_hook_anchor_zshrc (shell integration file)
echo "   Creating distribution zshrc..."
cp "$PROJECT_ROOT/config/hook_anchor_zshrc" "$RESOURCES_DIR/dist_hook_anchor_zshrc"

# Generate HTML documentation
echo "   Generating HTML documentation..."
cd "$PROJECT_ROOT"
just docs
# Copy HTML files to Resources (docs are in Obsidian vault, not code repo)
DOCS_DIR="/Users/oblinger/ob/kmr/prj/binproj/Hook Anchor/docs/User Docs"
if [ -f "$DOCS_DIR/README.html" ]; then
    cp "$DOCS_DIR"/*.html "$RESOURCES_DIR/"
    echo "   ✓ Copied HTML documentation to Resources"
else
    echo "   ⚠️  Warning: No HTML docs found at $DOCS_DIR"
fi

# Copy LICENSE to app bundle
echo "   Copying LICENSE to app bundle..."
if [ -f "$DOCS_DIR/LICENSE.txt" ]; then
    cp "$DOCS_DIR/LICENSE.txt" "$RESOURCES_DIR/LICENSE.txt"
    echo "   ✓ Copied LICENSE.txt to app Resources"
else
    echo "   ❌ ERROR: LICENSE.txt not found in $DOCS_DIR"
    exit 1
fi

# Copy other distribution default files
echo "   Copying distribution default files..."
if [ -d "$PROJECT_ROOT/dist/HookAnchor.app/Contents/Resources" ]; then
    cp "$PROJECT_ROOT/dist/HookAnchor.app/Contents/Resources"/dist_* "$RESOURCES_DIR/" 2>/dev/null || true
    echo "   Copied distribution files to Resources"
else
    echo "   Warning: No distribution files found"
fi

# Copy Karabiner complex modification resource file
echo "   Copying Karabiner resource file..."
if [ -f "$PROJECT_ROOT/Resources/hookanchor.json" ]; then
    cp "$PROJECT_ROOT/Resources/hookanchor.json" "$RESOURCES_DIR/"
    echo "   ✓ Copied hookanchor.json to Resources"
else
    echo "   ⚠️  Warning: hookanchor.json not found in Resources/"
fi

# Create install marker file (triggers installer on first run)
echo "   Creating install marker file..."
touch "$RESOURCES_DIR/install_pending"
echo "   ✓ Created install_pending marker file"

# Create Uninstaller app
echo "   Creating Uninstaller.app..."
"$PROJECT_ROOT/dev-scripts/build/generate_uninstaller_app.sh"

# 9. Sign the app (optional, requires developer certificate)
if security find-identity -v -p codesigning | grep -q "Developer ID Application"; then
    echo -e "${BLUE}🔏 Signing app bundle...${NC}"
    codesign --deep --force --verify --verbose --sign "Developer ID Application" "$APP_DIR"
else
    echo -e "${YELLOW}⚠️  No signing certificate found - app will require manual approval on first run${NC}"
fi

# 10. Create distribution package
echo -e "${BLUE}📦 Creating distribution package...${NC}"

# Copy app to dist directory
cp -R "$APP_DIR" "$DIST_DIR/"

# Create README
cat > "$DIST_DIR/README.md" << 'EOF'
# HookAnchor - Keyboard Launcher for Knowledge Management

## Quick Installation

1. **Drag to Applications**: Drag `HookAnchor.app` to `/Applications`

2. **First Launch - Bypass macOS Security**:
   - Double-click HookAnchor.app (will be blocked)
   - Open **System Settings** → **Privacy & Security**
   - Scroll to **Security** section
   - Click **"Open Anyway"** next to HookAnchor message
   - Try opening HookAnchor again, click **"Open"** in dialog

   _(This is only needed once for unsigned apps)_

3. **Grant Permissions**: Allow accessibility when prompted

4. **Start Using**: Press **Option+Space** (⌥Space) to open HookAnchor from anywhere!

## Features

- **Built-in Hotkey**: Option+Space works out of the box (configurable)
- **Fuses Local + Online Knowledge**: Access files, URLs, and commands
- **Auto-Discovery**: Scans your system for executables and markdown files
- **Smart Search**: Fuzzy matching for instant access

## Configuration

Config files: `~/.config/hookanchor/`
- `config.yaml` - Change hotkey, customize behavior
- `commands.txt` - Your command list (auto-managed)

## Uninstall

1. Delete `/Applications/HookAnchor.app`
2. Remove `~/.config/hookanchor/` directory
3. Remove Karabiner-Elements rule if configured

## Support

Report issues at: https://github.com/oblinger/hookanchor
EOF

# 11. Create DMG
echo -e "${BLUE}💿 Creating DMG...${NC}"
DMG_NAME="HookAnchor-$VERSION.dmg"

# Create a temporary DMG
hdiutil create -size 100m -fs HFS+ -volname "HookAnchor" "$TEMP_BUILD_DIR/temp.dmg" >/dev/null 2>&1
hdiutil attach "$TEMP_BUILD_DIR/temp.dmg" -mountpoint "$TEMP_BUILD_DIR/dmg_mount" >/dev/null 2>&1

# Copy contents
cp -R "$DIST_DIR/HookAnchor.app" "$TEMP_BUILD_DIR/dmg_mount/"
cp "$DIST_DIR/README.md" "$TEMP_BUILD_DIR/dmg_mount/"

# Copy LICENSE to DMG root
DOCS_DIR="/Users/oblinger/ob/kmr/prj/binproj/Hook Anchor/docs/User Docs"
if [ -f "$DOCS_DIR/LICENSE.txt" ]; then
    cp "$DOCS_DIR/LICENSE.txt" "$TEMP_BUILD_DIR/dmg_mount/LICENSE.txt"
    echo "   ✓ Added LICENSE.txt to DMG"
else
    echo "   ❌ ERROR: LICENSE.txt not found in $DOCS_DIR"
    exit 1
fi

ln -s /Applications "$TEMP_BUILD_DIR/dmg_mount/Applications"

# Create a simple background image or use existing
# ... (optional DMG styling)

# Unmount and compress
hdiutil detach "$TEMP_BUILD_DIR/dmg_mount" >/dev/null 2>&1
hdiutil convert "$TEMP_BUILD_DIR/temp.dmg" -format UDZO -o "$DIST_DIR/$DMG_NAME" >/dev/null 2>&1

# 12. Create ZIP as alternative
echo -e "${BLUE}🗜️  Creating ZIP archive...${NC}"
cd "$DIST_DIR"
zip -rq "HookAnchor-$VERSION.zip" HookAnchor.app README.md

# 13. Final cleanup
echo -e "${BLUE}🧹 Final cleanup...${NC}"
rm -rf "$TEMP_BUILD_DIR"

# 14. Verify no URL handlers leaked
echo -e "${BLUE}🔍 Verifying clean build...${NC}"
if ls /tmp/*URLHandler* /tmp/*HookAnchor* 2>/dev/null | grep -q .; then
    echo -e "${RED}❌ ERROR: Found leaked app bundles in /tmp!${NC}"
    rm -rf /tmp/*URLHandler* /tmp/*HookAnchor* 2>/dev/null
    exit 1
fi

# 14. Archive to versions folder
echo -e "${BLUE}📚 Archiving to versions folder...${NC}"
VERSIONS_DIR="$PROJECT_ROOT/versions/$VERSION"
mkdir -p "$VERSIONS_DIR"

# Copy distribution files to versions folder
cp "$DIST_DIR/$DMG_NAME" "$VERSIONS_DIR/"
cp "$DIST_DIR/HookAnchor-$VERSION.zip" "$VERSIONS_DIR/"
cp "$DIST_DIR/README.md" "$VERSIONS_DIR/"

# Create a build info file
cat > "$VERSIONS_DIR/BUILD_INFO.txt" << EOF
HookAnchor Version $VERSION
Built: $(date)
Git Commit: $(git rev-parse HEAD 2>/dev/null || echo "unknown")
Branch: $(git branch --show-current 2>/dev/null || echo "unknown")
Builder: $(whoami)@$(hostname)
Architecture: Universal (x86_64 + arm64)
EOF

echo "   Archived to: $VERSIONS_DIR"

# Success!
echo ""
echo -e "${GREEN}✅ Distribution build complete!${NC}"
echo "================================================="
echo -e "📦 Package: ${BLUE}$DIST_DIR/$DMG_NAME${NC}"
echo -e "📦 Archive: ${BLUE}$DIST_DIR/HookAnchor-$VERSION.zip${NC}"
echo -e "📚 Versions: ${BLUE}$VERSIONS_DIR${NC}"
echo ""
echo "The distribution package includes:"
echo "  • Universal binaries (Intel + Apple Silicon)"
echo "  • Embedded URL handler for hook:// URLs"
echo "  • No URL registration in main app"
echo "  • Ready for distribution"
echo ""
echo -e "${GREEN}Ready to distribute!${NC}"

# Clean up dist directory on successful completion
echo ""
echo -e "${BLUE}🧹 Cleaning up build workspace...${NC}"
rm -rf "$DIST_DIR"
echo "   Removed temporary dist/ directory"
echo "   All artifacts preserved in: $VERSIONS_DIR"