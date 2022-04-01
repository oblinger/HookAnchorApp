# Sh.Read --

        read name ... 
              One line is read from the standard input and, using the 
              internal  field separator, IFS (normally space or tab), 
              to delimit word boundaries, the first word is  assigned 
              to  the first name, the second word to the second name, 
              etc., with leftover words assigned to  the  last  name. 
              Lines  can  be  continued  using  \newline.  Characters 
              other than newline can be quoted by preceding them with 
              a  backslash.   These  backslashes  are  removed before 
              words are assigned to names, and no  interpretation  is 
              done  on the character that follows the backslash.  The 
              return code is 0, unless an EOF is encountered. 
     
     
