# WMRdiag.Issues -- Design issues and their resolution

    - Correct handling of STATE info 
    - Data manipulation speed. 
    - Statistics: how to count phrases,  how to graph automaticity 
    - Networked Data Organization 
    - Linkages to other packages & datafiles 
     
    RESOLVED 
    - Screen size assumption: 1024x768 
    - Logfile format: Use of word ids?     yes & local ids for each book 
    - Database computations? in JAVA 'till that fails. 
    - Fixed Tick marks? For now 
    - Are user names unique? yes 
    - Management of student data 
    - Data screen layout 
    - Efficient data manipulation package 
      : Use Raw Java, if needed then C or JDBC 
    - How is help displayed 
      : Context based popup hyper-text help 
     
     
    ----- 
     
    STATE INFO 
    NEEDS: 
    1) Return to a give state 
    2) Have printer inherit same settings as screen 
    3) Transform current state to get new state 
    SOLUTION #1 
    - State parms is a plist with <<>> user modifiable params. 
    - instantiate objs every time, w. inheritance from these parms. 
      <<>> objects can explicitly look for certain params in the StateParm 
           at .compute() time. 
    - Recursive method determines which parms are currently on locator (or options) 
    - problems: no differentiation between parms for multiple active displays 
                e.g. change threshold for one display 
    - DeclareParam(key,[key],initialVal, possibleVals, type, description text) 
      (each parm could be java objs, or plist entries)   
    SOLUTION #2 
    - State parms is a plist of plists: name&key->value 
    - Parameters are declared by current display objects 
    - Params are accessed by name&key at .compute() time. 
    - All parameters, are inherited from previous state (even inactive ones) 
     
     
    LINKAGES TO OTHER PACKAGES: 
    - JRE:            Assumed at \windows\jre.exe 
    - STUDENT DATA:   Obtained from registry 
    - CONFIGURATION:  Under current dir (must be the WMRdiag dir) 
    - .JAR files:     Absolute path placed in  
    - SHORTCUT:       Starts in WMRdiag dir 
     
    LINKAGES HACK FOR NOW 
    - Always install in <<>> 
    - Assume fixed dir for student data 
     
     
    STATISTICS: 
     
    Accuracy --  
    - CURRENT: correct iff part of a recognized phrase 
    - PROPOSED: correct (as above) but only incorrect iff not recognized & |phrase| =1 
     
    Automaticity -- 
    Only compute automaticity for correctly spoken words 
    - SINGLE-WORD: Only compute automaticity when when |phrase| =1 
    - PROPORTIONAL-BY-WORD: Diff of TS w. previous RECOG record divided by number of words 
    - PROPORTIONAL-BY-CHAR: As above but use ratio by characters 
     
     
    DATABASE COMPUTATIONS 
    - USE: DB, C code, Java code 
    - All data in memory  vs.  Per-student in memory 
    - Use of word ids 
     
     
    STATISTICS: HOW TO COUNT PHRASES  
    1) Ignore phrases 
    2) Count phrases as occurances of each of the individual words 
    3) Count phrases as occurances if correct, but ignore if incorrect 
     
    STATISTICS: HOW TO COMPUTE AUTOMATICITY 
    1) Seconds <<>> Chunk    (*) 
    2) Seconds <<>> Letter   (*) 
    3) STD from corpus of speech data 
       - based on a model linear in number of chars<<>>chunk 
       - based on a per chunk model 
     
     
     
    NETWORKED DATA ORGANIZATION 
    - Single root w. file and directory appending 
    - Networked version uses same single root structure w. 
      - w. per child locking & management tool locks is exclusive. 
    - WMR -->  
        log file, .wav file (per student, per book) --> 
        Utterance database  (per stuent, per book??) 
    - Incremental??  Caching?? 
     
     
     
    GUIDING PRINCIPLES 
    - (Most) Displays are student centric 
    - Visually based 
    - Click based navigation  (Click for next, prev, help or zoom) 
    - Word list based analysis 
    - Customizable screens  
     
     
     
     
     
    MANAGEMENT OF STUDENT DATA -- Provide a tool that allows teachers to: 
    - Define class as a collection of students  
    - Manipulate student info: Name, Phone number,  
    - Manipulate WMR data on a per-student or per-class basis: 
      - Create <<>> Destroy student databases 
      - Purge speech data older than a specified date (absolute or relative) 
      - Purge stats data older than ... 
      - Set auto-purging parameters (at least for .wav files) 
     
     
     
     
    HOW HELP IS HANDLED 
    - Click Popup menu with zoom and help options. 
     
     
