# Pi.Proof -- Proof of pi's convergence

    GUARANTEES:
    - Proof of optimality of algorithm.
    - Proof of convergence of algorithm.
    - Proof of convergence rate.
    DEFINITIONS:
    * Alt: state, space -> 2^state
      Set of states that can be reached from 'state' by varying only the 'space'
      portion of 'state'
    * Next: state X space -> state
      Result of updating the 'space' part of 'state'.
      By def: Next(state-x, space-i) is in Alt(state-x, space-i)
    * Score: state -> p-real 
      Utility of a complete state.
    * <<>> p-real
      Error tolerance in Score units.
    * MP: p-real
      Minimal progress is Score units.
    * Optimal: state X space -> True/False     (axis-optimal, locally-optimal)
      Is the 'space' part of 'state' optimal, given the rest of 'state'
      DEF: Optimal(state-x, space-i) iff for all state-y in Alt(state-y, space-i)
                Score(state-x) + E >= Score(state-y)
    * BestScore: real
      BestScore = Max(Score(space-x))
    PRECONDITIONS:
    0)  FINITE SCORES:
        There exists k such that k >= Score(state-x) for all state-x.
    1)  NEXT-STEP SUB-SPACE OPTIMALITY:           Need promise of P progress
        Optimal(Next*(space-i, state-x), space-i)
     
    2)  INITIAL-STATE SUB-SPACE OPTIMALITY:                (replaces #1)
        Optimal(initial-state, space-i)
    3)  GUARANTEE OF PROGRESS:                             (too strong?)
        if SCORE(state-x) + E < SCORE(state-y) then
        exists space-i such that
           SCORE(NEXT(state-x, space-i)) >= P + SCORE(state-x)
    4)  NO COMPLETE BARRIERS
        There does not exist state-x such that
           Score(state-x) < BestScore   and
           for all state-i, for all state-y in Alt(state-x, state-i)
              Score(state-y) < Score(state-x) + P
    5)  NON-INTERFERENCE:  climbing one axis will not mess w. another
        Optimal(Next*(Next(state1,space-j),space-i), space-i) ->
          Optimal(Next*(state1,space-i), space-i)
    6)  ITERATIVE INDEPENDANCE:  Hill climbing works.
        state2 = NEXT(space-x, state1)
        Optimal(NEXT(space-x, NEXT(space-y, state1)), space-x) if
          Optimal(state-x, state1)
    - #0, #3 implies optimality.
    - #0, #4, #1 implies optimality.
    - #0, #4, #2, #5 implies optimality.
    0425 PROOF of monotonicity & safety
    By Induction.
    Step-0 is safe by #2. The sequence <<>> has monotonic scores by definition.
    Assume Step-i-1 is safe for all space-j, and assume the sequence 
    <<>> has monotonic scores.
    For each space-j we have Safe(state-i, space-j) by #5.
    And Score(Step-i) > Score(Step-i-1) by ????????, so <<>>
    is monotonic in score.
    Thus all state sequences are monotonic and safe.  QED.
    (may need axis-optimality)
    Algorithm
    1) state <- safe starting point. (pick many, or optomize each axis)
    2) Find space-i where Score(Next(state, space-i)) >= Score(state) + P
    3) If space-i is found then state <- Next(state, space-i); goto 2
    4) Return state
    PROOF OF TERMINATION
    Let Si be the Score for state-i (the Ith iteration of the alg.)
    S0 is some positive number by definition.
    Each Si >= Si-1 + P because of the structure of the algorithm.
    Thus Si >= S0 + i*P.
    No Si > BestScore so the alg must terminate before (BestScore - S0)<<>> steps.
    Since BestScore is finite (by #0) and S0 is greater than zero, number of steps
    is finite.
    PROOF OF OPTIMALITY
    By contradiction.  Assume state-i is the final state returned by alg and
    assume state-i is not optimal Score(state-i) <= BestScore-E.
    Then by #4 there exists space-j where Score(Next(state-i,space-j))>=
      Score(state-i).  Contradiction.  Since state-i is final no such space can 
    exist.
    THOUGHTS:
      We take the point of view that degenerate interactions are too difficult
      too difficult to handle w. a general mechanism, thus only deal w.
      degenerate interactions if we have a uncertainty type designed for that
      type of interaction.
      This constraint is weaker than independance.
      - interactions possible.
      - Local optimum are permitted???  given right adjacency definition.
