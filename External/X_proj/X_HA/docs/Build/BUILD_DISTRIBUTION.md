# Building HookAnchor for Distribution

## Overview

HookAnchor uses a single unified build script (`build_distribution.sh`) that creates a complete distribution package with proper URL handler architecture.

## Architecture

The distribution package contains:

```
HookAnchor.app/
├── Contents/
│   ├── Info.plist               # Main app config (NO URL registration!)
│   ├── MacOS/
│   │   ├── HookAnchor           # Swift supervisor (main executable)
│   │   ├── popup_server         # Rust popup UI server
│   │   ├── ha                   # Command-line interface
│   │   └── popup                # Popup control utility
│   └── Resources/
│       ├── AppIcon.icns         # Application icon
│       ├── default_config.yaml  # Default configuration
│       └── URLHandler.app/      # Embedded URL handler
│           └── Contents/
│               ├── Info.plist   # Registers hook:// URLs
│               └── MacOS/
│                   └── url_launcher  # Minimal URL processor
```

## Key Features

### URL Handler Isolation
- **Main app** (`HookAnchor.app`) has NO URL registration
- **URL handler** (`URLHandler.app`) is embedded in Resources
- URL handler executes `ha --hook` and exits immediately
- Prevents the popup from appearing when handling URLs

### Universal Binaries
- Builds for both Intel (x86_64) and Apple Silicon (ARM64)
- Creates universal binaries using `lipo`
- Single app works on all modern Macs

### Safety Features
- **No temporary apps left on filesystem** that could register URLs
- **Direct deletion** (not trashing) of temporary files
- **Cleanup on error** using trap handlers
- **Verification** that no URL handlers leaked

## Build Process

### Prerequisites
1. **Rust toolchain** with both targets:
   ```bash
   rustup target add x86_64-apple-darwin
   rustup target add aarch64-apple-darwin
   ```

2. **Xcode Command Line Tools** for Swift compilation

3. **Developer Certificate** (optional) for code signing

### Building

Run the build script from the project root:

```bash
./build_distribution.sh
```

The script will:
1. Clean any existing temporary files
2. Build Rust components for both architectures
3. Build Swift components (supervisor and URL handler)
4. Create universal binaries
5. Assemble the app bundle with proper structure
6. Create the embedded URLHandler.app
7. Sign the app (if certificate available)
8. Create DMG and ZIP packages
9. Clean up all temporary files

### Output

The script creates a `dist/` directory containing:

- `HookAnchor-{version}.dmg` - Disk image for distribution
- `HookAnchor-{version}.zip` - ZIP archive alternative
- `HookAnchor.app` - Standalone app bundle
- `README.md` - User documentation

## Development vs Distribution

### Development Environment
- Uses symlinks to binaries in `target/release/`
- URL handler points to development binaries
- Configured by `setup_dev_environment.sh`

### Distribution Build
- Contains actual binaries (not symlinks)
- Self-contained with all dependencies
- URL handler uses embedded binaries
- Ready for end-user installation

## Critical Safety Rules

### Never Leave URL Handlers Around
- **Temporary apps can register URLs** even from `/tmp`
- **Items in Trash can still handle URLs**
- Always use `rm -rf` not `mv to trash`
- Clean up on both success and error

### URL Registration
- **Only URLHandler.app** should register hook:// URLs
- **Main app Info.plist** must NOT have CFBundleURLTypes
- **One handler per system** to avoid conflicts

## Testing Distribution

After building:

1. **Test installation**:
   ```bash
   cp -R dist/HookAnchor.app /Applications/
   ```

2. **Test URL handling**:
   ```bash
   open "hook://test"
   ```
   Should execute command without showing popup

3. **Test caps lock trigger**:
   - Press caps lock
   - Should show popup

4. **Verify no leaked handlers**:
   ```bash
   lsregister -dump | grep "hook:"
   ```
   Should only show com.hookanchor.urlhandler

## Troubleshooting

### Build Failures
- Check both Rust targets are installed
- Ensure Swift compiler is available
- Check disk space for universal binaries

### URL Handler Issues
- Verify only one handler registered
- Check no handlers in Trash
- Rebuild with clean state

### Distribution Issues
- Sign with Developer ID for smooth installation
- Include clear first-run instructions
- Test on both Intel and Apple Silicon Macs

## Maintenance

### Version Updates
- Version is read from `Cargo.toml`
- Automatically applied to all Info.plists
- Included in package names

### Adding New Components
1. Build for both architectures
2. Create universal binary with `lipo`
3. Add to app bundle in correct location
4. Update this documentation

### Removing Old Scripts
Old/obsolete build scripts are moved to `OLD/scripts/` for reference but are no longer maintained.

## Installation & First Run

### End-User Installation

1. **Download the DMG**: `HookAnchor-{version}.dmg`
2. **Mount and drag** HookAnchor.app to Applications folder
3. **First launch** will check for required dependencies:
   - If Karabiner-Elements is missing, prompts to install
   - Creates `~/.config/hookanchor/` if it doesn't exist
   - Installs default configuration (preserves existing)

### Configuration Preservation

The installer/first-run setup follows these safety rules:

- **Never overwrites** existing `config.yaml`
- **Preserves all user data** during upgrades
- **Creates backups** before any modifications
- **Only creates files** that don't already exist

See [installer-safety.md](installer-safety.md) for detailed safety guarantees.

### Karabiner Integration

On first run, HookAnchor sets up Karabiner-Elements to trigger with Caps Lock:

```json
{
  "title": "HookAnchor",
  "rules": [{
    "description": "Launch HookAnchor with Caps Lock",
    "manipulators": [{
      "type": "basic",
      "from": { "key_code": "caps_lock" },
      "to": [{ "shell_command": "open /Applications/HookAnchor.app" }]
    }]
  }]
}
```

This configuration is installed to:
`~/.config/karabiner/assets/complex_modifications/hookanchor.json`

### Post-Installation

After successful installation:
1. **Caps Lock** triggers the popup interface
2. **hook:// URLs** are handled by the embedded URLHandler
3. **Configuration** can be customized in `~/.config/hookanchor/config.yaml`
4. **Commands** are automatically scanned from configured markdown roots

## Version Archiving

The build script automatically archives distributions to:
```
versions/{version}/
├── HookAnchor-{version}.dmg
├── HookAnchor-{version}.zip
├── BUILD_INFO.txt
└── README.md
```

This provides a historical record of all releases.