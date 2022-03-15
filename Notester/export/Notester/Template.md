# Notester.Template --

    This is the library of note creation templates.
    They are used by the QuickCreate command.             (Documentation <<>>)

    ROOT LEVEL TEMPLATES     (Stored under <<>>)
     <<>>              Creates a dated activity
     <<>>               Creates a dated journal entry in a master journal
     <<>>               Creates notes entry for a non-project specific meeting
     <<>>               Creates a new project entry
     <<>>                 Creates a new topics category

    PROJECT SPECIFIC 
     <<>>         Creates an acticity contained within a project
     <<>>            Creates a simple child of the current note
     <<>>          Creates a dated journal entry under the current project
     <<>>          Creates a named note under 'meetings' under the current project root.    
     <<>>      Creates a project that is a child of some project, and is in a directory under the parent's
     <<>>            Creates a topic within a project (unlike sub-projects, topics do not have their own folder)

    STANDARD TEMPLATES       (These are used implicitly by various Notester creation mechanisms)
     <<>> Used when .nstr file causes note creation for unrepresented directories
     <<>> Used when .nstr file causes note creation (
     <<>>         Used by the 'CreateChild' command
     <<>>          Used when new todo list entry is created
     <<>>    Used to create a new root note for adding a group of attachments (NOT YET USED!)
     <<>>          Used by TAB-` to create todo item placed in the InBox

     <<>>  Used when user clicks on note's children list (view note children of a single type)

    FIXED CATEGORY TEMPLATES
     <<>>

    TOP OF THE TEMPLATES TREE STRUCTURE
     <<>>

