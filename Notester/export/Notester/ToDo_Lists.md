# Notester.ToDo\_Lists --

    <<>>   
    <<>>

    <<>>


    REQUIRED FEATURES
    * TASK & SUB-TASK HIERARCHY  (and operations on groups based on hierarchy)
    * MULTIPLE VIEWS
      * SYNC EDIT
      * WIHIN VIEW GROUPING
    * AGGREGATION
    * PRIORITIZE   ( DUE&SHOW DATES )
    * THROW

    ?  Todo Items should be enbeddable in other notes


    ASSUMPTIONS:
    * Designated todo notes anywhere in notester contains todo info.
    * Each line contains a single todo entry
    * Throw command moves single lines around  (to different lists)

    * Need to shift status of todo entry:  <<>>


    * Show 'today' note





    - Get embedding to work
    - Make 'prefix' mode indent items and remove blank between categories




    [OPTION: USE SPECIAL TODO VIEW WINDOW]
    - Editor w. control over insert/delete line


    =========================================================================================================

    [PROPOSAL:  ONE-LINE ENTRIES]
    - Todo list and project lists are all notes under the 'lst' namespace.
    - Todo region is part of note after '[Todo]' or is the whole note.
    - PREFIX: If prefix equals one of the list names then it is short hand for a ref to that list.
              'd' move to done; 'c' create in same proj as previous; 'n' next action. mark previous line as done and create next action in same proj
    - Each entry is in at most two lists.
    - Each list is a sublist of another list (the note's parent)
    - Each list may also be *listed* on one of the project collection lists.


    FORMAT:
      ' '  [  ':' ]  


    EXAMPLES:
    ...in the work list
    - PROJ_NAME:  Example of an action in one of the global todo lists.
    ...in PROJ_NAME
    w  Example of an action in one of the global todo lists.
    c this item can be acted on when previous is done  (this is the default)
    ,w this one can be acted upon at same time as previous
    -  this one is executed after both previous are don
    ...in Active Work Projects
    - <<>>   This is the entry for the project itself
    ...in Lst.July
    >c  Specifies that a call should be made to Joe Smith 856 555 1212
    ...



    COMMANDS:
    - Create entry
    - Edit entry
    - Delete/Done entry     move entry to 'done' and time-stamp entry.
    - Throw entry           move entry to specified list.

    - Move up/down          move up/down one line or one group or top bottom
    - Complete project
    - Process List          move each '>' entry as needed, and move remaining entries to default processing targets


    IMPLICIT ACTIONS:
    - if line is modified, then modify paired line
    - if line is added, then create paired entry if needed
      if paired project name is new, a new project is created w. parent MiscProj and is listed on ActiveProjects list
    - if line is deleted, then move to done

    KEYSTROKES:
    - TAB-T  D=done, N=next, =movement, 

    METHODS
    - get/cut current line
    - get paired link




    i
