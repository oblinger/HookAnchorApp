# 999icl.Primatives -- The icl general specification

    1) Constant containers are compiled from ASCII flat files. 
    2) Compute containers are ``realized'' from a STATEMENT-SET 
    3) Thus each object has a specification and realization form 
     
     
    >>>NAMING<<< 
    ICL uses java-like subcontainer naming. 
    The ">" character initiates an absolute object name. 
    The "^" character initiates an object name that is relative to the current pkg. 
    The ``PACKAGE '' statment declares a new current pkg. 
    The ``ENDPACKAGE'' statement pops back to the previous current pkg. 
     
     
     
    >>>CONTAINER<<< 
    A container is a set; it is an object that contained other objects. 
    The elements of a container are uniquedly named and ordered. 
    - A NAMESPACE container has explicitly named elements, which are 
      ordered lexographically. 
    - A LIST container has elements with explicit indicies, and whose names are 
      also this index. 
    - A STATEMENT-SET contains a set of ICL statments that are explicitly named 
      or are named automatically using an incrementing counter. 
     
     
    >>>A PACKAGE<<< 
    A package is a collection of containers that are related in a fixed way: 
    - "xxxx" container the is realization of the package 
    - "xxxx.spec" is the stmt-set defintion of the package 
      Not present if pkg is a constant pkg. 
    - "xxxx.base" is constant portion the package is computed from (a namespace). 
      Not present if pkg is computed from scratch 
    - "xxxx.conf" contains all values necessary to derive the "xxxx" from 
      "xxxx.base"  
    - xxxx.Spec.IncludedContainers is a list of containers that are  
      merged with the current container. 
    - Operations on packages: merge, assign, realize 
     
    A package is realized by: 
    1) Create TMP as quick merge & set as the merge of "xxxx.base" and "xxxx.spec" 
    2) Realize "TMP.spec.subpackages" 
    3) Realize each pkg in this list 
    4) Quick merge realized pkgs into TMP 
    5) Realize "TMP.conf" 
    6) Compute "xxxx" as the merge of "xxxx.base" "TMP.conf" and the subpkgs 
    7) Discard the "TMP" merge. 
     
     
    >>>DECLARATIVE SPECIFIERS<<< 
    - Each specification statment can be named and referred to by that name. 
      Each statment also has a cononical name based on it package's name. 
    - SPECIFIER FORMAT 
      Package xxx.yyy.zzz; 
      [`!' | `.'   ] [ `=']   ... 
      Eg: ^executables =add-foo include ^foo.exe 
     
    > CONTAINER SPECIFIERS (applies to namespaces, sets, lists, and stmt sets) 
    - INCLUDES  [] 
      declares that 'obj' is an element of the current or specified container 
      element of 'list' 
    - EXCLUDES  [] 
      Declares that 'obj' is not a member of 'list'.  By default this overrides 
      any contradictory INCLUDES statments. 
    - INCLUDES-ELEMENTS 
    - EXCLUDES-ELEMENTS 
      Merges elements from subcomponent into super component. 
    - OVERRIDES   
      Declares that a particular 'stmt' takes precedence over another statment 
      or over all statements in a particular package  
     
    > LIST SPECIFIERS 
    - Positional specifiers: FIRST, LAST, BEFORE, & AFTER 
    - PERMUTE  
      Changes the order of a list.  Note this is a non-composable operator. 
      ??? should be allowed? 
     
     
    - COMPUTED FROM ??? 
     
     
    = Must allow user to vary ``fixed'' system parameters 
     
    = Must allow package to express its installation as a tree of dependent 
    objects so that partial reinstall is possible (for quick changes) 
     
     
     
