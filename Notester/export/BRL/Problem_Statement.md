# BRL.Problem\_Statement --



    GOAL:  An Artifical Student       electronic student?

    GOAL:  
    *  An artificial student that is as teachable as possible
    *  A flexible performance transfer mechanism where a human teacher
       transfers their ability to perform tasks to the AS

    CONSTRAINTS:
    *  EXPLICIT LEARNING -- Performace task is EXPLICIT.  This means:
       - Teacher can explain each step in their performance.
       - Teacher can express all knowledge needed to solve task.
       - Teacher will not necessarily have sound theory of task, (but can be complete, if prompted)
         (e.g. frame problem exists in teacher's K, but for any given missing antecedent the teacher 
          can readily supply it, given a relevant context)

    *  NO DISCOVERY LEARNING --  Student is never required to DISCOVER relevant knowledge.
       All relevant knoweldge is presented at least once, but the student *is* 
       required to integrate mis-matched and conflicting knowledge into an effective
       performance system.

    *  NO DISCOVER LERANING --  Teacher can perform the target task.

    *  DESIGNED CURRICULM --  Teacher has a mental model of the electronic student, and uses that model to construct a curriculum 
       designed to teach the ES the new performance task.

    *  DOMAIN INDEPENDENT --  The artificial student's learning is not tuned to specifically for the performance task's domain.
       - The background knowledge provided to the student may be quite tuned for a specific domain,
         the point is that the learning processes that operate on that knowledge should not be.
       - We are not allows to specialize to specific domains, but we may specialize to particular teaching transfer protocals.
       (learning is POST GENOMIC)

    *  BOOTSTRAPPING -- Knowlege used at beginning of AS teaching should be of the same kind, as the knowledge that
       results from teaching, such that the process can be repeated.
       Further, one can remove some parts of the background knowlege, and design prerequisite curriculum that could teach that knowledge.


    NON-GOALS:
    -  Learn sublingual concepts
    -  Abstract raw sensor inputs
    -  Learn anything not taught
    -  Build commonsense knowlege needed for given domain
    -  Tune learning for specific domain
