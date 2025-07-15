#!/usr/bin/osascript

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