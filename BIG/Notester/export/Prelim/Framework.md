# Prelim.Framework -- Formal characterization of the pi framework.

    block diagram.
    - Minimal commitment:
      Uncertain K results in spaces of alternatives.
      Only concerned w. generating alternatives and how spaces are related
    - V               Refinement Var (Target concept)
      TYPE(v)         Syntactically expressed.  Determines refinement algorithm.
      RANGE(v)        Language of hypotheses (Range of refinement var V)
      ASSIGNMENT      <<>>
    - R               Rule.  Language of Plausible Knowledge.
      ANTECEDENTS(R)
      CONSEQUENT(R)
      MAP(R)
    - K               Plausible Domain knowledge (set of R)
      O               Current observations (set of ASSIGNMENT)
    - ALT(v,k)        Set of rules (bias). (generally one step rules)
    - S               Composite state of induction systems (Includes K&O)
      VARS(s)         List of <<>> variables  
      BEST(v,s)       From ALT(v,Sk)
      REFINE(a,v,s)
      PICK(s)         Returns appropriate refinement alg, & space for refinement.
    TERMS
    - S is a state of the system.
      It involves a set of spaces
    - SPACES: STATE -> 2^SPACE
    - PICK: STATE -> SPACE
    - NEXT: STATE, SPACE -> STATE
    - P is the set of PARAMETERS.   (Attributes)
    - V is the set of VALUES.
    - The world is described by assigning values to parameters.
      ASSIGN is the set of possible assignments.  ASSIGN is defined as P x V.
      A specific assignment is written p=v.
    - DESC is the set of descriptions. D = 2^ASSIGN
    - RANGE maps each parameter to set values the may be assigned to that parameter
      RANGE: P -> 2^V
    - PL is the langauge of plausible influents.
      PL is the set of all syntactically valid influents.
    - Each influent is characterized by relation over. a set of antecedent
      and consequent parameters:
      ANTECEDENCE: PL -> 2^P
      CONSEQUENT:  PL -> P
      RELATION:    PL -> 2^<<>>
      FN: PL -> ( 2^P -> P )
    - E is the set of training examples available to the reasoning system.
      E is a set of descriptions <<>>  Each Di in DESC.
    - K is the plausible domain theory.  K is some subset of PL.
    - CS is the langauge of combination spaces.
      Each combination space is written: Ctype(...) where 'type' is the type
      of the inductive space, and '...' is its parameters.
     - ALTERNATIVES(C) is a possibly infinite set of alternatives for the 
      combination space C.
    - SELECT(C, E) returns an ele from ALTERNATIVES(C) that is plausible given E.
      * SELECT should be computationally efficient.
      * SELECT should choose alternatives that allow the elimination of many
        other alternatives in the future.
      * SELECT should return more probable alternatives first.
    Names for COMBO spaces:
     Combination Space
     Refinement Space  (check)
      
