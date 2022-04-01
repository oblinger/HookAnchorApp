# Vilalta --

     <<>>   
     <<>>     
     <<>>     
     
    <<>> 
    1) Find application of his meta learning 
    2) Build class of examples that allow local learning w<<>>o global learning 
     
    basic idea is to re-represent problem as prediction from local labels 
    <<>> 
     
     
     
     
     
    InBetween --  IB(A, B, P) = 1 iff (A-B)^2 > (A-P)^2 + (B-P)^2 
     
    Neighbors -- N(A, B)=1  if not exist X_i where IB(A, B, X_i) 
     
    FractionInBetween -- FIB(A, B) = Sum_<<>> IB(A, B, X_i) <<>> n 
     
    Varience(A,X) = Count_i  [N(A,X) and Label(A)==Label(X)] <<>> Count_i N(A,X) 
     
    Varience(<<>>) = Sum_i Varience(X_i, X) <<>> n 
     
     
    ------ 
     
     
    ---------- 
    ECAI paper 
    ---------- 
     
    <1,1,1,1,1> 
     
    F(x) = computes contra by dist 
     
     
     
    WAYS TO MOTIVATE 
    - Outperform all others 
    - Predict performance of methods 
      measure coorelation between classifier instances and theta space location 
    - Frameowork for all learners:  f(point pairs) -> theta -s 
     
     
     
     
     
     
     
     
     
     
     
     
