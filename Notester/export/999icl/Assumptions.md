# 999icl.Assumptions -- Assumptions underlieing icl

    Here are the aspects about a computing environment that both cause ICL 
    to be necessary and allow it to operate effectively 
     
    > System is composed of many pieces whose development is at best only 
      loosly coordinated. 
     
    > Final system results from multiple layers of unrelated organizational  
      groups.  For example a laptop system might be comprised of 
      1) Shrink wrapped software developed by various sources 
      2) Software installed into an initial environment by the manufacturer 
      3) Generic machine image developed for a corporation 
      4) Specific modifiacations are made to this image for each group/division 
         within the corporations. 
      5) User further specializes each machine. 
     
    > Comprehensive, Low-level control is needed to monitor, add, and remove 
      changes to system. 
     
    > Change log grain size 
      Most changes can be expressed at useful grainsize 
      (e.g. not saying that registry file user.dat was updated) 
     
    > Changes can always be ascribed to an actor (e.g. an installer for 
      a piece of software), an executable, or the user). 
     
     
    LIMITATIONS: 
    - Two apps cannot be treated as independent if they have overlapping 
      influence.  Important to adopt fine grain size to avoid many overlapping 
      apps.  (i.e. should not treat registry as a single file) 
     
