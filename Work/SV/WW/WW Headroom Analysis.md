
BACKGROUND: 
Prioritizing CV work is very difficult, there are an infinite number of approaches, problems, and sub parts one might consider, each having its own unique and complex interacting tradeoffs. Given this it is difficult to effectively plan, project progress, and allocate limited R&D resources.

OBJECTIVE:
Build a framework for the assessment and understanding of potential CV efforts in order to provide insight to the larger team regarding the (1) timing, (2) likelihood, (3) quality of improvements, and (4) effort required for each of the possible approaches the CV team might undertake.




APPROACH:

HEADROOM - We refer to this a "head room" analysis since each improvement is assess relative to some explicitly specified "head room" available for improvement. For example, in developing track-label error correction algorithm, one might start by measuring the percentage of incorrectly labelled baskets that resulted from incorrect tracking.  This percentage is ones "head room", even if one were to build the perfect algorithm that correct all tracking errors, one could never improve our stats by more this headroom amount. Explicitly computing this headroom across a range of alternate efforts becomes especially important when potential efforts are affecting very different parts of our CV pipe. It gives us a "common currency" for comparing their relative importance even when one cannot directly compare the kinds of improvements they provide.


HEAD ROOM MATRIX - The headroom matrix has columns assessing relevant attributes of each potential CV effort, and each row in the matrix corresponds to a single CV effort that might be undertaken. Defining a combination function allows a single "merit" score to be computed over these efforts, and the table can optionally be sorted according to this merit score. (The merit score will necessarily be quite crude, thus the actual efforts undertaken will often not all be those with highest merit.  Still it is nice to have a rough gauge that puts the big impact items on top.)


PROCESS STEPS:
1. ENTRY - As each CV researcher/engineer thinks about possible approaches to our current CV challenges, one of their first actions will be to fill out a new row in the headroom matrix to represent the potential idea. If the idea has too little merit, they might decide to simply table further consideration for later. 
2. Correct assessments to fill into the headroom matrix take a great deal of expertise to assign, and will be subject to significant (healthy) debate. The CV team as a whole can discuss and arrive at best understanding through group discussion during sprint planning.  Arriving at best numbers for this matrix will be key to effective planning as decisions about what to work on next will come directly from comparative discussions between different rows in the matrix.
3. Upper management can view this matrix as something of a menu.  It represents the CV teams best understanding of outcomes and timelines that would occur depending upon the choices the CV team makes regarding its work.  Inputs regarding needs and preferences can be supplied on an ongoing basis to the CV team.
4. Sprint planning can now take place with this headroom matrix, and upper managements preferences as a backdrop. The exact efforts undertaken and their ordering will take into account many factors beyond just this matrix.  Still this matrix likely provides the single most important factor in decideing what is most important to work on next.
5. After sprint analysis can then be performed to see how well our predictions of level of effort and levels of improvement matched our predictions as a way of training ourselves to make better future predictions.



MATRIX COLUMNS:




that of various improvements



KEY FACTORS:
- SCOPE - Assuming the effort succeeded as well as can be expected, what group of potential or actual customers benefited from this
	- GROUP - what group might be affected: Existing/New  AAU/Mens/Womens, etc.
	- SIZE - what is the size (in expected numbers of customers or in expected revenue) of the listed group
	- PERCENT - what percentage of the group will likely be affected
	- CONFIDENCE - what probably do we associate with with this affect occurring
	
- IMPACT - What benefit did recipients obtain from this effort?  Didn't Churn.  Did onboard. Did pay. Did upsell.

- QUALITATIVE IMPACT - User describes impact as:
	- TRANSFORMATIVE <...> NOT-RELEVANT

- 