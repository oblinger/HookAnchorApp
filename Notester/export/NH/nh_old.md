# NH.nh\_old --


    FILEIT 
    - Edit Screen Fields: 
      UniqueID, title line, body text, Click box per desktop item,  
      Category menu, Creation Timestamp, storageDestination 
    - Load (from id, next<<>>prev, or search) 
    - Save 
     
     
     
    DESIGN 
    - Node parts: 'id-daystamp' 'name: line'  'body' 'attachment-refs' 
    - Node storage: idfile:  yymmddX-name.nd 
    - Library: directory with nodes 
    - always syncing edit changes with expanded directory entry for record 
    - load<<>>save dir 
    - SaveDir: move files, append to log .html file 
    - tab-d-c  CREATE-DOC ( in cronological dir ) 
               NEW  (UI: input word space line and double return ends) 
    - tab-d-s 
    - tab-d-l  LIST  
    - tab-d-v  VIEW 
    -          SEND-FILE-TO-NOTE 
      Start by ID<<>>name 
      get 2 desktop 
      build <<>> update index 
    - create sub directory rather than in same dir 
    - move files to new location for big proj things 
    - go into "edit" mode on entry (bring up editors on relevant stuff) 
       
     
     
    NODE SEMANTICS 
    - Edits cause node to be copied, so nodes link to previous versions 
    -  
     
     
     
     
    GUI (View <<>> Edit Node) 
     
    - Unique Node ID 
    - Name                (Category checkbox) 
    - Title line 
    - Fixed Width Font body window 
    - List of selectable attachments 
     
    Buttons:  
    - New create blank entry and edit that entry 
    - Goto (by name <<>> id) 
    - Search
