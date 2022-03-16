# Tcl -- Tcl/tk

     <<>>   
     <<>>   
     <<>>   
     
    set f [open temp.date r] 
    gets $f NOW 
    close $f 
     
     
        set lso [exec ls -l $file] 
        set fileinfo [lindex $lso 2] 
        lappend fileinfo [clock format [file mtime $file] -format "%d %b %Y at %H:%M" -gmt 1] 
        return $fileinfo 
     
    STD: set b [expr $a+8]  
     
    IO:  open 
         puts stdout "My first Tcl script." 
         gets 
     
    CONTROL FLOW 
    - if test1 body1 ?elseif test2 body2 elseif ...? ?else bodyn?    
    - while <<>> <<>> 
    - for init test reinit body    
    for <<>> <<>> <<>> <<>> <<>> 
    - switch ?options? string pattern body ?pattern body ...? 
    - switch ?options? string <<>> 
      Options: -exact, -glob (string match), or -regexp (reexp match) ('-' = next) 
    - eval arg ?arg arg ...?  
     
    SYNTAX 
     # comment 
     " $var substitution " 
     <<>> 
     " constant \$string }" 
     array($access) 
     upvar 
     uplevel 
     
    OPERATORS 
    - Relational  (<, <=, >, >=, ==, !=) return 0 (false) or 1 (true).  
    - Integer Bit (&, |, ^, <<, >>, ~) 
    - expr <<>> 
     
    STRING 
    - string match pattern string  
    - regexp <<>> "Walk 10 km" a b c  
      => a="10 km", b="10", and c="km" 
     
    LIST 
    - concat ?list list ...?                  # Joins lists 
    - join list ?joinString?                  # List 2 string 
    - lappend varName value ?value ...?       # Append to 'varName' 
    - lindex list index                       # get nth ele (start @ 0) 
      set x <<>> e} 
    - linsert $x 2 X Y Z                      # non-destructive insert before 
      => a b X Y Z <<>> e  
    - list ?value value ...?                  # Make list 
    - llength list                            # list len 
    - lrange list first last                  # Subsequence 
    - lreplace list first last ?value ...?    # Replace substring (val= one ele) 
    - lsearch ?-exact? ?-glob? ?-regexp? list pattern   # Index of substring 
    - lsort ?-ascii? ?-integer? ?-real?       # Sort (def: -ascii -inc) 
      ?-command command? ?-increasing? ?-decreasing? list  
    - split string ?splitChars?               # String 2 list 
     
     
