# Dos.Choice -- Choice command

     CHOICE [<<>>[:]choices] [<<>>] [<<>>] [<<>>[:]c,nn] [text] 
     DOES NOT WORK UNDER NT!! 
     
    :: EXAMPLE  Print "Hello", wait 5 seconds for 'a', 'b', or 'c'.  Default = 'a' 
    choice <<>> <<>> <<>>,5 
    if errorlevel=3 echo C pressed 
    if errorlevel=2 echo C or B pressed 
    if errorlevel=1 echo C or B or A pressed 
    echo never prints 
     
     
     
    CHOICE [<<>>[:]choices] [<<>>] [<<>>] [<<>>[:]c,nn] [text] 
       Waits for the user to choose one of a set of choices. 
       ERRORLEVEL is set to offset of key user presses in choices. 
       <<>>[:]choices Specifies allowable keys. Default is YN 
       <<>>           Do not display choices and ? at end of prompt string. 
       <<>>           Treat choice keys as case sensitive. 
       <<>>[:]c,nn    Default choice to c after nn seconds 
       text         Prompt string to display 
     
     
     
