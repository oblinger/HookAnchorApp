stand#pp  


# LOG


## 2025-12-11  


- Use the same SV Jobs API
- When will first paid game
- Proxy service injest the data.  
	- Need to create topic 


Ram and Sean:

We had a productive conversation today and outlined a possible tactical roadmap.
We made some assumptions about timing and requirements; if these are incorrect, we can adjust the plan to ensure it meets them.

NOW TARGETING UNIFIED API FOR ALL GAMES.
Ed noted that it would be logistically challenging for them to manually send us lists of games and accept file uploads of results, so he preferred using the Jobs API we have been jointly developing.  We also favor this pathway if it can work from a timing perspective.  If timing does not work out and we require a human-annotated game earlier, we will use a manual process: PlayOn emails game lists for SportsVisio operators to access manually, and emails result JSON files for PlayOn to use.  (With luck, we can avoid this more painful pathway.)

TIMING
- Paid Games - Based on integration readiness, it seems the first paid games for both human-in-the-loop and pure AI games will be in January.
- Jobs AI API - We have had the jobs API live for the last two weeks, and PlayOn has been accepting stub game results from it, and is testing full games right now.
- Integrated Jobs API - It is a slight stretch, but we aim to have an end-to-end integration of human-in-the-loop-annotated games into our existing API by Christmas.
- PlayOn Integration Work - During this period or thereafter, PlayOn will implement downstream processing for both Pure AI and human-in-the-loop games.
- Integration Testing for Human-in-the-Loop Games: Optimistically, we could begin end-to-end testing for human-in-the-loop games on Jan-1.
  If it did not happen then, it would certainly occur shortly after (at least on our end).
- Once end-to-end testing was complete, we could begin processing paid games.

ANNOTATION FORMAT
- PlayOn already has parsing in place for our joint AI-only game format.  Tomorrow, we will send updated documents outlining the extension of this format for human-in-the-loop games.  Ed suggests it may be easier for PlayOn to use this joint format.
- For simplicity and additional testing, we slightly prefer using the same format we provide for pure AI games, but we can easily provide whatever format PlayOn prefers.  For now, we are finalizing the output in the joint format and will add the VidSwap style format if desired. Just let us know.


OPEN QUESTIONS
- What is the earliest and latest date PlayOn will have implemented and validated the usage of Human-in-the-loop games from SportsVisio?
- What are the updated target dates for the first paid human-in-the-loop games from PlayOn and the first paid Pure AI games?
- How damaging will modest (1- or 2-week) schedule slippages be?
  (From our side, small slippages will not be an issue, but larger ones will risk having too steep a ramp-up in our systems usage.)


Best, 
Dan



## 2025-12-11  H3

### 2025-07-28  Ready for call

- How do we do with Cuts
- GPU avail??
- Timeline??

Game #2 - A684 - Issue detecting the hoop.
- This issue we think we can fix.
- [Practice in #2]()

### 2025-06-24  Call with CTO Andy

- Object Detection
- Make/Miss model.  presently our make miss accuracy is 80% in general, but only 50% on their videos.  correct?
- Rebounds and Assists


- Datasets.  15K 
- Games from prior seasons.

- automated ones.  
- acquire max-preps - they serve stats
- most are interested in the live experience.  not VOD.  shot chart
- click to see on shotchart in real time.
- post the score during the game.


