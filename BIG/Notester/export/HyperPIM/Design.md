# HyperPIM.Design --

    [PrimaryDesignPoints] 
    - UNIQUE URL.  Each node has a unique NodeSource, and address 
    - Each is embedded in a namespace (?derived from the address) 
     
    - Info&html pages have #anchors 
     
     
    NODE_SOURCE FILESYS DESIGN 
    -  
     
    NAME_SPACE_DESIGN 
    - single name links 
    - simple relative links 
    - absolute linking possible 
    - combining names  
     
     
     
    [PrimaryComponents] 
     
    - PAGE:    Basic interactive unit; made up of nodes 
    - NODE:    Atomic storage unit, hunk of text 
    - LINK:    Merged mulitword.  Linking is always relative to current node 
    - KEYS:    Attribute value pair associated with node 
    - ADDRESS: Each node has a unique address 
     
    PAGE 
     
     
    NODE 
    - name 
    - title 
    - parent 
    - address 
     
     
    LINK 
    - A link is any MergedMultiWord 
    - A link is also any word w. a synonym or sub node inerited by this page. 
     
     
    KEYS 
    - Inheritance Order: node,keys subnode, type super node 
     
        
    ADDRESS: 
    - Address prefix key used 
    - Sub node header appends name onto parent 
     
     
    ===  In 
     
     
    ================== 
    ===  CATEGORY  ===   ??? 
    ================== 
    - Taxonomy of all pages 
      - Each page is in a single category 
      - Each category contains pages directly and also indirectly 
        through containment of other pages. 
      - Category is specified as a keyword 
      - Reverse index of pages within a category is also maintained 
        under the [CategoryIndex] subnode 
     
    =========================== 
    ===  New Node Creation  === 
    =========================== 
    - Parameterless creation generates a node 
     
     
     
    TEMPLATE CREATION 
    - Copy node to new location 
     
     
     
