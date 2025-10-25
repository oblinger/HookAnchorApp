# Karabiner-Elements Global Hotkey Integration

This document outlines our planned approach for implementing robust global hotkey support using Karabiner-Elements.

## Overview

Karabiner-Elements is a powerful keyboard customization tool for macOS that provides reliable global hotkey functionality. Our plan is to create an installer that can automatically configure Karabiner to launch the Anchor Selector with a chosen hotkey (like F10).

## Implementation Strategy

### Smart Detection Installer (Recommended Approach)

The installer will follow this logic:

1. **Check if Karabiner-Elements is installed**
   - Look for `/Applications/Karabiner-Elements.app`
   - Check if Karabiner daemon is running

2. **If Karabiner is installed:**
   - Automatically generate and install JSON configuration
   - Configure chosen hotkey (F10, Cmd+F9, etc.)
   - Enable the rule in Karabiner
   - Provide success confirmation

3. **If Karabiner is not installed:**
   - Display clear installation instructions
   - Provide download link: https://karabiner-elements.pqrs.org/
   - Offer to retry configuration after user installs Karabiner
   - Save configuration for later application

### Configuration Details

#### JSON Configuration Structure
```json
{
  "title": "Anchor Selector Hotkey",
  "rules": [
    {
      "description": "Launch Anchor Selector with F10",
      "manipulators": [
        {
          "type": "basic",
          "from": {
            "key_code": "f10"
          },
          "to": [
            {
              "shell_command": "/Applications/HookAnchor.app/Contents/MacOS/hookanchor"
            }
          ]
        }
      ]
    }
  ]
}
```

#### Installation Path
- Configuration file: `~/.config/karabiner/assets/complex_modifications/hookanchor.json`
- Enable rule via Karabiner's Complex Modifications settings

### Benefits of Karabiner Approach

#### ✅ Advantages:
- **Reliable**: Works consistently across all macOS versions
- **Flexible**: Easy to change hotkeys or add multiple shortcuts
- **Popular**: Many developers already have Karabiner installed
- **Scriptable**: Can be fully automated once Karabiner exists
- **No permissions needed**: Karabiner handles all system-level access

#### ⚠️ Considerations:
- **Dependency**: Requires users to install Karabiner-Elements first
- **Learning curve**: Users unfamiliar with Karabiner might need guidance
- **Overkill**: Adds system-wide keyboard customization for one hotkey

## Alternative Hotkey Options

If F10 conflicts with system functions, we can easily configure alternatives:

- **F9**: Usually available
- **Cmd+F10**: Modifier reduces conflicts
- **Cmd+Option+A**: "A" for Anchor
- **Cmd+;**: Rarely used combination

## Installation Script Structure

```bash
#!/bin/bash
# install_karabiner_hotkey.sh

1. detect_karabiner_installation()
2. if_installed: generate_and_apply_config()
3. if_not_installed: show_install_instructions()
4. test_hotkey_functionality()
5. provide_user_feedback()
```

## Future Enhancements

- **Multiple hotkeys**: Different keys for different command types
- **Conditional rules**: Different behavior based on active application
- **Visual feedback**: Show notification when hotkey is triggered
- **Easy reconfiguration**: Simple script to change hotkey assignments

## User Experience Goal

**Ideal flow:**
1. User runs `install_hotkey.sh`
2. If Karabiner exists: "✅ F10 hotkey configured! Press F10 to test."
3. If Karabiner missing: "Please install Karabiner-Elements, then re-run this script."

**Result:** One-click hotkey setup with clear guidance when dependencies are missing.

---

*This approach provides the most robust and maintainable solution for global hotkey functionality while remaining easy to distribute and install.*