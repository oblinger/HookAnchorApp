# 8B-encylopedia.Problem -- Problem specification

    TASK 
     
    Given a document taxonomy and a set of unlabelled documents, associate 
    the documents with the taxonomy in order to facilitate later retrieval 
    of the documents. 
     
    Specifically we want to generate a possibly empty set of appropriate 
    nodes with in the taxonomy for each document. 
     
     
     
    APPROACH #1 -- Treat each node as an independent classifier 
     
    Much work on many different types of classifiers. (Bayes, neural,decision tree) 
    Any of these could be used to classify into two categrories those that 
    do and do not belong to a given node. 
    Thus an unlabelled document could be tested against classifiers for each  
    node in the taxonomy, and the set of node for which it was a member could 
    be returned. 
     
    PROBLEM WITH APPROACH #1 
     
    1) Ignores scoping. 
     
     
     
    APPROACH #2 -- Multi-class classification 
     
    A traditional solution to this problem is to use a classifier that 
    classifies docuements into many classes, one for each node in the tree. 
    So an unlabelled document is presented and the *single* best node of the 
    taxonomic tree is returned. 
     
    To some extent this approach handles the scoping problem.   
    Given document that has some relevance for a particular node in taxonomy, 
    but is better placed in a different node, this approach will succeed  
    since it is forced to return only one best node (its classification) for 
    the given docuement. 
     
    PROBLEM WITH APPROACH #2 
     
    2) Ignores composite nature of documents, and  
    3) Ignores the non-exclusivity between nodes in document taxonomies. 
     
     
     
     
