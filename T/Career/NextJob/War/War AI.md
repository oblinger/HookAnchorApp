
# AI-Related War Stories


### 2024  SV GALLERY / OCR Model

- [Grz Notes - OCR v3](https://www.notion.so/sportsvisio/OCR-v3-training-single-shot-person-to-jersey-number-model-d9f4c1b04b15464d951d4c333b53a5d9) 

OCR
- SEPARATE HEADS PER DIGIT:
- TORSO CROPPING: crop using pose points - goldilocks amount of cropping.
- BACKBONES: DinoV2-base fp16 is best (lg not worth it); Vision Transformer not as good.
- GELU Head: 
- ADD DON'T KNOW:

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
- Could we benefit from a better pose estimator? Yes.  But given potential gain vs cost I would put this very far on the list of things we could do.

MADE-MISS
- https://arxiv.org/abs/2004.04730
- Trained on hundreds of examples
- I haven't analyzed it yet. There are models with higher scores in benchmarks, but we care both about accuracy and computational cost. So I'm not sure I would stick to the same model or change, but we for sure need more data. Current model is trained on a couple hundreds of examples.


### 2024  SV Court Homography - DL & Pano

### 2024 SV Game Timeouts

### 2023  SV Object Detection

### 2023 SV Object Tracking

### 2018 AF Roof Modeling

