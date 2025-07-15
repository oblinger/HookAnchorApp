# HookAnchor Test Organization

This directory contains all tests for HookAnchor, organized by type and execution method.

## Directory Structure

```
tests/
├── README.md                    # This file
├── integration_*.rs             # Integration tests (require app installation)
│   └── integration_hook_url.rs  # Hook URL handling integration test
└── manual/                      # Manual/interactive tests  
    └── test_hook_urls.sh         # Hook URL handling manual test
```

## Test Types

### Integration Tests (`tests/integration_*.rs`)

**What**: Tests that require the full application to be installed and functional
**Format**: Rust test files in tests/ directory
**Naming**: `integration_*.rs` (Cargo requirement)
**Requirements**: HookAnchor.app must be installed in `/Applications/`
**Runtime**: 2-5 seconds each

```bash
# Run all integration tests
./run_integration_tests.sh

# Or via Makefile
make test-integration
```

**Example tests**:
- URL scheme handling end-to-end
- App bundle configuration validation
- System integration verification

### Manual Tests (`tests/manual/`)

**What**: Interactive tests that verify real system behavior
**Format**: Executable shell scripts (`*.sh`)
**Naming**: `test_*.sh` or descriptive names
**Requirements**: Varies by test (apps, permissions, etc.)
**Runtime**: 3-10 seconds each

```bash
# Run all manual tests
./run_manual_tests.sh

# Or via Makefile  
make test-manual

# Run individual test
./tests/manual/test_hook_urls.sh
```

**Example tests**:
- Hook URL handling verification
- System permission checks
- End-user workflow validation

## Adding New Tests

### Adding Integration Tests

1. Create a new `.rs` file in `tests/` with `integration_` prefix
2. Use standard Rust test format with `#[test]`
3. Include system requirements checks
4. Test will be automatically discovered by `./run_integration_tests.sh`

```rust
// tests/integration_new_feature.rs
#[test]
fn test_new_feature_integration() {
    // Skip if not on macOS
    if !cfg!(target_os = "macos") {
        return;
    }
    
    // Check prerequisites
    assert!(Path::new("/Applications/HookAnchor.app").exists(),
        "HookAnchor.app not installed");
    
    // Your test logic here
}
```

### Adding Manual Tests

1. Create a new `.sh` file in `tests/manual/`
2. Make it executable (`chmod +x`)
3. Follow the standard format with clear success/failure output
4. Test will be automatically discovered by `./run_manual_tests.sh`

```bash
#!/bin/bash
# tests/manual/test_new_feature.sh

set -e

echo "🧪 Testing new feature..."

# Your test logic here

if [[ $? -eq 0 ]]; then
    echo "✅ New feature test PASSED"
    exit 0
else
    echo "❌ New feature test FAILED"
    exit 1
fi
```

## Running Tests

### Top-Level Scripts

| Script | Purpose | Duration |
|--------|---------|----------|
| `./run_integration_tests.sh` | All integration tests | 2-10 seconds |
| `./run_manual_tests.sh` | All manual tests | 5-30 seconds |

### Makefile Targets

| Target | Description | Uses |
|--------|-------------|------|
| `make test-integration` | Integration tests | `./run_integration_tests.sh` |
| `make test-manual` | Manual tests | `./run_manual_tests.sh` |
| `make test` | Unit + integration | Both unit and integration |
| `make commit-tests` | Pre-commit validation | Unit + integration |

### Individual Tests

```bash
# Run specific integration test
cargo test --test test_hook_url_integration

# Run specific manual test
./tests/manual/test_hook_urls.sh
```

## Test Categories by Use Case

| Use Case | Test Type | When to Run |
|----------|-----------|-------------|
| **Development** | Unit tests | Every build |
| **Feature validation** | Integration tests | After changes |
| **System verification** | Manual tests | Before releases |
| **Regression detection** | All tests | Before commits |
| **Release validation** | All tests | Before deployment |

## Best Practices

### Integration Tests
- ✅ Check system prerequisites first
- ✅ Skip gracefully on unsupported platforms
- ✅ Use descriptive assertion messages
- ✅ Clean up after yourself
- ❌ Don't assume specific system configuration

### Manual Tests
- ✅ Provide clear success/failure output
- ✅ Include troubleshooting information
- ✅ Use consistent output formatting
- ✅ Exit with proper return codes (0 = success)
- ❌ Don't require user interaction (automate everything)

### Naming Conventions
- Integration tests: `integration_*.rs` (e.g., `integration_hook_url.rs`)
- Manual tests: `test_*.sh` (e.g., `test_hook_urls.sh`)
- Feature-specific: `integration_app_bundle.rs`, `test_permissions.sh`
- Clear, descriptive names that explain what's being tested

This organization ensures that tests are easy to find, run, and maintain while providing clear separation between different types of testing.