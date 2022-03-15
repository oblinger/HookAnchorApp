# WMRdiag.Modules -- Component level breakdown of the assessment tool

    PACKAGES/CLASSES 
     
      WMRDIAG          --- Application's toplevel 
    #   WMRdiag         -- Initialization 
    #   V               -- Global variables and constants 
    x   Toplevel        -- Toplevel control 
        InitialScreens 
     
      DISPLAY          --- Display objects 
    #   Display         -- Root object 
    #   View            --  
    x   Card            --  
    #   RowDisplay      --  
    #   BarGraphDisplay --  
    #   LabelDisplay    --  
        VBarGraphDisplay-- 
     
      DATA             ---  
    x   Location        -- All params necessary to specify a particular data view 
    #   Constraint      -- Selects some subset of entire dataset for analysis 
    x   Statistic       -- Fns over DB tables (like accuracy) 
    #   Rdb             -- Relational Database support 
                           Data Loading; Caching; Selection  
    #   Table           -- Basic Database object, and interface 
    #   DataRow         --  
     
      UTIL             --- Generic utility modules 
    #   Namespace       -- 
    #   Plist           -- 
     
      HYPER            --- Help display routines 
     
     
     
     
    >>>WMR-DIAG PACKAGE<<< 
    - WMRdiag      -- Top level control 
    - Destination  -- One of: a help node, a category, a screen, a method 
                      Controls browser update when user clicks, types, etc. 
    - Category     -- Constraint used to select a subset of the data for analysis 
                      (used to select particular student, book, date-range, etc.) 
    - Attribute    -- A real-valued function over a set of rows. 
     
     
    >>>DISPLAYABLE OBJECTS<<< 
    - Display      -- Super class for all screen classes  
    - Screen       -- Defines screen image (a List of cards) 
    - Card         -- Defines a panel on screen (List of Categories and Displays) 
    - DataDisplay  -- Defines a single display element (Display parms & attribute) 
    - Locator      -- Locator bar on top of data screens 
     
     
    >>>RDB PACKAGE<<< 
    - Rdb          -- A relational database 
    - RdbRow       -- An rdb row (one element of data) 
    - Attribute    -- Computes a value given a database row 
                      (Specifies: Title, HelpNode, Range, AttrMethod, AttrIndex) 
    - Range        -- Specifies the range info for an attribute.  Including the 
                      tick marks that should be display on screen 
                      (may need to be dynamically computed in future displays) 
     
    >>>HYPER PACKAGE<<< 
    - Node         -- A node of hypertext.  Contains an array of text entries, 
                      an array of destination nodes, and optional up, next,  
                      and previous nodes. 
    - Popup        -- A popup menu node (used to show abbreviated help, or to 
                      allow user to select a destination 
     
     
    >>>OBJ package<<< 
      Supports file-based program configuration.  Module associated execution 
      time objects with an ASCII name, so that their configuration can be  
      specified in an ASCII flat file. 
    - Plist        -- key/value objects w. inheritance 
    - Namespace    -- Named object support including: reading/writing to file 
                      Converting Plists to native Java objects 
     
     
    >>> OBJ package details<<<  
    PLIST CLASS 
      An array of Object: <<>> 
    - get0(field)        -- Gets a property directly from the plist 
    - get(prop)          -- Gets a property including inheritance 
    - set(prop, object)  -- Sets a plist property value 
    - get/setParent, get/setName, toString 
     
     
    NAMESPACE CLASS 
    - Namespace() 
    - lookup(string) 
    - get(string) 
    - setFwdRef(object, field, fwdRefObject) 
    - define(string, object) 
    - define(string, class) 
    - assign(string,object) 
     
    - readObject(inStream) 
    - writeObject(outStream, object) 
    - loadFromFile(fileName) 
    - saveToFile(fileName) 
    - plists2Objects 
    - writeNamespace(fileName) 
    - fillObject(object) 
     
     
    > OBJ package INTERNALS 
      A fwdReference is a linked list of objects and fields that should point 
      to the yet-to-be-loaded object.  When that object is loaded, the forward 
      references are resolved. 
     
      A namespace is a hashtable.  The associated javaPackage defines the  
      associated java namespace for the classes of the objects loaded. 
     
     
    OBJ FILE FORMAT 
     
    > typeName objName 
    propName1 <<>> 
    propName2 123 
    propName3  
    propName4 "This is a string" 
    propName5  
     
     
    Example: 
    > Display.StdAccuracy 
      bgColor:    
      width:     70 
      height:    20 
      bgClick:    
      fgClick:    
      title:     "Reading Accuracy" 
      desc:      <<>> 
      popup2: 
        This bar shows the accuracy 
        of the students reading 
      justNumbers: <<>> 
      test:  
     
     
    >>>DISPLAYABLE OBJECT PACKAGE DETAILS<<< 
     
    > DISPLAYABLE OBJECT 
      The superclass of all objects (or collections) are displayed on screen. 
      Extends the java Panel class and implements appropriate screen update methods 
    - Fields: location, size, title, titleOffset, titleFont, 
              font, fgColor, fg2Color, bgColor 
              plist, category, value 
    - Compute()   -- [Re]builds display data from student data. 
     
    > SCREEN, CARD, & DISPLAY OBJECTS 
      These are simply container object organize the student data 
     
    > H-BAR-DISPLAY 
      Horizontal bar graph.  Each bar extends from left to right and multiple 
      bars may be stacked. 
    - Fields: 
      category, sub-category 
       
     
     
    > HyperNode AccDspHlp 
    text: 
      The accuracy bar shows the percentage of %a utterances %a 
      spoken by the student that were recognized by the 
      computer as correct. 
    text: <<>> 
     
     
