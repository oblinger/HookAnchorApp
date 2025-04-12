

MDDB - MarkDown DataBase

This script uses a markdown template document to map between a folder of structured markdown documents and a relational table of data derived from those documents.
The script provides
- Easy methods to merge tabular data (CSV, TSV, XSLX, and MD tables) from ChatGPT or other sources into an evolving table of data

[[NJ Plan]] 


- Repeatedly import tabular data into this evolving structure.
- Create a structured template of the document data.
- Directly edit the data either in markdown or in Excel.
- Generate reports from the database
	- Tabular reports
	- Group lists
	- Tables and lists use SQL-like operators: order_by, select, where
- Programmatically derive data as a progression of derived columns
[[tg]]

Show:
- Five Interlocking Representations - folder of markdown files, excel sheet, Dict[str, Dict[str, str]]
   [MDDB dir](spot://mddbfolder)  [[6sense]]  <==>  [[_db.xlsx]]. using  [[_template]],     Inputs (.csv, .tsv, .md, .xslx).     Outputs  (SQL tables & lists)     Python Dict of Dict for scripting.
- IMPORTING
  [[NJ Queries|Queries]],   [_import dir](spot://importfolder),   [[List Missing Job|Missing Jobs]],  
- SCRIPTED OUTPUTS:
  [[Area DLP|DLP]], 
- HISTORY & SETUP:
  [MDDB dir](spot://mddbfolder)  
