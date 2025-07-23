# HookAnchor Testing Guide

This document describes the testing strategy for HookAnchor, particularly focusing on URL scheme handling which is critical for proper functionality.

## Test Categories

### 1. Unit Tests (Fast)
- **Purpose**: Test individual functions and modules in isolation
- **Runtime**: < 1 second
- **When to run**: Every code change, before commits
- **Command**: `cargo test --lib`

### 2. Integration Tests (Slower) 
- **Purpose**: Test end-to-end functionality including URL scheme handling
- **Runtime**: 2-5 seconds
- **When to run**: Before releases, after major changes, in CI/CD
- **Command**: `cargo test --test test_hook_url_integration`

### 3. Manual Tests (Interactive)
- **Purpose**: Quick verification during development
- **Runtime**: 3-5 seconds  
- **When to run**: After configuration changes, troubleshooting
- **Command**: `./scripts/test_hook_urls.sh`

## Critical Test: hook:// URL Handling

### Why This Test is Important

The `hook://` URL scheme handling is a **critical failure point** that can regress easily due to:

1. **macOS app bundle configuration changes** (Info.plist)
2. **Binary name changes** (popup vs ha vs applet)
3. **AppleScript compilation issues**
4. **Launch Services registration problems**

When this breaks, users get a popup window instead of command execution, which is confusing and incorrect behavior.

### Test Coverage

Our URL handling tests verify:

✅ **Correct behavior**:
- URL is processed by CLI mode (`URL_HANDLER` logs)
- Binary starts properly (`STARTUP` logs)  
- Command execution or "not found" message
- No popup window opens (`POPUP_OPEN` absent)

❌ **Regression detection**:
- Popup opening inappropriately
- URL not being processed at all
- AppleScript handler not working
- Binary not being found

### Optimized Test Strategy for Fast Development

| Scenario | Command | Duration | When |
|----------|---------|----------|------|
| **Development builds** | `make build` | < 1 sec | Every change (Keyboard Maestro) |
| **Quick verification** | `./scripts/test_hook_urls.sh` | 3-5 sec | As needed |
| **Pre-commit** | `make commit-tests` | 2-5 sec | Before commits (optional) |
| **Installation** | `make install` | 5-8 sec | When deploying |
| **Full validation** | `make test` | 3-7 sec | Before releases |

### When to Run Which Tests

| Scenario | Fast Build | Unit Tests | Integration Tests | Manual Test |
|----------|------------|------------|-------------------|-------------|
| **Regular development** | ✅ `make build` | Optional | No | Optional |
| **Keyboard Maestro builds** | ✅ `make build` | No | No | No |
| **Before commits** | ✅ Optional | ✅ `make commit-tests` | ✅ `make commit-tests` | Optional |
| **App bundle changes** | ✅ Required | ✅ Required | ✅ Required | ✅ Recommended |
| **Installation** | ✅ `make install` | No | No | ✅ Automatic |
| **Before releases** | ✅ Required | ✅ Required | ✅ Required | ✅ Required |
| **Troubleshooting** | N/A | Optional | ✅ Helpful | ✅ Required |

## Running Tests

### 🚀 Fast Development Loop (Keyboard Maestro Compatible)
```bash
# Ultra-fast builds (no tests) - perfect for Keyboard Maestro
make build                    # < 1 second

# Optional quick verification when needed
./scripts/test_hook_urls.sh   # 3-5 seconds
```

### 🧪 Pre-Commit Validation (Optional)
```bash
# Enable automated commit testing (one-time setup)
./scripts/setup_git_hooks.sh

# Manual pre-commit testing
make commit-tests            # 2-5 seconds (unit + integration)

# Skip tests when committing (if needed)
git commit --no-verify
```

### 📦 Installation & Deployment
```bash
# Install with automatic URL verification
make install                 # 5-8 seconds (includes test)

# Create distribution packages
make package                 # Build + package
```

