
## TODO
- Screen for Mutual fund / Hedge fund owner to be your partner

## IDEA MECHANICS
### PARTS
#### THE FED
- Runs the show.  
- Oversees the execution of the member banks
- Oversees the software and execution of the blockchain
#### JUDICIARY / COMPLIANCE / ACCOUNTANTS
- Oversees the validation procedures
- Is independent from the fed and member banks
#### MEMBER BANK
- For profit transactors
- Hold the assets for the currency
- Provide liquidity for basket
 
### MECHANICS

### INITIAL POLICY
- Provide day to day / month to month price stability against a variety of other asset classes
- Maintain a geometric market cap to market price ratio
- Provide acclerating execution premiums

### THE MATH
- Mt   =  market cap at time t
- Pt   =  price per coin at time t
- Ep 
- E(2) =  early adopter bump.            E = Pa/Pb   where  2(Ma) = Mb
- Ep   =  doubling percent.              Ep = [E(2) - 1 ] * 100


- Dt   =  cap doubling at time t         Dt = log2(Mt) = ln(Mt) / ln(2)
- C(Dt)=  cost at doubling time t
- B    =  bump for a single doubling     B = C(Dt+1)/C(Dt) = C(Db)/C(Da)

#### Math try #3 


#### Math try #2
ASSUME   C(d) = e^(s * d)

B = C(Dt+1)/C(Dt)
B = e^(s * (Dt+1)) / e^(s * Dt)
ln(B) = [ s * (Dt+1)] - [ s * Dt ]
ln(B) = s * (Dt+1-Dt)
ln(B) = s
B = e^s

~-~~-~-~
C(d) = e^(s * d)
C(d) = e^(ln(B) * d)
C(Mt) = e^(ln(B) * ln(Mt) / ln(2))
price(Mt) = e^(ln(Mt) * Q)   Q = ln(B)/ln(2)
y=e^(ln(x * 1))

ln(2)=.692
ln(1.25)=.223

Q=1      B=2     +100% per doubling
Q=.3219  B=1.25  +25%  per doubling


~-~~-~
C(d) = e^(s * d)
ln(C(d)) = s + ln(d)
ln(C(d)) = ln(B) + ln(d)





#### Math Try #1
ASSUME   C(d) = log2(s * d)

B = C(Dt+1)/C(Dt)
B = log2(s * (Dt+1)) / log2(s * Dt)
B = log2(s * (Dt+1-Dt))
B = log2(s)
2^B = s
price(d) = log2(2^B * d) = log2(2^B) + log2(d)
price(d) = B + log2(d)

=====
C(d) = log2(2^B * d)
2^C(d) = B * 2^d
======
C(d) = log2(s * d)
2^(C(d)) = s * d








AS EXPONENTIAL
- s 	  =  scaling factor.             Mt = e^(s*Pt)
              2(Ma) = Mb
2e^(s * Pa) = 2(Ma) = Mb = e^(s * Pb)
2e^(s * Pa)         =      e^(s * Pb)

2 = e^(s * Pa - s * Pb)
ln(2) = s * (Pa-Pb)
ln(2)/s = Pa - Pb
Pb = Pa - ln(2)/s



b^(P1) = M1 = M2/2 = b^(P2)/2
2b^(P1) = 2(M1) = M2 = b^(P2)
2b^(P1) = b^(P2)
2 = b^(P2-P1)


b^(P1) = b^(P2)/2
b^(P1-P2) = 1/2

P1 = P2 - log_b(2)




- a    =  quadradic factor.           Mt = a(Pt)^2 + b(Pt) + c     (Assume b=0 and c=0)
a(P1)^2 = M1 = M2 /2 = a(P2)^2 /2
a(P1)^2 = a(P2)^2 /2

M2 = M1*2

