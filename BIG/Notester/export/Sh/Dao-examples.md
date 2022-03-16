# Sh.Dao-examples --

    [VAR]  $<<>>  $<<>>  $<<>>  $<<>>->4  $<<>> 
           $<<>> 
    [CONTROL] 
    if test -e ~/.bash_profile ; then ... ; elif ...  ; fi 
    sub() <<>> 
    fn() <<>> 
    fn ddd 
     
     
    FILE STUFF 
    #Strip off the suffix and add the new one 
      newname=$(basename $f tex)latex 
     
     
     
     
     
     
     
    foo() <<>> finds shell vars w. specified prefix 
     
    [TEST] 
     
    function number <<>> 
     
    number "%Corr=57   Acc=47.68 Fred" 1 7-8 
     
     
    if test -e ~/.bash_profile ; then source ~/.bash_profile ; fi 
     
     
    while [ x"$1" != x ]; do 
        case $1 in 
        -c) instcmd="$cpprog" 
            shift 
            continue;; 
     
        *)  if [ x"$src" = x ] 
            then 
            src=$1 
            else 
            # this colon is to work around a 386BSD <<>> bug 
            : 
            dst=$1 
            fi 
            shift 
            continue;; 
        esac 
    done 
     
    if [ x"$src" = x ] 
    then 
        echo "install:  no input file specified" 
        exit 1 
    else 
        true 
    fi 
     
     
     
     
     
    Arguments  
    #!<<>> 
    if [ $# = 1 ] 
    then 
       string="It is " 
       ending="" 
    else  
       string="They are " 
       ending="s" 
    fi 
    print This $0 command has $# argument$<<>>. 
    if [ $# != 0 ] 
    then 
    print $string $* 
    fi 
     
    Adding Numbers  
    #!<<>> 
    print "input a number" 
    read number1 
     
    print "now input another number" 
    read number2 
     
    let answer=$number1+$number2 
    print "$number1 + $number2 = $answer" 
     
     
    Login Counting Script  
    #!<<>> 
    times=$(who | grep $1 | wc -l) 
    print "$1 is logged on $times times." 
     
     
    List Directories  
    #!<<>> 
    for file in $* 
    do 
      if [ -d $file ] 
      then 
         ls $file 
      fi 
    done 
     
     
    See Script  
    #!<<>> 
    for file in $* 
    do 
      if [ -d $file ] 
      then 
         print "using ls" 
         ls $file 
      else 
         more $file 
      fi 
    done 
     
     
    Safe Copying  
    #!<<>> 
    if [ -f $2 ] 
    then 
            print "$2 exists. Do you want to overwrite it? (y/n)" 
            read yn 
            if [ $yn = "N" -o $yn = "n" ] 
            then 
                exit 0 
            fi 
    fi 
    cp $1 $2 
     
     
    Changing the suffix  
    #!<<>> 
    for f in *.tex 
    do 
    #Strip off the suffix and add the new one 
      newname=$(basename $f tex)latex 
    # Do the name change 
      mv $f $newname 
    done 
     
    Mailmerge  
    #!<<>> 
    for name in $(letter 
      # here you coul print the letter file out 
    done 
     
     
    Local users  
     
     
     
     
