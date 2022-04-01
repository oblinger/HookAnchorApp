# Sh.Command-Introduction --

      Commands 
         A simple-command is a sequence of non-blank words  separated 
         by blanks.  The first word specifies the name of the command 
         to be executed.  Except as specified  below,  the  remaining 
         words  are  passed as arguments to the invoked command.  The 
         command name is passed as argument  0  (see  exec(2)).   The 
         value  of  a  simple-command  is  its exit status if it ter- 
         minates normally, or (octal)  200+status  if  it  terminates 
         abnormally; see signal(5) for a list of status values. 
     
         A pipeline is a sequence of one or more  commands  separated 
         by  | .  The standard output of each command but the last is 
         connected by a pipe(2) to the standard  input  of  the  next 
         command.   Each  command  is  run as a separate process; the 
         shell waits for the last command  to  terminate.   The  exit 
         status  of a pipeline is the exit status of the last command 
         in the pipeline. 
     
         A list is a sequence of one or more pipelines  separated  by 
         ;,  &,  &&,  or ||, and optionally terminated by ; or &.  Of 
         these four symbols, ; and & have equal precedence, which  is 
         lower  than  that  of && and ||.  The symbols && and || also 
         have equal precedence.  A semicolon  (;)  causes  sequential 
         execution  of  the  preceding  pipeline  (that is, the shell 
         waits for the pipeline to finish before executing  any  com- 
         mands  following  the  semicolon);  an  ampersand (&) causes 
         asynchronous execution of the preceding pipeline  (that  is, 
         the  shell  does not wait for that pipeline to finish).  The 
         symbol && (||) causes the list following it to  be  executed 
         only  if  the  preceding  pipeline returns a zero (non-zero) 
         exit status.  An arbitrary number of newlines may appear  in 
         a list, instead of semicolons, to delimit commands. 
     
         A command is either a simple-command or one of  the  follow- 
         ing.   Unless otherwise stated, the value returned by a com- 
         mand is that of the last simple-command executed in the com- 
         mand. 
     
     
     
