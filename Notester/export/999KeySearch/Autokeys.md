# 999KeySearch.Autokeys --

    Automatically generating the rkey mappings 
     
     
    MAPPING TABLES 
    - UNAME-SET 
    - QUERY-MAP:   Query term combos -> Rrefs 
    - RREF-TABLE:  Set of RREFs on the intra-net. 
    - BINDING:     RREF + QUERY-SCOPE -> URL  
    - SCOPES-MAP:  Predicts scope of a URL 
    - TERM-DIST:   Distance measure between terms 
     
     
    HOW COMPONENTS CAN BE LEARNED? 
     
    * UNAME-SET:   Simple stats on user entered queries. 
                   Stats on page titles 
     
    * UREFS:       Click thru rates 
     
    * QUERY-MAP:   Induced from user query trace 
                   (System peppers normal search results with RRef entries & 
                    watches terms before RRef click)  
     
     
     
    * BINDING-MAP:  
     
    * SCOPES-MAP:  Induced from the BINDINGS-MAP. 
     
    * TERM-DIST: 
      From Bindings (compare terms on bound page w. RRef terms 
      From Text (wordnet scores over intranet) 
      From QueryMap 
     
      To QueryMap 
      To Binding 
     
