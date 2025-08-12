#!/bin/bash

echo "=== Obsidian Spinning Ball Diagnostic ==="
echo "Run date: $(date)"
echo ""

echo "1. Checking for any hook:// URL registrations:"
/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister -dump 2>/dev/null | grep -i "hook" | wc -l

echo ""
echo "2. Checking Launch Services for hook references:"
defaults read com.apple.LaunchServices/com.apple.launchservices.secure 2>/dev/null | grep -i hook || echo "None found"

echo ""
echo "3. Looking for apps with hook in Info.plist:"
mdfind "kMDItemContentType == 'com.apple.application-bundle'" 2>/dev/null | while read app; do
  if plutil -p "$app/Contents/Info.plist" 2>/dev/null | grep -q "hook"; then
    echo "Found: $app"
  fi
done

echo ""
echo "4. Checking if Obsidian is running:"
ps aux | grep -i obsidian | grep -v grep || echo "Not running"

echo ""
echo "5. Checking system log for Obsidian issues in last 5 minutes:"
log show --last 5m --predicate 'process == "Obsidian"' 2>/dev/null | grep -E "hang|timeout|slow|spin|block|wait|URL|scheme" | tail -10

echo ""
echo "6. Checking for zombie or stuck processes:"
ps aux | grep -E "defunct|<defunct>" 

echo ""
echo "7. Memory pressure:"
vm_stat | grep -E "Pages free|Pages active|Pages inactive|Pages wired"

echo ""
echo "8. Disk I/O issues:"
iostat -c 2 -w 1 | tail -1

echo ""
echo "=== End Diagnostic ==="