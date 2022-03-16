# dos -- Help for ms dos batch commands

     <<>>   MS Undocumented features 
     <<>>   Details of the ms-dos universal booting process 
     
      
    CONTROL FLOW 
     if [not] exists   
     if '%1' == ''  
     goto  
     :                                  Destination of goto 
     call                               Call batch 
                                        Jump to batch 
     command <<>> <<>>   <...>     Execute cmd in subshell 
     for %%a in (*.bat) do echo %%a            For Loop 
     <<>>   
     
    TIPS <<>> TRICKS 
      WAIT 'N' SECONDS   choice <<>> <<>> <<>>,   
      PAUSE W. TIMEOUT   choice <<>> <<>> <<>>, Press ESC to continue  
      ANSWERING YES      echo y | del <<>>*.* 
      ECHO BLANK LINE    echo. 
      SUPRESS OUTPUT     ctty nul  .... ctty con 
      EXPAND ENV SPACE   COMMAND.COM <<>> <<>> <<>> 
     
     
    CRAP 
     if "%1" == "ed" goto ed 
     if not errorlevel 4 goto nel4 
    if not exist foo.bat copy \bin\foo.bat 
    COMMAND <<>> <<>>  
     
