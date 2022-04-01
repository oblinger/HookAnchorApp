# Nina.Find-feature-sets --

    Given: 
     
    - F set of features 
    - N num rows 
    - D matrix of N x |F| 
    - S min support 
    - M max size of membership queries 
     
    Find: 
      all feature sets, f subset F,  such that Support(f, D) > S and 
      for all f' parents of f Support(f', D) !> S 
     
    In time poly: N, number of max freq subsets, num min infreq. subset 
     
     
     
    ==== 
    Choose P1...Pm st 
    not exist P1'...Pm' 
     
    where Max(vol(P1...)) > Max(Vol P1') 
     
     
     
     
