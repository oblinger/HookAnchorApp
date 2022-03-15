# MailAddr.Todos --



    [[TODO]] 
     
    . CANONICAL EMAIL ADDRESS (REMOVE "@IBMUS") 
    . MANY ASPECTS OF THE SYSTEM (I.E. DEFAULT MAXSIZE) SHOULD BE UNDER 
    . CACHE MULTIPLE DBS 
    . PRIORITIZE DISPLAY BY NAME FREQUENCY 
     
    - Error msg about location document should also mention connetion 
      to server as possible error (check if accessible thru notes) 
     
    LATER 
    . Submit bugs report fails when copy fails 
    . DISPLAYING/SORTING AND SELECTING BY DATE. 
    . SEE THE EMAIL HEADER EASILY. 
    - Text windows do not resize 
    - Dates 
    . Troubleshoot NT 
    . Cache multiple DBs 
    . See the email header easily. 
    t Fix clipboard paste when different type exists (ASK PETER) 
     
    BUGS 
    - Shortcut not always added. 
    . RELABLE WINDOWS SHORTCUTS 
      (I use install shield to add the Control-Alt-A shortcut to start the app. 
       This is added but still doesn't work on some computers.  I don't yet 
       know why.) 
    . SUPPORT NT 
      (Notes interface fails on NT.  I have spent a few hours trying to  
       track this down, but failed.  I am guessing the problem is a simple 
       difference between notes on NT and on '95.  Its a matter of narrowing 
       down the space of possibilities and then working around the difference) 
    . ROB'S BUG: LOADING NOTES OVER NEWER VERSION 
      (The only bug I know of that is not handled:  You can confuse the loader 
       about what version is currently loaded by first installing an NEWER 
       verion of notes, and then installing a OLDER verions in the same 
       directory without uninstalling the NEWER verion first.  NOTE: 
       I don't want to ask notes what version it is since this test is 
       designed to verify that it is safe to talk with notes.  I have thought of 
       a fix for this problem, we just need to see if it is worth implementing.) 
     
     
    =============================================================================== 
     
    PRESUMED DONE 
    - NotesException.outOfMemory  (Leaves error in log) 
    . App locks up: startup; type 'email'; press email 
      (verify that "in CC..." has printed) 
    . Verbose; trouble shooting error messages 
     
    DONE 
    - Jacob's error 
    - Simple Environment analyzer launcher 
    - single .exe installer 
    . U.COLUMN does not resize 
    . Add labels to window 
    . Add close button control 
    . Name when minimized 
    . Names; Associated Names; Messages; CANCEL; OK 
    . Change background and add spacers 
    ? Don't do null searches 
    ? Click inserts when "SUB" 
    . Count number of entries not mail messages while building cache 
    X Out of memory crashes w/o printing error message 
    X RE-BUILD CACHE, [xx search by name] 
    t Force reboot 
    . change loader to dbg.log 
    . Add shortcut to view log. 
    - Add nickname support 
     
