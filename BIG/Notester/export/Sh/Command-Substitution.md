# Sh.Command-Substitution --

      Command Substitution 
         The shell reads commands from the string between  two  grave 
         accents  (` `)  and  the standard output from these commands 
         may be used as all or part of  a  word.   Trailing  newlines 
         from the standard output are removed. 
     
         No interpretation is done on the string before the string is 
         read,  except to remove backslashes (\) used to escape other 
         characters.  Backslashes may  be  used  to  escape  a  grave 
         accent  (`)  or another backslash (\) and are removed before 
         the command string is read.  Escaping grave  accents  allows 
         nested  command  substitution.   If the command substitution 
         lies within a pair of double quotes (" ...` ...` ...  "),  a 
         backslash  used  to  escape  a  double  quote  (\")  will be 
         removed; otherwise, it will be left intact. 
     
         If a backslash is used to escape a newline character  (\new- 
         line),  both  the backslash and the newline are removed (see 
         the later section on ``Quoting'').  In addition, backslashes 
         used  to  escape  dollar  signs  (\$) are removed.  Since no 
         parameter substitution is done on the command string  before 
         it  is  read,  inserting a backslash to escape a dollar sign 
         has no effect.  Backslashes that  precede  characters  other 
         than  \,  `, ", newline, and $ are left intact when the com- 
         mand string is read. 
     
     
