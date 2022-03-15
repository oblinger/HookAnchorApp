# Prelim.Combo-types -- Types of ambiguity in pl. k.

    DISCRETE, 
    >>> TYPES OF UNCERTAINTY
    Numeric -- hill climbing
    - Bennett.      THRESHOLDS
    - C4.5,cluster  DESCRETIZATION (generalization of threshold)
    X               ADDITIVE effects
    - Perceptrons.  WEIGHTED sums (generalization additive effect)
    - Bacon.        PARAMETRIC combination
    - Forb<<>>DeJong   MONOTONICITY constraints  (Q+)
    - Connect Net.  CONTINUOUS Parms w. continuous effects
                    (gen of parametric??? & monotonicity???)
    Numeric -- Noisy
    - PAC?          EPSILON<<>>DELTA noisy data
    - Stats.        PDF modelled error in data (directly computed)
    -               Parametric combination w. direct computational method
    Numeric --  Enumeration <<>> Selection
    Symbolic --
    -               DISCRETE Parameters (skolem var)
    X Reed Simmons? Conjunctive
    - Oblinger      Overrides, set thy combos (generalization of conjunctive)
    - Thy refine    Uncertainty = which syntactically close thy is correct
    - Russell.      Determinations
    X Non-mon.      ????
    MECHANISMS:
    - Direct - non-exhaustive computation of optimal hyp.
      Pdf, 
    - HillClimbing
    - DynamicProgramming
    - Refinement (Ordered hill climbing)
    - VersionSpace
    - Exhaustive - Generate & Test
