# Prelim.Bio-k-ex -- Examples pl k. in bio.

    Continuous/Discrete parms.  Overriding K.  Additive effects.
    - Hill climbing continuous parms.  Substitution table GEP GAP costs.
    - Additive effects.  
    - Functional composition.
      +  Weighted sum of substitution tables.
      +  Position specific weighting of cost function.
         (also uses intermediate observations)
    - Position specific (plausible) explanation of residue's purpose.
      OVERRIDING: not-hydro; makes-polar-contact
    - 
    SubstFreq(i,j,ctx) -> CostFn(i,j)
    CostFn(i,j) -> 
    CostFn() -> BestGAP
    - Determination
    - Continuous relation w. MatchScore
    CostFn(), GAP, GOP -> MatchScore
    - Simple Functional Relationship
    MatchScore, Threshold -> High-Z-Score(k,u)
    High-Z-Score(k,u) -> SameFold(k,u)
    - Explicit ordering of combo spaces ??
    Block diag.
    - Trying to predict list:
      Space being serached, Attr, Metric
    Formalism:
    - Type of combo
    - Influent
      Consequent, Antecedents,
      Order specifier: list containing lists of elements or elements themselves.
      An element is the consequent or an antecedent.
    A Metric:
    - Specifies a function and an parm.  Fn maps parm values to score.
      Metric may also specify qualifiers (only good for this class, etc.)
     
    Algorithm:
    - Combo-tree: alternative spaces w. top being main goal
    - Use Depth-first traverse of combo-space tree, making a SELECTion at each node
      traversal follows the ordering specfied by the influents at each node.
    - Once tree is complete refinement occurs at node w. highest priority.
      (This may not be a leaf since influent ordering may include 
       super node as well.)
    - If elements are paired then combined refinment is done.  Types of combine
      - CROSS all elements from cross production of spaces is considered.
      - SIMULTANEOUS iteratively hill climb each space.
      - ITERATIVE specify quanta for each space.  Use quanta for each. then repeat.
    - If some subparameter can infer some known metric then consider
      use of that metric for determining that parameter. ????
    Problem: What about getting to same node from diff paths??
             This will result in different metrics (average 
