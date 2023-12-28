
Objective:  RPT is wrapper script that unifies metric reporting performed by the CV group.  
This wrapper aims to generalize across:  (1) reports and computation types, 
(2) output formats, (3) datasets used, (4) code versions.  
Key Objectives:
- EXISTING - Ideally wraps existing command line reporting scripts with little/no changes.
- REPEATABILITY - Ensure repeatability in the face of code, dataset, and report-script changes.
- CHANGE - Unifies reporting in the face of changing code, APIs, and datasets while allowing user to control what is held constant.
The wrapper aims to 
- MERGE - Build reports across code versions, datasets, and dataset versions. 
- WRAP - Work with existing and future reporting scripts (be a least commitment wrapper)
- SCRIPT - The rpt script itself is in the /bin folder of the master branch of the XXXX repo
- SIMPLE - Concise cmdline tool with simple semantics that can be easily used as building block in other things


``dat 	                                    #  G-drive dervied data computation, caching, and versioning
``	-g  --get  SPEC [--inputs SPEC ...]     #  Computes, Caches, and Retrieves datum or data
``	-s  --set  SPEC  RELATIVE_PATH[.EXT]    #  Assigns a NEW datum or data
``	    --add  SPEC  RELATIVE_PATH[.EXT]    #  ADDS datum or data
``	    --rm   [NAME][.EXT][@VERSION]       #  Removes existing datum, data, definition, or version number
``	    --def  NAME                         #  Creates a new version
``		   -i  --inputs  SPEC1 . . .        #  Versioned, cached data used as inputs
``		   -o  --output  OUTPUT[.EXT]       #  Output file(s) to be cached as derived data
``		   -a  --as      CMDLINE            #  Command line that computes the cached data
``	       -b  --branch  GIT_BRANCH         #  Specifies the git branch used to compute the derivation
``	    --add-version    VERSION            #  Appends a new version number / name
``		   -c  --commit  FULL_COMMIT_HASH   #  Specifies repo commit for this version (defaults to current)
``	    --list [NAME][@VERSION]             #  Lists cached entries for name and/or version.  Or is lists all version num
``	    --purge SPEC                        #  Purges already deleted data
``	    --restore SPEC                      #  Restores deleted data that are not yet purged
``	
``	NAME[.EXT][@VERSION[:VERSION]]          #  <-- SPEC-FORMAT.  Indicates a data resource by logical name
``	                                        #  (It refers to the current, previous, or range of versions)


### Simple Use Case -- Managing a datafile

``	dat --add-version 2023-06-10
``	dat --set nifty-CSV-file.csv   local-CSV-file.csv
``	dat --set nifty-CSV-file.md    README.md             # Imports some docs about the nifty CSV file

``	dat --get nifty-CSV-file                             # will put a copy of CSV and docs in the current directory

``	dat --add-version 2023-06-20
``	dat --add nifty-CSV-file                             # imports a new version from the current directory
``	dat --get nifty-CSV@2023-06-10                       # gets the earlier version of the CSV
``	dat --get nifty-CSV-file                             # get the latest version into current dir
``	dat --get nifty-CSV@2023-06-10:2023-06               # extracts range of versions (and prints comma separated list of versions)


### Simple Use Case -- Running the same metric on different versions of the code base

``	dat --add-version 2023-06-10
``	dat --def pcheck --as 'pcheck $INPUT1.csv > pcheck.txt' --inputs cerebro10 
``	dat --add-version 2023-06-20         # implicitly captures current commit hash
``	dat --add-version 2023-07-05 --commit 1c15c426ddcd52b51345676676919e9bbe1e10b2
``	dat --add-version 2023-08-01

``	dat -g pcheck                         # returns the latest pcheck based on the main branch for version 2023-08-01
``	dat -g pcheck@2023-06-20              # returns pcheck computed for 6/20
``	dat -g pcheck@2023-06-20:             # returns sequence of pcheck's starting at 6/20.  Stored in pcheck_1.txt pcheck_2.txt ...



### Advanced Use Case -- Managing versioned derived data

