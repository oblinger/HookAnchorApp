# Docs.Todo Commands --

     
       LIST COMMANDS 
     <<>>
     | ENETER<<>>SPACE |  CREATE item         <<>> EDIT item                 |
     | 'T'  <<>> 'M'   |  THROW to other list <<>> MOVE item within list     |
     | 'U'  <<>> 'D'   |  Move item UP        <<>> Move item DOWN            |
     | 'N'  <<>> 'P'   |  Cursor to NEXT item <<>> Cursor to PREVIOUS item   |
     | 'X'  <<>> 'Z'   |  'X' out (delete)    <<>> ZAP off list (not deleted)|
     <<>>

    ENTER -- Opens a new todo list item.  Text typed will fill in item, and 
    a second ENTER will complete the entry.  If item begins with '[xxx] ' then 
    this todo item links to the specified project, or note entry.  

     <<>>
     <<>>

    UP/DOWN COMMANDS -- Moves the current item up or down within the current list.

    PREVIOUS/NEXT COMMANDS -- Move highlighting cursor to the previous or 
    next item in list.  This command just moves the cursor, the order of this 
    list itself is not modified.

    'X' COMMAND -- Deletes entry from current todo list.
    'Z' COMMAND -- Removes item from a list, but leaves on other lists (does not delete)
