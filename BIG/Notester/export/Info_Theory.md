# Info\_Theory --

     <<>>

    ENTROPY
      Differential Continuous Entropy:   h(x) = - Int f(x) log f(x) dx     (Differential entropy)
      Discrete entropy:                  H(discreteDistribution)  =  num bit for one char

     <<>>

    Surprise = log(P(x)) = some function of P(x)
    Surprise should be additive

    Expected surprise (amount of info)
    h(x) = -sum P(x) log(P(x))


    H(x,y) = - sum_x sum_y  p(x,y) log P(x,y)
    H(y|x) = - sum_xy p(x,y) log(p(y|x))

    Surprise
    H(x,y) = H(x) + H(y|x)
    H(x,y) = H(x) + H(y) iff x & Y independant

    H(x)-H(x|y) = H(y)-H(y|x) = I(x;y)    amount that y tells you about x


    H(*) = Surprise = Expected number of bits to send
     H(x)  bits for symbol
     H(x,y) bits to send both
     H(y|x) bits to sned y given receiver knows x



    H(x) = [ H(x|y) + <<>> = H(y)    (whole thing H(x,y) )



    D(q||p) = sum_x  q(x) log(q(x)<<>>(x))
