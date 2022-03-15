# 95-Assistant -- Intelligent email addressing assistant

     <<>>     
     
     <<>>     
     
    --- 
    - => maxSize -> Focus 
     
     
    @ibm_internal 
    @...@ 
     
    Desc, Prereq, Install, Support status, get it hints&tips 
    Rich Holodak 
     
    863-7830 
     
    - Happy about delay 
    - Status 
    - Material for meeting?  Slides? document? 
    - Informal points of contact? 
     
     
    grand-central station (extractor) 
     
    - Distribute: Tim Dinger, Vinny Cina 
     
     
     
    Dan, 
    We've had some changes in addressing schemes (and probably will have more) that invalidate lots of things hanging around in our mail databases. For example the form "joe blow/watson/ibm @ ibm research"  or the "joe blow/watson/ibm research" forms are no longer used and will cause mail to be returned. 
    The invalid forms get mixed up with the valid ones and I sometimes inadvertently select the wrong form. Here are the things that are returned by a search for "kellogg" (trying to get the real address for Wendy Kellogg). 
    wendy a kellogg/watson/ibm 
    wendy kellogg/watson/ibm@ibmus 
     wendy kellogg 
    wendy a kellogg/watson/ibm@ibmus 
    wendy a kellogg/watson/ibm research 
    wendy a kellogg/watson/ibm@ibm researchcatherine a chess (yes, this was in an address so it was dutifully returned). 
    The problem is that only two of these lines work, the first and the fourth. The others are bogus, but not some not obviously so.  
    You might consider being able to make a list of forms (or names) that ought to be removed after names have been re-cached.  
    Peter 
     
     
    === 
    Multi-word proposal 
    - Compute salient single & multi-term terms 
    - Add second scroll region to the right 
    - When a term is highlighted the appropriate sub terms are display to right 
    - scroll words are highlighted w. no effect 
      "->" button adds to search 
      "=>" button replaces search 
      double click adds to clipboard list 
     
     
     
    Search for m-of-n over a set of selected multi-word terms 
     
     
    Search:  training   
     
    Returns: 
     
    Bill Adams:     Jim Johnson 
    Bill Adams:     Dick Lam 
     
     
    List related terms: 
    - term 
    - term 
    - term 
    ... 
    singletons 
     
    === 
     
     
    peter said it did not find austin in '.../austin/...' 
     
    David wood (virtual assistant) 
    use mail assistant  
    ----- 
    Alex Morrow (in lotus & in IBM) 
    show him mail assistant 
    ----- 
     
     
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
     
     
