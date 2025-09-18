#!/bin/bash

# HookAnchor Uninstaller
# This script safely removes HookAnchor components while preserving user data by default.
# It will be copied to ~/.config/hookanchor/ during installation for easy access.

set -e  # Exit on any error

echo "🗑️  HookAnchor Uninstaller"
echo "========================="
echo ""

# Check what exists and show current state
echo "Checking current installation..."
app_exists=false
karabiner_exists=false
url_handler_exists=false

if [ -d "/Applications/HookAnchor.app" ]; then
    echo "✓ Found /Applications/HookAnchor.app"
    app_exists=true
fi

if [ -f "$HOME/.config/karabiner/assets/complex_modifications/hookanchor.json" ]; then
    echo "✓ Found Karabiner configuration"
    karabiner_exists=true
fi

# Check for URL handler registration (this is approximate)
if /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -dump | grep -q "hookanchor\|HookAnchor" 2>/dev/null; then
    echo "✓ Found URL handler registration"
    url_handler_exists=true
fi

if [ -d "$HOME/.config/hookanchor" ]; then
    echo "✓ Found configuration directory"
fi

# If nothing found, exit early
if [ "$app_exists" = false ] && [ "$karabiner_exists" = false ] && [ "$url_handler_exists" = false ]; then
    echo ""
    echo "No HookAnchor installation found. Nothing to uninstall."
    exit 0
fi

echo ""
echo "This will remove:"
echo "  • /Applications/HookAnchor.app"
echo "  • Karabiner configuration for HookAnchor"
echo "  • URL handler registration"
echo "  • Running HookAnchor processes"
echo ""
echo "Your configuration files will be PRESERVED in ~/.config/hookanchor"
echo ""
echo "Options:"
echo "  yes        - Uninstall HookAnchor (keep config files)"
echo "  everything - Remove EVERYTHING including all config files"
echo "  anything else - Cancel"
echo ""
read -p "Your choice: " response

case "$response" in
    yes|Yes|YES)
        echo ""
        echo "🗑️  Uninstalling HookAnchor (preserving config)..."

        # Kill any running processes
        echo "Stopping HookAnchor processes..."

        # Kill specific HookAnchor processes by exact name (safe)
        killall popup_server 2>/dev/null || true
        killall HookAnchor 2>/dev/null || true

        # Use pkill with specific paths to avoid false matches
        pkill -f "/Applications/HookAnchor.app" 2>/dev/null || true
        pkill -f "HookAnchor/target/release/popup_server" 2>/dev/null || true
        pkill -f "HookAnchor/target/release/HookAnchor" 2>/dev/null || true
        pkill -f "HookAnchor/target/release/ha.*--start-server-daemon" 2>/dev/null || true

        # Give processes time to shutdown gracefully
        sleep 2

        # Force kill if still running (only specific names)
        killall -9 popup_server 2>/dev/null || true
        killall -9 HookAnchor 2>/dev/null || true

        # Remove application
        if [ "$app_exists" = true ]; then
            echo "Removing /Applications/HookAnchor.app..."
            rm -rf "/Applications/HookAnchor.app"
        fi

        # Remove Karabiner configuration
        if [ "$karabiner_exists" = true ]; then
            echo "Removing Karabiner configuration..."
            rm -f "$HOME/.config/karabiner/assets/complex_modifications/hookanchor.json"
        fi

        # Unregister URL handler (best effort)
        echo "Unregistering URL handler..."
        /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -u "/Applications/HookAnchor.app" 2>/dev/null || true

        echo ""
        echo "✅ HookAnchor uninstalled successfully!"
        echo ""
        echo "Preserved:"
        echo "  • ~/.config/hookanchor (your commands and settings)"
        echo "  • This uninstall script"
        echo ""
        echo "To completely remove everything later, run this script again and choose 'everything'"
        ;;

    everything|Everything|EVERYTHING)
        echo ""
        echo "🗑️  COMPLETE REMOVAL - This will delete ALL HookAnchor data..."
        echo ""
        echo "⚠️  WARNING: This will permanently delete:"
        echo "  • All your commands and settings"
        echo "  • All configuration files"
        echo "  • This uninstall script itself"
        echo ""
        read -p "Are you absolutely sure? Type 'DELETE ALL' to confirm: " confirm

        if [ "$confirm" = "DELETE ALL" ]; then
            echo ""
            echo "🗑️  Removing everything..."

            # Kill any running processes
            echo "Stopping HookAnchor processes..."

            # Kill specific HookAnchor processes by exact name (safe)
            killall popup_server 2>/dev/null || true
            killall HookAnchor 2>/dev/null || true

            # Use pkill with specific paths to avoid false matches
            pkill -f "/Applications/HookAnchor.app" 2>/dev/null || true
            pkill -f "HookAnchor/target/release/popup_server" 2>/dev/null || true
            pkill -f "HookAnchor/target/release/HookAnchor" 2>/dev/null || true
            pkill -f "HookAnchor/target/release/ha.*--start-server-daemon" 2>/dev/null || true

            # Give processes time to shutdown gracefully
            sleep 2

            # Force kill if still running (only specific names)
            killall -9 popup_server 2>/dev/null || true
            killall -9 HookAnchor 2>/dev/null || true

            # Remove application
            if [ "$app_exists" = true ]; then
                echo "Removing /Applications/HookAnchor.app..."
                rm -rf "/Applications/HookAnchor.app"
            fi

            # Remove Karabiner configuration
            if [ "$karabiner_exists" = true ]; then
                echo "Removing Karabiner configuration..."
                rm -f "$HOME/.config/karabiner/assets/complex_modifications/hookanchor.json"
            fi

            # Unregister URL handler
            echo "Unregistering URL handler..."
            /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -u "/Applications/HookAnchor.app" 2>/dev/null || true

            # Remove entire config directory (including this script)
            echo "Removing all configuration files..."
            rm -rf "$HOME/.config/hookanchor"

            echo ""
            echo "✅ HookAnchor completely removed!"
            echo "Note: This uninstall script was also deleted."

        else
            echo "Complete removal cancelled. (You must type exactly 'DELETE ALL')"
        fi
        ;;

    *)
        echo "Uninstall cancelled."
        ;;
esac