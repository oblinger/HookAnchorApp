# 999icl.Goals -- Primary objectives of icl

    CORE IDEA 
    * DECLARATIVE   -- Computing environment is specified declaratively. 
    * DETERMINISTIC -- Computing environment is deterministically generated from 
                       it specificaton. 
    * COMPOSITIONAL -- Complex computing environments are composed from simpler 
                       (Partial) environments. 
     
     
     
     
     
    Easy configuration and maintaince of software installation. 
     
    - Easy understanding of current installation 
     
    - Switching between multiple configurations 
     
    - Testing new software 
     
    - Undo installation cleanly 
     
    - Distribute installed software  
     
    - Determine difference between two systems 
     
    - Go back to earlier time w. part of system. 
     
    - Package all changes made during a period of time 
     
     
     
     
     
    CONCRETE GOALS 
     
     
    ALL OF THESE SHOULD BE EASILY AND QUICKLY BUILT BY SITE MGR 
     
    - distribute MS words with site specific macros added. 
     
    - Preconfigured internet drivers (prompts user for missing info 
      or gathers automatically). 
     
     
    - Provide site specific values & help during setup 
     
    - Provide packages and groups of packages on a networked distribution  
       
     
    - Hands free customized burnin procedure. 
     
    - Auto testing of the veracity an installed package 
     
    - Automatically suggest and if ok'ed by user, configure socks support 
      for netscape.  Support depends on global parameter ``SOCKS-STATUS'' 
      - The first time this parm is queries, the system prompts user  
        for socks status 
      - The user's response is remembered, if parm is changed then  
        netscape install is reinstalled (if user accepts) 
      - If SOCKS... is changed back, then netscape should also revert back. 
     
    - Allow system do be flipped between different types of IP connections 
     
    SINGLE MACHINE ADMINISTRATION 
    - Gives structured breakdown of all system modifications since beginning 
    - Allow to be viewed/manipulated both cronologically and by source 
    - Temporaily modify system 
    - Remove or unmake some individual or group changes 
    - Ask systems what things about environment a particular pkg queried 
      and thus depends upon. 
    - Inspect individual parts of OS env (files, path entries, etc.) and 
      determine source. 
     
     
    USAGE SCENERIOS 
     
    PROBLEM: Carol has managed to install ProblematicComputing a third 
    party package used by all real estate agents at her company.  Bill 
    can't seem to get it installed correctly, and carol can't remember 
    exactly what she did to make her version work correctly. 
     
    SOLUTION:  Since her particular configuration of ProblematicComputing is 
    determinstically generated from a set of installation paramters, she 
    inserts a floppy into her computer and copies the specification of her 
    installation onto that floppy.  Bill then inserts this into his  
    default computing environment, and his computer grabs the raw installation 
    object for ProblematicComputing off the net (or off CDs he supplies). 
    Each query made during installation has a default from carol's system, but 
    Bill is still prompted, so that he may change things like "Agent's Name" 
    from Carol to Bill. 
     
     
     
