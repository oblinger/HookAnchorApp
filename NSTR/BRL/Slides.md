# BRL.Slides --



    [[SLIDES]]
    (1) BOOTSTRAPPED STRATEGY ACQUISITION
    - one shot; mathematically limited; 
    - by contrast: human learning is not ...
      (dont take rules of chess; navel gaze; & beat kasparov)
    - GOAL: a bootstrapping learning process

    (2) LADDER (concepts at different level)
    - Key is smaller steps
    - Goal is to learn ALL needed for next rung
    - Hard because strategies are complex; take small steps is not enough
    - GOAL: general solver that climbs *any* ladder.  (test w. hidden ladder)

    (3) CONVERGENT KNOWLEDGE
    - SIMULTANEOUS constraints from multiple knowlege types
    - SIMULTANEOUS (generative + selective)
      -- Designing building: hallway->elevator->bathrooms; designing traces; building simulator; CAD manipulations

    (4) KEY CHALLENGES
        1) Just having ladder of convergent K is challenge & beneficial to community
        2) Tractable; impedance mismatch problem
        3) EVERYTHING: new rep

    (5) POSSIBLE LADDERS
    - MANY different reps and knowledge
    - CONVERGENT K.

    (6) ARCHITECTURAL BUILDING BLOCKS
    - FUNCTIONAL UNIT:  Common compoent language that allows very flexible specialization & can be acquired
    - INSTANTIATED SOLVER:  Tuned for specific problem

    (7)  ARCHITECTURE
    - generic processes available for instantiation
    - many processes acquire knowledge and strategies
    - whole process tuned according to current problem set



    KEY IDEA SLIDE:  combinging sources
    - How pieces come together
      show different forms of knowledge, and different learning processes
      show synergies as matches between knowledge from different sources
      - picture that shows all effects are needed


    DOMAINS - Test problem domains (test protocals)

    - milestones metrics   (w<<>>o bias  w bias)

    - key problems to crack

    bottom up <<>> top down combinging



    [[[  EXAMPLES  ]]]


    CONVERGENCE  --  Noisy/partial/ambiguous access to structural knoweldge to be learned along with selective feedback.
     - Structural supports first; someone doing bikeframe; feedback

    LADDER  -- Bike
     -  Builtins:  Algebra solver; Motion modeling; Steric Constraint Modeling
     -  Concepts: Balanced forces; Torque
     -  General: gearing ratios, forces at joint modeling, tube modeling



    FN UNIT
     -  3D Component Layout:  
        INPUT:  components; constaints
        OUTPUT: layout postioning parameters
        HARD CODED STRATEGIES:             limited backtracking; randomized restart search
        SPECIALIZATION CONTROL POINTS:     (These strategies are learned or acquired)
          layout ordering rules:           most constrained first; fixed order by type; largest first; 
                                           Definition of "most constrained"
                                           Definition of component types for fixed order by type
          constraint types (w. adjustment strategies) 


    LEARNING LAYOUT FUNCTION
    - Notice there is a process called "component layout"
    - Inputs:
      Notice set of cad components.
      Notice constraint relationships:  Min/Distance distance, adjoins, connected to, supports, inside
    - Outputs:
      xlation and rotation for each input component.
    - Learns how to compute each constraint.
      "non-overlapping" or "adjoins"
    - Learns it is a constraint satisfaction problem.  e.g. find outputs parameter values such that all constraints are satisfied.
    - Choose implementation and tune for problem set.

    - Notice strategy specializations:
      - User applies ordering rule, to select order to layout components
        (one type of constraint satifaction method: serial decomposition repqures such input.)
      - User uses strategy of layout followed by directed updates based on constraint violation
        (Iterative refinement strategy reqires such transformation operators)
        - Nudging operators are learned for each constraint type
