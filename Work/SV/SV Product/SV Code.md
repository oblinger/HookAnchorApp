

Docs for STATE of the CV pipeline:

[https://bitbucket.org/SVEngineering/algorithms2/raw/f1cf89650cd08357082f1820b15cc9a95e016060/src/synch/sctemplate.py](https://bitbucket.org/SVEngineering/algorithms2/raw/f1cf89650cd08357082f1820b15cc9a95e016060/src/synch/sctemplate.py)




#  eval EXPR             -- Adds a new row with the result of the expression EXPR.
#  total TEXT            -- Adds a new row with the total of the column TEXT.
#  #RULESET:ITEM     -- Transaction's item/bucket value
# - Extend ValueExpr classes to include nary + and * and binary - and / operations.


#  _______________
#  RULE PROCESSING
#  eval EXPR             -- Adds a new row with the result of the expression EXPR.
#  total TEXT            -- Adds a new row with the total of the column TEXT.

# instructions:
# - add statement classes for output section and text name and eval statements.
# - Extend ValueExpr classes to include nary + and * and binary - and / operations.
#
# - extend program parsing to include parsing of output statement sections,
#   and program evaluation to include evaluation of output sections which will write
#   out the specified output csv in the working directory.  The colums will come from
#   the pivot tables columns and the rows will come from the statments in the output
#   section.

"""
- A budgeting program is a text file with both rulesets and output sections. 
- Rulesets contain rules that are sequentially applied to each transaction until stop/item.
- Output sections define output CSV files with pivot table columns and statement-defined rows.

Commands:

Output Section Commands:
  output NAME           -- Starts a new output section named <name>
  text TEXT            -- Adds a header/comment row with TEXT in first column
  name RULESET:NAME    -- Adds row with named item/bucket values
  eval EXPR           -- Adds row with expression result 
  total TEXT          -- Adds row with column TEXT's total

RuleSet Commands:
  ruleset NAME         -- Start a ruleset named <name>
  stop                -- Stop processing rules for transaction
  bucket NAME         -- Assign transaction to budget bucket
  item NAME          -- Assign transaction to budget item
  set FIELD VAL      -- Set transaction field value
  if COND            -- Execute following until elif/else if true
  elif COND          -- Check condition if prior if failed
  else              -- Execute if all prior if/elif failed

Expressions:
  VALUE              -- String or quoted "string"
  $FIELD            -- Transaction field value
  #RULESET:ITEM     -- Transaction's item/bucket value
  VAL1 + VAL2       -- Sum values
  VAL1 * VAL2       -- Multiply values 
  VAL1 - VAL2       -- Subtract values
  VAL1 / VAL2       -- Divide values
  VAL1 < VAL2       -- Less than comparison
  VAL1 <= VAL2      -- Less than or equal
  VAL1 == VAL2      -- Equal comparison
  VAL1 != VAL2      -- Not equal comparison
  not VAL           -- Logical NOT
  VAL1 and VAL2     -- Logical AND
  VAL1 or VAL2      -- Logical OR
"""
