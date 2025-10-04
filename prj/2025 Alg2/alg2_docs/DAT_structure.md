# DAT Structure Documentation

src/utils/annotations/annotations_v2.py


Frame skip moment.   
- so I'm looking for frame 0 in Centerview and I'm looking for the class the runtime me data and the start field. There is my frame offset number for the AI clips.
- and where is it in log file   "skip"  .... "processing"  

- VideoReader CV  -- src/utils/cv2_video_utils.py  -- use the set frame method to skip around. for efficiency.
- in GT.__spec__ TEST  tag   TEST_EDGE   


This document describes the JSON data structures used in the basketball analytics pipeline output files.

---

## shot_predictions.json

Basketball shot prediction annotations with detailed metadata about shot detection and validation.

```yaml
annotations:
  BasketballShotPrediction: list
    - time: str  # Shot time in format "HH:MM:SS.ffffff"
      version: str  # Annotation version (e.g., "0.2.0")
      kind: str  # "BasketballShotPrediction"
      id: str  # Unique shot identifier
      player_color: str  # Jersey color (e.g., "navy", "white")
      player_number: int  # Jersey number
      points: int|null  # Point value (null if unknown)
      made: bool  # Whether shot was made
      court_coordinates: dict|null
        x: float  # Normalized x coordinate on court
        y: float  # Normalized y coordinate on court
      attempt_time: str  # Shot attempt time in format "HH:MM:SS.ffffff"
      metadata: dict
        hoop_id: int  # Tracklet ID of the hoop
        ball_id: int  # Tracklet ID of the ball
        track_id: int|null  # Tracklet ID of shooter
        shooter_id_scenario: str  # Algorithm scenario used for shooter identification
        shot_detection_metadata: dict
          valid_min_samples_hoop: dict
            valid: bool  # Whether hoop has minimum track samples
            hoop_track_len: int  # Length of hoop track
          valid_min_samples_ball: dict
            valid: bool  # Whether ball has minimum track samples
            ball_track_len: int  # Length of ball track
          valid_ball_size: dict
            valid: bool  # Whether ball size is reasonable
            hoop_w: float  # Hoop width in pixels
            ball_w: float  # Ball width in pixels
            max_hoop_multiplier: float  # Maximum ratio threshold
          valid_close_to_hoop: dict
            valid: bool  # Whether ball is close enough to hoop
            ball_w: float  # Ball width in pixels
            dist: float  # Distance to hoop
            thresh_dist: float  # Threshold distance
          valid_pre_trajectory: dict
            valid: bool  # Whether pre-shot trajectory is valid
            rebound_frames: list  # List of rebound frame numbers
            ball_falling_above_hoop: bool  # Whether ball falling before shot
          valid_ball_movement: dict
            valid: bool  # Whether ball movement is sufficient
            movement_deltas: list  # Movement deltas in pixels
            delta_thresh: float  # Movement threshold
          valid_post_trajectory: dict
            valid: bool  # Whether post-shot trajectory is valid
            len_post_trajectory: int  # Number of trajectory samples
            angles: list  # Trajectory angles in degrees
            speeds: list  # Trajectory speeds in pixels/frame
```

---

## event_predictions.json

Basketball event predictions including assists and other game events.

```yaml
annotations:
  BasketballShotAssistPrediction: list
    - time: str  # Event time in format "HH:MM:SS.ffffff"
      version: str  # Annotation version (e.g., "0.1.0")
      kind: str  # "BasketballShotAssistPrediction"
      player: dict
        color: str  # Jersey color (e.g., "navy", "white")
        number: int  # Jersey number
      metadata: dict  # Additional metadata (currently empty)
```
