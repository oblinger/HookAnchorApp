#!/bin/bash

# Script to register hook:// URL handler for HookAnchor application

# Find the HookAnchor binary in common locations
POSSIBLE_PATHS=(
    "./target/release/ha"
    "./target/debug/ha"
    "$(which ha 2>/dev/null)"
    "$HOME/bin/ha"
    "/usr/local/bin/ha"
    "/Applications/HookAnchor.app/Contents/MacOS/ha"
)

POPUP_PATH=""
for path in "${POSSIBLE_PATHS[@]}"; do
    if [[ -f "$path" ]]; then
        POPUP_PATH="$path"
        break
    fi
done

if [[ -z "$POPUP_PATH" ]]; then
    echo "Error: HookAnchor binary not found in any of the following locations:"
    printf "  %s\n" "${POSSIBLE_PATHS[@]}"
    echo ""
    echo "Please build the project first with: cargo build --release"
    exit 1
fi

echo "Using HookAnchor binary at: $POPUP_PATH"
PLIST_PATH="$HOME/Library/LaunchAgents/com.hookanchor.url-handler.plist"

# Create the Launch Agent plist
cat > "$PLIST_PATH" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.hookanchor.url-handler</string>
    <key>ProgramArguments</key>
    <array>
        <string>$POPUP_PATH</string>
        <string>%u</string>
    </array>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleURLName</key>
            <string>Hook URL</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>hook</string>
            </array>
        </dict>
    </array>
</dict>
</plist>
EOF

# Load the Launch Agent
launchctl load "$PLIST_PATH"

echo "URL handler registered successfully!"
echo "You can now use URLs like: hook://spot"
echo ""
echo "To test: open 'hook://spot' in Terminal or browser"
echo "To unregister: launchctl unload '$PLIST_PATH' && rm '$PLIST_PATH'"