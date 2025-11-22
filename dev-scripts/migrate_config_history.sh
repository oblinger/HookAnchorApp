#!/usr/bin/env bash
# Migrate complete sanitized config history from private repo to public repo
#
# This script:
# 1. Creates git commits for all historical config changes (preserves commit history)
# 2. Creates versions/X.Y.Z/ folders for release snapshots (easy browsing)
# 3. Copies CHANGELOG.md to public repo
#
# Result: Public repo has full git history AND versions folder structure

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PRIVATE_REPO="/Users/oblinger/ob/proj/HookAnchor/HookAnchorApp"
PUBLIC_REPO="/Users/oblinger/ob/proj/HookAnchor/HookAnchor"
TEMP_DIR="/tmp/hookanchor_config_migration"

echo "=================================================="
echo -e "${BLUE}📜 Complete Config History Migration${NC}"
echo "=================================================="
echo ""
echo "This will:"
echo "  1. Create ~53 git commits in public repo (full history)"
echo "  2. Create versions/ folder with release snapshots"
echo "  3. Copy CHANGELOG.md to public repo"
echo ""
echo -e "${YELLOW}⚠️  WARNING: This will rewrite public repo history!${NC}"
echo ""
echo "Private repo: $PRIVATE_REPO"
echo "Public repo:  $PUBLIC_REPO"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Clean up temp directory
rm -rf "$TEMP_DIR"
mkdir -p "$TEMP_DIR"

# Read versions from CHANGELOG for version folder creation
echo ""
echo -e "${BLUE}📋 Reading release versions from CHANGELOG...${NC}"
cd "$PRIVATE_REPO"
grep "^## \[" CHANGELOG.md | sed 's/^## \[\([^]]*\)\].*/\1/' > "$TEMP_DIR/release_versions.txt"
RELEASE_COUNT=$(wc -l < "$TEMP_DIR/release_versions.txt" | tr -d ' ')
echo -e "${GREEN}Found $RELEASE_COUNT release versions${NC}"

# Get list of ALL commits that touched config files (oldest first)
echo ""
echo -e "${BLUE}📋 Finding all commits that modified config files...${NC}"

# Get commit list: hash|author name|author email|timestamp|subject
git log --reverse --format="%H|%an|%ae|%ai|%s" -- config/config.yaml config/config.js > "$TEMP_DIR/commits.txt"

COMMIT_COUNT=$(wc -l < "$TEMP_DIR/commits.txt" | tr -d ' ')
echo -e "${GREEN}Found $COMMIT_COUNT commits to migrate${NC}"
echo ""

# Backup and reset public repo
echo -e "${YELLOW}🔄 Preparing public repo...${NC}"
cd "$PUBLIC_REPO"

# Save current state
git branch -f backup-before-migration HEAD 2>/dev/null || true

# Reset to initial commit and create migration branch
FIRST_COMMIT=$(git rev-list --max-parents=0 HEAD)
git checkout -f $FIRST_COMMIT
git checkout -b temp-migration

# Remove any existing config directory
rm -rf config versions

echo -e "${GREEN}✓ Public repo reset to initial state${NC}"
echo ""

