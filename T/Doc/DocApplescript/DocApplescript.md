.[[DocApplescript]].
  , ,
  , ,
  , ,
  , [DocApplescript Obsidian](spot://docapplescriptobsidian),
  DELS: ,[DocApplescript Obsidian](spot://docapplescriptobsidian),
  , ,
  , [DocApplescript Note](spot://docapplescriptnote), 
  DELS: [DocApplescript Note](spot://docapplescriptnote),[DocApplescript Obsidian](spot://docapplescriptobsidian),[DocApplescript Note](spot://docapplescriptnote),[DocApplescript Note](spot://docapplescriptnote),

  [__DocApplescript__](__DocApplescript__.md)




https://developer.apple.com/library/mac/#documentation/applescript/conceptual/applescriptlangguide/reference/ASLR_classes.html%23//apple_ref/doc/uid/TP40000983-CH1g-SW2


# DOCS
  pdf          file://ob/data/docs/applescript/AppleScriptLanguageGuide.pdf  
             http://developer.apple.com/library/mac/#documentation/AppleScript/Conceptual/AppleScriptX/Concepts/osa.html#//apple_ref/doc/uid/TP40001571-BABEBGCF
conceptual   http://developer.apple.com/library/mac/#documentation/AppleScript/Conceptual/AppleScriptX/Concepts/work_with_as.html#//apple_ref/doc/uid/TP40001568-BABEBGCF
osx dev lib  http://developer.apple.com/library/mac/navigation/
wikipedia    http://en.wikipedia.org/wiki/AppleScript
index        http://developer.apple.com/library/mac/#documentation/AppleScript/Conceptual/AppleScriptLangGuide/reference/ASLR_classes.html#//apple_ref/doc/uid/TP40000983-CH1g-246384
osascript

# [TOPLEVEL]
#!/usr/bin/
return "yo"



 a.scpt:
    on run argv
      display alert "Insert"
      return "hello, " & item 1 of argv & "."
    end run

  % osascript a.scpt world
    hello, world.
delay 2

osascript -e 'tell app "Address Book" to get the name of every person' | perl -pe 's/, /\n/g' | sort | uniq -d

# [CONTROL]
osascript [-l language] [-s flags] [-e statement | programfile] [argument ...]
global foo \..\ set foo to 2 \..\ on myFn(x,y) \..\ if foo<3 return x+y \..\ end myFn
if x < 1000 then set x to x + 1
if x is greater than 3 then \..\ else \..\ end if
repeat              \..\  end repeat
repeat 10 times     \..\  end repeat
repeat while x > 0  \..\  end repeat
repeat until x ≤ 0  \..\  end repeat
repeat with i from 100 to 25 [by -25]   \..\  end repeat
repeat with x in {1, 2, 3, 4, 5}        \..\  end repeat

try \..\ on error \..\ end try
on mySubroutine(parameters...)  \..\  end myfunction


## Control Special

-- Runs when script is done.  waits 60 seconds before running again
on idle /../  return 60 /../ end idle

on quit
     --commands to execute before the script quits
  continue quit -- required for the script to actually quit
end quit

-- multi line comment
#  another comment
(*  Here are some
    --nested comments
    (* another comment within a comment *)
#

## Functions

on mySubroutine(parameters...) \..\ end myfunction


-- use my or me to call function from within a block
Tell application "Finder"
    set anyNumber to my (random number from 5 to 50)
    set x to my myHandler()
    -- or
    set x to myHandler() of me
end tell
 
on myHandler()
    --commands
end myHandler


## Variables
global foo
set variable1 to 1 -- create an integer variable called variable1
set variable2 to "Hello" -- create a text variable called variable2
copy {17, "doubleday"} to variable3 -- create a list variable called variable3
set {variable4 , variable5} to variable3 -- copy the list items of variable3 into separate variables variable4 and variable5
set variable6 to script myScript -- set a variable to an instance of a script

# DATASTRUCTURES

## String

"it" & "'s"

## List

{1, 7, "Beethoven", 4.5}


class of {"this", "is", "a", "list"} --result: list
item 3 of {"this", "is", "a", "list"} --result: "a"
items 2 thru 3 of {"soup", 2, "nuts"} --result: {2, "nuts"}
{"This"} & {"is", "a", "list"} --result: {"This", "is", "a", "list"}

## Map
{product:"pen", price:2.34}

# GET / SET some kind of info
## Users Home Folder
set myPath to (path to home folder) as string

## Access Application by Generic Name.  Access GUI elements
tell application "Google Chrome"
    activate
    tell application "System Events"
        tell application process "Google Chrome"
            get value of text field 1 of tool bar 1 of window 1
        end tell
    end tell
  end tell

# Action
## Action Dialog

set alertResult to display alert "Insert generic warning here." ¬
    buttons {"Cancel", "OK"} as warning ¬
    default button "Cancel" cancel button "Cancel" giving up after 

## Shell script execution
on log_event(themessage)
  set theLine to (do shell script ¬
   "date  +'%Y-%m-%d %H:%M:%S'" as string) ¬
   & " " & themessage
  do shell script "echo " & theLine & ¬
   " >> ~/Library/Logs/AppleScript-events.log"
end log_event

# Action Tell
-- Simple form
tell application "Safari" to activate
 
-- Compound
tell application "MyApp"
     -- commands for app
end tell

# BLAM

[CONTROL]

[EXECUTE]
tell application "Finder" to open POSIX file "/ob/bin/zzz.txt" --or--
tell application "Finder" open file "Myfile.txt" of desktop   --or--
do shell script "open ~/desktop/myfile.txt"                   --or--
tell application "myNonScriptableApp" to open posix file "/Users/me/desktop/myfile.txt"


[VARIABLES]
set myWidth to 1280
set front_app to (path to frontmost application as Unicode text)

[ACTIONS]
  beep
  say "hello"
set myHeight to 800

set front_app to (path to frontmost application as Unicode text)
tell application "Spotify"
	activate
	set bounds of window 1 to {0, (myHeight / 2), myWidth, myHeight}
end tell

tell application “iTunes”
	set y to (get index of last user playlist)
	repeat with i from y to 1 by -1
		try
			set thisPlaylist to user playlist i
			if special kind of thisPlaylist is none then
				if not (exists track 1 of thisPlaylist) then
					delete thisPlaylist
				end if
			end if
		end try
	end repeat
end tell


if ageOfCat > 1 then display dialog "This is not a kitten."
if currentTemp < 60 then
    set response to "It's a little chilly today."
else if currentTemp > 80 then
    set response to "It's getting hotter today."
else
    set response to "It's a nice day today."
end if
repeat
    -- perform operations
    if someBooleanTest then
        exit repeat
    end if
end repeat


# Commands
## List
  http://developer.apple.com/library/mac/navigation/
  http://developer.apple.com/library/mac/#documentation/AppleScript/Conceptual/AppleScriptLangGuide/reference/ASLR_cmds.html#//apple_ref/doc/uid/TP40000983-CH216-SW59
run \\ launch \\ activate \\ beep \\ delay \\ display dialog/alert


===APPLESCRIPT SUITE===
activate                    | Brings an application to the front, and opens it if it is on the local computer and not already running.
log                         | In Script Editor, displays a value in the Event Log History window or in the Event Log pane of a script window.

===CLIPBOARD COMMANDS SUITE===
clipboard info              | Returns information about the clipboard.
set the clipboard to        | Places data on the clipboard.
the clipboard               | Returns the contents of the clipboard.

===FILE COMMANDS SUITE===                                        
info for                    | Returns information for a file or folder.
list disks                  | Returns a list of the currently mounted volumes.
                            | Deprecated Use tell application "System Events" to get the name of every disk.
list folder                 | Returns the contents of a specified folder.
                            | Deprecated Use tell application "System Events" to get the name of every disk item of ....
mount volume                | Mounts the specified AppleShare volume.
path to (application)       | Returns the full path to the specified application.
path to (folder)            | Returns the full path to the specified folder.
path to resource            | Returns the full path to the specified resource.


===FILE READ/WRITE SUITE===       
close access                | Closes a file that was opened for access.
get eof                     | Returns the length, in bytes, of a file.
open for access             | Opens a disk file for the read and write commands.
read                        | Reads data from a file that has been opened for access.
set eof                     | Sets the length, in bytes, of a file.
write                       | Writes data to a file that was opened for access with write permission.
Internet suite
open location               | Opens a URL with the appropriate program.


===MISCELLANEOUS COMMANDS SUITE===
current date                | Returns the current date and time.
do shell script             | Executes a shell script using the sh shell.
get volume settings         | Returns the sound output and input volume settings.
random number               | Generates a random number.
round                       | Rounds a number to an integer.
set volume                  | Sets the sound output and/or input volume.
system attribute            | Gets environment variables or attributes of this computer.
system inf                  | Returns information about the system.
time to GMT                 | Returns the difference between local time and GMT (Universal Time).


====SCRIPTING SUITE===            
load script                 | Returns a script object loaded from a file.
run script                  | Runs a script or script file

scripting components          Returns a list of all scripting components.
store script                | Stores a script object into a file.

Standard suite              
copy                        | Copies one or more values into variables.                                             
count                       | Counts the number of e| 
get                         | Returns the value of a script expression or an application object.
launch                      | Launches the specified application without sending it a run command.
run                         | For an application, launches it. For a script application, launches it and sends it the run command. For a script script object, executes its run handler.
set                         | Assigns one or more values to one or more script variables or application objects.


===STRING COMMANDS SUITE===
ASCII character             | Converts a number to a character.
                              Deprecated starting in AppleScript 2.0. Use the id property of the text class instead.                                             
ASCII number                | Converts a char| 
                              Deprecated starting in AppleScript 2.0. Use the id property of the text class instead.                                             
localized string            | Returns the localized string for the specified key.
offset                      | Finds one piece of text inside another.
summarize                   | Summarizes the specified text or text file.

===USER INTERACTION SUITE===
beep                        | Beeps one or more times.
choose application          | Allows the user to choose an application.
choose color                | Allows the user to choose a color.
choose file                 | Allows the user to choose a file.
choose file name            | Allows the user to specify a new file reference.
choose folder               | Allows the user to choose a folder.
choose from list            | Allows the user to choose one or more items from a list.
choose remote application   | Allows the user to choose a running application on a remote machine.                                             
choose URL                  | Allows the user to specify a URL.
delay                       | Pauses for a fixed amount of time.
display alert               | Displays an alert.
display dialog              | Displays a dialog box, optionally requesting user input.
say                         | Speaks the specified text.

## tell
tell application "Finder"
    keystroke "#" using {command down, shift down}
    set frontWindowName to name of front window
    
    -- any number of additional statements can appear here
end tell


tell front window of application "Finder" to close

tell application "Finder"
    set frontWindowName to name of front window
    -- any number of additional statements can appear here
end tell

tell application "Finder"
    close front window
end tell
 
tell front window of application "Finder" to close
tell application "Finder" to close front window

tell application "Finder"
    tell document 1 of application "TextEdit"
        set newName to word 1 -- handled by TextEdit
    end tell
    set len to count characters in newName -- handled by AppleScript
    if (len > 2) and (len < 15) then -- comparisons handled by AppleScript
        set name of first item of disk "HD" to newName -- handled by Finder
    end if
end tell

### with timeout
tell application "TextEdit"
    with timeout of 20 seconds
        close document 1 saving ask
    end timeout
end tell

### with transaction
tell application "TextEdit"
    with timeout of 20 seconds
        close document 1 saving ask
    end timeout
end tell

### creating a session
tell application "TextEdit"
    with timeout of 20 seconds
        close document 1 saving ask
    end timeout
end tell

# Emamples
## Output
display dialog "Hello, world!" -- a modal window with "Ok" and "Cancel" buttons (you can customize the buttons)
-- or
display alert "Hello, world!"  -- a modal window with a single "Ok" button
-- or
say "Hello, world!" -- an audio message using a synthesized computer voice
-------------

## Input
-- Dialog
set dialogReply to display dialog ¬
        "Dialog Text" default answer ¬
        "Text Answer" hidden answer false ¬
        buttons {"Skip", "Okay", "Cancel"} ¬
        default button ¬
        "Okay" cancel button ¬
        "Skip" with title ¬
        "Dialog Window Title" with icon note ¬
        giving up after 20
------------
--Choose from list
set chosenListItem to choose from list {"A", "B", "3"} ¬
        with title  "List Title" ¬
        with prompt "Prompt Text" ¬
        default items "B" ¬
        OK button name "Looks Good!" ¬
        cancel button name "Nope, try again" ¬
        multiple selections allowed false ¬
        with empty selection allowed
-----------
--Alert
set resultAlertReply to display alert ¬
        "Alert Text" as warning ¬
        buttons {"Skip", "Okay", "Cancel"} ¬
        default button 2 ¬
        cancel button 1 ¬
        giving up after 2
-----------
--Dialog
display alert "Hello, world!" buttons {"Rudely decline", "Happily accept"}
set theAnswer to button returned of the result
if theAnswer is "Happily accept" then
        beep 5
else
        say "Piffle!"
end if

## String Manipulation
set str to "python /ob/key/_launcher.py " & "foo"
-----------
# String stuff
on trim_line(this_text, trim_chars, trim_indicator)
 -- 0 = beginning, 1 = end, 2 = both
 set x to the length of the trim_chars
 -- TRIM BEGINNING
 if the trim_indicator is in {0, 2} then
 repeat while this_text begins with the trim_chars
 try
 set this_text to characters (x + 1) thru -1 of this_text as string
 on error
 -- the text contains nothing but the trim characters
 return ""
 end try
 end repeat
 end if
 -- TRIM ENDING
 if the trim_indicator is in {1, 2} then
 repeat while this_text ends with the trim_chars
 try
 set this_text to characters 1 thru -(x + 1) of this_text as string
 on error
 -- the text contains nothing but the trim characters
 return ""
 end try
 end repeat
 end if
 return this_text
end trim_line

## Calling from external
---------------------
# ESCAPING APPLESCRIPT (avoid:    execution error: No user interaction allowed. (-1713))
#!/bin/bash
x=`/usr/bin/osascript <<EOT
tell application "Finder"
    activate
    set myReply to text returned of (display dialog "Here is a question" default answer "Here is a reply")
end tell
EOT`
echo $x


# RUBY CODE
def main
  puts "Hello World.  Args=#{ARGV.inspect}"
  x=`/usr/bin/osascript <<EOT
  tell application "Finder"
      activate
      set myReply to text returned of (display dialog "Here is a question" default answer "Here is a reply")
  end tell
  `
  puts "x=#{x}"
  
end



to notester(name)
	tell application "Keyboard Maestro Engine"
		make variable with properties {name:"test", value: name}
		do script "Nstr_Goto"
	end tell
end notester

notester("top")





# zzzz ** Misc
---------------------

tell document "/Users/oblinger/ref/todo.org"
	activate
	# keystroke "X"
	# keystroke "ref"
	
end tell




tell document "/Users/oblinger/ref/todo.org"
	activate
	# keystroke "X"
	# keystroke "ref"
	
end tell


---------------------------
# CONTROLLING AN i-TERM

-- A sample iTerm Applescript


tell application "iTerm"
	activate

	-- close the first session
	terminate the first session of the first terminal

	-- make a new terminal
	set myterm to (make new terminal)

	-- talk to the new terminal
	tell myterm

		-- set size
		set number of columns to 100
		set number of rows to 50
		-- make a new session
		set mysession to (make new session at the end of sessions)

		-- talk to the session
		tell mysession

			-- set some attributes
			set name to "tcsh"
			set foreground color to "red"
			set background color to "blue"
			set transparency to "0.6"

			-- execute a command
			exec command "/bin/tcsh"

		end tell -- we are done talking to the session

		-- we are back to talking to the terminal

		-- launch a default shell in a new tab in the same terminal
		launch session "Default Session"

		-- launch a saved session from the addressbook.
		launch session "Root Shell"
		-- select the previous session
		select mysession
		-- get the tty name of a session
		set myttyname to the tty of the first session
		-- refer to a session by its tty/id
		tell session id myttyname
		    set foreground color to "yellow"
		end tell

	end tell

	-- talk to the first terminal
	tell the first terminal

		-- launch a default shell in a new tab in the same terminal
		launch session "Default Session"

		tell the last session

			-- write some text
			write text "cd Projects/Cocoa/iTerm"
			-- write the contents of a file
			write contents of file "/path/to/file/"

		end tell

	end tell

	-- reposition window and name it
	set the bounds of the first window to {100, 100, 700, 700}
	set the name of the first window to "A Window Title"


end tell



-----------------------
=== NEXT THREE ARE COPY RIGHTED ====   (asshole)

This page holds a number of AppleScript examples that may prove useful to AppleScript learners.

Reading a Text file Into an AppleScript Variable

This is one of those things that you'll probably do a lot. It's not too hard in AppleScript but there are a few tricks. Note that there are a wide variety of ways to read from a file, such as reading in a certain number of characters, or until a certain character is encountered. I generally prefer to read the entire file into a variable and work from there for speed and simplicity. Here's how to do it:

set theFile to (choose file with prompt "Select a file to read:" of type {"TEXT"})
open for access theFile
set fileContents to (read theFile)
close access theFile
Writing an AppleScript Variable to a Text File

Another common task. Dead simple in HyperCard, but there are a few tricks in AppleScript. To open and write to a file, use this:

set newFile to new file with prompt "Output file:" default name "My New File"
open for access newFile with write permission
-- if you want to overwrite an existing file use set eof of newFile to 0 first.
write "something useful" to newFile
close access newFile
Drag 'n' Drop Applet Shell

Here is the skeleton of a script designed to function as a 'droplet' (a drag 'n' drop AppleScript application). When double-clicked the script begins executing from the first line. When files/folders are dropped onto the application icon the "on open" handler is called. This example simply passes on the object to "DisplayName", which displays the path name of each selected file or folder (and its entire contents). You can download this script as an AppleScript application here.

displayName(choose file with prompt "Select a file:") --if double-clicked
return -- not needed, but shows that the script stops here when "run"

on open of finderObjects -- "open" handler triggered by drag'n'drop launches
  repeat with i in (finderObjects) -- in case multiple objects dropped on applet
    displayName(i) -- show file/folder's info
    if folder of (info for i) is true then -- process folder's contents too
      tell application "Finder" to set temp to (entire contents of i)
      repeat with j in (temp)
        display dialog j as string -- example of doing something with each item
      end repeat
    end if
  end repeat
end open
Finding the Location of a File

This is a useful process. Often you will want to take a file, process it somehow and create a new file in the same location. There are two general ways to get the location of a file.

First you can use a one line "tell" statement to the Finder to get the file or folder's 'container'. This is very compact, but slow if you are processing a lot of files. Here is an example:

set pathToMe to (choose file with prompt "Display path of:")
display dialog GetParentPath(pathToMe)

on GetParentPath(theFile)
  tell application "Finder" to return container of theFile as text
end GetParentPath
You can also take the file's path name and use delimiters to determine the location component of a file's path. This is fast but complicated. Here is an example:

display dialog GetParentPath((choose file with prompt "Display path of:"))

on GetParentPath(myPath)
  set oldDelimiters to AppleScript's text item delimiters -- always preserve original delimiters
  set AppleScript's text item delimiters to {":"}
  set pathItems to text items of (myPath as text)
  if last item of pathItems is "" then set pathItems to items 1 thru -2 of pathItems -- its a folder
  set parentPath to ((reverse of the rest of reverse of pathItems) as string) & ":"
  (* The above line works better than the more obvious set parentPath to ((items 1 thru -2 of pathItems) as string) & ":"
     because it will not return an error when passed a path for a volume, i.e., "Macintosh HD:", but rather will return ":"
     indicating the desktop is the root of the given path. Andy Bachorski <andyb@APPLE.COM> *)
  set AppleScript's text item delimiters to oldDelimiters -- always restore original delimiters
  return parentPath
end GetParentPath
© Ben Lawson 1998. Feedback to ben@nobleswan.com. Click here to return to my home page.
