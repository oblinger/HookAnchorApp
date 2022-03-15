# 999forum.Structure --

    ================= 
    === STRUCTURE === 
    ================= 
     
     
    <<>> 
    |Note type   | Legal Childern of a note                 | 
    |            |                                          | 
    |            | Topic   Position Support Oppose  Disuss  | 
    <<>> 
    | Topic      | X       X        X       X       X       | 
    | Position   |                  X       X       X       | 
    | Support    |                  X       X       X       | 
    | Opposition |                  X       X       X       | 
    | Discussion |                  X       X       X       | 
    <<>> 
     
    ============= 
    === TYPES === 
    ============= 
    > Topic Area       Area in space of topics 
    > Issue            Issue for debate 
    > Position         Postion on an issue 
     
    > Summary          Summary of a note 
    > Alternate        Proposed replacement for a note 
    > Support          Support for parent note 
    > Opposition       Opposition for parent note 
    > Discussion       Discussion on specified point 
     
     
    ?? 
    > Evolutionary summary 
     
     
     
    ============ 
    === NOTE === 
    ============ 
    A note is the basic building block in the forums representation of the 
    on going discussion.  Each note represents a submission by a participant. 
    They have the following fields: 
     
    CONTENT 
    > Title 
    > Synopsis 
    > Body 
    > Details 
     
    STRUCTURAL 
    > Parent      The note that provides the context for this note 
    > Type         
    > Relation    Alternate, Revision 
    > Children    List of all notes that list this note as their parent 
    > Alternates  List of alternates for this note 
     
    ADMINISTRATIVE 
    > Author 
     
     
     
     
     
    X> Relation    Relationship of this note with its parent. 
                  Refinement, Summary, Revision, Alternative, 
                  Support, Opposition, Decision 
                  Discussion, Comment 
     
    X>RELATION TYPES 
      Revision        A revision (w. same content, and same author) 
      Alternative     A note that seeks to replace its parent 
      Refinement      A subtopic of the parent note 
      Summary         A summary of a note and its children 
      Position        A point of view on parent 
      Support         A note that supports its parent 
      Opposition      A note that opposes its parent 
      Decision        A pronouncement regarding the parent.  Off topic, etc. 
      Discussion      A discussion note within parent area. 
      Comment         A commentary on the parent note itself. 
     
     
     
    BASIC SEMANTICS 
    =============== 
     
    NODE.  The alternates and revisions links form an equivelance class. 
    A node is an equivelance class of alternates and revisions. 
     
    SUPPORT. Sum of support, support of support, and opposition of opposition. 
     
    OPPOSITION. Sum of opposition, support of opposition, and opposition of support 
     
     
     
     
