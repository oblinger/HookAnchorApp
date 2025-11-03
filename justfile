# HookAnchor Development Commands
# Use `just <command>` to run development tasks

# IMPORTANT: Build verification system
# When you run 'just build', the build.rs script embeds build metadata that gets
# verified at runtime to ensure you're running the correct code. This prevents
# accidentally running stale binaries or binaries built outside the official build process.

# Default recipe - show available commands
default:
    @just --list

# === BUILD COMMANDS ===

# Build Swift supervisor (only if source changed or binary missing)
build-supervisor:
    #!/usr/bin/env bash
    SWIFT_SRC="swift/Supervisor/HookAnchor.swift"
    SWIFT_BIN="target/release/HookAnchorSupervisor"

    # Build if binary missing or source is newer than binary
    if [ ! -f "$SWIFT_BIN" ] || [ "$SWIFT_SRC" -nt "$SWIFT_BIN" ]; then
        echo "🔨 Building Swift supervisor (source changed or binary missing)..."
        ./swift/build_supervisor.sh
    else
        echo "✅ Swift supervisor up to date"
    fi

# Build all release binaries (with build verification)
build: build-supervisor
    @echo "🔨 Building Rust components with Just (tracked build)..."
    @date
    JUST=1 cargo build --release
    @echo "📎 Creating symlinks..."
    @ln -sf HookAnchorCommand target/release/ha
    @ln -sf HookAnchorPopupServer target/release/popup_server
    @ln -sf HookAnchorPopup target/release/popup
    @echo "✅ Build complete - binaries embed build metadata for verification"

# Build for both architectures (universal binary)
build-universal:
    @echo "🔨 Building universal binaries..."
    rustup target add x86_64-apple-darwin || true
    rustup target add aarch64-apple-darwin || true
    cargo build --release --target aarch64-apple-darwin
    cargo build --release --target x86_64-apple-darwin
    @echo "✅ Universal builds complete"

# Build complete distribution DMG
build-dist:
    @echo "🚀 Building distribution package..."
    ./dev-scripts/build/build_distribution.sh

# Create Uninstaller.app (standalone)
create-uninstaller:
    @echo "📦 Creating Uninstaller.app..."
    ./dev-scripts/build/generate_uninstaller_app.sh

# Clean build artifacts
clean:
    cargo clean
    rm -rf temp_build/
    rm -rf dist/
    @echo "🧹 Cleaned build artifacts"

# === DEVELOPMENT COMMANDS ===

# Set up development environment (complete app bundle with URL handler)
setup:
    @echo "🛠️ Setting up development environment..."
    ./dev-scripts/setup/setup_dev_app.sh

# Alias for setup (backward compatibility)
setup-dev-app: setup

# Create symlinks for development (archived - symlinks handled by setup-dev-app)
# setup-symlinks: [removed - script archived]

# Deploy built app to /Applications for testing
deploy:
    @echo "🚀 Deploying to /Applications..."
    ./dev-scripts/build/deploy_to_applications.sh

# === URL HANDLER COMMANDS ===

# Set up development URL handler
setup-url-handler:
    @echo "🔗 Setting up URL handler..."
    ./dev-scripts/setup/setup_dev_url_handler.sh

# Register URL handler (archived - use setup-url-handler instead)
# register-url-handler: [removed - used outdated LaunchAgent approach]

# === TESTING COMMANDS ===

# Run all tests
test:
    cargo test

# Test basic actions
test-actions:
    @echo "🧪 Testing actions..."
    ./dev-scripts/test/test_actions.sh

# Test commands
test-commands:
    @echo "🧪 Testing commands..."
    ./dev-scripts/test/test_commands.sh

# Test Swift supervisor
test-supervisor:
    @echo "🧪 Testing Swift supervisor..."
    ./dev-scripts/test/test_swift_supervisor.sh

# Diagnose Obsidian integration
test-obsidian:
    @echo "🧪 Diagnosing Obsidian integration..."
    ./dev-scripts/test/diagnose_obsidian_issue.sh

# Test keystroke capture
test-keystrokes:
    @echo "🧪 Testing keystroke capture..."
    ./dev-scripts/test/test_keystroke_capture.sh

