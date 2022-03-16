# BSL.Sat\_To\_Layout --

     > Here we consider the bootstrapping of a constraint sat engine into a layout engine.



    [GENERAL SATISFACTION ENGINE]

    HARD-CODED FUNCTIONS   -- backtracking; monte-carlo; gradient ascent; randomized restarts; constraint propagation

    CONTROL PARAMETERS     -- which alg to use, number of random restarts, etc.

    RELEVANT REPS
    - Tuples
    - Variables
      - Variable names
      - Variable type
      - Variable values for each type
      - Variable ordering    (suggested order for var instantiation)
    - Constraint predicates  (t<<>>f values computed from a set of variable values)
    - Optomization metric    (numeric value computed from a set of variable values)

    CONTROL POINTS  (INPUTS)
    * Starting variable values               (default=random)
    * Tuple of Variables
    * Variable instantiation ordering        (default=random)
    * Weighted set of preference functions   (default=none)
    * Set of constraint predicates           (default=none)

    OUTPUTS
    * Tuple of values


    [BACKGROUND STUFF NEEDED]
    - occupied volume model: CAD description of regions that are <<>> are not occupied by an object


    [2D LAYOUT ENGINE]

    RELEVANT REPS
    - Tuple of CAD components to layout
    - CAD model of the layout region (where objects may/may not be placed) 
    - Object placement tuple (translation and rotation for object)
