#!/bin/bash
# HookAnchor Installation Script
#
# This script helps install or repair HookAnchor installation
# Usage: ./install.sh [--gui]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🚀 HookAnchor Installation Script"
echo "================================"
echo ""

# Detect the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
RELEASE_DIR="${SCRIPT_DIR}/target/release"

# Check if we're in a HookAnchor directory
if [ ! -f "${SCRIPT_DIR}/Cargo.toml" ] || ! grep -q "name = \"hookanchor\"" "${SCRIPT_DIR}/Cargo.toml" 2>/dev/null; then
    echo -e "${RED}Error: This script must be run from the HookAnchor project directory${NC}"
    echo "Please cd to the HookAnchor directory and try again."
    exit 1
fi

# Function to check if a binary exists
check_binary() {
    local binary="$1"
    if [ -f "${RELEASE_DIR}/${binary}" ]; then
        echo -e "${GREEN}✓${NC} Found ${binary}"
        return 0
    else
        echo -e "${YELLOW}✗${NC} ${binary} not found"
        return 1
    fi
}

# Function to build HookAnchor
build_hookanchor() {
    echo ""
    echo "Building HookAnchor..."
    if command -v cargo >/dev/null 2>&1; then
        cargo build --release
        echo -e "${GREEN}Build completed!${NC}"
    else
        echo -e "${RED}Error: Rust/Cargo not found. Please install Rust first:${NC}"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
}

# Parse command line arguments
USE_GUI=false
FORCE_BUILD=false
FORCE_INSTALL=false

for arg in "$@"; do
    case $arg in
        --gui)
            USE_GUI=true
            shift
            ;;
        --build)
            FORCE_BUILD=true
            shift
            ;;
        --force)
            FORCE_INSTALL=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --gui     Use GUI installer instead of CLI"
            echo "  --build   Force rebuild before installing"
            echo "  --force   Force reinstall (overwrite existing configs)"
            echo "  --help    Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0              # Run CLI installer"
            echo "  $0 --gui        # Run GUI installer"
            echo "  $0 --build      # Rebuild and run installer"
            echo "  $0 --gui --force # Force reinstall with GUI"
            exit 0
            ;;
        *)
            echo -e "${YELLOW}Unknown option: $arg${NC}"
            ;;
    esac
done

# Check current installation status
echo "Checking HookAnchor installation status..."
echo ""

# Check for existing config files
CONFIG_DIR="$HOME/.config/hookanchor"
if [ -d "$CONFIG_DIR" ]; then
    echo "Found existing configuration directory: $CONFIG_DIR"

    # Check what's installed
    [ -f "$CONFIG_DIR/config.yaml" ] && echo -e "  ${GREEN}✓${NC} config.yaml exists"
    [ -f "$CONFIG_DIR/config.js" ] && echo -e "  ${GREEN}✓${NC} config.js exists"
    [ -f "$CONFIG_DIR/hook_anchor_zshrc" ] && echo -e "  ${GREEN}✓${NC} hook_anchor_zshrc exists"
    [ -f "$CONFIG_DIR/commands.txt" ] && echo -e "  ${GREEN}✓${NC} commands.txt exists"

    # Check for Karabiner config
    if ls ~/.config/karabiner/assets/complex_modifications/hookanchor*.json >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓${NC} Karabiner configuration exists"
    fi
else
    echo "No existing HookAnchor configuration found."
fi

echo ""

# Check for binaries or build if needed
if [ "$FORCE_BUILD" = true ]; then
    build_hookanchor
else
    echo "Checking for HookAnchor binaries..."

    if check_binary "ha" && check_binary "popup_server"; then
        if [ "$USE_GUI" = true ] && ! check_binary "installer_gui"; then
            echo ""
            echo -e "${YELLOW}GUI installer not found. Building it now...${NC}"
            build_hookanchor
        fi
    else
        echo ""
        echo -e "${YELLOW}Some binaries are missing. Building HookAnchor...${NC}"
        build_hookanchor
    fi
fi

echo ""

# Add to PATH if not already there
if ! command -v ha >/dev/null 2>&1; then
    echo "Would you like to add HookAnchor to your PATH? (recommended)"
    echo "This will create a symlink in ~/bin/"
    read -p "Add to PATH? [Y/n]: " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]] || [ -z "$REPLY" ]; then
        mkdir -p ~/bin
        ln -sf "${RELEASE_DIR}/ha" ~/bin/ha
        echo -e "${GREEN}Created symlink: ~/bin/ha${NC}"
        echo ""
        echo "Add this to your shell configuration (.zshrc or .bashrc):"
        echo '  export PATH="$HOME/bin:$PATH"'
        echo ""
    fi
fi

# Run the installer
if [ "$USE_GUI" = true ]; then
    echo "Launching GUI installer..."
    echo ""

    if [ -f "${RELEASE_DIR}/installer_gui" ]; then
        "${RELEASE_DIR}/installer_gui"
    else
        echo -e "${RED}Error: GUI installer not found at ${RELEASE_DIR}/installer_gui${NC}"
        echo "Please run: cargo build --release --bin installer_gui"
        exit 1
    fi
else
    echo "Running CLI installer..."
    echo ""

    if [ -f "${RELEASE_DIR}/ha" ]; then
        if [ "$FORCE_INSTALL" = true ]; then
            "${RELEASE_DIR}/ha" --install --force
        else
            "${RELEASE_DIR}/ha" --install
        fi
    else
        echo -e "${RED}Error: ha binary not found at ${RELEASE_DIR}/ha${NC}"
        echo "Please run: cargo build --release"
        exit 1
    fi
fi

echo ""
echo "✨ Installation process completed!"
echo ""
echo "Next steps:"
echo "  1. Source your shell configuration: source ~/.zshrc"
echo "  2. Add this to your .zshrc to use ff, fp, fd commands:"
echo "     source ~/.config/hookanchor/hook_anchor_zshrc"
echo "  3. Test HookAnchor: ha"
echo ""