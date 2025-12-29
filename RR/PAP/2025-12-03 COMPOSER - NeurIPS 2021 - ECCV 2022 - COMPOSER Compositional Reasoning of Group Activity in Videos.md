
## 2025-09 - COMPOSER - NeurIPS 2021 - ECCV 2022 - COMPOSER: Compositional Reasoning of Group Activity in Videos
  Paper: https://arxiv.org/abs/2112.05892
  Project Page: https://sites.google.com/view/composer-action
  Code: https://github.com/hongluzhou/composer

  Authors: Honglu Zhou, Asim Kadav, Aviv Shamsian, Shijie Geng, Farley Lai, Long Zhao, Ting Liu, Mubbasir Kapadia, Hans Peter Graf

  Conference: NeurIPS 2021

 System Inputs and Outputs:
https://www.notion.so/sportsvisio/PROJ-9d1f8e89d87442a7a22e0c2a95561319?source=copy_link

  Inputs:
  {
      # Video data
      'video_frames': tensor,  # [T, H, W, 3] - Full scene video

      # Person tracklets (bounding boxes for each person)
      'person_tracks': [
          {
              'track_id': int,
              'bboxes': [[x1,y1,x2,y2], ...],  # One per frame
              'frames': [0, 1, 2, ...],  # Which frames they appear in
          },
          ... # For all N people in scene
      ],

      # Optional: Pre-extracted features
      'person_features': tensor,  # [N, T, D] - Features per person per frame
  }

  Outputs:
  {
      # Primary: Group activity classification
      'group_activity': str,  # e.g., "team_offense", "fast_break", "set_play"

      # Secondary: Individual actions (discovered latently)
      'individual_actions': [
          {'track_id': 0, 'action': 'shooting'},
          {'track_id': 1, 'action': 'screening'},
          {'track_id': 2, 'action': 'defending'},
          ...
      ],

      # Latent: Discovered sub-activities (not explicitly supervised)
      'latent_sub_activities': tensor,  # [N, K] - K latent activities per person

      # Interaction graph
      'interaction_weights': matrix  # [N, N] - Who influences whom
  }

  Core Architecture:

  Key Components:

  1. Multi-Scale Transformer Encoder
    - Encodes individual person features
    - Encodes group-level features
    - Maintains full scene context
  2. Compositional Reasoning Module
    - Learns latent sub-activities without labels
    - Discovers patterns like "pick-and-roll", "screening"
    - Uses mixture-of-experts approach
  3. Interaction Graph
    - Models relationships between all players
    - Attention weights show who affects whom
    - Preserves spatial relationships
