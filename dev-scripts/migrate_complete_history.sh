#!/usr/bin/env bash
# Migrate complete history: documentation + configurations
#
# This script:
# 1. Migrates documentation commits from kmr repo (oldest first)
# 2. Migrates ALL config commits from HookAnchorApp repo (even if sanitization produces no changes)
# 3. Creates versions/X.Y.Z/ folders for release snapshots
# 4. Copies CHANGELOG.md to public repo
#
# Result: Public repo has complete development history

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
KMR_REPO="/Users/oblinger/ob/kmr"
TEMP_DIR="/tmp/hookanchor_complete_migration"

echo "=================================================="
echo -e "${BLUE}📜 Complete History Migration${NC}"
echo -e "${BLUE}   Documentation + Configuration${NC}"
echo "=================================================="
echo ""
echo "This will:"
echo "  1. Migrate ~10 documentation commits from kmr repo"
echo "  2. Migrate ALL 53 config commits (complete history)"
echo "  3. Create versions/ folder with release snapshots"
echo "  4. Copy CHANGELOG.md to public repo"
echo ""
echo -e "${YELLOW}⚠️  WARNING: This will rewrite public repo history!${NC}"
echo ""
echo "Source repos:"
echo "  Docs:   $KMR_REPO"
echo "  Config: $PRIVATE_REPO"
echo "Public repo: $PUBLIC_REPO"
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

# Backup and reset public repo
echo ""
echo -e "${YELLOW}🔄 Preparing public repo...${NC}"
cd "$PUBLIC_REPO"

# Save current state
git branch -f backup-before-complete-migration HEAD 2>/dev/null || true

# Reset to initial commit and create migration branch
FIRST_COMMIT=$(git rev-list --max-parents=0 HEAD)
git checkout -f $FIRST_COMMIT
git checkout -b complete-migration

# Remove any existing directories
rm -rf config versions docs

echo -e "${GREEN}✓ Public repo reset to initial state${NC}"
echo ""

# ============================================================================
# PART 1: MIGRATE DOCUMENTATION HISTORY
# ============================================================================

echo "=================================================="
echo -e "${BLUE}📚 PART 1: Documentation History${NC}"
echo "=================================================="
echo ""

cd "$KMR_REPO"

# Get documentation commit list (oldest first)
echo -e "${BLUE}Finding documentation commits...${NC}"
git log --reverse --format="%H|%an|%ae|%ai|%s" -- \
    "prj/binproj/Hook Anchor/website-docs" \
    "prj/binproj/Hook Anchor/docs/User Docs" \
    > "$TEMP_DIR/doc_commits.txt"

DOC_COMMIT_COUNT=$(wc -l < "$TEMP_DIR/doc_commits.txt" | tr -d ' ')
echo -e "${GREEN}Found $DOC_COMMIT_COUNT documentation commits${NC}"
echo ""

