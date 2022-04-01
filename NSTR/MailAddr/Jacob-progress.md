# MailAddr.Jacob-progress --

    Jacob, 
     
    I have a new version of the Mail Assistant, it is included as an 
    attachment to this message.  The lines in UPPERCASE below are my notes on 
    features added, I annotated these with a one sentence description. 
    There is no need to go through the list, we can chat about those items with 
    outstanding issues at our next meeting.  I will schedule a meeting 
    toward the end of this month.   
     
    Several of your requests have been incorporated, and a couple will be  
    added next.  And several require discussion.  Specifically:  
     
     
    . ADDED NICKNAMES 
      (This was an amayzingly easy change.  Now each word is replaced by 
       all varients according to the nicknames file.) 
    . PRIORITIZE DISPLAY BY NAME FREQUENCY 
      (I almost completed this item.  I need an additional half day to complete.) 
    . AUTO-PASTE 
      I never thought about leaving the assistant running.  I have changed it 
      so that it automatically updates the clipboard at all times 
      (not just when you press OK), thus you can simply iconify it and paste 
      the current contents.  Later you can do another search and paste those 
      results.  The OK button is now replaced by a close button since that 
      is closer to it current function. 
     
     
    Peter told me you had a request regarding a visitor to Watson. 
    I would be glad to assist if needed. 
     
    Cheers, 
    Dan 
     
     
     
    ------------- 
     
     
    [[NEW FEATURES]] 
     
    . SIMPLE ENVIRONMENT ANALYZER LAUNCHER 
      (Wrote simple C program that creates correct 'path' and verifies 
       that notes is alive and safe to access before Java is run. 
       NOTE: this allows me to catch errors, and setup parts of the environment 
       that are out of java's control) 
    . SINGLE .EXE INSTALLER 
      (Used program "PACKAGE FOR WEB" to make installer that can be 
       emailed.  Tim Dinger wanted a version that could be passed by email) 
    . ADDED VERBOSE ERROR MESSAGES FOR SEVERAL SITUATIONS WHERE 
      the user has an incorrect version, or incorrect location document, etc. 
      When possible these message tell the user steps to fix the problem. 
     
     
     
    [[NEW DETAILED FEATURES]] 
     
    .  EXPLICIT VERSION CHECKING 
      (Since the datafile format is evolving I must verify that the newly 
       installed MailAssistant is compatible with the existing datafile 
       cached on the client machine.) 
    .  U.COLUMN DOES NOT RESIZE 
      (screen now resizes; although it does not change number of line in 
       each display;  JACOB: This could be added w. 2hrs work) 
    .  ADD LABELS TO WINDOW 
      (title each region so user knows their purpose) 
    .  ADD CLOSE BUTTON CONTROL 
      (the X widow handle eXits) 
    .  NAME WHEN MINIMIZED 
      (name displayed when iconified) 
    .  NAMES; ASSOCIATED NAMES; MESSAGES; CANCEL; OK 
      (Changed names to make GUI look like a windows dialog box) 
    .  CHANGE BACKGROUND AND ADD SPACERS 
      (Simple reshaping of gaps and placements on interface. 
       Change color.) 
    .  DON'T DO NULL SEARCHES 
      (Empty string does not do full retrieval, now "*" does) 
    .  CLICK INSERTS WHEN "SUB" 
      (Remove restriction against pasting subject lines) 
    .  COUNT NUMBER OF ENTRIES NOT MAIL MESSAGES WHILE BUILDING CACHE 
      (This is the number of entries actually being searched) 
    .  FORCE REBOOT 
      (Force install shield to reboot after install.  Sometimes windows will 
       ignore a new shortcut key (Cntrol-Alt-A) untill the next reboot.  Gross.) 
    .  CHANGE LOADER TO DBG.LOG 
      (Now all debugging info from the C loader the Java program and the Notes 
       interface, all write to the same log file, and that log file is accessible 
       from the start menu.) 
    .  DEFAULTS 
      (Added default MaxSize, and changed the default display mode to NAMES.) 
    .  JACOB CHANGES  
      (Changed interface to say "colleagues", and seperated the Email headers 
       option from the other two) 
    .  SPLASH SCREEN 
      (Changed installation so that the initial caching is done at install  
       time, rather than when the tool is first used.) 
     
     
     
     
    [[BUG FIXES]] 
     
    .  JACOB'S ERROR 
      (Notes locks up when one attempts to use the java interface on 
       a version prior to 4.6.  Now I explicitly check that a new enough 
       verion is being used.) 
    .  OUT OF MEMORY CRASHES W/O PRINTING ERROR MESSAGE 
      (All info in now captured in a log that is controlled by the C-loader 
       program, so that crashes that catch java before it can react will 
       still be logged.  Such error messages will only occur on computers 
       whose disks are so full that no virtual memory is available.) 
    .  FIX CLIPBOARD PASTE WHEN DIFFERENT TYPE EXISTS (ASK PETER) 
       (I was using the clipboard incorrectly.  It would fail if a 
        non-text object was currently on the clipboard.) 
    .  APP LOCKS UP.  TO REPODUCE: STARTUP; TYPE 'EMAIL'; PRESS EMAIL 
       (Switched type of EXEC call being made, I think that fixed it.) 
    .  SUBMIT BUGS REPORT FAILS 
       (Fixed) 
    .  NOTES LOCAL SERVER LOCKS UP.  (WHEN SERVER IS ACCESSED TWICE AND THE  
       FIRST ACCESS WAS IMMEDIATELY AFTER INSTALLATION) 
      (Jacob: I have know idea what is up with this.  I added a workaround 
       to the code to avoid the situation, but it worries me, I cannot 
       guarantee there aren't other circumstances where it will occur, since 
       I do not know the cause.  If related bugs are found in the future, I 
       will try to boil the problem down into a very simple test program  
       that generates the failure and take it to Lotus.) 
    .  MAXSIZE BUG 
      (System ignored the default value I added to that field) 
     
     
     
    [[THINGS THAT WILL OR COULD BE ADDED]] 
     
    . CANONICAL EMAIL ADDRESS (REMOVE "@IBMUS") 
      (Many addresses could be simplified.  I can write a simple rewrite system 
       that loads a set of rules from a file.  Before I do this we should  
       write a list of rewrite rules currently needed so I do go overboard 
       in generality on that rewrite system.) 
    . MANY ASPECTS OF THE SYSTEM (I.E. DEFAULT MAXSIZE) SHOULD BE UNDER 
      user control. I can add a screen of configuration settings and  
      and .ini file reading and writing for these parameters.) 
    . CACHE MULTIPLE DBS  (Let user explicitly add DB's to the MailAssistant's 
      search space.) 
    . PRIORITIZE DISPLAY BY NAME FREQUENCY 
      (I almost completed this item.  I need an additional half day to complete.) 
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
    . DISPLAYING/SORTING AND SELECTING BY DATE. 
      (This will take three days or so, since I need to modify the basic data 
       collected, saved, stored, manipulated.  I will also need to incorporate 
       code to parse, display, sort, and select by date. 
    . SEE THE EMAIL HEADER EASILY. 
      (Click on entry to see Email header easily) 
     
     
     
     
