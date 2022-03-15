# BSL.Facile\_Representation --

    A facile representation contains *EVERYTHING* needed to effectively 
    operate over a specific represenation.  This includes the ability to 
    translate to/from other facile representations, problem solve in representation,
    to learn from, and to reason about resoning in representation.


    [  RELATIONSHIPS BETWEEN FACILE REPRESENTATIONS  ]

    Let A and B be facile representations, then
     A embeds B              iff  all K and problems in A can be mapped and solved in B
     A is a varient of B     iff  A embeds B and B embeds A
     A is derived from B     iff  K, processes, or reps from B are used to construct A
     A has components from B iff  K, processes, or reps from B are contained (Without modification) in A


    [  COMPONENTS OF A FACILE REPRESENTATION  ]

    SYNTACTIC TYPES  --  A set of syntactic types.  Each syntatic type may have:
    - predicate specifying if a tuple is an instance of the type.
    - fns for generating instances of the type (either randomly or from other K)


    FUNCTIONS  --  Functions over instances of the specified syntactic types.  Types of operators include
    * metrics     -- fn that maps to Reals
    * predicate   -- fn that maps to true/false
    * operators   -- fn that maps from and to syntactic types
    * xlators     -- fn that maps elements between different the types of different facile representations
    * strategies  -- fn that maps rep elements and goal into to operators to execute
    * meta        -- fn that bridge this rep and the meta reasoning facile representation

    REPRESENTATION META KNOWLEDGE
    * Specification of names         -- for the rep itself for the operators for the metrics for everything in the rep.
    * Specification the rep's goals  --  (can be many)
