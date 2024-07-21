

# Meta Meta Parts

## CNODE -- CHOICE NODE -- A tree structured set of named choices (JSON structure)
--- CNODE NAME -- A globally unique name for this cnode (can be long and recursive)
--- CNODE SPACE -- The CSPACE containing this node
--- CNODE PARENT -- The CNODE that somehow this node was derived from
--- CNODE CHILDREN -- The CNODES that have been created in some fashion from this node

## CSPACE -- CHOICE SPACE -- A syntactically defined space of possible CNODE values. 
--- ROOT -- CNODE(s) that started off this space
--- BOUND -- an indicator function that tests if a CNODE is within the CSPACE
--- ORDERING -- a comparator function that tests the relative value of two CNODES
--- SCORE -- a scoring function that maps a CNODE to a float where larger values are more fit
--- GENERATOR -- a function that generates a new CNODE based on prior CNODES




## SLOT -- A named variable that can take can take on a symbolic or numberic value
--- SLOT NAME
--- SLOT VALUE -- the value a parameter takes on within some CNODE
--- SLOT TYPE
--- SLOT VALUES
--- SLOT CSPACE


## AN EXAMPLE -- A MOMENT -- A SNAPSHOT -- AN INSTANCE -- Parameter settings at an instant in time.
## XSET -- DATASET -- A stream of example set, or a fixed set of examples
--- XSET VIEW -- an xset where each item is mapped to a reduced item with a subset of the 
    parameter settings relevant for some purpose.


## LANGUAGE
### DATA -- Universal representation of information
---- A JSON Object with ASCII keys and arbitrary values
---- STRINGS:  A string starting w. an ASCII alpha is a symbol
               A string beginning w. a quote character (') is a interpreted as a constant string
---- '_' is the HEAD of the object and '__' is the optional fixed args for the object
---- The printed form is:  HEAD( ARG1, ARG2, ..., KEY1=VALUE1, ... )
### EVALUATION
----  String, numbers, True, False, and null all evaluate to themselves
----  A JSON Object with a HEAD '_' evaluates the fn or special form bound to HEAD as applied to the object itself
----  Other JSON objects or Lists evaluate to a copy of the object/list with each element evaluated.
----  VARIABLES -- 


### FUNCTIONAL FORM
--- An ARGBODY is the actual args for some function:   Arg1, Arg2, ..., Key1=Value1, Key2=Value2, ...
--- A Lambda form is fn( arg1, arg2, ..., _keys, key1, key2, ...)
      _body  the functional forms to eval
--- TYPESPEC
    int(some_var, null, 100)   float(my_var, 10.0, 100.0, 10.0)
    choice(some_var, val1, val2, ...)
    string(var_name, _default)
    int_pdf
    float_pdf
    sym_pdf
    
      
#### FUNCTION TYPES
----  Indicator Function -- maps things onto boolean values
----  Comparator
----  Scoring Fn
----  Generator
----  Distance
----  Monotone

----  OBJECTIVE FUNCTION -- A real valued 'objective' function of a CNODE and an EXAMPLE.


## ALGORITHM -- An encoded procdure for achiving some objective

### OPTOMIZATION ALGORITHMS
    Alg that attempt to select a best CNODE from some CSPACE to maximize some objective

#### OPTOMIZER
##### CLIMBING OPTOMIZER
##### EXPLORATIONAL OPTOMIZER
##### META OPTOMIZER
##### HETRO OPTOMIZER

##### SURROGATOR -- A strategy that maps a CSPACE into some surrogate CSPACE in a way that solving
      in the surrogate space can be productively mapped back into the original space.

#### ANLOLOGOZER



### CNODE OPERATOR -- A potentially paramterized operator that produces zero or more children CNODES
    from a starting CNODE.






