# ICL -- Interaction control language

    Revision: 10/8/97 
     
     <<>>   Encap directory type of icl system (10/8/97) 
     
     
    >>>OVERVIEW<<< 
     
    - GOAL: allow easy installation, combination, and configuration of 
      a constellation of software packages using resolution of declarative 
      statements regarding each package. 
     
    - An environment is a namespace w. executables, man pages, etc. defined. 
    - A software package is an environment. 
    - User & Global envs. are enviroments. 
    - A software package, p, is installed in an environment, e, by adding it 
      to the list of objects combined to form e. 
    - Namespace combinations are incrementally and partialy computed as needed. 
     
     
     
     
    >>>OBJECTIVES<<< 
     
    -  Each entity is a package. 
     
    -  Allow software developers to specify the requirements and interface 
       of their package, so it may be integrated automatically into a variety 
       of installations. 
     
    -  Declare requirements: 
       Eg. gzip 
     
    -  Declare Interface: 
       ./bin/emacs1931 
     
     
     
    >>>SPECIFIC PROBLEMS<<< 
     
    - Mime Types File 
     
    - Running make file 
     
    - Display variable 
     
    Maintaining a list of ActiveBinaries 
     Global env. has Binaries an aggregate set. 
     
     
     
    >>> BASICS <<<   (New version 12/1/96) 
     
    Aggregate Object 
       Constructed from a list of sub-objects: obj1 ... objN 
       Built using: Combine( obj1, Combine(obj2, ...) ) 
       Combination actions for various objects: 
     
       - BaseValue: 
         Uses first object that is not the undefined object. 
       - AggVector: 
         Appends vectors from each obj 
       - AggSet: 
         Unions elements from each obj 
       - AggNameSpace: 
         Recursively applies 'combine' to objects w. the same name. 
       - AggComputedValue 
         Computes value from combined objects 
     
     
    Package --  
       A NameSpace with a set of predefined entries: 
       - Executables     (NameSpace) 
       - Procedures      (NameSpace) 
       - Files           (TreeSpace)???? 
       - EnvVarVals      ??????? 
       - Man pages       (Set) 
       - Start Menu      ??????? 
       - InitFns         (Set)  ???? 
       - IncludeFns      (Set)  ???? 
     
     
     
        
     
     
     
     
     
    >>>GLOSSARY<<< 
     
    Aggregate Object -- (outdated) 
       Incrementally computed objects.  Computed from a list of aggregate objects. 
       Combined using:  Union, Merge, or Override 
       -  Union        One value is used.  Conflicts are disallowed. 
       -  Merge        Values are combined in listed order. 
       -  Override     Use first value in listed order. 
     
    Aggregate Object Types -- 
    * AggVector -- Ordered set of values 
        Union: Use available value; error on conflict 
        Merge: Concatenates secondary vect to primary 
        Override: Uses secondary vect only when no primary 
     
    * AggNameSpace -- Mapping from name --> value  (an environment) 
        Aggregate values are recursively combined, otherwise: 
        Union: Use availble value; error on conflict 
        Merge: Use available value; if conflict use primary 
        Override: same as merge  
     
    * AggSet -- Set of values 
        Union: Use union 
        Merge: Use union 
        Override: Use primary or secondary 
     
         
     
    Component -- The basic unit for combination in ICL 
     
    Component Name --  
       Component Identifier.  If completely specified, it should be unique 
       across systems, and across time. 
        
       Format:  <<>> <<>> 
       Predefined keys:  Org, Host, Time, Id 
       Example: emacs.19.31,org=Gnu 
     
     
    Component Type -- 
        
     
    Component Types -- 
    BASE: 
     Variable          Variable w. an atomic value. 
     Alias             Component that is an alias for another component 
     Set               Set of values 
     Vector            Ordered set of values 
     Aggregate         Incrementally specified set 
     Order             Incrementally specified ordering 
     NameSpace         Mapping from names to values 
     
     FS file 
     Composite         <<>> of other components 
     Executable 
     Source file 
     ManPage 
     ICL Doc           ICL Documentation w. appropriate tags 
        
      
     
    Environment -- 
       A component that specifies parameter settings for a particular 
       instantiation.  Environments can be combined, shadowed, & refined. 
       Is it just a namespace??? 
     
    Environment combination ordering -- 
       The combination of environments maybe ordered or unordered. 
       Unordered combo. requires explicit resolution of conflicts. 
       Ordered combo. specifes a precidence between environments being combined. 
     
     
    Environment -- 
       An incrementally defined NameSpace. 
     
     Declarations -- 
       InEnvironment(env)            All declarations  
       Define(ComponentName, Value) 
       Inherit(environment) 
       resolve(name, env1, env2)     Orders env1:name before env2:name 
       Inherit(env1, env2, ...)      Inherits from specified envs in specified 
                                     order. 
        
    Actions -- 
     Environment Instantiation -- 
       Uses environment specifications to generate an environment. 
       Can explicitly control the dynamic vs. static tradoff. 
     Update -- 
       Updates an instantiated environment, to coorespond to the 
       new environment specification. 
     AddDeclaration -- 
       Adds declaraction to environment & updates its instantiation 
     DeleteDeclaration -- 
       Removes declaration from environment & updates its instantiation 
     
     
     
     
     
     
     
     
           
     
    Environment (predefined environments) 
       Active    The active environment.  The user's environment shadowed by 
                 the global environment (or another if specified by user) 
       Global    The default environment that contains installation setup. 
     
     
    Environment (predefined names) 
       ManPages 
       Binaries 
       InfoDirectories 
       Modules            Components ``known'' in this environment. 
       Active             Active modules tree? w. specific activation info. 
        
     
     
    ICL 
     
     
    ICL Commands -- 
     
    ICL Examples -- 
     
    foo_man = ./man.1 
    Incl(foo_man, ManPages) 
    Incl(./foo, Binaries) 
     
     
     
       
    Method 
     
    Methods, Predefined 
      
     
     
    Value -- 
        
    Values 
       String 
       Integer 
       ComponentName 
       Computed (from) 
          Binary, Template, Function 
          Specifies dependancies, and recompute schedule 
     
     
     
     
