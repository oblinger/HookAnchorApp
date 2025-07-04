#!/bin/bash

# Script to register hook:// URL handler for anchor_selector popup

POPUP_PATH="/Users/oblinger/ob/kmr/prj/2025-06 anchor_selector/target/release/popup"
PLIST_PATH="$HOME/Library/LaunchAgents/com.anchor-selector.url-handler.plist"

# Check if popup binary exists
if [[ ! -f "$POPUP_PATH" ]]; then
    echo "Error: popup binary not found at $POPUP_PATH"
    echo "Please build the popup first with: cargo build --release --bin popup"
    exit 1
fi

# Create the Launch Agent plist
cat > "$PLIST_PATH" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.anchor-selector.url-handler</string>
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