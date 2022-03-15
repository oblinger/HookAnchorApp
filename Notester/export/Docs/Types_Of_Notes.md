# Docs.Types\_Of\_Notes --


    Notester has several different types of notes.  Each type has different default 
    behaviors consistent with their intended usage.  Here are the key types, 
    how they are created, and retrieved.

    JOURNAL  -- The simplest item is a journal entry, it has no name and no 
                associate files, it represents a chunk of info typically entered 
                at one time (as one might in a diary)

    ACTIVITY -- An activity is used when files are associated with the note, and 
                when it is expected that one will go back to that note over a short 
                period of time (days or weeks).

    PROJECT  -- Is an activity that is given a permanent name.  It is a task that 
                is active over months or years, and often contains other activities.


    <<>>

      TAB-Q-P   [Q]UICK-CREATE  [P]ROJECT
                This will create a new toplevel project with its own directory.
                (TAB-E will [E]XPLORE that directory)

      TAB-G     [G]OTO project by name

      TAB-S-P   [S]EARCH  [P]ROJECT
                This list all <<>> in reverse chronological order.

    <<>>

      TAB-W-A   [W]ITHIN-PROJECT  [A]CTIVITY
                Creates an activity *within* the current project.  It is a
                good idea to try to put activities into project since you cannot
                find them by name.

      serach    To find such an activity TAB-G (goto) its containing project
                and use the 'Children' view (on the left) to look at activities
                associated with that project.

      TAB-Q-A   [Q]UICK-CREATE  [A]CTIVITY
                This will create an activity under the 'General' project.
                This is used for activities not associated with any project.

      TAB-S-A   [S]EARCH  [A]CTIVITIES
                Lists all activities in all projects in reverse chronological order.


    --- JOURNAL ---    
              
                Journal command mirror the activity commands above.  When possible
                use the 'TAB-W-J' command to add journal entries [W]ITHIN a 
                particualar project.

      TAB-W-J   [W]ITHIN-PROJECT  [J]OURNAL  create
      TAB-Q-J   [Q]UICK-CREATE    [J]OURNAL  (under the 'General' project)
      TAB-S-J   [S]EARCH all      [J]OURNALs



    -------------------
    --  OTHER TYPES  --
    -------------------

     TOPICS     -- Non task info  (like list of computer accounts)
     MEETING    -- Like an activity or journal but capturing the contents of a meeting
     SUBPROJECT -- A project contained in another project
