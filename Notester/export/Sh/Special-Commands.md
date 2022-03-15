# Sh.Special-Commands --

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
     
     
