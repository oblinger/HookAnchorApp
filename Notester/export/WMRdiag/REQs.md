# WMRdiag.REQs --

    FINISHED FOR THIS RELEASE.   
     
    We can now program each screen to order and limit the data as follows 
      1) From most to least or least to most accurate. 
      2) To include upto a fix number of words. 
      3) To include upto (or downto) a specific accuracy threshold. 
      4) To exclude words with too little data for hi-confidence. 
      These parameter may be applied in arbitrary combination in independantly 
      programmable way to each user screen type. 
     
     
    OUTSTANDING REQUIREMENTS ON THE WMR DIAGNOSTICS TOOL 
     
    Status Priority 
     
    DONE  *MED* 
      Allow users to independently choose word-list and screen type. 
     
    DONE  *HI* 
      Add ability to print words without any explicit scoring. 
      Don's idea is that teacher will be more comfortable sending a list 
      of words home for each child to work on, if there is no explicit 
      measurments on each page. 
     
    DONE *MED* 
      User must be able to dynamically enter a range of dates for display. 
      There must be an easy way to specify a range of dates immediately 
      prior to the current date (e.g. ``last two weeks'') 
     
     
    *LOW* 
      Some screens should be designed to show different fixed date windows on 
      the data (e.g. all data, last month, and last week). 
     
    DONE  *MED* 
      Error reporting facility.  JAVA's error reporting functions only write 
      to standard Error (stupid).  I want to redirect this to a click box on  
      the screen, and also write them to a file that can be automatically  
      written to floppy (just so that I can get actual error messages when 
      bugs occur in the field). 
     
    DONE  *LOW* 
      Build an install-shield installer for WMR diagnostics. 
     
    DONE  *LOW* 
      Write a bootstrap loader (in C) or dynamically create shortcuts 
      on installation to start tool without the use of batch files. 
     
    DONE  *LOW* 
      Get info about WMR file locations from registry. 
     
     
    There are a number of other features that that we have imagined adding to this 
    tool (e.g. computing the probablility that a child has systematic difficulties  
    with a particular class of words like ``th'' dipthong words). 
    I did not discuss this with the teachers, and I will work on such extensions 
    only if time permits. 
     
     
    DONE  *MED* 
      Sort words from least accurate to most accurate (or the reverse). 
     
    DONE  *HI* 
      Limit the list of words by: 
      - a fixed number (e.g. only show the first ten) 
      - a fixed score  (e.g. only show those below 50% accuracy) 
      - the number of occurances (e.g. only show words read more than 7 times) 
     
    *DROP* 
      Automaticity.  Display time spent reading each word. 
      (There are concerns with the meaning & ultimate utility of this display, 
       but I have included it as a possible extension since it is on a 
       requrements document that Diane has.) 
     
     
     
     
     
     
    *DROP* 
      Directory of word lists. 
      Some teachers though new word lists might need to be added without the 
      aid of a programmer.  One possiblity is to have a directory of text files 
      each of which is a different word list.  Thus notepad could be used to 
      add or update the set of existing word lists.  (This will also require  
      screens to be modified to allow dynamic selection of associated word list.) 
     
    *DROP* 
      Dynamically set word lists. 
      To reduce the number of preprogrammed screens, perhaps I should redesign 
      the system to allow the user to dynamically select the word list to  
      display on each screen. 
     
     
