# Budget.Fields --

    SPREADSHEET FIELDS 
     
    08  STUB-Salary             'Salary' 
    09  STUB-Taxes-Total        Below 'Total' to left of 'Taxes' 
    10  =                       B8-B9  Base pay 
    11  STUB-TDSP               'TDSP' 
    12  STUB-Stock              'Stock' 
    13  STUB-Other-Deductions   Below 'Other*' to left of 'Deductions' 
    14  STUB-NetPay             'Net Pay' 
    15  =B8-B9-B11-B12-B13-B14  Stub error (Should be exactly zero) 
    16  =B14-??                 Stub<<>>Checking Error (Roughly zero) 
     
    18  SPECIAL-401K            Matching 401K rate(this is really an over estimate) 
    19  SPECIAL-Stock           Stock discount 
    20  SPECIAL-Var-Pay         Variable Pay Rate 
    21  SPECIAL-Options         Options 
    22  QUICKEN-Inv-Perf        Investment Performance ???? 
    23  =B18+B19+B20+B21+B22    Sum of all annualized <<>> virtual benefits 
     
    29                          CAT-Divisor is used to scale all CAT entries 
    30  CAT-Pay                 IBM Pay (not Stock or Vpay) 
    31  =B32-B0                 CAT-OtherInc  (exact calc) 
    32  CAT-1-Income            Total inflows. (exact calc)  (**1) 
    33  CAT-7-Investment        Net flow to Schwab (**1) 
    34  CAT-2-Daily             Daily discretionary spending 
    35  CAT-3-Durables          Spending on durable goods 
    36  CAT-4-Fixed             Non-discretionary, fixed spending 
    37  CAT-5-Sporadic          Non-discretionary one of a kind items 
    38  CAT-6-Zero              Expected to be small 
    39 ZZZZ= Sum(36:40)             Sum of On Budget spending 
    40 ZZZZ= B41+B35                Sum outflows  (including investments) 
    41 CAT-Overall              Quicken sum of all income & expense 
    42 ZZZZ= B42-B32                Actual savings rate 
     
     
    <<>> 
    SOURCES 
      STUB      = From my IBM pay stub 
      SPECIAL   = From IBM policies 
      CAT       = From quicken category total  (Use Dan's P&L) 
      CAT-TOTAL = From quicken category total that include sub categories 
      CAT-#     = From quicken group total 
     
     
    **1 -- ANNUALIZED CHARGES 
    - IBM Stock and Var pay are annualized. 
    - When payment are made they come into to checking or Schwab from the 
      'annualized' investment account.  This account is listed under group #7 
      so payments into checking followed by payment out to schwab does 
      not have a net affect on level of investment. 
    - Over a long time window we expect STUB-Stock, SPECIAL-Stock plus 
      SPECIAL-Var-Pay to roughly equal Annualize output 
     
     
    NOTES 
     
    SPECIAL (comes from IBM policies) 
    - Matching 401K 
      Virtual Income from IBM's matching 401K matching funds 
      3% is assuming that I keep my rate above 6% but under 10% total for the year. 
    - Stock Discount (85 cents on the dollar) 
      Virtual Income from IBM's stock discount 
    - Var Pay 
      Virtual Income from Var pay Averaged monthly 
    - ROI Return on Investments 
      Quicken: Investment Performance 
      (Schwab interest) 
     
     
    BUDGET (comes from Quicken 'Dans Qtr Budget') 
    - Annual Stk/Var pay 
      These annual payments are factored out of my pay since they are 
      already counted in my virtual income. 
    - Pay 
      Actual pay for the period.  This is split into: NetPay, Annual & Other 
    - Total Income  (BUDGET: Income category.  Does not include interest?) 
    - BUDGET:Investments  (flow to Schwab. But not interest) 
    -  
     
     
     
    FIXES 
    - Add 'buisness' category for excess IBM payments and for bus expenses 
    - split out Differed income (that goes to TDSP and options) 
      (NetPay is copied from above.  Other pay is the difference) 
    -  
     
     
    Q 
    - Stock/var pay (Obtained gotten from quicken) 
      Special Category for annual Pay already covered in virtual calcs 
     
    - Unqualified Savings are ignored savings cause they are counted elsewhere. 
     
    BUDGET 
     
    +add pay annual 
     
