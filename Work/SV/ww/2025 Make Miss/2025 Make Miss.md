.[[2025-09 Make Miss]].  [[Make]] 
  Use [[2025 Alg2]] 
- [[GCP]] 
- docs:  [[001_classification_task]], [[002_classification_papers]], 
- 
## Todo
- Run just pull on local machine
- Run CI build on my machine
- Get GCP Dev box setup (has linux commands)

## Plan
- Extract Data
	- Get Data from a Norm Game. (use tiny to start)
	- Shot meta data; Video Frames
	- Find the shot intervals as a list of dict with all meta data needed to build the training set.
- Train System
	- Research best algo to use
	- get or write basic training alg
- POC System
	- Scans folder of games to extract dataset
	- Trains system based on dataset
	- Reports cross fold validation over dataset.
- Scale
	- Get the command that pulls many G-games.
	- Run these games and save to a runset.




