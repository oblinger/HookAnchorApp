.[[2023 Belief Propagation]].
  ,   ,
  , ,
  DELS: [[2023 Belief Propagation]],[[belief]], 

(See [[2023 Belief Propagation]])




- [[BELIEF Gdrive]] 
- [[2023 Belief Propagation]]

[[belief]]
- [Quick Hack](https://docs.google.com/document/d/1O8ytJQGBQgYgtyM60XN-OFuGngv5lW-T9EHNpvldQiM/edit#heading=h.1dy43qe05cwu) 



- n = 2.5 seconds (minimum tracklet / scrum len)

- t = 1.0 seconds 'tick' size

- At each frame moment an ordered IOU tracklet sets ordered left to right

- Once set has existed for n seconds it is added to the graph 
	- as an arc if set is a singleton tracklet
	- as a node if it is multiple tracklets

- At each tick the current list of active sets adjusted to move each active set towards the center of its available space
- Then the list of active tracklets is printed for this tick, connecting each to the tick before

- Tracklet labels are only applied once every ten seconds



Building The Scrum Graph

Parameters:
SCRUM_IOU = .3




### 2024-01-02  test example
line 1
line 2
