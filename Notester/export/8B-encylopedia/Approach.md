# 8B-encylopedia.Approach -- Description of the scoping idea

    Inputs: 
     - A taxonomy T, represented as directed graph of classifiers, <<>> 
       Each node in the taxonomy has: 
       - a name 
       - a classifier that computes a score for any given document 
       - in-links  (up) 
       - out-links (down) 
     
     - A set of unlabelled documents, Di. 
     
    Output: 
     - An association score between each document, Di, and each Taxonomy node Tj. 
     
     
     
    Algorithm: 
     
    Parameters:  
    - Suppression function, SUPP 
    - Association threshold, TH 
     
    1) For each node Tj compute its neighbor hood set, NEIGHBORS(Tj), as a 
       set of nodes Tk and a path specificaton from Tj to Tk (see below). 
     
    2) Compute the classification SCORE(i,j) of Di on node Tj for all docs & nodes 
     
    3) For each unlabelled document, D, in <<>> 
     
    4)   For each node, T, in <<>> where SCORE(i,j) > TH 
     
    5)      Compute suppressed score SS = SUPP( NEIGHBORS(Tj), SCORE(i,*) ) 
     
    6)      If SS > TH Then set SSCORE(Di, Tj) = SS 
     
    7) Return SSCORE 
     
     
     
