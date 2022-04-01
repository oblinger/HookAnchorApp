# MailAddr.Meeting-9129 --

    MEETING 
    - Timing 
    - Demo: objectives; approach; eg. jacob, oblin uiuc; alg; code 
    - Next Steps 
     
     
     
    NEXT STEPS 
    ========== 
     
    COMMUNITY FINDING ALGORITHM 
      Finds: UC -- overlapping sets of related recipients 
             UP -- users sending profile over these communities 
      Confidence in pieces: partitioning, set cover. 
      I believe that their successful combination is plausible. 
      Indeed there are a number of interesting research questions: 
        - community grain size; metrics; efficient algs; convergence proofs 
        - maintaining labels over time. 
        - overlapping set cover 
     
      Criterion for success:  
      A taxonomy is good if it helps you guess where mail will be sent. 
     
      Why I should not work on this problem: 
      - Even if it works the output sets are not labeled in anyway. 
        (Roy Byrd is doing some summarization work that might apply. 
         & some uses of the communities diagram do not need labels) 
     
      - Even if I can label groups of people, I don't have any concrete 
        use of the taxonomy.  (Though it seems it may have KM application.) 
     
     
    Possibles Uses 
      - auto mailing list server 
        lotus workgroup invitations 
        ! data too noisy? 
      - call center email routing  
      - combine with content analysis 
      - human readable directory of expertise 
     
     
     
     
    CC ASSISTANT 
    - Iterative Restriction & Accumulative Selection 
    - Conjunctive specification 
     
    CC approach 
    - demo in JAVA.  Portable.      Limited UI. Slow. 
    - demo in C++.   Not portable.  ???     UI. Fast. 
    - demo as app.   Not portable.  unlim   UI. Fast. 
     
    PROBLEMS 
    - Limited UI provided by Lotus Script 
     
     
    EXTEND THE NAME EXPANDER 
    >     I have spent 20% of the effort & have 80% of the results 
    > Goal: Demo of the concept 
    > Goal: Alpha-works install kit 
    > Goal: Include in StdR4Mail 
    > Goal: Sell to Lotus; possibly code in C notes API 
     
    - Multiple address formats 
      e.g. cn=kevin singley/ou=watson/o=ibm research@ibm research 
      richard_caruana@ux3.sp.cs.cmu.edu @ d01au023 
    - Garbage collection of cache file 
    - Cache incomming mail automatically 
    - Should combine frequency, recency, and to/from weighting 
    - Should learn your associations 
    - Should have a NEXT-BEST button 
    ! Java interface is slightly buggy. 
    ! Installation is difficult (impossible?) to automate. 
    ! Does not play well with address book expander. 
      (Auto Address Expansion Occurs before any user event) 
     
     
     
     
