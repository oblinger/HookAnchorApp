# WMRdiag.Prime --

    - Cancel during print causes failure 
     
    === PROPOSED === 
     
    > FLEXIBLE PARAMS 
      - Register a configuration panel. 
      - Clone display objs. (when control changed)  
      - Options screen 
     
    1 Auto screen size (per screen type) 
    . Screen inherits top object's background 
    . Parse fails when not keys are defined 
    . Simple Click to other screen 
    - Add Attributes: TotalTime 
    > Popup help 
     
    5 7/2 - Selection methods: by date, by person, by book 
     
    3 4/1 - Attributes: Accuracy, Word groups, TimePerWord, NumberOfUtterances 
                    TotalTime, etc. 
     
     
     
     
    KNOWN BUGS: 
    - memory error ??  
    - Screen refresh bug (when in the background after emacs is selected) 
     
    ACCEPTED BUGS: 
    - Screen flashes during init and printing 
     
     
     
     
     
     
     
    =============================================================================== 
    DONE: 
    # 3 - WMR Datafile parsing 
    # 4 - Master word dictionary & id translator 
    # 4 - MainDB code, Caching? 
    # 4 - WordList Screen (really depends on how efficient/tricky I get) 
    # 4 - Toplevel: LocationMerging, Context sensitive clicks, etc. 
    # 3/1 - Build configuration files 
    # 3 PRINTER SUPPORT:  
      # Print arbitrary group of displays to printer 
      # Print screen button 
      # Screen that prints entire list 
    # Force bar graphs to be ``in range'' 
    # Center titles 
    # Window controls (iconify, etc.) 
    # Word List (all lowercase except ``i'' 
    # Move locator flush left 
    # Remove numRows from non-container objects 
    # center titles & offset captions 
    # Paging the word list (``<'', and  ``>''  buttons) 
    # locator to left 
    # printer popup menu 
    # add window controls 
    # Computing and displaying data certainty 
    # Compute label sizes from widths 
    # Resize screen info propagation 
    # Build printer menu 
    PERFORMED IN HOUSTON 
    # Get word lists from teachers 
    # Fix init file 
    # Fix screen layouts in conf file 
    # look @ db data 
    # Dates: remove from screen (do fixed date ranges on displays) 
    # Get data for demo setup 
    # Multiplexer error 
    X Convert locator to flow layout (Just assuming big screen) 
    # Houston Word lists 
    # Test printers (Fix multiple pages) 
    # Build smaller screens  (to fit on 640x480 display) 
    X 5 - Detailed screens (listen to individual words) 
    X  2 - Wave File generation 
    # 3 WORD LIST MANIPULATIONS 
      # Variable Bar Widths 
      # Thresholding word lists 
      # Sorting word lists 
      X greying words  
    #Mon: Travel; Lunch w. Diane; Setup computer in library&loaded softW. left 8pm 
    #Tue: 9:30am Tested printers; Wrote meeting outline & questions 
    #            Added a production-mode/development-mode flag  
    #Wed:  
    #Thur: 
    #Fri: Installed software 
    # 2 Initial screen & initial menus 
    # 1 2/3 Package for installation: zip, jar, jre, conf dirs, data dirs 
    # Test install function on my laptop 
    # data dir as conf parm 
    # Print word list only 
    # Good printer screens (Printer <<>> x <<>>)  72dpi 
    X Reuse single frame so screen switches do not result in tool in background 
      (Instead, avoid ever setting visible false between same screen) 
    # Dump error messages to file & stdout 
    # Don't print frame on initialization (just flashed) 
    # Build diagnostics icon 
    # label-of-screen on screen top-left 
    # InDevelopment flag 
    # Print all pages (if scrolling) 
    # 3 DATES 
    #   - Incorporate date range selection 
    #   # 2:1 Adding date controls to locator (parsing/printing dates) 
    #   1 Incorporating relative dates 
    # Add printObj fields 
    # Get info about WMR locations from registry 
    # Test the date select function 
    # 1 Input/Output test 
    X FULL PRINTING SUPPORT (4 days) 
    X X Change all pixel specs to % specs 
    X ? Build a get printer instance methods 
