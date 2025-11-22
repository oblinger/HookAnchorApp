#!/usr/bin/env bash
# Sanitize configuration files for public distribution
# Removes API keys, personal paths, and sensitive data

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Function to sanitize YAML config
sanitize_yaml() {
    local input_file="$1"
    local output_file="$2"

    echo -e "${YELLOW}📝 Sanitizing YAML config...${NC}"
    echo "  Input: $input_file"
    echo "  Output: $output_file"

    # Use Python script for comprehensive sanitization
    python3 "$SCRIPT_DIR/generate_default_config.py" "$output_file" "$input_file"

    # Additional safety: Replace any environment variable references with placeholders
    sed -i '' 's/\${HOOKANCHOR_[A-Z_]*}/""      # Add your API key here/g' "$output_file"

    echo -e "${GREEN}  ✓ YAML sanitized${NC}"
}

# Function to sanitize JavaScript config
sanitize_js() {
    local input_file="$1"
    local output_file="$2"

    echo -e "${YELLOW}📝 Sanitizing JavaScript config...${NC}"
    echo "  Input: $input_file"
    echo "  Output: $output_file"

    # Use Python script for comprehensive sanitization
    python3 "$SCRIPT_DIR/generate_default_config_js.py" "$output_file" "$input_file"

    echo -e "${GREEN}  ✓ JavaScript sanitized${NC}"
}

# Function to verify no sensitive data remains
verify_clean() {
    local file="$1"

    echo -e "${YELLOW}🔍 Verifying $file is clean...${NC}"

    local issues=0

    # Check for API keys
    if grep -q "ntn_[a-zA-Z0-9]" "$file" 2>/dev/null; then
        echo -e "${RED}  ❌ Found Notion API key!${NC}"
        issues=$((issues + 1))
    fi

    if grep -q "secret_[a-zA-Z0-9]" "$file" 2>/dev/null; then
        echo -e "${RED}  ❌ Found secret key!${NC}"
        issues=$((issues + 1))
    fi

    # Check for personal paths
    if grep -q "/Users/oblinger/ob/kmr" "$file" 2>/dev/null; then
        echo -e "${RED}  ❌ Found personal path: /Users/oblinger/ob/kmr${NC}"
        issues=$((issues + 1))
    fi

    if grep -q "~/ob/kmr" "$file" 2>/dev/null; then
        echo -e "${RED}  ❌ Found personal path: ~/ob/kmr${NC}"
        issues=$((issues + 1))
    fi

    # Check for hostname
    if grep -q "Daniels-MacBook-Pro" "$file" 2>/dev/null; then
        echo -e "${RED}  ❌ Found hostname!${NC}"
        issues=$((issues + 1))
    fi

    # Check for personal vault name
    if grep -q 'obsidian_vault_name.*"kmr"' "$file" 2>/dev/null; then
        echo -e "${RED}  ❌ Found personal vault name: kmr${NC}"
        issues=$((issues + 1))
    fi

    if [ $issues -eq 0 ]; then
        echo -e "${GREEN}  ✓ File is clean${NC}"
        return 0
    else
        echo -e "${RED}  ❌ Found $issues security issues!${NC}"
        return 1
    fi
}

# Main execution
main() {
    echo "=================================================="
    echo -e "${YELLOW}🔒 Configuration Sanitization${NC}"
    echo "=================================================="
    echo ""

    # Default input/output paths
    INPUT_YAML="${INPUT_YAML:-$PROJECT_ROOT/config/config.yaml}"
    INPUT_JS="${INPUT_JS:-$PROJECT_ROOT/config/config.js}"
    OUTPUT_YAML="${OUTPUT_YAML:-$PROJECT_ROOT/config/default_config.yaml}"
    OUTPUT_JS="${OUTPUT_JS:-$PROJECT_ROOT/config/default_config.js}"

    # Check if input files exist
    if [ ! -f "$INPUT_YAML" ]; then
        echo -e "${RED}❌ Input YAML not found: $INPUT_YAML${NC}"
        exit 1
    fi

    if [ ! -f "$INPUT_JS" ]; then
        echo -e "${RED}❌ Input JavaScript not found: $INPUT_JS${NC}"
        exit 1
    fi

    # Sanitize YAML
    sanitize_yaml "$INPUT_YAML" "$OUTPUT_YAML"

    # Sanitize JavaScript
    sanitize_js "$INPUT_JS" "$OUTPUT_JS"

    echo ""
    echo "=================================================="
    echo -e "${YELLOW}🔍 Security Verification${NC}"
    echo "=================================================="
    echo ""

    # Verify both files are clean
    yaml_clean=0
    js_clean=0

    verify_clean "$OUTPUT_YAML" && yaml_clean=1
    verify_clean "$OUTPUT_JS" && js_clean=1

    echo ""

    if [ $yaml_clean -eq 1 ] && [ $js_clean -eq 1 ]; then
        echo -e "${GREEN}✅ All configurations sanitized successfully!${NC}"
        echo ""
        echo "Sanitized files:"
        echo "  - $OUTPUT_YAML"
        echo "  - $OUTPUT_JS"
        return 0
    else
        echo -e "${RED}❌ Sanitization failed - sensitive data remains!${NC}"
        echo "Please review the output files before distributing."
        return 1
    fi
}

# Run main function
main "$@"
