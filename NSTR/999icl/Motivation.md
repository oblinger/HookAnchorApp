# 999icl.Motivation -- Motivation of problem and solution

    PROBLEM 
     
    * Total person years invested and tools empoloyed in software testing 
      and development are both much better than that used 20 years ago yet 
      software relaibility is lower today. 
     
    * Perhaps the bugs per line of code is lower today, but twenty years ago 
      software didn't just change its behavior "spontaneously" unless there was a 
      hardware failure.  Today this is common place.  Why? 
     
    * The number of seperately developed/tested components has dramatically  
      increased.  This has increased the amount of configuration or "glue" 
      information required.  Current computing environment requires  
      configuring this "glue" info over and over again for each installation 
      environment and each component on those system. 
       
    * Most computer problems today are stem from this glue info, and the 
      human resource required to solve these problems is only going to get 
      worse.   
     
    - Wizards as designed today will ultimately fail. 
    - Cannot reduce number of independent modules without loosing impt 
      flexibility. 
     
    SOLUTION 
     
    * Design the computing environment so that the inevitable "glue" 
      is more managable.  How? 
     
    1)  Force software component independence through mechanisms analgous 
        to those used to force process independence in modern a modern OS. 
     
    2)  Require or auto-compute DECLARATIVE statements regarding all software 
        component interactions.  This facilitates: 
        - Imporves human understanding of a computing system and its behavior. 
        - Facilitates localization and repair of problems. 
        - Allows software better ability to introspect on its environment, and 
          thus manage its own "glue" information better. 
     
    3)  Restrict component interactions in ways that allow qualitatively new 
        operations in system administration.  These facilities provide  
        a number of generic (not application specific) ways for non-experts 
        to diagnose and correct software problems. 
     
    ===== 
    The glue between software components is the problem; a new paradigm 
    for combining software components is the solutio. 
     
     
     
     
     
