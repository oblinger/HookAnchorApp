#!/bin/bash

# ⚠️ CRITICAL WARNING: READ docs/URL_HANDLING.md BEFORE MODIFYING URL HANDLING ⚠️
# This script creates a single .app bundle in target/dist for URL handling on dev machine
# The .app uses the safe URL handler binary to prevent GUI popups that lock up the system

set -e

# Configuration
APP_NAME="HookAnchor"
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

print_colored "Setting up development URL handler at ${APP_DIR}..." $YELLOW

# Create directory structure
mkdir -p "$MACOS_DIR"

# Copy both binaries
cp "$DISPATCHER_BINARY" "$MACOS_DIR/ha"
cp "$POPUP_BINARY" "$MACOS_DIR/popup"
chmod +x "$MACOS_DIR/ha"
chmod +x "$MACOS_DIR/popup"

# Create AppleScript wrapper as the main executable
cat > "$MACOS_DIR/${APP_NAME}" << 'EOF'
#!/usr/bin/osascript

-- HookAnchor AppleScript Wrapper
-- Routes URL schemes to the dispatcher with --hook flag

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

print_colored "✓ Development URL handler created successfully!" $GREEN
print_colored "Location: ${APP_DIR}" $GREEN
print_colored "" $NC
print_colored "Next steps:" $YELLOW
print_colored "1. Double-click the app once to register URL handling:" $NC
print_colored "   open '${APP_DIR}'" $YELLOW
print_colored "" $NC
print_colored "2. Test URL handling:" $NC
print_colored "   open 'hook://test'" $YELLOW
print_colored "" $NC
print_colored "3. After future builds, run this script again to update the binary" $NC
print_colored "" $NC
print_colored "To remove URL handler registration:" $YELLOW
print_colored "  rm -rf '${APP_DIR}'" $YELLOW