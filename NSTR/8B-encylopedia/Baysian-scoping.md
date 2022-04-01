# 8B-encylopedia.Baysian-scoping --

    Let d be a document in the set of documents D. 
    Let c be a category in Taxonomy T 
    Relations: 
      d  in         c    -- means document d is in category c. 
      c1 ParentOf   c2   -- means c1 is directly above c2 in taxonomy. 
      c1 AncestorOf c2   iff c1 ParentOf c2, or c1 ParentOf x and x AncestorOf c2 
      c1 SiblingOf  c2   iff exists x and x ParentOf c1, and x ParentOf c2 
     
    RULE #1: No document is in a category and its ancestor. 
             d in c1 implies P(d in c2)=0 where c2 AncestorOf c1  
     
    RULE #2: No docuement is in sibling categories 
             d in c1 implies P(d in c2)=0 where c2 SiblingOf c1  
     
    RULE #3: doc in lowest category. ??? 
     
    BP(d, c) is the Base Probability that d is in c given the words in d without 
             consideration of rules #1 & #2 
     
    SP(d, c) is the probability of document d being in category c give BP, and 
             rules #1 and #2. 
     
     
     
     
     
     
     
