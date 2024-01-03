TODO:
- Dict Docs
- command line

- [LCM-037](https://docs.google.com/document/d/1Q70YbaXN9YVHPQnrY7mmWViaquSdoXcVngBAcw2RgQs/edit#heading=h.e3wwovqopszq): 
- [[2023 Least Commitment Model For CV Pipe]]
- [Least Commit v2](https://docs.google.com/document/d/1kxGMrmPOr9S6whRnV0LI0UpWGeyBzTn8WGDq7ITM4Sc/edit): grz


# LOG
### 2024-01-02  foo foo foo
line 1
line 2
line 3


### 2023-12-26  Notes for the Run class implementation:


Execution time: 0.3102356659946963 seconds.  -- cached accessor access
Execution time: 0.09941066603641957 seconds. -- simple accessor access
![[Screenshot 2023-12-26 at 8.17.00 PM.png]]
### 2023-11-30  Visualization

vis SPEC



#### Fields
-Viz"：
"CAMERA*: true,
"BBBOX": false,
"JID": false,
"DRAW_OBJECT_DETECTIONS": false,
"TARGET DETECTIONS": 
	"ball",
	"basketball",
	"person",
	"hoop",
	"backboard"

"PERSON MULTI COLOR MODE": false,
"DRAW OBJECT TRACKS BALL": true,
"DRAW_OBJECT_TRACKS PERSON": true,
"DRAW OBJECT _TRACKS HOOP": true,
"DRAW OBJECT TRACKS BACKBOARD": true,
"DRAW ACTIVE DETECTION HOOP": true,
"ACTIVE DETECTION BALL": false,
"PLAYER IDENTIFICATION": true,
"PLAYER ID ASSIGNMENT*: true,
"PLAYER ASSIGNMENT_THRESHOLD": 0.0,
"NARRATION": true,
"SHOT _START": true,
"'POSE": false,
"RAWTRACK": false,
"RBPTRACK": false,
"FPS": true,
"display filename":
"HOMO": false,
"BALL POSSESSION": true,
"SHOOTER IDENTIFIER": true,
"search_shooter_by_player_id": true,
"courtmap filename":
"overlay courtlines": false,
"display": false,
"track id": true,
"persist jid": true




### 2023-10-19  Report Semantics

REPORT
- derives points from merging result sources
- runs reports


- points(source_paths) -> merged points (stored in & under CWD)
- table(points, value_key, row_key, col_key, sheet_key)
- sb

xxx

	import rpt
	.
	lst = rpt.points()
	df = rpt.df()
	df

### 2023-10-14  JOB semantics

#### Slide

- recursive expansion
- one expands to three
- cacheable gallery
- restartable scripts
- Sharable across  S3
#### Text


A JOB SCRIPT is an executable that performs some processing over some INPUT data.  The first argument to this script must be the "job spec" then an optional "input spec" and args.  When a job script is run it will read in its spec and use that as parameters while reading and processing its input.  It will write out is results into the current folder, and return the JobResult objects as well if it is a run as a python function.  Here are the expected args for a generic a job script expressed as a python script and as a python function:
        job_script.py SPEC INPUT_FOLDER
        job_script(spec, input_folder:str) -> JobResult
    

A JOB SPEC is the configuration data for a job script.  It can be:
- a dict of dict
- a folder with a [_JOB_.json] or [_JOB_.yaml].
- a python file [_JOB_.py] that has a global in its root called JOB that contains the expected dict.  The advantage of using a python file to define the job is that the functions defined in that module can be used a do method or other functions to be used within its own spec.
- A job spec is called a "complete" job, if it specifies the input data to be used, or if this kind of job does not need an input.  It is "complete" in the sense that it is ready to run w/o any additional info, else an input must be supplied to run it.


A JOB RESULT is the data produce by running a job spec over some input.  It is stored in folder containing a _ RESULTS _ .json along with any other data files produced by the job.
By default job results are stored in the CWD where the job script is run.  (the 'job' command will create a dated directory to store results)


JOB.PY launcher script provides a convenient command line wrapper for the Job class.  The Job class manages the setup and execution of the indicated job script; this includes:
- Loading of the initial spec object.
- Application of the "--set" overrides.
- Recursive expansion of the initial spec dict to produce the fully expanded job spec.
- The automatic creation of the results folder where the job script can write any produced outputs.
- Change CWD to this results folder during execution of the job script.
- Writes fully expanded spec as [_SPEC_.json]
- Triggers execution of the job script or job function with the CWD set to be the newly created results folder.
- Writes the result of the job script (which should be a results object) to [_RESULTS_.json]

	job SPEC ARGS
		-o --output
		-s --set key.subkey...=value
		-S --sets key.subkey...=value,
		--- ARGS ...


- A JobResult is the output of a job
- A "do" action is the spec of a thing one can do to a job
- A "run" script specifies how a job is performed.  
  It is either an executable (a #! script) in the bin folder, or a python function registered as a job function.
  First arg of script is the job_spec, second arg in input_spec (both as JSON or dict) and output is result (saved in CWD)





### 2023-10-14  Pts Semantics

A "Report" is a job that produces 


- For each element, e, in the input tree:
	- If e is a game then
		- Execute run.sh
		- Execute each_do
		- Save into folder
		- Add points to dataset
	- if e is a result
		- Load result
		- Execute each_do
		- Add points to dataset
- Create the Report Result with dataset points
- Execute 

### 2023-10-06  Pts Command


Executes the indicated action using the specified script over the specified arguments and input, followed by any specfifien.

	pts KEYS ARGS
		-a  --args ARGS				# Specifies the action to be performed (all args etc.)
		-s  --script SCRIPT_NAME		# Specifies the script or python function used to perform this action
		-i  --input INPUT			# Specifies the input data to be processed
		-d  --do DO_COMMANDS			# Specifies additional do methods to perform
		-o  --output PATH			# Specifies folder path for output (else at YYYY-MM-DD under CWD)
		.
		.    --axes AXES
		.	--avg AXES
		.    --report_axes AXES
		.	--report_actions 
		.
		.
	job PTS_JOB_SPEC_NAME KEYS ARGS
		-d	--dated
		-g	--game_dos
		-r	--report_dos
		-o 	--output PATH			# Specifies folder path for output (else at YYYY-MM-DD under CWD)
		.    
		.

Performs a batch of processing

	batch
		-a --action actions
		-i --input  INPUT ...
		-d --do     METHOD ...


ACTION METHODS

	all -a ACTION INPUT_OR_RESULT . . . 
	avg 


DO METHODS

	table PATH VALUE_KEY COL_KEY ROW_KEY SHEET_KEY
	graph PATH VALUE_KEY COL_KEY ROW_KEY 

### 2023-10-03  Batch examples

	batch .
	batch fuzzy_videos       # used default
	batch -c test_pipe fuzzy_videos
### 2023-10-03  Command Line Thinking


	run.sh --config [config] --input [input] --output [path]
run
- performs a single 'run.sh' of our system
- a config / input / output is the path to a folder containing the corresponding 

``

	batch [keys] [config] [input] ...
	batch -s --set key=value
	batch -o --output path
batch
Uses the indicated config to process all specified inputs in order to produce a folder of outputs.
- INPUT
	- one or more folders containing datasets or inputs
	- flattened into a single list, if folder name is reused then -N version number is appended to make it unique.
	- First CWD is used to look for specified input paths
	- Then GAME_DATA env var is used to search the local repo
	- Then 
	- includes CWD if it itself is an input and no input given
- OUTPUT 
	- path of folder where output folder will be created.
	- output folder will have name YYYY-MM-DD-R
	  where R is the optionally added run number if multiple
- CONFIG
	- if specified on command line that is used
	- If CWD has a game_config file, then it is used
	- if 




Can we use classes
	GameInput, GameConfig, GameOutput
### 2023-10-03  LCM Data Packet

A DATA PACKET

KINDS OF DATA PACKETS 
Sports Visio Has Four (so far)
- GAME INPUT - videos and related meta data for a single game
- GAME DATASET - A collection of inputs
- GAME CONFIG - configuration for some version of our pipe
- GAME RESULT - output pickle and other data from a run

PACKET FORMAT
- A packet is always a folder 
- The name of the packet is the filename (but not full path) of that folder
- The kind of packet is determined by the existence of a yaml, json, or csv file inside the folder:
	-  _game_input_.json,  _game_config_.json, _game_result_.json



