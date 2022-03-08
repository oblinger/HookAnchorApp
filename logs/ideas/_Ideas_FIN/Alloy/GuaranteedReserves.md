


### Idea

Create a cryptocurrency that is backed by other crypto with zero counter party risk.  (e.g. there is currency risk if the other currency goes to zero, but there is no counter party risk.)


### Key Ideas

QUESTION:  Why hold dollars when the USG is free to (and actually does) devalue them?

Answers:
- LIQUIDITY
- COUNTER-PARTY RISK
- VOLUME
- STABILITY







### Math


Community parameters
- BFA(i) -- buying friction for asset i
- SFA(i) -- selling friction for asset i
- Spread_A(i) -- the exchange spread allowed for asset i
- Target_A(i) -- Targeted number of active exchanges for A(i)
- Volume_A(i) -- Targeted available volume per day on exchanges for A(i)



Assume n coin assets with no counter party risk.


TCC = Total Coins Created

TAi = Total Reserve Asset i  (total of all blender transactions for asset i)
PAi = Current price of Reserve Asset i (in coins)

TAV  = Total Asset Value = Sum_i TA(i)*PA(i)

alpha = TAV / TCC	(currency reserve rate)

BFA(i) = buying friction = % above break even FED allows blender transaction
SFA(i) = selling friction = % above break even where FED allows blender selling


BuyPrice = PA(i) * (alpha + BF)


Assume 1coin = .1 BTC  w. 93% reserve rate and 5% buying friction
.1 BTC * (93% + 5%) = .098 BTC    delta = .002 BTC

So exchanges will net a profit each time they mint a new blender contract in this environment until either those 98% reserve contracts pull the reserve rate up to 95%, or the surplus of buyers at a 10% premium dwindles.



### Homeostasis

Community seeks to maintain liquidity against designated asset classes by maintaining 