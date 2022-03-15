# MailAddr -- Intelligent email addressing assistant

    peter said it did not find austin in '.../austin/...' 
     
     <<>>     
     <<>>     
     
     <<>>     
     
     
     <<>>     
     <<>>     
     
     <<>>     
     <<>>     
     
     <<>>     
     <<>>     
     
     <<>>   Mail translation rules 
     
     <<>>     
     <<>>     
     <<>>     Algorithms for finding mail communities 
     <<>>            Analysis of alternatives 
     <<>>                 Organization and installation of mail assistant 
     <<>>     
     
     
    Issues 
    - Access to incoming mail.  Java|Lotus Script 
    - Data storage 
    - Internal data structures. 
     
     
     
    MAIL ADDR TASKS 
    - uiDoc 
    - static vars 
    - lstAlternatives 
    - calling an agent; param passing 
     
     
    POINTS TO JACOB 
    - could use cascaded mail file 
     
     
     
     
     
    Assumptions 
    -  50K = 50bytes/record  x  ~1,000 cataloged email addresses 
     
     
     
    Sub Goals 
    - Java script that prints all to/from/cc in mail file 
    - Java button that prints hello in to field 
    - Simple button based address assistant 
    - Field-exit type address assistant          
     
     
    QUESTIONS 
     - doc.SendTo.count  
     - How to find fields in db? 
     - How to call fn w. args 
     - How to call agent 
     
     
    >>> DESIGN 
    DB-DESIGN 
    - NewMemo form constructed in Build and manually copied to MailAddr 
    - StdR4Mail-1 inherits from StdR4Mail and the NewMemo form from MailAddr. 
    - User DB inherits from StdR4Mail-1 
     
    OBJECT 
     
    - Form:NewMemo-1 
     
     
    >>> HOW TO 
    CREATE TEMPLATES     -- Workspace:File:Database:New  (change to .ntf) 
     
     
