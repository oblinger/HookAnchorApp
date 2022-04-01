# Retire.Planning-data -- Data for projecting retirement income

    - Monthly savings for last year 
    - Projected monthly savings (today $$ or constant) 
     
     
     
    DATA (1997) 
     
    ?=should-be-verified,  u=unknown-source, m=mom, q=from-quicken, f=fact 
     
    1   PERSONAL 
    1.1 Birthday Dates     f   5/16/1938;  11/1/1942 
    2   INCOME 
    2.1 Income             m?  40,111; 29,900   (1996, Follows inflation) 
    3   TAXES 
    3.1 State;Fed+St tax   q   17%  (?? From Quicken) 
    3.2 Inflation          q   3%   (a little optomistic) 
    4   SAVINGS 
    4.1 Tax Deferred Sav   m?  ;  
    4.2 Taxable Savings    m?  99,000 
    4.3 Additional $/yr    m?  14,000 
    5   EXPENSES 
    5.1 Liv Exp Today      q   Use conservative est. (missing stuff) 
    5.2                    m?  90% of current 
    6   RETIRE BENEFITS 
    6.1 Pension            ?   10,618  (dad@65, today$, COLA=0) 
    7   RETURN 
    7.1 Rate of return         8% 
     
     
     
     
    - Age                 Fact  Dad 59 (1938),  Mom 55 (1942) 
    - Retire              Par   Dad 65          Mom 65 
    - Retire time         Me     30yrs 
    - Inflation           Gen       4% 
    - Investment          Gen       8% 
    - Tax Bracket         Me       28% 
    - Gross Inc           ??             40,111 (dad) 29,900 (mom)  1996 
    - Retire Inc          Me       75%      of 1996 
    - Social Sec          ??    17,856  <-- Dan check 
    - Retire Benefits     ??    10,618  <-- Dan check 
    - Current Deferred    ??    35,000  <-- Mom check 
    - Current Taxable     ??    35,000  <-- Mom 
    - IRA save      <<>>            0 
    - Deferred save <<>> xx     5,000  2000dad + 3,000 (COULD USE 15%mom) 
    - Taxable  save <<>> xx    10,000  <-- Mom 
     
    - Dave Payback        ??    35,000  (they pay 287/mo for 25K) 
    - Mom's extra $$      M?    61,689  <-- Salary - social security 
     
     
    Tax Deferred (1996, in K)  
      5.46  First Investors (dad)  linda 
      6.34  First Investors (mom)  linda 
      5.62  American Funds (mom) linda 
      5.62? American funds (dad) linda 
      ? other Linda stuff 
      2.9   Schwab (mom) 
      3.17  Idex (mom) 
      3.17  Idex (dad) 
      1.55  Motorist 
      1.84  Motorist (dad) 
     
     
    6/97 Investments: 
     51K   Linda 
     25.4  Gradison 
     10    P&G 
     10    Charles Schwab 
       .5  AT&T 
      3    Bell South 
    --- 
     99.9K  Total 
     
     
     
    COMPUTING THIS MANUALLY 
    1) Current $$, yr-sav, --> (taxable & deferred @ dad 65) 
    2) Transition_yr_deficit = Dad_salary - Dad_pension - Dad_social_sec - yr_sav 
    3) Savings@d65, tr_yr_deficit --> Savings@m65 
    4)  
     
    Equations 
    - Compounding: Init, Delta/mo, time, Final,  
        Interest rate, tax on output, tax on increase 
     
    Ways to figure 
    - Vangard: Mom adjust.  Don't know what it is doing. 
    - EMB: too simple 
     
     
    CALCULATION 
     
    CurrentInc   <-- MomSal, DadSal, Payments(house, retire) 
    RetireInc    <-- CurrentInc, #yrs, rate 
    Gap          <-- Retire, SS mom 
     
    TaxableTotal <-- Taxable/yr, #yrs 
    DeferedTotal <-- Defered/yr, #yrs 
     
     
     
    PER YEAR NUMBERS (* = inflations adjusted) 
    Income <-- CurrentInc* 
    Gap    <-- Income, SS-mom*, SS-dad*, Pension 
     
    TaxableSavings <-- TaxableSavings, Gap, Interest(tax it) 
     
    DeferedSavings <-- DeferedSavings, Gap(add-tax), Interest 
     
    DaveAdjust 
     
    Income, TaxSav, DefSav, SS-mom, SS-dad,  
     
     
     
    -------------------- 
    Linda's Investments  11/14/96 
     
    7463  New Perspective          Agg? 
    6132  Capital World            Growth/Inc 
     
          IRA Mom 
    7204  Growth Fund of Am        Growth? 
    5979  Capital Builder          Gr/Inc? 
    6481  (move 2 growth) 
     
          IRA Dad 
    7536  Investment Co. of Am     ???? 
    5124  Utility Fund             Inc? 
    5588  (move 2 Inv Co) 
     
    51,057 Total
