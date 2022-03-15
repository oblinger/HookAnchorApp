# Prelim.Main-pts -- Primary points for phd thesis.

    Submit to KDD?
    FRAMEWORK FOR PLAUSIBLE INFERENCE
    - K. Rep. tool for specifying what is known (w/o over commitment) &
      allow induction to refine what is not known.
      Eg: Static friction depends on contact surfaces (that's the context)
          A stock's price is the cumulative effect of market index and sector index
    - For practical reasons we need to split model of world into pieces.  
      Logic is not sufficient for this: Qualification Problem.
      Hence the idea of a combination space
    - Need to split K. into pieces
    ?? - Context tree
    Two types of <<>>
     * Skolemized relational rules
       horn clause like <<>> function like; space of alternatives; 
     * Combination knowledge
       combination rules; induction alg; tied to various predicates
    - Intermediate observation  (or intermediate learning problems)
      * Gathering stats or optimizing parms independantly.
    - Combining learning algs?
    How the framework came to be:
     - Started w. basic idea
     - Choose relevant domain; solved problem in domain
     - Generalized framework to capture interesting aspects of domain knowledge
    Bio Problem
    - Description of structure prediction; and its importance
    Bio Results to date
    - System
    - Hill climbing to beat identity
    - K used:  Cost fn is related to counts table for ``similar'' context
               GOP and GEP are determined by table.
    - EXPECTED RESULTS: 
      More profound use of PI is to explain similarity through physics
      Scatter plots.  1d-maps.
    - What is the hangup?
    Ex: of many types of over commitments
    - Deductive closure.  Buried car example.
    - ~Q->~P 
    IDEAS A LA CART
    IMPLICIT CONTEXT: Where did it go??
    - Saying an influent mean P & ctx --> Q  is not sufficient.
      Must further admit that combination w. other K. can fix problem. &
      Must specify how to combine w. other K.
      (Cannot utter an influent w/o understanding the combo space it will
       be used in)
    FUTURE WORK
    Semi-Automated Decomposition of Induction.
    - Concepts are too complex to learn, must get intermediate feedback.
      Ex: A res context is good if it is conserved across alignments.
      Ex: A scatter plot is good if it differentiates amino-acids
    DETAILS
    COMBO SPACES:
      TYPES           USE             Examples:
    - Overrides.                      Birds fly.
    - Additive.                       Forces in physics.  Stock market.
    - Parametric.             
    - Monotonic.      Params in tranparent eq.   
    - Conjunctive.    Limiting factors.   Safe place to park car.
    - Alternatives.                   Newtonian vs. Eignsteinain Physics
    Each combination space specifies:
    - A generator of plausible combinations,  Given a set of influents.
    - Informed generation.  Also takes a set of examples
    ? A propagation of directionality. (How inupts affect outputs)
    - 
