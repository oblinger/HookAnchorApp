


- [[2023 Full-AI Basketball Highlights Product]]


- [T34-FullAI](https://docs.google.com/document/d/1Iw5e200QBn2uxzgiAcHHcLi7DDTBZjtPXpbFmBcCEYE/edit) 

## Todo
- [Product Spec](https://docs.google.com/document/d/18qqnVHsXttr1g39iLYnjIOYTfNVbC67JyAATvRQwEfQ/edit?usp=drive_link):	
- [Cost Analysis](https://docs.google.com/document/d/1WYVwgCfHYk6wC8pGFo68t_vAju_oczbSeBTS-zba-XI/edit?usp=drive_link):	


#### From Mike

Thinking about shooter ID, what we've done and what to consider doing.  
To get the shooter, we need to find 3 things: when, where, and who.  
We need "when" to be able to segment video highlight clips.  
We need "where" to be able to determine shot points.  
We need "who" to be able to determine player stats.  
* = indicates the items below that we have implementedAssume all tracks have an attribute:  
offense, defense, referee, nonplayer, or unknown  
OU = offense or unknown (may be offense)  
DU = defense or unknownEvidence for when a shot has occurred:  
* ball near the hoop  
* eliminate non-shot balls near hoop  
* eliminate non-active hoops  
eliminate non-active balls  
OU player throwing ball up  
ball arc falling toward hoop  
OU player hand near hoop  
referee hand signals  
players in free-throw configuration  
rebound  
DU throw-in near hoop  
crowd noise  
scoreboard points incremented  
eliminate fouled shots  
eliminate time-out shotsEvidence for where the shooter is:  
* ball backtracked from hoop to OU player  
OU player throwing ball up  
ball in "front" of OU player  
OU player track locations at "when"  
OU player with hands up  
near DU player with hands up  
location of OU player having current or very recent possession  
OU player jumping  
OU player on court  
account for camera motion in ball and player tracks  
eliminate player tracks moving away from hoop  
eliminate defense, ref, and nonplayer tracks  
eliminate non-active player tracksEvidence for who the shooter is:  
* OU player track ID consistent with when and where  
* OU player ID having current or very recent possession  
eliminate offense player IDs visible somewhere else  
eliminate non-active player IDs



# Breakdown



BELIEF PROPAGATION

ACTION MODEL


ACTION 


SHOOTER ORIGINATION
- Torso Pose, Head Pose, Arc-Basket Relation
- Hands up, Jumping, Hand Near Ball


PLAYER ID
- OCR - 


VELOCITY
Unified data form
- Money Metric




### Other Stuff
- ACTIVE BALL - Eliminate non-active balls.
- FOULS - Eliminated fouls.
- HAND SIGNALS - Referee hand signals  
- FREE THREplayers in free-throw configuration  
rebound  
DU throw-in near hoop  
crowd noise  