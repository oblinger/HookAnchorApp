# Hotkey Setup Alternatives

Since global hotkey registration requires specific macOS permissions that can be tricky to configure, here are multiple approaches to bind F10 to launch the Anchor Selector:

## Option 1: macOS System Settings (Recommended)

This is the most reliable method and doesn't require any special permissions:

1. **Go to System Settings > Keyboard > Keyboard Shortcuts**
2. **Click "App Shortcuts" in the left sidebar**
3. **Click the "+" button to add a new shortcut**
4. **Configure:**
   - Application: "All Applications" 
   - Menu Title: (leave blank)
   - Keyboard Shortcut: Press F10
   - Command: Browse and select `launch_popup.sh`

## Option 2: Alfred/Raycast (If you use them)

If you use Alfred or Raycast:
- Create a workflow that runs `./launch_popup.sh`
- Bind it to F10

## Option 3: Programmatic Global Hotkey (Advanced)

If you want to use the programmatic approach:

1. **Grant Accessibility permissions:**
   ```bash
   ./test_permissions.sh  # Check current permissions
   ```

2. **Add Terminal to Accessibility:**
   - System Settings > Privacy & Security > Accessibility
   - Click "+" and add Terminal.app
   - Ensure toggle is ON

3. **Run the hotkey listener:**
   ```bash
   ./target/release/rdev_hotkey_listener
   ```

## Option 4: Manual Launch

Simple manual testing:
```bash
./launch_popup.sh
```

## Testing

To test any setup:
1. Press F10 (or your configured key)
2. The Anchor Selector should appear
3. Type a command and press Enter
4. The popup should exit and execute the command

## Troubleshooting

**F10 not working:**
- Check if F10 is used by another app
- Try Option 1 (System Settings) as it's most reliable
- Verify the popup works manually: `./launch_popup.sh`

**Permission errors:**
- Use `./test_permissions.sh` to diagnose
- Grant Accessibility permissions in System Settings
- Consider using Option 1 instead of programmatic hotkeys