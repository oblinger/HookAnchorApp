# Unix.Sh-if --

    If     <<>>
    Branch statement 
     
     
    IF expr 
    THEN list 
    ELIF expr 
    THEN list 
    ELSE list 
    FI 
     
    'Then' must be the first thing on the line. 
     

    if [ "$sec_class" = "2" ]       # B1 Trusted System
    then
        reqid=$1
    else
        user=$2
    fi

    if test "$bpp" = "$<<>>"
    then
        bpp=1
    else
        device=`basename $<<>> .$bpp`
    fi

     
    if [ ! -e <<>> -a ! -L <<>> ] ; then
      <<>> -l -c > <<>>
    fi
