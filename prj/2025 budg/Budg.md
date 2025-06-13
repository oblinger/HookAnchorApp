.[[Budg]].
  , [Budg Note](spot://budgnote), 



**Objective**: Script to process and display my financial transactions.

**Usage**:  (See [[FIN Track Transactions]] or [[FIN Tracking]])



## Usage Steps
- ROCKET - Login -> !More Transactions -> !Export  // WEF !Download File
- SCHWAB - Login  
- $ budg.py
- NOTE:  To change budg.py parameters, first run it.  then edit '1_budget_script.txt' and rerun


|             | .                                               |
| ----------- | ----------------------------------------------- |
| Description | From Source CSV                                 |
| Category    | From Source (Rocket Money or Brokerage Company) |
| BgtBucket   | Budget Bucket (By Report Name)                  |
| BgtItem     | Budget Items (by report name)                   |
| AcctType    | From Source                                     |
| AcctName    | Rocket "Account Name"                           |
| InstName    | Rocket "Institution Name"                       |

```
  
class TF(Enum):   # Transaction Fields  
    SOURCE = "Source"           # REQ source of the transaction  
    DATE = "Date"               # REQ transaction date  
    NAME = "Name"               # REQ merchant name (from source)  
    AMOUNT = "Amount"           # REQ transaction amount  
    DESCRIPTION = "Description" # REQ transaction desc from Merchant  
    CATEGORY = "Category"       # REQ transaction category (from brokerage)  
    BGT_BUCKET = "BgtBucket"    # Budget buckets (by report name)  
    BGT_ITEM = "BgtItem"        # Budget items (by report name)  
  
    ACTION = "Action"           # Matches the CSV header "Action"  
    SYMBOL = "Symbol"           # Matches the CSV header "Symbol"  
    QUANTITY = "Quantity"       # Matches the CSV header "Quantity"  
    PRICE = "Price"             # Matches the CSV header "Price"  
    FEES = "Fees"               # Schwab ""Fees & Comm"  
    ORIG_DATE = "OrigDate"      # Rocket "Original Date"  
    ACCT_TYPE = "AcctType"      # Matches the CSV header "Account Type"  
    ACCT_NAME = "AcctName"      # Rocket "Account Name"  
    ACCT_NUMBER = "AcctNumber"  # Rocket "Account Number"  
    INST_NAME = "InstName"      # Rocket "Institution Name"  
  
    CUST_NAME = "CustName"      # Rocket "Custom Name"  
    NOTE = "Note"               # Matches the CSV header "Note"  
    IGNORED_FROM = "IgnoredFrom" # Matches the CSV header "Ignored From"  
    TAX_DEDUCT = "TaxDeduct"    # Schwab "Tax Deductible"  
  
SCHWAB_ACTION = 'Action'  # Mapped to account_type  
SCHWAB_SYMBOL = 'Symbol'  # Mapped to account_number

```



