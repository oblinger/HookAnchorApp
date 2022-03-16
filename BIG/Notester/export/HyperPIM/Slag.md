# HyperPIM.Slag --

        <<>>** 
     
    !Hpim b src=a.sys.emacs.DietList 
     
    [DietList] 
     
     
     
     
    * DietList1999 -- 
      * DietList.Log 1999 
      * <<>>  
      * Ref: Me baby 
     
     
    [<<>>] 
     
    [LogX] 
    [LogInfo] 
     
    [[DayLog]]                                                   <<>> 
    loc=a.sys.emacs.info 
    type=ParentNode 
     
     
    [ToDo] 
    02Skill: Skill one item 
             Skill two item 
    BioLink: This is line1 
     
     
     
     
     
    $$__ 
    [DayLog.Keys] 
     
     
    * NodeValue --  
     
     
     
        *<<>> 
     
     
     
     
    <<>>** proposal 8<<>>10<<>>01 
     
    ... add NodeMgr.getEncoding(...) 
     
    INTERMEDIATES-DESIGN 
    - raw form is parsed into intermediate body-parts form 
    - info-files are the base rep used 
    - .html is linearly parsed (if needed later) 
    - Output to HTML is done 
    - Output to editable text is done 
    - Input from editalbe text is used as the base rep (maybe later or over time) 
     
     
    <<>> 
    Proposal 12<<>>10<<>>01 
    - Nested relative namespace used to reference knowledge objects 
    - Methods: 
      - GetNodeHTML        Loc 
      - GetEditableForm    Loc 
      - UpdateEditableForm Loc NewText 
      - FollowLink         Loc Pos 
      - Click              Loc Pos 
    - EditMethods: 
      - Create<<>>Rename<<>>Delete node 
      - Move<<>>Copy<<>>Delete tree 
    - Location Spec 
      - Uniquely specifies node location using "dot.notation.name" 
    - BackEndMethods: 
      - ComposeNode loc 
      - SaveNode 
      - ResolveLink Loc LinkText 
      - LookupNo 
      - GetNodesUnder 
    - Xlates into backend pageget pageset 
     
     
     
    DESIGN 
    > TOPLEVEL 
      - IncorporatePage 
      - GeneratePage 
      - ClickAt 
    > PRIMARY METHODS 
      - ComposeNode 
      -  
     
    > SUPPORT METHODS 
      - ParseText    string 
      - Output       writer html recursive 
      - GetNodeText  loc 
      - SetNodeText  loc text 
      - FindNode     loc text 
     
    > TOPLEVEL 
      - editor control (watch master file) extract click mark [%PTR%|cmd] 
      - incorporate (calls parser then incorp) 
      - browser (call findNodes<<>>searchNodes; instantiate; and writer) 
      - click (call read component at;  
     
    > PAGE MANAGER (List of nodes; expanded form) 
      - static list of NodeSources  
      - static incorporate                      Writes parsed Node structure back 
      - static findNodes, searchNodes -> Page 
      - instantiate  
     
    > NODE SOURCE 
     
    > NODE I<<>>O PARSER<<>>WRITER      reads<<>>writes components<<>>nodes<<>>pages 
      - Read(text, idx) 
      - ReadObj(obj, text, idx) 
    xxxx  - ReadComponentAt(page, idx) 
      - Write 
      - WriteObj 
     
    > COMPONENTS 
     
      
     
     
     
    full.path.to.the.node  
    [NiceName]  
     
    Here is a simple [embedded] link. 
     
    [EmbeddedSection]  
     
    This is a simple list 
      FirstRef          This is a reference node 
      EmbeddedRef:      This is a single embedded line 
      MultiEmbed: 
        This is a multi-line embed 
        SubNodeEmbed: 
          You get the picture 
      ClosedLink+ 
     
     
    [ThisIsALocalRefDef]  Title line 
     
    [AnotherLocalDef] 
    Embedded stuff is always glued to top 
     
     
      [FR:First Ref]    This is a reference node 
      <<>>  
      [Another Ref]  Another ref 
     
     
    ToDo 
    - Master 
     
     
     
     
     
       
     
    *<<>> 
     
    <<>>**  
     * 
     *  build cache 
     *    URLs -> names 
     *    cacheName 
     * 
     *  BASIC COMPONENTS 
     *   
     *  NODE             -- A text object/container. 
     *  NODE_COMPONENT   --  
     *  URL              -- Unique location of a node 
     *  NAME             -- Short node name (requires xlation to URL) 
     * 
     * 
     *  EXAMPLES: 
     *  URL:       <<>>|<<>> 
     *             <<>> 
     *  NAME:      camping 
     * 
     *<<>> 
     
     
        <<>>** Converts the location into a file path specifier. <<>> 
        public String getPath() throws Exception <<>> 
        <<>>** Generates a pretty location name. <<>> 
        public String getPrettyLocation() throws Exception <<>> 
        public NodeSource getNodeSource(Node node) throws Exception <<>><<>>**<<>> 
     
     
