#!/bin/bash

# HookAnchor Signing and Notarization Script
# Signs binaries with hardened runtime and notarizes with Apple
#
# Prerequisites:
# 1. Developer ID Application certificate installed in Keychain
# 2. App Store Connect API key OR App-specific password configured
#
# Setup for notarization (choose one):
# Option A: App Store Connect API key (recommended)
#   - Create API key at https://appstoreconnect.apple.com/access/api
#   - Store as ~/.private_keys/AuthKey_XXXXXXXXXX.p8
#   - Set environment variables:
#     export APPLE_API_KEY_ID="your_key_id"
#     export APPLE_API_ISSUER="your_issuer_id"
#
# Option B: App-specific password
#   - Create at https://appleid.apple.com/account/manage
#   - Store in keychain: xcrun notarytool store-credentials "AC_PASSWORD" \
#       --apple-id "your@email.com" --team-id "7M5VK5V43M" --password "your-app-password"

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEAM_ID="7M5VK5V43M"  # Daniel Oblinger
SIGNING_IDENTITY="Developer ID Application: Daniel Oblinger (7M5VK5V43M)"

# Entitlements file path
ENTITLEMENTS_FILE="$SCRIPT_DIR/entitlements.plist"

# Usage
usage() {
    echo "Usage: $0 [options] <target>"
    echo ""
    echo "Targets:"
    echo "  binaries    Sign release binaries only"
    echo "  app         Sign and notarize HookAnchor.app"
    echo "  dmg         Sign and notarize distribution DMG"
    echo ""
    echo "Options:"
    echo "  --skip-notarize    Sign only, don't notarize"
    echo "  --verbose          Show detailed output"
    echo ""
    echo "Examples:"
    echo "  $0 binaries           # Sign release binaries for local dev"
    echo "  $0 app                # Sign and notarize app bundle"
    echo "  $0 dmg                # Sign and notarize DMG for distribution"
    exit 1
}

# Check prerequisites
check_prereqs() {
    echo -e "${BLUE}Checking prerequisites...${NC}"

    # Check for signing identity
    if ! security find-identity -v -p codesigning | grep -q "$SIGNING_IDENTITY"; then
        echo -e "${RED}ERROR: Signing identity not found: $SIGNING_IDENTITY${NC}"
        echo "Available identities:"
        security find-identity -v -p codesigning
        exit 1
    fi
    echo "  ✓ Signing identity found"

    # Check for entitlements file
    if [ ! -f "$ENTITLEMENTS_FILE" ]; then
        echo -e "${YELLOW}Creating entitlements file...${NC}"
        create_entitlements
    fi
    echo "  ✓ Entitlements file exists"

    # Check notarization credentials (only for notarize operations)
    if [ "$SKIP_NOTARIZE" != "true" ]; then
        if [ -n "$APPLE_API_KEY_ID" ] && [ -n "$APPLE_API_ISSUER" ]; then
            echo "  ✓ API key credentials configured"
        elif xcrun notarytool store-credentials --help >/dev/null 2>&1; then
            if xcrun notarytool history --keychain-profile "AC_PASSWORD" >/dev/null 2>&1; then
                echo "  ✓ Keychain profile 'AC_PASSWORD' found"
            else
                echo -e "${YELLOW}  ⚠ No notarization credentials found${NC}"
                echo "  Set up credentials with:"
                echo "    xcrun notarytool store-credentials 'AC_PASSWORD' --apple-id EMAIL --team-id $TEAM_ID"
            fi
        fi
    fi
}

# Create entitlements file
create_entitlements() {
    cat > "$ENTITLEMENTS_FILE" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <!-- Allow JIT compilation for JavaScript engine -->
    <key>com.apple.security.cs.allow-jit</key>
    <true/>
    <!-- Allow unsigned executable memory (needed for rquickjs) -->
    <key>com.apple.security.cs.allow-unsigned-executable-memory</key>
    <true/>
    <!-- Allow dyld environment variables for debugging -->
    <key>com.apple.security.cs.disable-library-validation</key>
    <true/>
    <!-- Accessibility access -->
    <key>com.apple.security.automation.apple-events</key>
    <true/>
</dict>
</plist>
EOF
    echo "  Created entitlements file: $ENTITLEMENTS_FILE"
}