if [ $DOC_COMMIT_COUNT -gt 0 ]; then
    DOC_COUNTER=0
    while IFS='|' read -r COMMIT_HASH AUTHOR_NAME AUTHOR_EMAIL COMMIT_DATE COMMIT_MSG; do
        DOC_COUNTER=$((DOC_COUNTER + 1))
        echo "=================================================="
        echo -e "${BLUE}[DOC $DOC_COUNTER/$DOC_COMMIT_COUNT] ${COMMIT_HASH:0:8}${NC}"
        echo "=================================================="
        echo "  Date: $COMMIT_DATE"
        echo "  Message: $COMMIT_MSG"

        cd "$KMR_REPO"

        # Extract documentation at this commit
        mkdir -p "$TEMP_DIR/docs"
        rm -rf "$TEMP_DIR/docs"/*

        # Try website-docs first (newer location)
        if git show "$COMMIT_HASH:prj/binproj/Hook Anchor/website-docs/" > /dev/null 2>&1; then
            git archive "$COMMIT_HASH" "prj/binproj/Hook Anchor/website-docs/" | tar -x -C "$TEMP_DIR/docs" --strip-components=4 2>/dev/null || true
            echo "  ✓ Found website-docs/"
        # Fall back to old User Docs location
        elif git show "$COMMIT_HASH:prj/binproj/Hook Anchor/docs/User Docs/" > /dev/null 2>&1; then
            git archive "$COMMIT_HASH" "prj/binproj/Hook Anchor/docs/User Docs/" | tar -x -C "$TEMP_DIR/docs" --strip-components=5 2>/dev/null || true
            echo "  ✓ Found docs/User Docs/"
        else
            echo -e "${YELLOW}  ⚠ No docs found, skipping${NC}"
            echo ""
            continue
        fi

        # Check if we got files
        if [ ! "$(ls -A $TEMP_DIR/docs 2>/dev/null)" ]; then
            echo -e "${YELLOW}  ⚠ No doc files extracted, skipping${NC}"
            echo ""
            continue
        fi

        # Sanitize documentation (remove personal paths)
        echo "  🔒 Sanitizing documentation..."
        find "$TEMP_DIR/docs" -name "*.md" -type f -exec sed -i '' \
            -e 's|/Users/oblinger/ob/kmr|~/Documents/Notes|g' \
            -e 's|~/ob/kmr|~/Documents/Notes|g' \
            -e 's|~/ob/prj|~/Projects|g' \
            -e 's|/Users/oblinger/ob/prj|~/Projects|g' \
            -e 's|~/ob/proj|~/Projects|g' \
            -e 's|/Users/oblinger/ob/proj|~/Projects|g' \
            {} \;

        # Copy to public repo
        cd "$PUBLIC_REPO"
        mkdir -p docs
        cp -r "$TEMP_DIR/docs"/* docs/ 2>/dev/null || true

        git add docs/

        # Commit with original metadata (always commit, even if no changes)
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

    done < "$TEMP_DIR/doc_commits.txt"
else
    echo -e "${YELLOW}No documentation commits found${NC}"
    echo ""
fi

# ============================================================================
# PART 2: MIGRATE CONFIGURATION HISTORY
# ============================================================================

echo "=================================================="
echo -e "${BLUE}📝 PART 2: Configuration History${NC}"
echo "=================================================="
echo ""

cd "$PRIVATE_REPO"

# Get ALL commits that touched config files (oldest first)
echo -e "${BLUE}Finding configuration commits...${NC}"
git log --reverse --format="%H|%an|%ae|%ai|%s" -- config/config.yaml config/config.js > "$TEMP_DIR/config_commits.txt"

CONFIG_COMMIT_COUNT=$(wc -l < "$TEMP_DIR/config_commits.txt" | tr -d ' ')
echo -e "${GREEN}Found $CONFIG_COMMIT_COUNT configuration commits${NC}"
echo ""

# Process each commit
CONFIG_COUNTER=0
while IFS='|' read -r COMMIT_HASH AUTHOR_NAME AUTHOR_EMAIL COMMIT_DATE COMMIT_MSG; do
    CONFIG_COUNTER=$((CONFIG_COUNTER + 1))
    echo "=================================================="
    echo -e "${BLUE}[CFG $CONFIG_COUNTER/$CONFIG_COMMIT_COUNT] ${COMMIT_HASH:0:8}${NC}"
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
        echo -e "${YELLOW}  ⚠ No config files found${NC}"
        # Still create empty commit to preserve history
        cd "$PUBLIC_REPO"
        echo "  📝 Creating empty commit (preserves history)..."

        GIT_AUTHOR_NAME="$AUTHOR_NAME" \
        GIT_AUTHOR_EMAIL="$AUTHOR_EMAIL" \
        GIT_AUTHOR_DATE="$COMMIT_DATE" \
        GIT_COMMITTER_NAME="$AUTHOR_NAME" \
        GIT_COMMITTER_EMAIL="$AUTHOR_EMAIL" \
        GIT_COMMITTER_DATE="$COMMIT_DATE" \
        git commit --allow-empty -m "$COMMIT_MSG"

        echo -e "${GREEN}  ✓ Empty commit created${NC}"
        echo ""
        continue
    fi

    # Check version in this commit to see if it's a release
    VERSION=""
    IS_RELEASE=0
    if git show "$COMMIT_HASH:Cargo.toml" > "$TEMP_DIR/Cargo.toml" 2>/dev/null; then
        VERSION=$(grep '^version = ' "$TEMP_DIR/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')

        # Check if this version is in our release list
        if grep -q "^$VERSION$" "$TEMP_DIR/release_versions.txt"; then
            echo "  🎯 Release version: $VERSION"
            IS_RELEASE=1
        fi
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

    # ALWAYS commit (even if no changes) to preserve history
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

done < "$TEMP_DIR/config_commits.txt"

# ============================================================================
# PART 3: ADD METADATA FILES
# ============================================================================

echo ""
echo "=================================================="
echo -e "${BLUE}📚 PART 3: Adding Metadata Files${NC}"
echo "=================================================="
echo ""

# Create versions README
echo -e "${BLUE}Creating versions/README.md...${NC}"
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

# Copy CHANGELOG
echo -e "${BLUE}Copying CHANGELOG.md...${NC}"
cp "$PRIVATE_REPO/CHANGELOG.md" CHANGELOG.md
git add CHANGELOG.md
git commit -m "Add CHANGELOG from private repository"
echo -e "${GREEN}✓ Copied CHANGELOG.md${NC}"

# Create config README
echo -e "${BLUE}Creating config/README.md...${NC}"
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
echo -e "${GREEN}✓ Created config/README.md${NC}"

echo ""
echo "=================================================="
echo -e "${GREEN}✅ Complete Migration Finished!${NC}"
echo "=================================================="
echo ""
echo "Statistics:"
echo "  - Documentation commits: $DOC_COUNTER"
echo "  - Configuration commits: $CONFIG_COUNTER"
echo "  - Total commits: $(git rev-list --count complete-migration ^$FIRST_COMMIT)"
echo "  - Version snapshots: $(ls -1 versions 2>/dev/null | grep -E '^[0-9]' | wc -l | tr -d ' ')"
echo ""
echo "The migration is on branch 'complete-migration'"
echo "Original state backed up to 'backup-before-complete-migration'"
echo ""
echo "Review the results:"
echo "  git log complete-migration --oneline"
echo "  ls -la versions/"
echo "  ls -la docs/"
echo ""
echo "If satisfied, apply the changes:"
echo "  git checkout main"
echo "  git reset --hard complete-migration"
echo "  git push --force origin main"
echo ""
echo "Clean up branches:"
echo "  git branch -D complete-migration backup-before-complete-migration"
echo ""
echo -e "${YELLOW}⚠️  Remember: You'll need to force push to update the remote${NC}"
echo ""
