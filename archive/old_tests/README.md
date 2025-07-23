# Old Tests and Experimental Code

This directory contains test files and experimental code that are no longer part of the main build but are preserved for reference.

## Contents

### Test Utilities (Legacy)
- `test_amap.rs` - Specific matching test for "amap" queries
- `test_config.rs` - Configuration file parsing tests
- `test_delete.rs` - Command deletion functionality tests
- `test_roundtrip.rs` - File save/load roundtrip validation
- `test_save.rs` - Command file saving tests
- `test_submenu.rs` - Submenu detection and display tests
- `test_timing.rs` - Performance timing measurements

### Experimental Features
- `debug_popup.rs` - Debug version of popup with extra logging
- `hotkey_listener.rs` - Global hotkey listener using global-hotkey crate
- `rdev_hotkey_listener.rs` - Alternative hotkey listener using rdev crate

## Why Moved Here

These files were moved to keep the main source tree clean and focused. The core functionality they tested is now covered by:

1. **Unit tests** in the main modules (launcher.rs, eval.rs, lib.rs)
2. **Integration test binaries** (test_launcher.rs, test_action_types.rs, etc.)
3. **Built-in validation** in the main application

The experimental hotkey listeners may be useful for future global hotkey implementation but are not currently needed for the core anchor selector functionality.