# Sh.Trap --

        trap [ argument ] [ n ] ... 
              The command argument is to be read  and  executed  when 
              the  shell  receives numeric or symbolic signal(s) (n). 
              (Note: argument is scanned once when the  trap  is  set 
              and  once  when  the trap is taken.)  Trap commands are 
              executed in order of  signal  number  or  corresponding 
              symbolic  names.  Any attempt to set a trap on a signal 
              that was ignored on entry to the current shell is inef- 
              fective.   An  attempt  to  trap  on  signal 11 (memory 
              fault) produces an error.  If argument  is  absent  all 
              trap(s) n are reset to their original values.  If argu- 
              ment is the null string this signal is ignored  by  the 
              shell  and  by  the commands it invokes.  If n is 0 the 
              command argument is executed on exit  from  the  shell. 
              The  trap  command  with  no arguments prints a list of 
              commands associated with each signal number. 
     
     
