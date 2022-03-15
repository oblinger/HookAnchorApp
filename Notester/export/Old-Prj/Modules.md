# Old-Prj.Modules --

    ===================== 
    BACKEND DATA-CLEANING 
    ===================== 
    Input:  Notes mail databases 
    Output: Feature vector per person 
     
    > NOTES DB INTERFACE 
      Connects and scans a persons' mail database.  Also incrementally updates 
      word count statistics as subsequent mail traffic occurs. 
     
    > MULTI-LANGUAGE STEMMER 
      Converts free text into a stream of canonical tokens for each word. 
     
    > TERM EXTRACTION 
      Identifies frequently occuring multi-word terms, and replaces them with a 
      single term token. 
     
    > ATTACHMENT READER 
      Extracts free text from many common attachment types. 
     
    > STRUCTURAL FEATURE COMPUTATION 
      Computes features based on the graph structure formed by the network  
      traffic.  (E.g. number of messages sent to known experts on topic X.) 
     
    > WORD CONTEXT RESOLUTION 
      Each word annotated with limited context information.  This context 
      information is available to the learning algorithm, and may be used 
      in weighing the importance of the word in inferring expertise. 
      Typical context terms would deal with the placement of the word in 
      the document, as well as aspects of the document itself. 
      Examples include: "In Mass Mailing", "In Subject Line", "In Replied To Mail", 
      etc. 
     
    > FEATURE VECTOR COMPILATION 
      Uses modules above to compute a sparse feature vector representation of  
      a single employee's email traffic.  This large, sparse feature vector  
      contains annotated term counts & structural features representing the 
      total email traffic for a single employee. 
      - The feature counts are accumulated from all email traffic for that employee 
      - Contribution of individual documents are attenuated over time. 
      - Term count contributions for each document are normalized by the size 
        of the document or absolute counts are compressed. 
     
    > INCREMENTAL VECTOR UPDATE 
      Maintains a repository of current feature vectors for each user, and  
      updates the counts as new mail traffic is available. 
     
     
    ====================== 
    CLASSIFICATIION SYSTEM 
    ====================== 
    Input:   Manually entered expertise assignments, email traffic. 
    Output:  Expertise classifiers 
     
    > TRAINING DATA EXTRACTOR 
      Extracts expertise classes from expertise locator, and feature vectors 
      from prior email traffic to build a training dataset. 
     
    > LINEAR CLASSIFIER 
     
    > OBLIQUE TREE CLASSIFIER 
     
    > NEAREST NEIGHBOR CLASSIFIER  
     
     
    ========================= 
    ADMINISTRATOR'S INTERFACE 
    ========================= 
    Input/Output:   
      Taxonomy of expertise categories, 
      the computed expertise classifiers,  
      the list of employees 
     
    An interactive web-based tool that allows inspection and modification of the  
    resources above. 
     
     
    ==================== 
    EMPLOYEE'S INTERFACE 
    ==================== 
    Input/Output:  Expertise matrix. 
     
    Interactive tool that validates user and allows interactive modification of  
    the users profile. 
     
     
     
    ================ 
    SEARCH INTERFACE 
    ================ 
    Input:  Expertise matrix, user queries 
    Output: Profiles of employees with sought expertise 
     
    Web-based boolean constraint search engine. 
     
