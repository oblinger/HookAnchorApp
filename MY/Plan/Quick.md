- [ ] papers chew weekly
- [ ] rince
- [ ] port replicator
hair
paperwork
- [ ] repeat
      build paln
- [ ] read slack
- [ ] Google Teams - 
- [ ] Game annotation pipe - 
- [ ] Testing tools @The 




Grz:

I was thinking some about the player/team discrimination problem. It seems that there is a hugely valuable info source that should be pretty decorrelated with errors in detection: detections in subsequent/previous frames where we have high-confidence tracks. For the moment I am imagining an inefficient method of running thru the entirely length of a game and finding all tracks that are high reliability, and perhaps are sufficiently separated from other tracks that we are confident in our tracking.  Then independently we label that patches along these tracks, and notice when we get discrepancies in our labelling.

This should give us a reliable source of ground truth to improve our per-player and per-team discriminators.  We can re-train the discriminators with new error correction examples added in. Ultimately this could even be fed back into the tracker to make it better, but that loop would probably be too expensive to run on a per-game basis.  We would need to build some kind of pre-training strategy for this.

Before getting further into the details, what do you think about that direction?

--dan







The direction I am thinking about here, will not be the most efficient per game