``	dat --add-version 2023-05-30
``	dat --set A.txt -
``	This line is in A.txt
``	^D
``	dat --set B.txt -
``	This line is in B.txt
``	^D
``	dat --def AB --input A B --as 'cat $INPUT1.txt $INPUT2.txt > AB.txt'         # This defines AB.txt as the concat of its inputs
``	
``	dat --add-version 2023-06-10
``	dat --overwrite B.txt -
``	This is the updated content for B.txt
``	^D
``	
``	dat --get AB.txt -
``	This line is in A.txt
``	This is the updated content for B.txt
``	
``	dat --get AB.txt@2023-05-30 - 
``	This line is in A.txt
``	This line is in B.txt


``	dat --set C.txt -
``	This line is in C.txt
``	^D
``	dat --def ABC --input AB C --as 'cat $INPUT1.txt $INPUT2.txt > ABC.txt'         # This defines AB.txt as the concat of A and B
``	dat --get ABC.txt -
``	This line is in A.txt
``	This is the updated content for B.txt
``	
``	dat --get ABC.txt@2023-05-30 - 
``	This line is in A.txt
``	This line is in B.txt
``	This line is in C.txt


### Advanced Use Case -- 



### Examples

``	dat --add-version 2023-06-05
``	dat --set cerebro10
``	dat --def pcheck -i cerebro10 --as 'pcheck $INPUT1 > pcheck.txt ; echo our standard check for $VERSION1 > pcheck.md'
``	dat --add-version baller-is-live
``	dat --add-version 2023-06-20
``	dat --add cerebro10                 # adds new version 
``	dat --add-version 2023-07-14
``
``	dat --get pcheck@baller-is-live -

``	dat --get pcheck@baller-is-live --input cerebro10@2023-07-14
``	dat --add cerebro10      # Generates error since this has already been implicitly added by prior computation
``	
``	dat 


IMPL-ADD
- Add DB file paths to the local purge file
- Add files to the local DB and push each one

IMPLEMENTATION
- db_add(name, ext, version, path)
	- 
- parse_spec -> (name ext ver1 ver2)


DB_ADD(path, spec, calc)
- Check if existing current or previous version is conflicting and error if so
- Create unique UUID
- Add filename to local purge list
- Push


dat --def pcheck-cerebro10 -i cerebro10 --as cat cerebro10.txt 


rep <report_name>
  -d  --dataset   dataset
  -c  --code      commit          # Specifies git version for the code used to run test
  
  -s   --save
  -o   --output   pathname



DATASET MANAGEMENT
- Use S3 buckets.
- datasets.txt on the master branch in repo contains the full list of buckets maintained by our team.
  
- We never change contents of a bucket once it is assigned. 
- A single name is never assigned to more than one bucket
- The bucket that a name is maps to may change over time
- 
 




DATA.TXT
- A YAML file that initially only uses the '-' list item entry.  Entries:
>
	- version  	{VER_NUM}            	-- monotonic version number at top of file for code and data (begins w. v)
	- data     	{URL}                    	-- anonymous bucket used in a dataset
	- named  	{NAME} {URL}        	-- named data bucket
    - history 	{VER_NUM} -d {DATE} -n {NAME}


DATA TYPES
- VERNUM			Alpha numeric beginning with 'v' and potentially have '.' separators
- DATE				YYYY.MM.DD  with options fields:  YYYY.MM.DD.HH.MM.SS.MICRO.NANO
- NAME				Valid python identifier
- URL				Valid URL


OUTPUT TYPES
- .pdf		
- .xlsx		
- .cube??
- ???				pickle file
- .points 			points format  (list of data point maps, with special meta entry)
- .zip				zip file with potentially structured data outputs


DERIVED DATA
- Index via the 'data-derived.txt' in XXXX Repo
- Stored in buckets.
- Cached data is *ALWAYS* computed programmatically using a freshly cloned repo into a ?docker?.
- Note: reports themselves are a kind of derived data and can be cached in the same manner.
- NAMING CONVENTION:  {NAME} _ {VERNUM} . {OUTPUT TYPE}


