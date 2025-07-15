#!/bin/bash

echo "Building HookAnchor Release with --grab feature..."
echo "=================================================="

# Set environment
export PATH="/opt/homebrew/bin:$PATH"

# Build release
echo "Running: cargo build --release"
cargo build --release

if [[ $? -eq 0 ]]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "Testing --grab option:"
    ./target/release/ha --help | grep grab || echo "❌ --grab option not found in help"
    echo ""
    echo "Binary info:"
    ls -la ./target/release/ha
    echo ""
    echo "You can now test with:"
    echo "  ./target/release/ha --grab 0"
else
    echo "❌ Build failed!"
    exit 1
fi