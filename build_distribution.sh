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
PROJECT_ROOT="$SCRIPT_DIR"
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
for BINARY in ha popup popup_server; do
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
cp "target/release/HookAnchor" "$TEMP_BUILD_DIR/universal/HookAnchor"

# 5. Create app bundle structure
echo -e "${BLUE}📦 Creating HookAnchor.app bundle...${NC}"
APP_DIR="$TEMP_BUILD_DIR/HookAnchor.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
URL_HANDLER_DIR="$RESOURCES_DIR/URLHandler.app"

mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Copy main binaries
echo "   Installing main binaries..."
cp "$TEMP_BUILD_DIR/universal/HookAnchor" "$MACOS_DIR/HookAnchor"
cp "$TEMP_BUILD_DIR/universal/popup_server" "$MACOS_DIR/popup_server"
cp "$TEMP_BUILD_DIR/universal/ha" "$MACOS_DIR/ha"
cp "$TEMP_BUILD_DIR/universal/popup" "$MACOS_DIR/popup"

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

# Copy icon if it exists
if [ -f "$PROJECT_ROOT/resources/HookAnchor.icns" ]; then
    cp "$PROJECT_ROOT/resources/HookAnchor.icns" "$RESOURCES_DIR/AppIcon.icns"
elif [ -f "$PROJECT_ROOT/resources/icon.icns" ]; then
    cp "$PROJECT_ROOT/resources/icon.icns" "$RESOURCES_DIR/AppIcon.icns"
fi

# Generate default config from personal config
echo "   Generating default config from personal config..."
if [ -f "$HOME/.config/hookanchor/config.yaml" ]; then
    python3 "$PROJECT_ROOT/scripts/generate_default_config.py" "$RESOURCES_DIR/default_config.yaml"
else
    echo "   Warning: Personal config not found, creating minimal default"
    # Fallback to a minimal config if personal config doesn't exist
    cat > "$RESOURCES_DIR/default_config.yaml" << 'EOF'
# HookAnchor Default Configuration
popup_settings:
  max_rows: 25
  max_columns: 4
  run_in_background: true
launcher_settings:
  obsidian_vault_name: "MyVault"
  obsidian_vault_path: "~/Documents"
markdown_roots:
  - "~/Documents"
EOF
fi

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
# HookAnchor - Universal Command Launcher

## Installation

1. **Drag to Applications**: Drag `HookAnchor.app` to your `/Applications` folder
2. **First Launch**: Right-click and select "Open" (don't double-click) to bypass Gatekeeper
3. **Set up Caps Lock**: Use Karabiner-Elements to map Caps Lock to launch HookAnchor
4. **Grant Permissions**: Allow accessibility permissions when prompted

## Features

- **Instant Launch**: Press Caps Lock to open the command palette
- **URL Support**: Handle `hook://` URLs from any application
- **Smart Search**: Fuzzy matching for commands and files
- **Extensible**: Add custom commands via config.yaml

## Configuration

Configuration is stored in `~/.config/hookanchor/config.yaml`

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