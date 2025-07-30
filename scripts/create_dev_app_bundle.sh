#!/bin/bash

# ⚠️ CRITICAL WARNING: READ docs/URL_HANDLING.md BEFORE MODIFYING URL HANDLING ⚠️
# This script creates a development .app bundle that uses the URL handler binary
# for safe URL processing without GUI popups.

set -e

# Configuration
APP_NAME="HookAnchor-Dev"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="${PROJECT_ROOT}/target/dist"
APP_DIR="${DIST_DIR}/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"

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
HA_BINARY="./target/release/ha"
URL_HANDLER_BINARY="./target/release/hook_url_handler"

if [[ ! -f "$HA_BINARY" ]]; then
    print_colored "Error: Main binary not found at $HA_BINARY" $RED
    print_colored "Please build first: cargo build --release" $RED
    exit 1
fi

if [[ ! -f "$URL_HANDLER_BINARY" ]]; then
    print_colored "Error: URL handler binary not found at $URL_HANDLER_BINARY" $RED
    print_colored "Please build first: cargo build --release" $RED
    exit 1
fi

print_colored "Creating development app bundle at ${APP_DIR}..." $YELLOW

# Create directory structure
mkdir -p "$MACOS_DIR"
mkdir -p "$DIST_DIR"

# Remove existing app if it exists
if [[ -d "$APP_DIR" ]]; then
    print_colored "Removing existing app bundle..." $YELLOW
    rm -rf "$APP_DIR"
    mkdir -p "$MACOS_DIR"
fi

# Copy the URL handler binary 
cp "$URL_HANDLER_BINARY" "$MACOS_DIR/hook_url_handler"

# Copy the main binary for reference
cp "$HA_BINARY" "$MACOS_DIR/hookanchor"

# Create an AppleScript-based launcher that handles URL events
cat > "$MACOS_DIR/${APP_NAME}" << 'EOF'
#!/usr/bin/osascript

on run
    -- Normal app launch (no URL) - launch GUI
    set script_dir to (do shell script "dirname " & quoted form of POSIX path of (path to me))
    do shell script quoted form of (script_dir & "/hookanchor")
end run

on open location url_string
    -- URL scheme handler - called when hook:// URLs are opened
    set script_dir to (do shell script "dirname " & quoted form of POSIX path of (path to me))
    set quoted_url to quoted form of url_string
    do shell script quoted form of (script_dir & "/hook_url_handler") & " " & quoted_url
end open location
EOF

chmod +x "$MACOS_DIR/${APP_NAME}"

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Create Info.plist with URL handling enabled
cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.hookanchor.dev</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>HookAnchor (Dev)</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>LSUIElement</key>
    <true/>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLName</key>
            <string>HookAnchor URL Handler (Dev)</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>hook</string>
            </array>
        </dict>
    </array>
</dict>
</plist>
EOF

print_colored "✓ Development app bundle created successfully!" $GREEN
print_colored "Location: ${APP_DIR}" $GREEN
print_colored "" $NC
print_colored "To register for URL handling:" $YELLOW
print_colored "  /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f '${APP_DIR}'" $YELLOW
print_colored "" $NC
print_colored "To test URL handling:" $YELLOW
print_colored "  open 'hook://test'" $YELLOW
print_colored "" $NC
print_colored "To unregister/remove:" $YELLOW
print_colored "  rm -rf '${APP_DIR}'" $YELLOW
print_colored "  /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -u '${APP_DIR}'" $YELLOW