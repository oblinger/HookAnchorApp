# 999icl.Integration -- Integration with existing operating systems

    >>>QUESTIONS<<< 
    - When software is keyed, are the disks the same? 
    - What is MS install language? 
    - Does it control (or monitor read/writes) to filesystem? 
    - Can whole environment be control through WINDOWS env var? 
    - Are hard or soft links possible on any MS filesystems? 
     
     
    >>>THE BIG CHALLENGES<<< 
    1) Robustly undo, and redo changes to the OS environment. 
    2) Attribute arbitrary changes to OS environment to correct source. 
    3) Maintain (and/or quickly compute) multiple OS environments. 
     
    >>>OS ENVIRONMENT PARAMETERS<<< 
    - Execution path 
    - Default Environmental Parameters 
    - File System 
    - Sharable Libraries (.DLLs) 
    - Shared writable datafiles 
    - Shared directories (\WINDOWS) 
    - Package parameters 
    - Shared parameters 
    - System specification (Username, Hostname, TimeZone) 
     
     
     
     
    >>>EXAMPLE DEPENDENCIES<<< 
    Standard ICL pkg dependancies.  These dependances would be defined in  
    the ICL-CORE.  All objects are assumed to depend on the ICL-CORE. 
     
    CORE-PARMS <-- ICL-CORE, user 
     
    RAW-INSTALL <-- RAW-SPEC, CORE-PARMS, PACKAGE1, ... 
    NETWORKED-INSTALL <-- ... 
    DEFAULT-INSTALL <-- DEFAULT-INSTALL-SPEC, NETWORKED-INSTALL 
     
    - ICL-CORE: The ICL core defintions 
    - CORE-PARMS: Base Global System Parameters 
      Computed from the raw machine, and from user input. 
     
    - RAW-INSTALL  
    - RAW-INSTALL-SPEC: 
    - NETWORKED-INSTALL: 
    - NETWORKED-INSTALL-SPEC: 
    - DEFAULT-INSTALL: 
    - DEFAULT-INSTALL-SPEC: 
     
     
