- (See [[QUICKADD]])

FORMAT for each CMD.md file
=[[Cmd]]  url  ...


| suffix  | cmdtype       | arg         | Action behavior                                         |
| ------- | ------------- | ----------- | ------------------------------------------------------- |
| 1Pass   | shell _ 1pass | 1pass entry | Uses _ 1pass script to initiate login                   |
| App     | app           | command     | Launches specified Mac App                              |
|         | console       | cmdline     | Runs in console & allows user input                     |
| Doc     | doc           | file path   | Opens referenced document using its default application |
| Folder  | folder        | folder path | Opens specified folder in the finder                    |
| Shell   | shell         | cmd line    | Runs cmd line in headless mode                          |
| Webpage | url           | url         | Opens url in default browser                            |

IMPLEMENTATION (of execution)
- JS plugin "Action Notes"

- Triggers `_1pass` in the bin directory:
  `_1pass  WORD`         # will open browser and login to specified _1pass target

IMPLEMENTATION (of key definition)
- QUICK Add (need to restart after adding)
	- 

EXAMPLES
`    =[[Cmd]]  shell _1pass crunchbase`

