#!/bin/bash

# Build the URL handler binary
# This compiles the Swift URL handler that processes hook:// URLs

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET_DIR="$PROJECT_ROOT/target/release"

echo "Building URL Handler (Universal Binary)..."

# Build for both architectures
ARM64_BINARY="$TARGET_DIR/url_launcher-arm64"
X86_BINARY="$TARGET_DIR/url_launcher-x86_64"
UNIVERSAL_BINARY="$TARGET_DIR/url_launcher"

# Compile for ARM64 (Apple Silicon)
echo "  Building for ARM64..."
swiftc -parse-as-library \
    -target arm64-apple-macosx10.15 \
    -o "$ARM64_BINARY" \
    "$SCRIPT_DIR/url_launcher.swift"

# Compile for x86_64 (Intel)
echo "  Building for x86_64..."
swiftc -parse-as-library \
    -target x86_64-apple-macosx10.15 \
    -o "$X86_BINARY" \
    "$SCRIPT_DIR/url_launcher.swift"

# Create universal binary
echo "  Creating universal binary..."
lipo -create -output "$UNIVERSAL_BINARY" "$ARM64_BINARY" "$X86_BINARY"

# Clean up architecture-specific binaries
rm -f "$ARM64_BINARY" "$X86_BINARY"

if [ -f "$UNIVERSAL_BINARY" ]; then
    echo "✅ URL handler built successfully at: $TARGET_DIR/url_launcher"
    # Verify it's universal
    echo "  Architecture info:"
    lipo -info "$UNIVERSAL_BINARY" | sed 's/^/    /'
else
    echo "❌ Failed to build URL handler"
    exit 1
fi