v   m # Make/Miss Dataset

Dataset for training shot outcome classification (made vs missed shots).

## Dataset Format

### Directory Structure

```
dataset/
  metadata.json          # All sample annotations
  clips/
    shot_001.mp4         # Video clip around shot attempt
    shot_002.mp4
    non_shot_001.mp4     # Negative samples (no shot)
    ...
```

### Metadata JSON

```json
{
  "samples": [
    {
      "id": "shot_001",
      "game_id": "G1",
      "label": "made",              // "made", "miss", or "no_shot"
      "start_frame": 1000,          // First frame of clip
      "end_frame": 1100,            // Last frame of clip
      "shot_frame": 1050,           // Frame where shot occurs (null for no_shot)
      "player_color": "navy",
      "player_number": 3,
      "points": null,
      "court_coordinates": {
        "x": 0.15,
        "y": 0.07
      },
      "video_path": "clips/shot_001.mp4"
    }
  ]
}
```

### Fields

- **id**: Unique sample identifier
- **game_id**: Source game identifier
- **label**: Shot outcome (made/miss/no_shot)
- **start_frame**: First frame number in clip
- **end_frame**: Last frame number in clip
- **shot_frame**: Frame number where shot attempt occurs (null for no_shot samples)
- **player_color**: Jersey color of shooter
- **player_number**: Jersey number of shooter
- **points**: Point value (null if unknown)
- **court_coordinates**: Normalized shot location on court (null if unavailable)
- **video_path**: Path to video clip relative to dataset root

---

## Make/Miss Dataset Builder

The dataset builder extracts video clips from game footage using shot annotations.

### Approach

**Shot Extraction:**
1. Parse `shot_predictions.json` to get shot timing and outcomes
2. Convert attempt time to frame number using video FPS
3. Extract clip window around shot (e.g., ±30 frames from shot_frame)
4. Write MP4 clip named `{game_id}_{shot_num:03d}.mp4`
5. Store metadata with label (made/miss), frame ranges, and player info

**Non-Shot Extraction:**
1. Calculate number of non-shots: `num_shots × non_shot_ratio`
2. Sample random time windows where hoop is visible
3. Avoid exclusion zones (shot frames ± buffer)
4. Verify hoop visibility in frame (basic: random sampling; advanced: check hoop tracks)
5. Write MP4 clips named `{game_id}_nonshot_{num:03d}.mp4`
6. Store metadata with label "no_shot"

**Multi-Game Building:**
1. Iterate through all games in runsets directory
2. Extract shots and non-shots for each game
3. Merge all samples into single metadata.json
4. `non_shot_ratio=1.0` creates balanced dataset (equal shots/non-shots)

**Advanced Non-Shot Sampling (Future):**
- Track ball trajectory near hoop using `object-tracks-basketball` from pickle
- Find intervals where ball passes near hoop but no shot is annotated
- Provides harder negative examples for classifier training
