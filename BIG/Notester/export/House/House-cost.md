# House.House-cost --



       <<>>




    Dave & Nina,  
     
    I started this the night we got off the phone (and didn't get to sleep 
    until 4:30).  Since then I have been so busy that I forgot about them. 
    I finshed these off this weekend, and I wanted you to have a look. 
    I think you will be interested, the main formula computes the annualized 
    cost of home ownership. 
     
    I'm guessing that Dave is the only one who cares enough to actually 
    slog though this thing in enough detail to have an opinion on whether 
    I got it right.  I would be greatful if you did this (I think it may 
    take more than one sitting).  In particular is the analysis that leads 
    to equation 8 correct (especially the tax consequences)?  Did I miss 
    any big things? 
     
    Also, I am curious what this formula says about your house ownership. 
     
     
     
     
     
     
    Dave & Nina, 
     
    Thanks for the housing info.  After we got off the phone I stayed up 
    an hour and a half twiddling equations. 
     
    I really had fun with this.  The math isn't hard, but I wasn't really 
    sure of exactly what quantity I was trying to compute.  I came up with 
    a rather bizarre way to look at a housing loan, but it seems to make 
    the Estimation of total cost easier. 
     
    The basic questions: Am I getting screwed financially by renting 
    instead of buying?  How quickly can I sell the house (if I move) and 
    not take a bath financially?  How much annual appreciation in the 
    housing market is required for home ownership to be a good investment 
    vehicle. 
     
    Cheers,  
    Dan 
     
     
    =========================== THE BASIC FORMULA ================================= 
     
    Ok, we are trying to convert a one-time house purchase into an annual 
    ``virtual-rent.''  The virtual rent is the annualized total cost 
    (lossed money) of owning a house.  This is the fraction of the $$ you 
    put into your house that is not building equity, not defraying income 
    taxes etc.  Just like real rent, 100% of your virtual rent is lost to 
    you.  Virtual Rent is a nice number: compare it to your real rent and 
    you know how much money you are saving<<>>loosing by renting.  Compare it 
    to your morgage rate and see how much of your payments is lost money. 
    If you move, it tells you how much you have to sublet for inorder to  
    break even.  You can also use it to determine under what conditions 
    you can actually make money by owning a home.  Nifty huh? 
     
     
    Here are the relevant parameters: 
     
      Variables:             Nominal Values:    Meaning: 
      V (virtual rent)                      Annualized cost of home ownership 
      H (Purchase price)                    Cost of home as a lum sum payment 
      R (Morgage rate)       6%    ~7%      Annual interest on loan 
      P (Property taxes)     1.3   ~1.5%    Annual property taxes on full value of home 
      K (Kombined rate)                     Combo of annual rates against house cost H 
      C (Closing Costs)      20K   ~5,000    = Points + Title Ins + ... 
      Y (# of years owned)          5       Total ownership period 
      D (Down Payment)             ~Px20%   Initial downpayment on house 
      E (Equity in house)          >D >>mo of $$ tied up in house 
      T (Income retain rate) 60%   67%      Money RETAINED after state & fed taxes 
      G (Gains retain rate)  72%            Money RETAINED after cap gains = 100% - 28% 
      A (Appreciation)       =I             Adjusted Annual appreciation rate on house 
      I (Inflation)          ~4%            Annual inflation 
      M (Market Return)      8%    ~9.5%    Return on money invested in market 
     
     
    Here is house buying as Dan understands it: 
     
    1) Any money not invested in your home would have been invested in the market. 
     
    2) Lots of one time up front costs occur these are all bundled into C. 
     
    3) Loan default insurance is not necessary if more than 20% is put down 
       initially, if not the cost of this insurance is added to C. 
     
    4) Home morgage payments are Federal and state tax deductable.       ???
     
    5) Capital gains taxes do not apply to a house when sold. 
     
    6) Both of these taxes do apply to all market investments. 
     
    7) Property taxes are a fixed percentage of the ``assessed value'' of 
       your home, and the ``assessed value'' is a fixed percentage of your 
       house's closing cost.  Both of these are multiplied together to 
       arrive at, P, your property taxes as a percent of the full value of your 
       home, H. 
     
     
     
     
    First assume that we are buying a house with no closing costs no 
    downpayments and no appreciation<<>>depreciation.  Further assume an 
    infinite payback period (only the interest is paid, the principle is 
    never paid).  In this simple case the virtual rent, V, is the House 
    purchase price, H, multiplied by the sum of the morgage interest rate 
    and the property tax rate.  (Note: P is the effective tax rate. 
    I.e. the product of the assessment rate with the stated property tax 
    rate.)  The first two equations show this: 
     
      (1) K = R + P 
      (2) V = H * K 
     
    Closing costs (discounting the interest on them) can be paid for on an 
    annualized basis by just splitting them over the total years owned, 
    this adds a C<<>>Y term to equation (1): 
     
      (3)  V = H*K + C<<>>Y 
     
    The annual appreciation (depreciation) on the house is multiplied by 
    the full value of the house so it adds another (H*A) term to equation 
    three.  We can capture this by simply factoring the ``A'' term into 
    our K-ombined rate <<>> 
     
      (4)  K = R + P - A 
     
    Now for the bizzare twist.  These equations above seem to only hold 
    for the first year of the loan, after than the principle remaining is 
    lower and so interest paid etc. all change.  Let us imagine that we 
    have a second ``principle'' investment account that pays an annual 
    rate of return exactly equal to our Morgage Rate, R.  Instead of 
    paying off any principle on our loan we just put money into this 
    account.  Think about it for a moment, every dollar put the ``principle'' 
    account generates a return exactly equal to the lowering in interest 
    rates we would have had if we put it into pay off our loan.  Using the 
    ``principle'' account allows us to tease apart several simultaneous 
    effects that change over time.  By using this account we imagine that 
    we are not actually paying off our loan at all, instead we are filling 
    our ``principle'' account. 
     
    Every dollar that goes into the imanginary account is not being 
    invested in the real stock market (and we know that that's where 
    everyone puts every spare dollars).  So our total opportunity cost 
    associated with our house is the product of our current equity in the 
    house multiplied by the difference between the market rate of return, 
    M, and our imagnary accounts return, R.  Equations five and six show 
    this addition to equations three and four: 
     
      (5)    O = M - R 
      (6)    V = H*K + C<<>>M + O*E 
             K = R+P-A  
     
     
    Now lets add tax considerations into our increasingly complex little world. 
     
    Assuming that our retaining rate after taxes (T) is 67% then we see 
    that for every $100 spent of before tax money it costs us only $67 
    dollars of after tax money.  Thus a pre-tax rate of x% is equivelant 
    an adjusted post-tax rate of x*T% 
     
    Consider how this adjustment affects each part of equation 6. 
    - dollars paid for the moragage needs to be adjusted, so R should be adjusted. 
    - assuming property taxes are tax deductable P needs to be adjusted. 
    - gains in house value, A, is not affected by income taxes 
      (capital gains tax handled later) 
    - assuming that closing costs are tax deductable so C<<>>M needs to be adjusted. 
    - the opportunity cost (O) is measure in after tax dollars, but money 
      put into the ``principle'' account at rate R is pretax, so we need 
      to divide R by T to convert to post-tax dollars 
     
    Thus we rewrite the definition of K to include these changes, and we 
    include a T term in with the closing cost term in order to adjust it 
    bto after taxes money: 
     
      (7)  V = H*K + C*T<<>>M + O*E 
           K = (R+P)*T - A 
           O = M - R<<>>T 
     
      R=6%, P=1%, T=60%, A=3%  ==>  K=-.6%   (assuming 2.6% maintainance on house) then 2% 

      Variables:             Nominal Values:    Meaning:       (COPIED FROM ABOVE)
      V (virtual rent)                      Annualized cost of home ownership 
      H (Purchase price)                    Cost of home as a lum sum payment 
      R (Morgage rate)       6%    ~7%      Annual interest on loan 
      P (Property taxes)     1.3   ~1.5%    Annual property taxes on full value of home 
      K (Kombined rate)                     Combo of annual rates against house cost H 
      C (Closing Costs)      20K   ~5,000    = Points + Title Ins + ... 
      Y (# of years owned)          5       Total ownership period 
      D (Down Payment)             ~Px20%   Initial downpayment on house 
      E (Equity in house)          >D >>mo of $$ tied up in house 
      T (Income retain rate) 60%   67%      Money RETAINED after state & fed taxes 
      G (Gains retain rate)  72%            Money RETAINED after cap gains = 100% - 28% 
      A (Appreciation)       =I             Adjusted Annual appreciation rate on house 
      I (Inflation)          ~4%            Annual inflation 
      M (Market Return)      8%    ~9.5%    Return on money invested in market 
     

     
    The second tax consideration is the lack of capital gains taxes from 
    any appreciation in the house's value.  This does not directly affect 
    your virtual rent since no taxes are paid.  Its effects are indirect; 
    it effects the opportunity cost -- what you could have made on your 
    money if you had not bought your house.  Any money invested in the  
    market at rate M will have a capital gains tax, G, applied.  If the 
    the capital gains are realized at the end of each year then the  
    adjust rate of return will be M*G.  This give us a new opportunity cost 
    equation:  O = M*G - R<<>>T.  Putting these all together we get: 
     
      (8) V = H*K + C*T<<>>M + O*E 
          K = (R+P)*T - A 
          O = M*G - R<<>>T 
     
    This is the annualized cost, V, of owning a home valued at H. 
    We can invert this equation to solve for H in terms of V as follows: 
     
      (9)    H = (V - C*T<<>>M - O*E) <<>> K 
     
    Use Equation (8) to compute virtual rent given a fixed housing cost. 
    Use Equation (9) to compute the housing equivelant for a particular 
    rental cost. 
     
    =============================== AN EXAMPLE =================================== 
    Lets run some concrete numbers for my situation.   
     
    V = 9900 = my real rent.   (My current rent is $825<<>>mo  or $9900<<>>yr) 
     
    G = 72%           Cap gains retain rate 
    M = 14.5%         Investment Mkt return rate 
    R = 7%            Morgage Rate 
    O = M*G - R<<>>T=0%  Opportunity cost rate for dollars invested in house. 
    E = 35K           House equity.  This will change continuously over time 
    O*E = 0           (This means I am indifferent to house vs. mkt investment 
                       if I can get 14.5% from market.  I just want this term 
                       to drop out for the moment.) 
     
    R = 7%            Morgage rate 
    P = 1.5%          Property tax 
    T = 67%           Tax retaining rate 
    A = 3.5% = I      House annual appreciation (I assumed it follows inflation) 
    K = (7%+1.5%)*67% - 3.5%*72% = 3.18% 
     
    C = 5K 
    C*T = 3350        Tax adjusted closing cost 
    M = 1, 2, 3, 5    Lets look at the virtual rent increase for (1, 2, and 5) years of ownership 
    C*T<<>>M = $3350, $1675, $1117, $670 
     
    H = (V    - C*T<<>>M - O*E) <<>> K 
    H = (9900 -  670) <<>> 3.18%  = 290K   (Holding 5yr)  (Using equation #6) 
    H = (9900 - 1117) <<>> 3.18%  = 276K   (Holding 3yr) 
    H = (9900 - 1675) <<>> 3.18%  = 257K   (Holding 2yr) 
    H = (9900 - 3350) <<>> 3.18%  = 206K   (Holding 1yr) 
     
    So if I held it for 3 years I could buy a 276K house w. a virtual rent 
    equal to my current rent. 
     
    --- 
    Going the other way, a house that cost 450K held for 5yrs would have a 
    virtual rent of $1250 per month: 
     
    V = H*K             + C*T/M + O*E 
    V = 450,000 * 3.18% + 670   + 0    = 15000/yr  or  1,250/mo 
     
     
    Notice that in many cases the most important term is the H*K term. 
    So as a rule of thumb you can use the K-factor which is defined as: 
      K-factor = K * 1000/12     or fully expanded as: 
      K-factor = ( (R+P)*T - A ) * 1000/12 
     
    Just multiply the cost of your house (in thousands) by this number and  
    the result is your MONTHLY virtual rent disregarding closing costs and 
    opportunity cost.  Our K-factor from above is: 3.18% * 1000/12 = 2.65 
     
    So a 450K house will has a virtual rent of 450 * 2.65 = $1193/mo.  (Pretty close to 
    the more accurate 1,250/mo estimate above) 
     
    Its nice to just know your personal 'K-factor', with it you can quickly convert 
    between house cost and monthly virtual-rent (lost $$). 
     
    NOTE: remember T is your income retention rate ( 100% - FED&STATE TAXES ) 
     
     
    =============================================================================== 
    A CONFUSING DISCUSSION ABOUT THE OPPORTUNITY COST OF THE $$ PUT INTO YOUR HOUSE 
     
    Every dollar put into your house could have been earning you billions 
    (or not) in the market.  O, the opportunity cost rate, is the 
    annualized difference between investing in the market and investing in 
    your house. 
     
    Now these are not really apples and apples but we are going to compute 
    their difference anyway.  If we don't we can't compute an annulaized 
    conversion and thus we cannot translate a one time housing investments 
    into a virtual rent (which is the whole point of this exercise). 
     
    [[Side note:  is *G and <<>> counting taxes twice?? did I take both forms 
      of tax benefits into account correctly??  My head just hurts thinking  
      about this.]] 
     
    Two considerations:   
    (1)  The rest of the equations above take into accout the 
    cost of the loan 'H', so for this part of the equation we take as a 
    given that this loan exists.  Thus paying off part of the loan (as 
    opposed to investing in the market) should be considered a riskless 
    investment whose rate equals your morgage rate.  Neglecting the 
    possibility of going bankrunpt you <<>> payback your loan, thus 
    every dollar you invest in your loan has a 100% certain chance of 
    increasing your future net worth by the interest rate on your loan. 
    The market rate for almost all investment strategies includes some 
    amount of risk.  If aversion to risk is important, then this 
    difference needs to be considered by lowering our assumed market return. 
     
    (2)  Capital gains in your house are not taxed, whereas capital gains tax 
    is assessed on money invested in the market.  We assumed that gains in our 
    market position are realized annually.  This made our calculations a  
    whole lot easier.  (My head just hurts when I think about the alternative). 
    Still its not an apples to apples comparison.  We are assuming that  
    moneys invested in our house stay put indefinitely, but moneys put into 
    the market are moved annually (a very unfavorable strategy). 
    This means that we are underestimating the cost of the house because we 
    are overemphasizing the taxing on market investments. 
    I am not too worried about this miss-estimation since most market investment  
    strategies (ie. most mutual funds) do result in turnovers every few years. 
     
     
    OTHER CONSIDERATIONS ABOUT THE ASSUMPTIONS MADE 
     
    (3) We could have represented the E term as a percent of the total 
    housing cost.  This change would have altered the shape of the 
    equations above (the opportunity cost term would have become part of 
    the K (kombined rate) term.  Which way is right?  Well when imagining 
    alternative realities is it best to fix the total amount that saved as 
    house equity after a fixed amount of time (say 10 years) or is it best 
    to assume that a fix percent of the house will be payed off after a 
    fixed amount of time (say 35%).  Since we only have a fixed but 
    arbitrary ( :] ) income rate perhaps the absolute total amount is 
    best.  On the other hand many house loans are 30 year loans so 
    regardless of your income you must have a certain percent of the house 
    payed after a particular time period. 
     
     
     
    ==============  ESTIMATING YOUR HOUSE'S EXPECTED APPRECIATION  ================ 
     
    I am pleased that all the terms that have a large impact on equation #8 
    are terms that can be known with high precision--all terms but one. 
    The appreciation term A can have a tremendous impact on the annualized  
    cost of home ownership and pick it is like taking a shot in the dark. 
    The equation #9 (H as a function of V) becomes meaningless as large values 
    of A drive K to zero.  Thus we want to make as good a shot in the dark as 
    possible. 
     
    It seems there are two considerations: market trend, and the depreciation 
    trend.  The price of a new three bed room house may increase by 
    10% per year for the next ten years, but your house wont be a <<>> 
    three bedroom at the end of ten years. 
     
    So select a fixed type of house: e.g. 3-bedroom, 10000sq foot, two story. 
    Estimate the annual price change over the last n years. 
    Then look at the housing costs in the current year for houses of 
    the same type that only vary by their age.  Estimate the annual  
    depreciation due to aging. 
     
    Subtract these two numbers.  This is your appreciation rate A. 
     
    In order to get a accurate appreciation rate you may want to include 
    balanced sets of market crashes and upswings in your estimation of 
    annual appreciation. 
     
     
     
    ================== SO CAN I MAKE MONEY BY OWNING A HOUSE? ===================== 
     
    Here is equation 8 reprinted from above. 
     
      (8) V = H*K + C*T/M + O*E 
          K = (R+P)*T - A 
          O = M*G - R/T 
     
    Well the closing costs are simply a loss, but a small one in the grand 
    scheme of things so lets just ignore the C*T/M term.  The opportunity 
    cost may be small depending on your assumptions about the market, but 
    unless your a real pesimist about the market they wont gain you any money, 
    so lets ignore the O*E term.  Now they virtual rent will go negative 
    (making money) only when K goes negative.  This can only happen if 
    your house appreciation out weights your morgage and taxes.  Let set 
    K=0 and solve for <<>> 
     
      (10) A = (R+P)*T 
     
    Using the numbers from above we get: 
     
      A = (7%+1.5%)*67% = 5.7% 
     
    If your annualized house appreciation approaches this value, then 
    the first term of equation 8 drops out, and if your opportunity cost 
    rate, O, is near zero then money invested in your house earns the same 
    as the market, so under these circumstances you are making as much money 
    as you would in the market alone. 
     
     
    ======== WEIRD THINGS THAT FOLLOW FROM THESE EQUATIONS ======= 
     
    Here are some amazing consequences of these equations.  Some of them 
    are so amazing that it makes me nervous that the equations are wrong! 
     
     
     
    (1) In many cases the opportunity cost rate will be small (or even 
    negative), in these cases investing in a home is just as good as 
    investing in the market.  As your appreciation rate, A, approaches the 
    value shown in equation 10, the K term approaches zero.  This means 
    that the cost of owning such a house is ZERO regardless of how much 
    the house costs!!  Go big!  Really BIG!  Why not all you gotta do is 
    cover closing costs! 
     
    Well ok, as we noted above this equation is very sensitive to changes 
    in A as K tends toward zero, so maybe market downturn would ruin you. 
    Besides, an increase of 6%/yr is really steep even in markets where 
    housing is going up and up, simply because the foundation in your 
    house is sagging down and down and your house is simply not going to 
    keep up with new housing prices.  Still, kinda makes you think 
    BIG-HOUSE-FOR-FREE! 
     
     
    (2) Not so fast, there is no continuum here.  If K is positive, then 
    it is always more economical to own the cheapest house possible.  If 
    K is negative then the most expensive is best. 
     
    The optimal housing price does <<>> increase at all until the condition 
    in Equation 10 is met, then the optimal price goes to infinity. 
     
    (Of couse this neglects the coolness value of owning a BIG BIG home) 
     
     
    (3) The more of your prinicple you have paid off, the less you are 
    paying in interest, thus the annual cost of owning your home would go 
    down. 
     
    This is any obvious statement, and it is WRONG!!! 
     
    In some cases the opportunity cost rate, O, is slightly positive, so 
    according to the equations above, the more of your house you have 
    paid off the <<>> it is costing you own that house!!!  The 
    counteracting factor is the opportunity cost of having all that money 
    tied up in your home rather than in the stock market. 
     
    To understand how these two effect balance each other we did our nifty 
    split of our house into a loan for the full amount of the closing 
    cost, and an ``principle'' investment account whose return is the morgage 
    rate R.  Opportunity cost is the difference between the market and R. 
    If that difference is positive then savings from reduction in 
    principle is more than offset by the loss of income because that money 
    is not in the market. 
     
    The more money paid back the more you loose. 
     
     
    (4) Your house doesn't stop ``costing'' money even after the loan is 
    paid off.   
     
    Again its the opportunity cost term that nails you.  You are loosing 
    H * M in money that could have been earning returns in the market. 
    If your house is appreciating like as shown in Equation 10 you are 
    safe, otherwise you are loosing by owning even after the loan is paid off. 
     
     
     
    (5) Amazing facts #3 and #4 taken together imply that it is 
    theoretically possible for BUYING big homes is a big win and at the 
    same time actually OWNING them is a big loss!! 
     
    It is possible for Jim to buy Luxury Suite A in loads-o-dough towers, 
    and make fists of money while paying incredible loan payments. 
    Meanwhile Jack who lives in the identical Suite B across the hall is 
    loosing money by the buckets because he is not making loan payments 
    even though both have titles identical apartments for the same time 
    period in the same housing market!! 
     
     
     
    I should have been an accountant for the mob.  I think I could justify 
    anything convincingly!  My only problem is that I don't work for the 
    mob; I can't afford to loose buckets of money, and I would like to 
    actually make buying decisions some day informed, at least in part, by 
    the numbers above.  Given this, I really would prefer that the numbers 
    I use have some vague connection with reality.  Do they?  What do you 
    think? 
     
     
     
     
     
