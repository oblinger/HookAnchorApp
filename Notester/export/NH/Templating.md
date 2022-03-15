# NH.Templating --


    COUNTER-PROPOSAL:  Just make renaming change directories.

    PROPOSAL:  Applying a template to note (and its decendents) as well as files controlled by that note.
     - Moves tree root (overwrites root fields with template fields, which may change parent)
     ? If parent namespace changes, then perform move on tree to the new namespace 
     - relocate files to locations they would be placed if they were created as attachments to the new tree.
     - remove empy directories




    - When new topic is created it is created using a default template
      Then when state changes, retemplating is used to move it to new location (and move files too)





    INLINE SERACH RESULTS
     * Build page from concatenation  *AFTER* contents of current page.  
     * Default 'add info' operation == template create where name of template is inherited from current page
     * Program default template for project to go to as child of LOG page
     * Program log page to concat children

     
    [DONE]
    RELATED SEARCH CHANGES
    - Add new search check for 'project roots' only
    - Add project root flag
    - Add search projects page
