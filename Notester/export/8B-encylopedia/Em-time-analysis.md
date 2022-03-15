# 8B-encylopedia.Em-time-analysis --

    ORDER RUNTIME 
    ------------- 
    Loop Steps: 
    1: Order( |Pages| ) 
    2: Order( |Pages| * |Cats| ) 
    3: Order( |Pages| * fanIO + |Cats| * DocMatch ) 
     
     
     
    Reasonable Problem Scale: 
    |Pages| = 1,000,000 
    |Cats|  =    10,000 
    fanIO   =       100 
     
     
    TimeBottleNeck: 10 Gigs of loop2 ~20min @ 10Meg/sec 
     
    SpaceDominator: 
     
    - Links map.  Bytes 10*|Pages|*fanIO = 1Gig 
     
    or  
     
    - 1Meg pages * 10K work count = 10Gig 
     
     
