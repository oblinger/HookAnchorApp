# Sh.End-Stuff -- See also

    EXIT CODES 
         Errors detected by the shell, such as syntax  errors,  cause 
         the shell to return a non-zero exit status.  If the shell is 
         being used non-interactively execution of the shell file  is 
         abandoned.   Otherwise, the shell returns the exit status of 
         the last command executed (see also the exit command above). 
     
      jsh Only 
         If the shell is invoked as jsh and an  attempt  is  made  to 
         exit  the  shell  while  there  are  stopped jobs, the shell 
         issues one warning: 
     
         There are stopped jobs. 
     
         This is the only message.  If another exit attempt is  made, 
         and  there are still stopped jobs they will be sent a SIGHUP 
         signal from the kernel and the shell is exited. 
     
    FILES 
         $HOME/.profile 
         <<>> 
         <<>> 
         <<>>* 
     
    SEE ALSO 
         intro(1), echo(1),  getoptcvt(1)  login(1),  pwd(1),  ps(1), 
         shell_builtins(1),  stty(1),  rsh(1M),  newgrp(1M),  dup(2), 
         exec(2),   fork(2),   getrlimit(2),   pipe(2),    ulimit(2), 
         setlocale(3C), profile(4), passwd(4), environ(5), signal(5) 
     
    NOTES 
         Words used for filenames in input/output redirection are not 
         interpreted for filename generation (see ``File Name Genera- 
         tion,'' above).  For example, cat file1 >a*  will  create  a 
         file named a*. 
     
         Because commands in pipelines are run as separate processes, 
         variables  set  in  a  pipeline have no effect on the parent 
         shell. 
     
         If  you  get  the  error  message  cannot  fork,  too   many 
         processes,  try  using  the wait(1) command to clean up your 
         background processes.  If this doesn't help, the system pro- 
         cess  table  is  probably  full  or you have too many active 
         foreground processes.  (There is a limit to  the  number  of 
         process  ids  associated  with your login, and to the number 
         the system can keep track of.) 
     
         Only the last process in a pipeline can be waited for. 
     
         If a command is executed, and a command with the  same  name 
         is  installed  in  a directory in the search path before the 
         directory where the original command was  found,  the  shell 
         will  continue  to  exec the original command.  Use the hash 
         command to correct this situation. 
     
     
