# HookAnchor Build and Test Makefile

.PHONY: build test test-unit test-integration test-hook-urls commit-tests install clean help

# Default target
help:
	@echo "HookAnchor Build Targets:"
	@echo ""
	@echo "🚀 Development (fast):"
	@echo "  build              - Build release version (no tests)"
	@echo "  test-unit          - Run fast unit tests only"
	@echo "  test-hook-urls     - Test hook:// URL handling (manual)"
	@echo ""
	@echo "🧪 Quality Assurance (slower):"
	@echo "  test               - Run all tests"
	@echo "  test-integration   - Run slower integration tests"
	@echo "  commit-tests       - Pre-commit validation (2-5 sec)"
	@echo ""
	@echo "📦 Release:"
	@echo "  install            - Build and install with verification"
	@echo "  package            - Create distribution packages"
	@echo "  clean              - Clean build artifacts"

# Fast build for development (no tests)
build:
	@echo "🔨 Building HookAnchor (fast)..."
	cargo build --release
	@echo "✅ Build complete (no tests run)"

# Run fast unit tests only
test-unit:
	@echo "🧪 Running unit tests..."
	cargo test --lib
	@echo "✅ Unit tests complete"

# Run integration tests (slower)
test-integration:
	@echo "🧪 Running integration tests..."
	@echo "⚠️  Note: These tests require HookAnchor.app to be installed"
	cargo test --test test_hook_url_integration
	@echo "✅ Integration tests complete"

# Manual URL test script
test-hook-urls:
	@echo "🧪 Running manual hook URL test..."
	./scripts/test_hook_urls.sh

# Run all tests (unit + integration)
test: test-unit test-integration

# Pre-commit validation (runs before git commit)
commit-tests:
	@echo "🧪 Running pre-commit tests..."
	@echo "  → Unit tests (fast)"
	@$(MAKE) test-unit --no-print-directory
	@echo "  → Integration tests (2-5 sec)"
	@$(MAKE) test-integration --no-print-directory
	@echo "✅ Pre-commit tests passed"

# Install with verification (slower due to testing)
install: build
	@echo "📦 Installing HookAnchor with verification..."
	cp target/release/ha "/Applications/HookAnchor.app/Contents/MacOS/ha"
	cp target/release/ha "/Applications/HookAnchor.app/Contents/MacOS/popup"
	@echo "✅ Installation complete"
	@echo "🧪 Running URL handling verification (3-5 sec)..."
	@./scripts/test_hook_urls.sh

# Create distribution packages
package: build
	@echo "📦 Creating distribution packages..."
	./package_for_distribution.sh
	@echo "✅ Distribution packages created"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf dist/
	@echo "✅ Clean complete"

# Continuous Integration target
ci: test-unit build install test-integration
	@echo "🎉 CI pipeline completed successfully!"