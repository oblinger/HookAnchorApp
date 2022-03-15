# TL.Text --




    <<>>
    <<>>
    <<>>

    Setup conf call:  DanA, DanO, Leslie, Tomas, sing1, sing2, MIT prof

    - YoonKey




    Is this an efficient basis to build from?
     - What data are we starting with?
     - What feature analysis code are we starting with?
       Code for:  Name->EntityClassification
     - difference in problems

    Is this a domain that could yeild transistion
     - Who would be the customer


    --------------------------------------------------------------------------------

    EACH TEMPLATE FILLING TASK   (TASKS WHERE TRANSFER LEARNING CAN BE APPLIED)
    (1) Identify instances of a specific textual form.   (e.g. Terriorist Incident)
    (2) Fill in fields from text for that instance of the general form.

    THE COMPOONENTS

    THE ARCHITECTURE
      ENTITY LABELER:    Documents (with segmented entities)  --->  algorithm labels entities      (they currently have an adaptive alg for this)       MAX ENTROPY
      SYNTACTIC PARSER:  Part of speech; phrase segmenter                                          (Not adaptive)
      FIELD FILLER:      Id field entries; does not handle conflicts.
      TEMPLATE MERGING:  given possible field assignements --> filled consistent templates.

    THE DATA
      SOURCE:     MUC4 - Set of 700 docs & templated items of interest from Terrorist incidents
      TARGET:     Same form different medium/genre
      TARGET:     Different form different medium   
                  NAME-ENTITY DATA (from Conll) - 900 docs   (sing will add template)       <--- just to test components not whole system
                  Target of full system:  Speech transcripts; news articles on the web (with manual labelling)

    PROPOSED WORK
    (1) Test existing TL algorithms on Entity Labelling problem & compare to Max Entropy.    (Train/test data ready)
        PLAN:  WebData --> DSO feature extraction files --> either party applies algs
    (2) Do research to apply existing/new TL algs to: Parsing, Filling, Merging
        PLAN:  
    (3) 
    (3)




    ----------------------------

    WSfile
    --- Human inspired reasoning
    --- Horizon scanning
    --- MadCat <---> Adaptive text classification ?
    --- TL  <---> Transfer











    Stan Mushaw

    DIRO Space & International

    (571) 218-4427
