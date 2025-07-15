#!/usr/bin/osascript

on run
    log "=== FINDER SELECTION TEST ==="
    
    tell application "Finder"
        -- Test 1: Basic selection test
        log "Test 1: Getting selection..."
        try
            set sel to selection
            log "Selection count: " & (count of sel)
            
            if (count of sel) > 0 then
                repeat with i from 1 to count of sel
                    set selectedItem to item i of sel
                    log "Item " & i & ": " & (name of selectedItem)
                    log "Class: " & (class of selectedItem as string)
                    log "POSIX path: " & (POSIX path of (selectedItem as alias))
                end repeat
            else
                log "No items selected"
            end if
        on error errMsg
            log "Error getting selection: " & errMsg
        end try
        
        log ""
        
        -- Test 2: Get first selected item details
        log "Test 2: First selected item details..."
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
                log "Result: " & thePath & "|" & isFolder
            else
                log "Result: --NO-SELECTION--"
            end if
        on error errorMessage
            log "Result: --ERROR: " & errorMessage & "--"
        end try
        
        log ""
        
        -- Test 3: Get front window path
        log "Test 3: Front window path..."
        try
            if (count of windows) > 0 then
                set frontWindow to front window
                set windowPath to (POSIX path of (target of frontWindow as alias))
                log "Window path: " & windowPath
            else
                log "No windows open"
            end if
        on error errMsg
            log "Error getting window path: " & errMsg
        end try
    end tell
    
    log "=== END TEST ==="
end run