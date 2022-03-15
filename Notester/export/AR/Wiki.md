# AR.Wiki --

    <<>>


    DATA PIPE TODO ITEMS


    -- HIGHEST PRIORIY --

      MessageRouter.java
      - Transfer methods from original. update message objects to the non-map versions of PWEntities.
      - Replace segmenter support with simplified built-in segmenter.

      DWSystem.java
      - Build plug board class from scratch.
      - Write simple initialization methods for support of testing methods for various parts of system

      RPRINT support (simple print method for internal datastructures will assist in development<<>>debugging)
      - extend rprint for Entities, Traces, ProcedureStep, WorldModel, etc. 

      L-NAME refactoring.  Translate old l-name code to new platform just
      using new uNmames to start with.

      TIMESTAMPS -- add timestamping to PWEntities and provide automatic
      stamping in the message rounter.  (used by many parts of system
      including sequencing world model updates.)

      WRITE TRACE -- output 3 XML files formate one for each of the following datastructures
      - messages
      - world model instances
      - state action pairs

      REFACTOR TRACE COLLECTION AWAY  (Manually set DWSystem pointer in each trace & move the AttrMap into DWSystem not trace collection)
     
    -- DEFAULT PRIORITY --

      Object Input <<>> Output
      - Extract relevant reading/writing code from pwXMLutils.java into IO.java
      - Walk Node Trees to build XML Doms
      - Walk XML Doms to build Node Trees
      - Walk Java objects to build Node Trees
      - Walk Node Trees to contruct native java objects.
      - Write Testing cases for IO.java

      Entity objects
      - Handle parent/child, and world model lookup methods for Entity.java and subclasses
      - Write entity validation methods for all Entity.java subclasses
      - Create top-level Entity types for DocWizard system:  UserAction, WorldEntity, ProcedureStep, Message, 

      Trace.java 
      - Transfer relevant methods from original.  reroute calls to WorldModel (which is not handled seperately).
      - Write new 'acceptMessage' method with DocWizards logic.
      - Write XMLizable interface

      WorldModel.java
      - Transfer relevant methods from trace.java original.  
      - Update code to handle non-map Entity types (particularly hierarchy parent/child method need to be updated)
      - Write XMLizable interface

      StateAction.java
      - Write XMLizable interface

      Config.java
      - Build simple configuration methods for DocWizards  
        (*all* components in System.java should rely only on these config parameters so that System.java can have a meaningful null constructor.)

      ComputeAttrs.java
      - Build method per world-model feature from scratch.  



      VALIDATION  --  hard bugs occur when data-structure become inconsistent.
      - Many classes will have a 'validate' method that checks that the instance in question is in a valid state.
        (for example, an Enitty with a trace and an id set, should be the same entity returned when looking up that id in that trace.)
      - Create validation toplevel that calls these validation methods in a few standard places in the data pipe.


      READ TRACE -- Read a trace object from file

      DEEP COPY -- Performs


    -- LATER --

      RPRINT -- simple printing method for all core data structures in system (primarily for debugging)
      - Handle cyclic data structures





    TOPICS
    - validation
    - XML parsing
      - Implementing XMLize inputon each Entity type

    - Input/Output
      - implement XMLize on each entity type
      - implement XML parsing 
      - XML output to file
      - XML Cursor methods for input and output


    LATER
     - Adding support for XML hierarchy
