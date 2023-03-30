

# INITIAL IDEA AS SHARED TO SCOTT

  
Since we know that we will miss many quick clock stops, but can catch all of the longer time clock stoppages we assume the smaller stoppages are somewhat uniformly spread over the course of the game.  Thus we “peanut butter” those stoppages over the entire length of the non-stopped portions of the video using a multiplicative “contraction factor” from [0-1]  
  
Annotations:  
- GameBegin  
- GameEnd  
- Begin Timeout  
- End Timeout  
  
  
Calculation:  
  GameTime = 48 minutes  
  VideoTime = GameEnd - GameBegin  
  TimeOuts = Sum of all timeouts  
  ContractionFactor = GameTime / (VideoTime - TimeOuts)  
  
  
GameTimePoint =  ( VideoTimePoint - GameBegin - Sum of all Timeouts prior to this VideoTimePoint ) * ContractionFactor  
  
~  
Once we have GameTimePoints for all player swaps, we can just add up these time intervals as they will automatically be accounting for all timeouts that occurred during the time they were on the court.


# Refinement Email

Scott, I think we are saying the same thing for including the offsets.  (I assume “start time of offset” and “end time of offset” are two time points in the recording where the game clock is stopped between them.  Correct?)


But I had one additional idea.  As I understand it.  We will not capture all timeouts in all games.  Many games will also have very short clock stoppages that we will not annotate.

If that is correct, then I was going to be squishy with those missing annotations.  Essentially I know how many minutes was on the game clock at the end of the game.  (48 minutes).  And I know how many explicit time outs where in the game, let’s say 10 minutes.    

So if the recording time was 58 minutes, I would know that I captured every single timeout in that game perfectly.

But what if the recorded time was 63 minutes.  Now I have 5 minutes too much video.  So I was going to scale the non stopped 48 minutes by:
   (48-5)/48 so that I smoothly incorporated those unannotated minutes into the game… assuming that missing annotations were somewhat uniform.


I think this is a small adjustment to the calculation you are proposing…. I think it fits your proposal’s implementation well.

Did this make sense?  Am I missing your idea?  Very happy to jump on the phone for 5-min.

