# 001 Problem Framing - Basketball Shooter Identification

## Problem Statement

Develop a supervised learning system that identifies which player took a shot in basketball game footage, given video clips and player tracking metadata. The system must handle cases where multiple players are near the shot origin and use visual cues (pose, shooting motion) to determine the actual shooter.

## Core Constraints

### Known Information
- **Shot origin**: Exact pixel location where shot originated (center of image)
- **Shot timing**: Exact frame when ball was launched (middle of sequence)
- **Video window**: 2-3 seconds before and after shot moment
- **Player tracking**: Bounding boxes and player IDs from existing detectors

### Challenge
When multiple offensive players are at the shot location, the system must analyze detailed pose and motion to identify which player is actually shooting.

## System Inputs

### Training Time
1. **Video clips**
   - Duration: ~2 seconds before and after shot
   - Shot moment: Exactly at middle frame
   - Shot location: Center pixel of frame
   - Resolution: Standard video (e.g., 1920x1080)
   - Frame rate: 30 FPS

2. **Player tracking data per frame**
   - Bounding boxes for all detected persons
   - Player IDs (when available):
     - Specific player identity (jersey number)
     - Referee designation
     - Non-player designation
   - Some detections may lack player ID
   - Multiple detections possible for same player (tracking uncertainties)

3. **Ground truth**
   - Identity of actual shooter (for supervised learning)

### Inference Time
- Same as training inputs (video + tracking data)
- No ground truth needed
- Must predict: Which player is the shooter

## System Outputs

### Primary Output
- **Shooter identification**: Which player took the shot
  - Could be jersey number
  - Or offensive player index (O1-O5)
  - Confidence score

### Potential Additional Outputs
- Confidence scores for each candidate player
- Pose keypoints at shot moment
- Uncertainty measures when identification is ambiguous

## Existing System Context

### Already Implemented Components
1. **Shot detection system**
   - Detects shot attempts via ball trajectory analysis
   - Provides shot origin pixel and frame
   - Tracks ball arc

2. **Player detection pipeline**
   - DETR-based person detection
   - Pose estimation for detected players
   - Jersey number recognition (specialized classifier)
   - Gallery-based player identification model
   - Multi-object tracking across frames

3. **Court analysis**
   - Court boundary detection
   - Zone classification (2-point, 3-point, free-throw)

### Integration Points
The shooter identification component will:
- Receive inputs from existing detection/tracking systems
- Focus specifically on the "who shot" question
- Leverage known shot location/timing to narrow search space
- Output results to shot record generation system

## Key Technical Challenges

1. **Occlusion handling**: Players often partially block each other near the basket
2. **Track uncertainty**: Detection/tracking may lose players or mix identities
3. **Multiple candidates**: Several players may be at shot origin location
4. **Pose discrimination**: Must identify shooting motion vs other actions
5. **Limited visibility**: Jersey numbers often not visible at shot moment
6. **Temporal association**: May need to look before/after shot for clear player ID

## Data Characteristics

### Available Training Data
- ~100,000 basketball shots with video
- Existing labels for shooter identity
- Player tracking data from existing pipeline
- Gallery of player appearances per game

### Data Representation Considerations
- Could use court-space coordinates (top-down view)
- Or image-space coordinates (camera view)
- Tracking data may be sparse/incomplete
- Need to handle variable number of players (0-10)

## Success Metrics

### Primary Metric
- Shooter identification accuracy (% correct)

### Secondary Metrics
- Performance on occluded cases
- Accuracy when multiple players present
- Confidence calibration
- Processing speed for real-time applications

## Proposed Approach Overview

Develop a multi-modal system that:
1. Processes video to extract visual shooting cues
2. Incorporates player tracking metadata
3. Focuses attention on shot origin location
4. Uses temporal context when shot moment is ambiguous
5. Outputs player identity with confidence

The system should leverage the known shot location and timing as strong priors, while using video analysis to resolve ambiguity when multiple players are present.