#!/bin/bash

# Script to create AppleScript app bundle for hook:// URL handling
# Following the pattern from Spot5.app

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
APPLESCRIPT_FILE="$SCRIPT_DIR/hook_url_handler.applescript"
APP_NAME="HookAnchor"
APP_PATH="/Applications/$APP_NAME.app"

echo "Creating AppleScript app bundle for hook:// URL handling..."

# Check if AppleScript file exists
if [[ ! -f "$APPLESCRIPT_FILE" ]]; then
    echo "Error: AppleScript file not found at $APPLESCRIPT_FILE"
    exit 1
fi

# Check if HookAnchor binary exists
BINARY_PATH="$PROJECT_DIR/target/release/ha"
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "Error: HookAnchor binary not found at $BINARY_PATH"
    echo "Please build the binary first with: cargo build --release"
    exit 1
fi

# Remove existing app if it exists
if [[ -d "$APP_PATH" ]]; then
    echo "Removing existing app bundle..."
    rm -rf "$APP_PATH"
fi

# Compile the AppleScript into an app bundle
echo "Compiling AppleScript..."
osacompile -o "$APP_PATH" "$APPLESCRIPT_FILE"

if [[ ! -d "$APP_PATH" ]]; then
    echo "Error: Failed to create app bundle"
    exit 1
fi

# Create the Info.plist with URL scheme registration
echo "Creating Info.plist with URL scheme registration..."
cat > "$APP_PATH/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleAllowMixedLocalizations</key>
    <true/>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>applet</string>
    <key>CFBundleIconFile</key>
    <string>applet</string>
    <key>CFBundleIdentifier</key>
    <string>com.hookanchor.applescript</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>HookAnchor</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleSignature</key>
    <string>aplt</string>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLName</key>
            <string>com.hookanchor.hook</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>hook</string>
            </array>
        </dict>
    </array>
    <key>LSMinimumSystemVersionByArchitecture</key>
    <dict>
        <key>x86_64</key>
        <string>10.6</string>
    </dict>
    <key>LSRequiresCarbon</key>
    <true/>
    <key>NSAppleEventsUsageDescription</key>
    <string>This script needs to control other applications to run.</string>
    <key>OSAAppletShowStartupScreen</key>
    <false/>
</dict>
</plist>
EOF

# Register the URL scheme with the system
echo "Registering URL scheme with Launch Services..."
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f "$APP_PATH"

# Reset Launch Services database to ensure clean registration
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -kill -r -domain local -domain system -domain user

echo "AppleScript app bundle created successfully!"
echo "Location: $APP_PATH"
echo ""
echo "To test: open 'hook://test'"
echo "To uninstall: rm -rf '$APP_PATH'"