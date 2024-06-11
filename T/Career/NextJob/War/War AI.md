
# AI-Related War Stories


### 2024  SV GALLERY / OCR Model

- [Notes](https://www.notion.so/sportsvisio/OCR-v3-training-single-shot-person-to-jersey-number-model-d9f4c1b04b15464d951d4c333b53a5d9) 

OCR
- Use of pose to focus digit reco is crucial
- Causes clipping bugs
- Best to build per digit classifiers since body rounding/clipping on left and right are different.
- trained our model on 120K raw images filtered to 40K edge cases.
- Tried Vit and DinoV2 (base & large)
  went with DinoV2 base model.

COLOR
- 7 colors 10K images

PLAYER ID
- Using whole feature vector

POSE
- https://arxiv.org/abs/2204.12484

MADE-MISS
- https://arxiv.org/abs/2004.04730
- Trained on hundreds of examples


### 2024  SV Court Homography

### 2023  SV Object Detection

### 2023 SV Object Tracking

### 2018 AF Roof Modeling

