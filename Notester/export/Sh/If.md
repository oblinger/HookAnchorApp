# Sh.If --

        if list; then list; [  elif list; then list ] 
            [ else action ] ; fi 
              The list following if is executed and, if it returns  a 
              zero  exit status, the list following the first then is 
              executed.  Otherwise, the list following elif  is  exe- 
              cuted and, if its value is zero, the list following the 
              next then is executed.  Failing that, the else list  is 
              executed.   If  no  else list or then list is executed, 
              then the if command returns a zero exit status. 
     
