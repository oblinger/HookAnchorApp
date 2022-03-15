# Sh.Set --

        set [ --aefhkntuvx [ argument ... ] ] 
              -a    Mark variables which are modified or created  for 
                   export. 
              -e    Exit immediately if a command exits with  a  non- 
                   zero exit status. 
              -f    Disable file name generation. 
              -h    Locate and remember function  commands  as  func- 
                   tions  are defined (function commands are normally 
                   located when the function is executed). 
              -k    All keyword arguments are placed in the  environ- 
                   ment  for  a  command, not just those that precede 
                   the command name. 
              -n    Read commands but do not execute them. 
              -t    Exit after reading and executing one command. 
              -u    Treat unset variables as an error when substitut- 
                   ing. 
              -v    Print shell input lines as they are read. 
              -x    Print commands and their arguments  as  they  are 
                   executed. 
              --     Do not change any of the flags; useful  in  set- 
                   ting $1 to -. 
              Using + rather than - causes these flags to  be  turned 
              off.   These  flags can also be used upon invocation of 
              the shell.  The current set of flags may be found in $- 
              .   The  remaining  arguments are positional parameters 
              and are assigned, in order, to  $1,  $2,  ....   If  no 
              arguments  are  given  the  values  of  all  names  are 
              printed. 
     
     
