TODO:
- Dict Docs


 [LCM-013](https://docs.google.com/document/d/1kxGMrmPOr9S6whRnV0LI0UpWGeyBzTn8WGDq7ITM4Sc/edit):  V2 docs for input
 
- [[2023 Least Commitment Model For CV Pipe]]


# LOG

### 2023-10-03  Batch examples

	batch .
	batch fuzzy_videos       # used default
	batch -c test_pipe fuzzy_videos
### 2023-10-03  Command Line Thinking


	run --config [config] --input [input] --output [path]
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



