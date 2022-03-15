# Sh.Command-Processing --

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
     
         for name [ in word ... ] do list done 
              Each time a for command is executed, name is set to the 
              next  word taken from the in word list.  If in word ... 
              is omitted, then the for command executes the  do  list 
              once  for  each  positional  parameter that is set (see 
              ``Parameter  Substitution,''  below).   Execution  ends 
              when there are no more words in the list. 
     
         case word in [ pattern [                                   | 
               pattern ] ... ) list ;; ] ... esac 
              A case command executes the list  associated  with  the 
              first  pattern that matches word.  The form of the pat- 
              terns is the same as that used for file-name generation 
              (see  ``File  Name Generation'') except that a slash, a 
              leading dot, or a dot  immediately  following  a  slash 
              need not be matched explicitly. 
     
         if list; then list; [  elif list; then list ] 
            [ else action ] ; fi 
              The list following if is executed and, if it returns  a 
              zero  exit status, the list following the first then is 
              executed.  Otherwise, the list following elif  is  exe- 
              cuted and, if its value is zero, the list following the 
              next then is executed.  Failing that, the else list  is 
              executed.   If  no  else list or then list is executed, 
              then the if command returns a zero exit status. 
     
         while list do list done 
              A while command repeatedly executes the while list and, 
              if  the  exit status of the last command in the list is 
              zero, executes the do list;  otherwise  the  loop  ter- 
              minates.   If  no commands in the do list are executed, 
              then the while command  returns  a  zero  exit  status; 
              until  may be used in place of while to negate the loop 
              termination test. 
     
         (list) 
              Execute list in a sub-shell. 
     
         <<>> 
              list is executed  in  the  current  (that  is,  parent) 
              shell.  The <<>> 
              Define a function which is referenced by name. The body 
              of  the  function is the list of commands between <<>>.  The <<>> are unnecessary if the body of the function  is 
              a command as defined above, under ``Commands.'' 
     
         The following words are only recognized as the first word of 
         a command and when not quoted: 
     
         if then else elif fi case esac for while until do done <<>> 
     
      Comments Lines 
         A word beginning with # causes that word and all the follow- 
         ing characters up to a newline to be ignored. 
     
