# Transformational-learning.Framework --

    Let  
     
    - WW be a an example representation laguage 
      w   represents a single training/testing data point 
      w_t represents the target term in w???? 
    - PP is the space of learning problems. 
      P   in PP is a single learning task 
      P_w is a set <<>> of examples 
      P_c is the target concept for the task 
    - TT be a set of transformation (TT:WW->WW) 
      t   represents a single transformation that maps an example w onto w' = t(w) 
     
    - An operator O, in OO, maps T -> <<>> 
     
    - MM is the set of metrics, each M in MM maps problems from PP onto a real num 
     
    - Rules  
      P, t --> t' 
     
     
     
    Some transforms 
     
    - Drop attributes 
    - Functionally combine attrs to create new attr 
    - Demultiplex attrs 
    - Segment continuous attributes 
     
     
    IDEAS 
    - Tlearning is like decision tree learning except it does not 
      partition the data but rather set covers it 
      (at least if we assume that each example is predicted by 
       its most predictive single attr.) 
    - Need rich (recursive | rule-like) language for examples 
