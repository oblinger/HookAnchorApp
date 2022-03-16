# 999icl.Issues -- How to tackle the tough issues

    FILE SYSTEM TRAPPING 
    #1  Hide the actual file system drive, and expose a virtual drive. 
        This drive efficiently handles many UNIX-like mounts from the actual drive. 
        Each mount may be read-only, and may log file reads and/or writes. 
     
    #2  Raw files may be stored off to the side and moved into place as needed. 
        sweeps of the filesystem would be necessary to determined what changed. 
     
     
    USING OFF THE SHELF INSTALLERS 
    - Create an INSTALL environment. 
    - Run installation; keeping track of: 
      - User input 
      - File System access & changes 
      - Registry access & changes 
     
     
     
