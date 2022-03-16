# Nina.E-proof --

    Dan, 
    Here's a rough e-proof sketch<<>>general idea of what P. Pitt and 
    I discussed at colt. 
    -n. 
     
    ----------------------------------------------------------------------- 
     
    d_p: Let d_p be the vc dimension of k-sided convex polytopes in n dimensions. 
    d_x: Let d_x be the vc dimension of xor of pairs of k-sided 
         convex polytopes  in n-dimensions 
     
    Let S be a sample and D be the distribution. 
    Implicit in the sample size bounds below is a dependence on 1<<>>epsilon, 
    1<<>>delta (or ln 1<<>>delta) that we're ignoring for now. 
     
    ********************************************************************* 
    Lemma <<>> ``Can Hit A Heavy XOR Region With m_1 Points'' 
       With a sample S of m_1=poly(d_x) points and for polytopes P1,P2 
       [D(P1 xor P2) >= a]   ->  [S contains a point in (P1 xor P2) whp] 
    ********************************************************************* 
     
    T_D(P)->[0,1]: For a polytop P, Let T_D(P) be the tightness of P 
                   relative to D.  Bounding tightness to be <= 1 
               gives us a way of bounding how much a cluster can 
               effect the overall tightness. 
     
    ********************************************************************* 
    Lemma <<>> ``Can Estimate Tightness of a Polytope with m_2 Points'' 
    If S is a sample of m_2=poly(n,k) points in a polytope P then whp 
        T_S(P) ~ T_D(P) 
    ********************************************************************* 
    (Note: By T_S(P) we mean measure tightness of P by sampling uniformly 
    from S with replacement) 
     
    T_D^*(P)=Sum_i  D(P_i) T(P_i): Let T_D^* denote the tighness of a clustering 
                               P_1,..P_k. 
        (Note: later we'll have to sort out the detail of estimating D(P_i).) 
     
           _ 
    S,k-> |A| -> P_1,..,P_k  : Given a set of points S and an integer k 
           -    let A be a clustering algorithm that outputs polytopes 
            P_1,..,P_k s.t. 
                       T_S^*(P_1,..,P_k) = min_<<>> T_S*(C1,..,Ck) 
     
            Ideally, we'd like A to be an approximate clustering 
            algorithm, but this causes problems later on (in the 
            "chain of Epsilons" theorem), maybe. 
     
    ********************************************************************* 
    Lemma <<>> ``Can Obtain m_2 Points In A Heavy Polytope With m_3 Points'' 
    Let P be an e-significant polytope.  Let S be a sample of m_3(ln 1<<>>delta) 
    points drawn from D.   Whp will obtain m2 points in P.  (And thus 
    by Lemma B can estimate T_D(P).) 
    ********************************************************************* 
     
    Corollary of <<>> 
    Let P_1,..,P_x be e-significant polytopes.  m3(ln x<<>>delta) points are 
    sufficient to estimate each T_D(P_i) w<<>> prob >= 1- delta<<>>x 
     
     
    Let S be a sample of m>=max(m1,m3(ln m^<<>> <<>> delta)). 
           (Can solve for m above and we're still polynomial.) 
     
    The algorithm is:  run A on S. 
     
    ********************************************************************* 
    Lemma <<>> ``Have No Fear'' 
    If P_i and P_j dichotomize S in the same way then 
      (i)   D(P_i xor P_j) is small. 
      (ii)  T_D(P_i)~T_D(P_j) 
      (iii) T_S(P_i)~T_S(P_j) 
    ********************************************************************* 
    Pf of (i): If bwoc D(P_i xor P_j) is large, then whp we would 
    have obtained a point in this significant difference region by Lemma A. 
    Thus P_i and P_j would not have dichotomized S in the same way, 
    a contradiction. 
     
     
    ********************************************************************* 
    Lemma <<>> ``|Dichotomies| grows Polynomially'' 
    If S is a sample of size >= m points then 
        |Dichotomies of S| <= m^<<>> 
    ********************************************************************* 
    Pf: B/c vcdim of polytopes is d_p & thus |dichotomies| grows like 
    m^<<>> and not 2^m.) 
     
    Let t=m^<<>> and let r_1,..,r_t be representatives of the t 
    dichotomies.  We want to estimate the tightness of these representatives 
    effectively.  If we can do that then we'll be happy because every 
    polytope P can be mapped onto one of these representatives. 
     
    ********************************************************************* 
    Lemma <<>> ``Sample Tightness of r_i Close to True Tightness of r_i'' 
    Can estimate T_D(r_1),..,T_D(r_t) with T_S(r_1),..,T_S(r_t) 
    respectively, whp. 
    ********************************************************************* 
    Pf: Assume r_i is e-significant.  (If it was not e-significant then r_i 
    can contribute at most Pr(r_i)*T(r_i) = e*1 to the overall tightness.) 
    By Lemma C, we obtained enough points (>= m_3) in r_i to approximately 
    estimate T_D(r_i) by T_S(r_i). 
     
     
    For a polytope P_i, let r(P_i) be the representative r_j s.t. 
    r_j dichotomizes S the same way that P_i does. 
    Suppose that A outputs P_1,..,P_k and the optimum polytopes 
    are P_1^*,..,P_k^*. 
     
     
    ********************************************************************* 
    Lemma <<>> ``True Tightness of P_i Estimable With Sample TIghtness of r(P_i)'' 
    T_D(P_i)~T_S(r(P_i)) whp 
    ********************************************************************* 
    Pf: 
    (1) T_D(P_i) ~ T_D(r(P_i)) whp 
    By Have No Fear since P_i and r(P_i) dichotomize S the same way, 
    D(P_i xor r(P_i)) is small  and thus T_D(P_i)~T_D(r(P_i)). 
    Note that the same argument can be made for P_i^*, namely that 
    whp T_D(P_i^*) ~ T_D(r(P_i^*)) 
     
    (2) T_D(r(P_i)) ~ T_S(r(P_i)) whp 
    Immediately from Lemma F. 
     
    Combining we have that 
    T_D(P_i) ~ T_D(r(P_i)) ~ T_S(r(P_i)). 
    and the lemma follows. 
     
    ********************************************************************* 
    Corollary: 
    T_D^*(P_1,..,P_k) ~ T_S^*(r(P_1),..,r(P_k)) 
    ********************************************************************* 
    Pf: By applying the above theorem to each polytope P_i and doing the 
    appropriate math for D(P_i) 
     
    Our real goal was to show that T_D^*(P_1,..,P_k)~T_D^*(P_1^*,..,P_k^*) 
    and we're now almost there. 
     
    ********************************************************************* 
    Final Theorem: ''Chain of Epsilons'' 
    T_D^*(P_1,..,P_k)~T_D^*(P_1^*,..,P_k^*) 
    ********************************************************************* 
    Pf: We have an algorithm A that given a set of points S outputs a 
    clustering P_1,..,P_k with optimal tightness.  How does its 
    tightness measured wrt D compare to the tightness of the best 
    clustering P_1^*,..,P_k^* measured wrt D? 
     
    (1) T_S(P_1,..,P_k)     ~ T_S(r(P_1),..,r(P_k))     HNF (iii) 
                ~ T_D(r(P_1),..,r(P_k))     Lemma F 
                ~ T_D(P_1,..,P_k)       HNF (ii) 
     
    (2) Same for P_i^*, namely: 
        T_S(P_1^*,..,P_k^*) ~ T_D(P_1^*,..,P_k^*) 
     
    (3) T_D(P_1,..,P_k) <= T_S(P_1,..,P_k)            + epsilon 
                                 [  by (1) above ] 
                <= T_S(P_1^*,..,P_k^*)        + epsilon 
                              [ b/c by defn P_i minimizes T_S ] 
                <= T_D(P_1^*,..,P_k^*)<<>> epsilon 
                                 [ by (2) above ] 
     
    ------------------------------------------------------------------------------ 
     
     
     
