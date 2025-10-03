# Basketball Game Analysis Algorithms


This document provides an overview of the computer vision algorithms used to analyze basketball games and detect key game events. The algorithms work together to track player actions, ball movement, and game flow to identify specific basketball events like shots, assists, rebounds, and turnovers.

## Rebounds

Our system detects rebounds by tracking who gains possession after a missed shot. We monitor a 3-second window after the ball bounces off the rim, looking for the closest player to the ball based on hand proximity. The approach combines multiple detection methods: direct possession tracking (who physically gets closest to the ball), court flow analysis (whether play continues toward the same basket or reverses), and trajectory analysis (detecting the bounce patterns off the rim). We determine offensive vs defensive rebounds by checking if the rebounder is on the same team as the shooter.

### Rebounder Detection Algorithm
**File**: `src/synch/screbounder.py` calls `src/bball_analyze/action_tracking/rebounder_detector.py`

• **Trigger Condition**: Activates after a missed shot is detected
• **Time Window**: Searches for rebounders within 3 seconds (configurable) after the shot ends
• **Detection Method**:
  - Analyzes ball possession data in frames following a missed shot
  - Identifies the first player to gain possession of the ball after the shot
  - Uses player tracking and ball position data to determine who controls the ball
• **Rebound Classification**:
  - **Offensive Rebound**: Ball possession goes to a player from the same team as the shooter
  - **Defensive Rebound**: Ball possession goes to a player from the opposing team
• **Proximity Analysis**: Calculates distance between ball center and player wrist positions to determine closest player
• **Confidence Scoring**: Uses distance-based confidence calculation where closer distances yield higher confidence scores

### Rebound Qualifier Algorithm
**File**: `src/synch/scrbpreboundqualifier.py` calls `src/bball_analyze/region_based_processing/region_based_processing.py`

• **Purpose**: Determines if a rebound is offensive or defensive using court positioning analysis
• **Method 1 - Shot Timeline Analysis**:
  - If a shot was attempted within the rebound search frame range, analyzes court positioning
  - Offensive rebound: Play switched ends of court after the rebound
  - Defensive rebound: Play remained on the same end of court
• **Method 2 - Player Accumulation Analysis** (`src/bball_analyze/region_based_processing/region_based_action_processing.py`):
  - Analyzes where players are clustering on the court
  - If players accumulate on the same side as the shot location: offensive rebound
  - If players move away from shot location: defensive rebound

### Ball Trajectory Rebound Detection
**File**: `src/bball_analyze/shots/shot_detector.py` function `_find_rebounds()`

• **Analysis Method**: Examines ball trajectory data for sudden changes indicating rebounds
• **Detection Criteria**:
  - High angle changes in ball trajectory (sudden direction shifts)
  - Speed changes in ball movement
  - Both conditions must be met simultaneously
• **Output**: Returns trajectory indexes where rebounds occur for further analysis

## Assists

Our approach works backwards from made baskets to identify the assisting player. We examine a 3-second window before the shot, tracking ball possession frame by frame to find the last teammate who had the ball before the shooter. Detection relies on proximity analysis between the ball and players' hands, with distance thresholds determining possession. The system validates that the passer and shooter are teammates and only processes made shots (missed shots can't have assists). We use confidence scoring based on how close and how long a player controlled the ball before passing.

### Assist Detection Algorithm
**File**: `src/synch/scassister.py` calls `src/bball_analyze/action_tracking/assister_detector.py`

• **Trigger Condition**: Activates only after a made shot is detected (no assists for missed shots)
• **Time Window**: Searches 3 seconds (configurable) backwards from the shot start time
• **Detection Method**:
  - Analyzes ball possession data in frames preceding a made shot
  - Identifies the last player (other than the shooter) from the same team to possess the ball
  - Uses proximity analysis between ball and player wrist positions
• **Team Validation**:
  - Ensures the assisting player is on the same team as the shooter
  - Filters out invalid player colors and numbers
  - Excludes the shooter from being their own assister
• **Proximity Calculation**: Measures euclidean distance between ball center and player right wrist position
• **Confidence Scoring**: Distance-based confidence where:
  - ≤20 pixels: 90-95% confidence
  - 20-40 pixels: 80-90% confidence
  - 40-60 pixels: 70-80% confidence
  - Decreasing confidence as distance increases

### Historical Assist Detection (Legacy)
**File**: `src/bball_analyze/action_tracking/assister_detector.py` function `findAssister()`

