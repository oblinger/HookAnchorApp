#!/bin/bash

# One-time setup: Create development app bundle in target/release/
# After this, builds will just update the binaries inside

RELEASE_DIR="./target/release"
APP_DIR="$RELEASE_DIR/HookAnchor.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

echo "Setting up development app bundle in $RELEASE_DIR..."

# Create directory structure
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Create Info.plist
cat > "$CONTENTS_DIR/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>HookAnchor</string>
    <key>CFBundleIdentifier</key>
    <string>com.hookanchor.dev</string>
    <key>CFBundleName</key>
    <string>HookAnchor Dev</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>LSUIElement</key>
    <true/>
    <key>LSMultipleInstancesProhibited</key>
    <true/>
</dict>
</plist>
EOF

# Copy icon if available
if [ -f "./resources/icons/icon.icns" ]; then
    cp "./resources/icons/icon.icns" "$RESOURCES_DIR/icon.icns"
fi

echo "✅ Development app bundle created at: $APP_DIR"
echo ""
echo "Now your build process is:"
echo "1. ./swift/build_supervisor.sh  (builds HookAnchor)"
echo "2. cargo build --release        (builds popup_server)"
echo "3. The binaries are automatically in the right place!"
echo ""
echo "To run: open $APP_DIR"
echo "Or double-click HookAnchor.app in target/release/"