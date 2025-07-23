#!/bin/bash

# Simple launcher script for Anchor Selector
# This can be assigned to F10 in macOS System Settings > Keyboard > Keyboard Shortcuts

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
POPUP_BINARY="$SCRIPT_DIR/target/release/popup"

# Build if needed
if [ ! -f "$POPUP_BINARY" ]; then
    echo "Building popup..."
    cd "$SCRIPT_DIR"
    cargo build --release --bin popup
fi

# Launch the popup
exec "$POPUP_BINARY"