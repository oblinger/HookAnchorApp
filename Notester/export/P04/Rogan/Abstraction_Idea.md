# P04.Rogan.Abstraction\_Idea --



    The objective of this project is to take multiple traces and
    find repeated patterns within these traces, and build an abstraction
    hierarchy from these reptitions.  


    A simple graphic to illustrate:


           X                                   X
        <<>>    \                            <<>>         \
       A      C       A          F       A           C
      <<>> \    <<>> \     <<>> \       <<>>   \    <<>> \         <<>> \
     a   b  c   g   a   b  c  f  e  d  a   b   h   c   g


    The lowest level is a trace, and we have recognized some common patterns.
    For example 'ab' is a common pattern as is 'cg' and 'fed'
    Note that an 'A' pattern is often followed by a 'C' pattern so this is
    a higher-level pattern 'X'

    The challenge of this work would be biasing the system to pay attention to 
    relevant features of the events in the trace, since our events will never
    be exactly the same.  Also patterns will probably have some amount of noise,
    the second 'X' pattern above has the event 'h' in the middle, but it is still recognized.

    The reserach would look at exaisting algs for detecting repeat patterns, and 
    try to apply them in the face of these artifcts of this domain, or try out
    a new one.


    The output hierarhy could then be used to annoatate a new trace, that is
    nodes within this tree would be mapped on to the trace.  
    (1) In itself this would serve as a hierarchical grouping of the steps in a trace.  
    (2) If the nodes in the tree a labelled, then this provides a multi-resolution description
    of the trace as steps and sub-steps.
    (3) Applying this mapping to traces in a library, and applying it to a current trace,
        would allow one to tell the user what actions were taken next is other trace
        at the "same" point in the procedure.

    The simplest form of each of these things is not much additional work over lerning the
    herarchy, so we might explore a simple demonstration of more than one of them.

