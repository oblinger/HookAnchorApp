  [__DocBash__](__DocBash__.md)


BASH SPECIFIC
BASH SPECIFIC

# BLAM BASH
x="${x}more"

[TOPLEVEL]
sleep 30

[CONTROL]
while [ x"$1" != x ]; do \\..\\ done


# BLAM BASH
x="${x}more"

[TOPLEVEL]
sleep 30

[CONTROL]
while [ x"$1" != x ]; do \\..\\ done





Borne Shell 

## Subroutines

#!/bin/sh

 

mysub()

{
        echo "Arg 1: $1"
        return 3

}

 

mysub "This is an arg"

echo "Subroutine returned $?"

## Sh-case_     Multiway branch. 
Case 
Multiway branch. 
 
 
CASE word IN 
 
pattern [|pattern] ... list;; 
... 
 
ESAC 
 
 
Uses command name substitution. 
 
 
 
 
 
 
 
 
 
 
## Sh-eval_     Evals arg as if part of the shell script. 
Eval 
 
Evals arg as if it were part of the shell script. 
 
 
##Sh-exec_     Exec cmd specified in place of current script. 
Exec 
 
Execute command specified in place of current script. 
 
 
## Sh-if_       Branch statement 
If     _sh.test_
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

if test "$bpp" = "${device}"
then
    bpp=1
else
    device=`basename ${device} .$bpp`
fi

 
if [ ! -e /etc/passwd -a ! -L /etc/passwd ] ; then
  /bin/mkpasswd -l -c > /etc/passwd
fi

## Sh-test_     Eval conditional expression. 
Test 
Eval conditional expression. 
 
 
 
 _sh.test_
 
 
 
 
 
 
 

## Sh-until_    Until loop. 
Until 
Until loop. 
 
 
UNTIL list 
DO list 
DONE 
 
## Sh-$-_       String of the names of current execution flags. 
$- 
 
Evaluates to a string of the names of current execution 
flags. 
 
## Sh-variables_   Shell variable manipulation. 
Variables 
Shell variable manipulation. 
 
 
$name evaluates to the value of a shell variable. 
name=string set the value of a shell variable.  (DO NOT USE SPACES) 
 
 
## Sh-export_   Export variables to children processes. 
## Sh-read_     Read variable values from standard input. 
Read 
Read variable values from standard input. 
 
Eg. 
 
read foo bar rest 
echo foo=$foo bar=$bar rest=$rest 
 
user: 
1 2 3 4 
 
foo=1 bar=2 rest=3 4 
 
 
 
 
 
 
 
 
 
 
 
## Sh-shift_    Shift [n] Shift input args 
Shift 
Shift [n]   shift input args 
 
## Sh-wait_     Wait [n] Wait for child process to end. 
## Sh-while_    While loop. 
## Sh-break    Break from loop. 
Break 
Break from loop. 
 
Break 
 
Break [n] 
 
## Sh-continue
 Continue execution of a loop. 
ntinue Continue execution of a loop. 

## Sh-dot-file Execute commands in file. do not spawn subshell. 
Dot File 
Execute commands in file.  do not spawn subshell. 
 
## Sh-exit Causes shell to exit with specified exit value. 
Exit 
Causes shell to exit with specified exit value. 
 
## Sh-for For loop. 
For 
For loop. 
 
FOR name IN word ... 
DO list 
DONE 
 
If IN word ... is not specified then the loop goes 
through the input 
parameters. 
 
## Sh-function Shell functions. 
Function 
Shell functions. 
 
Defines a sh function: 
 
name () { list; ... } 
 
## Sh-hash Display/compute command name hash table. 
Hash 
Display/compute command name hash table. 
 
hash -r  #recompute hash table 

## Shift
Shift 
Shift [n]   shift input args 
 
## Sh-times Show accumulated user execution times. 
Times 
Show accumulated user execution times. 
 
## Sh-ulimit Sets or queries size limits. 
Ulimit 
Sets or queries size limits. 
 
 
## Sh-var Var 
Var 
 
 
$ 
$! 
$# 
$$ 
$? 
 
## Sh-set Sets shell flags 
Set 
Sets shell flags 
 
