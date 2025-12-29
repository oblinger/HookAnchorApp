.[[RR PAPERS]].  [[LRN Todo]]  [[RR Articles]] 
  [[RR DEFINITIONS]],
  [[RR Blog]],
  , ,
  , [[2025-06-12 LLaVA-ST - Spatio-Temporal Understanding]],
  DELS: [[2026-06-12 LLaVA-ST - Spatio-Temporal Understanding]], 


mamba block

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


## 2025-09-02  "TubeR: Tubelet Transformer for Video Action Detection" (CVPR 2022)
  - Paper: https://arxiv.org/abs/2104.00969
  - Code: https://github.com/salesforce/TubeR
  - Uses "tubelets" (tracks) as queries to transformer
  - Combines track proposals with video features
  - Very close to your use case


## 2025-08-30  [VideoMAE](https://arxiv.org/abs/2203.12602) 

IDEAS:
- Video masked auto encoders are data efficient learners.
- Video reconstruction requires high-level structure learning.

### **VideoMAE (Masked Autoencoders)**
- Self-supervised pre-training approach
- Excellent for leveraging your 100K unlabeled videos
- Fine-tune on labeled subset for shot detection
- State-of-the-art on multiple benchmarks
- Reduces annotation requirements significantly
- **Papers**: 
  - "VideoMAE: Masked Autoencoders are Data-Efficient Learners" (CVPR 2022) - Start here
    - https://arxiv.org/abs/2203.12602 
  - "VideoMAE V2: Scaling Video Masked Autoencoders" (CVPR 2023)
    - https://arxiv.org/abs/2303.16727
- **Code**: https://github.com/MCG-NJU/VideoMAE

#### **VideoMAE V2**
- Latest masked autoencoder approach
- Billion-scale pre-training capability
- Excellent for your 100K video dataset
- Minimal labeled data required
- State-of-the-art transfer learning
- **Paper**: "VideoMAE V2: Scaling Video Masked Autoencoders with Dual Masking" (CVPR 2023)
  - https://arxiv.org/abs/2303.16727
- **Code**: https://github.com/OpenGVLab/VideoMAEv2





### 2025-06-17  [SAM 2: Segment Anything](https://arxiv.org/abs/2408.00714) 

  Title: "SAM 2: Segment Anything in Images and Videos"Authors: Nikhila Ravi + 17 co-authors (Meta AI)arXiv ID: 2408.00714URL:
  

  Key Innovations

  1. Unified Image + Video Segmentation
  - Extends original SAM from images to videos
  - Real-time video processing capabilities
  - Maintains object consistency across frames

  1. Architecture: Transformer + Streaming Memory
  - Memory Encoder: Stores features from past frames
  - Memory Bank: Maintains temporal information
  - Memory Attention: Uses stored info for consistent tracking

  1. Data Engine + Largest Video Dataset
  - Interactive data collection improving model iteratively
  - Created largest video segmentation dataset to date
  - Human-in-the-loop data engine

  Performance Improvements
  - Video: 3x fewer interactions than prior methods
  - Images: 6x faster + more accurate than original SAM
  - Strong performance across wide range of tasks

  Availability
  - Code & model checkpoints on GitHub
  - Interactive demo available
  - Dataset released publicly

  The paper represents a major step forward in foundation models for computer vision, bringing prompt-based segmentation to the temporal domain while improving image performance.





### 2025-06-12  [LLaVA-ST: A Multimodal Large Language Model for Fine-Grained Spatial-Temporal Understanding](https://arxiv.org/pdf/2501.08282)


[[2025-06-12 LLaVA-ST - Spatio-Temporal Understanding]]


### ! 2025-05-14  [DeepSeek-V3: Scaling & HW Architectrues](https://arxiv.org/abs/2505.09343?utm_source=alphasignal) 
### !! 2025-05-27  [Constitutional AI](https://arxiv.org/pdf/2212.08073) 


### 2025-04-22  Code Buff

[[codebuff.pdf]] 


### 2025-04-22  Non-parametric Feature Impact and Importance

[[2006.04750v1.pdf]]


### 2025-02-20  Verify step by step

https://arxiv.org/pdf/2305.20050



### 2024-12-18  DETRs Beat YOLOs on real-time object detection

https://arxiv.org/abs/2304.08069



Outcome-supervised Reward Models (ORMs) -- trained using final result of the model’s chain-of-thought,
Process-supervised Reward Models (PRMs) --  receive feedback for each step in the chain of thought.


Our main contributions are as follows:
1. We show that process supervision can train much more reliable reward
models than outcome supervision. We use our state-of-the-art PRM to
solve 78.2% of problems from a representative subset of the MATH test
set.
2. We show that a large reward model can reliably approximate human supervision for smaller reward models, and that it can be used to efficiently
conduct large-scale data collection ablations.
3. We show that active learning leads to a 2.6× improvement in the data
efficiency of process supervision.
4. We release our full



### !2024-05-13 - [The Platonic Representation Hypothesis](https://arxiv.org/abs/2405.07987)

- Representational Convergence -- The idea that structures in model weights are converging.

Observations
- Splicing lower levels from one transformer onto another with an affine transform works well
- "We might say: all strong models are alike, each weak model is weak in its own way."
- representations are aligned with the structure of human brains
- COLOR - we verified that convergence occurs in real data


Why
- MANY TASKS PIN IT DOWN - The need to solve many tasks at once leads to a few possible solutions.
- SIMPLICITY PINS IT DOWN - 
- [[Contrastive Learning]] - are all gunning for the same optimality point (assuming a BIJECTIVE (lossless) sensing of the world)
- TO - "a statistical model of the underlying reality"
- TO = "a unified model that is proficient across various domains and modalities, grounded in the statistical properties of the underlying world. "

### !2024-06-01 - [Attention is all you need](https://arxiv.org/pdf/1706.03762) 
[Medium Article](https://towardsdatascience.com/attention-is-all-you-need-discovering-the-transformer-paper-73e5ff5e0634):
[[Transformers]] 


### !2013-09-07 - [Efficient Estimation of Word Representations in Vector Space](https://arxiv.org/pdf/1301.3781) 

