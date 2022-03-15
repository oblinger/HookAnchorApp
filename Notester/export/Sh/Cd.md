# Sh.Cd -- Change directory

         cd [ argument ] 
              Change the current directory to  argument.   The  shell 
              parameter  HOME  is  the  default  argument.  The shell 
              parameter CDPATH defines the search path for the direc- 
              tory  containing argument.  Alternative directory names 
              are separated by a colon  (:).   The  default  path  is 
                (specifying the current directory).  Note:  The 
              current directory is specified by  a  null  path  name, 
              which  can  appear  immediately after the equal sign or 
              between the colon delimiters anywhere else in the  path 
              list.   If  argument begins with a <<>> the search path is 
              not used.  Otherwise, each directory  in  the  path  is 
              searched for argument. 
     
