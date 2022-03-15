# AR.Rep --

    qattara 
     
     
    STRAW MAN PROCEDURE REPRESENTATION 
     
    Significant IOUs: 
    - Language of observable features 
    - Language of output actions 
    - Language of functional compositions (possible hypotheses for gate & action) 
     
     
    STATE 
    - (1) An element from Flat list <<>>               (Automata) 
      (2) An element from Hierarchical State Set <<>>  (Automata w. shared transitons) 
      (3) Set of elements from hierarchy        
     
     
    OBSERVABLES 
    - Current state is an observable feature 
    - Precoded feature extractors  based on  
      * system events 
      * system screen 
      * user actions 
    - (1) Set of feature values assignments <<>> 
      (2) Feature values are structured (sets, lists, subkeys) 
    (SPECIALIZED LANGUAGE NEEDED HERE) 
     
     
    EXECTION TRACE 
    - Sequence of observables 
     
     
    PREDICATE 
    - Binary function of observables 
      (First pass could be decision tree, but specialized lang to be determined) 
     
     
    TRANSITIONS 
    - Mapping between states.  Specified by the following parts: 
      SRC:     starting state 
      DEST:    ending state 
      GATE:    activation predicate 
      ACTION:  output action performed by transition 
     
     
     
     
    Execution Behavior 
    - All trasitions whose src is the current state are monitored. 
    - When one of their gate conditions is satisfied by the environment 
      that transition is "executed".  This changes the state to the 
      destination state, and outputs any specified action. 
     
     
    Learning   
      (Inspired by the Expectation Maximization learning of 
       Hidden Markov Models) 
    - GIVEN 
      * Multiple traces containing procedure execution 
      * Static similarity metric between observable states 
     
     
    (1) Cluster trace states by static metric. 
     
     
     
     
     
     
     
     
     
     
     
     