# Process each commit
COUNTER=0
while IFS='|' read -r COMMIT_HASH AUTHOR_NAME AUTHOR_EMAIL COMMIT_DATE COMMIT_MSG; do
    COUNTER=$((COUNTER + 1))
    echo "=================================================="
    echo -e "${BLUE}[$COUNTER/$COMMIT_COUNT] ${COMMIT_HASH:0:8}${NC}"
    echo "=================================================="
    echo "  Date: $COMMIT_DATE"
    echo "  Message: $COMMIT_MSG"

    cd "$PRIVATE_REPO"

    # Extract config files at this commit
    mkdir -p "$TEMP_DIR/configs"
    rm -f "$TEMP_DIR/configs"/*

    HAS_YAML=0
    HAS_JS=0

    # Try to extract YAML config
    if git show "$COMMIT_HASH:config/config.yaml" > "$TEMP_DIR/configs/config.yaml" 2>/dev/null; then
        echo "  ✓ Found config.yaml"
        HAS_YAML=1
    fi

    # Try to extract JS config
    if git show "$COMMIT_HASH:config/config.js" > "$TEMP_DIR/configs/config.js" 2>/dev/null; then
        echo "  ✓ Found config.js"
        HAS_JS=1
    fi

    # Check if we got any files
    if [ $HAS_YAML -eq 0 ] && [ $HAS_JS -eq 0 ]; then
        echo -e "${YELLOW}  ⚠ No config files found, skipping${NC}"
        echo ""
        continue
    fi

    # Check version in this commit to see if it's a release
    VERSION=""
    if git show "$COMMIT_HASH:Cargo.toml" > "$TEMP_DIR/Cargo.toml" 2>/dev/null; then
        VERSION=$(grep '^version = ' "$TEMP_DIR/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')

        # Check if this version is in our release list
        if grep -q "^$VERSION$" "$TEMP_DIR/release_versions.txt"; then
            echo "  🎯 Release version: $VERSION"
            IS_RELEASE=1
        else
            IS_RELEASE=0
        fi
    else
        IS_RELEASE=0
    fi

    # Sanitize the extracted configs
    echo "  🔒 Sanitizing configs..."

    export OUTPUT_YAML="$TEMP_DIR/sanitized/default_config.yaml"
    export OUTPUT_JS="$TEMP_DIR/sanitized/default_config.js"
    mkdir -p "$TEMP_DIR/sanitized"
    rm -f "$TEMP_DIR/sanitized"/*

    # Copy to temp location for sanitization
    if [ $HAS_YAML -eq 1 ]; then
        cp "$TEMP_DIR/configs/config.yaml" "$TEMP_DIR/original_config.yaml"
    fi
    if [ $HAS_JS -eq 1 ]; then
        cp "$TEMP_DIR/configs/config.js" "$TEMP_DIR/original_config.js"
    fi

    # Run sanitization
    if "$SCRIPT_DIR/build/sanitize_config.sh" > "$TEMP_DIR/sanitize.log" 2>&1; then
        echo "  ✓ Sanitization successful"
    else
        echo -e "${YELLOW}  ⚠ Sanitization warnings (may be old format)${NC}"
    fi

    # Copy sanitized files to public repo config/
    cd "$PUBLIC_REPO"
    mkdir -p config

    if [ -f "$OUTPUT_YAML" ]; then
        cp "$OUTPUT_YAML" config/default_config.yaml
        git add config/default_config.yaml
    fi

    if [ -f "$OUTPUT_JS" ]; then
        cp "$OUTPUT_JS" config/default_config.js
        git add config/default_config.js
    fi

    # If this is a release version, ALSO create versions/X.Y.Z/
    if [ $IS_RELEASE -eq 1 ] && [ -n "$VERSION" ]; then
        echo "  📦 Creating version snapshot: versions/$VERSION/"
        mkdir -p "versions/$VERSION"

        if [ -f "$OUTPUT_YAML" ]; then
            cp "$OUTPUT_YAML" "versions/$VERSION/default_config.yaml"
            git add "versions/$VERSION/default_config.yaml"
        fi

        if [ -f "$OUTPUT_JS" ]; then
            cp "$OUTPUT_JS" "versions/$VERSION/default_config.js"
            git add "versions/$VERSION/default_config.js"
        fi
    fi

    # Check if there are changes to commit
    if git diff --cached --quiet; then
        echo -e "${YELLOW}  ⚠ No changes, skipping commit${NC}"
        echo ""
        continue
    fi

    # Commit with original metadata
    echo "  📝 Creating git commit..."

    GIT_AUTHOR_NAME="$AUTHOR_NAME" \
    GIT_AUTHOR_EMAIL="$AUTHOR_EMAIL" \
    GIT_AUTHOR_DATE="$COMMIT_DATE" \
    GIT_COMMITTER_NAME="$AUTHOR_NAME" \
    GIT_COMMITTER_EMAIL="$AUTHOR_EMAIL" \
    GIT_COMMITTER_DATE="$COMMIT_DATE" \
    git commit -m "$COMMIT_MSG" --allow-empty

    echo -e "${GREEN}  ✓ Committed${NC}"
    echo ""

done < "$TEMP_DIR/commits.txt"

echo ""
echo "=================================================="
echo -e "${BLUE}📚 Creating versions/README.md${NC}"
echo "=================================================="

# Create versions README
mkdir -p versions
cat > versions/README.md << 'EOF'
# HookAnchor Configuration Version History

This directory contains snapshots of the default configuration files for each released version of HookAnchor.

## Structure

Each version is in its own folder (e.g., `0.20.1/`) containing:
- `default_config.yaml` - YAML configuration for that version
- `default_config.js` - JavaScript configuration for that version

## Using Historical Configs

These are useful for:
- **Upgrading**: Compare your config with newer defaults to see what changed
- **Downgrading**: Reference older config format if needed
- **Learning**: See how the configuration evolved over time

## Latest Version

The latest configuration is always in the `../config/` directory.

## Full History

This repository has complete git history. Use `git log -- config/` to see all changes.

## Version History

See [../CHANGELOG.md](../CHANGELOG.md) for detailed changes in each version.
EOF

git add versions/README.md
git commit -m "Add versions/ folder documentation"

echo -e "${GREEN}✓ Created versions/README.md${NC}"
echo ""

# Copy CHANGELOG from private repo
echo "=================================================="
echo -e "${BLUE}📄 Copying CHANGELOG.md${NC}"
echo "=================================================="

cp "$PRIVATE_REPO/CHANGELOG.md" CHANGELOG.md
git add CHANGELOG.md
git commit -m "Add CHANGELOG from private repository"

echo -e "${GREEN}✓ Copied CHANGELOG.md${NC}"
echo ""

# Create config README
mkdir -p config
cat > config/README.md << 'EOF'
# HookAnchor Configuration Files

This directory contains default configuration files for HookAnchor.

## Installation

1. Install HookAnchor from the DMG distribution
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

## Historical Versions

See [../versions/](../versions/) for configuration files from previous releases.

## Documentation

See the [docs/](../docs/) directory for complete documentation:
- [User Guide](../docs/USER_GUIDE.md)
- [Configuration Reference](../docs/CONFIG_REFERENCE.md)
- [Templates and Scripting](../docs/TEMPLATES_AND_SCRIPTING.md)
EOF

git add config/README.md
git commit -m "Add config directory documentation"

echo ""
echo "=================================================="
echo -e "${GREEN}✅ Migration Complete!${NC}"
echo "=================================================="
echo ""
echo "Statistics:"
echo "  - Git commits created: $(git rev-list --count temp-migration ^$FIRST_COMMIT)"
echo "  - Version snapshots: $(ls -1 versions | grep -E '^[0-9]' | wc -l | tr -d ' ')"
echo ""
echo "The migration is on branch 'temp-migration'"
echo "Original state backed up to branch 'backup-before-migration'"
echo ""
echo "Review the results:"
echo "  git log temp-migration --oneline"
echo "  ls -la versions/"
echo ""
echo "If satisfied, apply the changes:"
echo "  git checkout main"
echo "  git reset --hard temp-migration"
echo "  git push --force origin main"
echo ""
echo "Clean up branches:"
echo "  git branch -D temp-migration backup-before-migration"
echo ""
echo -e "${YELLOW}⚠️  Remember: You'll need to force push to update the remote${NC}"
echo ""
