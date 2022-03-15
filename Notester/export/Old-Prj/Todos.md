# Old-Prj.Todos --



    > Determine if all of the three skill mining hypotheses are true. 
      Try to fix them if they are not. 
      - USAGE HYPOTHESIS:      Experts discuss their skills in email. 
      - JARGON HYPOTHESIS:     Jargon words can identify some messages accurately. 
      - SEPERATION HYPOTHESIS: We can exclude unauthored or misleading messages. 
     
    * Determine if experts use skill words for 2 experts for 3 skills (6 people) 
    * Determine is non-experts use skill words as much 
    * Determine if misleading non-expert mail can be filtered 
     
    * Word counts table. 
      - Input: List of words for a single skill 
      - Output: Table whose rows are the users and whose columns are numbers  
                of skill words in each message.  Entries in the table specify 
                how many messages from that users contain at least 'n' skill 
                words, where 'n' is the column number. 
     
    * Raw words table. 
      - Input: List of users and words 
      - Output:  A table where each row is a message.  And all messages 
                 from a user are contiguous and ordered by the number of 
                 words from the specified list are in each message 
     
    * Infrastructure: 
      - get U102 to scan 
      - annotate each mail message with:  MASS-MAIL FROM TO CC as appropriate 
        (mass mail is not from or to you, or has many recipients) 
      - profile parsing 
      - Obj serialization 
     
     
      <<>>** Returns specified skill level for specified user and skill. 
       *<<>> 
      public static String getProfileValue(String user, String skill,  
            String templateDataset) 
     
      <<>>** Returns a map of the lines in a profile file. with a ':' character. *<<>> 
      public static Map parseProfile(String file); 
     
     
      <<>> Object serialization.  In U.java: 
     
      public static String pvarDir = "<<>>"; 
     
      <<>>** *<<>> 
      public static void setPvar (String name, Object value); 
     
      public static Object getPvar (String name) 
     
     
    ###################################   LEVEL1   ################################ 
     
    - INSPECTION      Understand cross learning results 
    - INFRASTRUCTURE   
     
    ###################################   LEVEL2   ################################ 
     
    >INSPECTION 
     <<>>     
     
      * Raw word analysis. 
      * Determine where failures are occuring. 
      * Connect original email to processing by adding message timestamp metadata 
        (the why function should call a submethod that computes this and other msg stats) 
      * Why User&Skill --> set of messages 
      * Why Message --> Raw features, NB impt keys 
        * Show raw mail features 
        * Classes in order of prediction 
     
    ? Explain why didn't a msg get voted java 
     
    >INFRASTRUCTURE 
     - ObjCashing, (Obj serialization) 
     
    ###################################   DESIGN   ################################ 
     
     
    > SKILL MINING HYPOTHESES 
      - USAGE HYPOTHESIS:      Experts discuss their skills in email. 
      - JARGON HYPOTHESIS:     Jargon words can identify some messages accurately. 
      - SEPERATION HYPOTHESIS: We can exclude unauthored or misleading messages. 
       
     
    - User X Skill matrix of messages profiler 
    - Parse user profiles 
     
     
    LIST 
    - List email by category MassMail From To 
    - HYPOTHESES 
      - Jargon Hypothesis 
      - Skill Usage Hypothesis 
      - Seperation hypothesis 
     
     
     
     
     
     
     
     
    > PARSING PROFILES 
      <<>>** Returns specified skill level for specified user and skill. 
       *<<>> 
      public static String getProfileValue(String user, String skill,  
            String templateDataset) 
     
      <<>>** Returns a map of the lines in a profile file. with a ':' character. *<<>> 
      public static Map parseProfile(String file); 
     
        
     
    <<>> Also pass 'tempateDataset' to buildAnonUser 
     
     
     
    ##################################   UNSORTED   ############################### 
          - Parse profiles (into what?) 
    ?     - Serialize Objs 
    *     - Out of mem error 
     
     
    ###############################   COMPLETED ITEMS   ########################### 
     
    # XLEARN      Run Cross Learning Experiment 
     
    >XLEARN 
    * - Build training DS 
          * file str; attr map extension 
    * - Build email DSs  
    * - Write Toplevel of experiment 
    *   - Run Experiment 
     
     
     
         
        - Build the lowest skip Usenet DataSet possible. 
     
        - Fast NB: first write  
     
        - Threshold prediction confidences. 
     
        - Compute Why on user&msg.  Returns list of top 'n' words  
          public static List DecisionProc.why(AttrVect av, Map args)  
          - int maxFeatures = Obj.getInt(args, V.WHY_MAX_FEATURES, 30); 
          - boolean showFeatProbs  
        - Print the Whyset for a user 
          public static List View.why(String user) 
     
        - Print mail resultset (in excel?) with labels 
     
        - See if a Java message with jsp, java, jre, jdk, srevlets, jar,  
              datainputstream, can be classified. 
        *<<>> 
     
     
     
     
     
     
     
    <<>> 
     
     
     
    1.5> Create taxonomy  (1.5 days) 
      - directory structure 
      - profile.txt 
     
     
    6> Accessing the email 
      - interning DS 
      - modify button to point to oblinger/watson.  (1.5 day) 
        - test button 
      - building email dataset  (4.5 days) 
        - skipping non-IBM email  (.5 day) 
        - anonmizing; and parsing user list  (1 day) 
        - Access to multiple notes files NoteAPI weirdness. 
        - Into dataset (1 day) 
     
    ------- 
    .5> applying ind alg  (.5 day) 
      & generating output 
     
    2> compiliation stats (2 days)  
      - write 2-3 comp stats to try 
     
    4> Toplevel structure  (4 days)  
      -  
     
    2> run experiment (2 day) 
     
     
     
     
    > Analysis 
      - Datacube 
     
      - Taxonomy parsing; aliasing 
      - profile parsing & attrvect gen. 
     
     
    precond  (1.5) 
    - Usenet folder 
     
    - obtain email data  
      * Creation tax ... 
      * button (1.5)          
      * send message       < Wed next 
      * 6/15 
     
    -  
     
     
     
     
     
     
     
     
     
     
     
     
     
     
     
     
    [LOG] 
     
     
    - Group seperators 
      - transpose matrix 
      - x-product axes 
     
     
    - Try to find distinguishing rules 
    - Better TFIDF stats 
    - Better data cleaning 
    - Combined seperation 
      - entropy measure 
     
     
    TopCoorelate 
     - all users           (add user selectors into code) 
     - getUserFeatures 
     - getUserXML 
     - getRules 
     - Coorelation Report 
       - PrintCoorelationVector 
       - CoorelateRule 
         - evalRule(features, rule 
     
     
     
     
    [TODO] 
     
     
    ToDo: 
    * Write mail file dataset builder 
    * get usenet data 
    ? Create list of categories 
    - Create profiles input doc 
    - Write access button 
    - Forcing users 
     
    <<>> 
    - Pricipled new/old feature selection. 
    -  
     
     
    - Learn expertise in ?x from mail data 
     
    X Batch relation computation. 
     
    i 
     
    By Next Friday 
    - Feature Coorelations 
      * Sub user features 
      * combos of attrs 
      ++ Save computed feature vectors 
     
    Immediately After 
    - Skill profile update demoable 
    - Have 68 Categories in system 
     
    Differential tfidf 
     
     
    - Correlational tester 
    - Ask Sam abt 'ibm.com' srch 
    - Get minstrel modem to wk 
    - Ping abt Net finity server 
    - Suck in features (select by person) 
    - Build GUI 
    - remove all non ibm.com 
      from a person's email 
     
    [ASAC] 
    - PING IGS folks 
     
    [OFF END] 
    - Get Palm dev env 
    - Understand PALM API 
     
     
     
    [DONE] 
    - Design Document for me and contractor 
     
