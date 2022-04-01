# Perl.Regexp --

    $line =~ s<<>>old<<>>new<<>>g;       # SUBST (Globally) 'old' into 'new' 
    $sentence =~ tr<<>>abc<<>>edf<<>>    # TRANSLATE  a->e b->d ... 
     
    # Untested file splitter 
    if (($base, $name, $ext) = ($line =~ <<>>^(\S+)\\(\S+)\.(.*)<<>>)) <<>> 
     
    # Untested split at last slash 
    ($path, $name) = ($line =~ <<>>^.*\<<>>[^<<>>]*$<<>>); 
     if (($F1, $F2, $Etc) = ($foo =~ <<>>^(\S+)\s+(\S+)\s*(.*)<<>>)) 
     
           This last example splits $foo into the first two words and the  
           remainder of the line, and assigns those three fields to $F1,  
           $F2, and $Etc. The conditional is true if any variables were  
           assigned, i.e., if the pattern matched.  
     
    m//=match;  s/// =srch/replace;  qr// qx// 
     
    .   # Any single character except a newline 
    ^   # The beginning of the line or string 
    $   # The end of the line or string 
    *   # Zero or more of the last character 
    +   # One or more of the last character 
    ?   # Zero or one of the last character 
     
     
    [qjk]       # Either q or j or k 
    [^qjk]      # Neither q nor j nor k 
    [a-z]       # Anything from a to z inclusive 
    [^a-z]      # No lower case letters 
    [a-zA-Z]    # Any letter 
    [a-z]+      # Any non-zero sequence of lower case letters 
     
     
    jelly|cream # Either jelly or cream 
    (eg|le)gs   # Either eggs or legs 
    (da)+       # Either da or dada or dadada or... 
     
     
    \n      # A newline 
    \t      # A tab 
    \w      # Any alphanumeric (word) character. 
            # The same as [a-zA-Z0-9_] 
    \W      # Any non-word character. 
            # The same as [^a-zA-Z0-9_] 
    \d      # Any digit. The same as [0-9] 
    \D      # Any non-digit. The same as [^0-9] 
    \s      # Any whitespace character: space, 
            # tab, newline, etc 
    \S      # Any non-whitespace character 
    \b      # A word boundary, outside [] only 
    \B      # No word boundary 
     
     
        \w  Match a "word" character (alphanumeric plus "_") 
        \W  Match a non-word character 
        \s  Match a whitespace character 
        \S  Match a non-whitespace character 
        \d  Match a digit character 
        \D  Match a non-digit character 
     
     
     
     
