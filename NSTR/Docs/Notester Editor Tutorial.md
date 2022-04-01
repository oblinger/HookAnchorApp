# Docs.Notester Editor Tutorial --


    [INTRODUCTION]

    You already use the filesystem on your computer to keep track
    of projects, coorespondance, documentation, and other information.
    Notester assists you in that task, and unlike the general
    purpose filesystem, Notester was designed solely for that task
    of organizing your information.


    [OVERVIEW]

    Notester information is organized into a set of pages called notes.  Each note is like a 
    web page that can be edited on the fly.  Like web pages these notes have an address 
    (their name), a title, and links to other pages contained in them.  The note you are
    reading right now for example has the name 'QuickStart' which is listed at the top
    line in the display.  Its title 'Quick start tutorial for Notester' is listed in the 
    line below that.

    Notester pages are organized into folders and sub-folders like the directories on 
    your computer.  Just like each file has containing folder, each note has a parent.
    This note has the parent note 'Docs' which can in the top bar on the right. 
    Children of a note are listed in the panel on the left.  This note has a 'TestChild'
    and 'SecondTestChild' listed there.

    We will see this relatively simple tree structure with hypertext links provides 
    a manageable organization of large amounts of personal information.  


    [BASIC NAVIGATION]

    Clicking on one of the child notes listed in the left will goto that child--it will
    cause that child to be viewed in the main window.  Similarly clicking a link in
    the body of a note like <<>> will go to the view of that note's contents.

    Clicking the "<=" button on the top line of the Notester display above will go back 
    to viewing a previously viewed note.  Please Click the <<>> link now to see
    this jump to the new page.


    [BASIC EDITING]

    A central Notester design theme is the simplicity of information entry.  A page
    can be changed by simply editing it directly.  No "edit" mode is needed, and no
    saves are required.  Changes simply become part of that note.  A history of all
    edits is maintained so inadvertent modifications can be recovered from.
    (The history commands needed to access this info is not yet written, but you
    can *carefully* copy old version from the notester.log in the notester directory.)

    Most of the conventional Windows editing features keys and mouse operations are available 
    for this editing, so we will not elaborate those operations here.  You can go
    to the <<>> an modify that note, then return to see that the changes were
    permanent.


    [TAB COMMANDS]

    Just as contents can be modified on the fly, so can the structure of the notes themselves.
    In this section we describe a small set of commands which are sufficient for creating,
    modifying, and navigate this tree of notes.  Each command below is executed by pressing the TAB 
    key followed by the letter key.  We provide a nemonic word to remember each command by.

    A is for ATTACHMENT.  attaches some file to the current note
    B is for BACK.        goes back to the previously viewed note
    C is for CREATE.      creates a child note for the current note
    D is for DELETE.      deletes current note
    E is for EXPLORE.     shows the directory associated with the current note
    G is for GOTO.        goes to a node by name
    H is for HELP.        shows the help info
    I is for ICONONIFY.   iconifies notester so you can get some real work done
    K is for KILL.        kills (deletes) current note
    N is for NAME.        sets the Name of the note
    P is for PARENT.      set the Parent of the note
    Q is for QUICK.       creates a Quick note without name or parent
    S is for SEARCH.      searches for a note contains specified text
    U is for UP.          goes up to the parent of the current note


    TAB-A  Prompts the user for a file to be attached.  A child is created for the current
           note with the file attached to it.

    TAB-B  Goes back to viewing the note view before this note

    TAB-C  Creates a child of the current note.

    TAB-D  Deletes the current note.

    TAB-E  Explore filesystem directory associated with note.

    TAB-G  Goto a named note.  To avoid confusion letter case ('A' verses 'a'), spaces, dashes,
           and underscores are ignored in note names.

    TAB-H  Show the top help page for Notester.

    TAB-I  Iconify Notester.  (Do this rather than exiting Notester)

    TAB-N  Rename this note, and update all children notes for this
           note so that they refer to the new name as their parent.

    TAB-P  Change the parent of this note, and change the list of 
           children for the old and new parents appropriately.

    TAB-Q  Create a "quick" note whose name and parent are automatically
           generated.  (These notes are only organized by time entered initially.)

    TAB-S  Search all notes for a specific text string.

    TAB-U  Go "up" to the parent note of the current note.
           generated name and parent.


    USE THE BACK COMMAND (TAB-B) TO THE MAIN TUTORIAL PAGE

