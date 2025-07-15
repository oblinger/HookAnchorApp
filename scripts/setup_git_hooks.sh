#!/bin/bash

# Setup git hooks for HookAnchor development
# This is optional - developers can choose whether to enable commit-time testing

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
HOOKS_DIR="$PROJECT_ROOT/.git/hooks"

echo "🔧 Setting up git hooks for HookAnchor"
echo "======================================"

# Check if we're in a git repository
if [[ ! -d "$PROJECT_ROOT/.git" ]]; then
    echo "❌ Error: Not in a git repository"
    exit 1
fi

echo "📁 Git hooks directory: $HOOKS_DIR"

# Create pre-commit hook
cat > "$HOOKS_DIR/pre-commit" << 'EOF'
#!/bin/bash

# HookAnchor pre-commit hook
# Runs tests before allowing commits to catch regressions early

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🧪 Running pre-commit tests...${NC}"

# Run the commit tests (unit + integration tests)
if make commit-tests; then
    echo -e "${GREEN}✅ All tests passed - commit allowed${NC}"
    exit 0
else
    echo -e "${RED}❌ Tests failed - commit blocked${NC}"
    echo -e "${YELLOW}💡 To skip tests: git commit --no-verify${NC}"
    echo -e "${YELLOW}💡 To fix: make test-unit && make test-integration${NC}"
    exit 1
fi
EOF

# Make it executable
chmod +x "$HOOKS_DIR/pre-commit"

echo "✅ Pre-commit hook installed"
echo ""
echo "📋 Usage:"
echo "  • Normal commits now run tests (adds 2-5 seconds)"
echo "  • Skip tests: git commit --no-verify"
echo "  • Manual test: make commit-tests"
echo ""
echo "🔧 To disable:"
echo "  rm $HOOKS_DIR/pre-commit"
echo ""
echo "🎉 Git hooks setup complete!"