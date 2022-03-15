# Prelim2.Framework -- Formal characterization of the pi framework.

    TERMS
    - P is the set of PARAMETERS.
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
      
