# Sh.Special-Environment-Variables --

         The following parameters are used by the shell.  The parame- 
         ters  in  this  section  are also referred to as environment 
         variables. 
              HOME The default argument (home directory) for  the  cd 
                   command,  set  to  the  user's  login directory by 
                   login(1) from the password file (see passwd(4)). 
              PATH The search path for commands  (see  ``Execution,'' 
                   below). 
              CDPATH 
                   The search path for the cd command. 
              MAIL If this parameter is set to the  name  of  a  mail 
                   file  and  the  MAILPATH parameter is not set, the 
                   shell informs the user of the arrival of  mail  in 
                   the specified file. 
              MAILCHECK 
                   This parameter specifies how  often  (in  seconds) 
                   the  shell  will  check for the arrival of mail in 
                   the files specified by the MAILPATH or MAIL param- 
                   eters.   The  default  value  is  600  seconds (10 
                   minutes).  If set  to  0,  the  shell  will  check 
                   before each prompt. 
              MAILPATH 
                   A colon (:)  separated list  of  file  names.   If 
                   this  parameter is set, the shell informs the user 
                   of the arrival of mail in  any  of  the  specified 
                   files.   Each file name can be followed by % and a 
                   message that will be printed when the modification 
                   time  changes.   The  default  message is you have 
                   mail. 
              PS1  Primary prompt string, by default ``$ ''. 
              PS2  Secondary prompt string, by default ``> ''. 
              IFS  Internal field separators,  normally  space,  tab, 
                   and newline (see ``Blank Interpretation''). 
              SHACCT 
                   If this parameter is set to the  name  of  a  file 
                   writable  by  the  user,  the  shell will write an 
                   accounting record in the file for each shell  pro- 
                   cedure executed. 
              SHELL 
                   When the shell is invoked, it scans  the  environ- 
                   ment (see ``Environment,'' below) for this name. 
              LC_CTYPE 
                   Determines how the shell handles characters.  When 
                   LC_CTYPE  is  set  to a valid value, the shell can 
                   display and handle text and  filenames  containing 
                   valid  characters  for  that locale. The shell can 
                   display and handle Extended Unix Code (EUC)  char- 
                   acters where any individual character can be 1, 2, 
                   or 3 bytes wide. The shell  can  also  handle  EUC 
                   characters  of 1, 2, or more column widths. In the 
                   "C" locale, only characters from  ISO  8859-1  are 
                   valid. 
              LC_MESSAGES 
                   Determines how diagnostic and informative messages 
                   are  presented.  This  includes  the  language and 
                   style of the messages, and  the  correct  form  of 
                   affirmative  and  negative  responses.  In the "C" 
                   locale, the messages are presented in the  default 
                   form  found  in the program itself (in most cases, 
                   U.S. English). 
              If LC_CTYPE and LC_MESSAGES (see  environ(5))  are  not 
              set in the environment, the operational behavior of the 
              shell for each corresponding locale category is  deter- 
              mined  by  the  value of the LANG environment variable. 
              If LC_ALL is set, its contents  are  used  to  override 
              both the LANG and the other LC_* variables.  If none of 
              the above variables is set in the environment, the  "C" 
              (U.S. style) locale determines how the shell behaves. 
     
         The shell gives default values to PATH, PS1, PS2, MAILCHECK, 
         and IFS.  HOME and MAIL are set by login(1). 
     
