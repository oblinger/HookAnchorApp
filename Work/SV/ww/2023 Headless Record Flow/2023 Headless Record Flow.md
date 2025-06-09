
- [Spec](https://docs.google.com/document/d/1ZZbLfJBvZebmRCy4ILwn-46HHaGyu2F0_8Ulpr3RrD8/edit#) 
- [Scott flow](https://docs.google.com/presentation/d/1dPhzfmOEM04qu98Aorc4Cp5Rwk3YrdMKrBlAGeoh7Dg/edit#slide=id.g22b1ba1399d_0_13) - 


# LOG

### 2023-04-20  Record App 2.0


Background: I have talking w/ Jack and some with Mark about needs for 'headless' game recording.  I think I might see a dramatically more flexible and simpler design struggling to escape our current complexity.  Jack asked me to hurry up and finish a spec for this, so Agus can mock it up, as she has a gap now.  Attached is a starter spec.

Here are some thoughts about next steps:
- This is Brian's team and brian's resources, so he will be deciding what we are really doing (Brian please chime in).
- Jack once you look at this you should think about it from a user perspective and verify indeed it is an awesome north start to aim at.
  Subject to Mark's time he should also give it a thumbs up either earlier or later in the process before we build it.
- I think this capability can be built upon Scott's Record Later capability, but I am not positive, so this should be verified.
- If Scott and Jack give it a tentative thumbs up (we can do a quick call to talk it thru) then Agus can mock it up.
- It includes a proposed progression of features, so then Brian & Jasu can figure out when various parts would go live.
  (The proposal's step one is matched to the team's current record flow, so it builds from what we are building now... 
  its step two would begin extending beyond what we are currently spec-ing out.)


Scott is currently hammered by fast moving requirements from the Cerebro effort, so we should be sensitive to his time.
Jack if you are confident the design is in a direction that Mark will want, (with Brian's thumbs up), we can mock it up now, that way we can have a more concrete discussion with Mark and Scott before any further planning is done.




(though I am hopeful a tentative thumbs up might happen in a very focused 10 minute conversation...  but of course I have been accused of being an optimist.)

And Brian should give his input before anyone does anything.




~~
**

-   TABS - The top of the page is two tabs.  Most of the time the user is viewing the “Current” recordings list  (these are entries that have not yet been recorded, not yet uploaded, or do not yet have all the info needed in order to properly index and annotate recording.)  
    Once an entry has all of its info and is uploaded, then it is moved to the “Completed” tab.  (Usually a user has little reason to look at this list.  We might even decide to simply remove it, and force them to go to the hoops app to find and/or edit info about completed tiles, since there is not much else they can do with it here.)
    

**





### 2023-04-20  Original Note

SEND TODAY LATER


Brain, you have your hands full right now with weekend issues on Production.  Still I know next sprint we are going to be making fixes to our Record flow.  I think the existing flow has over constrained our users in how info MUST be entered in the record flow.  I want to avoid replacing one over-constrained record ordering with a second-different over-constrained ordering.

Here is the doc I have been trying to pull together on this issue.  Big picture I think we need:
 1 some kind of placeholder UUID or fake game that requires ZERO input from the user or backend to create a new recording
 2 that LATER we can "route" (associate) this placeholder to a game that may or may not exist at recording time
 3 that we can have "soft fields"  fields that can hold arbitrary string OR an enumerated id (for team or such)
The doc says more, but in a nutshell I think those bedrock things might really help us.

Still I recognize realities that we need to make incremental progress, even if you think all of these ideas are great, you wont be trying to implement the whole think in one sprint.  ***This is really in your wheelhouse, so I am leaving all of this up to you***, how/when you want to consider or integrate these ideas with the changes that your team will be making.  Here is a doc that details the ideas.  I am happy to talk about them at your convenience, and/or join calls with others.


For those who don't want to dig into the document, here is the top-level summary of what I see as the target:

_Max Flexibility_. Remove ALL validation & ALL ordering constraints from record app. Specifically:
- Allow recording to be created before/after games are created.
- Allow any info (e. g. team name) to be applied at any point:  
    before recording; before upload; after both but on mobile; before or after but on web.
- Require ZERO internet connection at any point except during upload.
- Allow unvalidated ‘soft’ data input.  E.g.  you can list a recording against the ‘saints’ even before that team has been created, and/or without you knowing how to find that team within the system…   your text is a kind of “comment” on the team name.
- A recording is distinct from a game, and meta data (like opposing team name) can be specified on the recording entry, or on the game entry.  When the recording it ‘routed’ (paired with a game) the meta data can flow in either direction.

Key implementation change:
·      Separating video **_recording_** from **_uploading_** and **_routing_** recording to the proper game.
·      **_Use UUIDs_** to track recordings don’t tie them to games before “routing”.
·      **_Using ‘soft’ fields_** to facilitate partially specified game info.  E. g. allow user to enter any text to remind themselves later of the team, or tournament, etc.







### Camera Recording APP

Stand Alone App - App is fully dedicated to recording&uploading games with no other functionality.

ON BOARDING / FUNCTIONING

DOWNLOAD APP - User downloads app from the Apple App Store, or from the Google Play Store


APP FUNCTIONING
- PRESS RECORD - App starts recording current video input, and associates it with a randomly generated id for this video stream, and the UTC time of start.
- SET TARGET - At anytime prior or during recording the user can select a destination folder from the "teams taxonomy"
- PRESS STOP - When stopping the recording the user is presented with the options to:
	- DISCARD VIDEO
	- UPLOAD to the currently selected target folder
	- SET TARGET

- BACKGROUND UPLOAD - Anytime all of the following are true, the system attempts to perform a background upload
	- AVAILABLE - A completed video is available for upload
	- IDLE - It has been at least n seconds since a video upload was attempted, and the app is not presently uploading a video



