
FORMAT
=[[CMD]]  url  ...


| cmdtype | arg         | Action behavior                                         |
|:-------:| ----------- | ------------------------------------------------------- |
|   app   | command     |                                                         |
| console | cmdline     | Runs in console & allows user input                     |
|   doc   | file path   | Opens referenced document using its default application |
| folder  | folder path |                                                         |
|  shell  | cmd line    | Runs cmd line in headless mode                          |
|   url   | url         | Opens url in default browswer                           |
|         |             |                                                         |


SUFFIX USED
- 			--  No suffix overrides others as leading choice
- App		--  Launches Mac App (3)
- 1Pass		--  Launches from 1password (5)
- gDrive		--  Web folder (6)
- Folder		--  Launches finder on Folder (6)
- WebPage	--  Launches URL (7)
- InfoPage	--  Background info on Keyword (8)

SCRIPTING
    _ 1pass  <word>         # will open browser and login to specified _1pass target

EXAMPLES
    =[[CMD]]  shell _1pass crunchbase

