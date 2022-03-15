# Java.Limitations --

    GENERAL 
    - No macros 
     
    SPECIFIC 
    - not recursively interchangeable (cannot embed stmts in expressions) 
     
    - Limits on modularity: e.g. cannot put listener code w. object 
      all code for a class must reside in class def 
     
     
     
    public U.GUI MyButton; 
    ... 
     
    void listener (Event ev) { 
       Object Source = ... 
     
       switch (handler) { 
          case  
     
