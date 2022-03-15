# Wmr.Chaff -- The chaff test

    d,Book.dat 
    - 
    w,0 
    t,0,one,0,0,0,0 
    t,1,own,1,0,0,1 
    t,2,no,0,0,0,0 
    - 
    w,1 
    t,0,one,0,0,0,0 
    t,1,own,1,0,0,1 
    t,2, 
     
    Usage  
    1) Start program 
    2) Click on a word 
    3) Wait for it to turn red 
    4) Say some word 
    5) Click on the word you actually said 
    6) Repeat steps 2-6 as desired 
     
    - If you want to ignore (not log) a particular attempt, then press "IGNORE" 
     
    - If you want to move to different sets of words use the next/previous 
      line or page buttons. 
     
    - The sets of words are defined in the boot.dat file 
      - Each "$TXT " line defines an alternative in the lists shown 
      - Each "$WRD"        line seperates one set of alternatives from the next. 
      - Each "$PAG"        line seperates one page of sets from the next. 
      See the \CHAFFTST\DATA\BOOK.DAT for an example. 
     
     
     
    LOG FILE FORMAT: 
     
    d : Name of the data file (file containing the test words) used. 
    - : Separates testcases. 
    w : Wave file number of word pronounced.  
        Suppose this number is 5, then you can hear the word pronounced by  
            double-clicking audio5.wav in the output dir of the test session.  
    t : Testdata, there is a record for every word displayed on the screen  
            for this testcase.  The 6 fields for each record contain the following: 
        - Index of the word in the data file.  We start counting from 0.  
              Therefore index n means, this is word n-1 in the data file. 
        - Word (string) 
        - 1 indicates this was the word tested.   
        - Score that came back from the speech engine. 
          Sometimes all words have score 0, meaning no word was recognized. 
        - n means this was the word with the n-th score. 
          0 means, this word got no score at all.   
          1 means, this was the word with the highest score.  
          A higher number means this word was one of the alternatives. 
        - Flag indicating if this was the word actually pronounced. 
          When a kids makes a reading error, they may pronounce another  
              word in the list than the word that is being tested, or they may  
              come up with a totally different, or non-excisting word.  After  
              each testcase, the test leader can click on the word that was  
              actually pronounced, or click outside the words to indicate the  
              word pronounced was not in the list. In the last case, all words  
              will have 0 in this field. 
     
     
