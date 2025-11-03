HookAnchor Installation
======================

1. Drag HookAnchor.app to your Applications folder

2. Bypass macOS Security (First Launch Only)

   HookAnchor is not signed with an Apple Developer certificate, so macOS
   will block it on first launch.

   If you get a "Move to Trash" dialog:
   a. Click "Cancel" (do NOT move to trash)
   b. Open System Settings → Privacy & Security
   c. Scroll down to the Security section
   d. You'll see: "HookAnchor.app was blocked from use..."
   e. Click "Open Anyway"
   f. In the confirmation dialog, click "Open"

   Alternative method:
   - Right-click HookAnchor.app in Applications
   - Hold Option key and select "Open"
   - Click "Open" in the security dialog

   This is only needed once. After that, HookAnchor opens normally.

3. Launch HookAnchor from Applications to complete initial setup

4. Grant accessibility permissions when prompted

5. Set Up Keyboard Shortcut

   Recommended (macOS Native - Easiest):
   - Open System Settings → Keyboard → Keyboard Shortcuts
   - Select "App Shortcuts" in left sidebar
   - Click "+" button
   - Application: Select "HookAnchor" (navigate to /Applications)
   - Menu Title: Leave blank
   - Keyboard Shortcut: Press ⌥Space (Option+Space)
   - Click "Add"

   Now press ⌥Space (Option+Space) to open HookAnchor from anywhere.

   Alternative (Karabiner-Elements for Caps Lock):
   If you prefer using Caps Lock as your trigger, install Karabiner-Elements
   (https://karabiner-elements.pqrs.org/) and configure it to launch
   HookAnchor. See the config file at ~/.config/hookanchor/karabiner/hookanchor.json
   for the Karabiner rule.

The setup assistant will:
- Check for required dependencies (Karabiner-Elements)
- Create your configuration files
- Set up the keyboard shortcut (Caps Lock)

For more information, visit: https://github.com/yourusername/hookanchor

Keyboard Shortcut
-----------------
By default, HookAnchor is triggered by pressing Caps Lock (when pressed alone).
You can change this in Karabiner-Elements preferences.

Troubleshooting
---------------
If HookAnchor doesn't launch with Caps Lock:
1. Open Karabiner-Elements preferences
2. Go to Complex Modifications tab
3. Click "Add rule"
4. Find and enable "HookAnchor"

Support
-------
For issues or questions, please file an issue on GitHub.