# Sign a single binary
sign_binary() {
    local binary="$1"
    local name=$(basename "$binary")

    if [ ! -f "$binary" ]; then
        echo -e "${YELLOW}  Skipping $name (not found)${NC}"
        return
    fi

    echo "  Signing $name..."
    codesign --force --options runtime \
        --sign "$SIGNING_IDENTITY" \
        --entitlements "$ENTITLEMENTS_FILE" \
        --timestamp \
        "$binary"

    # Verify signature
    if codesign --verify --verbose=2 "$binary" 2>&1 | grep -q "valid on disk"; then
        echo "    ✓ $name signed and verified"
    else
        echo -e "${RED}    ✗ $name signature verification failed${NC}"
        return 1
    fi
}

# Sign release binaries
sign_binaries() {
    echo -e "${BLUE}Signing release binaries...${NC}"

    local release_dir="$PROJECT_ROOT/target/release"

    if [ ! -d "$release_dir" ]; then
        echo -e "${RED}ERROR: Release directory not found: $release_dir${NC}"
        echo "Run 'just build' first"
        exit 1
    fi

    # Main binaries to sign
    local binaries=(
        "HookAnchorCommand"
        "HookAnchorPopupServer"
        "HookAnchorPopup"
        "HookAnchorDialog"
        "HookAnchorInstaller"
        "HookAnchorHistoryViewer"
        "HookAnchorSupervisor"
    )

    for binary in "${binaries[@]}"; do
        sign_binary "$release_dir/$binary"
    done

    echo -e "${GREEN}✓ All binaries signed${NC}"
}

