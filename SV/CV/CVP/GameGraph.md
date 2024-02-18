
OUT OF DATE. (see [Game Graph](https://docs.google.com/document/d/1ZL5zMCpwDJ3Ai2groT2dmtfiw0yFLfnt3aU0u4mm7zw/edit) )



MCproc.state.

s.data[]



SVDetection


**  

A Detect Node may additionally contain:

- frame_num - The frame number (or first frame for a merge).
    
- box - A bounding box covering all pixels.  (from all detects for a merge)
    
- 2d_location - (could also be called pixel location)
    
- 2d_size - width and height in pixels
    
- 3d_location - (this is really still 2d it is just in 3-space. could also be called court_location)
    
- image - The pixels within the detect’s bounding box
    
- mask -
    
- merge_pdf - List of condensed nodes this detect maps “up” into
    

  
**



# The Game Graph

***Objective**: The Game Graph is a data structured proposed as a centralizing, organizing structure allowing the decomposition of the Player Tracking Problem into quasi-independent sub-problems.  Including the sub-problems of object detection, player identification, tracking, inference/belief-propagation, action identification, and fine-grained image analysis (e.g. shoe, sleeve, hair identification).*
- The intent is for this structure to be consistent with, and interoperable with our existing tracking based methods, but to generalize to processes that don't operate temporally over the data as a tracker does.  It facilitates iterative and structural updating processes that operate in a-temporal fashion.
- This structure is intended to be realized as an interface that is accessed and modified by multiple parts of our system.
- Fields in this structure may not exist in memory but only be derived as it is accessed.
- Fields in this structure may only exist as certain points and then be deleted to conserve memory.
- There is no requirement that the entire structure is produced at one time and stored in memory as a whole
- The structure may be arrived at in an iterative fashion where certain processes tweaked and progressively rewritten as new information is processed.
- This is intended to be a live document, we would rewrite it to best match our existing code, and to evolve it as our code evolves.
- The core aim is to allow:
  (1) decomposition of player tracking into plug and play sub problems, and 
  (2) to provide independent performance assessment for these sub-systems.




The ***Game Graph (GG)*** is a directed graph of detections derived from the game video.  The optional edges map 

A ***Detection (DETECT)*** is a node in the game graph that may contain:
- frame_num - The number of the image frame that contains this detect
- duration - Game graph as duration 1, but condensed node have duration over multiple frames
- box - A bounding box 
- image - Pixels within the detect's bounding box
- mask - 
- 2d_location - (could also be called pixel location)
- 2d_size - width and height in pixels
- 3d_location - (this is really still 2d it is just in 3-space.  could also be called court_location)
- 2d_velocity(other_node) - 
- 3d_velocity - 
- players - Unique player ids that could map onto 
- players_pdf -
- tracks - List of linkages to detects in the next (or next several frames)
- tracks_pdf - List of probabilities for each track in tracks.  (should be in descending order?).  Typically only one or two entries.

{{ please rename these to better match existing code/discussions }}



The **Condensed Game Graph** is a partitioning of the game graph where multiple related detects are merged into a single condensed "node" such that there is a many to one mapping from detects to nodes.  And nodes occur within a relatively short number of frames (its duration).
- Each Node's properties summarize, in some way, the commonalities and information inherent across the detects that are "condensed" into this node.  
- The *entirety* of Condensed Game Graph is functionally (and ideally deterministically) derived from the Game Graph.

{{ Is there a better term for a "node"? }}

Note: A graph of tracklets is a kind of CGG but likely we will be constructing non-tracklet style condensation in cases of proximity or occlusion especially in those cases with less motion.  Fine-grained analysis also likely yields non-tracklet-based condensations.



The **Game Info (GI)** object is a composite structure that may containing all inputs, derived data, and final stats from a single game including: 
- The Game Graph or the Condensed Game Graph respectively referred to as the: "Full Game Info" or the "Condensed Game Info"
- Ball tracking (in 2D & 3D)
- Shot Attempts
- Annotations
- Stats



### Annotations





# Implementation Thoughts


### Graph size estimation

{{ Please correct these numbers }}

60 minutes of video time per game (median number)
60 second per minute
30 frame per second
40 person detects per frame 
~ 4.3M detects per graph

~43K nodes in the condensed graph.  (assuming a ?100x? avg reduction)