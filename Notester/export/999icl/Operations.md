# 999icl.Operations -- High-level operations that icl should support

    PROPERTIES OF AN ENVIRONMENT 
     
     
     
     
     
     
    OPERATIONS ON ENVIRONMENTS 
     
    > SPECIALIZE 
      Define an environement as a set of changes on an existing environment 
     
    > REALIZE 
      Explicitly "build" a previously specified environment, so that access 
      to the environment is more efficient. 
     
    > INSTANTIATE 
      Create a specific instance of an application. 
      - An application is a read only object, or a collection of applications 
      - Each application has instantate functions: 
          conf <-- Configure(env, old-conf) 
            The configure function inspects the environment, prompt the user, and 
            relies on the old configuration to define a configuration object. 
          env  <-- Build(app, conf) 
            A deterministic function that builds an instantiated app from  
            configuration information and the application itself. 
       
     
    > INCORPORATE 
      Install one environment into another. 
     
    > ACTIVATE 
      Make an environment be the current environment on a machine 
     
    > EXECUTE 
      Start an application within an environment (perhaps that environment,  
      or an ancestor of that environment needs to be installed first. 
     
     
    ====== 
     
    > Tree View 
     
    > Do/Undo/ReDo & Have it both ways 
     
    > Recreatable 
      State of an environment is specified in a wa 
     
    > Differential diagnosis 
      What is different between two environments (systems, configurations, etc) 
      with respect to a particualar subsystem. 
     
     
    > Tight & complete representation of system. 
      All params needed to re 
     
