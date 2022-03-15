# EventMatchingLanguage (for tutor) --

    Purpose:   
    1) The recognition language is used to specify particular event patterns. 
    2) The search language is used to specify a space of rules to search 
       for patterns (and its ordering). 
        
     
    THE RECOGNITION LANGUAGE 
     
        term(x1,...,xn) '<-' expr(x1,...,xn,y1,...,yn) 
     
     
     
    THE BIAS LANGUAGE -- is a weighted, first-order, context-free grammar: 
     
        term(x1,...,xn) '<-' expr(x1,...,xn,y1,...,yn) '[' weighting-expression ']' 
     
     
    THE MATCHING PROCEDURE 
     
       Match(rule, start, end) returns all binding <<>>,start,end} 
          for 'rule' over range event 'start' thru event 'end-1' 
     
       Match can be incrementally computed and propagated for each non terminal 
       using a rete-match type token passing algorithm.  (This is probably 
       simpler than a non-incremental backtracking matching alg anyway.) 
     
       A max history horizon needs to be set so that memory usage is limited. 
     
       Temporal constraints on each sub portion within a single rule can simply 
       be check for satisfaction bottom-up, as each rule attempts to 'fire' 
       (asserts another match binding). 
     
       Just to be concrete: if we have a rule: "D(x) <- A(x) then B(x) then C", 
       we look for matches of the subcomponents and then see if their bindings  
       of 'x' are consistent and at the same time check if their starts and ends 
       are in the right order, and are close enough together, only then 
       do we assert Match of D(x), with appropriate bindings and start/end  
       indicies.  Using the rete-match style this match may then trigger other 
       upstream binding checks in rules involving D(x). 
     
       If you don't like rete-match fancyness we could assume that the matcher 
       is not incremental but is instead given all events at once, in a single 
       batch.  Then we could try to do this in a backchaining style.  Still 
       we are going to have to handle backtracking over rules with embedded 
       temporal constraints, something that makes my head hurt, and I think  
       we will probably need to cache partial results (which we get for  
       free with with rete). 
     
       Of course we could just do a non-backtracking, non-rete approach. 
       This won't be incremental, it won't be 
       time efficient (since it will recompute partial results) down 
       each depth first branch, and it will miss pattern matches that occur 
       when multiple bindings are possible (eg. there are multiple create events 
       that could be paired with a MenuSelect event.) 
     
     
     
    THE SEARCH PROCEDURE 
     
       A space of A* searches can be used build (learn) new rules. 
     
       For each term(x1,...,xn), keep 
       - a set of partial expansions and their current cost. 
       - a set of complete expansions and their complete cost. 
       - the current min cost in the partial expansions space. 
     
       1) The A* space whose min incremental cost is greatest is 
          maintained as the 'active space'. 
     
       2) When a space is active its best partial expansion and augment  
          it using all possible substitutions, and the new rules are added  
          with their incremental cost term added on. 
     
       3) If any term can be directly evaluated, then it is evaluated against 
          each example and a cost is assessed for any positive examples missed. 
     
       (Note: any space with no partial expansions has infinite min cost. 
        this forces, a depth-first traversal to nodes where complete costs can 
        be assessed.  After this the search reverts to a best first search.) 
     
     
       I believe that this alg is complete, and guaranteed optimal assuming that 
       the incremental cost function is never negative for any rule. 
       (perhaps this is not a good thing; it is really assuming that the 
        rules supplied by the user are sufficiently constrained that a nearly 
        full expansion of the CFG is tractable.  Are your rules going to be 
        that constrained?) 
     
       The other obvious approach is a bottom up greed aggregator of  
       sub-expressions with the greatest information gain (seperation of 
       positive and negative examples.) 
     
     
       The first method also seems to avoid the need for explicit negative  
       examples by finding the smallest expansion that matches the positive 
       examples.  (I think this trick will be busted if we allow negation 
       in our rules; i.e. if we can say ``... and *no* CreateButton event occurred'') 
     
        
     
    Connectives: 
     
    CONSTRAINT  --  the right hand side of a rule 
      A ';' B       Event A near subsequent event B 
      A '<' B       Event A followed by event B 
      A '&' B       Events A and B occured in some order 
     '(' A ')'      Just for pairing off terms 
             An event term 
              A functional composition of *non* event predicates 
     
    EVENT 
      term '(' key=  ',' ... ')' 
     
     
    EXPR 
      ?var 
      integer 
      string 
      fn_name '('  ',' ... ')' 
       
     
     
     
    Examples: 
     
       MenuSelect() > Create(Text=?y)  =>  FileOpenPattern(?x)  [10] 
     
       Hmmm, we really need to be able to embed functions, ie. 
       <<>>  but then we would need inverses for each function, 
       too complicated ... 
     
     
     
     
     
            
     
     
