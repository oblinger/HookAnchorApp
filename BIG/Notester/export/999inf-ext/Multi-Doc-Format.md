# 999inf-ext.Multi-Doc-Format --

    IDEA 
    - Small API needed on each doc type 
    - Central link server used to manage links; browsing and editing 
     
     
     
    ININITAL DOC TYPES 
    - .INFO files  (using emacs info) 
    - .HTML files  (both local and web) 
    - .DOC  files 
    - Notes messages? 
     
     
    UPI 
    - Universal place indicator 
      (URL like, may specific both doc and position within doc) 
     
     
     
    METHODS NEEDED FOR EACH TYPE OF DOC BROWSER 
    - Browse(upi) 
      Opens browser/editor to specified object at specified position. 
      Position can be a line number or a pattern to find, it may be ignored. 
     
    - GetLocation()-->upi 
      Obtains and returns the current document, position. 
     
    - GetLink() 
      Obtains the id of the current link 
     
    - GetChildren() 
      Returns list if children ids 
     
     
     
    METHODS ON MASTER CONTROLER 
    - Lookup(upi, id)-->upi 
    -  
     
      Looksup 
