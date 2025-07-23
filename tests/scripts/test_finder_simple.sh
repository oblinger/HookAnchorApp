#!/bin/bash

echo "Testing Finder selection..."

# Test 1: Get selection
echo "Test 1: Get selection"
osascript -e 'tell application "Finder" to get selection'

echo ""
echo "Test 2: Get selection as POSIX path"
osascript -e 'tell application "Finder"
    set sel to selection
    if (count of sel) > 0 then
        return POSIX path of (item 1 of sel as alias)
    else
        return "NO SELECTION"
    end if
end tell'

echo ""
echo "Test 3: Get front window path"
osascript -e 'tell application "Finder"
    if (count of windows) > 0 then
        return POSIX path of (target of front window as alias)
    else
        return "NO WINDOWS"
    end if
end tell'