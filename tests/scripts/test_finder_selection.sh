#!/bin/bash

echo "Testing Finder selection AppleScript..."

# Test the selection script
echo "=== Testing selection script ==="
osascript -e '
tell application "Finder"
    try
        set selectedItem to item 1 of (get selection)
        if selectedItem is not missing value then
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
echo "=== Testing simplified selection script ==="
osascript -e '
tell application "Finder"
    try
        set selectedItems to selection
        if (count of selectedItems) > 0 then
            set firstItem to item 1 of selectedItems
            return (POSIX path of (firstItem as alias))
        else
            return "NO-SELECTION"
        end if
    on error errorMessage
        return "ERROR: " & errorMessage
    end try
end tell
'