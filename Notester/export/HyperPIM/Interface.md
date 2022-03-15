# HyperPIM.Interface --

    [FileControlInterface; per user data] 
    - Initially this is run thru editor; later sent by pipe to an applet 
    - System maintains N edits in progress 
    - On startup: 
      1. App scans for existing edit files. 
      2. Tries to delete any lock file. 
      3. If lock was deleted or did not exist, then app takes control of this edit 
      4. Otherwise a new edit is started. 
    - On edit 
      1. DataFile created 
      2. Lock file is created and left open 
      3. User editing occurs.  Results are routinely written back to DataFile. 
      4. When edit completes DataFile is processed and deleted. 
      5. Lock file is closed and deleted. 
     
     
    [ApplicationInterface] 
    - Text window for editing page 
     
     
     
     
    [FileInterface] 
     
     
     
    [WebInterface] 
     
    controls 
     
     
     
    [Applet]  Simple applet interface 
    - Header contains controls 
    - Uses file control interface above 
     
