# 999inf-ext -- New features for emacs info mode

     <<>>     
     
     <<>>     
     <<>>     
     <<>>   
     
    Make "normal mode work correctly" 
     
     
     
     
     
       
     
     
     
     
     
    OUTSTANDING FEATURES TO ADD 
     
    -9323 A links page per .info file. 
          ability to "throw" a line to a node or link 
          ability to "go" to a link 
     
    -7213 Mouse select: cut and append to existing node  (Tia) 
    -7212 Make a backup button that really returns from a jil excursion. 
    -7112 Push <<>> pop info location (to go back or flip between to previous loc.) 
     
    -7114 Integrate following info: 
    EMACS INFO EXTENSIONS 
     
    * Quick generation <<>> organization of emacs info pages. 
    * Std. info. packet with many views 
    * Need not choose orginazation when K is added. 
    * Need not have one orginazation. 
     
    - Single key to create entry in log.info file 
    - Cmd to "integrate" page. 
     
     
    IMPLEMENTATION 
    - Doc-DoPages iterates over the pages in a document. 
    - Doc-DoInfoFiles iterates over a list of doc files (in a list of files<<>>dirs) 
    - Doc-title, Doc-Id, Doc-start, Doc-end, Doc-Parent, Doc-Children 
     
    - Integrate page command:  Uniqueifies id.  Adds entry parent's children. 
      Check that all children exist & have correct titles 
     
    Format of a log.info entry: 
     
    Node: [id] 
    *Menu: [parent `:'] [Title] 
     
    <<>> 
     
    body .... 
     
     
    12<<>>13<<>>96 - 12<<>>20 
     
    Features: (in both edit and search mode) 
    CMD: Quick jump cmd to lists 
    CMD: Inline info subnodes 
    CMD: Create sub node 
    CMD: Delete a leaf node 
    CMD: Move selection to a subnode at end of current page 
     
     
    -- is expanded to: 
     
    +<<>> ::  
    body ... 
    +<<>> 
     
     
     
    ======== 
    12<<>>15<<>>96 
     
     
    EMACS INFO EXTENSIONS 
     
    * Quick generation <<>> organization of emacs info pages. 
    * Std. info. packet with many views 
    * Need not choose orginazation when K is added. 
    * Need not have one orginazation. 
     
    INTERFACE COMMANDS 
    - Create entry in log.info file 
    - "integrate" info page (perhaps added to the save file key) 
    - Convert selection into an inline page. 
    - Toggle between inline and child entry. 
    - Allow inline edits (re-imports inlines; sets write-back flag for "integrate") 
     
    IMPLEMENTATION 
    - A page is the cons of a document and the position of its ^_ character  ???? 
    - Inf-DoPages iterates over the pages in a document. 
    - Inf-DoInfoFiles iterates over a list of doc files (in a list of files/dirs) 
    - Inf-title, Inf-Id, Inf-start, Inf-end, Inf-Parent, Inf-Children 
    - Inf-Create adds page 
     
    - Inf-Title-Line returns a single menu entry line for page.  Fmt: 
      "*" id "::- " title 
     <<>>   General personal info manager 
     
    - Integrate page command:  Uniqueifies id.  Adds to entry's parent's children. 
      Delete children that do not list page as parent. 
      Update child-titles and parents child-title. 
     
    Format of a log.info entry: 
     
    Node: [id] [date] 
    *Menu: [parent `:'] [Title] 
     
    <<>>  
     
     
     
     
    EXAMPLES:  
     
    This is a simple node 
     
    +examples17:: Samples of the system in use 
     
    A full featured example 
     
    Then perhaps a simple bare bones example 
     
    + 
    Remaining text.  The inline above would be converted 
    to the following menu entry: 
     
    *example17:: Samples of the system in use. 
