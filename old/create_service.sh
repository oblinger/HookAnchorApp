#!/bin/bash

# Create a macOS Service for AnchorSelector that can be bound to F10
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LAUNCHER_PATH="$SCRIPT_DIR/launch_popup.sh"
SERVICE_DIR="$HOME/Library/Services"
SERVICE_NAME="Launch Anchor Selector.workflow"
SERVICE_PATH="$SERVICE_DIR/$SERVICE_NAME"

echo "Creating macOS Service for AnchorSelector"
echo "========================================="

# Create Services directory if it doesn't exist
mkdir -p "$SERVICE_DIR"

# Remove existing service if present
if [ -d "$SERVICE_PATH" ]; then
    echo "🗑️  Removing existing service..."
    rm -rf "$SERVICE_PATH"
fi

echo "📁 Creating service at: $SERVICE_PATH"

# Create the Automator workflow directory structure
mkdir -p "$SERVICE_PATH/Contents"
mkdir -p "$SERVICE_PATH/Contents/document.wflow"

# Create Info.plist for the service
cat > "$SERVICE_PATH/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.apple.Automator.Launch Anchor Selector</string>
    <key>CFBundleName</key>
    <string>Launch Anchor Selector</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
</dict>
</plist>
EOF

# Create the workflow document
cat > "$SERVICE_PATH/Contents/document.wflow" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>AMApplicationBuild</key>
    <string>523</string>
    <key>AMApplicationVersion</key>
    <string>2.10</string>
    <key>AMDocumentVersion</key>
    <string>2</string>
    <key>actions</key>
    <array>
        <dict>
            <key>action</key>
            <dict>
                <key>AMActionVersion</key>
                <string>2.0.3</string>
                <key>AMApplication</key>
                <array>
                    <string>Automator</string>
                </array>
                <key>AMParameterProperties</key>
                <dict>
                    <key>COMMAND_STRING</key>
                    <dict>
                        <key>tokenizedValue</key>
                        <array>
                            <string>$LAUNCHER_PATH</string>
                        </array>
                    </dict>
                    <key>CheckedForUserDefaultShell</key>
                    <true/>
                    <key>inputMethod</key>
                    <integer>0</integer>
                    <key>shell</key>
                    <string>/bin/bash</string>
                    <key>source</key>
                    <string></string>
                </dict>
                <key>AMProvides</key>
                <dict>
                    <key>Container</key>
                    <string>List</string>
                    <key>Types</key>
                    <array>
                        <string>com.apple.cocoa.string</string>
                    </array>
                </dict>
                <key>ActionBundlePath</key>
                <string>/System/Library/Automator/Run Shell Script.action</string>
                <key>ActionName</key>
                <string>Run Shell Script</string>
                <key>ActionParameters</key>
                <dict>
                    <key>COMMAND_STRING</key>
                    <string>$LAUNCHER_PATH</string>
                    <key>CheckedForUserDefaultShell</key>
                    <true/>
                    <key>inputMethod</key>
                    <integer>0</integer>
                    <key>shell</key>
                    <string>/bin/bash</string>
                    <key>source</key>
                    <string></string>
                </dict>
                <key>BundleIdentifier</key>
                <string>com.apple.RunShellScript</string>
                <key>CFBundleVersion</key>
                <string>2.0.3</string>
                <key>CanShowSelectedItemsWhenRun</key>
                <false/>
                <key>CanShowWhenRun</key>
                <true/>
                <key>Category</key>
                <array>
                    <string>AMCategoryUtilities</string>
                </array>
                <key>Class Name</key>
                <string>RunShellScriptAction</string>
                <key>InputUUID</key>
                <string>12345678-1234-1234-1234-123456789ABC</string>
                <key>Keywords</key>
                <array>
                    <string>Shell</string>
                    <string>Script</string>
                    <string>Command</string>
                    <string>Run</string>
                    <string>Unix</string>
                </array>
                <key>OutputUUID</key>
                <string>87654321-4321-4321-4321-CBA987654321</string>
                <key>UUID</key>
                <string>ABCDEF12-3456-7890-ABCD-EF1234567890</string>
                <key>UnlocalizedApplications</key>
                <array>
                    <string>Automator</string>
                </array>
                <key>arguments</key>
                <dict>
                    <key>0</key>
                    <dict>
                        <key>default value</key>
                        <string>cat</string>
                        <key>name</key>
                        <string>COMMAND_STRING</string>
                        <key>required</key>
                        <string>0</string>
                        <key>type</key>
                        <string>0</string>
                        <key>uuid</key>
                        <string>0</string>
                    </dict>
                </dict>
                <key>isViewVisible</key>
                <true/>
                <key>location</key>
                <string>449.000000:316.000000</string>
                <key>nibPath</key>
                <string>/System/Library/Automator/Run Shell Script.action/Contents/Resources/Base.lproj/main.nib</string>
            </dict>
        </dict>
    </array>
    <key>connectors</key>
    <dict/>
    <key>workflowMetaData</key>
    <dict>
        <key>serviceApplicationBundleID</key>
        <string></string>
        <key>serviceApplicationPath</key>
        <string></string>
        <key>serviceInputTypeIdentifier</key>
        <string>com.apple.Automator.nothing</string>
        <key>serviceOutputTypeIdentifier</key>
        <string>com.apple.Automator.nothing</string>
        <key>serviceProcessesInput</key>
        <integer>0</integer>
        <key>workflowTypeIdentifier</key>
        <string>com.apple.Automator.servicesMenu</string>
    </dict>
</dict>
</plist>
EOF

if [ -d "$SERVICE_PATH" ]; then
    echo "✅ Service created successfully!"
    echo ""
    echo "🎯 To set up F10 shortcut:"
    echo "1. Go to System Settings > Keyboard > Shortcuts"
    echo "2. Click 'Services' in the left sidebar"
    echo "3. Scroll down to find 'Launch Anchor Selector'"
    echo "4. Click 'Add Shortcut' and press F10"
    echo ""
    echo "🔄 You may need to log out and back in for the service to appear"
    echo ""
    echo "📝 Alternative: Use the installed app at /Applications/AnchorSelector.app"
else
    echo "❌ Service creation failed"
fi