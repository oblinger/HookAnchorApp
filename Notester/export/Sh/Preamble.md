# Sh.Preamble -- Stuff at the top of the man page

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
     
