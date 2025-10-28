# HookAnchor Development Scripts Analysis

## Summary Table

| Script | Category | Status | Recommendation | Reason |
|--------|----------|--------|----------------|---------|
| `build/build_distribution.sh` | Build | **Keep** | Keep as-is | Core distribution building - actively used |
| `build/build.sh` | Build | **Remove** | Remove (redundant) | Replaced by `just build` command |
| `build/create_applescript_app.sh` | Build | **Archive** | Move to old/ | Legacy - no longer used |
| `build/create_supervisor_app_bundle.sh` | Build | **Archive** | Move to old/ | Legacy - supervisor built differently now |
| `build/create_uninstaller_app.sh` | Build | **Keep** | Keep as-is | Recently added, actively used |
| `build/deploy_to_applications.sh` | Build | **Keep** | Keep as-is | Used for development deployment |
| `build/generate_default_config_js.py` | Build | **Keep** | Keep as-is | Part of distribution build process |
| `build/generate_default_config.py` | Build | **Keep** | Keep as-is | Part of distribution build process |
| `migration/migrate_config_to_actions.py` | Migration | **Archive** | Move to old/ | One-time migration - likely complete |
| `migration/refactor_actions.py` | Migration | **Archive** | Move to old/ | One-time refactoring - likely complete |
| `performance/measure_performance.py` | Performance | **Keep** | Keep as-is | Performance testing utility |
| `setup/create_app_symlink.sh` | Setup | **Keep** | Keep as-is | Development environment setup |
| `setup/register_url_handler.sh` | Setup | **Keep** | Keep as-is | URL handler registration |
| `setup/setup_dev_app.sh` | Setup | **Keep** | Keep as-is | Development app setup |
| `setup/setup_dev_environment.sh` | Setup | **Keep** | Keep as-is | Core development setup |
| `setup/setup_dev_url_handler.sh` | Setup | **Keep** | Keep as-is | URL handler development setup |
| `setup/setup_git_hooks.sh` | Setup | **Keep** | Keep as-is | Git workflow setup |
| `setup/setup_url_handler.py` | Setup | **Consolidate** | Merge with .sh version | Duplicate of shell script version |
| `test/comprehensive_trigger_test.py` | Test | **Keep** | Keep as-is | Comprehensive testing utility |
| `test/diagnose_obsidian_issue.sh` | Test | **Archive** | Move to old/ | Specific issue diagnosis - likely resolved |
| `test/realistic_test.py` | Test | **Keep** | Keep as-is | Realistic scenario testing |
| `test/simple_test.py` | Test | **Keep** | Keep as-is | Basic functionality testing |
| `test/test_actions.sh` | Test | **Keep** | Keep as-is | Actions system testing |
| `test/test_commands.sh` | Test | **Keep** | Keep as-is | Commands testing |
| `test/test_hacon.sh` | Test | **Review** | Unclear purpose | Need to examine what "hacon" refers to |
| `test/test_installer.py` | Test | **Keep** | Keep as-is | Installer testing utility |
| `test/test_keystroke_capture.sh` | Test | **Keep** | Keep as-is | Core functionality testing |
| `test/test_swift_supervisor.sh` | Test | **Keep** | Keep as-is | Supervisor component testing |
| `test/trigger_timing_test.py` | Test | **Keep** | Keep as-is | Performance/timing testing |
| `uninstall.sh` | Root | **Keep** | Keep as-is | Source for distribution uninstaller |

## Scripts Requiring Discussion

### Migration Scripts

**migrate_config_to_actions.py** and **refactor_actions.py**
- These appear to be one-time migration scripts for moving from the old config system to the unified actions system
- **Recommendation**: Archive to old/ folder since this migration has likely been completed
- **Consideration**: Keep if users might still have very old configs that need migration

### Setup Scripts - Duplication

**setup_url_handler.py** vs **setup_dev_url_handler.sh**
- There appear to be both Python and shell script versions for URL handler setup
- **Recommendation**: Consolidate into a single approach (probably the shell script since it's more recent)
- **Action**: Review both and merge functionality into the more complete version

### Legacy Build Scripts

**create_applescript_app.sh** and **create_supervisor_app_bundle.sh**
- These appear to be legacy build approaches that have been superseded
- **Recommendation**: Archive to old/ folder
- **Reasoning**: The current build system uses different approaches for app bundling

### Test Script Clarity

**test_hacon.sh**
- Unclear what "hacon" refers to - could be an old name or specific component
- **Recommendation**: Review contents to determine if still relevant or rename for clarity
- **Options**:
  1. Rename to reflect actual purpose
  2. Archive if it's testing deprecated functionality

### Simple Build Script

**build.sh**
- This is a very simple wrapper around `cargo build --release`
- **Recommendation**: Remove since `just build` provides the same functionality
- **Reasoning**: The justfile is now the primary interface for development commands

## Recommended Actions

### Immediate Cleanup (Safe to do now)
1. **Remove**: `build/build.sh` - Redundant with justfile
2. **Archive**: `migration/migrate_config_to_actions.py` - One-time migration complete
3. **Archive**: `migration/refactor_actions.py` - One-time refactoring complete
4. **Archive**: `test/diagnose_obsidian_issue.sh` - Specific issue diagnosis

### Review and Consolidate
1. **URL Handler Scripts**: Compare and consolidate the Python and shell versions
2. **Legacy Build Scripts**: Verify these are truly unused before archiving
3. **Test Scripts**: Review test_hacon.sh for clarity and relevance

### Create Archive Structure
```
dev-scripts/
├── old/
│   ├── migration/
│   ├── legacy-build/
│   └── specific-issues/
├── build/
├── setup/
├── test/
└── performance/
```

## Notes

- Scripts marked as "Keep" are either actively used in current workflows or provide ongoing utility
- Migration scripts could be valuable for users with very old configs, but are likely no longer needed for current development
- Consider creating a `dev-scripts/old/` directory structure to preserve historical scripts while cleaning up active development folders
- The justfile now provides the primary interface for development commands, reducing the need for some simple wrapper scripts