# BSL.Architecture --



    [TYPES OF OBJECTS] 
    OBJECT 
    > PROCEDURE               Computes value 
      - TRANSFORM 
      - METRIC 
        - OBJECTIVE           Measures the efficacy of a procedure 
      - LEARNING 
        - PREDICTOR           Computes a rule 
          - CHOICE            Alternative procedures given computation 
        - OPTOMIZER           Optomizes a set of parm settings 
          - P-SPACE           Space of param settings 
          - T-SPACE           Space defined by a set of transforms 
          - META-LEARN         
     
          - PARAMETER 
          - H-CLIMBER   
      - VALUE                 Constant procedure that return 'value'   
      - FN 
        - RULE 
    > VARIABLE                Named value 
      - PARAMETER value 
    > VALUE 
      - DATASET 
    > TASK 
       
     
     
     
    OBJECT name() 
     NAME         Text id for object. 
     
    PROCEDURE code, stats, objective, cost, args (inputs) -> outputs 
     CODE         Thing that does it 
     STATS        Statistics on history of calls of the procedure 
     OBJECTIVE    Measures how well system did what it was supposed to do 
     UTILITY      Estimates utility of executing the procedure (from inputs) 
     ARGLIST      Vector of arg specifiers 
     INPUT        Set of objects and procedures 
     OUTPUT       Vector of values computed for a specified input 
     
    P-SPACE fn arglist 
                  Finds optomizes constant parameter settings for 'FN' 
                  Objective inherited & Cost derived. 
     FN           Procedure being optomized 
     ARGLIST      List of parameters w. specified ranges 
     
    T-SPACE InitialFn, Transform 
                  Hill climbs to optimal fn 
     
    META-LEARN InitialFn, LearnedFn 
       Builds  
     
    TRANSFORM InitialFn, GeneratedFns 
     
    RULE    
     
    FN 
    RULE 
    TRANSFORM(SrcObject, NewObjects) 
    METRIC 
    OBJECTIVE(OutputValues, InputValues) -> number 
    TASK(Objective, Procedure,  
     
     
     
     
    <<>> 
    [DEFINITIONS] 
    PROCEDURE -- a procedure for computing feature values from parameters & inputs 
    VARIABLE  -- a named object w. a specified value 
    TASK      -- a value to be computed; a FN; a DATASET 
    DATASET   -- a set of FEATURE SETS (which are a set of features & values) 
    TRANSFORM -- a mapping of objects to other objects of the same type 
    METRIC    -- a  
     
     
