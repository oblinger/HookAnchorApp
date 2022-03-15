# Sh.DETAILS --

      File Name Generation 
         Before a command is executed, each command word  is  scanned 
         for  the characters *, ?, and [.  If one of these characters 
         appears the word is regarded as  a  pattern.   The  word  is 
         replaced  with  alphabetically  sorted file names that match 
         the pattern.  If no file name is found that matches the pat- 
         tern,  the  word is left unchanged.  The character .  at the 
         start of a file name or immediately following a <<>>,  as  well 
         as the character <<>> itself, must be matched explicitly. 
     
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
         The following example first prints a=b c and <<>> 
     
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
         <<>>  The current directory is specified by a null path 
         name, which can appear immediately  after  the  equal  sign, 
         between  two  colon delimiters anywhere in the path list, or 
         at the end of the path list.  If the command name contains a 
         <<>> the search path is not used.  Otherwise, each directory in 
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
     
