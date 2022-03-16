# Sh.Test --

            test [expr] 
           test <<>> 
     
     
     <<>>     
     
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
                  arg1  is equal, not-equal, less-than, less<<>> 
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
     
