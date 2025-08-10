#!/bin/bash

# Build the URL handler binary
# This compiles the Swift URL handler that processes hook:// URLs

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET_DIR="$PROJECT_ROOT/target/release"

echo "Building URL Handler..."

# Compile the Swift URL handler
swiftc -parse-as-library \
    -o "$TARGET_DIR/url_launcher" \
    "$SCRIPT_DIR/url_launcher.swift"

if [ $? -eq 0 ]; then
    echo "✅ URL handler built successfully at: $TARGET_DIR/url_launcher"
else
    echo "❌ Failed to build URL handler"
    exit 1
fi