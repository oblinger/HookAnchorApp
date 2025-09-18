#!/bin/bash

# Create AppleScript Uninstaller App
# This script creates an app bundle that launches the uninstall script in Terminal

set -e

echo "🔨 Creating Uninstaller.app..."

# Get script location and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEMP_BUILD_DIR="$PROJECT_ROOT/temp_build"

# Create app bundle structure in the temp build
UNINSTALLER_APP="$TEMP_BUILD_DIR/HookAnchor.app/Contents/Resources/Uninstaller.app"
rm -rf "$UNINSTALLER_APP"
mkdir -p "$UNINSTALLER_APP/Contents/MacOS"
mkdir -p "$UNINSTALLER_APP/Contents/Resources"

# Create the AppleScript that launches the shell script in Terminal
cat > "$UNINSTALLER_APP/Contents/MacOS/applet" << 'EOF'
#!/usr/bin/osascript

tell application "Terminal"
    activate
    -- Get the path to this app's Resources folder
    set appPath to (path to me as string)
    set resourcesPath to appPath & "Contents:Resources:"
    set posixResourcesPath to POSIX path of (resourcesPath as alias)

    -- Run the uninstall script
    do script "cd '" & posixResourcesPath & "' && ./uninstall.sh"
end tell
EOF

# Make the AppleScript executable
chmod +x "$UNINSTALLER_APP/Contents/MacOS/applet"

# Create Info.plist for the app bundle
cat > "$UNINSTALLER_APP/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>applet</string>
    <key>CFBundleIconFile</key>
    <string>applet</string>
    <key>CFBundleIdentifier</key>
    <string>com.hookanchor.uninstaller</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>HookAnchor Uninstaller</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.9</string>
    <key>LSUIElement</key>
    <false/>
</dict>
</plist>
EOF

# Copy the uninstall script to the app's Resources
# Use the dist_uninstall.sh that was just created in the temp build
RESOURCES_DIR="$TEMP_BUILD_DIR/HookAnchor.app/Contents/Resources"
if [ -f "$RESOURCES_DIR/dist_uninstall.sh" ]; then
    cp "$RESOURCES_DIR/dist_uninstall.sh" "$UNINSTALLER_APP/Contents/Resources/uninstall.sh"
else
    echo "Error: Could not find dist_uninstall.sh in $RESOURCES_DIR"
    exit 1
fi

# Create a simple icon (we can improve this later)
# Use the system trash icon as a placeholder
cp "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/TrashIcon.icns" "$UNINSTALLER_APP/Contents/Resources/applet.icns" 2>/dev/null || true

echo "✅ Created Uninstaller.app"
echo "   Location: $UNINSTALLER_APP"
echo "   Users can double-click this to run the uninstaller"