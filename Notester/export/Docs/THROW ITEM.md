# Docs.THROW ITEM --


    THROW ITEM -- Throws item to another list.  
    Moves an item from one todo list to a second list

    * The throw command waits for the first letter of a global list name 
      to be typed, this is the list where the item will be thrown.
      So typing 'TW' (THROW WEEK) would cause the current to be throw 
      onto the WEEK list.  See <<>> for throw characters.

    * Items can also be thrown onto any project or other notester list.  
      Typing 'T' 'space' will prompt for destination list name.

    * If source list is a global lists below (see <<>>) , then item 
      is *MOVED* off of the global list to the other.  Othewise item is *COPIED* 
      onto the target list, but is not removed from the source.  Thus items can 
      be listed on a project page, and listed on a global list like WEEK or TODO.
      (Use the 'Z'ap command if you want to later remove item from global list without 
       deleting it.)
