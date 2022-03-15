# Unix.Sh-pun --

    Pun 
    Meaning of special characters 
     
    ""      Treat character as a single string.  Do file substitution. 
    #      Begin comment (end at newline) 
    &       Asynchronous execution (background processing) 
    &&      And.  Execute remainder if preceding had 0 return val 
    ''      Treat characters as a single string.  Do not do file subst. 
    *       File name substitution.  Match any file. 
    ;       Sequential execution.  Ex: cp a b ; cp b c 
    <       Redirect standard input. 
    << str  Redirect input.  read from this file until 'str' is found. 
    >       Redirect standard output.  Ex: ls >foo 
            cc dan.c 2>err    # this redirects errors 
    >>      Redirect standard output.  Append to a file. 
    ?       File name substitution.  Match single char. 
    [!...]  File name substitution.  Matches all chars. NOT specified. 
    [...]   File name substitution.  Match any specified char. 
            A minus sign is used to denote a range of chars. 
    [class] File name substitution.  Match single char of a class. 
            This matches any character in a particular class. 
    ...     may be replaced with the following class names: 
            alnum, alpha, digit, lower, print, punct, space, upper, or  
            xdigit.  Ex: [:alpha:] 
    \       Quote character.  Do not use char's special meaning. 
            Ex: % echo \*hi\*    prints: *hi* 
    ``      Capture output of command into a string. 
    <<>>      Ececute list in current shell. 
    ||      Execute remainder only if preceding had <>0 return val 
    RETURN  Sets return value for shell function.  Ex: Return 1 
     
     
     
     
     
     
     
     
     
     
     
