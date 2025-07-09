#!/bin/bash

# Create a minimal macOS app bundle for URL scheme registration
APP_NAME="HookAnchor"
APP_DIR="/Applications/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"

# Find the HookAnchor binary in common locations
POSSIBLE_PATHS=(
    "./target/release/ha"
    "./target/debug/ha"
    "$(which ha 2>/dev/null)"
    "$HOME/bin/ha"
    "/usr/local/bin/ha"
)

HA_BINARY=""
for path in "${POSSIBLE_PATHS[@]}"; do
    if [[ -f "$path" ]]; then
        HA_BINARY="$path"
        break
    fi
done

if [[ -z "$HA_BINARY" ]]; then
    echo "Error: HookAnchor binary not found in any of the following locations:"
    printf "  %s\n" "${POSSIBLE_PATHS[@]}"
    echo ""
    echo "Please build the project first with: cargo build --release"
    exit 1
fi

echo "Using HookAnchor binary at: $HA_BINARY"

echo "Creating app bundle at ${APP_DIR}..."

# Remove existing app if it exists
if [[ -d "$APP_DIR" ]]; then
    echo "Removing existing app bundle..."
    rm -rf "$APP_DIR"
fi

# Create directory structure
mkdir -p "$MACOS_DIR"

# Copy the binary
cp "$HA_BINARY" "$MACOS_DIR/${APP_NAME}"

# Create Info.plist
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
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLName</key>
            <string>Hook URL Handler</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>hook</string>
            </array>
            <key>CFBundleURLIconFile</key>
            <string></string>
        </dict>
    </array>
</dict>
</plist>
EOF

echo "App bundle created successfully!"
echo "Location: ${APP_DIR}"
echo ""
echo "To register URL scheme, run:"
echo "  /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f ${APP_DIR}"
echo ""
echo "To test: open hook://test"
echo "To uninstall: rm -rf '${APP_DIR}'"