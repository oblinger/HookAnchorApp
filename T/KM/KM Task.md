< [[OBS]]

## SINGLE CHARACTER TASK TARGETS

####  		ADD		 W	JUMP		SEND  	  Notes

- 	NAME		FLAGS		NOTES  (Flags: A=ADD  J=JUMP  W=WORSPACE  S=SEND-TO)
- A	[[active]]		A J   S		Active project pages   *{{send to ARCHIVE}}*
- B	Back		    J			Jumps to the previous workspace configuration
- C	[[COMS]]		A W 		*WORKSPACE* of professional communication pages.  ([[Calls]] page is used for personal coms)
- D	Daily		   W			*WORKSPACE* of daily lists
- E	
- F	[[fried]]		A J   S		Things to do when 'fried'
- G	[[gap]]		   W			*WORKSPACE* of activities ...
- H	[[hack]]		A J   S		Hacking tasks
- I
- J						
- K
- L	[[later]]		A			Items to be reconsidered quarterly in order to be handled at some undefined "later" time
- M
- N
- O	[[OBS]]		A			Obsidian tasks
- P	[[Plan]]		A W 		WORKSPACE showing all weekly planning lists
- Q	[[quick]]		A J   		Quickly added info
- R	[[Repeat]]					Weekly 'repeat' list
- S	[[MY/Plan/self]]			A W		WORKSPACE of personal planning pages
- T	[[todo]]		A J   S		Primary, unified, small-item todo list
- U
- V
- W	[[external]]		A J   S		List of items waiting on an external action
- X		
- Y		
- Z		

 - [[#ADD ITEM COMMAND|Add Item]], [[#JUMP ITEM COMMAND|Jump to page]],  [[#SEND ITEM COMMAND|Send Item]],  

### ADD ITEM COMMAND
Dialog used to add a single line item to a todo list note

See Keyboard Maestro CAPS-S for latest version

{
    "a": "/Users/oblinger/ob/kmr/MY/Plan/active.md",
    "c": "/Users/oblinger/ob/kmr/MY/Plan/coms.md",
    "f": "/Users/oblinger/ob/kmr/MY/Plan/fried.md",
    "h": "/Users/oblinger/ob/kmr/MY/Plan/hack.md",
    "l": "/Users/oblinger/ob/kmr/MY/Plan/later.md",
    "o": "/Users/oblinger/ob/kmr/MY/Plan/other.md",
    "p": "/Users/oblinger/ob/kmr/MY/Plan/prime.md",
    "q": "/Users/oblinger/ob/kmr/MY/Plan/quick.md",
    "s": "/Users/oblinger/ob/kmr/MY/Plan/self.md",
    "t": "/Users/oblinger/ob/kmr/MY/Plan/todo.md",
    "w": "/Users/oblinger/ob/kmr/MY/Plan/work.md"
}


### JUMP TO PAGE COMMAND

- Keyboard Maestro global CAPS-J commands form menu
- Commands either open a single page, or
- Open a specified workspace


### SEND ITEM TO PAGE COMMAND
(FLING commands)

- Keyboard Maestro "Macros for Obsidian" group.
- Look at commands with CAPS-S binding, these form the SEND ITEM menu


    - Kbd Maestro		# CAPS-S entries each fling to distinct target file (only while in obsidian)
    - _fling      	# python script to write data to file


## DASHBOARD CONFIGURATIONS


### DAILY VIEW DASHBOARD

|   col-1    | col-2 |
|:----------:|:-----:|
| [[quick]]  | Week  |
|  [[todo]]  |       |
|     "      |       |
| [[work]] |       |
|     "      |       |
|  [[active]]  |       |
|            |       |



### COMMS DASHBOARD

|    col-1    |    col-2     |  col-3   |
|:-----------:|:------------:|:--------:|
| [[COM Now]] | [[COM Next]] | [[COMS]] |
|      "      |      "       |    "     |
|      "      | [[external]] |    "     |
|      "      |      "       |    "     |



### GAP DASHBOARD

|   col-1   |  col-2   |   col-3   |
|:---------:|:--------:|:---------:|
| [[fried]] | [[hack]] | [[ToBuy]] |
|     "     | [[OBS]]  | [[quick]] |
|    [[gap]]    | [[fun]]  | [[todo]]  |
|           |          |           |



### PLANNING VIEW DASHBOARD

| col-1  | col-2 | col-3 |  col-4  |
|:------:|:-----:|:-----:|:-------:|
|  Week  | Todo  | Prime |   Q2    |
| Active |   "   |   "   |  [[later]]  |
|  Work  | Wings | Full  |  [[quick]]  |
|   "    |   "   |   "   |  [[other]] |



### SELF DASHBOARD

| col-1  | col-2 | col-3 |  col-4  |
|:------:|:-----:|:-----:|:-------:|
|  Self  | Todo  | Prime |   Q2    |
|   "    |   "   |   "   |  Later  |
| Active | Wings | Full  |  Quick  |
|   "    |   "   |   "   | Waiting |




# LOG
### 2022-12-11  Daily

| col-1 |  col-2  | col-3  | .......................................................... |     |
|:-----:|:-------:|:------:|:---------------------------------------------------------- |:---:|
| Today |  Quick  | Prime  |                                                            |     |
|   "   |  Todo   |        |                                                            |     |
| Prev. | Current | Second |                                                            |     |
|   "   | Waiting |  Full  |                                                            |     |
|       |         |        |                                                            |     |

