

## 2025-09 - ICCV 2021 "Learning to Track with Object Permanence" (ICCV 2021)
  Paper: https://arxiv.org/abs/2103.14258
  Project Page: https://sites.google.com/view/object-permanence
  Code: https://github.com/TRI-ML/permatrack
  Authors: Pavel Tokmakov, Jie Li, Wolfram Burgard, Adrien Gaidon (Toyota Research Institute)

System Inputs and Outputs:

  Inputs:
  # Per frame:
  - RGB image (H x W x 3)
  - Detection results from base detector (CenterNet):
    - Bounding boxes [x1, y1, x2, y2]
    - Detection scores
    - Class labels (person, vehicle, etc.)

  # From previous frames:
  - Track history (past trajectories)
  - Track features (appearance embeddings)

  Outputs:
  # Per frame:
  - Active tracks: {
      'track_id': int,
      'bbox': [x1, y1, x2, y2],
      'confidence': float,
      'visibility': ['visible', 'occluded', 'absent'],
      'predicted_position': [x, y]  # Even when occluded
    }

  # Key feature: Maintains tracks even when object not detected

  Core Architecture:

  Main Components:

  1. Detection Module: CenterNet (or any detector)
  2. Appearance Encoder: ResNet features for Re-ID
  3. Prediction Module: Predicts future positions using:
    - Kalman filter for motion
    - Learned offsets for occlusion patterns
  4. Association Module: Hungarian matching with:
    - Spatial distance (IoU)
    - Appearance similarity
    - Motion consistency


  What PermaTrack gives you:
  - Maintains tracks through occlusions (object permanence)
  - Predicts positions during gaps
  - Associates detections across time

  Your advantage:
  - Gallery model provides reliable "anchor" detections
  - Each player is a distinct object class (Player_O1, not generic person)
  - Only need short-term Re-ID between gallery detections

  Perfect fit because:
  - Designed to handle missing detections (your gaps between gallery IDs)
  - Maintains spatial coherence (tracking without seeing)
  - Can treat pre-identified players as different object types

  The code is available and well-documented on their GitHub, making it straightforward to adapt for your gallery-guided tracking approach.

