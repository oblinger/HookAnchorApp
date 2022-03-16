# Docs.Basic Lists --


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

    'T' THROW COMMAND -- Throws item to another list.  Moves an item from one todo 
    list to a second list. below is the list of global lists with their one key 
    designators.  So typing 'TW' (THROW WEEK) would cause the current to be throw 
    onto the WEEK list.  Items can also be thrown onto any project or other
    notester note.  Typing 'T' 'space' will prompt for destination list name.
    * If source  list is a global lists below, then item is *MOVED* from one list 
      to the other.  Othewise item is *COPIED* onto the target list, but is not 
      removed from the source.  Thus items can be listed on a project page, and 
      listed on a global list like WEEK or TODO.

    'M' MOVE COMMAND -- Moves within list.  This more advanced command assumes list is broken into groups
    of items separated by blank lines.  Move command used to MOVE items within different groups in the list.
    The '?' key can be used to see all destinations for the MOVE command.  'MN'  MOVE to NEXT group and 
    'MP' MOVE to PREVIOUS group are the most common destinations.

    'U' <<>> 'D' UP<<>>DOWN COMMANDS -- Moves the current item up or down within the current list.

    'P' <<>> 'N' PREVIOUS/NEXT COMMANDS -- Move highlighting cursor to the previous or next item in list.
    This command just moves the cursor, the order of this list itself is not modified.

    'X' COMMAND -- Deletes entry from current todo list.

    Once you get a feel for these ten basic todo list operators, there are more advanced ways of structuring
    larger or more specialized info.  But quite a bit of life can be juggled using these simple commands.

    NEXT: <<>>
