#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Project root is two directories up from scripts/build/
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "Building Rust components..."
cargo build --release

echo "Building Swift supervisor..."
if [ -f "$PROJECT_ROOT/swift/build_supervisor.sh" ]; then
    "$PROJECT_ROOT/swift/build_supervisor.sh"
else
    echo "Warning: Swift supervisor build script not found"
fi

echo "Build completed. Check for any errors above."