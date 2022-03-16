# Sh.Input-Output-Redirection --

      Input/Output Redirection 
         A command's input and output may be redirected using a  spe- 
         cial  notation  interpreted by the shell.  The following may 
         appear anywhere in a simple-command or may precede or follow 
         a  command and are not passed on as arguments to the invoked 
         command.  Note:  Parameter and command  substitution  occurs 
         before word or digit is used. 
     
         word         Use  file  word  as  standard   output   (file 
                       descriptor 1).  If the file does not exist, it 
                       is created; otherwise, it is truncated to zero 
                       length. 
         >>word        Use file word as standard output.  If the file 
                       exists,  output  is  appended  to it (by first 
                       seeking to the EOF); otherwise,  the  file  is 
                       created. 
         <<[-]word      After parameter and command  substitution  is 
                       done  on  word,  the shell input is read up to 
                       the first  line  that  literally  matches  the 
                       resulting  word,  or to an EOF. If, however, - 
                       is appended to <<: 
                       1)  leading tabs are stripped from word before 
                           the   shell   input  is  read  (but  after 
                           parameter and command substitution is done 
                           on word), 
                       2)  leading tabs are stripped from  the  shell 
                           input  as  it is read and before each line 
                           is compared with word, and 
                       3)  shell input is read up to the  first  line 
                           that literally matches the resulting word, 
                           or to an EOF. 
                       If  any  character  of  word  is  quoted  (see 
                       ``Quoting,''  later), no additional processing 
                       is done to the shell input.  If no  characters 
                       of word are quoted: 
                       1)  parameter and command substitution occurs, 
                       2)  (escaped) \newlines are removed, and 
                       3)  \ must be used to quote the characters  \, 
                           $, and `. 
                       The resulting document  becomes  the  standard 
                       input. 
         <&digit       Use the file associated with  file  descriptor 
                       digit  as  standard  input.  Similarly for the 
                       standard output using >&digit. 
         <&-            The standard input is closed.  Similarly  for 
                       the standard output using >&-. 
     
         If any of the  above  is  preceded  by  a  digit,  the  file 
         descriptor  which  will  be associated with the file is that 
         specified by the digit (instead of the default 0 or 1).  For 
         example: 
     
              ... 2>&1 
     
         associates file descriptor 2 with the file currently associ- 
         ated with file descriptor 1. 
     
         The order in which redirections are  specified  is  signifi- 
         cant.   The shell evaluates redirections left-to-right.  For 
         example: 
     
              ... 1>xxx 2>&1 
     
         first associates file descriptor 1 with file xxx.  It  asso- 
         ciates  file descriptor 2 with the file associated with file 
         descriptor 1 (that is, xxx).  If the order  of  redirections 
         were  reversed,  file  descriptor 2 would be associated with 
         the terminal (assuming file descriptor 1 had been) and  file 
         descriptor 1 would be associated with file xxx. 
     
         Using the terminology introduced on the  first  page,  under 
         ``Commands,''  if  a  command  is composed of several simple 
         commands, redirection will be evaluated for the entire  com- 
         mand  before  it is evaluated for each simple command.  That 
         is, the shell evaluates redirection  for  the  entire  list, 
         then each pipeline within the list, then each command within 
         each pipeline, then each list within each command. 
     
         If a command is followed by & the default standard input for 
         the  command  is  the  empty file <<>>  Otherwise, the 
         environment for the execution of a command contains the file 
         descriptors   of   the   invoking   shell   as  modified  by 
         input/output specifications. 
     
