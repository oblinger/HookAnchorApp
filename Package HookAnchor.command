#!/bin/bash

# Double-clickable version of the distribution packaging script
# This file can be double-clicked in Finder to run

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Clear terminal and show header
clear
echo "🔨 HookAnchor Distribution Packager"
echo "==================================="
echo "This will rebuild and package HookAnchor for distribution."
echo ""
read -p "Press Enter to continue or Ctrl+C to cancel..."

# Run the main packaging script
if [ -f "./package_for_distribution.sh" ]; then
    ./package_for_distribution.sh
else
    echo "Error: package_for_distribution.sh not found!"
    exit 1
fi

echo ""
echo "🎉 Packaging complete!"
echo "Distribution files are ready in the 'dist' folder."
echo ""
echo "Press Enter to open the distribution folder..."
read
open ./dist

echo "Done! You can now close this window."