# === MAINTENANCE COMMANDS ===

# Set up git hooks
setup-git:
    @echo "🔧 Setting up git hooks..."
    ./dev-scripts/setup/setup_git_hooks.sh

# Uninstall HookAnchor
uninstall:
    @echo "🗑️ Running uninstaller..."
    ./dev-scripts/uninstall.sh

# Migrate Obsidian files to markdown
migrate-obsidian:
    @echo "📝 Migrating Obsidian files..."
    ./dev-scripts/migrate_obs_to_markdown.sh

# === UTILITY COMMANDS ===

# Generate HTML documentation from markdown
docs:
    #!/usr/bin/env bash
    set -e
    echo "📚 Generating HTML documentation..."

    # Docs are in Obsidian vault, not in the code repo
    DOCS_DIR="/Users/oblinger/ob/kmr/prj/binproj/Hook Anchor/docs/User Docs"

    # Check if pandoc is installed
    if ! command -v pandoc &> /dev/null; then
        echo "❌ Error: pandoc is not installed"
        echo "Install with: brew install pandoc"
        exit 1
    fi

    # Copy CSS file to docs directory
    PROJECT_ROOT="{{justfile_directory()}}"
    cp "$PROJECT_ROOT/docs-style.css" "$DOCS_DIR/docs-style.css"

    # Convert each markdown file to HTML with nice styling
    for md_name in README USER_GUIDE TEMPLATES_AND_SCRIPTING; do
        md_file="$DOCS_DIR/${md_name}.md"
        if [ -f "$md_file" ]; then
            html_file="$DOCS_DIR/${md_name}.html"
            echo "  Converting ${md_name}.md → ${md_name}.html..."

            # First convert markdown links to point to HTML files
            temp_md="$DOCS_DIR/.${md_name}_temp.md"
            sed 's/\.md)/.html)/g' "$md_file" > "$temp_md"

            pandoc "$temp_md" \
                --standalone \
                --from markdown \
                --to html5 \
                --css=docs-style.css \
                --metadata title="HookAnchor - ${md_name//_/ }" \
                --highlight-style=tango \
                --table-of-contents \
                --toc-depth=3 \
                -o "$html_file"

            rm "$temp_md"
        else
            echo "  ⚠️  ${md_name}.md not found at: $md_file"
        fi
    done

    echo "✅ Documentation generated in $DOCS_DIR/"
    echo "📖 Open with: open '$DOCS_DIR/USER_GUIDE.html'"

# Show version from Cargo.toml
version:
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'

# Show project status
status:
    @echo "📊 HookAnchor Development Status"
    @echo "================================"
    @echo "Version: $(just version)"
    @echo "Built binaries:"
    @ls -la target/release/{ha,popup,popup_server,installer_gui} 2>/dev/null || echo "  No release binaries found"
    @echo ""
    @echo "Distribution:"
    @ls -la dist/*.dmg 2>/dev/null || echo "  No distribution packages found"
    @echo ""
    @echo "Installation:"
    @ls -la /Applications/HookAnchor.app 2>/dev/null && echo "  ✅ Installed in /Applications" || echo "  ❌ Not installed in /Applications"

# Kill all running HookAnchor processes
kill:
    @echo "🛑 Stopping all HookAnchor processes..."
    pkill -f "hookanchor" 2>/dev/null || true
    pkill -f "popup_server" 2>/dev/null || true
    pkill -f "HookAnchor" 2>/dev/null || true
    @echo "✅ Processes stopped"

# === QUICK DEVELOPMENT WORKFLOW ===

# Quick build (simple wrapper)
build-quick:
    @echo "🔨 Quick build..."
    ./dev-scripts/build/build.sh

# Quick development cycle: build, deploy, test
dev: build deploy
    @echo "🚀 Development cycle complete!"
    @echo "Ready to test: /Applications/HookAnchor.app"

# Full development setup from scratch
bootstrap: setup setup-url-handler setup-git
    @echo "🎉 Development environment fully bootstrapped!"

# === CI/RELEASE COMMANDS ===

# Prepare release (build universal + distribution)
release: build-universal build-dist
    @echo "🎉 Release package ready!"
    @echo "Distribution: dist/HookAnchor-$(just version).dmg"