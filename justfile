# HookAnchor Development Commands
# Use `just <command>` to run development tasks

# Default recipe - show available commands
default:
    @just --list

# === BUILD COMMANDS ===

# Build all release binaries
build:
    cargo build --release

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