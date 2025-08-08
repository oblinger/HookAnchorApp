#!/bin/bash

# Create macOS app bundle with Swift supervisor as the main executable
# This matches the PRD architecture: supervisor is the single entry point

APP_NAME="HookAnchor"
APP_DIR="/Applications/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo "Creating supervisor-based app bundle..."

# Check for required binaries
if [[ ! -f "./target/release/HookAnchor" ]]; then
    echo "Building HookAnchor supervisor..."
    ./swift/build_supervisor.sh
fi

if [[ ! -f "./target/release/popup_server" ]]; then
    echo "Building Rust popup_server..."
    cargo build --release
fi

# Remove existing app if it exists
if [[ -d "$APP_DIR" ]]; then
    echo "Removing existing app bundle..."
    rm -rf "$APP_DIR"
fi

# Create directory structure
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Copy binaries
echo "Copying HookAnchor supervisor as main executable..."
cp "./target/release/HookAnchor" "$MACOS_DIR/${APP_NAME}"

echo "Copying popup_server..."
cp "./target/release/popup_server" "$MACOS_DIR/popup_server"

# Also copy ha for CLI usage
echo "Copying ha dispatcher..."
cp "./target/release/ha" "$MACOS_DIR/ha"

# Copy icon if available
if [[ -f "./resources/icons/icon.icns" ]]; then
    cp "./resources/icons/icon.icns" "$RESOURCES_DIR/icon.icns"
fi

# Create Info.plist with supervisor as the executable
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
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleIconFile</key>
    <string>icon</string>
    <key>LSUIElement</key>
    <true/>
    <key>LSMultipleInstancesProhibited</key>
    <true/>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLName</key>
            <string>Hook URL Handler</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>hook</string>
            </array>
        </dict>
    </array>
</dict>
</plist>
EOF

echo "App bundle created successfully!"
echo ""
echo "Structure:"
echo "  ${APP_DIR}"
echo "  ├── Contents/"
echo "  │   ├── Info.plist (points to supervisor)"
echo "  │   ├── MacOS/"
echo "  │   │   ├── ${APP_NAME} (supervisor - main entry)"
echo "  │   │   ├── popup_server (Rust GUI)"
echo "  │   │   └── ha (CLI dispatcher)"
echo "  │   └── Resources/"
echo "  │       └── icon.icns"
echo ""
echo "The supervisor is now the main entry point!"
echo ""
echo "To register: /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f ${APP_DIR}"
echo "To test: open ${APP_DIR}"
echo "To test URL: open hook://test"