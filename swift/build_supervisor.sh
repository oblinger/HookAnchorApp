#!/bin/bash

# Build script for Swift Supervisor

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"
SWIFT_DIR="$SCRIPT_DIR/Supervisor"
BUILD_DIR="$PROJECT_ROOT/target/release"

echo "Building Swift Supervisor (Universal Binary)..."

# Create build directory if it doesn't exist
mkdir -p "$BUILD_DIR"

# Build for both architectures
ARM64_BINARY="$BUILD_DIR/HookAnchorSupervisor-arm64"
X86_BINARY="$BUILD_DIR/HookAnchorSupervisor-x86_64"
UNIVERSAL_BINARY="$BUILD_DIR/HookAnchorSupervisor"

# Compile for ARM64 (Apple Silicon)
echo "  Building for ARM64..."
swiftc -O \
    -parse-as-library \
    -target arm64-apple-macosx10.15 \
    -o "$ARM64_BINARY" \
    "$SWIFT_DIR/HookAnchor.swift" \
    -framework Cocoa \
    -framework Foundation

# Compile for x86_64 (Intel)
echo "  Building for x86_64..."
swiftc -O \
    -parse-as-library \
    -target x86_64-apple-macosx10.15 \
    -o "$X86_BINARY" \
    "$SWIFT_DIR/HookAnchor.swift" \
    -framework Cocoa \
    -framework Foundation

# Create universal binary
echo "  Creating universal binary..."
lipo -create -output "$UNIVERSAL_BINARY" "$ARM64_BINARY" "$X86_BINARY"

# Clean up architecture-specific binaries
rm -f "$ARM64_BINARY" "$X86_BINARY"

echo "HookAnchor supervisor built at: $BUILD_DIR/HookAnchorSupervisor"

# Make it executable
chmod +x "$BUILD_DIR/HookAnchorSupervisor"

# Verify it's universal
echo "  Architecture info:"
lipo -info "$UNIVERSAL_BINARY" | sed 's/^/    /'

# IMPORTANT: Never copy binaries! The app bundle uses symlinks
echo "✅ Build complete!"
echo "Note: /Applications/HookAnchor.app uses symlinks to this binary"
echo "To test, run: $BUILD_DIR/HookAnchorSupervisor"