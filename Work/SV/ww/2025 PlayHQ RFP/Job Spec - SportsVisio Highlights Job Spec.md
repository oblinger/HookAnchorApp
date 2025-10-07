# SportsVisio Highlights Job Spec

A **highlights** job processes a basketball video, extracts the detected shots from it, and returns them as a json payload.

# Highlights job spec

The highlights job spec that should be provided to the `POST /v1/jobs` job API endpoint request under the `job` field is:

```json
{
	"type": "highlights",
	"video_url": "<video-url-here>",  // URL to the video
	"roster": {  // Roster for this game (or null if unavailable)
		"team_1_color_or_id": [1, 2, 3, 4, 5, 6, 7],  // Mapping from team_id to the team's jersey numbers
		"team_2_color_or_id": [4, 5, 6, 7, 8, 9, 10, 11, 12]
	}
}
```

### video_url

- The `video_url` field may point to a direct download link for an `.mp4` video file, or it may be an HTTP Live Streaming (HLS) url

### roster

- If available, a roster can be provided as a mapping of team_ids to the jersey numbers of the players from each team

---

# Highlights job results

The structure of the results of a highlights job, which is provided as the response of the `GET /v1/jobs/{job_id}` endpoint under the `results` field is:

```json
{
	"sport": "basketball",  // Currently, only "basketball" is implemented
	"version": "v0.1.0",  // Semantic version of the schema
	"teams": {
		"my_team_1_id": {},  // Team ID. Currently there are no team-specific attributes
		"my_team_2_id": {},
	},
	"players": {
		"my_player_1_id": {  // Player ID
			"team_id": "my_team_1_id",  // Reference to the team the player belongs to
			"jersey_number": 30,  // The player's jersey number
			"jersey_color": "blue",  // The player's jersey color
		},
		"my_player_2_id": {...},
	},
	"events": [
		<HIGHLIGHT_EVENT1>, 
		...]  // Events detected during the game
}
```

### version

- The version of the spec follows semantic versioning, where:
    - Major changes indicate a modification in structure or a change in the types of values accepted in each field
    - Minor changes indicate the addition of a new field

### teams and players

- These fields describe the teams and players involved in a game
- Their IDs reference teams and players in other objects (like events), so they must be **unique within a game**
- If the client provides the roster for a game (i.e. in the highlights job spec under `roster`), they may use whatever arbitrary IDs they choose, and the system will re-use them when providing results
- If the client doesn’t provide a roster, the player and team IDs will be auto-generated. The guidelines (but no guarantees) as to how the system auto-generates IDs are:
    - Team IDs will be the team’s main solid jersey color (e.g. `blue`)
    - Player IDs will be a combination of [jersey_color]_[jersey_number] (e.g. `blue_10`)
    - The system **may** generate IDs that don’t match the above standard in special cases, so it’s not recommended to rely on the IDs’ structure

### events

- Lists all events found during the analysis of a game

---

# HIGHLIGHT_EVENT

The highlights job may detect and export several types of events. Each event has a `type` that defines its structure.

## Events of type `shot`

The structure individual  `shot` events found in the Highlights job results under the `events` field is: 

```json
{
	"id": "9a47ce54-d947-4725-8ba3-76d08b638786",  // Unique UUIDv4 event identifier
	"type": "shot",  // Type of event ('shot' for shot events)
	"player_id": "blue_4",  // Player ID corresponding to this event (or null)
	"start_time": 123.40,  // Start time of the event in seconds
	"end_time": 567.80,  // End time of the event in seconds
	"made": true,  // Whether the shot attempt was made (true) or missed (false)
	"shooter_track": [  // List of track data for the shooter (or null)
		{
			"time": 123.40,  // Instant time of this track data in seconds
			"bbox": [0.1, 0.1, 0.2, 0.2]  // Coordinates of the shooter bbox in (tl_x, tl_y, br_x, br_y)
		},
		...
	],
	"ball_track": [  // List of track data for the ball (or null)
		{
			"time": 123.40,  // Instant time of this track data in seconds
			"bbox": [0.05, 0.05, 0.07, 0.07]  // Coordinates of the ball bbox in (tl_x, tl_y, br_x, br_y)
		},
		...
	] 
}
```

### player_id

- Reference to the player ID that attempted the shot
- If there was a shot detected, but the system was unsure about the player ID that attempted it (or only has partial information), `player_id` will be `null`

### shooter_track and ball_track

- The system may have the bounding box location of the shooter and the ball for every frame during the period of a shot. The data for both may be exposed in the `shooter_track` and `ball_track` fields, respectively.
    - `shooter_track` - Track data corresponding to the player that attempted a shot
    - `ball_track` - Track data corresponding to the ball used in a shot
    - Track data will be provided for every frame between the shot attempt (around the ball release moment) and the shot outcome moment (when the shot misses or goes through the hoop)
- All bounding box coordinates are provided in the form `(tl_x, tl_y, br_x, br_y)`
    - Where the origin of the frame is on the top-left corner
    - Coordinates are provided in relative form, as floats between 0 and 1 (inclusive)
    - X coordinates are relative to the frame’s width and y coordinates to the frame’s height

---

# Discussion points

### Providing a roster

- When the client submits a job in `POST /v1/jobs`, we’d like to open up the possibility to include a roster for the game
    - The format to provide a roster is yet TBD, but the one proposed is the one seen in the [Highlights job spec](https://www.notion.so/Highlights-job-spec-279719d6f2bd8063a0c3f7efd4e62990?pvs=21) section
    - Notice that we’re not specifying any colors or player IDs here. We can include those in the schema as well, depending on the information that is available by the time the request is made.

### Providing time for instant track data

- The `time` field in the instant track data is in seconds. This is unusual, since it would be more natural to provide it in frames.
- The reason behind this is to provide a video-agnostic time (just like `start_time` and `end_time` in the event). However, given that a bbox location is inherently video-specific, we could provide the frame number instead, and either keep `start_time` and `end_time` in seconds, or also use frame numbers and tie the whole event to a specific video.

### Video frame rate

- Presently, the system is limited to handling videos of constant frame rates of 30 fps, and a maximum duration of 2 hrs
- We’ve noticed that some videos are at 60 fps. While we won’t handle that frame rate directly (due to memory constraints), we can adapt the system to down sample the framerate to 30 fps and process it that way. Let us know if this is a use case of interest, so we can make the necessary adjustments and implement this functionality

### Discrete color options

- Fields that reference a color (e.g. `jersey_color`) can have the following values: `grey`, `black`, `green`, `purple`, `blue`, `brown`, `orange`, `red`, `white`, `yellow`, `pink`, `navy`, `fuchsia`, `lime`, `aqua`, `silver`
- These can evolve in the future, as the system supports more nominal colors