.[[FIN Budget]].
  [[Tracking]]
 
- [[2025 Budget]]: 
- [[FIN Flows]]: coarse-grained budget
- [[FIN Categories]]: Budget categories

BUDGET
- [[2025 Sporadic Spending.xlsx]]: 


## INFO
### Adjustments

| Date       | Amount      | Investment Transfers - In/Out of Grand Central |     |
| ---------- | ----------- | ---------------------------------------------- | --- |
| 2021-06-21 | -100,000.00 | Bought space-X                                 |     |
| 2021-06-25 | + 46,350.00 | Sold 1/2 space-X back                          |     |
| 2021-07-16 | - 65,431.67 | House Refi                                     |     |
|            |             |                                                |     |
|            |             |                                                |     |

| Date       | Amount        | Spending Adjustments -- Spending occurring outside of JOINT account                |
| ---------- | ------------- | ---------------------------------------------------------------------------------- |
| 2022-03-22 | - 9,827.23 J* | (House Tax on 3/21/22 for $9,827.23)  * = did manual from GC to compensate 4/25    |
| 2021-11-26 | - 9,827.23 G  | (House Tax on 11/24/2021 $9,827.23) CITY CNTY OF SF CTY SF P                       |
| 2021-05-26 | -10,929.40 J* | (House Tax on 5/25/2021 $10,929.40) CITY CNTY OF SF CTY SF P  (* = see above) 4/25 |
| 2020-10-21 | - 9,894.91 G  | (House Tax on 10/19/2020 for $9,894.91) CITY CNTY OF SF CTY SF P                   |
| 2020-04-13 | - 9,532.02 G  | (House Tax on 4/10/2020 for $9,532.02)                                             |
| 2019-12-04 | - 9,532.02 G  | (House Tax on 12/03/2019 for $9,532.02)                                            |
| 2019-01-14 | - 2.756.87 J  | (House Tax on 1/11/2019 for $2,756.87) SAN FRANCISCO CA SAN FRA                    |
|            |               | (House Tax on 4/18/2019 for $2,801.87)                                             |
| 2019-06-10 | - 5,161,67 G  | (House Tax on 6/6/2019 for $5,161.67)                                              |
| 2019-10-02 | - 5,161.67 J  | (House Tax on 9/30/2019 for $5,161.67)                                             |
|            |               |                                                                                    |
|            |               |  


| Date | Irregularity |
| ---- | ------------ |
|      |              |



# LOG

### 2025-01-10  OLD MINT BUDGETING


budget.py

taxes don't come from joint
- easier to see spending
- taxes are not really spending
- house is easy to remember)

#### How To

**MINT**
- VERIFY CURRENT ON:  DanO, AmzMisc, Bills(Freedom), Groc, Sapphire, timetime
- Mint.com --> Transactions --> !export XXXX transactions (@bottom)
- $ budget.py   # historical data in ~/ob/data/budget

**SCHWAB**
- Account -> Balances ! Joint Checking      (15K monthly flow starting before 2021)


#### BUDGET CROSS CHECKS
- SIMILAR:  **GrandTotal** ~ **Joint_Out** 	GrandTotal sums many transactions,   JointOut sums credit card payments and outputs of Joint Checking
- SMALL:    Transfers 					Transactions with category 'transfer'
- SMALL:    JOINT_OTHER      			Transactions from 'Joint Checking' account that are not covered in the budget below
- EXACT: 	  CrossCheck          			These items should exactly? match ALL_CARDS section


#### BUDGET SECTIONS EXPLANATION


"""
SECTIONS

**CARVE OUTS**			= Excludes transactions from budget below

**INCOME_ALL**     		= Income from all sources

**BIG_ALL** 		
- Mortgage, School, Nanny

**ITEMS_ALL**
- Ele_Gas, Water, Data (phone&comcast)
- Auto, House(grass,lowes,...), Medical, Content(WSJ,...)
- Fees, Pet, Service (legal,checks,shipping,thermomix,life insurance...)
- Venmo, Checking
- Other (any other payments from the freedom card)

**GROC_ALL**
- Costco, Weee, Dragon, Amz Groc, Groc Other

**DISC_ALL**			= Many discretionary expenses

**BUDGET SUMMARY** 	= Adds up sections from above


**>>> JOINT CHECKING ANALYSIS <<<**
**CARVE OUTS**

**ALL_BIG**
- Mortgage, School, Gov
**ALL_CARDS**
- Spending, Util, Groc, AmzMisc, 
**ALL_CASH**
- Cash, Checking, VenmoEtc, Interest

**JOINT FLOWS**				= Flows in to and out of Joint Checking

**CROSS CHECK**  			= Should match credit cards in all cards (it is the inverse transaction)

 DATES:  Groc 4th    AmzMisc 13-15th    Sapphire 20-23th    Bills 26-1th




#### BUDGET APP
- (See Pycharm -> bin)   
- Master in budget.py





See Bills -> CashFlow for active accounts tracked in mint



- Transactions are sorted by column K (Main)

#### BUDGET.PY

| SECTIONS        | Begins          | Col       | Notes                                   |
| --------------- | --------------- | --------- | --------------------------------------- |
| TOTALS          | CREDITS_OTR ... |           | Totals In & Out                         |
| UTILITIES       | Ele_Gas ...     | Main (K)  | Utilities Breakout                      |
| CATEGORIES      | QL_In           | Main (K)  | Breakout by category and sub-categories |
| JOINT ONE LEVEL | _ Summary _     | Joint (L) | Toplevel breakdown                      |
| JOINT TWO LEVEL | OTR_CREDIT      | Joint (L) | Sub-breakdown                           |
|                 |                 |           |                                         |


### 2023-01-10  Energy Usage

PGE Ele:		$150/mo		Wastage: [Dishwasher]; Laundry
PGE Gas: 	$90/mo		
Water:			$200/mo		Showers    <---- 



### 2020-07-18 - Budget


Monthly
	6000	Morgage
	3500	Nanny
	

Mensuralized
	2000	Property Tax


6.0  Rent
2.0  Property Tax
3.0  Daily
4.0  Nanny