### 🔍 Full Validation (Before Releases)
```bash
# Complete test suite
make test                    # 3-7 seconds

# Or CI pipeline
make ci                      # test-unit → build → install → test-integration
```

### Troubleshooting Failed Tests

#### If URL test fails with "Popup opened incorrectly":

1. Check `Info.plist` has `CFBundleExecutable = applet`
2. Verify AppleScript is compiled: `/Applications/HookAnchor.app/Contents/Resources/Scripts/main.scpt`
3. Ensure binary exists: `/Applications/HookAnchor.app/Contents/MacOS/ha`
4. Re-register URL scheme: `make install`

#### If URL test fails with "No URL_HANDLER entries":

1. Check app bundle exists: `/Applications/HookAnchor.app`
2. Verify URL scheme registration: `open hook://test`
3. Check AppleScript can find binary
4. Look for errors in Console.app

#### If tests are too slow:

- Use `make test-unit` for fast feedback
- Run integration tests only before commits/releases
- Use manual script for quick verification

## Test Automation Strategy

### Local Development
- **Pre-commit hook**: Run unit tests
- **Manual verification**: Use `./scripts/test_hook_urls.sh`

### CI/CD Pipeline
```yaml
# Example GitHub Actions
- name: Test HookAnchor
  run: |
    make test-unit       # Fast tests always
    make build install   # Build and install 
    make test-integration # Full integration tests
```

### Release Process
1. `make test-unit` - Verify no regressions
2. `make build install` - Build and install with URL test
3. `make test-integration` - Full end-to-end verification
4. `make package` - Create distribution packages
5. Manual testing on clean system

## Commit Time Impact Analysis

### Performance Impact of Commit-Time Testing

| Component | Duration | What It Does |
|-----------|----------|--------------|
| **Unit tests** | < 1 second | Test individual functions |
| **Integration tests** | 2-4 seconds | Test `open hook://...` end-to-end |
| **Total commit delay** | **2-5 seconds** | Complete pre-commit validation |

### Commit Workflow Options

#### Option 1: Automatic Testing (Recommended for Critical Changes)
```bash
# One-time setup
./scripts/setup_git_hooks.sh

# Normal commits now include testing
git commit -m "Fix URL handling"  # Adds 2-5 seconds
```

#### Option 2: Manual Testing (Fast Commits)
```bash
# Fast commits (no automatic testing)
git commit -m "Minor change"  # < 1 second

# Manual testing when needed
make commit-tests             # 2-5 seconds
```

#### Option 3: Skip Testing (Emergency Commits)
```bash
# Skip all tests when needed
git commit --no-verify -m "Hotfix"  # < 1 second
```

**Answer to your question**: Commit-time testing adds **2-5 seconds** to each commit, but it's optional and can be skipped with `--no-verify`.

## Performance Expectations

| Test Type | Expected Duration | Alert If |
|-----------|------------------|----------|
| Unit tests | < 1 second | > 5 seconds |
| Integration tests | 2-5 seconds | > 10 seconds |
| Manual URL test | 3-5 seconds | > 10 seconds |
| Full CI pipeline | < 30 seconds | > 2 minutes |

## Debugging Test Failures

### Enable Verbose Logging
```bash
# Check what's happening in the logs
tail -f ~/.anchor.log

# Run test and watch logs simultaneously
./scripts/test_hook_urls.sh &
tail -f ~/.anchor.log
```

### Check App Bundle State
```bash
# Verify app bundle configuration
make test-integration  # Includes bundle configuration test

# Manual checks
ls -la /Applications/HookAnchor.app/Contents/MacOS/
cat /Applications/HookAnchor.app/Contents/Info.plist | grep -A1 CFBundleExecutable
```

### Test URL Registration
```bash
# Check which app handles hook:// URLs
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -dump | grep hook
```

This testing strategy ensures that the critical URL handling functionality remains reliable and any regressions are caught quickly.