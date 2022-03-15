# Structure-acquisition --

    [SUMMARY] 
    Learning thru structured knoledge acquisition 
     
    [JUSTIFICATION] 
    - Winograds 'arch' assumption 
      (learn in easy one-step increments) 
     
     
    [MAIN IDEAS] 
    - REIFICATION:  Set of structured connections is repeatably mapped to the 
      same canonical gensymed handle symbol that denotes the specified  
      relationship structure. 
     
    - WORKING MEMEORY.  Set of activated symbols. 
     
    - DIRECTED EXPANSION.  Groups of symbols can activate related symbols 
      by adding them to working memory. 
     
    - INDEXING.  Because of reification, all concepts can be indexed by 
      the presents (and absents of some set of symbols in working memeory) 
    - INDEXING.  By storing all sub-expressions of a memory symbol, we can 
      greedily match a new experience against all memories by building all 
      terminal symbols and greedily combine symbols only when existing  
      combined structures are found in memory. 
     
    - ONE-STEP LERANING.  we can limit new "active" memories to only one 
      reification level beyond previous "active" memories at a time.  In 
      this way only reinforced patterns can be learned in an auto 
      retrievable fashion.  (Episodic memories of an experiemnce may be 
      stored in its entirety on the first experince of it.  Still 
      retrieval can only be triggered by "active" symbols.) 
     
     
     
    - ABDUCTION/CLUSTERING.  Simply similarity based learning is continuously  
      performed over repeated patterns in experiences.  Surprising patterns 
      are reified. 
     
     
    ############################################################################## 
    ###                             N O T A T I O N                            ### 
    ############################################################################## 
     
     
    DATA STRUCTURES 
    - EXPERIENCE(i) -- A sequence of experience set.  Each is a set of symbols. 
    - WORKING_MEMORY -- A set of active symbols 
     
    - MEMORY(wm,th) -- The set of memory symbols retrieved from working memory, 
      wm, and a match threshold, th. 
     
     
    FUNCTIONS 
    - Ground(vnf, var) 
    - Vars(vnf) 
    - Reify(symbol-set) 
    - Parts(symbol) 
     
     
     
     
    ############################################################################## 
    ###  R E P R E S E N T A T I O N s   A N D   D A T A   S T R U C T U R E S ### 
    ############################################################################## 
     
     
    [REPRESENTATION] 
     
    VARIABLE NORMAL FORM: 
    - Conjunction of predicates with simple variable args 
    - Ground instances are represented as unary prediate: e.g. dan(x) 
     
    SYMBOL FORM 
    - All refs to a single var can be expressed as a set of predicate 
      names along with the index of that variable in the prediacate. 
      e.g. "dan1".  These symbols are called ground symbols. 
    - Ground(vnf, var) is the set of ground symbols associated with the  
      specified 'var' from the 'vnf' expression. 
    - Vars(vnf) is the set of variables in the 'vnf' expression 
    - Reify(symbols) returns the symbol representing the reification of a set of 
      symbols 
    - Parts(symbol) returns the set of symbols represented by a reified symbol 
      or the empty set if the symbol is ground. 
     
    REIFICATION 
    A 'vnf' expression can be reified in the following manner: 
    - For each var in Vars(vnf) Let symbol(var) be Reify(Ground(vnf, var)) 
    - Let syms be the set of all symbols 'symbol(var)'  
    - Return Reify(syms) 
     
    (not quite since predicates may repeat. Build symbol up from related parts.) 
    - Opportunistically Reify groups of symbols that have been seen together 
      before. 
     
    SYMBOL MATH 
    - Two symbols can be intersected to yeild their common pattern 
      (which is another symbol) 
    - A symbol and a pattern it fits can be used to specify an  
      index symbol.  This index specifies the original symbol by filling 
      in the variablized part of the pattern. 
     
     
     
    MEMORY 
    - A set of pattern symbols that can be indexed by active memory 
     
     
     
     
    ############################################################################### 
    ###              P R O C E S S E S   A N D   A L G O R I T H M S           ### 
    ############################################################################### 
    - Layered Activation 
    - Learn activation expectation among wm elements 
      ? Generalize activation expectationsns 
     
    SCANNING LAYERED ACTIVATION -- Scans a sequence of experience assertions 
    and maintains a layered activation set ACTIVE(i) where i is the level 
    of reification of the symbols. 
    Given: An experience sequence, EXPERIENCE(i) 
    Output: Generate ACTIVE(j) mapping instantiated for each Ei. 
     
    For each e in EXPERIENCE 
      Let ACTIVE(0) be e 
      Let ACTIVE(i) = MEMORY(ACTIVE(0)+...+ACTIVE(i-1)) 
       
     
    NOTICING 
    - Heuristically create patterns by intersecting pairs (or small sets) 
      of potentially related experiences. 
    - Create a category of index symbols found for the pattern. 
    - Setup auto-reification of this new category based on the index symbol 
     
     
     
    GROUPING 
     
     
     
    EXPANSIVE REIFICATION 
    - Given nested DAG of symbol sets 
    - From the bottom to the top 
    - Make a given node symbol active 
    - Include all reifications that are activated 
    - Reify salient active symbols 
    - Pass salient symbols up to next level of the tree 
    - Repeat process for all tree nodes 
     
     
     
     
     
     
    EXAMPLE:  Learning "on the table" 
     
    bf1-on,bf2-the 
     
     
    Notice: 
    on -> bf1-on,bf2-the | bf1-on,bf2-a 
