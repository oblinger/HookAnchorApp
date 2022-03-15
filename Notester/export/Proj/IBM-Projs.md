# Proj.IBM-Projs -- Potential projects

    PROJECTS 
    - Speech Diagnosis 
    - Speech Tutor 
     
     
    > PEOPLE TO TALK WITH 
    Handicapped speech input (peter knows this person says bill). 
    Low-level speech guy 
     
     
    > SPEECH DIAGNOSIS 
     
    GOAL: Recognize reading/speech difficulties. 
     
    REALIZATION:  
    - (Automtatically?) Associate words with a prenumerated set of difficulties. 
    - Statistically test for these difficulties. 
     
     
     
    > DIFFICULTY TYPES 
      - Articulation difficulties 
        LRH problems 
      - Selection Errors 
        long vs short vowels 
        hard/soft C 
      - Recognition of special case 
        - Dipthongs: ``TH'' 
       
      -  
     
     
     
    > METHODS FOR TESTING 
      - Class vs. Background differential error rate. 
      - Exhaustive Selection Alternative Analysis. 
     
     
     
     
    > SPEECH TUTOR 
     
    GOAL: Diagnose difficulties and provide specialized tutoring. 
     
     
    TUTOR TYPES 
     
     
    WORD FEATURE TUTOR 
    GOAL: Teach the feature common to a set of words. 
     
    A feature is set of words with some sub-portion annotated,  
    or a function that accepts and annotates a specific feature. 
     
    BEHAVIOR: 
    - Optionally Reads word list for child 
    - Listens for each of the words from child 
    - If many are missed repeats set 
    - If few then just says and listens for that one 
    - Exit if hi-accuracy or hi-frustration else select next set. 
     
     
     
    Data Maintained for each child-feature pair: 
    - Failure Rate, and confidence (std) 
    - Novel Failure Rate, and confidence. 
     
    Data needed for each feature: 
    - Optional fn that annotates arbitrary words 
    - Word list 
    - Annotated word list (serves as exception list too 
     
     
    <<>>  Let's practice the ``PH'' sound. 
    |                          PH    |  I've written down some words 
    | <<>>           |   with ``PH'' sound. 
    | |  telePHone <<>>   |           |  They are: telePHone, PHoto 
    | |      PHoto       | \    *    |  Now you try... 
    | |  ...       <<>>   |  `---+    |  (Each word is highlighted in turn) 
    | |            <<>>   |      |    | 
    | |                  |      |    |  Lets try some more, this time you 
    | <<>>     <<>> \   |  read them first... 
    |                                | 
    <<>> 
     
     
     
     
    INVOKED: 
    -  
    - In WMR child clicks on the professor, and he asks 
     
     
     
