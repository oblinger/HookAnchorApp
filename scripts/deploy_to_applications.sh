#!/bin/bash

# Deploy HookAnchor to /Applications folder
# This script copies the release build to /Applications

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
APP_PATH="$PROJECT_ROOT/target/release/HookAnchor.app"
DEST_PATH="/Applications/HookAnchor.app"

if [ ! -d "$APP_PATH" ]; then
    echo "❌ Error: HookAnchor.app not found at $APP_PATH"
    echo "Please build the app first with: cargo build --release"
    exit 1
fi

echo "🚀 Deploying HookAnchor to /Applications..."

# Kill running instances
killall HookAnchor 2>/dev/null
killall popup_server 2>/dev/null

# Small delay to ensure processes are terminated
sleep 0.5

# Remove old version and copy new one
rm -rf "$DEST_PATH"
cp -R "$APP_PATH" "$DEST_PATH"

echo "✅ HookAnchor deployed to /Applications"
echo "   You can now launch it from /Applications/HookAnchor.app"