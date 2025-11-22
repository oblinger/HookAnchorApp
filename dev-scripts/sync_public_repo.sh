#!/usr/bin/env bash
# Sync documentation and sanitized configs to public HookAnchor repository

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PUBLIC_REPO="/Users/oblinger/ob/proj/HookAnchor/HookAnchor"
DOCS_SOURCE="/Users/oblinger/ob/kmr/prj/binproj/Hook Anchor/website-docs"

echo "=================================================="
echo -e "${BLUE}📤 Syncing to Public HookAnchor Repository${NC}"
echo "=================================================="
echo ""

# Verify public repo exists
if [ ! -d "$PUBLIC_REPO/.git" ]; then
    echo -e "${RED}❌ Public repo not found at: $PUBLIC_REPO${NC}"
    echo "Please create the repository first"
    exit 1
fi

# Get version from Cargo.toml
VERSION=$(grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "${BLUE}Version: $VERSION${NC}"
echo ""

# 1. Sanitize configs
echo -e "${YELLOW}🔒 Step 1: Sanitizing configuration files...${NC}"
cd "$PROJECT_ROOT"

# Set output paths for sanitization
export OUTPUT_YAML="$PUBLIC_REPO/config/default_config.yaml"
export OUTPUT_JS="$PUBLIC_REPO/config/default_config.js"

# Create config directory if it doesn't exist
mkdir -p "$PUBLIC_REPO/config"

# Run sanitization script
"$SCRIPT_DIR/build/sanitize_config.sh"

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Config sanitization failed!${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Configs sanitized${NC}"
echo ""

# 2. Sync documentation
echo -e "${YELLOW}📚 Step 2: Syncing documentation...${NC}"

if [ ! -d "$DOCS_SOURCE" ]; then
    echo -e "${RED}❌ Documentation source not found: $DOCS_SOURCE${NC}"
    exit 1
fi

# Create docs directory in public repo
mkdir -p "$PUBLIC_REPO/docs"

# Copy markdown files (not HTML - those are generated)
echo "  Copying markdown documentation..."
rsync -av --delete \
    --include='*.md' \
    --include='*.css' \
    --include='*.png' \
    --include='*.jpg' \
    --exclude='*.html' \
    --exclude='.DS_Store' \
    "$DOCS_SOURCE/" "$PUBLIC_REPO/docs/"

echo -e "${GREEN}✓ Documentation synced${NC}"
echo ""

# 3. Create/update config README
echo -e "${YELLOW}📝 Step 3: Creating config README...${NC}"

cat > "$PUBLIC_REPO/config/README.md" << 'EOF'
# HookAnchor Configuration Files

This directory contains default configuration files for HookAnchor.

## Installation

1. Install Hook Anchor from the DMG distribution
2. The installer will create `~/.config/hookanchor/` with these defaults
3. Customize the configuration files for your workflow

## Files

### default_config.yaml

Main configuration file defining:
- Keyboard bindings
- Popup settings
- File scanning roots
- Actions and templates
- Integration settings (Obsidian, Notion, etc.)

### default_config.js

JavaScript configuration for advanced customization:
- Custom action handlers
- Template variable functions
- Integration with external services

## Customization

After installation, your personal configs are at:
- `~/.config/hookanchor/config.yaml`
- `~/.config/hookanchor/config.js`

Edit these files to customize HookAnchor for your needs.

## Environment Variables

You can use environment variables in your config files:

```yaml
integrations:
  notion:
    api_key: "${HOOKANCHOR_NOTION_API_KEY}"  # Reads from environment
```

## Documentation

See [../docs/](../docs/) for complete documentation:
- [User Guide](../docs/USER_GUIDE.md)
- [Configuration Reference](../docs/CONFIG_REFERENCE.md)
- [Templates and Scripting](../docs/TEMPLATES_AND_SCRIPTING.md)
EOF

echo -e "${GREEN}✓ Config README created${NC}"
echo ""

# 4. Verify no sensitive data
echo -e "${YELLOW}🔍 Step 4: Final security verification...${NC}"

ISSUES=0

# Check all files in public repo
for file in "$PUBLIC_REPO/config"/*.{yaml,js} "$PUBLIC_REPO/docs"/*.md; do
    if [ -f "$file" ]; then
        # Check for API keys
        if grep -q "ntn_[a-zA-Z0-9]" "$file" 2>/dev/null; then
            echo -e "${RED}  ❌ Found Notion API key in: $(basename $file)${NC}"
            ISSUES=$((ISSUES + 1))
        fi

        # Check for personal paths
        if grep -q "/Users/oblinger/ob/kmr" "$file" 2>/dev/null; then
            echo -e "${RED}  ❌ Found personal path in: $(basename $file)${NC}"
            ISSUES=$((ISSUES + 1))
        fi

        # Check for hostname
        if grep -q "Daniels-MacBook-Pro" "$file" 2>/dev/null; then
            echo -e "${RED}  ❌ Found hostname in: $(basename $file)${NC}"
            ISSUES=$((ISSUES + 1))
        fi
    fi
done

if [ $ISSUES -eq 0 ]; then
    echo -e "${GREEN}✓ No sensitive data found${NC}"
else
    echo -e "${RED}❌ Found $ISSUES security issues - ABORTING${NC}"
    echo "Please review and fix security issues before pushing"
    exit 1
fi

echo ""

# 5. Git operations
echo -e "${YELLOW}📦 Step 5: Committing to public repository...${NC}"

cd "$PUBLIC_REPO"

# Add all changes
git add -A

# Check if there are changes to commit
if git diff --cached --quiet; then
    echo -e "${YELLOW}  No changes to commit${NC}"
else
    # Commit with version info
    git commit -m "Update documentation and configs for v$VERSION"
    echo -e "${GREEN}✓ Changes committed${NC}"
fi

echo ""

# 6. Push to GitHub
echo -e "${YELLOW}🚀 Step 6: Pushing to GitHub...${NC}"

git push origin main

echo -e "${GREEN}✓ Pushed to GitHub${NC}"
echo ""

# Summary
echo "=================================================="
echo -e "${GREEN}✅ Public Repository Sync Complete!${NC}"
echo "=================================================="
echo ""
echo "Synced content:"
echo "  - Documentation: $(ls -1 $PUBLIC_REPO/docs/*.md | wc -l | tr -d ' ') markdown files"
echo "  - Configs: default_config.yaml, default_config.js"
echo "  - Version: $VERSION"
echo ""
echo "Public repository: https://github.com/oblinger/HookAnchor"
echo ""
