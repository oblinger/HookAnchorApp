# R.InformationTheory --

    ============================ 
    ===  INFORMATION THEORY  === 
    ============================ 
     
    X       A random variable.  
    Pi      Probability for each symbol x in X. 
          
    H(X)    = Pi log( Pi ) 
            Entropy.  Measure of the number of bits to code symbols on X. 
    H(X,Y)  = P(x,y) 
            Entropy of cross product.  # bits to encode a pair. 
    H(X|Y)  = 
            Conditional entropy.  # bits to encode X given Y 
    I(X,Y)  = H(X)+H(Y) - H(X,Y) 
            Mutual Information.  # bits saved by jointly coding X and Y. 
