# ur.Association\_Engine --





    [CORE-API-METHODS]
    - Assert trigger rule   SYM_1, SYM_2, ..., SYM_i  =>  SYM_out
    - Assert/Retract SYM
    - Get Triggered/Retracted Symbols


    [EXTENDED-API-METHODS]
    - Global control (clear memory, clear symbol groups, fire rules (upto max levels))
    - Assert/Retract Context terms (for activating groups of rules)



    [STATS-API]
    - Get sym stats
      - num firings
    - Get rule stats
      - num firings
      - num near firings by antecedents


    [DEPENDANCE-ANALYSIS]
    - Get list of rules & symbols that underly triggered symbol
    - Indirect credit assignments


    [ABDUCTION-API]
    - Get association stats
      - Unconnected pairs co-occurances (triggered during different assert propagations) 
      - Symbol clustering sets (groups of symbols that co-occur partitioned into
        subgroups that are triggered during different asset propagations)
