-- AppleScript to handle hook:// URLs
-- This script receives Apple Events for hook:// URLs and delegates to the HookAnchor binary

on open location theURL
	-- Try to find the HookAnchor binary in common locations
	set possiblePaths to {¬
		"/Applications/HookAnchor.app/Contents/MacOS/ha", ¬
		"/usr/local/bin/ha", ¬
		(do shell script "echo $HOME") & "/bin/ha", ¬
		(do shell script "echo $HOME") & "/.local/bin/ha"}
	
	set haPath to ""
	repeat with currentPath in possiblePaths
		try
			do shell script "test -f '" & currentPath & "'"
			set haPath to currentPath
			exit repeat
		end try
	end repeat
	
	-- Fallback to PATH lookup
	if haPath is "" then
		try
			set haPath to do shell script "which ha"
		on error
			display dialog "HookAnchor binary not found. Please ensure it's installed." buttons {"OK"} default button 1
			return
		end try
	end if
	
	-- Execute the HookAnchor binary with the URL
	do shell script "'" & haPath & "' '" & theURL & "'"
end open location