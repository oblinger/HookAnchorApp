#!/bin/bash

echo "HookAnchor Versioned Release Builder"
echo "==================================="

# Parse flags
force_overwrite=false
target_version=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --force|-f)
            force_overwrite=true
            shift
            ;;
        *)
            if [ -z "$target_version" ]; then
                target_version="$1"
            fi
            shift
            ;;
    esac
done

# Function to get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "//' | sed 's/"//'
}

# Function to get most recent release version
get_most_recent_version() {
    if [ -d "versions" ] && [ "$(ls -A versions 2>/dev/null)" ]; then
        ls versions | grep -E '^[0-9]+\.[0-9]+\.[0-9]+$' | sort -V | tail -1
    else
        echo "No previous versions found"
    fi
}

# Function to update version in Cargo.toml
update_cargo_version() {
    local new_version="$1"
    sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    echo "Updated Cargo.toml version to: $new_version"
}

# Show current and most recent versions
current_version=$(get_current_version)
recent_version=$(get_most_recent_version)

echo "Current Cargo.toml version: $current_version"
echo "Most recent release version: $recent_version"
echo ""

# Check if version argument provided
if [ -z "$target_version" ]; then
    echo "❌ No version specified!"
    echo ""
    echo "Usage: $0 <version> [--force|-f]"
    echo "Example: $0 0.2.0"
    echo "Example: $0 0.2.0 --force  # Skip overwrite confirmation"
    echo ""
    echo "Please specify the release version you want to create."
    exit 1
fi

# Validate version format (basic semver check)
if ! echo "$target_version" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo "❌ Invalid version format: $target_version"
    echo "Version must be in format: MAJOR.MINOR.PATCH (e.g., 1.2.3)"
    exit 1
fi

echo "Building version: $target_version"

# Check if version already exists
if [ -d "versions/$target_version" ]; then
    echo "⚠️  Version $target_version already exists!"
    echo "Contents of versions/$target_version:"
    ls -la "versions/$target_version"
    echo ""
    
    if [ "$force_overwrite" = true ]; then
        echo "Force flag specified, overwriting without confirmation..."
        confirm="y"
    else
        read -p "Do you want to overwrite this version? (y/N): " confirm
        echo "User response: '$confirm'"  # Debug line
    fi
    
    if [[ ! $confirm =~ ^[Yy]$ ]]; then
        echo "Build cancelled."
        exit 0
    fi
    
    echo "Removing old version..."
    echo "Running: rm -rf \"versions/$target_version\""  # Debug line
    rm -rf "versions/$target_version"
    
    # Verify deletion worked
    if [ -d "versions/$target_version" ]; then
        echo "❌ Failed to delete existing version directory!"
        echo "Directory still exists: versions/$target_version"
        ls -la "versions/$target_version"
        exit 1
    else
        echo "✅ Old version successfully deleted"
    fi
fi

# Update Cargo.toml with target version
echo "Updating version in Cargo.toml..."
update_cargo_version "$target_version"

# Set environment
export PATH="/opt/homebrew/bin:$PATH"

# Build release
echo ""
echo "Building release..."
echo "=================="
cargo build --release

if [[ $? -ne 0 ]]; then
    echo "❌ Build failed!"
    exit 1
fi

echo ""
echo "✅ Build successful!"

# Test the built binary
echo ""
echo "Testing built binary..."
echo "======================"
./target/release/ha --help | grep -q "HookAnchor" && echo "✅ Binary runs correctly" || echo "❌ Binary test failed"

# Create version directory and copy files
echo ""
echo "Creating version directory..."
echo "============================"
mkdir -p "versions/$target_version"

# Copy binary
cp "./target/release/ha" "versions/$target_version/ha"

# Copy to unversioned location for current use
cp "./target/release/ha" "./target/release/ha-current"

# Create version info file
cat > "versions/$target_version/VERSION" << EOF
Version: $target_version
Built: $(date)
Git Commit: $(git rev-parse HEAD 2>/dev/null || echo "unknown")
Built By: $(whoami)
EOF

# Create DMG for distribution
echo ""
echo "Creating DMG installer..."
echo "========================"
if python3 scripts/create_dmg.py; then
    echo "✅ DMG created successfully!"
    
    # Copy DMG to versioned directory
    dmg_file="target/dist/HookAnchor-$target_version.dmg"
    if [ -f "$dmg_file" ]; then
        cp "$dmg_file" "versions/$target_version/HookAnchor-$target_version.dmg"
        echo "  DMG copied to versions directory"
    fi
else
    echo "⚠️  DMG creation failed, but binary build was successful"
fi

echo ""
echo "✅ Version $target_version created successfully!"
echo ""
echo "Files created:"
echo "  versions/$target_version/ha (versioned binary)"
echo "  versions/$target_version/VERSION (version info)"
echo "  versions/$target_version/HookAnchor-$target_version.dmg (installer)"
echo "  target/release/ha-current (current unversioned binary)"
echo "  target/dist/HookAnchor-$target_version.dmg (distribution DMG)"
echo ""
echo "Binary info:"
ls -la "versions/$target_version/ha"
echo ""
echo "DMG info:"
if [ -f "versions/$target_version/HookAnchor-$target_version.dmg" ]; then
    ls -la "versions/$target_version/HookAnchor-$target_version.dmg"
else
    echo "  DMG not created"
fi
echo ""
echo "You can test with:"
echo "  ./versions/$target_version/ha --help"
echo "  ./target/release/ha-current --help"
echo ""
echo "Distribution ready:"
echo "  Upload versions/$target_version/HookAnchor-$target_version.dmg for users to download"