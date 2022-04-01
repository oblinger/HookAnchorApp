# Recursive-learning.Framework --

    [OBJECTIVES] 
    - Explore the tips of many representational spaces before exploring too deep 
      in any one space. 
    - Consider many overlapping generalizations at many levels. 
     
     
    The recursive learner is specified as an onion.  At its core are the following 
    three basic types: 
     
     
    [CORE OF THE FRAMEWORK] 
     
    FUNCTION -- (a executable procedure) 
        An execution procedure for computing a set of output values 
        from a set of input values. 
    DATA -- (a mapping from parameters to values) 
        A set of attributes and values.  Values may be structured. 
    TASK -- (a function, a parameter set, utility parameters) 
        An instantiated function to be executed.   
        - Input values are specified in the associated ParameterSet.   
        - The utility parameters are used to determine inter-task execution order. 
     
    EXECUTION -- 
        - A task is executed by applying its function to its parameter set. 
        - During execution a task may declare completion and specify a 
          set of output values. 
        - Execution may cause sub tasks to be created.   
        - Completion of a sub task may trigger processing in the parent task. 
        NOTE: This may result in multiple declarations of completion. 
        This typically represents successively better, or alternative solutions 
        for the task. 
         
    <<>> 
    SPECIALIZATIONS 
    =============================================================================== 
    EVALUATION TASK --  
    OPTIMIZATION TASK -- (problem type, optimization space, target, metric, dataset) 
    CHOICE TASK --  
        Chooses between N matching refinements for a specified task 
    CONTROL TASK -- 
     
    DATASET -- (A set of parameter sets.) 
     
     
    <<>> 
    How might this all fit together: 
     
    Suppose user supplies three optimiztion tasks that all belong to a family of  
    problems that have been seen many times before. 
     
    Default execution for a user specified opt task, is to spawn a choice 
    task to decide the best process to apply to each. 
     
    Of the matching tasks, a previously learned decision surface for this  
    family of opt tasks has the highest utility, so it is executed. 
     
    It specifies use of 10-fold cross validation over a decision splitter. 
    (This is accomplished by coordinated sets of sub-tasks) 
    This decision surface also suggest a second approach with a much lower 
    utility:  10-fold cross validation over bagged decision trees. 
     
    The cross validated trees are computed for all users tasks and the results 
    are returned. 
     
    Successful completion of these three problems in the common family of opt 
    tasks, also triggers action in a low priority task for optimizing 
    the decision surface used by the choice task associated with this family  
    of opt tasks. 
     
    The new data causes no update to the decision surface. 
     
    The low priority task for computing bagged trees is now the higest priority 
    unprocessed task, and so it is executed. 
     
    Its results are reported to user specified task.  In this case the results 
    are much better than the simple decision trees, so they are conveyed as 
    a better solution to the user. 
     
    These new results are again matched by the task to optimized the decision  
    surface.  In this case, a sub task is created to try an learn the boundary 
    between a now significant subset of opt tasks better suited to bagging. 
     
    Many approaches are tried in turn by a meta-learning choice task. 
    - One of these tries to refine this decision surface using distinctions 
      that are used for other learning task families. 
    - Another approach simply applies all known problem metrics to the  
      instances of the family and tried to induce a better decision surface. 
    - The approach that first succeeds to beat the oridinal decision 
      surface, is a very specialized metric & associated procedure: 
      This metric applies to the output of a decision tree learner.  It  
      attempts measure the fraction of the discision surface (described 
      by the output tree) that is NOT axis aligned (it looks for jagged  
      sloped edges.  Only when there are enough jagged edged is the  
      second (much more expensive) opt task created. 
     
    This combo approach is instantiated with C4.5 as the first learner, followed 
    by a bagged version only when the JAG() metric has value greater than 78. 
     
    The meta-learning task declares a new decision surface for that family  
    of opt tasks.  The meta learner, however, worked very hard (tried many things) 
    to find this new decision surface.  It is not certain if is now simply 
    overfitting the family of opt tasks.  Thus it creates a low priority  
    task to recompute this decision surface from scratch using cross validation. 
     
    The meta-learning task also applies the new decision surface to all known 
    instances of the opt task family and find four tasks whose properties 
    suggest the combo approach suggested by the new decision surface.  Tasks 
    are created to recompute those problems using the combo approach. 
    Of the four only one is still of interest to the user, the other three 
    have had their utility contribution from the user set to zero.  (Still 
    even those will be computed eventually, since they lend evidence for  
    or against the new decision surface.) 
     
    Successful application of the combo opt. method using he JAG() metric 
    also serves as another training data-point for the meta-meta-learning 
    task of optomizing the meta-learning rule which suggests the combo approach. 
    Given enough supporting evidence over time might even cause that  
    choice alternative to become the default (first choice) when confronting 
    a new learning task. 
     
     
     
     
     
    <<>> 
     
     
     
     
    [TOP LEVEL CONTROL] 
    - At the highest level a set of control tasks manage a queue of tasks to 
      be executed. 
    - The next level is a set of control tasks 
     
    a single task management control procedure 
     
     set of control tasks control the execution 
      of the whole system.  Each of these control tasks represent 
     
     
     
     
     
     
     
     
     
     
    FROM T-LERANING 
    Let  
     
    - WW be a an example representation laguage 
      w   represents a single training/testing data point 
      w_t represents the target term in w???? 
    - PP is the space of learning problems. 
      P   in PP is a single learning task 
      P_w is a set <<>> of examples 
      P_c is the target concept for the task 
    - TT be a set of transformation (TT:WW->WW) 
      t   represents a single transformation that maps an example w onto w' = t(w) 
     
    - An operator O, in OO, maps T -> <<>> 
     
    - MM is the set of metrics, each M in MM maps problems from PP onto a real num 
     
    - Rules  
      P, t --> t' 
     
     
     
    Some transforms 
     
    - Drop attributes 
    - Functionally combine attrs to create new attr 
    - Demultiplex attrs 
    - Segment continuous attributes 
     
     
    IDEAS 
    - Tlearning is like decision tree learning except it does not 
      partition the data but rather set covers it 
      (at least if we assume that each example is predicted by 
       its most predictive single attr.) 
    - Need rich (recursive | rule-like) language for examples 
