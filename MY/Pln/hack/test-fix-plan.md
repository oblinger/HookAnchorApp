# HookAnchor Test Fix Plan

## Current Status (After Initial Fixes)

### ✅ Tests That Work (2)
1. **active_tests.rs** - ✅ 10 passed
2. **test_delete_key_functionality.rs** - ✅ 1 passed

### ⚠️ Tests That Compile But Fail (4)
1. **display_tests.rs** - 34 passed, 10 failed
2. **integration_finder_selection.rs** - 0 passed, 3 failed
3. **integration_hook_url.rs** - 1 passed, 2 failed
4. **regression_url_processing.rs** - 10 passed, 1 failed

### ❌ Tests That Won't Compile (16)

## Fixing Strategy

### Priority 1: Fix Failing But Compiling Tests

#### display_tests.rs (10 failures)
- Already compiles, just needs assertion fixes
- Most important test file
- **Action:** Debug the 10 failing tests and fix assertions

#### integration_finder_selection.rs (3 failures)
- Compiles, runtime failures
- **Action:** Check what's failing and fix

#### integration_hook_url.rs (2 failures)
- Compiles, runtime failures
- **Action:** Check what's failing and fix

#### regression_url_processing.rs (1 failure)
- Compiles, runtime failures
- **Action:** Check what's failing and fix

### Priority 2: Fix Simple Import Errors

These tests just need updated imports to use public APIs:

#### test_command_editor_opening.rs
**Error:** `struct hookanchor::core::Command has no field named full_line`
**Fix:** Command struct changed - need to update test to not use `full_line` field
**Difficulty:** Medium - need to understand new Command structure

#### test_merged_command_execution.rs
**Error:** `unresolved import hookanchor::Command`
**Fix:** Change to `hookanchor::core::Command` (already attempted, may have other issues)
**Difficulty:** Easy-Medium

#### test_url_handler.rs
**Error:** `unresolved import hookanchor::Command`
**Fix:** Change to `hookanchor::core::Command`
**Difficulty:** Easy-Medium

#### split_commands_tests.rs
**Error:** `unresolved imports hookanchor::Command, hookanchor::split_commands`
**Fix:**
- `hookanchor::Command` → `hookanchor::core::Command`
- `hookanchor::split_commands` → Need to find where this function is now
**Difficulty:** Medium

#### test_alias_functionality.rs
**Error:** `unresolved imports hookanchor::Command, hookanchor::load_commands`
**Fix:**
- `hookanchor::Command` → `hookanchor::core::Command`
- `hookanchor::load_commands` → Use `hookanchor::test_helpers::parse_test_commands` or `hookanchor::core::initialize()`
**Difficulty:** Medium

#### test_display_sync.rs
**Error:** `unresolved imports hookanchor::Command, hookanchor::filter_commands`
**Fix:**
- `hookanchor::Command` → `hookanchor::core::Command`
- `hookanchor::filter_commands` → `hookanchor::core::filter_commands`
**Difficulty:** Easy

#### test_hook_url_handler.rs
**Error:** `unresolved import hookanchor::load_commands`
**Fix:** Use `hookanchor::core::initialize()` or test_helpers
**Difficulty:** Medium

#### test_patch_inference.rs
**Error:** `unresolved imports hookanchor::Command, hookanchor::Patch, hookanchor::infer_patch`
**Fix:**
- `hookanchor::Command` → `hookanchor::core::Command`
- `hookanchor::Patch` → `hookanchor::core::Patch`
- `hookanchor::infer_patch` → `hookanchor::core::infer_patch`
**Difficulty:** Easy

### Priority 3: Fix Launcher Module Dependencies

These tests depend on a `launcher` module that doesn't exist:

- javascript_actions.rs
- launcher_integration.rs
- onepassword_integration.rs
- test_shell_environment.rs
- test_shutdown_command.rs

**Options:**
1. Find where launcher functionality moved to (`hookanchor::execute` module?)
2. Delete these tests if launcher was removed
3. Move them to unit tests in the source if they test private APIs

**Difficulty:** Hard - need to understand what happened to launcher module

### Priority 4: Fix Private Module Access

These tests access private modules and need more substantial rewrites:

#### functional_test_suite.rs
**Error:** Accessing private `hookanchor::core::commands` module
**Fix:** Rewrite to use public APIs and test_helpers
**Difficulty:** Hard - comprehensive test that needs major rewrite

#### test_submenu_logic.rs
**Error:** Accessing private module functions
**Fix:** Either expose these through public API or move to unit tests in source
**Difficulty:** Hard

#### debug_part1.rs
**Error:** Type mismatches
**Fix:** Unclear without seeing full errors
**Difficulty:** Unknown

## Recommended Workflow

1. **Start with display_tests.rs** - Fix the 10 failing assertions
   - Run: `cargo test --test display_tests`
   - Fix one test at a time

2. **Fix easy import errors**:
   ```bash
   # test_display_sync.rs
   sed -i '' 's/hookanchor::filter_commands/hookanchor::core::filter_commands/g' tests/test_display_sync.rs

   # test_patch_inference.rs
   sed -i '' 's/hookanchor::Patch/hookanchor::core::Patch/g' tests/test_patch_inference.rs
   sed -i '' 's/hookanchor::infer_patch/hookanchor::core::infer_patch/g' tests/test_patch_inference.rs
   ```

3. **Fix the 4 integration tests** that compile but fail

4. **Decide on launcher tests** - delete or rewrite?

5. **Decide on private module tests** - move to unit tests or expose APIs?

## Commands Reference

```bash
# Test all tests and see status
/tmp/test_check.sh

# Test specific file
cargo test --test display_tests

# See what's failing in a test
cargo test --test display_tests 2>&1 | grep "test.*FAILED"

# Run only failing tests
cargo test --test display_tests 2>&1 | grep "FAILED" | awk '{print $2}' | while read test; do cargo test --test display_tests $test; done
```

## Files to Check

- Command struct definition: Need to find where this is and what fields it has now
- launcher module: Where did it go? Check `src/execute/` maybe?
- split_commands function: Where is this now?
- Public API: `src/core/mod.rs` - lines 58-132 show what's exported

## Next Session Checklist

- [ ] Fix display_tests.rs 10 failures
- [ ] Fix test_display_sync.rs (easy import fix)
- [ ] Fix test_patch_inference.rs (easy import fix)
- [ ] Find Command struct definition and document fields
- [ ] Find where launcher module went
- [ ] Decide: Keep or delete launcher-dependent tests?
- [ ] Decide: Expose submenu functions or move tests to unit tests?
