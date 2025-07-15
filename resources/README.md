# HookAnchor Icon Management

This directory contains the source icon for HookAnchor.

## Files

- `icon.icns` - The main application icon in Apple ICNS format
- `README.md` - This documentation

## Updating the Icon

### Method 1: Update from PNG (Recommended)
1. Create or obtain your new icon as a high-resolution PNG (1024x1024 or larger)
2. Convert it to ICNS format using the provided script:
   ```bash
   ./create_icon.sh path/to/your/new-icon.png
   ```
3. This will create a new `icon.icns` file with all required sizes

### Method 2: Replace ICNS Directly
1. Replace `resources/icon.icns` with your new ICNS file
2. Make sure it contains multiple sizes (16x16 to 1024x1024)

### Method 3: Use the Icon Management Script
```bash
./update_icon.sh
```

## Applying Icon Changes

After updating the icon file, run:
```bash
./update_icon.sh
```

This will:
- Update the installed app bundle
- Clear icon cache
- Restart UI services
- Update distribution files (if they exist)

## Automatic Integration

The icon is automatically integrated when you run:
- `./package_for_distribution.sh` - Includes the latest icon in distribution packages
- `./update_icon.sh` - Applies icon changes to installed app

## Icon Requirements

- **Format**: Apple ICNS format
- **Sizes**: Should contain multiple sizes (16x16, 32x32, 64x64, 128x128, 256x256, 512x512, 1024x1024)
- **Style**: Should follow macOS design guidelines
- **Transparency**: Supported for non-square designs

## Current Icon

The current icon is the anchor + lightning bolt design that represents:
- **Anchor**: The "hooking" aspect of the application
- **Lightning**: Speed and instant execution
- **Colors**: Professional blue and orange theme