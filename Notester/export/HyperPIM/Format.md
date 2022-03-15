# HyperPIM.Format --

    [KEYS] 
    __parent=this value 
     
     
     
     
     
     
     
     
    Objective to provide multiple realizations of the following structure 
    - Head: Element marks beginning of node.  Specifies: name, address, type 
            Type: Node, SubNode, EmbeddedNode 
            End of node is implicit, beginning of next node or thru indention 
            Name/Address may be implicit in source of content. 
    - Key:  Key value pair 
    - Text: Constant text (w. markup) 
    - Link: Pointer text w. display form and dest string 
     
     
    Supported Actions: 
    - Output HTML to browser 
    - Output HTML to JEditorPane 
    - Input  HTML from JEditorPane 
    - Input  HTML from external editor 
    - Build  ANNOTATIONS from document  (List of nodes w key maps) 
    - Split  DOCUMENT 
    - I/O    HTML to/from backend files 
     
    - Input  INFO files 
     
     
    - File read/write  
     
     
     
     
     
     
     
     
    OUTPUT HTML REALIZATION 
    Head: 
       '>>>' <<>> <<<>>*> 
       '[' <<>> ']' <<<>>*> 
     
    Link: 
       <<>> <<<>>> 
     
    Key: 
      '__' <<>> '=' <<>> 
     
     
     
     
    - Mapping from plain text to HTML markup. 
      Structure: 
      page -> node 
     
     
     
     
     
     
     
    >>> MasterToDo 
     
    Proj1: line 1 
    -      Line 2 
    Proj2: whatever 
    - Still in proj two 
    - SubProj2.2:  This line 
      plus this one 
    - but not this one 
    Proj3: this line 
     
    not part of any  
     
    [Proj1] 
    def of node o.a.p.17-hpim.src.MasterToDo. 
    Proj1 
     
    End of embedded node: 
      - Blank line 
      - less equal indented embedded node, or  
      - less embedded line 
     
     
    [Keys]        ;; Address prefix.address.DefHeaderForNode.Keys 
    __parent=value 
     
     
     
    [ToDo] 
    -  
     
     
     
    ;;; Alternates 
    >>> DefHeaderForNode 
     
     
