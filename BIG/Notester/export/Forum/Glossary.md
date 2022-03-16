# Forum.Glossary --

    GLOSSARY
    ========


    SUMMARY -- A node summary is a description of the meaning of the node, 
        and an arrangement and description of the node's children.

    REVISION -- 

    REPLACEMENT -- 



    proposal -- 

    ENDORSEMENT --


    ISSUE -- An issue is a topic area for debate.  
        (e.g. Gun Control)

    POINT OF CONTENTION -- A specific point of contention.  
        (e.g. Effect of gun posession on the safety of the wearer)

    POSITION -- A conclusion or sub-conclusion about an issue or point.
        (e.g. Handguns increase the safety of the wearer)

    JUSTIFICATION -- Supporting argument for a position.
        Study ... in the New England Journal of Medicine and ...

    VARIENT -- An alternative for an issue, a point, position or justification.
        (e.g. Handguns decrease the mortalitity rate of adult wearers)


    SUPPORTING OPINION -- For a position, justification, or opinion

    OPPOSING OPINION -- 

    NOTE -- A message posted by a participant



    PARTICIPANT -- A person that posts notes, opinions, or positions.

    REPUTATION -- Scores on the accuracy, impartialitity, etc. for a person
        or an organization.


    MAJOR NODE -- A node with significant support relative to its siblings.
        (Perhaps it has at least x% of the median level support)

    MINOR NODE -- A node with some minimal support.

    PROPOSED NODE -- A submitted node with less than minor node support.


    ---- Varient Mechanism ----
    Use summaries instead.

    VARIENT -- 

    CANONICAL VARIENT


    ---- Fact Mechanism ----

    ASSERTION
    VOUCH
    CONTEST


    
    Node: structure, Up: Top
    * Menu:                                 


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
