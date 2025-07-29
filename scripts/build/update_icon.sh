#!/bin/bash

# HookAnchor Icon Management Script
# This script updates the application icon from a source PNG file

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Project root is two directories up from scripts/build/
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
APP_BUNDLE="/Applications/HookAnchor.app"
RESOURCES_DIR="$PROJECT_ROOT/resources"
SOURCE_ICON="$RESOURCES_DIR/icon.icns"

echo "🎨 HookAnchor Icon Management"
echo "============================"

# Check if source icon exists
if [ ! -f "$SOURCE_ICON" ]; then
    echo "❌ Source icon not found: $SOURCE_ICON"
    echo "Please add your icon.icns file to the resources/ directory"
    exit 1
fi

echo "📋 Source icon: $SOURCE_ICON"
echo "🎯 Target app: $APP_BUNDLE"

# Function to update icon in app bundle
update_app_icon() {
    echo "📦 Updating app bundle icon..."
    
    # Copy to both names (for compatibility)
    cp "$SOURCE_ICON" "$APP_BUNDLE/Contents/Resources/applet.icns"
    cp "$SOURCE_ICON" "$APP_BUNDLE/Contents/Resources/popup.icns"
    
    # Update app bundle timestamp
    touch "$APP_BUNDLE"
    
    echo "✅ App bundle icon updated"
}

# Function to update icon in distribution
update_distribution_icon() {
    echo "📦 Updating distribution icon..."
    
    if [ -d "$PROJECT_ROOT/dist" ]; then
        if [ -d "$PROJECT_ROOT/dist/HookAnchor.app" ]; then
            cp "$SOURCE_ICON" "$PROJECT_ROOT/dist/HookAnchor.app/Contents/Resources/applet.icns"
            cp "$SOURCE_ICON" "$PROJECT_ROOT/dist/HookAnchor.app/Contents/Resources/popup.icns"
            echo "✅ Distribution icon updated"
        else
            echo "ℹ️  Distribution app not found - run ./package_for_distribution.sh first"
        fi
    else
        echo "ℹ️  Distribution directory not found - run ./package_for_distribution.sh first"
    fi
}

# Function to clear icon cache
clear_icon_cache() {
    echo "🧹 Clearing icon cache..."
    
    # Clear system icon cache
    sudo rm -rf /Library/Caches/com.apple.iconservices.store 2>/dev/null || true
    
    # Clear user icon cache
    find /private/var/folders/ -name "com.apple.dock.iconcache" -delete 2>/dev/null || true
    find /private/var/folders/ -name "*.csstore" -delete 2>/dev/null || true
    
    # Re-register app
    sudo /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f "$APP_BUNDLE" 2>/dev/null || true
    
    echo "✅ Icon cache cleared"
}

# Function to restart UI services
restart_ui_services() {
    echo "🔄 Restarting UI services..."
    
    killall Dock 2>/dev/null || true
    killall Finder 2>/dev/null || true
    
    echo "✅ UI services restarted"
}

# Main execution
echo ""
echo "🔄 Updating HookAnchor icon..."
echo ""

# Update icons
update_app_icon
update_distribution_icon

# Clear cache and restart services
clear_icon_cache
restart_ui_services

echo ""
echo "🎉 Icon update complete!"
echo ""
echo "Changes applied to:"
echo "  • App bundle: $APP_BUNDLE"
echo "  • Distribution: $PROJECT_ROOT/dist/ (if exists)"
echo ""
echo "You may need to:"
echo "  • Remove and re-add HookAnchor from dock"
echo "  • Restart HookAnchor to see running icon changes"
echo ""
echo "Next time you run ./package_for_distribution.sh, the new icon will be included automatically."