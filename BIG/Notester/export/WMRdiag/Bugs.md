# WMRdiag.Bugs --

    ERR01 
    JREWAWT.dll crashes after job.end() when: 
    - Font >= size 30 is used (or mentioned in a component)? 
    - On ERR01 userData 
    - when print all users in list is selected 
    COMMENT:  I suspect bug in JRE. 
    FIX: Don't use very large fonts 
     
     
    Err02 
    Printouts sometimes clip top line (usually when less info on page) 
     
    Err03 
    Null ptr exception at end of printer print list. 
    Does not stop execution.  Just prints in log. 
     
     
     
