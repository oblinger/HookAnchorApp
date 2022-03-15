# Sh.Invocation -- Invocation of sh (script?)

      Invocation 
         If the shell is invoked through exec(2) and the first  char- 
         acter  of  argument  zero  is -, commands are initially read 
         from <<>> and from  $HOME/.profile,  if  such  files 
         exist.   Thereafter,  commands  are read as described below, 
         which is  also  the  case  when  the  shell  is  invoked  as 
         <<>>   The  flags below are interpreted by the shell 
         on invocation only.  Note:  Unless the -c  or  - s  flag  is 
         specified, the first argument is assumed to be the name of a 
         file containing commands, and the  remaining  arguments  are 
         passed as positional parameters to that command file: 
     
         -c string  If the -c flag is present commands are read  from 
                   string. 
         -i         If the -i flag is present or if the  shell  input 
                   and  output are attached to a terminal, this shell 
                   is interactive.  In this case TERMINATE is ignored 
                   (so  that  kill  0  does  not  kill an interactive 
                   shell) and INTERRUPT is  caught  and  ignored  (so 
                   that  wait  is interruptible).  In all cases, QUIT 
                   is ignored by the shell. 
         -p         If the -p flag is present, the shell will not set 
                   the  effective user and group IDs to the real user 
                   and group IDs. 
         -r         If the -r flag is present the  shell  is  a  res- 
                   tricted shell (see rsh(1M)). 
         -s         If the -s flag is  present  or  if  no  arguments 
                   remain, commands are read from the standard input. 
                   Any remaining  arguments  specify  the  positional 
                   parameters.  Shell output (except for Special Com- 
                   mands) is written to file descriptor 2. 
     
         The remaining flags and arguments are  described  under  the 
         set command above. 
     
