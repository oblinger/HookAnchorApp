# Old-Prj.Stats --

      <<>>** 
       *   Mw[c,w]  matrix[Category, Word]     Num msg in cat containing word 
       *   Mt[c]    msgInCat[Category]         Total msg in cat 
       *    
       *   Tw[w]    = Sum_c Mw[c,w]            Sum all msgs containing word 
       *   Total    = Sum_c Mt[c]              Sum all msgs 
       * 
       *   Pb[c,w]  = ( Tw[w] - Mw[c,w] )      Prob of w in msg in outside cat c 
       *              <<>> ( Total - Mt[c] ) 
       *   Pw[c,w]  = Mw[c,w] <<>> Tw[w]          Prob of w in msg in cat c 
       * 
       *   result[c,w] = Pw[c,w] <<>> Pb[c,w]     Final word score 
       *<<>> 
     
     
    Goal: 
      Minimize Error Rate (k*Fp + Fn). 
      Assume 700 mgs per person. 
      Assume if more than N msgs contain word then report T else Nil. 
     
      Choose word w such that P(m in C | w>N) is as large as possible 
     
     
     
       Diluted z-score = (P(Mc|Mw)-P(Mb|Mw)) <<>> <<>> sqrt( P(Mb|Mw) ) 
     
     
       P(Mc|Mw) = Mw[c,w] <<>> Tw[w] 
       P(Mb|Mw) = ( Tw[w]-Mw[c,w] ) <<>> Tw[w]  
       diff     = -1 + 2 * Mw[c,w]<<>>[w] 
     
       P(Mb|Mw) = ( Tw[w]-Mw[c,w] ) <<>> Tw[w] = 1 - Mw[c,w]<<>>[w] 
       sqrt     = sqrt(1 - Mw[c,w]<<>>[w]) 
     
     
     
