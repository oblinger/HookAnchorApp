# 999inf-ext.Todo-list-mgmt --

    Idea:  To allow pages to be computed from parts of other pages 
     
     
     
    ASSUME: 
    - Page section announces where forwarded links go 
    - Each section marks items to be forwareded to parent pages 
    - Forwarded links maintain their order in a parent, and are  
      grouped together in parent 
     
     
      Node: master-todo 
     
      * (proj1)::  Setup conference room 
      * (proj1)::  Get tickets for speaker 
      * (eb-proj):: 
            * (eb-proposal):: Wait for response & send proposal 
            Build prototype 
            Scan data 
      *  
     
     
    IDEA: 
    - Allow page to be a multi-page composed of other page pieces. 
    - Multi-page is computed dynamically from a base page text and a set of 
      index offsets, and piece identifiers 
    - After edit, all piece sources are update. 
    - Pieces can be embeddeded & original source is always updated. 
     
      EXAMPLE: 
     
      [ACTIVE] 
      Skill project 
      - Finish proposal 
        - Run tests for performance figs 
      - Build prototype 
        - Get API  
     
      [WAITING] 
      - xxx 
     
         Meta-Data:  Idx:13 (skill-proj)top#active 
                     Idx:15 (ai-pic)schedule#active 
     
     
     
      (skill-proj) 
       
      [ACTIVE] 
     
     
     
     
    IDEA: 
     
    > AUTO-LINE-BASED X-fer 
     
      - Special heading indicator maps marked entries from one node/region' 
        to another node/region 
     
    Ways to align line in parallel regions 
    - All aligned regions maintain a consistently ordered set of linked 
      lines.  When a node is updated, all linked lines in other notes are also 
      updated. 
     
     
      <-> (master)todo, (fun)todo,  
     
     
    Implementation: 
    - update-linked-entries node-body [entries] 
      Returns a list of entries from a node body, or splices new entries 
      in, and returns the new node body.  entries with INS-Before or INS-After 
      are inserted into the body 
     