# Sign app bundle
sign_app() {
    local app_path="$1"

    if [ ! -d "$app_path" ]; then
        echo -e "${RED}ERROR: App bundle not found: $app_path${NC}"
        exit 1
    fi

    echo -e "${BLUE}Signing app bundle: $app_path${NC}"

    # Sign embedded frameworks/helpers first (deep signing)
    # Sign URLHandler.app if present
    if [ -d "$app_path/Contents/Resources/URLHandler.app" ]; then
        echo "  Signing embedded URLHandler.app..."
        codesign --force --options runtime \
            --sign "$SIGNING_IDENTITY" \
            --entitlements "$ENTITLEMENTS_FILE" \
            --timestamp \
            "$app_path/Contents/Resources/URLHandler.app"
    fi

    # Sign all binaries in MacOS folder
    echo "  Signing binaries in Contents/MacOS..."
    for binary in "$app_path/Contents/MacOS"/*; do
        if [ -f "$binary" ] && ! [ -L "$binary" ]; then
            local name=$(basename "$binary")
            echo "    Signing $name..."
            codesign --force --options runtime \
                --sign "$SIGNING_IDENTITY" \
                --entitlements "$ENTITLEMENTS_FILE" \
                --timestamp \
                "$binary"
        fi
    done

    # Sign the main app bundle
    echo "  Signing main app bundle..."
    codesign --force --options runtime \
        --sign "$SIGNING_IDENTITY" \
        --entitlements "$ENTITLEMENTS_FILE" \
        --timestamp \
        "$app_path"

    # Verify
    echo "  Verifying signature..."
    if codesign --verify --deep --verbose=2 "$app_path" 2>&1 | grep -q "valid on disk"; then
        echo -e "${GREEN}✓ App bundle signed and verified${NC}"
    else
        echo -e "${RED}✗ Signature verification failed${NC}"
        codesign --verify --deep --verbose=2 "$app_path"
        exit 1
    fi
}

# Notarize with Apple
notarize() {
    local target="$1"
    local target_name=$(basename "$target")

    echo -e "${BLUE}Notarizing: $target_name${NC}"

    # Create zip for notarization
    local zip_path="/tmp/hookanchor_notarize.zip"
    rm -f "$zip_path"

    if [[ "$target" == *.app ]]; then
        ditto -c -k --keepParent "$target" "$zip_path"
    elif [[ "$target" == *.dmg ]]; then
        cp "$target" "$zip_path"
        zip_path="$target"  # DMGs can be submitted directly
    else
        echo -e "${RED}ERROR: Unknown target type for notarization${NC}"
        exit 1
    fi

    echo "  Submitting for notarization..."

    # Try API key first, then keychain profile
    local submit_output
    if [ -n "$APPLE_API_KEY_ID" ] && [ -n "$APPLE_API_ISSUER" ]; then
        submit_output=$(xcrun notarytool submit "$zip_path" \
            --key ~/.private_keys/AuthKey_${APPLE_API_KEY_ID}.p8 \
            --key-id "$APPLE_API_KEY_ID" \
            --issuer "$APPLE_API_ISSUER" \
            --wait 2>&1)
    else
        submit_output=$(xcrun notarytool submit "$zip_path" \
            --keychain-profile "AC_PASSWORD" \
            --wait 2>&1)
    fi

    echo "$submit_output"

    if echo "$submit_output" | grep -q "status: Accepted"; then
        echo -e "${GREEN}✓ Notarization successful${NC}"

        # Staple the ticket
        echo "  Stapling notarization ticket..."
        if [[ "$target" == *.app ]]; then
            xcrun stapler staple "$target"
        elif [[ "$target" == *.dmg ]]; then
            xcrun stapler staple "$target"
        fi
        echo -e "${GREEN}✓ Ticket stapled${NC}"
    else
        echo -e "${RED}✗ Notarization failed${NC}"

        # Get submission ID for log retrieval
        local submission_id=$(echo "$submit_output" | grep -o 'id: [a-f0-9-]*' | head -1 | cut -d' ' -f2)
        if [ -n "$submission_id" ]; then
            echo "  Fetching notarization log..."
            if [ -n "$APPLE_API_KEY_ID" ]; then
                xcrun notarytool log "$submission_id" \
                    --key ~/.private_keys/AuthKey_${APPLE_API_KEY_ID}.p8 \
                    --key-id "$APPLE_API_KEY_ID" \
                    --issuer "$APPLE_API_ISSUER"
            else
                xcrun notarytool log "$submission_id" --keychain-profile "AC_PASSWORD"
            fi
        fi
        exit 1
    fi

    # Cleanup
    if [[ "$target" == *.app ]]; then
        rm -f "$zip_path"
    fi
}

# Main
SKIP_NOTARIZE="false"
VERBOSE="false"
TARGET=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-notarize)
            SKIP_NOTARIZE="true"
            shift
            ;;
        --verbose)
            VERBOSE="true"
            shift
            ;;
        -h|--help)
            usage
            ;;
        *)
            TARGET="$1"
            shift
            ;;
    esac
done

if [ -z "$TARGET" ]; then
    usage
fi

check_prereqs

case $TARGET in
    binaries)
        sign_binaries
        ;;
    app)
        # Default app location
        APP_PATH="$PROJECT_ROOT/dist/HookAnchor.app"
        if [ ! -d "$APP_PATH" ]; then
            APP_PATH="$PROJECT_ROOT/temp_build/HookAnchor.app"
        fi
        if [ ! -d "$APP_PATH" ]; then
            echo -e "${RED}ERROR: HookAnchor.app not found${NC}"
            echo "Run 'just build-dist' first, or specify app path"
            exit 1
        fi
        sign_app "$APP_PATH"
        if [ "$SKIP_NOTARIZE" != "true" ]; then
            notarize "$APP_PATH"
        fi
        ;;
    dmg)
        # Find latest DMG
        VERSION=$(grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')
        DMG_PATH="$PROJECT_ROOT/versions/$VERSION/HookAnchor-$VERSION.dmg"
        if [ ! -f "$DMG_PATH" ]; then
            DMG_PATH="$PROJECT_ROOT/dist/HookAnchor-$VERSION.dmg"
        fi
        if [ ! -f "$DMG_PATH" ]; then
            echo -e "${RED}ERROR: DMG not found${NC}"
            echo "Run 'just build-dist' first"
            exit 1
        fi
        # Sign the DMG
        echo -e "${BLUE}Signing DMG...${NC}"
        codesign --force --sign "$SIGNING_IDENTITY" --timestamp "$DMG_PATH"
        if [ "$SKIP_NOTARIZE" != "true" ]; then
            notarize "$DMG_PATH"
        fi
        ;;
    *)
        echo -e "${RED}Unknown target: $TARGET${NC}"
        usage
        ;;
esac

echo ""
echo -e "${GREEN}Done!${NC}"
