#!/bin/bash

# Create a production macOS app bundle with both GUI and safe URL handling
# This bundle can handle both normal launches (GUI) and URL schemes (no GUI)

set -e

# Configuration
APP_NAME="HookAnchor"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${1:-dist}"  # Allow custom output directory
APP_DIR="${OUTPUT_DIR}/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_colored() {
    echo -e "${2}${1}${NC}"
}

# Ensure we're in the project root
cd "$PROJECT_ROOT"

# Check for required binaries
DISPATCHER_BINARY="./target/release/ha"
POPUP_BINARY="./target/release/popup"

if [[ ! -f "$DISPATCHER_BINARY" ]]; then
    print_colored "Error: Dispatcher binary not found at $DISPATCHER_BINARY" $RED
    print_colored "Please build first: cargo build --release" $RED
    exit 1
fi

if [[ ! -f "$POPUP_BINARY" ]]; then
    print_colored "Error: Popup binary not found at $POPUP_BINARY" $RED
    print_colored "Please build first: cargo build --release" $RED
    exit 1
fi

print_colored "Creating production app bundle at ${APP_DIR}..." $YELLOW

# Create directory structure
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"
mkdir -p "$OUTPUT_DIR"

# Remove existing app if it exists
if [[ -d "$APP_DIR" ]]; then
    print_colored "Removing existing app bundle..." $YELLOW
    rm -rf "$APP_DIR"
    mkdir -p "$MACOS_DIR"
    mkdir -p "$RESOURCES_DIR"
fi

# Copy binaries
cp "$DISPATCHER_BINARY" "$MACOS_DIR/ha"
cp "$POPUP_BINARY" "$MACOS_DIR/popup"

# Create an AppleScript wrapper that handles Apple Events
cat > "$MACOS_DIR/${APP_NAME}" << 'EOF'
#!/usr/bin/osascript

-- HookAnchor AppleScript Wrapper
-- Routes normal launches and URL schemes to the dispatcher

on run
    -- Normal app launch (no URL) - directly launch popup
    set script_dir to (do shell script "dirname " & quoted form of POSIX path of (path to me))
    do shell script "exec '" & script_dir & "/popup'"
end run

on open location url_string
    -- URL scheme handler - pass to dispatcher with --hook flag
    set script_dir to (do shell script "dirname " & quoted form of POSIX path of (path to me))
    set quoted_url to quoted form of url_string
    do shell script "'" & script_dir & "/ha' --hook " & quoted_url
end open location
EOF

chmod +x "$MACOS_DIR/${APP_NAME}"
chmod +x "$MACOS_DIR/ha"
chmod +x "$MACOS_DIR/popup"

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Copy icon if exists
ICON_SRC="${PROJECT_ROOT}/installer/resources/HookAnchor.icns"
if [[ -f "$ICON_SRC" ]]; then
    cp "$ICON_SRC" "$RESOURCES_DIR/HookAnchor.icns"
fi

# Create Info.plist with URL handling enabled
cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.hookanchor.app</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleIconFile</key>
    <string>HookAnchor</string>
    <key>LSUIElement</key>
    <true/>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLName</key>
            <string>HookAnchor URL Handler</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>hook</string>
            </array>
        </dict>
    </array>
</dict>
</plist>
EOF

print_colored "✓ Production app bundle created successfully!" $GREEN
print_colored "Location: ${APP_DIR}" $GREEN
print_colored "" $NC
print_colored "App bundle contains:" $YELLOW
print_colored "  - GUI app (ha) for normal launches" $NC
print_colored "  - URL handler (hook_url_handler) for hook:// URLs" $NC
print_colored "  - Smart launcher script that routes appropriately" $NC
print_colored "" $NC
print_colored "This bundle is ready for DMG packaging and distribution!" $GREEN