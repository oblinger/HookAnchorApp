# Sh.Ulimit --

        ulimit [ -[HS][a | cdfnstv] ] 
         ulimit [ -[HS][c | d | f | n | s | t | v] ] limit 
              ulimit prints or sets hard  or  soft  resource  limits. 
              These limits are described in getrlimit(2). 
              If limit is not present, ulimit  prints  the  specified 
              limits.   Any  number  of  limits may be printed at one 
              time.  The -a option prints all limits. 
              If limit is present, ulimit sets the specified limit to 
              limit.  The string unlimited requests the largest valid 
              limit.  Limits may be set for only one  resource  at  a 
              time.  Any user may set a soft limit to any value below 
              the hard limit.  Any user may lower a hard limit.  Only 
              a super-user may raise a hard limit; see su(1M). 
              The -H option specifies a hard limit.  The  - S  option 
              specifies  a  soft  limit.  If neither option is speci- 
              fied, ulimit will set both limits and  print  the  soft 
              limit. 
              The following options specify the resource whose limits 
              are  to  be printed or set.  If no option is specified, 
              the file size limit is printed or set. 
                   -c    maximum core file size (in 512-byte blocks) 
                   -d    maximum size of data  segment  or  heap  (in 
                        kbytes) 
                   -f    maximum file size (in 512-byte blocks) 
                   -n    maximum file descriptor plus 1 
                   -s    maximum size of stack segment (in kbytes) 
                   -t    maximum CPU time (in seconds) 
                   -v    maximum size of virtual memory (in kbytes) 
     
     
