# R.LSA --

    LSA -- Latent Semantic Analysis 
     
    Good intro by the "inventors". Light on the maths and a bit preachy but a 
    good overview: <<>> 
     
    This one's a bit more mathy: <<>> 
     
    It's cool stuff. You take a term by document matrix and decompose it into a 
    singular value representation: A = U S V' where U and V are orthogonal, S is 
    diagonal and ' means transpose. You order the values in S from highest to 
    lowest, truncate S to S_k, where you keep only the k highest values. Then 
    form A_k = U S_k V'. A_k is a sort of generalization of the original 
    documents and their terms. It seems to find term-term relationships that 
    aren't necessarily explicit in the original document set (ie, "partial" and 
    "ordinary" are related via the terms "differential" and "equation" even 
    though they may never appear in the same document together). 
