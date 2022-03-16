# 999icl.Encap-interface -- Encap dir icl implementation

    ;;: Overview 
    ;;: Text representation 
    ;;: Directory representation 
    ;;: Types 
    ;;: Examples 
     
     
    ;;; Overview 
     
    An encapsulation object is a composable container of application info. 
    It is designed to simplify the task of maintaining software installations. 
     
    - Each CAPsule is a namespace. 
    - Each namespace has a type? 
     
     
     
    ;;; Text representation 
     
    TEXTUAL REPRESENTATION OF A CAP: 
     
    '>'  
    '.'  
    '='   
    '<'  
     
     
     
     
    ;;; Directory representation 
     
    A simple encapsulation object's structure (a cap): 
     
    - is a directory with the following structure. 
     
    encap-root: 
    - ``dir.cap'' This file declares that this directory is a CAP object 
    - Type Directories (e.g.  bin, info, man, etc.) 
    - Component Directories (directories that are also CAP objects) 
     
    ;;; Namespace operations 
     
    ns_fullname(ns) 
    ns_type(ns) 
    ns_components(ns) 
    ns_get(ns, key) 
    ns_set(ns, key, value) 
     
     
     
    ;;; Methods 
     
    Update  
     
     
     
    ;;; Types  
     
    ENCAP        -- Encapsulation directory object.  Has typed namespaces: 
      Installed  -- List of 'active' caps 
      Available  -- List of Trees of available caps. 
      Bin        --   
      Man        --  
       
    BIN    --  Space of executables.  (Non-conflicting aggrigation) 
    MAN    -- 
    INFO   -- 
     
    ;;; Combination methods 
     
    NON-CONFLICTING AGGRIGATION  (NCA) 
    - Union namespaces.  If conflict, insert error object as value. 
      If conflict, but equal values then insert value. 
     
     
     
     
    ;;; Example 
     
     
    An encap directory with a software pkg foo 
     
    <<>>                The encap capsule 
    <<>>            The aggrigation of files from the  
    <<>>            The foo capsule 
    <<>>        The bin capsule 
    <<>>        The man capsule 
    <<>>    The foo CAP file: 
    <<>>     
     
     
    Foo CAP file: 
    > 
    > action 
    = foo-link  "link  
     
       
     
     
    Goals: 
     
    HANDLING FILE SYSTEM UPDATES 
     
    FS-Mods 
    For each updated file system location a list of updates are kept: 
         
     
    Override -- Puts a file, link, directory, or nothing at specified location. 
    Move     -- Moves contents from one place to another  
    Compute  -- Computes contents from prior contents & (file system)? 
    Init     -- Set location up if not there (never removed) 
     
    Override-Declare:  (these are the components of an override specification) 
          
     
       Id        -- a unique string identifying <<>> particular update 
       Location  -- path of location to update 
       Type      -- File, Directory, Link 
       Contents  -- Path of new contents (relative to contents dir) 
     
    Override-Sync: 
    - If file system contents != current update file then 
        If current update is a user update just merge else  prompt user to: 
          1) Integrate changed file back into source of override 
          2) Add ``User-update'' entry to fs-mods 
          3) Return contents back to pushed contents 
          4) Fail. 
     
    Override-Push:   (make location reflect specified mod) 
    - If there is a FS-mod for location, then 
    -   Delete location's contents 
    -   Transfer current location's contents 
     
    Override-Install: 
    - Override-Sync file 
    - Add entry to FS-Mod 
    - Override-Push file 
     
    Override-Remove: 
    - Override-Sync 
    - Remove entry from FS-Mod 
    - Override-Push 
     
     
    User Interface: 
    - encap install  
    - encap remove   
    - encap open       ; Writes changes to specified cap 
    - encap close 
    - encap sync     ...   ; Invoke before modifying file 
     
