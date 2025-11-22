#!/usr/bin/env bash
# Migration script: Move HookAnchor repo to new structure
# Old: ~/ob/proj/HookAnchor
# New: ~/ob/proj/HookAnchor/HookAnchorApp

set -e  # Exit on error

echo "=================================================="
echo "HookAnchor Repository Migration Script"
echo "=================================================="
echo ""
echo "This will:"
echo "  1. Stop all HookAnchor processes"
echo "  2. Create ~/ob/proj/HookAnchor/HookAnchorApp"
echo "  3. Move repo contents to new location"
echo "  4. Update git remote to HookAnchorApp"
echo "  5. Update all hardcoded paths in source files"
echo "  6. Run setup script to recreate all symlinks"
echo "  7. Rebuild binaries"
echo ""
echo "Old location: ~/ob/proj/HookAnchor"
echo "New location: ~/ob/proj/HookAnchor/HookAnchorApp"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

OLD_DIR="$HOME/ob/proj/HookAnchor"
NEW_PARENT="$HOME/ob/proj/HookAnchor"
NEW_DIR="$HOME/ob/proj/HookAnchor/HookAnchorApp"

# Verify we're in the right place
if [ ! -d "$OLD_DIR" ]; then
    echo "❌ Error: $OLD_DIR doesn't exist"
    exit 1
fi

if [ "$PWD" != "$OLD_DIR" ]; then
    echo "❌ Error: Please run this script from $OLD_DIR"
    exit 1
fi

# Step 1: Stop all processes
echo ""
echo "🛑 Step 1: Stopping all HookAnchor processes..."
pkill -f "HookAnchor" 2>/dev/null || true
pkill -f "hookanchor" 2>/dev/null || true
pkill -f "popup_server" 2>/dev/null || true
sleep 2
echo "✅ Processes stopped"

# Step 2: Create temporary directory and move contents
echo ""
echo "📁 Step 2: Moving repository to new location..."
TEMP_DIR="$HOME/ob/proj/HookAnchor_repo_temp"

# Move current repo to temp location
echo "  Moving repo to temporary location..."
cd "$HOME/ob/proj"
mv HookAnchor "$TEMP_DIR"

# Create new parent directory
mkdir -p "$NEW_PARENT"

# Move repo into new location
mv "$TEMP_DIR" "$NEW_DIR"

echo "✅ Repository moved to $NEW_DIR"

# Step 3: Update git remote
echo ""
echo "🔗 Step 3: Updating git remote URL..."
cd "$NEW_DIR"
CURRENT_REMOTE=$(git remote get-url origin)
echo "  Current remote: $CURRENT_REMOTE"

if [[ "$CURRENT_REMOTE" == *"HookAnchor.git" ]]; then
    NEW_REMOTE="${CURRENT_REMOTE//HookAnchor.git/HookAnchorApp.git}"
    git remote set-url origin "$NEW_REMOTE"
    echo "  Updated to: $NEW_REMOTE"
    echo "✅ Git remote updated"
else
    echo "  ⚠️  Remote URL doesn't match expected pattern, skipping update"
fi

# Step 4: Update hardcoded paths in source files
echo ""
echo "📝 Step 4: Updating hardcoded paths in source files..."

# Function to update paths in a file
update_file_paths() {
    local file="$1"
    if [ -f "$file" ]; then
        echo "  Updating $file..."
        # Replace ~/ob/proj/HookAnchor with ~/ob/proj/HookAnchor/HookAnchorApp
        # But NOT if it already has HookAnchorApp in it
        sed -i '' 's|~/ob/proj/HookAnchor/HookAnchorApp|TEMP_PLACEHOLDER|g' "$file"
        sed -i '' 's|~/ob/proj/HookAnchor|~/ob/proj/HookAnchor/HookAnchorApp|g' "$file"
        sed -i '' 's|TEMP_PLACEHOLDER|~/ob/proj/HookAnchor/HookAnchorApp|g' "$file"

        # Same for absolute paths
        sed -i '' 's|/Users/oblinger/ob/proj/HookAnchor/HookAnchorApp|TEMP_PLACEHOLDER|g' "$file"
        sed -i '' 's|/Users/oblinger/ob/proj/HookAnchor|/Users/oblinger/ob/proj/HookAnchor/HookAnchorApp|g' "$file"
        sed -i '' 's|TEMP_PLACEHOLDER|/Users/oblinger/ob/proj/HookAnchor/HookAnchorApp|g' "$file"
    fi
}

# List of files to update
FILES_TO_UPDATE=(
    "Cargo.toml"
    "justfile"
    "src/cmd.rs"
    "src/utils/build_verification.rs"
    "swift/Supervisor/HookAnchor.swift"
    "config/config.js"
)

for file in "${FILES_TO_UPDATE[@]}"; do
    update_file_paths "$file"
done

# Update test scripts
find tests -name "*.sh" -type f 2>/dev/null | while read -r file; do
    update_file_paths "$file"
done

echo "✅ Source files updated"

# Step 5: Run setup script to recreate all symlinks
echo ""
echo "🔗 Step 5: Running setup script to recreate symlinks..."
cd "$NEW_DIR"

if [ -f "dev-scripts/setup/setup_dev_app.sh" ]; then
    echo "  Running dev-scripts/setup/setup_dev_app.sh..."
    ./dev-scripts/setup/setup_dev_app.sh
    echo "✅ Setup script completed - all symlinks recreated"
else
    echo "  ❌ Setup script not found!"
    exit 1
fi

# Step 6: Verify symlinks
echo ""
echo "🔍 Step 6: Verifying symlinks..."

echo "  App bundle symlinks:"
ls -la /Applications/HookAnchor.app/Contents/MacOS/ | grep -E "HookAnchor|popup|ha" || true

echo ""
echo "  ~/bin symlinks:"
ls -la ~/bin/ha ~/bin/popup_server 2>/dev/null || true

echo "✅ Symlinks verified"

# Final verification
echo ""
echo "=================================================="
echo "✅ Migration Complete!"
echo "=================================================="
echo ""
echo "New repository location: $NEW_DIR"
echo ""
echo "Summary of changes:"
echo "  ✓ Repository moved to: $NEW_DIR"
echo "  ✓ Git remote updated to HookAnchorApp.git"
echo "  ✓ Hardcoded paths updated in source files"
echo "  ✓ All symlinks recreated by setup script"
echo "  ✓ Binaries built in new location"
echo ""
echo "Next steps:"
echo "  1. Start the app: $NEW_DIR/target/release/ha --restart"
echo "  2. Test with: ha -m test"
echo "  3. Test URL handler: open 'hook://test'"
echo "  4. Verify everything works as expected"
echo ""
