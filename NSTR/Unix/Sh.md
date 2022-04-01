# Unix.Sh --

    Borne Shell 
     
     <<>>     Multiway branch. 
     <<>>     Evals arg as if part of the shell script. 
     <<>>     Exec cmd specified in place of current script. 
     <<>>       Branch statement 
     <<>>     Eval conditional expression. 
     <<>>    Until loop. 
     <<>>       String of the names of current execution flags. 
     <<>>   Shell variable manipulation. 
     <<>>   Export variables to children processes. 
     <<>>     Read variable values from standard input. 
     <<>>    Shift [n] Shift input args 
     <<>>     Wait [n] Wait for child process to end. 
     <<>>    While loop. 
     <<>>    Break from loop. 
     <<>>   Continue execution of a loop. 
     <<>>   Execute commands in file. do not spawn subshell. 
     <<>>     Causes shell to exit with specified exit value. 
     <<>>      For loop. 
     <<>>   Shell functions. 
     <<>>     Display/compute command name hash table. 
     <<>>    Show accumulated user execution times. 
     <<>>   Sets or queries size limits. 
     <<>>      Var 
     <<>>      Sets shell flags 
    * sh-type 
     <<>>     Trap [arg] [n]  Sets a trap to be executed when a signal 
     <<>>     Echo args on screen. 
     <<>>         Meaning of special characters 
     <<>>     Set file creation mask 
     
     
        echo -n " [y/n] "
        read YN junk
        if [ "$YN" = "n" ]
        then
            return -1;
        else
            return 0;
        fi
    case "$<<>>" in
      direct)
            gsoutput="cat 1>&3" ;;
    #       cat $<<>> 1>&3 & ;;
      indirect)
            gsoutput="lpr -P$<<>>.raw" ;;
    #       cat $<<>> | lpr -P$<<>>.raw & ;;
    esac



     for fn in history.info readline.info rluserman.info ; do 
      install-info --dir-file=./dir --info-file=$<<>>
    done

     while [ $# != 0 ]
    do  case "$1" in
      -n)   user=$2 ; shift ;;
      -h)   host=$2 ; shift ;;
      -*)   ;;
      *)    acctfile=$1 ;;
      esac
      shift
    done

     
     
     
     
     

     
     
     
     
     
     
     
     
     
