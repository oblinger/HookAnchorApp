# Sh.Parameter-Substitution --

    $<<>>   ==> $vname or "defaultval" 
     
     
     
     
      Parameter Substitution 
         The character $ is used to introduce  substitutable  parame- 
         ters.   There  are  two  types of parameters, positional and 
         keyword.  If parameter is a digit, it is a positional param- 
         eter.   Positional parameters may be assigned values by set. 
         Keyword parameters (also known as variables) may be assigned 
         values by writing: 
     
              name=value [ name=value ] ... 
     
         Pattern-matching is not performed on value.  There cannot be 
         a function and a variable with the same name. 
     
         $<<>> 
              The value, if any, of  the  parameter  is  substituted. 
              The braces are required only when parameter is followed 
              by a letter, digit, or underscore that  is  not  to  be 
              interpreted  as part of its name.  If parameter is * or 
              @, all the positional parameters, starting with $1, are 
              substituted (separated by spaces).  Parameter $0 is set 
              from argument zero when the shell is invoked. 
         $<<>> 
              If parameter is set and  is  non-null,  substitute  its 
              value; otherwise substitute word. 
         $<<>> 
              If parameter is not set or is null set it to word;  the 
              value  of  the  parameter  is  substituted.  Positional 
              parameters may not be assigned in this way. 
         $<<>> 
              If parameter is set and  is  non-null,  substitute  its 
              value;  otherwise,  print word and exit from the shell. 
              If word is omitted, the message ``parameter null or not 
              set'' is printed. 
         $<<>> 
              If parameter is set and is non-null,  substitute  word; 
              otherwise substitute nothing. 
     
         In the above, word is not evaluated unless it is to be  used 
         as  the  substituted string, so that, in the following exam- 
         ple, pwd is executed only if d is not set or is null: 
     
              echo $<<>> 
     
         If the colon (:)  is omitted from the above expressions, the 
         shell only checks whether parameter is set or not. 
     
         The following parameters are automatically set by the shell. 
              #    The number of positional parameters in decimal. 
              -     Flags supplied to the shell on invocation  or  by 
                   the set command. 
              ?    The decimal value returned by  the  last  synchro- 
                   nously executed command. 
              $    The process number of this shell. 
              !    The process number of the last background  command 
                   invoked. 
     
     
     
     
      Blank Interpretation 
         After parameter and command  substitution,  the  results  of 
         substitution  are scanned for internal field separator char- 
         acters (those found in IFS) and split  into  distinct  argu- 
         ments  where such characters are found.  Explicit null argu- 
         ments ("" or '')  are  retained.   Implicit  null  arguments 
         (those  resulting  from  parameters that have no values) are 
         removed. 
     
