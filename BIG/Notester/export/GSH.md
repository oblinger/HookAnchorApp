# GSH --

    GSH: MenuExec fails to spawn process in winMinimize
    GSH: sleep is in microseconds on runInput
    GSH: escape from menuinput & menuexec ''
    GSH: FileExists returns '0'
    GSH: MenuExec does not use std stack frame (all cmds should!)
         Deprecate evInt etc.


    ?GSH dont error on illegal keystroke
    ?GSH use '' not '0' as arg default
    ?GSH MenuExecute MenuInput 
    ??GSH Printf no extra \n (and change so ^n means newline)
    ?GSH Optional ',' in function calls??




    # Script to launch PW build environment

    # Trace   # <-- Uncomment this for debugging

    # Bind <<>>
    Subst <<>> GetExeDir                

    # Bind <<>> to parts
    Subst <<>> ScanForDrive GetExeDir   

    # Start Bash Shell in Cygwin and use 
    RunCygBash('--rcfile' 'bin/anchor.sh' '-i')
