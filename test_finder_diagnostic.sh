#!/bin/bash

echo "=== FINDER SELECTION DIAGNOSTIC ==="
echo ""

echo "1. Testing if Finder is running..."
osascript -e 'tell application "System Events" to (name of processes) contains "Finder"'
echo ""

echo "2. Testing basic selection retrieval..."
osascript -e 'tell application "Finder" to get selection'
echo ""

echo "3. Testing selection count..."
osascript -e 'tell application "Finder" to count of selection'
echo ""

echo "4. Testing the exact selection script from grabber.rs..."
osascript -e '
tell application "Finder"
    try
        set selectedItems to selection
        if (count of selectedItems) > 0 then
            set selectedItem to item 1 of selectedItems
            set thePath to POSIX path of (selectedItem as alias)
            set itemClass to class of selectedItem
            if itemClass is folder then
                set isFolder to "true"
            else
                set isFolder to "false"
            end if
            return thePath & "|" & isFolder
        else
            return "--NO-SELECTION--"
        end if
    on error errorMessage
        return "--ERROR: " & errorMessage & "--"
    end try
end tell
'
echo ""

echo "5. Testing window path retrieval..."
osascript -e '
tell application "Finder"
    try
        if (count of windows) > 0 then
            set frontWindow to front window
            set windowPath to (POSIX path of (target of frontWindow as alias))
            return windowPath
        else
            return "--ERROR--"
        end if
    on error
        return "--ERROR--"
    end try
end tell
'
echo ""

echo "=== END DIAGNOSTIC ==="