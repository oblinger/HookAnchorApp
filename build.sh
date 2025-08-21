#!/bin/bash
# Quick build script for HookAnchor

echo "Building HookAnchor release version..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
    echo "Binary location: ./target/release/ha"
else
    echo "✗ Build failed!"
    exit 1
fi