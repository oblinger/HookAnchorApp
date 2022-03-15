# 999icl.Introspectable -- Support for introspectablity

    PROBLEM:  Behavior & failures in complex software artifacts are very  
              difficult to understand. 
     
    OBJECTIVES: 
    - Automatically track problem to correct level (its source) 
    - Provide differential diagnosis (why it works here & not there) 
     
    TREE OF COMPATENCIES: 
    - Composed of rules of the form:  C <-- A1, A2, A3 
      where competence on A1, ... An is sufficient for competence on C 
    - Many nodes have partial or through testing mechanisms, this allows 
      the system to check for particular compentancies when diagnosing an error. 
     
    IDEA:  
    - Each layer may be able test itself for life. 
    - Each layer may be able to perform a complete check on itself. 
    - Each layer may have a dynamically computed set of dependent layers 
      that also need to be functioning. 
     
     
     
    EXAMPLE PROBLEM: 
    - Networking support fails.  This can happen for many reasons and the  
      source of the problem can be at many levels. 
     
    - Cannot read or write to a file particular file. 
      - Is device accessible 
      - File exist? 
      - Permissions set correctly? 
     
     
     
     
     
