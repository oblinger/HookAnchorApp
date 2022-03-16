# 999icl.Properties -- Properties of environments that form the basis of the approach

    AN ENVIRONMENT  
    ============== 
    Software components are configured for each specific 
    installation--this resulting configured component is called a software 
    environment.  These environments may depend on other installed 
    software (other environments), and may be combined with other 
    environments to form composit enviroments contained a number of 
    configured software components.  Any particular computing system can 
    be COMPLETELY described as an environment composed through this 
    recursive process. 
     
     
    PROPERTIES OF AN ENVIRONMENT 
    ============================ 
    The benefits of this approach stem from a number of properties that are  
    fundamental to a basic building block--the environment. 
     
    > RECURSIVE 
      Complex software enviroments are built through composition of simpler 
      software environments. 
     
    > DECLARATIVE 
      Effect of a component environment on the containing environment is 
      explicitly represented as a property of the individual component  
      rather than as simply a combined property of the containing environment. 
     
    > FUNCTIONAL 
      Environments are functionally computed from their declarative  
      specification. 
     
    > DETERMINISTIC 
      The computation of an environment is effectively deterministic. 
      (If two environments are computed from the same specification, the  
      resulting environments will be functionally equivalent) 
     
    > INTROSPECTABLE 
      Humans and software agents must both be able to observe and control 
      the dependence and combination structure between software environments. 
     
    > IMPLICIT COMPLIENCE 
      Adherance to these properties listed above is mediated by the OS itself. 
      Environments cannot have unauthorized effect on other environments simply 
      because the OS does not give these component access to those environments. 
      (This is analogous to one software process not being able to corrupt 
      another's memory, simply because the virtual memory scheme simply does not 
      provide access to the other's memory) 
     
    > COMPOSITIONAL PRIMATIVES 
      Affording software components the ability to control how they  
      integrate into a composite computing environment, while maintaining  
      implicit complience, requres a rich language of compositional primatives. 
      (E.g.  each application needs to be able to specify specific directories  
       that should be part of the global executable search path, without having 
       the ability to arbitrarily modify other aspects of the global path.) 
     
    > GRAINSIZE 
      Changes made by an environment on its containing environment must be 
      expressed at a sufficiently detailed grain size.  This grain size must 
      detailed enough to permit changes made by DIFFERENT software  omponents 
      to be expressed at changes to DIFFERENT object with in the combined 
      environment.  (E.G. simply specifying the registry as a single file 
      would be too course a grain size since each application would be affecting 
      the same object in the environment.  Expressing the registry as 
      a collection of attributes and values allows the modification made 
      by seperate applications to be considered updates to independant  
      objects in the environment. 
     
     