SET [flag [arg] ... ] 
 
flags: 
-a  auto export variables 
-e  exit if error 
-f  disable file name substitution 
-h 
-k 
-n  read command but do not execute 
-t  exit after reading and executing one command. 
-u  treat unset vars. as errors 
-v  display shell lines as they are executed 
-x  display commands and args as they are executed. 
-- 
 
NOTE: + will unset flag. 
$- contains current flag settings. 
 
## sh-type 
Trap 
Trap [arg] [n]  sets a trap to be executed when a signal 
is enco 
 
## Sh-trap Trap [arg] [n]  Sets a trap to be executed when a signal 

## Sh-echo Echo args on screen. 
Echo 
Echo args on screen. 
 
## Pun Meaning of special characters 

## Sh-mask Set file creation mask 
Mask 
Set file creation mask 
 
sh [nnn] 
 
this has something to do with setting this mask. 
 



## DAO Examples
[VAR]  ${varNAME:-defaultVAL}  ${var:3}  ${var:3:5}  ${#var}->4  ${v/old/new} 
       ${chop%/*} 
[CONTROL] 
if test -e ~/.bash_profile ; then ... ; elif ...  ; fi 
sub() {PROJ_DIR=$PWD ; } 
fn() { echo $1 ; } 
fn ddd 
 
 
FILE STUFF 
#Strip off the suffix and add the new one 
  newname=$(basename $f tex)latex 
 
 
 
 
 
 
 
foo() { 
  echo $1 
${!namePrefix*} finds shell vars w. specified prefix 
 
[TEST] 
 
function number { 
   print $1 | cut -d' ' -f$2 | cut -c $3  
} 
 
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
		# this colon is to work around a 386BSD /bin/sh bug 
		: 
		dst=$1 
	    fi 
	    shift 
	    continue;; 
    esac 
done 
 
if [ x"$src" = x ] 
then 
	echo "install:	no input file specified" 
	exit 1 
else 
	true 
fi 
 
 
 
 
 
Arguments  
#!/bin/sh 
if [ $# = 1 ] 
then 
   string="It is " 
   ending="" 
else  
   string="They are " 
   ending="s" 
fi 
print This $0 command has $# argument${ending}. 
if [ $# != 0 ] 
then 
print $string $* 
fi 
 
Adding Numbers  
#!/bin/sh 
print "input a number" 
read number1 
 
print "now input another number" 
read number2 
 
let answer=$number1+$number2 
print "$number1 + $number2 = $answer" 
 
 
Login Counting Script  
#!/bin/sh 
times=$(who | grep $1 | wc -l) 
print "$1 is logged on $times times." 
 
 
List Directories  
#!/bin/sh 
for file in $* 
do 
  if [ -d $file ] 
  then 
     ls $file 
  fi 
done 
 
 
See Script  
#!/bin/sh 
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
#!/bin/sh 
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
#!/bin/sh 
for f in *.tex 
do 
#Strip off the suffix and add the new one 
  newname=$(basename $f tex)latex 
# Do the name change 
  mv $f $newname 
done 
 
Mailmerge  
#!/bin/sh 
for name in $(<names) 
do 
  sed s/NAME/$name/ <template >letter 
  # here you coul print the letter file out 
done 
 
 
Local users  
 
 
 
 
## TEST command
       test [expr] 
       test {--help,--version} 
 
 
 _Test-preamble_     
 
BINARY TEST 
       string True if the length of string is non-zero. 
       string1 = string2 
              True if the strings are equal. 
       string1 != string2 
              True if the strings are not equal. 
       ! expr True if expr is false. 
       expr1 -a expr2 
              True if both expr1 and expr2 are true. 
       expr1 -o expr2 
              True if either expr1 or expr2 is true. 
       arg1 OP arg2 
              OP  is  one  of  -eq,  -ne,  -lt, -le, -gt, or -ge. 
              These arithmetic binary operators  return  true  if 
              arg1  is equal, not-equal, less-than, less-than-or- 
              equal, greater-than, or greater-than-or-equal  than 
              arg2,  respectively.  arg1 and arg2 may be positive 
              integers, negative integers, or the special expres- 
              sion  -l  string,  which evaluates to the length of 
              string. 
 
UNARY TESTS 
       -b file 
              True if file exists and is block special. 
       -c file 
              True if file exists and is character special. 
       -d file 
              True if file exists and is a directory. 
       -e file 
              True if file exists 
       -f file 
              True if file exists and is a regular file. 
       -g file 
              True if file exists and is set-group-id. 
       -k file 
              True if file has its ``sticky'' bit set. 
       -L file 
              True if file exists and is a symbolic link. 
       -p file 
              True if file exists and is a named pipe. 
       -r file 
              True if file exists and is readable. 
       -s file 
              True if file exists and has  a  size  greater  than 
              zero. 
       -S file 
              True if file exists and is a socket. 
       -t [fd] 
              True if fd is opened on a terminal.  If fd is omit- 
              ted, it defaults to 1 (standard output). 
       -u file 
              True if file exists and its set-user-id bit is set. 
       -w file 
              True if file exists and is writable. 
       -x file 
              True if file exists and is executable. 
       -O file 
              True  if  file exists and is owned by the effective 
              user id. 
       -G file 
              True if file exists and is owned by  the  effective 
              group id. 
       file1 -nt file2 
              True  if  file1 is newer (according to modification 
              date) than file2. 
       file1 -ot file2 
              True if file1 is older than file2. 
       file1 -ef file2 
              True if file1 and file2 have the  same  device  and 
              inode numbers. 
       -z string 
              True if the length of string is zero. 
       -n string 
 

## Man page preamble
sh(1)                     User Commands                     sh(1) 
 
 
 
NAME 
     sh, jsh - shell: the standard shell, and job  control  shell 
     -- command interpreters 
 
SYNOPSIS 
     sh [ -acefhiknprstuvx ] [ argument...] 
     jsh [ -acefhiknprstuvx ] [ argument...] 
 
AVAILABILITY 
     SUNWcsu 
 
DESCRIPTION 
     sh is a command programming language that executes  commands 
     read  from  a  terminal  or  a  file.  The command jsh is an 
     interface to the shell which provides all of the functional- 
     ity  of  sh  and  enables  Job Control (see ``Job Control,'' 
     below).  See ``Invocation,'' below for the meaning of  argu- 
     ments to the shell. 
 
  Definitions 
     A blank is a tab or a space.  A name is a sequence of  ASCII 
     letters,  digits, or underscores, beginning with a letter or 
     an underscore.  A parameter is a name, a digit,  or  any  of 
     the characters *, @, #, ?, -, $, and !\^. 
 
USAGE 
 

## MAN PG -- command processing
  Commands 
     A simple-command is a sequence of non-blank words  separated 
     by blanks.  The first word specifies the name of the command 
     to be executed.  Except as specified  below,  the  remaining 
     words  are  passed as arguments to the invoked command.  The 
     command name is passed as argument  0  (see  exec(2)).   The 
     value  of  a  simple-command  is  its exit status if it ter- 
     minates normally, or (octal)  200+status  if  it  terminates 
     abnormally; see signal(5) for a list of status values. 
 
     A pipeline is a sequence of one or more  commands  separated 
     by  | .  The standard output of each command but the last is 
     connected by a pipe(2) to the standard  input  of  the  next 
     command.   Each  command  is  run as a separate process; the 
     shell waits for the last command  to  terminate.   The  exit 
     status  of a pipeline is the exit status of the last command 
     in the pipeline. 
 
     A list is a sequence of one or more pipelines  separated  by 
     ;,  &,  &&,  or ||, and optionally terminated by ; or &.  Of 
     these four symbols, ; and & have equal precedence, which  is 
     lower  than  that  of && and ||.  The symbols && and || also 
     have equal precedence.  A semicolon  (;)  causes  sequential 
     execution  of  the  preceding  pipeline  (that is, the shell 
     waits for the pipeline to finish before executing  any  com- 
     mands  following  the  semicolon);  an  ampersand (&) causes 
     asynchronous execution of the preceding pipeline  (that  is, 
     the  shell  does not wait for that pipeline to finish).  The 
     symbol && (||) causes the list following it to  be  executed 
     only  if  the  preceding  pipeline returns a zero (non-zero) 
     exit status.  An arbitrary number of newlines may appear  in 
     a list, instead of semicolons, to delimit commands. 
 
     A command is either a simple-command or one of  the  follow- 
     ing.   Unless otherwise stated, the value returned by a com- 
     mand is that of the last simple-command executed in the com- 
     mand. 
 
     for name [ in word ... ] do list done 
          Each time a for command is executed, name is set to the 
          next  word taken from the in word list.  If in word ... 
          is omitted, then the for command executes the  do  list 
          once  for  each  positional  parameter that is set (see 
          ``Parameter  Substitution,''  below).   Execution  ends 
          when there are no more words in the list. 
 
     case word in [ pattern [                                   | 
           pattern ] ... ) list ;; ] ... esac 
          A case command executes the list  associated  with  the 
          first  pattern that matches word.  The form of the pat- 
          terns is the same as that used for file-name generation 
          (see  ``File  Name Generation'') except that a slash, a 
          leading dot, or a dot  immediately  following  a  slash 
          need not be matched explicitly. 
 
     if list; then list; [  elif list; then list ] 
        [ else action ] ; fi 
          The list following if is executed and, if it returns  a 
          zero  exit status, the list following the first then is 
          executed.  Otherwise, the list following elif  is  exe- 
          cuted and, if its value is zero, the list following the 
          next then is executed.  Failing that, the else list  is 
          executed.   If  no  else list or then list is executed, 
          then the if command returns a zero exit status. 
 
     while list do list done 
          A while command repeatedly executes the while list and, 
          if  the  exit status of the last command in the list is 
          zero, executes the do list;  otherwise  the  loop  ter- 
          minates.   If  no commands in the do list are executed, 
          then the while command  returns  a  zero  exit  status; 
          until  may be used in place of while to negate the loop 
          termination test. 
 
     (list) 
          Execute list in a sub-shell. 
 
     { list;} 
          list is executed  in  the  current  (that  is,  parent) 
          shell.  The { must be followed by a space. 
 
     name () { list;} 
          Define a function which is referenced by name. The body 
          of  the  function is the list of commands between { and 
          }.  The { must be followed by a  space.   Execution  of 
          functions  is described below (see ``Execution'').  The 
          { and } are unnecessary if the body of the function  is 
          a command as defined above, under ``Commands.'' 
 
     The following words are only recognized as the first word of 
     a command and when not quoted: 
 
     if then else elif fi case esac for while until do done { } 
 
  Comments Lines 
     A word beginning with # causes that word and all the follow- 
     ing characters up to a newline to be ignored. 
 

 
 
 



STRING STUFF

X=`expr "$Y" : '\([0-9]*).*'`    X assigned numeric prefix
	echo -n " [y/n] "
	read YN junk
	if [ "$YN" = "n" ]
	then
		return -1;
	else
		return 0;
	fi
case "${type}" in
  direct)
		gsoutput="cat 1>&3" ;;
#		cat ${gspipe} 1>&3 & ;;
  indirect)
		gsoutput="lpr -P${device}.raw" ;;
#		cat ${gspipe} | lpr -P${device}.raw & ;;
esac



 for fn in history.info readline.info rluserman.info ; do 
  install-info --dir-file=./dir --info-file=${fn}
done

 while [ $# != 0 ]
do  case "$1" in
  -n)	user=$2 ; shift ;;
  -h)	host=$2 ; shift ;;
  -*)	;;
  *)	acctfile=$1 ;;
  esac
  shift
done

 
 
 
 
 

 
 
 
 
 
 
 
 
 
## MAN PG -- Command Substitution
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
 
 
## MAN PG -- Parameter Substitution
${vname:-defaultval}   ==> $vname or "defaultval" 
 
 
 
 
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
 
     ${parameter} 
          The value, if any, of  the  parameter  is  substituted. 
          The braces are required only when parameter is followed 
          by a letter, digit, or underscore that  is  not  to  be 
          interpreted  as part of its name.  If parameter is * or 
          @, all the positional parameters, starting with $1, are 
          substituted (separated by spaces).  Parameter $0 is set 
          from argument zero when the shell is invoked. 
     ${parameter:-word} 
          If parameter is set and  is  non-null,  substitute  its 
          value; otherwise substitute word. 
     ${parameter:=word} 
          If parameter is not set or is null set it to word;  the 
          value  of  the  parameter  is  substituted.  Positional 
          parameters may not be assigned in this way. 
     ${parameter:?word} 
          If parameter is set and  is  non-null,  substitute  its 
          value;  otherwise,  print word and exit from the shell. 
          If word is omitted, the message ``parameter null or not 
          set'' is printed. 
     ${parameter:+word} 
          If parameter is set and is non-null,  substitute  word; 
          otherwise substitute nothing. 
 
     In the above, word is not evaluated unless it is to be  used 
     as  the  substituted string, so that, in the following exam- 
     ple, pwd is executed only if d is not set or is null: 
 
          echo ${d:-`pwd`} 
 
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
 
## MAN PG -- Special Environment Variables
     The following parameters are used by the shell.  The parame- 
     ters  in  this  section  are also referred to as environment 
     variables. 
          HOME The default argument (home directory) for  the  cd 
               command,  set  to  the  user's  login directory by 
               login(1) from the password file (see passwd(4)). 
          PATH The search path for commands  (see  ``Execution,'' 
               below). 
          CDPATH 
               The search path for the cd command. 
          MAIL If this parameter is set to the  name  of  a  mail 
               file  and  the  MAILPATH parameter is not set, the 
               shell informs the user of the arrival of  mail  in 
               the specified file. 
          MAILCHECK 
               This parameter specifies how  often  (in  seconds) 
               the  shell  will  check for the arrival of mail in 
               the files specified by the MAILPATH or MAIL param- 
               eters.   The  default  value  is  600  seconds (10 
               minutes).  If set  to  0,  the  shell  will  check 
               before each prompt. 
          MAILPATH 
               A colon (:)  separated list  of  file  names.   If 
               this  parameter is set, the shell informs the user 
               of the arrival of mail in  any  of  the  specified 
               files.   Each file name can be followed by % and a 
               message that will be printed when the modification 
               time  changes.   The  default  message is you have 
               mail. 
          PS1  Primary prompt string, by default ``$ ''. 
          PS2  Secondary prompt string, by default ``> ''. 
          IFS  Internal field separators,  normally  space,  tab, 
               and newline (see ``Blank Interpretation''). 
          SHACCT 
               If this parameter is set to the  name  of  a  file 
               writable  by  the  user,  the  shell will write an 
               accounting record in the file for each shell  pro- 
               cedure executed. 
          SHELL 
               When the shell is invoked, it scans  the  environ- 
               ment (see ``Environment,'' below) for this name. 
          LC_CTYPE 
               Determines how the shell handles characters.  When 
               LC_CTYPE  is  set  to a valid value, the shell can 
               display and handle text and  filenames  containing 
               valid  characters  for  that locale. The shell can 
               display and handle Extended Unix Code (EUC)  char- 
               acters where any individual character can be 1, 2, 
               or 3 bytes wide. The shell  can  also  handle  EUC 
               characters  of 1, 2, or more column widths. In the 
               "C" locale, only characters from  ISO  8859-1  are 
               valid. 
          LC_MESSAGES 
               Determines how diagnostic and informative messages 
               are  presented.  This  includes  the  language and 
               style of the messages, and  the  correct  form  of 
               affirmative  and  negative  responses.  In the "C" 
               locale, the messages are presented in the  default 
               form  found  in the program itself (in most cases, 
               U.S. English). 
          If LC_CTYPE and LC_MESSAGES (see  environ(5))  are  not 
          set in the environment, the operational behavior of the 
          shell for each corresponding locale category is  deter- 
          mined  by  the  value of the LANG environment variable. 
          If LC_ALL is set, its contents  are  used  to  override 
          both the LANG and the other LC_* variables.  If none of 
          the above variables is set in the environment, the  "C" 
          (U.S. style) locale determines how the shell behaves. 
 
     The shell gives default values to PATH, PS1, PS2, MAILCHECK, 
     and IFS.  HOME and MAIL are set by login(1). 
 

## MAN PG -- Input Output redirection
  Input/Output Redirection 
     A command's input and output may be redirected using a  spe- 
     cial  notation  interpreted by the shell.  The following may 
     appear anywhere in a simple-command or may precede or follow 
     a  command and are not passed on as arguments to the invoked 
     command.  Note:  Parameter and command  substitution  occurs 
     before word or digit is used. 
 
     <word         Use file word as standard input (file descrip- 
                   tor 0). 
     >word         Use  file  word  as  standard   output   (file 
                   descriptor 1).  If the file does not exist, it 
                   is created; otherwise, it is truncated to zero 
                   length. 
     >>word        Use file word as standard output.  If the file 
                   exists,  output  is  appended  to it (by first 
                   seeking to the EOF); otherwise,  the  file  is 
                   created. 
     <<[-]word      After parameter and command  substitution  is 
                   done  on  word,  the shell input is read up to 
                   the first  line  that  literally  matches  the 
                   resulting  word,  or to an EOF. If, however, - 
                   is appended to <<: 
                   1)  leading tabs are stripped from word before 
                       the   shell   input  is  read  (but  after 
                       parameter and command substitution is done 
                       on word), 
                   2)  leading tabs are stripped from  the  shell 
                       input  as  it is read and before each line 
                       is compared with word, and 
                   3)  shell input is read up to the  first  line 
                       that literally matches the resulting word, 
                       or to an EOF. 
                   If  any  character  of  word  is  quoted  (see 
                   ``Quoting,''  later), no additional processing 
                   is done to the shell input.  If no  characters 
                   of word are quoted: 
                   1)  parameter and command substitution occurs, 
                   2)  (escaped) \newlines are removed, and 
                   3)  \ must be used to quote the characters  \, 
                       $, and `. 
                   The resulting document  becomes  the  standard 
                   input. 
     <&digit       Use the file associated with  file  descriptor 
                   digit  as  standard  input.  Similarly for the 
                   standard output using >&digit. 
     <&-            The standard input is closed.  Similarly  for 
                   the standard output using >&-. 
 
     If any of the  above  is  preceded  by  a  digit,  the  file 
     descriptor  which  will  be associated with the file is that 
     specified by the digit (instead of the default 0 or 1).  For 
     example: 
 
          ... 2>&1 
 
     associates file descriptor 2 with the file currently associ- 
     ated with file descriptor 1. 
 
     The order in which redirections are  specified  is  signifi- 
     cant.   The shell evaluates redirections left-to-right.  For 
     example: 
 
          ... 1>xxx 2>&1 
 
     first associates file descriptor 1 with file xxx.  It  asso- 
     ciates  file descriptor 2 with the file associated with file 
     descriptor 1 (that is, xxx).  If the order  of  redirections 
     were  reversed,  file  descriptor 2 would be associated with 
     the terminal (assuming file descriptor 1 had been) and  file 
     descriptor 1 would be associated with file xxx. 
 
     Using the terminology introduced on the  first  page,  under 
     ``Commands,''  if  a  command  is composed of several simple 
     commands, redirection will be evaluated for the entire  com- 
     mand  before  it is evaluated for each simple command.  That 
     is, the shell evaluates redirection  for  the  entire  list, 
     then each pipeline within the list, then each command within 
     each pipeline, then each list within each command. 
 
     If a command is followed by & the default standard input for 
     the  command  is  the  empty file /dev/null.  Otherwise, the 
     environment for the execution of a command contains the file 
     descriptors   of   the   invoking   shell   as  modified  by 
     input/output specifications. 
 

## MAN PG -- Details

  File Name Generation 
     Before a command is executed, each command word  is  scanned 
     for  the characters *, ?, and [.  If one of these characters 
     appears the word is regarded as  a  pattern.   The  word  is 
     replaced  with  alphabetically  sorted file names that match 
     the pattern.  If no file name is found that matches the pat- 
     tern,  the  word is left unchanged.  The character .  at the 
     start of a file name or immediately following a /,  as  well 
     as the character / itself, must be matched explicitly. 
 
          *     Matches any string, including the null string. 
          ?    Matches any single character. 
          [...] 
               Matches any one of  the  enclosed  characters.   A 
               pair  of  characters  separated  by  - matches any 
               character lexically between the  pair,  inclusive. 
               If  the first character following the opening [ is 
               a !, any character not enclosed is matched. 
          Note:   All  quoted  characters  (see  below)  must  be 
          matched explicitly in a filename. 
 
  Quoting 
     The following characters have a special meaning to the shell 
     and cause termination of a word unless quoted: 
 
          ;  &  (  )  |  ^  <  >  newline  space  tab 
 
     A character may be  quoted  (that  is,  made  to  stand  for 
     itself) by preceding it with a backslash (\) or inserting it 
     between a pair of quote marks ('' or "").   During  process- 
     ing,  the shell may quote certain characters to prevent them 
     from taking on a special meaning.  Backslashes used to quote 
     a single character are removed from the word before the com- 
     mand is executed.  The pair \newline is removed from a  word 
     before command and parameter substitution. 
 
     All characters enclosed between a pair of single quote marks 
     (''),  except  a  single  quote,  are  quoted  by the shell. 
     Backslash has no special meaning inside  a  pair  of  single 
     quotes.   A single quote may be quoted inside a pair of dou- 
     ble quote marks (for example, "'"), but a single  quote  can 
     not be quoted inside a pair of single quotes. 
 
     Inside a pair of double quote marks (""), parameter and com- 
     mand substitution occurs and the shell quotes the results to 
     avoid blank interpretation and file name generation.  If $ * 
     is within a pair of double quotes, the positional parameters 
     are substituted and quoted, separated by quoted spaces  ("$1 
     $2  ..."); however, if $@ is within a pair of double quotes, 
     the  positional  parameters  are  substituted  and   quoted, 
     separated by unquoted spaces ("$1" "$2" ... ).  \ quotes the 
     characters \, `,  , and $.  The  pair  \newline  is  removed 
     before  parameter  and command substitution.  If a backslash 
     precedes characters other than \, `,  , $, and newline, then 
     the backslash itself is quoted by the shell. 
 
  Prompting 
     When used interactively, the shell prompts with the value of 
     PS1  before  reading a command.  If at any time a newline is 
     typed and further input is needed to complete a command, the 
     secondary prompt (that is, the value of PS2) is issued. 
 
  Environment 
     The environment (see environ(5)) is  a  list  of  name-value 
     pairs  that is passed to an executed program in the same way 
     as a normal argument list.  The  shell  interacts  with  the 
     environment in several ways.  On invocation, the shell scans 
     the environment and creates a parameter for each name found, 
     giving it the corresponding value.  If the user modifies the 
     value of any of these parameters or creates new  parameters, 
     none of these affects the environment unless the export com- 
     mand is used to bind the shell's parameter to  the  environ- 
     ment (see also set -a).  A parameter may be removed from the 
     environment with the unset command.  The environment seen by 
     any  executed  command  is  thus  composed of any unmodified 
     name-value pairs originally inherited by  the  shell,  minus 
     any  pairs removed by unset, plus any modifications or addi- 
     tions, all of which must be noted in export commands. 
 
     The environment for any simple-command may be  augmented  by 
     prefixing  it  with  one  or more assignments to parameters. 
     Thus: 
 
          TERM=450 command 
          and 
          (export TERM; TERM=450; command) 
 
     are equivalent as far as the execution of  command  is  con- 
     cerned if command is not a Special Command.  If command is a 
     Special Command, then 
          TERM=450 command 
     will modify the TERM variable in the current shell. 
 
     If the -k flag is set, all keyword arguments are  placed  in 
     the  environment, even if they occur after the command name. 
     The following example first prints a=b c and c: 
 
          echo a=b c 
          a=b c 
          set -k 
          echo a=b c 
          c 
 
  Signals 
     The INTERRUPT and QUIT signals for an  invoked  command  are 
     ignored  if  the command is followed by &; otherwise signals 
     have the values inherited by the shell from its parent, with 
     the  exception  of  signal 11 (but see also the trap command 
     below). 
 
  Execution 
     Each time a command is executed, the  command  substitution, 
     parameter  substitution,  blank interpretation, input/output 
     redirection, and filename generation listed above  are  car- 
     ried out.  If the command name matches the name of a defined 
     function, the function is  executed  in  the  shell  process 
     (note  how  this  differs  from  the execution of shell pro- 
     cedures).  If the command name does not match the name of  a 
     defined  function,  but  matches one of the Special Commands 
     listed below, it is executed  in  the  shell  process.   The 
     positional parameters $1, $2, ....  are set to the arguments 
     of the function.  If the command name matches neither a Spe- 
     cial  Command nor the name of a defined function, a new pro- 
     cess is created and an attempt is made to execute  the  com- 
     mand via exec(2). 
 
     The shell parameter PATH defines the  search  path  for  the 
     directory  containing  the  command.   Alternative directory 
     names are separated by a colon (:).   The  default  path  is 
     /usr/bin.  The current directory is specified by a null path 
     name, which can appear immediately  after  the  equal  sign, 
     between  two  colon delimiters anywhere in the path list, or 
     at the end of the path list.  If the command name contains a 
     / the search path is not used.  Otherwise, each directory in 
     the path is searched for an executable file.   If  the  file 
     has  execute  permission  but  is  not  an a.out file, it is 
     assumed to be a file containing shell commands.  A sub-shell 
     is spawned to read it.  A parenthesized command is also exe- 
     cuted in a sub-shell. 
 
     The location in the search path where a command was found is 
     remembered  by  the  shell  (to help avoid unnecessary execs 
     later).  If the command was found in a  relative  directory, 
     its  location  must  be  re-determined  whenever the current 
     directory changes.  The shell forgets all  remembered  loca- 
     tions  whenever  the PATH variable is changed or the hash -r 
     command is executed (see below). 
 

## MAN PG - Command Introduction
  Commands 
     A simple-command is a sequence of non-blank words  separated 
     by blanks.  The first word specifies the name of the command 
     to be executed.  Except as specified  below,  the  remaining 
     words  are  passed as arguments to the invoked command.  The 
     command name is passed as argument  0  (see  exec(2)).   The 
     value  of  a  simple-command  is  its exit status if it ter- 
     minates normally, or (octal)  200+status  if  it  terminates 
     abnormally; see signal(5) for a list of status values. 
 
     A pipeline is a sequence of one or more  commands  separated 
     by  | .  The standard output of each command but the last is 
     connected by a pipe(2) to the standard  input  of  the  next 
     command.   Each  command  is  run as a separate process; the 
     shell waits for the last command  to  terminate.   The  exit 
     status  of a pipeline is the exit status of the last command 
     in the pipeline. 
 
     A list is a sequence of one or more pipelines  separated  by 
     ;,  &,  &&,  or ||, and optionally terminated by ; or &.  Of 
     these four symbols, ; and & have equal precedence, which  is 
     lower  than  that  of && and ||.  The symbols && and || also 
     have equal precedence.  A semicolon  (;)  causes  sequential 
     execution  of  the  preceding  pipeline  (that is, the shell 
     waits for the pipeline to finish before executing  any  com- 
     mands  following  the  semicolon);  an  ampersand (&) causes 
     asynchronous execution of the preceding pipeline  (that  is, 
     the  shell  does not wait for that pipeline to finish).  The 
     symbol && (||) causes the list following it to  be  executed 
     only  if  the  preceding  pipeline returns a zero (non-zero) 
     exit status.  An arbitrary number of newlines may appear  in 
     a list, instead of semicolons, to delimit commands. 
 
     A command is either a simple-command or one of  the  follow- 
     ing.   Unless otherwise stated, the value returned by a com- 
     mand is that of the last simple-command executed in the com- 
     mand. 
 
 
 
## MAN PG -- Special Commands
  Special Commands 
     Input/output redirection is now  permitted  for  these  com- 
     mands.   File  descriptor  1 is the default output location. 
     When Job Control is enabled, additional Special Commands are 
     added to the shell's environment (see ``Job Control''). 
 
     :    No effect; the command does nothing.  A zero exit  code 
          is returned. 
     . filename 
          Read and execute commands  from  filename  and  return. 
          The  search  path specified by PATH is used to find the 
          directory containing filename. 
 
 

