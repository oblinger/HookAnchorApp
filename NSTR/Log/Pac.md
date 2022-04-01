# Log.Pac --

    if P(moved points) < epsilon   then delta tightness(region) < max-D * epsilon 
     
     
     
    Lemma 1: GridAlign(D)  
     
    Lemma 2: Movement ( ) 
      Let S be set of pts 
          S' disjoing set of points D' is dist of S u S' 
          S'' disjoing set D'' S u S'' 
     
    P(s') = P(s'') 
    P(s') <<>> P(S u S') < epsilon 
     
    Tightness_D'(R) ~~ Tightness_D''(R)  
     
     
    delta @ G = P(D @ G) - P(D^ @ G) 
     
    with conf delta we know    delta @ G < epsilon 
     
    let S be all points in D. 
    let S' be a smallest set of points such that if S \ S' 
       
    P(D @ G) = P(D^ @ G) for all G where set of point in D that are over subset of points 
     
    sum over all g in C is less than (epsilon * C) 
     
     
     
     
    D is true dist 
     
    Tight_D(Pi) ~~ Tight_D^(Pi) 
     
    Tight_D(Pi) ~~ Tight_Grid(D)(Pi) ~~  
     
    Tight_Grid(D^)(Pi) ~~ Tight_D^ 
     
     
    P(s') < epsilon 
     
     
    Limit sum delta(prob at grid pt) < epsilon 
     
     
     
     
     
     
     
     
     
     
     
     
     
     
     
     
     
     
    S* set of points. 
    V set of voronoi k-partitions over S*. 
     
    EPD(DD) = Sum_<<>> dist(x,y) * DD(x) * DD(y) 
    Bigness(D, P) = max_<<>> EPD(D&Pi) 
     
     
    Best_D = MIN_<<>> Bigness(D,p) 
    Opt_D  is a p in K with Bigness(D,p) = Best_D 
     
    GOAL: 
    Bigness(D,Opt(S)) <= Bigness(D,Opt(D)) * (1+epsilon) with prob 1-delta 
    where |S| is poly in k, 1<<>>epsilon, 1<<>>delta, N 
     
    Bigness(S*,Opt(S)) <= Bigness(S,Opt(S)) * (1+epsilon) 
     
    === 
     
    DELTA(Opt(S*)) = <<>> 
    DELTA_epsilon(Opt(S*)) = <<>> 
     
     
     
    S is an-epsilon net for D(C*) if every region of D_epsilon(C*) is hit by 
    a point in S.  That is for every R in D_epsilon(C*), S & R <> null 
     
     
    ==== 
    WTL 
     
    A sample, S, f-covers a set of clusters, C, iff 
    for every c in C, |c&S|>= f*|c| 
     
    if |S| > poly(f, |all-points|, 1/delta, |C|) then 
      P(random S not f-cover C) < delta 
     
     
     
     
     
     
     
     
     
     
     
     
     <<>>     
     
     
     
     
