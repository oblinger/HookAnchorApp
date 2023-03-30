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



