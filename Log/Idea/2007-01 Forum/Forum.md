.[[Forum]].
  ADDS:   [[2021-01-00 Short start with Yashin]],
  ADDS:   [[Truth]],
  ADDS:   [Forum.PPTX Doc](spot://forum.pptxdoc),
















































































































































































































:: [[2021-01-00 Short start with Yashin]]
## Todo  [[LOGS/Ideas/Ideas Social/Forum/Forum.pptx|Power Point]] 
- [ ] Research prototype Forum platform
- [ ] Look for partners:  Jason, The common wealth club, Braver Angels, ...
- [ ] Build & Test prototype; Run 4-6 person alpha.
- [ ] Run time-limited alpha test.


# 2022 - THE FORUM - WHAT WE BELIEVE
## = APPROACH
### Steps

#### 1. CLUSTERS - Derive Population Factors and Clusters
- Use existing surveys to derive an order list of 'factors' that best describe survey data.
- Cluster US population according to these factors.

#### 2. PARTICIPANTS - Survey & Recruit Participants To Fit Into Clusters
- Survey existing participants to determine which cluster they belong in
- Measure under-representation both at the factor (axis) level, and at the cluster level.
- Recruit new participant for under-represented areas.
	- Perform recruiting from within specific sub-populations (e.g. specialized reddit forum)
	- Use existing participants of a given type to recruit more like minded participants
- Monitor voting behavior of participants to see they are not far out of line w/ their cluster

#### 3. DISCUSSION - Bottom Up Participant Discussions
- POST - Participants engage in the forum by reading and authoring forum 'posts'.  Post parts:
	- BODY:  Each post has a body of text.
	- PARENT:  Each is placed into the forum hierarchy thru its attachment to a parent post.  (like email threading)
- BUCKET - Each post defines a bucket of all posts in the threaded responses that directly or indirectly attach to it.

#### 4. STANCE - Expressing Support or Opposition
In addition to simply adding to the discussion of a topic, a post may indicate a "vote" -- the support or opposition to some perspective, claim, or position.  
- POST - These additional fields on a post help indicate this extra info:
	- STANCE: A post might indicate a personal disposition towards its parent using a "+1" or "-1".

	- NAME: A note might have a naming phrase which can be used to refer to the note.
	- TITLE: A note might have a 'one-liner' title that summarizes its content.

#### 5. Mechanical Synthesis
- A purely mechanical (algorithmic) process computes the output of the forum:
         an annotated, ranked, synthesis of human though expressed by the participants over their topics of interest.

HIERARCHY - The '+1' and '-1' marks provide the first bit of structure for the forum:
- VOTE - a '+1' or '-1' note is a called a vote on the note it references.
- STANCE - a note that begins receiving but positive and negative votes is call a 'stance' -- that is accepted/rejected by others
- ISSUE - a note that has stances as children are called an 'issue' they are a topic upon which there exists competing stances
- TOPIC - a parent of an issues or topics

COVERAGE - At all levels in the taxonomy the votes of relevant participants are used to derive an order lists of position notes to "cover" the diversity of thoughts (called ***positions***) on a particular topic or issue.  Algorithm:
- Initially the set of uncovered voters is the set of all participants that issues a "+1" vote in this sub-tree, 
  and the list covering notes is empty.
- The note gaining the most "+1" from the set of uncovered voters is added to the list of covering notes.
- All voters that voted for this covering note are then removed from the set of uncovered voters, and the process repeats.

POSITION / COVERED - Each note within the covering list is called a ***position***, and each participant whose "+1" was responsible for that position to including in the covering is said to be ***covered*** by that stance. 

SUPPORT - A note's support from a given set of participants is positively by the number of "+1" votes it receives and strongly negatively affected by any "-1" votes from that population.

BIAS - The 'bias' of a note is a measure of the balance in support it receives across participants covered by different stances.
- BIAS = the entropy of a note's support across the stances covering its peers.


#### 6. Authorized Participants -- (Creating Safe Sub-communities of Thought)
- AVOIDING THE TYRANNY OF THE MAJORITY - The forum avoids the "tyranny of the majority" where minority positions are manipulated or overwhelmed by greater numbers in the majority with contrary perspectives using the "STANCE TREE".  
- STANCE TREE - Covering stances are computed at all levels in the forum's taxonomy using the "+1" votes from the participants that supported that particular stance.  
- AUTHORIZED PARTICIPANTS - This sub-population of participants are the only participants that are authorized to extend this part of the forum tree.  This sub-tree becomes their "playground" in which they can clarify the meaning, claims, and details of this particular stance.


#### 7. Combination and Summarization
- The forum is intended to synthesize concentrated summaries of important positions, areas, etc. of human thought.
- A 'summary' is intended to provide such a synthesis over some subset of its space of discourse.
- SUMMARY NOTE - In addition to the "+1" and "-1" intents listed above, and note might specify intent = summary
	- VOTES - All notes that list themselves as a summary for a node within the forum taxonomy are voted upon.
	- COVERAGE - The same coverage algorithms used over those summary notes, but only those with some minimum support are considered, and instead of popularity, it is the summary with the least bias that is chosen first.
	- Areas with little contention the top summary will have wide support and very little anti-support.  In cases with great contention one will want to look at the second and maybe even third summary in order to even get a sense about how various 
- CONTENTION - 
	- Even areas with significant contention might be able to agree on an unbiased summary with many '+1' and few '-1' votes.  
	- For the most contentious areas however this will be impossible.  The framing (even *naming* of the area) might be impossible w/o great dissent from certain sub groups.
	- The Bias coverage algorithm is designed specifically for this case.  It lists a sequence of summaries that collectively cover an ever greater fraction of all voters with a summary that they personally had accepted.
	- The forum's Reader is set to automatically show all summaries down to some some minimum size (e.g. perhaps down to >10% of the voting population), thus extra summaries are only provided by default when there is great contention.  Otherwise one summary will suffice.
- COMPOSITION -
	- FORUM IS BOTTOM UP: The forums discussions are very bottom up.  Any top-down framing of the discussion will be seen by some as biasing its output.  Still high value output require synthesizing larger claims, stances, issues and areas from these more granular contributions.
	- PARTS: To counter this, new notes may explicitly specify existing buckets as their children and then synthesize into a summary thought as described in their body.
	- VOTING UP:  Of course since ANY participant can combine nodes from ANY place in the taxonomy there is no guarantee that other will find value in this synthesis.  Thus these combined synthesis notes will undergo the same voting process as all other notes.  Only those with greatest support will bubble up within the tree.
	- COMPOSITION IS RECURSIVE:
		- Facts (external references) combine into claims.
		- Claims combine into larger claims or into stances.
		- Stances combine into issues.
		- Issues combine into topic areas.
		- Topic areas combine into larger topics.
		- Topic areas can be multiply organized into as many taxonomies as the participants have interest in maintaining and voting upon.
		- E.g. Largest topics of 2022.  Political topics.  Topics relevant to gender.  Humanity and spirituality. etc.
		- The existence of one lens on the roiling mass of discussion on the forum need not preclude a competing lens over the same set of topics.






#### 8. Adjudication
- They may offer a more technical adjudication on the note as well:  +CONSTRUCTIVE, +ACCURATE, +COGENT
  (move this to later)





#### 4. Top Down Topic Tree Construction
- Population-balanced groups of the most open, constructive participants organize discussions into topic taxonomies
- Taxonomies are DAGS where each node's children are organized into more central and more peripheral topics
- Textual topic thumbnails give an quick overview of a topic area.
  NOTE: the more detailed summary for each sub-area and for each position is produced by those sub-groups.

#### 5. Stance Authoring
- Any participant may author any claim/position/stance on any topic or issue.
- By default a stance is simply the stance of that single author, and they are the only ones who have authority to edit this stance.
- The author may select individual participants as co-editors.
- The author may also may propose it as a stance held by any cluster they are a member of, in that case all other members may edit that stance.



### Definitions

**NOTE TYPE** -- The intention of a note specifies what the author 'intends' to convey by their note.  These include:
- **post**	  -  General discussion on topic
- **edit**		  -  Proposed edit on part or all of a note
- **subarea**	  -  Named Sub section of a note
- **issue**	  -  A topic upon which one may adopt different positions
- **position**	  -  A particular stance on an issue
- **critique**	  -  A critique of a position, or the framing of an area
- **+1**		  -  A vote in support of a note
- **-1**		  -  A vote in opposition to a note


**SUPPORT** -- The ***support*** of note within a population is the fraction (or percent) of that population that have posted a "+1" for that note.

**OPPOSITION** -- The ***opposition*** of note within a population is the fraction (or percent) of that population that have posted a "-1" for that note.

**COVERING** -- The ***covering*** for a population of voters given a set of alternative notes is an ordering of the alternatives that best "covers" the support received by the voters.  The covering constructed by repeatedly selecting the alternative with the greatest support among the remaining voters, where voters are removed from the set of remaining voters each time a new alternative is added to the growing list of alternatives.  This process continues until no voters remain.
 
**CUMULATIVE COVERING** -- The ***cumulative covering*** is the 

ALPHA COVERING -- 
and removing its supporters from the set of remaining uncovered voters.  This process continues until no position remains with support greater than alpha.  The remaining uncovered voters are lumped together and treated as a special "none-of-the-above" position.  Thus a 10%-cover of an issue might be a list of four positions along with their supporting voters: {A with 70%, B with 15%, C with 11%, and OTHERS with 14%}

COHORT -- The voters within each element of the alpha cover are called a ***cohort***---the ith cohort are that voters that were responsible for the ith position in the alpha cover, or are the final, special 'none-of-the-above' cohort.

AUTHORIZED COHORTS -- The authorized cohorts for a given note are those cohorts whose participants are "authorized" to participate in the refinement of the note.
- default - By default a note will inherit the authorized cohorts of its parent note.
- issue - An issue's authorized cohort its alpha-covering.
- position - A position's authorized cohorts is a single cohort containing all voters that supported the position.
- critique - A critique's authorized cohort is the set of cohorts that are NOT supporters of it parent note.

BALANCE -- The ***balance*** of a note is a number from 0 to 1, where a balance of 0 means it has no balance, e.g. it was supported entirely by voters from a single cohort, zero supporters from other cohorts.  A balance of 1 on the other hand indicates that its level of support s, that is precisely the same level over all cohorts, in this case it has a perfectly balanced support.  The cohorts are chosen from the first ancestor of the note to have cohorts, e.g. the first ancestor that is an issue.
Formally the balance of an note is the entropy of its support across its participating cohorts:
	Given a note n, with authorized cohorts {cohort_0, cohort_1, ..., cohort_k}
	balance(*n*) = Sum j in [0, k]  support(n, cohort_j) * log_k support(n, cohort_j)

REPRESENTATIVES -- The versions of a note is the 60% cumulative covering of its editset for a note.









    Balance(note, issue) = Sum cohort_i in alpha-cover(issue)  support(n, cohort_i) * log_k( support(n, cohort_i)




HIERARCHY - The '+1' and '-1' marks provide the first bit of structure for the forum:
- VOTE - a '+1' or '-1' note is a called a vote on the note it references.
- STANCE - a note that begins receiving but positive and negative votes is call a 'stance' -- that is accepted/rejected by others
- ISSUE - a note that has stances as children are called an 'issue' they are a topic upon which there exists competing stances
- TOPIC - a parent of an issues or topics

COVERAGE - At all levels in the taxonomy the votes of relevant participants are used to derive an order lists of position notes to "cover" the diversity of thoughts (called ***positions***) on a particular topic or issue.  Algorithm:
- Initially the set of uncovered voters is the set of all participants that issues a "+1" vote in this sub-tree, 
  and the list covering notes is empty.
- The note gaining the most "+1" from the set of uncovered voters is added to the list of covering notes.
- All voters that voted for this covering note are then removed from the set of uncovered voters, and the process repeats.

POSITION / COVERED - Each note within the covering list is called a ***position***, and each participant whose "+1" was responsible for that position to including in the covering is said to be ***covered*** by that stance. 

SUPPORT - A note's support from a given set of participants is positively by the number of "+1" votes it receives and strongly negatively affected by any "-1" votes from that population.

BIAS - The 'bias' of a note is a measure of the balance in support it receives across participants covered by different stances.
- BIAS = the entropy of a note's support across the stances covering its peers.

## = ALGORITHMICS

DATA

    participant
    cohort
    note
    forum
    vote(note, participant)          -- +1, -1, or 0  depending on the participant's support for a note
    query(question, note, ...)       -- a query about a note (which could involve further args)
    opinion(participant, question)   -- returns  -1 to +1

DERIVED TERMS

    axis(participant)                           -- Maps a participant onto [0,1]
    covering(notes, participants)               -- Ordering of notes based on participant '+1' votes
    alpha_covering(notes, participants, alpha)  -- List of cohorts
    cumulative_covering(notes, participants, )  -- List of cohorts

NOTE METRICS

    support(note, participants)                 -- the fraction of participants that posted a '+1' for a note
    opposition(note, participants)              -- the fraction of participants that posted a '-1' for a note
    
PARTICIPANT METRICS

    alignment(participant, axis)                -- [0,1] position of a participant on a population axis
    authority(participant, axis)                -- 
    ???
    congeniality(participant, opinions)         -- [0,1] denoting  
    accuracy(participant, forum)                --  
    openness(participant, forum)                --  
    expertise(participant, forum, area)         --  
    


## Topics
### Minimal Build
- DISPLAY NOTE STACK
	- CORE SCREEN - this screen is the heart of the forum interface
	- META - Full name, title, bread crumb path
	- RICH TEXT - Bold, Italic, Underline, images
	- COMMENTS - Both +1 and -1 comments listed to the side.
	- ARRANGEMENT - Author orders and prioritizes comments
	- SECTIONS - A note can have a list of parts or sections
	- CRITIQUE - Each note has a critique note
- EDIT NOTE
- ACCOUNT MGT
- ALGORITHM
- 
# 2020 - DESCRIPTION
## = OVERVIEWS
### NAMES 
- WWB - What We Believe
- Spectrum of opinions
- Reality
- Informed Opinions 
- Mechanically unbiased
- Ungamable 
- What we observe
### KEY POINTS
- VALUE PROPOSITION 
	- SUMMARY - Provide a demonstrably - unbiased view of what society believes.
	- ANSWER - Quickly see what informed, open peers believe about an issue.
	- FACT CHECK - Quickly see most informed counter points for your favorite stance on an issue.
- KEY MOTIVATING INSIGHTS
	- MEANING - People want to make a difference/contribution if the output of the forum is valuable, people will be motivated to help create it.
	- HEARD - People want to be heard.  We each feel our thoughts on topics of interest will have value if they are heard by others (especially those that disagree).  If the forum does this for them, they will use it.
	- CONVINCE - People think they are right, and have something of import to share

### FACT CHECKER CANVAS
#### --- PROBLEM ---
	Provide accurate, accepted fact-based judgements about specific media as measured against some published rubrick
	_
### FORUM LEAN CANVAS
#### --- parts ---


	Lean Canvas Order of Exploration
	1 PROBLEM		Top 3 Problems
	2 SEGMENTS		Target Customer Segments
	3 VALUE PROP	Unique Value (& high lev)
	4 SOLUTION		Top 3 Features
	5 UNFAIR ADV	Things that cannot be easily copied or bought
	6 REVENUE		Revenue Streams
	7 COST			Cost Structure: Cust Acquisition; Distribution Costs; Hosting; People
	8 METRICS		Key Metrics
	9 CHANNELS		Paths to Customers

	Layout:  1 4/8  3  5/9 2  lower: 7  6


	~-~-~  [10-20-30](https://entrepreneurshandbook.co/guy-kawasakis-10-20-30-rule-for-perfect-presentations-129ba78d2fc1) 
	    Title
	    Problem/Opportunity
	    Value Proposition
	    Underlying Magic
	    Business Model
	    Go-To-Market Plan
	    Competitive Analysis
	    Management Team
	    Financial Projections and Key Metrics
	    Current Status
	_
#### --- PROBLEM ---

	Leverage the energies of authors with conflicting perspectives over some scope of human discourse to synthesize a concise, cohesive presentation of the range of thought on the subject.


	SYNTHESIZE -- Generate parsimonious, researched, supported "best" summaries of perspectives on important issues from which readers can quickly draw their own informed conclusions 
	 of key stances and beliefs 

	FACILITATE INFORMED OPINION FORMATION, VALIDATION, AND REVISION -- Provide readers with "best" summaries of competing perspectives on issues of interest in order to enable them to most efficiently derive a well informed personal opinion on the topic.

	BUILD TRUST -- Build reader trust in the forum mechanism itself so they can trust summary materials when forming their opinions based on summaries of all available evidence.

	MOTIVATE AUTHORS -- 
	of materials so they can confidently rely on their 

	ADJUDICATE -- provide ruling the larger population can trust
	_
#### --- SEGEMENTS ---

	Authors
	- Online forum style discussion platform users:
		Reddit, Quora, Medium

	Readers



	_
#### --- VALUE PROP ---

	MECHANISM & INCENTIVES

	- TURN DISCUSSION INTO SYNTHESIS -- 

	- BUILDS TRUST since majority cannot silence truth -- 

	- PROMOTES POLITENESS, FACTS & OPENNESS



	**MECHANICALLY UNBIASED** -- The forum itself is an entirely mechanical process that is designed to be resistant to gaming by determined and/or numerically superior subgroup of participants.  The system aims to present the full range of user perspectives down to some minimum perspective size with similar weight.

	**FREEDOM AND COHESION** -- Artfully integrate competing objectives of discussion freedom and discussion cohesion.

	**INCENTIVIZE SYNTHESIS** -- Strongly drive participants to author content the captures the essense of each subtree of discourse

	**SEPARATE CONSENSUS & CONFLICT** -- Drive disparate groups to try to find and agree upon the parts of an issue where consensus can be found, so system can highlight the bits of each issue that are truly fully conflicted ideas.

	DISCUSSION RESULTS IN AUTHORING

	For Authors
	- IMPACT -- Authors want to be heard, and they want to have impact on issues that they care about.
	- DISCUSSION -- Authors want to discuss issues that matter to them.

	For Readers
	- SYNTHESIS -- Readers want to quickly understand what conclusions certain others have come to regarding a topic after they have put large collective effort into undertanding all info relevant to the topic.
	- TRUST -- Readers want to trust the expertise, biases, and motivations of those providing syntesis conclusions.
	- AGGREGATION -- Readers want to quickly get the aggegated benefit of the many authors writing on a given topic.
	_
#### --- SOLUTION ---

	FORUM AUTOMATION MECHANICALLY
	- ENSURES THOUGHT DIVERSITY 
	- VISUALLY HIGHLIGHTS THOUGH QUESTIONS
	- INCENTIVISE PIECEMEAL SYNTHESIS -- Drive Users
	- ELEVATE CONVERSATION


	REDUCES BIAS == ENSURE THOUGHT DIVERSITY

	_

### Quotes

(2) I have a gleam in my eye.  The germ of an idea for a social network that uses simple algorithms to bubble up the most concise, informed, critiqued  summaries of human though on key topics along with the best critiques of each of these positions.  In my dream future, all citizens come to trust the site as being the "wikipedia" of human though on a topic.  In this dream world there would even be browser plugins that could flag content if it was refuted BY YOUR CHOSEN TRIBE.  in this future world I am not trying to get folks to listen to a different tribe, I only want to be sure each person is aware of the criticisms of the elders of their own tribe would levy against the content.  I think this might reduce the batshit crazy stuff the flys around.  I want informed reasonable conservatives to provide a check on conservatives, and the same for liberals.  (There is much more to this idea, but alas it does not fit into the margin on this page.) 

## = PRINCIPLES

COMPETITION&EGO DRIVEN

CONSCISE&POLISHED -- Goal is to have people compete on providing conscise and polished 

MECHANISTIC&UNBIASED
  Mechanistic (and thus position neutral) processing.
  Universal objective procedures applied uniformly to all positions
  on all issues.
COMPLETE
- Support (or opposition) on a submission only affects its promanence
  not its publication, all submissions are published.


## PRESENTATION 
### === Fact Checker Business ===
#### _

	    Title
	    Problem/Opportunity
	    Value Proposition
	    Underlying Magic
	    Business Model
	    Go-To-Market Plan
	    Competitive Analysis
	    Management Team
	    Financial Projections and Key Metrics
	    Current Status
	_
#### --- PROBLEM ---
	- Agendas really MATTER 
	- Every person and every orgnaization has an agenda
	- Thus there is no single person or org that all resonable people should trust

	_
#### --- SOLUTION ---
	- Explicitly balance competing agendas against each other mechanically in order to triangulate the truth

	_
#### --- SECRET SAUCE ---
	- Triangulation -- using many judgement from other agenda's 
		- Do you agree with your future self?
		- Do you change your mind when you dig deeper?
		- Are you consistent with others that share your agenda?
		- Are your judgements coorelated with your agenda (E.g. always in favor)
		- Are you consistent with high confidence judgements?

	_
#### --- BUSINESS MODEL ---
	- Non-profit community-based discussion board
	- Sell adjudicator services to social media companies

	_
#### --- GO-TO-MARKET ---


	_

## ODYSSEY
YEAR 1 - Discuss idea w/ many; Hack 1st proto; 
YEAR 2 - 
YEAR 3 - 
YEAR 4 - 
YEAR 5 - 

Questions:
- Will a motivated community grow around the goal of producing synthesis summaries?
- Would for-profit companies pay for assessments of their content and of their processes?
- Will I like a role of shepherding such a community and effort forward?

## 2020 - SOLUTION DETAILS 
## KEY IDEAS
### _

- **CONSTRUCTIVE** -- System is designed to support positive/constructive discussion and debate among folks with holding wildly different beliefs about issues of interest.  Just as with wikipedia today, ones privledges are tied to past behavior.  Those who are not constructive in their interactions are pushed down in visibility so the overall tenor of the debate can be quite pointed with folks strongly disagreeing, but doing so in constructive ways.

- **TRIANGULATION** -- using feedback from multiple authors to auto test accuracy, bias, etc. in them.

- **PLACEMENT** -- algorithms that use history of stances and support to 'place' both stances and authors on various spectrums.  (e.g. conservative vs. liberal, or control-focused vs. freedom-focused)

- **COMPRESSION** -- succintly, accurately, unbiasedly summarizing some content using some much shorter content.   (e.g. a one sentence compression of 50 studies on a topic might be:  most studies had small sample sizes, among those with larger sample sizes a slight (.2-.6%) increase in death was observed.)

_
### --- MISSION ---

Build an organization whose judgements can be trusted by all since they are informed by a range of agendas

#### Take all concerns of all groups seriously
Today we very often take shortcuts.  Based on our own agenda, we decide which questions should be taken seriously, and what kinds of analysis are appropriate for analysis.

This leads to a predicable consequence: those who don't share our agenda don't listen to our results.

It may seem prudent to NOT indulge "wrong" concerns and "wrong" approaches since one lends them creedance in even discussing them.  But in the end this only fuels more suspicion about our results and our agenda.


We propose an alternate path:
-- take every serious concern of every reasonably sized group seriously.
-- honestly take conclusion as valid and try hard to support those (sometimes crazy) conclusions
-- attempt to incorporate most every form of evidence which that group finds valuable
   see how far those arguments can go, THEN contrast those with the best counter arguments

e.g. AVOID "shut down" style arguments as a sole




 
_
### --- PLACEMENT --- 

- Use self-reported question answering against a short-fixed questionare in order to get an initial assessement of author's placement on relavant axes.  (e.g. ask them if they are a conservative, and ask if they support gun rights etc.)

- Use "hubs and authorities" style of iterative assigment algorithm to provide assignments of both authors and stances to places on different axes of relevance.

- Algorithm select placement assignments that minimizes the conflict between placement of authors and their supported stances.

Output is a numerical score for each author and stance on each axis of interest.

Users can see, but not edit their scores.

_
### --- TRIANGULATION ---

Steps:
1. High accuracy, disinterested authors are presented with specific adjudications in order to offer an opinion.
2. Both past and future authors with an interest in this topic are presented with the same adjudication for consideration.


The degree to which an author's adjudications are DECORRELATED with their personal support for stances on related issues is a measure of the level of adjudication BALANCE they have.

The degree to which they adjudications match consensus adjudications overall is a measure of their ACCURACY.


~-~~-~~
_NOTE about unbiasedness & accuracy:  One may have a strongly biased opinion over some area (e.g. be strongly biased towards liberal or conservative stances) and yet be an accurate and unbiased adjudicator over those same issues.  Such an unbiased accurate adjudicator will judge AGAINST content whose conclusions match stances that they support in cases when the content is not logical, not supported by evidence, etc.  So we seek to measure adjudication bias and adjudication accuracy independent of personal bias towards/against particular placement positions._

~-~~-~ 
**How this works in detail**

1. Content is continously adjudicated as it is read.  Any reader is invited to offer constructive feedback on the content they read.  Feedback should be written in a constructive, positive, and as narrow a way as possible even when one is commenting on stances that one strongly disagrees with.
	- When possible feedback should be tied to the phrase, sentence or paragragh to which it applies.
2. Authors are given opportunity to edit their phrasing etc. to "fix" the issue raised... even if they dont fully agree with the criticism if they can make an edit to make their audience "happy" they may do this, just to minimize the number of highlighted issues raised with their content.
3. Each point of feedback on each stance is itself a stance and others may agree or disagree with this feedback itself.  A stance on a stance is called an ADJUDICATION

highly rated stances are the ones most often shown to a viewer when they look at some content area.  this provides opportunity to agree with or disagree with each stance (and feedback stances on stances too)

4. Over time we the system continuously measures the level of bias occuring within the adjudications over each piece of content.   e.g. if many conservatives rate some content as illogical while many liberals rate it as logically sound then the system knows is bias happening in this adjudication.  This is known since the adjudication stances are splitting strongly based on the author's conservative-liberal placement we KNOW there is high bias involved on one or both sides.... but we don't how much is happening from each side.

In these cases the system secretly forwards this same content for review by DISINTERESTED users (those that are not strongly conservative or liberal), or those who have high IMPARTIALITY scores.

These triangulation scores provide mechanical feedback on the biases of the specific adjudications provided earlier in the process.

This process DOES require strongly impartial folks at the top of the pyramid.  That is ensured via other means.  Triangulation itself is a mechanism that amplifies the scope of impact of those smaller number of impartial judges to have a ripple effect over the entire population of adjudicators.

~-~



Assuming that we get strong consensus from this adjudication set, then we have a good indication of what the ground-truth regarding the adjudication of this underlying content.  We even have the written consensus commentary from these adjudicators.  

The system then uses this consensus adjudication result to look for BALANCED respondants that may nonetheless may place as strongly liberal or conservative based on their supported stances.  Such users were willing to "go against" their underlying preference and adjudicate this content in a way OPPOSITE to their underlying bias on the issue.

Such reversal cases are really the hallmark of an UNBIASED adjudicator.

And pure consensus matching is the hallmark of an ACCURATE adjudicator.


Not only do we provide a bias/accuracy score for each user, we also can provide human written commentary to back up this score.

we can show a piece that you scored as logical and empirically justified right along side consensus views that agreed with you when they did, AND show the consensus comments that disgreed with your assessment along with human written justifications.


~-~~-~~-~-~
OUTCOME: My thinking is that one would not need to frequently adjudicate-the-adjudications in order to really change the dynamic of providing feedback in this system.

I think many folks pride themselves on being UNBIASED and ACCURATE, and they are going to COMPETE to get good scores on these measures.  But in order to get a good score, they are going to have to NOT overshoot or undershoot when giving feedback.

~-~~-~~-~~-~-~
OUTCOME OF THE OUTCOME:  Once I have have a subset of users that are biased liberals and strongly biased conservatives who are nonetheless UNBIASED and ACCURATE adjudicators we are in good position to generate quality outputs.

Now among strongly supported liberal content and strongly supported conserviative content we and let the CONSERVATIVE users select the most accurate, empirically supported, etc. stances FOR their conservative position, and those same users can provide the most accurate commentary on the accuracy and empirical support for liberal stances which they may disagree with.

_See what we are doing here?_
Creating consensus voices of reason that all can trust and listen to:
We are creating/identifying users that a conservative can trust to review and frame stances from both sides.  At the same time we have liberal users that liberals can trust too.

Both liberal and conservative reviewers were selected for thier willingness to call BULLSHIT on their own side -- even if they are strongly biased towards one side or the other mathematically we have aimed to ensure their adjudications are not biased!

And I think there voices can best be HEARD by all users.

Afterall if I know deep deep deep in your heart you are a trump supporter, but still you are calling bullshit on some argument... well now I have to pause... maybe it IS bullshit.


~-~~-~~





_
### --- EDIQUETTE / CONSTRUCTIVITY ---

Feedback on non-constructive 


_
### --- ADJUDICATION ---
### --- MECHANICALLY UNBIASED ---

The system provides a collection of judges:
1. Whose biases are mechanically selected to reflect the biases of the population at large, 
2. Who have been shown to render impartial judgements as measured mechanically in comparison with their peers
3. Who show significant engagement with the platform in rendering opinions.


- The tool focused participants on important questions of bias and of fact, as suggested by the participants and as suggested by sponsors.
- Participants author opinions on these questions, and then discuss and refine those opinions in a collaborative fashion.  
- The collaboration itself is monitoried and participant's ability to author material and render judgements which stand the test of time, and stand the test of subsequent scruitiny, serve as a measure of their impartiality.
- Importantly only the judgements of others with similar overall biases are used to assess ones impartiality.
- System actively canvases to gain participation across the full spectrum of social opinion within the population.
	- Multiple surveys are used to model the larger population, as well as the paricipant population.
- Mid-tier judge selection.
	- UNIFOMLY REPRESENT POPULATION -- Are selected at random in order to uniformly match the demographics of the larger population.
	- HAVE HIGH IMPARTIALITY AND EXPERTISE SCORES -- 
	- HAVE MODERATELY HIGH ACTIVITY 
- Senior Judge selection
	- VERY HIGH IMPARTIATLITY SCORES
	- AGREE TO BE PAID AND GET TRAINED
	- GET TRAINED TO FOLLOW A GIVEN STANDARD OF JUDGEMENT


SCORE DRIFT
- CONSENSUS SCORING -- No impartiality scoring when there is a lack of consensus.
	In order to avoid any kind of scoring drift anytime there is a lack of consensus regarding a particular judgement, the adjudications in that case are not used to update anyone's impartiality scoring.  In this way we avoid cases where the entire set of judges impartiality scores somehow skew over time into some self re-inforcing pattern. It is ones ability to render adjudications reliably in cases where a clear consensus exists that provide the sole measure of ones impartiality.
- RANGE FLATTENING -- Knowlege of consensus scoring itself can provide an incentive towards watering down owns own judgements in order to avoid ever going against a consensus ruling and their by loose impartiality points.  This perverse incentive could cause an undesirable uniform squeeze towards the middle as judges shy away from adjudications that they feel might result in a consensus against their opinion.  In a worse case this could yeild a pressure towards a reinforcing pattern of systematic mis-judgement towards the center.  To counter this effect an explicit flattening weighting is applied to the impartiality updating.  We ensure that any such tendency is countered by over weighting cases where one renders judgements in favor of the edges.

_
### --- OVERVIEW VALUE PROP ---

THE SYSTEM IS RIGGED NOT TO BE RIGGED
- When consensus statements are presented with a high degreee of validation, once can be confident they truely do represent a consensus of those that share ones own bias and are vetted by the most impartial among those sharing that consensus.



_
## UX - USER EXPERIENCE
### AUTHOR EXPERIENCE INSIDE OTHER FORUMS
- JUST POSTS - Author simply participates in external forum (like sub stack, medium, or reddit) 
- MINIMAL POSTING - Anyone in these forums can participate by simply replying to a full posting with a response that begins with one of the minimal intentions:  (e.g. +1, -1, or feedback)
	- Over time this user responses are collected to impute the cluster they belong to.
- FULL POSTING / REGISTERED ACCOUNTS - 
	- USER ACCOUNT - A full postings can only be created by a user registered on the forum.  This is a one time process where a user 'lays claim' to one or more external reddit, medium, etc. accounts.
		- They use these accounts to post a note that ties back to their FORUM username.
		- They also fill out a short survey used to slot them into appropriate clusters.
	- EXPANDED INTENTIONS - (maybe all intensions can be can be done w/o an account)
- BACKLINK HEADERS
	- Forum headers can also be expressed as a backlink which will open the relevant forum page
	- Forum links are designed to be manually or automatically created.
#### HEADER DETAILS
When posting the author uses a text header to indicate their forum action.
Examples include:

-1 for [Ukraine Conflict, Caused By, NATO Expansion]
- Soon if 'NATO expansion' gains sufficient weight within the 'Ukraine Conflict' area it is shortened to

-1 for [Ukraine Conflict, NATO Expansion]
- When presenting info about this note, this newer shorter name would be preferred.
- The original name will remain valid, the shorter name will now just mean the same thing.

-1 for [NATO Expansion]
- If the topic of 'NATO expansion' being a cause of the Ukraine conflict itself becomes a massive topic of discussion, a new even shorter name would be added.

-1 for [Ukraine Conflict, NATO Expansion]
- One month after the topic of 'NATO Expansion' cooled off, it very short abbreviated for would stop being the preferred displayed form.  Still all names would continue to work as expected, only the preferred name changed at one month.
- SIX MONTHS after the preferred name changed, that old name would stop working as a target
- SIX MORE MONTHS after it stopped working that short name would be recycled and become available to potentially refer to a new note grouping.


### --- DATA PRESENTATIONS ---

NOTE VIEW -- Viewing a single note
- BODY -- A note's hook, body, parts and links are rendered for view according to the markdown provided by the note's data contents.
- COMMENTARY -- Leading commentary for a note is provided as annotations within the text of the note, and as a list of comments in the right margin by the note's content.

HOOK VIEW -- Area notes do not display their contents only their hook is shown followed by leading commentary


LEADING COMMENTARY -- List of notes that maximize coverage

_
### --- Reader User Flows ---

GOOGLE SEARCH
A reader who is interested in a particular topic does a google search for material on this topic and finds themself on one of our content pages.  Reader does not know about the forum or its purpose, they have just clicked.
- User sees a wikipedia style page that describes a particular area of human interest along with a taxonomy of sub-topics for this area, OR
- User sees a single issue page that describes a particular issue upon which different authors might express different stances toward the issue.  Along with a summary of the major stances taken along with links to the best representatives of each.
- User sees a single stance taken on a particular issue.


BLOG READER FLOW
A user of the forum decides they would like to stay abreast of discussion within areas of their personal interest.

_
### --- Reader Interface ---

The markdown text view of a note's content is shown with bold, italic, and heading text rendered appropriately, along with tree of subnotes if a note has them.

Just as with MS work or google docs this basic text is marked up with different kinds of highlights on the text representing feedback from others on this piece.  Because a give popular piece may have many many comments on it, these are generally reduced to the most supported comments first.  Still since we care to get adjudication on less popular feedback we to randomly show all feedback, at least on occasion.


PARTS
- CONTENT PANE -- 
- FEEDBACK MARKS -- 


FEEDBACK MARK TYPES
- UNDERBAR -- critical marks shown as red underline
	- UNDERBAR "J" -- 
	- UNDERBAR "
- OVERBAR -- postive marks are shown using a green overbar

_
### --- Reader Feedback ---

- HIGHLIGHT -- Reader optionally selects a range of text to comment upon.
- START -- Using a keyboard shortcut or hitting the pencil icon a user may provide commentary or feedback on this selected content, or upon the entire note.
- CHOOSE TYPE -- Using key or click the user then selects the type of feedback being provided from the feedback types list.
- BODY -- The 

_
### --- Feedback types ---

- COMMENT -- Reader is just providing discussion about a given topic.
- JUSTIFICATION -- Reader is claiming that some part of the content is making a claim that requires justification beyond what has been provided.
- CLARITY -- Reader 
- BIASED LANGUAGE --

_
### --- Content Markdown ---

" # "	INLINE SUB HOOK MARK
" . "	REFERENCED SUB HOOK MARK
" [] "	REFERENCED HOOK MARK
"-->"	CONCLUSION MARK
"_xx _"	ITALIC MARK
" + "	AGGREGATION INCLUSION MARK




_
## DATA-FOR-NOTE
### -- Note Fields --
- PARENT 	--  Link to the note where the author feels this note belongs
- LINK		--  Relationship between this note and its parent
- SELECTION	--  Interval of parent reference by this note (OPTIONAL)
- OWNER		--  The user / usergroup that controls this note
- ID			--  Unique ID for this particular note/post
- NUMBER		--  Unique version number for this partiular 'post' or 'edit' of the note
- NAME		--  Nickname for this note (OPTIONAL)
- HOOK		--
- BODY		--  Markdown string that includes part and link references.
- PARTS	--  
- LINKS 	--  In markdown
- ATTR	--  Key/Value attributes specified in the note's body

#### -- Hook Name -- 
NOTE COHORT - A note cohort is a set of notes that all represent the 'same thing' as indicated by their authors.  (see cohort discussion)

HOOK NAME - A hook name is a human understandable identifier that refers to a note cohort.
- HOOK PATH - A hook path is the sequence of hook parts that make up a hook name.
- HOOK SCOPE - The set of notes under a particular cohort of notes
- HOOK PART - An alpha-numeric identifier that is ignores case, spaces, and punctuations.
	- A part only has meaning within a sufficiently narrow hook scope
- HOOK FORMATS
	- BRACKETED FORM - [this, is a, hook]
	- DOT FORM - this.isa.hook
	- URL FORM - https://forum.com/this/is+a/hook
	- INTENSIONAL FORM     +1 for [this, isa, hook]
- BASENAMES - an author and community agreed abbreviated names for the note group.
	- WEIGHTED - The more popular a note is, the shorter its abbreviation can be.
		- Can only be changed by significant amounts when it popularity spikes
	- TIME VARYING - Name looses status if its popularity wanes by significant amount and time.
	- REDUNDANT - 
- PATH NAMING - One can only use a basename in a context where it is unique (80% of the weight is in one node) otherwise it must be prefixed with the derived name for its parent.


JUSRISDICTION -- Each jurisdiction has its own root and all descendents of that root operate according to the rules of the jurisdiction.

_
### -- Note Types & Link Types --

INTENTION OF THE CHILD NOTE 
- AREA		--  Defines a sub area of social discourse that discussion may be "inside of" or not.
- ISSUE		--  Defines a specific issue upon which different people might adopt difference stances
- STANCE		--  Defines a specific stance on an issues that people might support or oppose
- PART		--  Defines a 'part' of parent intended to be referenced explicitly within parent body.
- +1			--  Expresses a supporting "vote" for its parent stance, issue, or area
- -1			--  Expresses an opposing "vote" against its parent stance, issue, or area
- FEEDBACK	--  Provides structured feedback based on one of the enumerated discourse rubricks
- POST		--  Author is contributing to discussion about the parent note

NOTE RELATED
- COHORT 	-- The subset of a notes children that have the same intention and selection
- CONSENSUS	-- A designation on a note intended by its author to summarize all peers in its cohort


INCLUSION TYPES
- EMBEDDED	-- The full text of the note is embedded in it parent
- HOOKED 	-- Just the hook text is shown on its own lines, and is expandable to full note
- FOOTNOTE	-- The name of the note is included inline, and its hook text is included as an 		
		  associated, clickable footnote at bottom of content.
- INLINE		-- The hook-text is included as a clickable inline text.
- IMPLICIT	-- The note is not referenced by the parent's body at all, so it is listed below body

LATER 
- ALIAS		--  Defines an alias for one or more nodes that redefines 


_
#### ,
- PART	-- Author views this node as a part of its target node 
- == 	-- Author views this node as an alternative (of same type) for its target node

- CHILD
- PART
- REF
- HOOK	-- shows hook only
- ALIAS
- **ALT**	-- Author views this node as an alternative (of same type) of its parent node
			Delta:	Title describing goal of this ALT
- **ALIAS**	-- A note that stands in for another note
- **PART**	-- Author views this node as a subpart (of same type) as its parent node.
- **HOOK** 	-- Author views this node as taxonomic sub-area of its parent's domain node.
			This node defines some domains or area of human discourse
			text: defines the boundaries of the area (what is in/out)
			parts: incomplete list author views as "in" area
- **ISSUE**	-- Author views node as question or topic upon which one might have a stance
- **STANCE** -- Author views node as a stance, a position or perspective on an issue 
- **VOTE**	-- Author is asserting relative support (or anti-support) for item
			aspect: specifies the aspect being voted upon:
				support, cooperative, accuracy, consistency
- **CHAT**	-- Author adding commentary or discussion about the parent node.

_
### -- Note Tree --

**DISCOURSE TREE** -- Tree of all notes and posts

**NOTE** -- A a single cohesive, versioned piece of content that was contributed created by a specific user at a specific monent in time.

**COHORT** -- The set of children notes that share a common: parent, intention type, and parent selection.
- Each note is a member of exactly one cohort and each cohort is a child of exactly one parent note.
- Thus each notes and cohorts form an alternating tree.


_
### -- Note Coverage --
	
**COVERAGE LIST** -- The "cover" for a note is the sequence of child notes that best cover all user perspectives.

- **CHILD** -- x is a _**child**_ of y, if the parent of x is y.  Thus any user may add a child to any other note (though the added not may have little or no support).

- **VOTE** -- A vote note simply lists zero or more stances support by a given user on a given issue.
The user (or user group) may change their votes by simply publishing a different vote at any time.


**COVERAGE ALGORITHM** -- Computes the sequence of note children that best cover the range of user stances for that note.
- Once a child note supported by a user is included on the coverage list, the algorithm considers that users perspective "covered" and no longer considers that support for subsequent notes.
- At each step the algorithm consider computes the level of support for all remaining children notes.
The number of "uncovered" users that have expressed any level of support for a given note.
- The note with the highest level of support is added to the coverage list, and covered users are removed from the list of "uncovered" users.



**NOTE CHILDREN**
Children(x):
	BestCovering( {n in NOTES where n.hook = x} )		if x is a Hook
	x.parts												if x is a Note


**COVERING ALG**
Given: list of children, CHILDREN, and list of supporting users, USERS, th is the min threshold 
Let REMAINING <- USERS, TOP <- []
Loop:
	For c in CHILDREN\TOP: supporters(c) = count of users in REMAINING that support c
	let t = argmax{ i in c of  len(supports(i)) } 
	if len(supporters(t) < th * len(USERS) then return TOP
	TOP <- TOP + t
	REMAINING <- REMAINING \ supporters(t)

**AGGREGATIONS / OVERVIEWS NOTES**
A note may aggregate or summarize many other areas/stances into a single summary note.

Anyone can cover many areas or many stances with a single summary or aggregation note.
_
#### -- How data changes over time --

HOOK CHANGES
- Any change to any hook string will cause ALL hooks containing that string to be updated.
- Authors of those sub hooks will be prompted to accept the new positioning, if they reject it, then they will keep the original parent hook.
- Authors whose hook change results in many rejections are noted as being non-constructive
	(they should have created a NEW hook instead of editing the old one, and allowed folks to migrate)
- Frequent hook changes with many children are also considered non-constructive

_
## DATA FOR USER

**USER** -- those that author and read the forum notes.  Each note:
- Has a unique username for identification
- Has a profile note "USER.username" that announces existence of this user and provides profile info 
- Has a public/private key pair used to identify user (part of the profile note)
- Has different privileges within different user groups.
- Has authoring control over all notes whose name is prefixed by their username

### -- User Groups --

**USER GROUP** -- a grouping of users that share some common perspective, goals, or association.

Each group:
- Has a unique groupname for identification
- Has has a profile note "GROUP.groupname" that announces the existence of this group & provides info
- Has a list of senior members listed in the note "GROUP.groupname.board"
- Any board member can control any aspect of a group.
- Each board member can unilaterally execute any action on behalf of the group.
- Board membership is the only group property that requires a majority of group members to sign in order to execute the change.

Change of the board.  
- A majority of current board members must support any edit that changes the list of board of directors.  
- Thus the system will not accept any change to the note "GROUP.groupname.board" until that note has the support of 1/2 of the current board.  That is until there are sufficient notes: "USER.username.VOTE.group.groupname.board" that support a particular new slate of board members.
- Thus it is important the groups keep their board member lists up to date, since the algorithm will require 1/2 of all listed accounts (even if those users are no longer active).

### --- User Data Fields ---
- HANDLE
- UUID
- PKI

- UNBIAS
- ACCURACY
- POLITENESS
- CONSTRUCTIVENESS

- BIAS
- ILLOGIC
_
## DATA FOR HOOK

**HOOK** -- A sequence of phrases used to identify a specific "place" within the space of human discourse

**Properties**

- **TREE** -- All Forum Data is organized into a single global tree of hook phrases

- **COVERAGE BASED VISIBILITY** -- When viewing a node in the hook tree, those notes with greatest coverage AT ANY LEVEL below this hook level are presented in coverage order w/o regard to their depth.
- SUMMARY NOTES -- Authors may summarize a range of notes within a kind of overview.  Once enough users that support the notes summarized by a particular note also support the summary of that note, then the sub-note will not be visible, except as listed in the summary note.
- AGGREGATION HOOKS -- Any hook that begins with a "+" is an aggregation hook.  These hooks are not presented as alternatives to each other, but rather as different instances of the same kind of data

- **CONTROL** -- Who can edit what
- HOOKS -- controlled by their author (at each level in the tree)
- NOTE -- controlled by their author
- SUPPORT -- Each user controls the support the provide to each note.
- VISIBILITY -- is indirectly controlled to maximize coverage of most supporting users

- **CONSENSUS GRAPH** -- A majority in of high constructive members of all major placement groups must agree on adding or editing the consensus graph.  In rare cases one can create a placement specific hooks when different groups disagree.  e.g.:  liberal-XXX and conservative-XXX
Consensus graph construction minimizes conflict by not defining any intrinsic meaning for any given hook phrase, and by being permissive in allowing varient entries.  Each community can utilize their own taxonomy, and these alternatives can be combined and recombined as needed.

_
### --- HOOK DATA ---

**HOOK PHRASE** -- A short string of text identifying a conceptual "place" in space of ideas

**HOOK** -- A sequence of hook phrases

Properties: 
- TREE -- hooks form a tree
- PARENT -- the parent of each hook note is another hook note
- CONTROL -- Each hook note is controlled over time by its author.  
- AGGREGATION HOOK -- A hook phrase the is prepended by a "+".  These phrases are aggregated when included within the body of another note.
- UNIQUENESS -- If two (non-aggregation) hooks at within a sub-tree are identically named, then the largest distinguishing user group name covering its author and its supporters is pre-pended to its display form in order to differentiate it:
conservative: summary    liberal: summary

- Each hook string is associated with a parent hook, 
- ??? and other side up linkages
- Author of a hook has full control over that hook

### -- Data Tree Presentation --

Presenting a place (a hook) within the data tree is done by 

- NOTES PRESENTED -- Consider all notes that link up to this parent.
- BEST COVERING -- Select best covering regardless of depth of note

_
## HOOK NAMING

Mostly participants don't think a lot about naming
They just use the naming that others are using, and when needed they create their own new names.
Over time a small list of repeatedly used names becomes the 'standard' names for each area
Popular area will have very short names to reference them.


CORE IDEAS:
- HOOKS ARE NOT OWNED OR CENTRALLY CONTROLLED - No one 'owns' the meaning of a given hook name.  They are created, but then each takes on the meaning that the community collectively ascribes to that hook by the notes which are attached to it.
- HOOK PATH - All hooks derive their meaning from the full path of parent hooks above it.
- MEANING THRU USAGE - A hook means all the notes attached to it.
- EMERGENT USAGE BASED NAMING - In order to tell a cohesive story participants naturally reuse popular/meaningful names.


#### NAME CONTENTION - Resolves 'name gaming' 
- POPULAR - The most popular summary for a hook is adopted as the meaning or definition of the scope for a hook.
- VESTED INTERESTS - If a hook represents a stance, only those supporting the stance may vote on its summary meaning.
- PART CONTENTION - If the most popular summary has more than one '-1' vote for every three '+1' votes, this name is retired and the '-1' and '+1' voters each select a new PART for their groups.

#### SHORT FORM - Shortened names

- EXTENDING 

### HOOK EMERGENCE
- CREATION - Within a scope many part summary pairs are created and maintained by interested authors
- SUPPORT EMERGENCE - '+1' support causes a few names to rise above the others.
- SPLIT - The '-1' voters on a note naturally form a splinter group of voters and are the only ones elegible to vote on an alternative name+summary pair.  This process can be repeated to further excluding ever more voters until homogeneous group of voters is achieved.  (splits are encouraged but not required to adopt alternate names)
- PREFERRED MEANING - Once sufficient weight (+1 votes) and cohesion (lack of -1 votes or lack of votes for alternative summaries) a part name can become the preferred meaning for a



### HOOK CONTENTION

- MOST POPULAR NAME+SUMMARY PAIR - Supporters of a hook vote for the most supported summary note for that hook.
- VOTERS - Participants that support a stance, or a note within the scope of an area are eligible voters for that hook.
- CONTENTION - Contention occurs anytime a summary has more than one-in-four '-1' votes with more than 20% of the eligible voters participating.
- SPLIT - When contention occurs voters are split 
- with more than 25% '-1' votes is under contention
- CONTENTION - Contention occurs when 

## DYNAMICS
### ...

**DYNAMICS** -- The interplay of the forum's software behavior against the agendas and behaviors of the range of readers, authors, and board-members.

Achieving our aim of driving productive discussions towards concise, unbiased summaries of human thought will require careful tweaking of forum execution rules inorder to drive the intended dynamics.

The three big dynamics control mechanism in the forum are:
- **ETTIQUETTE** -- written rules of ettiquette for guiding participant behavior.
- **COVERAGE** -- mechanism providing increased visibility for content that captures competing consensuses
- **ADJUDICATION** -- a back-stop mechanism for driving constructive behavior.

_
### --- ETTIQUETTE ---

**ASSUMTIONS / OBJECTIVES** -- Form system behaviors are designed around the following assumptions/goals for user behavior:
- **CONSTRUCTIVE** -- Users are expected, encouraged, and occasionally forced to operate behave in contructive ways within the Forum.  
- **GENTLE** -- At all levels within the Forum the least aggressive responses are taken before harsher measures are taken.  For example a private note from a senior member of your community is less aggressive, than a note from a conflicting community, is less agressive than temporary reduction of priveldges is less agressive than longer term reduction in in privledges is less aggressive than outright banning.
- **COMPETITIVE** -- While are expected to be polite and constructive, that does not preclude them from being pointed/blunt in their attack of an idea.  Further they may reason and collaborate strategically in order to get views they believe in to have greater visibility, as long as they are not engaging in "non-constructive" strategies to achieve this end.  (see rules of etiquette)
- **EGO-AWARE** -- In many cases both and author's ego and an author's identity may be a significant motivator for their participation within the forum.  This reality is understood, accepted, and to a degreee supported.  At the same time such ties to ego and identity can foment many non-constructive behaviors.  Thus the etiquette and forum operations (specifically ownership/versioning rules) are designed to balance desires of ego/ownership against productive forum outputs. 

**RULES OF ETIQUETTE** -- Community expectations are written out, complete with examples, so users understand and can understand and encourage the right levels cooperations/competition.  There will be opportunities to "hijack" the system and boost ones views beyond the support they "naturally" should have.  The community itself can consider the pros and cons of different rules and adjudicate this list over time to evolve constructive / productive / fun / engaging / blunt / polite interactions.

As the system evolves it is possible that different subcommunities might want to split into different regions allowing different different rules of ediquette to apply.  This would be adjudicated just as any simple rule change would be adjudicated.


**SOME EXAMPLES OF LIKELY ETIUQUETTE RULES**:
- Creating duplicate / fake user accounts strongly violates eiquette and if found should have more agressive conseuqences (perhaps for all accounts)
- Falsely claiming support for positions in order to pervert the voting within those communities would also be a break with etiquette.
- Cursing in ones messages might be a light violation of etiquette.  Ad hominim attacks might also be a violation.

_
### --- COVERAGE ---

COVERAGE -- The bottom-up method to ensure subtree summarizations were unbiased and representative of the diversity of user opinion within the indicated subtree.

_
### --- ADJUDICATION ---
#### ...

**ADJUDICATION** -- An _**adjudication**_ is a human judgement (stance or vote) regarding some specfic human generated content. 

The forum automatically combines feedback from multiple readers to form a consensus adjudication on the underlying content.  These consensus adjudications do not assess the conclusions drawn by the underlying content, rather they focus on judgements about several carefully prescribed dimensions expected of constructive human discourse.  Does the level of certainty expressed in it assertions match the level of supporting data or links provided?  Is it compressive?  Is it emotionally constructive discourse?  Does it use biased language?

Having precise measures for these dimensions is difficult and not necessary here.  Instead what is needed are rubricks of behavior which are clear enough that well intentioned participants can at least agree when instances of egregious violations of contructive discourse have occurred.

Arriving at these consesnus adjudications on behavior allows the Forum mechanism to take the minimially-disruptive actions to discourage these non-constructive behvaiors.  The whole point of the Forum is to avoid systematic bias, thus any global-unilaterial mechanim must be viewed with extreme suspicion lest that mechanism be used to squelch legitimate diversity in human thought.  At the same time, various trolling behaviors must not be allowed to be an effective way to amplify ones visibilty, lest it become a dominate dystopian mode of discourse.  Adjudication rules must thread carefully between these two pitfalls.



These adjudications are key to having "good" content bubble to the top, and having non-constructive, non-justified, etc. content drops into the background.  Notice no attempt is made to have certain stances drop to the background...  if a sizeable fraction of the readership supports a stance then it is listed up front, it is just that the "best" version of that stance which is listed up front.


**AN EXAMPLE**:  No attempt is made to draw mechanised consensus about the correctness of any stance taken by authors.  (e.g. Are masks effective against Covid?)  Some will have stances in favor while others against.  The system itself supports no one opinion over another intrinsically on this or any question.  Readers will however provide adjudicative feedback on the content they read.  Suppose there are several stances expressed in favor of masks and others against them.  Readers will provide feedback intended to improve these stance statements.  This feedback (if valid and ignored) also serves as a kind of adjudication against that content.  These automated adjudications provide a way to "bubble up" good versions above versions that make emprically founded assertion, or use biased language, etc.

_
#### -- Adjudication Dimensions --

**ADJUDICATION DIMENSION** -- One of the enumerated aspects of human discourse identified within the Forum as so important that significant violations of them must be flagged and eventually acted upon in order to keep Forum discourse constructive and productive.

Dimensions include:

- BACKED -- A measure of the degree to which assertions made in some content are backed by the supporting material for that content.  So if an author states "I believe wearing masks to be an over-reaction to covid" this should get a high support score since each person is an always an expert on their own opinion. If they wrote "wearing masks is an over-reaction to covid" this would only get a high support score if they had strong empirical studies showing such a result.  Otherwise the would need to say something like.  "I know lots of folks not wearing masks and they dont appear to be getting covid".  This could receive a strong support score, but is clearly a weaker statement.

- COHESION -- A measure of the degree to which some content forms an internally consistent coherent whole.  In the case of an argument, it is the degree to which the argument is logical in the connections it makes.

- BIASED LANGUAGE -- A measure of the amount of language biasing being done.  Biasing refers to the use of emotion laden descriptions in cases where neutral alternatives exist.

- CONSTRUCTIVE -- A measure of the degree to which content is intentionally aimed at subverting execution of the forum mechanism in some way.  e.g. 

_
#### -- Adjudication Rubricks --

ADJUDICATION RUBRICK -- A written description of what constitutes good and bad content according to this dimension.  

The rubrick should be precise enough that well-intentioned individuals adopting dramatically opposed stances on issues of common interest can nonetheless generally agree on adjudication of content within their area.  This need for general agreement on adjudications means that the adjudication dimension must be limited to areas that are narrow enough and can be made precise enough that well intentioned individuals can agree on the dimensions even when they disagree on their support for the content itself.


EXAMPLE:
For example well-intentioned individuals who are both for or against particular groups should agree that refering to those groups as "freedom fighters" or "terrorists" is a more biased adjective than callin those same groups "combatants".  Notice these groups likely can agree that both freedom fighter and terrorist are more biased than combatant even though they would strongly disagree on which biased term was more appropiate.

Each rubrick is defined by a totally ordered scale of adjudicative stances one might take.
For example, the bias dimension might be broken into a four point scale:
- clinical -- content is studiously choosing to use the absolutely least biased terms for all aspects of the content.  
- neutral -- generally using neutral, non-emotionally loaded words to describe concepts
- biased -- readers viewing the words in this content can clearly assess the bias of the author by looking at the descriptors and language being used w/o its surrounding content, and alternatives do exist which still convey underlying meaning w/o bias.  Still author appears to be attempting to reign in their biased language, and while biased this langauge is not factually without merit, nor chosen singualarly for its emotional connotations.
- flaming -- Author is flagarently using biased language with little regard for neutrality.

This rubrick would includ dozens of specific examples of text that had been adjudiated to global consensus on what kinds of language consitute specific levels of bias.

_
#### -- Adjudication Voting -- 

VOTING -- just publish a note 

+feedback

_
#### -- Adjudication Consequences --

Our objective here is to take the smallest possible corrective action sufficient to keep the Forum fairly constructive.

Corrective Actions:
- WITTEN FEEDBACK -- Notice the act of voting on a dimension is tied with the act of simply providing feedback to that user about their content.  This encourages the first key line of defense is constructive non-corercisve feedback on how to improve their content.
	- PRIVATE FEEDBACK -- The lightest possible consquence would be providing private feedback.
	- PUBLIC FEEDBACK -- The next level 
	- MODERATOR FEEDBACK

- SCORE FEEDBACK -- Once enough readers have provided public feedback the system will score content according to the readers average feedback

- CONSENSUS FEEDBACK -- 
_
#### -- Adjudication Automation --

ADJUDICATION AUTOMATION -- Once defined levels of consensus in adjudication against some content, some user, or some usergroup, the Forum itself will take defined corrective action.


IF-THEN-RULES:  Just as with etiquette rules these rules are written down both in english as well as in forum executable "if-then" style.  consequences can range from simply downgrading an items score, to reducing its visibility, to downgrading priveldges, and finally to deletion or banning.

RULES OF ENGAGEMENT:  The collection of if-then rules used by the forum to operate the forum are called the rules of engagement, and they are an agreed upon computable adjudication-action plan that serves a the last line of defense in driving constructive forum discourse.

ADJUSTED BY COMMUNITY:  Also just as with etiquette rules, these rules would be adjusted over time by community consensus.  Consensus on rule updates would themselves automatically be adopted by the forum software as the communities achieved sufficient consensus on particular rule change.

POTENTIALLY BIFURCATED:  The rules of engagement are coercive.  They ultimate can ban a user or remove content.  Some communities may want to keep a tighter reign on discourse, while others may want to allow a more rough and tumble play.  

SUBTREE JURISTICTION:  the root of the discourse tree is adjudicated according to the root or default adjudication rules.  But any child topic area created may specify a different adjudication jurisdiction by simply referencing this new adjudication automation document.  Any content within this subtree will then play by those new rules.  Just as with every thing else in the forum, if the larger community does not want to play by those rules, they will "vote with their feet" and create and move to that same topic to a jurisdiction where they do like the rules.  (see eyes and feet)





, and it is possible that different domains of discourse would want to adopt different form of adjudication automation.  Some communities might favor 

Specified levels  of adjudication against ertain levels of adjudication certainty and 


_
#### -- Adjudication Meta Adjudication --

- Does your narrow assessments match your broad assessments within group



- Errors of systematic bias will occur "in favor" of your group

_
### --- REVIEWER SCORING ---

- AGENDA CHECK -- compare judgement against those with contrasting beliefs
- UP CHECK -- comparing judgements to those with higher score
- DEPTH CHECK --
- FORWARD CHECK -- compare judgement against future judgements you or others make
- BREADTH CHECK -- compare judgement to judgements made on same question by many other judges
- TRIANGULATION CHECK --
- SPECIALIST CHECK -- compare judgements with those made by relevant specialists


DIVISIVE ADJUDICATION -- An adjudication that has a strong systematic judgement delta between a group and the population as a whole.

ADJUDICATION EXCENTRICITY -- The difference between the mean adjudication rate for a specific judgement within a group relative to the population as a whole.


CONSENSUS GROUP -- Group of adjudicators that render similar judgements.
CONSENSUS SCORE -- A measure of how well a reviewer matches the median judgement of their closest group.






FUTURE SCORING -- Does the future validate or refure ones adjudications

WEIGHT SCORING -- Does greater scrutiny validate or refute ones adjudications

EXPERT SCORING -- Do highly rated adjudicators validate or refute ones adjudications

MIDDLE SCORING -- Do those leaning towards you validate or refute ones adjudications








### --- COHESION VERSES FREEDOM ---
#### _

A key aim of the forum is artful combining of two opposing forces towards _**Freedom**_ and towards _**Cohesion**_ 

**FREEDOM** -- the ability of any user or group to carryout their discourse without external constraint. 

**COHESION** -- the expressing a range of human though within a singular simple whole which captures the key aspects of that full range of thought in the simplest most concise way possible.

The Forum has two mechanisms to manage these competing objectives:
- Voting with eyes and feet
- Top-downing and bottom-upping

_
#### -- Voting with ones eyes and feet ---

**VOTING WITH YOUR FEET** -- Anytime a reader or author feels strongly that a particular scope of discourse has been "poisoned" by a non-constructive issue framings, non-constructive published stances, non-constructive cliques of other users, non-constructive rules of engagement, non-constructive rules of etiquette, _they can just leave_!  They can create their own verions of that same area of discourse, but with a context that is "not poisoned" by a non-constructive context.  Notice this judgement is theirs to make, there does not exist any meta judgement upon judgements of constructivity itself.  Any user or group and just pick up their marbles and go elsewhere (potentially ?maybe? creating a full copy of the content subtree as a starting point if they like)


But "voting with your feet" taken to the extreme will completely fail.  It will yield as many copies of human discourse as there are humans.  There will be no cohesion as all.  Thus:

**VOTING WITH YOUR EYES** -- The more engagement that a subtree has relative to other peer subtrees (readership, authorship, and voting) the more _**visibility**_ that tree will by default when any user approches this areas of the tree.  

Thus each authors will has a tradeoff to make.  No community, no topic framing, etc. will be perfect in their estimation, the dominant published stances will not be exactly right, etc.  So either they can create a whole new topic area try to build a community from scratch, or perhaps just create a new forumlation of a given stance and try to garner support.  Otherwise they can just edit and contribute to the dominate scope which already has great visibility.  


All areas of the forum will have some level of freedom and cohesion, by adjusting the forum rules one adjust propensity toward freedom vs cohesion.


_
#### -- Top-downing and bottom-upping --

Dynamics combine best aspects of top-down and bottom-up to yield a workable non-coercive, cohesive, multi-perspective synthesis of user thought on topics of discourse.  Specifically:

**TOP-DOWN SYNTHESIS** -- Any user or usegroup may author and maintain a cohesive, top-down, single-author-perspective of some area, some issue, some stance, some body of data.  
- COHESIVE -- Such summaries are good because they can sythesize multiple variational thoughts into a comprehensible cohesive presentation of a body of thought.  
- COMPREHESIBLE -- Such summaries, being human generated can be more comprehnsible than a machine combined summary of the same area.
- CONCISE -- A well written synthesis can concisely express the core themes underlying huge range of discussion/debate on a topic.
- COERSIVE -- Such summaries are coercive in that they inevitably adopt a framing which better matches certain lines of reasoning over others.  Further as it is a summarization potentially of hundreds or thousands of human perspectives it will necessarily be coercive in selecting a small fraction of all thought to represent the whole.
- BIASED -- Such summaries by additionally be biased as they come from one group or one user.  It is possible the for the author to take great pains to avoid such bias, nonetheless it continues to be large concern for any top-down summarization.

**BOTTOM-UP SYNTHESIS** -- By contrast, using expressing areas, issues, stances etc. using a covering set of perspectives has almost the opposite set of properties:
- FRACTURED / NON-COHISIVE -- Multiple, independently-authored, competiting perspectives are simply presented as a list of alternatives as a covering of an area, issue, or stance.  Such a list will necessarily be much less cohisive than an authored synthesis.
- COMPREHENSIBLE / CONCISE -- A list competing idea will likely be harder to understand as a whole, and will be much less concise than an authored summary.  (To mitigate this effect the algorithm strives to select a small number of "covering" pieces in order minimize this bloat, and rules
- NON-COERCIVE / UNBIASED -- Covering content lists are much less coercive/biased than top down authoring is two ways.  First, simply the act of providing multiple, chosen-diverse documents instead of a single document is inherently less biased, coercive since each new document cover intentionally opposing perspectives.  Second, by ordering presentation with greatest coverage first and allowing users to support multiple alternatives we are putting pressure towards consensus documents that effectively combine the capturing of ideas from multiple directions with in ways that are supported by many users.



**TOP-DOWN GAMIFICATION** -- The coverage mechanism and presentation mechanics are tuned to drive otherwise coercive / biased users to strive to publish more neutral single author top-down content.  When this succeeds one can get the both concise/cohesive AND non-coerecive/non-biased.  How does this work?

- System mechanically computes bias/coercion score for each each potential summaries for each scope.  If there is one or two notes with high consensus scores across ALL interestd groups then this consensus these consensus document and will not present any single author sumaries for an area that when there are no 

_
## METRICS
### - PARTICIPANT ACCURACY
- Do their scores match their peers well.
- Percentile rank within their group.  So cannot compare well across groups.
### - PARTICIPANT OPENNESS
- Will a participant go against their group when middle third strongly goes against their position
### - PARTICIPANT CONSTRUCTIVITY / CONGENEALITY
- Does participant operate in destructive, deceptive, dishonest ways (as assessed by their peers)
- Do they operate in constructive ways (as assessed by everyone)
### ...

Forum automation performs it decison making on the basis of a handful of key metrics.

The key idea of the Forum is to have an entirely mechanical process at it center.
This does not mean the materials curated by the Forum are free from bias, it means that the forum itself is designed specifically to avoid having any instrinsic bias between biases.  It seeks to summarize and present the range human thought on topics of human interest.

_
### --- KP-SPLIT ---

Identify a covering set of maximally distinct schools of thought within a given population.

**KP-SPLIT** -- Finds the k _most distinct_ schools of thought among the most compact p-sized aggregates.

Given:
	_G(i)_ a set of groups of population memebers.  (the entire population may be expressed as a set of singletons)
	_k_ is the number of distinct schools of thought to return
	_p_ is the minimum size of the compact aggregations (expressed as a percentage of the whole population)

	let _pop_ = |union(_G_)|											// the size of the population
	while min( |_G(i)_| ) < _pop_ * _p_:
		_smallest_ = argmin _g_ in _G_ { |_g_| }							// Pick the smallest aggregate
		_closest_ = argmin _g_ in _G_ \ _smallest_  { dist(_g_, _smallest_) }	// Pick is closest neighbor
		_G_ = G\smallest\closest + union(_smallest, closest_)			// merge these two to form a bigger aggregate
	while |_G_| > _k_:
		a,b = argmin a,b in G x G { dist(a, b) }					// Pick the closest pair of aggregates
		G = G\a\b + union(a, b)										// Merge these into a larger aggregate
	return _G_														  // Return the _k_ most distinct aggregates
	
_
### --- BIAS - a measure of the uneveness in support provided ---

_**NoteBias(note, [usergroups])**_ -- Note bias is a measure of the uneveness of its support across a range of user groups.

for each note and each user it is counted only if that user has supported at least one note peer (including itself)

note bias is presented as a gaussian combination of the bias distribution for each group.

if bias is presented as a single number then it is the lower bound of the 95th percentile of its distribution.


_
### --- COVERAGE - an ordered subset of a notes children that "cover" perspectives of interested users --

**COVERAGE** -- an ordered subset of a notes children that best represent the full range of perspectives on a particular scope of disucssion.

- CONCISE -- The coverge sequence is ordered to present the broadest range of represented human thought with fewest entries at front of list.
- REPRESENTATIVE -- The sequence maximizes _diversity of thought_ covered, it does NOT maximize the number of users covered.


REPRESENTATION OF THOUGHT **NOT** REPRESENTATION OF USERS
For a variety of reasons it is likely that the forum will user counts will significantly under or over represent certain lines of thinking (maybe many more liberals than conservative will sign up).  And even if one were to obtain representative samples of people that should still not be use as a measure of the value of a thought.  Right is right even if only a minority see it!

So instead of a popularity contest of ideas, we aim for a _**representative covering**_ of thought.
These thoughts are maximially diverse and are yet still well regarded by at least some minimal size subgroup of people.


**COVERAGE CALCULATION**
The coverage for a note, N, within the discourse tree is an ordered sequence of its children that best "cover" the opinions of those in the user groups that have voting rights on N:

- let _Bests_ be [], Scores be []						-- Extended one "best" perpective at a time
- let _Coverage_ be 0
- let _C(i)_ be the children of _N_						  -- These are the possible key perspectives
- let G(g) be the user groups allowed to vote on N	  -- These are the opinions we must cover
- let _S(g)_, the kp-split usergroups, G, 				-- Major "schools of thought"
		based on their voting record on C.
- let SW(g) be 1/n where 'n' is the					  -- Scale group by its size
	number of users in split S(g)
- let W(u) be the max SW(g) the u is member of		  -- Scale each user by their rarest group
- while _Coverage_ < threshold
	- Let _V(g,i)_ be the voter support rate 			-- Updates each childs support after removing 
		for child C(i) in split group S(g) 					first "Best" children from consideration
		after dicounting children in Bests
	- Select b the C(i) with greatest cumulative 	  -- Find the next best child
		support Sum(i) across all thought groups.
	- Append b to Bests; Append Sum(i) to Scores
	- Let Coverge = Coverage - Sum(i)				  -- update coverage 
- return Bests, Scores								  -- Returns 'best' representatives for tree


_
### --- CONSENSUS ---

Dendgram-based survey of opinions.

**A MOTIVATING EXAMPLE**:  
Suppose out of 90-liberals and 10-conservative we have 90% support for a note, 90 votes of support from the liberals, and zero from the conservatives.  Can we say we have reached a consensus on this note since we now have a 90% support rate?  Hell no!  But suppose instead we have 77% support from 70 liberals and 7 conservatives?  Now that seems much more like consensus even though the raw percentage of support is lower.


COMPUTATION
(1) Compute a kp-slice of the group or user dendogram
(2) Compute the support rate for each of the k slices of users
(3) Return the minium support rate as your consensus score

For any given mesure of consensus we specify some minimum threshold and say consensus was achieved if that threshold is exceeded.  The threshold may be require a working consensus or an absolute consensus, and it may require a higher 2/3rds or 4/5ths consensus as well.


Absolute Support(Note) -- Fraction of all members of a group that support this note above any other note
Simple Support(Note) -- Fraction of all members of a group that support this note relative to the number that have expressed an opinion on this note or any of its alternative peer children.



_
### --- RATIFICATION ---

RATIFICATION -- Board members request members of a group to consider support for some note or notes.

The can be done to short circuit noisy discussions if they feel that consensus is possible.
They can do this in many ways, but need to no "harass their members" with too many questions.

- CALL FOR CONSENSUS -- They look at best parts of multiple stances and write what they see as their groups consensus.  In order to achieve consensus they may need to highlight both sides of one or two contensions in the group, thus everyone can support since everyone feels heard.
- SUPER CONSENSUS -- The board band together with the boards of like minded groups to create a super consensus spanning these multiple groups.  Then each group can ask its own members to ratify it.
- CRYSTALIZED CHOICE -- If the board sees several key stances emerging, but it does not know which is best support by their members they could ask them to consider voting for one or all of them.

_
### --- INTEREST - measures how much a set of users cares about a particular scope of discussion ---

**INTEREST** -- measures of how much a group of users cares about a particular scope of discussion.

- BROKEN INTO SEVERAL MEASURES OF INTEREST:  NUMBER OF READS, COMMENT, VOTES, and EDITS executed by the group.
- EACH CAN BE EXPRESSED as 0/1 or as mean number of occurances
- SCALED BY SIZE -- larger groups will naturally have larger amounts of activity thus larger numbers of interest indicating actions.  Thus we consider the number of these interactions as a fraction of the group size to account for this.
- CORRECTED FOR SMALL GROUP SIZES -- This calulation over weights very small groups since it is much easier to obtain 100% interest as a group becomes tiny (or just one person).  There are several correction factors that can be applied for this.

_**SimpleInterest(users, subtree)**_ -- Fraction of users within the group that have cast any vote within a scope.   (This score would be trimmed or ignored for very small group sizes)

~-~~

_**PostingVolume(entity, subtree)**_ -- The number of posts that user or group of users have posted within a subtree of the discourse tree.

_**VotingVolume(entity, subtree)**_ -- Similarly voting volume is the number of support assertions made within a given subtree.

_**VotingPolarity(entity, subtree)**_ -- One minus the 95th percentile lower bound on the entropy of voting for children at each level within the subtree.
     (really need to work on this metric!)


_
### --- GROUP MEASURES ---
#### --- GROUP DENDOGRAM - the dendogram of user groups ---

**GROUP DENDOGRAM** -- the dendogram of user groups as orgnaized by a measure of group similarity.

_
#### --- GROUP SIMILARITY - measures how similar the discourse behavior is between two groups of users ---

**GROUP SIMILARITY MEASURE** - any real-valued computation based upon two groups of users along with their record of posts.

**VOTING SIMILARITY** - the voting similarity of two groups of users is the count-weighted average KL-distance between the pdfs of their voting record at each note in the discourse tree.
_
#### --- GROUP BEHAVIOR - a measure of the discourse behavior of a group ---

**GROUP BEHAVIORAL MEASURE** -- any function of the posts from a group (possibly scaled by the posting behavior of non-group members)

**VOTING BEHAVIOR** -- a mapping of notes onto an integer number of votes cast for that note.

_
#### --- K-SLICE - a partitioning of users or usergroups into k sets with maximally distinct thought ---



_
#### ...
- ALT SET -- List of nodes within a given altset defined by transitive closure of ALT
- COMPETENCE -- Authors are scored on their competence along several dimensions:
	- ADJUDICATION BIAS -- 
	- ADJUDICATION ACCURACY -- 




_
_
## === GLOSSARY ===

**DISCOURSE TREE** -- the global tree of all hook names

**NOTE** -- The current state for a named bit of human thought.  (see note data type)

**POST** -- A message containing the state of a note at a given moment in time (the tree of notes is updated by posts overwrite old notes contents)

PUBLISH -- A user 

**SUBTREE** -- A scope is the subset of all notes within the global tree of notes.The scope for a note within the note tree 

_
## === EXAMPLE ===

**GUN RIGHTS ISSUE AND STANCE**
Joe creates "Gun Rights" issue under the "Policy Topics" node
Joe creates a stance "Guns ownership is a key right in a democracy"

Several others add different stances and comments to this issue.

**GUN RIGHTS CONSENSUS STANCE**
Joe creates a consensus note and selects 4 of the top 9 stances claiming his note covers them.  He does not get consensus from the supporters of those 4 stances, but 3 out of the 4 groups do agree with him, so he removes the one stance froming his consensus note, thus forming a narrower conensus of his gun rights stance.

Jeff creates a consensus stance "Guns Save Lives" which merges 3 more gun related stances.. it also pro-gun, but is takes a really different than the gun rights angle Joe used. 

**ANTI-GUN ISSUE AND STANCE**
Chris does not like Joe's "Gun Rights" framing of the issue itself.  It frames the discussion purely in terms of rights and freedoms which effectively biases towards his own stance.
So Chris creates the issue "Gun Control" under "Policy Topics" as new issues statement and claims it is a "consensus" version of Joe's original "Gun Rights" issue.  He also creates a strong anti-gun stance called "Guns Kill" under these two versions of the issue.

The larger community cannot agree on the "gun right" vs "gun control" version of the issue to present, so both are are listed as alternatives when you search under "Guns"

**CONSENSUS GUN ISSUE**
Kim has lots of very highly visible consensus pieces, and she sees an opportunity to build a new consensus piece, she is a firm gun control advocate, but still does a passable job of summarizing the three key stances on the issues:  Guns are rights, Gun save lives, and Guns kill.  She struggles finding a super neutral title, but eventually lands on "Gun Policy".  She searches for highly rated pro-gun supporters of other stances, and finds Jeff who has also authored many consensus pieces too, but he is very pro gun.  He edits her piece and after several rounds they are both satisfied, and they jointly publish this piece to their respective board of directors as a consensus piece.

**BOARD SUPPORT**
Their boards recognize this is a very contention issue and edit its permissions as board-editable only by both board-groups as well as by the original authors.  They push a request for ratification to all in their groups who have supported any stance on any related issue framings, along with the unanimous support from both boards.

**RATIFICATION**
The larger communities ratify this note as a consensus framing of the issue.  
Now when users type "gun" into the search bar this consensus "gun policy" note comes up first, and all existing issues that are identified as synthesized by this consensus will also have a link back to this consensus note that a consensus was formed, so readers can follow that link to go to the consensus note.


_
## === ADJUDICATION MECHANICS ===

1. TOPIC -- Sponsor or community select topics for consideration.

=== COLLABORATION PERIOD ===
2. INITIAL AUTHORING -- Left and Right authors are selected at random from appropriate quality class based on resources provided.
3. PAIRWISE COLLABORATIVE REFINEMENT -- Each pen their own separate analysis, and collaborate with each other to constructively revise their own analysis based on feedback from their counter part.
4. CONSENSUS STATEMENT -- They also work with their counterparts to collectively author a list of consensus points that both accept as supported.
5. OBJECTION CODIFICATION -- In cases where one side does not accept an edit from the other, the objection is formalized as objection or a qualifying adjudication.

=== FEEDBACK PERIOD ===
6. COMMUNITY FEEDBACK -- The larger community is then invited to comment on all parts of the analysis.  The consensus points made, the left and right framings of the issue, as well as all objecting adjudications made by the initial collaborators.
7. RECURSIVE FEEDBACK -- Depending on the resources allocated to this topic, and the amount of contention found a certain amount of recursive adjudication of community adjudications is also performed.
8. SEALED HOLDBACK -- 20% of the adjudications are randomly sealed.  The results of the adjudication vote is counted in as part of the feedback process, but the name of the adjudicator and the text of their analysis is sealed by the system.
9. POSITION EVOLUTION -- Over time the system maintains a current "best" leading edits of the original position, as well as a best list of supports and objections for this best piece.  (NOTICE: only those on the left are free to suggest edits to the left position, and to suggest edits to objections to the right, etc.)
10. CONSENSUS SUMMARY -- After sufficient multi-round feedback and quiecence is reached, the final consensus statement, left and right positions as well as best objections are captured as a completed analysis for the topic in question.  This is published as an completed analysis by the forum.
11. EVOLUTION -- Over time as interest indicates, or as additional facts become available participants may continue to augment or shape the resulting consensus.  Thus important issues may remain "live" for an extended time.  Others will simply stand as an analysis done at a specfic point in time.
12. EDITS -- During feedback each adjudication is added with visibility into all over adjudications and comments made prior to its submission.  Judgements about this adjudication are based upon the precise state of all relevant adjudications at the moment of submission.  But later as new adjudications are added and new discussions are had, each participant is free to EDIT their adjudication's body text, as well as the adjudication's vote as well.  The analysis of each adjudication is performed with the benefit of all published info at the time of analysis.  Thus it is usually beneficial to edit ones adjudication if new ideas become known.  There is a modest impartiality penalty paid when one changes ones own mind, but there is a much larger penalty paid when adjudications are finalized against your position, thus changing that position of one is wrong is generally better.

12. HOLDBACK ANALYSIS -- At a later time these sealed holdback adjudications are randomly sent to multiple participants with similar left-right bias for evaluation.  This is done to assess the assessors.  The goal is to perform a kind of auditing of the specific adjudication that was submitted.  Was it clear, did it accurately track the published rubrick for the conclusions it drew, etc.  The goal is to gather feedback on this adjudicator, as well as a test of the one the adjudication is sent to.  This test check if the hold back recipient is consistent with all other recipients of the same adjudication, and it is a test of the underlying participants original adjudication.

	The results of this holdback analysis are used to update the impartiality, accuracy, clarity, etc. scores for all participants involved.

13. TRIANGULATION -- Some fraction of these holdback adjudications are further analyzed thur triangulation.  During triangulation highly impartial (but still biased) judges are then called upon to perform a deeper analysis of this same adjudication.  This deeper analysis is used to further adjust the impartialty scores for these lesser judges.  Triangulation not only involves judges rated to be especially impartial, but it also involves multiple of these judges who must achieve consensus in order to the triangulation to proceed.  In those cases all lesser judges are updated based on their agreement with or disagreement with the senior judge consensus.

14. EDITS -- 


9. s of a
- 
- to derive a subset of consensus points.


## === CHEAP HACK FIRST SYSTEM ===
- GDOCS -- Use personal google doc accounts with public pages a basis for each person's note contributions.
- SERVER -- Synthesis web server.  Scans pages and presents tree (with comments etc.)
- ANNNOTATION COMMANDS -- Text-based annotation commands
- BOF -- Birds of a feather, users that mutually agree to share editing rights.  (social trainee)




Annotations:
- >Note 7496		>N7496	-- Header for note 7496	
- >Talk 2953		>C2953	-- Header for section discussing Indicates what follows is in reference to note 2953
- Version 113	V113	-- Provides version number for a note

C2953-Ver113 

U10-11	How do we know this is true?  Did this come from a study? some expert's opinion?
C43		Just because the rate of gun usage was higher it does not follow this is the cause of lower thefts.
		But you can say there was a correlation in this one case, you also cannot say it is a general trend w/o more evidence.

_
# INFO
## === PTRS
### - People
- Here's a guy from the right who might see value in the modified discussion thread project: 
  https://time.com/6087577/michael-fanone-january-6-interview/

### - Change-My-View Channel

- [Change My View](https://www.reddit.com/r/changemyview/)   channel dedicated to persuasive arguments 


### r2022-05-14  Best Forum Software
https://www.hostinger.com/tutorials/best-forum-software

### --- COMPANIES ---
#### -- For pay fact checkers --
[Tech Crunch about Reuters](https://techcrunch.com/2020/02/12/reuters-facebook-fact-checker/) 
Reuters joins the list of US fact-checking partners that include The Associated Press, PolitiFact, Factcheck.org, and four others. Facebook offers fact-checking in over 60 countries, though often with just one partner like Agence France-Presse’s local branches.

_
# REF
### t2022-06-11  Medium how fear and outrage is sold for profit.

https://tobiasrose.medium.com/the-enemy-in-our-feeds-e86511488de

# LOG
### m2022-08-24  Matt Pinson - Friend Christy - Friend of Yashin


304-654-0087

[Welcome to Huntingtonnews.net | Huntington News | Huntington News](https://nam02.safelinks.protection.outlook.com/?url=https%3A%2F%2Fwww.huntingtonnews.net%2F&data=05%7C01%7Cchristine.risch%40marshall.edu%7C7af239706c9a45710f0d08da845c4315%7C239ab2783bba4c78b41d8508a541e025%7C0%7C0%7C637967829057083101%7CUnknown%7CTWFpbGZsb3d8eyJWIjoiMC4wLjAwMDAiLCJQIjoiV2luMzIiLCJBTiI6Ik1haWwiLCJXVCI6Mn0%3D%7C3000%7C%7C%7C&sdata=4R3WE1E9qjpyBqCMqmqrAgaqafTzFXFZcOB8I4C1szY%3D&reserved=0)



### m2022-06-21  Joe Yeh


- [ ] 
- [ ] Heritage Foundation; Commonwealth Club; Cato Institute; 
	- [ ] Who would provide visibility?
	- [ ] Thought leaders
- [ ] https://www.commonwealthclub.org    Live Events    $1000 plate fundraising events
- [ ] Connected with a school - 
### t2022-06-10  Yashin: Forum example of moderate acceptance of voter fraud claim

  
  

> January 6 widens the political divide fora number of reasons not the least because it makes Republicans just look like people who have alien logic. It's difficult to relate to.

  

:-)   spoken like a liberal.  I think the a conservative can look negatively upon the Jan 6th commission w/o alien logic.  But I think it might not be worth me trying to support that claim.

  

  

  

> I'm looking through some of the footage of the January 6th commission. No doubt what Philippa said about people not being interested in evidence that contradicts their beliefs is shaped by this recent event. There really was no evidence of widespread fraud, and 70% of Republicans still believe the election was not fair. I don't get it either, but it really does tell you the power of having someone that is highly respected (Trump) influencing people's beliefs.

  

Indeed.  And more than that, it is simply shows the power of believing what you are motivated to believe.

  

> The problem is that he's not evidence based. This is also scary in that out could suggest that the left, you or me could also be susceptible to such brain skips.

  

Not could be, WE ARE susceptable.  Progressives are blinded by believing themselves different.

  

But this is not about not being evidenced based.  Trump uses lots of evidence and ignores even more.  One should not describe what Nancy Pelosi, Ted Cruz, or Donald Trump is doing is getting to truth thru evidence…..   No they already know the conclusion that want, and they are looking for the best evidence which can support that conclusion.

  

The “evidence rules" that Nancy must play by are more different than those of Trump, but their approach and goals are the same.  

  

(Also don’t need to debate exactly what I mean by this claim either.)

  

  

  

  

> So for me, I'm interested in what the forum can do as a go to place for those people at the edges, who might be undecided or questioning. Or the Ragfenspergers. That's the main hope I have.

  

Yes providing a source for those who are genuinely questioning / undecided is a huge goal.

  

And another perhaps bigger goal:   Ground our national conversation with a tiny bit of reality.   

  

  

Here is how:

(1) Assuming that the forum really has strong/articulate conservative voices in it.

      It will have powerfully presented conservative perspectives on key issues.

(2) Assuming it has open-minded liberals in it, those open minded ones will ARGEE with key claims made

      As part of those conservative perspectives.   (They don’t agree w/ conclusions, just agree with certain claims)

(3) Non-open minded conservatives from the larger population “cherry pick” those perspectives they already believe.

      They will say:   ``See!!  Even Liberals accept X and Y are true!!”

  

(4) Over time conservatives see the forum as having honest conservative values, and informed perspectives.

      They will have used liberal agreement with certain facts they know to be true as evidence in arguments online & off

(5) Then when some open minded conservative agrees with a liberal claim that YYYY is true they will pause and consider

      (Especially since that open-minded conservative explains how they concede this small point, but still retain their 

       Larger conservative conclusions about the larger issue.)

  

  

Notice how the Forum is different here:

  

Imagine this is a conservative who says  ``American elections are inherently unsafe, cheating can happen and the moves towards mail in ballots is just a ploy to allow MASSIVE cheating.  I think liberals are systematically trying to throw the election unfairly in their favor on each election.  And I think illegal immigration is just a ploy to get more democrat voters.''

  

     …. but I have to concede, while I still think fraud probably happened, we just don’t have the evidence to prove that it did.

     …  It is true that Trump appointed judges saw the evidence presented, and dismissed it as not having the basis for a case.

  

  

See how a close minded conservative is much more likely to listen to this voice.  This voice is not trying to convince them that fraud did not occur.  They can tell this voice WANTS to support the fraud claim, but just cannot support the claim that we have proof, since we don’t.  It is the perspective of this voice that makes it so much more powerful for the somewhat close minded listener. 

  

It is just w/o the forum we have no mechanism to elevate the opinions of these rare “strongly partisan, open-minded thinkers”

  

That is what the forum does.  (I hope)

  

  

  

NOTICE:  My goal with the forum is not to try to convince the conservative that voter fraud did not occur, or that our election system is in good shape, or that liberals aren’t trying to steal etc.  None of that is on the table.  What **IS** on the table is the narrow claim that we don’t have evidence to massive fraud against Trump.  That narrow claim is it.  (But if that narrow claim was broadly accepted it would massively adjust the national discussion.  It would inject just a bit of reality into that discussion.)

  

  

—dan
### t2022-06-07  Yashin response about OARS and DARN

>   
> Seems like if the forum could incorporate principles of behavior change, it could perhaps help?
> 
>   
> 
> Observing the email conversations with Mark and Leslie, sometimes it seems that the more interested you were in having them follow you into a line of logic you were confident would prove they actually might agree with you, the less interested Mark was in engaging. 

  

Indeed it is an occasional failure mode of mine.  When I perceive the other person to be ignoring a cut-and-dried, black-and-white consideration I first try to express it in a constructive way.

  

When I perceive them as deliberately dodging the bullet, I can revert into a prove-it-to-you mode and the other person generally can sense this, and goes into hyper defense.  Denying even that 2+2=4 as a defensive tactic against a logic weapon being wielded.

  

Not so constructive from my side.  I generally don’t do it in cases that I don’t respect the other person.  Rather it happens when I have some or alot of respect for the other, but they are being intentionally or unintentionally obtuse as a arguementation tactic.

  

I HATE THIS TACTIC!  Still, I have to concede that my response to it, is not very helpful.

  

  

>   
> 
> Not sure the below is helpful, but I think what Philippa was pointing out it's that the average person isn't motivated to listening to counter arguments. That's really true. 

  

agreed.

  

> We're all focused elsewhere, and after maybe ones 20s, exploring new views is even lower priority than before. So how to emphasize the importance of productive discourse? I understand that in the forum the motivation is to produce an accurate statement of my beliefs and the underlying evidence supporting it. Correct?  

  

Is correct, but it aims for more.  Both sides are aiming for the most compelling mind changing framings.  So there are two aspects of the system that help at least a little bit for this:

  

(1) Smart partisans will read the counter points being made against their arguments, and naturally they will want to select the most compelling arguments….  Those will involve facts, but they will be the most COMPELLING ones… including emotion and other considerations.

  

(2) Assuming I can select for those rare partisans that are open minded in accepting counter evidence (while not changing their overall stance) I hope to co-opt them in producing the persuasive arguments.  (AGAINST the position that they hold.)

  

This is a subtle idea… let me provide an example:   Joe believes the climate change winers are overblowing the issues, and it has strongly against most legislation that tries to mitigate climate change.  But Joe is also pretty sophisticated in understanding studies and statistics etc.

  

So the Forum sees he is anti-climate, but that he has judged this specific claim made about climate change causing ice sheet melting.  A pro-climate person argues this study shows compellingly that the haderwhite self’s collapse in 2014 was driven significantly by human driven warming.

  

The algorithms sees he has pretty high authority score too, so it invites him to justify his reason for accepting this pro-climate person’s argument.  

  

Now remember Joe STILL does not think we need to enact legislation … things are not that bad, BUT this study and argument IS valid.  He must concede it.

  

When the algorithm asks you to write up your arguments, it is very likely that they will be prominent on the site, since it already knows the structure of the votes, and your position in them.  So Joe is motivated to do a good job with his post.

  

  

Now think about the situation….   An anti-climate person sees this claim…  human warming collapse ice shelf, and it has three check marks.  Meaning it is being accepted by the open minded folks ON HIS SIDE.  So he thinks, well let me see that these “so called” conservatives are saying.  (He is skeptical that this is just a RINO pretending to be conservative.)

  

But all the words written by Joe are clearly from an anti-climate perspective.  Yes this shelf melted, but that does not mean others will, or even if they do, we can deal with the elevated water (or whatever).  I think hearing this perspective is a much more compelling argument for the anti-climate person.

  

It is more compelling for the simple reason that the reader can tell Joe is NOT trying to make an argument that is pro-climate … Joe is just acknowledging this narrow fact in a begrudging kind of way.   It is the begrudging nature of Joe’s perspective that lets the reader drop their guard.  Joe is NOT trying to slide one over on them.

  

They can tell Joe is ALSO anti-climate, yet begrudgingly Joe is accepting this one fact, because of his deeper understanding the stats, and careful reading of the paper.

  

~~

  

There are alot of other ideas in the really interesting stuff you copied here.  In one way the goal of the form is not behavior change, but rather it is belief change.  And indeed it is not even belief change.  (Since the forum does not know which positions are superior, it is not really trying to aim towards any particular position.)

  

Its aim is simply to elevate the discussion into a place where as many starting assumptions as possible are accepted by many.

  

The other gap to that info, is the forum is focused on ’self-serve’ info output.  Maybe there could be a way to cause the forum to have each person execute OARS or DARN on others.  It is an interesting idea, but I think the number of really good ‘rare’ people used to drive the system will be pretty rare, so it will be harder to have them doing any kind of one-on-one coaching.  Still maybe they could execute a kind of DARN via text discussion with many others at one time.

  

This is kind of the sort of interaction I imagine a person with high authority would be doing within their community.

  

Interesting way to think about it.

  

—d


### n2022-05-28  Raw material for Immigration issue based slides


Conservative issues:  Immigration, 2020 Election Fraud
Liberal issues: Guns, Climate


- **2020 Immigration** -- At issue are the causes, consequences, and levels of recent undocumented/illegal immigrants entering the United States
	- **Disastrous Biden Policies** -- In a few short years Biden has enacted a number policies which together have had a disastrous effect on illegal immigration to the United States.
		- **Cancelled remain in mexico** -- This likely has significantly increased illegal immigration for immigrants waiting for their court case to be heard.
		- **Reinstated catch and release** -- 
			- **Its the Law!** -- US law guarantees every seeker their day in court.  Using title 42 to deny this w/o medical need it illegal.
		- **Spiked under Trump too** -- Immigration spikes happen, we had a similar sized spike under Trump.
	- **Undocumented Immigrant Issue improves with time** -- Biden and Trump both had a spike in undocumented immigrants at the border.  Despite these spikes, the overall trend in the number of illegal immigrants in the country has been steadily declining over the last 25 years.
	- **Immigrants Drive American Greatness** -- We are a country of immigrants, just as as mass immigration from China's country sides drive its immigration so to do immigrants from Elon Musk down to the homeless child at our border drive our greatness.

- Remain in Mexico worked -- 
- Catch and release 

- +++

	+++ Tribe, Neutrals, Experts
	

Percentile match to a given tribe
Percentile rank of opinions on axis





### n2022-05-27  Thinking about how to drive compromise  (Using 2020 election fraud example)

- Assuming there is little evidence for massive fraud in the 2020 election, 
  how do we ensure reasonable conservatives a successful push for a balanced conservative position?


TRUMP WON -- Massive fraud perpetrated during the 2020 election

:thumbs down:
The 2020 US election was host to widespread election fraud.  
Several of these frauds were large enough to have independently thrown the election fraudulently in favor of Joe Biden.
Thus we know that Donald Trump is almost certainly the rightful winner of the 2020 election.

DISSENTING ALLY POSITION:
There were voting irregularities in the 2020 election of unknown magnitude.  We did not have the time, resources or access to properly assess this election, so we will never be sure.  We cannot be sure that Trump would have won, but we also cannot be sure that he lost.

NEUTRAL CRITIQUE:
Every election presumably has some degree of fraud.  The 2020 election had a many claims of fraud which resulted in a record number of 59 court cases being filed on behalf of Donald Trump.  Everyone of these 59 cases was thrown out of court as lacking merit.  Thus it seems quite improbable that somehow a massive fraud was committed, large enough to throw off the election across multiple indpendent states and not leave enough evidence even be considered in court.

~
















### m2022-05-09  Yashin and Phippa

- Explain idea at highest level & get a gut check reaction; quick pointers
- 


### t2022-05-05  Invitation to talk w. Al

Al:

Good to electronically meet!  It sounds like we share a common interest regarding disinformation and hyper partisanship.


For me, this has been a line of thinking that is many years in the making, though I have never really done anything with it, since I was never clear if I could turn it into some kind of sustaining business  (I am still not clear about that).

My aim is a discussion forum that synthesizes succinct (single phrase) summaries of what different communities think about important issues.  e.g. is there significant evidence of climate change?  is there significant evidence of 2020 election fraud?  The first aim would not be to arrive at a single “correct” answer, but rather succinct summaries of different schools of thought.

The second analysis, however, would be to try to push as much consensus as possible around these issues.
e.g.  conservatives agree giving global consensus that certain minimal statements about climate change are empirically supported, while liberals agree giving global consensus that other stronger statements are not supported.

The result could be looked as a spectrum of statements (which could be succinctly shown on a single page!)
At the ends of the list would be statements broadly agreed, with hotly contested statements in the middle.

Such a summary could really advance the societal discussion, since it could show everyone which statements are not really even supported by their own clan when their members look deeply at the topic.

~-~~

But that only works if one can trust the analysis system itself, and only if the system can good judges from all political sides of an issue.

### n2022-04-28  My big question about the forum
My world has flipped upside down, with regards to what problems I get to work on.  So I don’t have confidence that I can begin work on that in any specific timeframe.  Still it is a huge problem for society, and if I gained evidence that I had something of value to contribute, then I would get much more excited about it.

  

There are many forums where people can talk.  I dream of one where the ’talking’ leaves behind these polished, condensed, summaries of the different perspectives.

  

My first and most massive question is whether anyone would really care?  Would the put their energies into collaboratively producing these summaries or not.

  

It seems like Philippa might be in a good position to have an informed opinion on that question.  So yeah, it would be COOL to talk with her!
### 2022-02-16 -- Email based first system.

- Use scripted email or forms based system to select topics for analysis.
- Use script to select author pairs


### 2020-09-17 -- Police violence

An example:

Suppose a reader were reading a note/post about police violence against Blacks, and in the author’s opinion such actions ``almost always occurs when someone is being non-compliant with legitimate police request.''

Now the reader remembers reading an article where someone listed many cases where “the Black did nothing wrong”.  So they create a post for the news feed talking about this, and they highlight the sentence in the original posting and create a “-1” link to their new post.

~-~~

Later others plus +1 and -1 his original counter argument, each one adding evidence in one direction or another, and most people are creating a blog entry that they intend to read like a normal threaded conversation.

~-~~

After ten of these someone in the same user group as she is in edits her post/note to have the name:  "police abuse of compliant Blacks” 
And quickly cuts and pastes content from 10 messages into a nice list of evidence with phrase summaries for each and a link to original sources if those posts included them.  This person was a bit biased however and did not address the many counter evidence to this counter evidence.

~-~~

The original author sees this consensus stance against his original stance is ignoring all of these other posts (which he can easily see), so he summarizes them and creates the counter summary and ties them together as a different stance on the same note “police abuse ….”

~-~~

After after many edits and time original posting bubbles up as a key “pro-police” stances… so it gets more scrutiny … and cooler heads prevail over this little sub argument about "police abuse of compliant blacks”

This cooler head is able to do justice to both evidence in favor and against this assertion that this happens.  They even put a cohesive title on their piece:
      ``Police abuse of compliant citizens — it does happen, but not often’'

Their summary does a good job of weeding thru the best evidence in both directions at the top, and left the rest in a sub note for reference.
Their note did not get as much support as either of the other two stances, but the forum algorithms recognized it as less biased, as so put it at the top with is summary text fully expanded, and left all others posts with just their one-line link listed below it.

~-~~

I am trying to make it so that people can engage in tiny ways that move the final argument structure forward.

The HARD part is ensuring that folks with really competing views aren’t just deleting or otherwise screwing with opposing content.

—dan

_
### 2020-09-07 -- Shinster email

I will have failed if I get all the right “experts” to contribute but then they are not believed.
Instead I would hope to get leading anti-vaccers on the same site with immunologists, and let them each present their perspective, but they cannot avoid having the best critique of their perspective posted right along side their position.


The website would look a little like wikipedia, but instead of a single article on “vaccine issues” 
Multiple people might edit and fork a range of framing summaries, and we would present the middle least biased one (determined by math + reviewers)

Then under that would be a range of “stance” posts that would try to clarify a single narrow issue like   ``what does the data say about their safety?’'
This would get split into a range of “for” and “against” arguments, many people can edit and fork these arguments, so there are many.
But we algorithmically judge “logical precision”. And “biased presentation” etc.   we do this in the same ways that folks like https://mediabiasfactcheck.com/ do.

The hard part would be getting liberal and conservative, pro-science and science-skeptical folks who were open to rational debate….  I think one could essentially “judge the judges”
This is tricky, since I need to do that judging in a way that everyone agrees is unbiased.  I think it would be based on internal consistency against a set of agreed (written rubrics) for assessing “logical presentation” and “biased presentation”

Judges themselves grow to have greater scores in their ability to discern and in their neutrality.  So it becomes a self policing system where judges are judging each other and we measure consistency in blind comparisons.  (e.g.  do judges consistently agree)

Big picture:
— Use the energies that many have to “speak out” about issues they care about, energy to “be heard” as the driver for content.
— Use comparative analysis between users to see the more neutral better framings of pro and anti- stances
— Force each person to see the counter evidence for the things they believe.


p.s. a-la-carte idea that was missed in above:

The idea is to create a deeply annotated discussion of the issue, by allowing users to comment directly on the sentences in the leading stance that they oppose.
They can “simply wrong” then a short paragraph showing with references why that sentence are wrong.  The author of that stance probably does not want a highlight shown to everyone that says “simply wrong” in the margin along with references!  So they will probably edit or remove that sentence.  Which is fine, then the consensus stance become more justified.  

The idea is that as you read a stance and all the counter points you can click buttons to agree or slightly edit what you see, and the algorithm presents everyone’s edits as a consensus… showing the annotations with heaviest support most clearly and immediately.   Other annotations are still there, but you really need to read EVERYTHING on the issue to see them.


_
### 2020-00-00 UNSORTED

THE FORUM

Key ideas

TWO VIEWS



TWO ORGANIZING PRINCIPLES --  ‘by-time’   and   ‘by-topic’
Discussions (used for information input) are best organized by time (that a post was submitted).  Global organization (used for retrieval or output) are best organized by topic/sub-topic.

USE BEST ORGANAZATION FOR

MOSTLY AUTOMATIC TRANSLATION BETWEEN ORGANIZATION

 
_



