# Notester.Streams --


    [GOALS]
      - look at recent log entries, recent meetings, recent meetings in project

    - ALL new notes (and note edits) are records in streams

    - Stream viewed in reverse chronlogical order
    - Stream can be split into substreams in hierarchy
    - Streams can be split by multiple hierarchies.

    * Each create/update will have a location on *each* hierarchy.
      By default it will be under 'other' or null or n/a



    AUTOMATIC HIERARCHYS   Time (auto), Types (specified by creation template), Proj (defaults to current project)
    *** TIME:  Automatically entered.  Really two hierarchies:  Creation time, Modification time
    *** TYPES:  TC, REF        (default is TC for many creation templates, and )
        - TC (transitory communication): Phone, Email(&IM), Web, Meeting
        - TC: Auth (group authored stuff)
        - REF (default, type=null):  Reference material (time independent primary structure)
          Code(&API, Documentations, etc.); Documents
    *** PROJ:  Hierarchy of projects/sub-projects
        - [SUB-]PROJECT-NAME
        - [SUB-]TOPIC-NAME
    *** TOPIC: Place within the notester topic hierarchy (non-project pages)  (category like patents)

    OTHER INFO
    *** NAME/TITLE single name title field
    *** UP (parent)




    Key Strokes

    TAB-Q-P Phone
    TAB-Q-M Meeting

    "TAB"  "Q"  
         ","    RETURN
       "P"  
       "P"  TAB   RETURN


    KEYS:  N=name,  =parent, T=type, P=proj, C=category, 
