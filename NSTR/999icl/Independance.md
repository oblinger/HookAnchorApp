# 999icl.Independance -- Notions of environmental independence

    The type of independance achieved by two environments affects the ways in 
    which they may be automatically combined and recombined. 
     
    NOTE: These relationships between environments can be automatically  
    inferred by our approach by monitoring their behavior. 
     
    Software environments may said to be: 
    * Independant 
    * Sequentially dependant (implicitly or explicitly) 
    * Dependant 
     
     
    DEPENDANCE: 
    One environment depends on another if its creation depends on access to the 
    other environment.  (This is simply the notion of functional dependancy) 
     
    SEQUENTIAL INDEPENDANCE: 
    Two environments are sequentially independant iff 
      1) neither was created in an environment is dependant on the other 
      2) the additive effects of each are communative 
         (i.e. EffectsOfA + EffectsOfB = EffectsOfB + EffectsOfA) 
      3) inherited effects from the dependant environments of each are also 
         communative 
     
    SEQUENTIAL DEPENDANCE: 
    Two environments are said to be sequentially dependant if only condition 
    one above holds.  Sequential dependance may be explicit, meaning that 
    the ordering was explicitly declared by a software or human agent.  It may 
    also be implicit if the sequential nature of the interaction was determined 
    automatically. 
     
     
     
    > IMPLICATIONS 
    * Sequential independance between environments implies that they can be 
      combined and recombined in arbitrarily without fear of an inconsistant  
      or ill-defined states. 
    * Sequentially dependant environments may be added and removed from a 
      containing environment, but the effects on the containing environment  
      are dependant on temporal ordering and may potentially need to be 
      recomputed from scratch each time. 
    * Implicit sequential dependance may be weaker than explicit sequential 
      dependance in that it may be possible to simply ignore the sequential 
      nature of the updates, and treat the environments as indepenant. 
    * Dependant environments must be combined in the manner specified by the 
      dependancy. 
     
     
