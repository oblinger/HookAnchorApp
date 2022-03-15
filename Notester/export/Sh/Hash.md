# Sh.Hash --

        hash [ -r ] [ name ... ] 
              For each name, the location in the search path  of  the 
              command  specified by name is determined and remembered 
              by the shell.  The -r option causes the shell to forget 
              all  remembered  locations.  If no arguments are given, 
              information about  remembered  commands  is  presented. 
              Hits  is the number of times a command has been invoked 
              by the shell process.  Cost is a measure  of  the  work 
              required  to locate a command in the search path.  If a 
              command is found  in  a  "relative"  directory  in  the 
              search  path,  after  changing  to  that directory, the 
              stored location of that command is recalculated.   Com- 
              mands  for  which this will be done are indicated by an 
              asterisk (*) adjacent to the  hits  information.   Cost 
              will be incremented when the recalculation is done. 
     
     
