# Nina --

     <<>>

     <<>>
     <<>>
     <<>>     
     
     <<>>     
     <<>>     
     <<>>     
     
     <<>>     
     
     <<>>     
     <<>>     
     <<>>     
     <<>>     
     <<>>     
     <<>>     
     
     
     
    ----- 
     
    for an arbitrary random var X.   
    g 
    suppose we draw n examples iid  X1, ..., Xn 
     
    Let delta = P[ |Av(X1...Xn)-E(X)| > ep ] 
     
    delta = e^-n 
    ---- 
     
    PAC clusterable 
     
    - chain 
     
     
     
    Let 
    - D be a distribution of points 
    - P = P1...Pn be a partition 
     
    Tight_D(P) = Sum_<<>> dist(x,y)*D(x)*D(y)* SameCell(x,y) 
     
     
     
     
    PROOF SKETCH 6<<>>25<<>>99 
     
    * Enoughs points & all significant cells are accurately estimated 
     
     
    -------------- 
    CURRENT AGENDA 
    - think about how lenny's tighness fits proof structure. 
     
     
     
    NINA'S THM:                            
     
     
    THM: VC(C) = VC(TRIANGLE_C(c)) 
    --- 
    THM: if VC(C)=d then 
         the number of distinct truth-value-combos of m points is AT MOST m^d 
    --- 
    THM: if VC(C)=d then draw sample S that is poly(1<<>>e, log(1<<>>d), d) 
         then WHP S is an epsilon net for C 
     
     
     
    if VC>>     
     
     
    Sample space: OMEGA = <<>> 
    Set of all possible outcomes of experiment. 
     
    Event: A is OMEGA. 
    An event is a subset of OMEGA. 
     
     
    Chernoff 
    Sequence of <<>> bernolli trials 
     
    ------ 
    Central Limit Theorem 
    Given Random var X. 
          E(X) = Mu 
          var(X) = sigma^2 
     
    Let S = <<>> from X 
     
    Let R =  ( Sum(S) - n*Mu ) <<>> ( sigma * root(n) ) 
    E(R)=0 
    var(R)=1 
    -------- 
     
     
    given delta=.95 
    then use cdf for normal dist E=0 var=1 solve for  
    Let bound = cdf^-1(delta) 
     
    set R = bound. set ( sum(S) - n*Mu ) = n*epsilon 
     
     
     
    ---- 
    Strong law of large numbers 
    Let x1,x2,...,xn be a sequence of Independently & Identically Distributed R var 
        E(Xi) = Mu 
         
    then with P=1 Avg(Xi) --> Mu  
     
    ---- 
    Weak law of large numbers 
     
    P( |Avg(sample) - Mu|>epsilon ) lim 0 as N --> inf 
     
     
     
    Let  
     
     
     
    Sample space:  DxD 
     
    Rnd Var Dist(DxD) 
     
    tighness def E(dist(P)) 
     
     
     
     
    tightness = E(dist(p)) ~~ sample P ==  
     
     
     
     
    Sample space: D^N 
    Rnd Var R = Avg pair dist 
     
     
    ---   ---   ---   --- 
     
    Let DD = <<>> be initial sample space. 
        Pdd(x) = probability of x in initial sample. 
     
    We now define 
     
        TT = DDxDD 
        Ptt() = Pdd(Di) * Pdd(Dj) * norm 
          ... 
     
    We further define function   dist:TT--> R 
    It is the euclidean distance between the points  in TT. 
     
    Dist is a random variable. 
     
    Define tightness as E(dist). 
     
    ---   ---   ---  
     
    E(dist) - Avg(dist( 
     
     
     
     
     
    OMEGA 
     
    Let T be DxD such that P(t 
     
     
    Def sample 
    def rnd var 
    def tightness E(dist(P)) 
     
     
     
     
     
     
     
     
     
    Assume XX distribution of points 
     
    Construct DD  
     
     
     
    E(tighness(X)) ~~  
     
    tighness of a region doesn't change much with subsampling. 
     
     
     
    Question: 
     
     
     
