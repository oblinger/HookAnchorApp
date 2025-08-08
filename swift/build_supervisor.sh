#!/bin/bash

# Build script for Swift Supervisor

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"
SWIFT_DIR="$SCRIPT_DIR/Supervisor"
BUILD_DIR="$PROJECT_ROOT/target/release"

echo "Building Swift Supervisor..."

# Create build directory if it doesn't exist
mkdir -p "$BUILD_DIR"

# Compile Swift supervisor
swiftc -O \
    -parse-as-library \
    -o "$BUILD_DIR/supervisor" \
    "$SWIFT_DIR/main.swift" \
    -framework Cocoa \
    -framework Foundation

echo "Swift Supervisor built at: $BUILD_DIR/supervisor"

# Make it executable
chmod +x "$BUILD_DIR/supervisor"

# Optional: Create a simple launcher script for testing
cat > "$BUILD_DIR/launch_supervisor.sh" << 'EOF'
#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
"$DIR/supervisor"
EOF
chmod +x "$BUILD_DIR/launch_supervisor.sh"

echo "Build complete!"
echo "To test, run: $BUILD_DIR/launch_supervisor.sh"