# Fin.Analysis\_Net --

    _ 



      SAVINGS = INCOME - SPENDING

    - Quicken:Reports:MyReports:Dan's Net
    - Report:  Dan's Net   (Organization=CategoryGroup, Accounts=AllButInvest&Vanguard, CategoryGroups=AllButInvest)
      - Manually set 10 months
      - Income<<>>10       = 10100
      - OverallTotal<<>>10 =  4800
      - (Savings=diff)     5300


    - REASONING:
      This gives us our net income & fraction saved


    <<>>


     How the Fuck do I determine how much:   income <<>> spending <<>> savings ?


     Spending =    
        Total outflows to everything except Inv


    Proposal #1:  Use Schwab account as clearning house.
         Report=Income&Expense (for Schwab account only)  (Include all tranfers)
      Income   = Income total
      Saving   = Manually add Tfrs to/from Vanguard
      Spending = Outflows - Savings


    Proposal #2:  Use JP-Morgan account as clearing house.
        Report=CategoryGroup   (assign Vanguard to Investment Group)
        2a:       Income=IncomeGrp   Spending=ReportTotals   (exclude inventment group)
                      Savings = Delta
        2b:       Savings = Investment Group  (Does not include 