• **Frame Analysis**: Examines consecutive frames before shot detection
• **Temporal Separation**:
  - Frames closer to shot (within `framesToLeave`): Tagged as shooter frames
  - Earlier frames: Tagged as potential assist frames
• **Validation Criteria**: Requires 5 consecutive frames with distance ≤50 pixels threshold
• **Player Identification**: Uses majority vote from the 5-frame window to identify the assisting player

## Turnovers

We detect turnovers by monitoring court flow and identifying when play switches direction without a shot attempt. The algorithm tracks which basket each team is attacking and flags possession changes when the ball starts moving toward the opposite end unexpectedly. Our detection logic has two main triggers: play switching ends without a shot, or a shot followed by an offensive rebound (though there's a noted issue with this logic). The system accounts for recent shot attempts and rebound types to avoid false positives. This is a challenging detection because turnovers encompass many different scenarios - steals, bad passes, out of bounds, offensive fouls - and we're essentially inferring them from game flow patterns rather than detecting the specific turnover action.

### Turnover Detection Algorithm
**File**: `src/synch/scrbpturnover.py`

• **Detection Logic**: Identifies turnovers by analyzing court flow and possession changes
• **Primary Conditions for Turnover**:
  - Play switched ends of the court without a shot attempt, OR
  - There was a shot attempt followed by an offensive rebound
• **Court Switching Analysis**:
  - Monitors directional flow using region-based processing
  - Detects when team possession changes from one end of court to the other
  - Uses configurable time offset (default: 2 seconds) to account for tracking delays
• **Shot Integration**:
  - Tracks recent shot attempts and their outcomes
  - Considers relationship between shots, rebounds, and possession changes
• **Rebound Context**: Incorporates rebound type (offensive/defensive) in turnover determination

**Note**: Michael's comments suggest potential issue with logic - algorithm checks for offensive rebounds when it might should check for defensive rebounds.

## Blocks

Our system couldn't detect blocked shots due to a fundamental limitation in our shot detection approach. The shot detection algorithm only triggers when the ball gets close to the hoop, but blocked shots get deflected away before reaching that proximity threshold. From the system's perspective, a blocked shot is indistinguishable from regular ball movement near the basket. We'd need to detect the shooting motion itself (not just ball-to-hoop proximity) to identify when shots are attempted but blocked.

### Block Detection Status
**File**: Referenced in Michael's notes

• **Current Implementation**: No dedicated block detection algorithm exists
• **Technical Limitation**: Shots are only detected when ball approaches the hoop
• **Challenge**: Blocked shots typically don't reach the hoop, making them invisible to the shot detection system
• **Detection Gap**: System cannot distinguish between a missed shot and a blocked shot

## Steals

We didn't implement explicit steal detection - instead, steals are captured within our broader turnover detection. The system recognizes that possession changed but can't distinguish whether it was due to a steal, bad pass, out of bounds, or offensive foul. Proper steal detection would require analyzing the specific moment of ball-defender interaction and differentiating defensive takeaways from other turnover causes. This would need more granular tracking of hand-to-ball proximity during possession changes than our current implementation provides.

### Steal Detection Status
**File**: Referenced in Michael's notes

• **Current Implementation**: No separate steal detection algorithm
• **Relationship to Turnovers**: Steals are a subset of turnovers but not explicitly identified
• **Turnover Sources**: Current turnover detection encompasses multiple causes:
  - Steals (defensive player intercepts ball)
  - Fouls (possession changes due to violations)
  - Defensive rebounds (missed shots leading to possession change)
  - Out-of-bounds plays (ball possession changes)
• **Detection Challenge**: Distinguishing steal-caused turnovers from other turnover types requires additional analysis of player interactions and ball movement patterns

## Algorithm Integration

### Data Flow
• **Input Sources**: Video feeds, object detection (YOLO/RT-DETR), player tracking (ByteTracker), pose estimation
• **Processing Pipeline**: Real-time analysis with frame-by-frame event detection
• **Output**: Structured event data with timestamps, player IDs, confidence scores, and event metadata

### Common Components
• **Player Validation**: All algorithms filter invalid player colors and jersey numbers
• **Distance Calculations**: Euclidean distance between ball and player positions (primarily wrist tracking)
• **Confidence Scoring**: Distance-based confidence metrics across all proximity-dependent algorithms
• **Temporal Windows**: Configurable time ranges for event detection (typically 3 seconds)
• **Team Identification**: Color-based team assignment for proper event attribution