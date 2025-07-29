#!/bin/bash

echo "🔧 Building HookAnchor with Grabber Focus Fix"
echo "============================================="

# Set environment
export PATH="/opt/homebrew/bin:$PATH"

# Build release version
echo "Building release version..."
cargo build --release

if [[ $? -eq 0 ]]; then
    echo ""
    echo "✅ Build successful!"
    
    # Update build timestamp in binary
    NEW_TIMESTAMP=$(date '+%I:%M %p %B %d, %Y')
    echo "New build timestamp: $NEW_TIMESTAMP"
    
    echo ""
    echo "🧪 Test the fix:"
    echo "1. Select a file in Finder (highlight it in blue)"
    echo "2. Launch HookAnchor GUI"
    echo "3. Press '+' key to start grabber"
    echo "4. Wait for countdown"
    echo "5. Should now capture the SELECTED FILE instead of the folder"
    echo ""
    echo "Expected behavior:"
    echo "  Before: Creates 'folder' command for current directory"
    echo "  After:  Creates 'obs' or 'doc' command for selected file"
    echo ""
    echo "💡 The fix: Grabber now captures BEFORE switching focus back to HookAnchor"
else
    echo "❌ Build failed!"
    exit 1
fi