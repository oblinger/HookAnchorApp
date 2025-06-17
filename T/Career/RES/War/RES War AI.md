
# AI-Related War Stories

## Sports Visio
### 2024  SV GALLERY / OCR 
- [Related CVPR paper](https://lukaszgrad.github.io/jnr/) 
- Self-trained per-game p. ayer-detector using a DinoV2 backbone
- Fine-tuned a "digit on fabric" DinoV2-based classifier using a large synthetically in-painted training set.

- [[2024-12-00 OCRv3.pdf]]    (Notion [Grz Notes - OCR v3](https://www.notion.so/sportsvisio/OCR-v3-training-single-shot-person-to-jersey-number-model-d9f4c1b04b15464d951d4c333b53a5d9))

GALLERY
- Pose, Byte-tracker, Object-detector, DinoV2-base, court

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

### 2024 SV Maxim Court Homography

Im using a few  papers just for reference,  but my implementation is an ad hoc version made in house so there is no paper for the current architecture/algorithms Im using.
- Segformer backbone: [https://arxiv.org/abs/2105.15203](https://arxiv.org/abs/2105.15203) 
- CaliKalib (the idea about using a grid came from here): [https://arxiv.org/abs/2209.07795](https://arxiv.org/abs/2209.07795) 
- The idea of UNET for kpoints prediction on non professional courts for different sports: [https://arxiv.org/pdf/2003.14109](https://arxiv.org/pdf/2003.14109)

- Trained a U-Net CNN for 2D homography from diverse nonprofessional court layouts.
### 2024 SV Game Timeouts

### 2024 SV Tracking

The byte tracker: [https://arxiv.org/abs/2110.06864](https://arxiv.org/abs/2110.06864)  
The interpolation tracker, which is used only for the ball, has no paper. It is a layer above that was implemented by me to filter wrong tracks, join and extend other ones. (edited)

### 2023  SV Object Detection

### 2023 SV Object Tracking

## Analytics Fire
### 2018 AF Roof Modeling


### 2019-2021 Instant Design Story
1. Non-ML-based Roof Edge extraction
2. Hand-built energy-based edge cleanup
3. NN-based obstruction detection
4. Combine NN & Geo edge detection


**Edge Extraction**
- find flat planes
- glue together
- derive edges

**Energy based cleanup**
- remove small planes
- remove super sharp corners
- replace line pairs that are nearly 180-degrees
- (sides of roof were not symmetric; fixed this for sales)

**NN MODELS** - pixel level models (RGB+depth)
	- Tree detection
	- obstruction dectection
	- pixel based-edge detection   (weakest model)

NN-models
- Do batch normalization
- Use residual connection
- grab resnet-50.
- Started with plain units.
- We always trained from scratch 

- do contrast learning (never did that to do pretraining)
- 2% delta from best to worse model, so we needed to care.  (semantics segmentation accracy at the pixel level)  
	- was 97 - 98% at pixel level

ultimate failures came from mis labelling one part of roof and failed from there (e.g. mis-labelled one part... and it kills )

- IDEA:  Do object detect, instead of  (use mask CNN to do it)
	- Grabbed the RTR from facebook


OTHER IDEAS:
- RoofGan paper: [https://arxiv.org/abs/2012.09340](https://arxiv.org/abs/2012.09340)  code: [https://github.com/yi-ming-qian/roofgan](https://github.com/yi-ming-qian/roofgan)
- Object detection:  DETR: [https://github.com/facebookresearch/detr](https://github.com/facebookresearch/detr)
- RTF frome facebook

- Hand written  CV
	- derivative of DSM data, flat planes  (Grz Bartchek; MIT)
	- hand written CV geo algorithms
	- apply RGB data (Energy minimization)
		- corners should meet at non-nearly straight angles (energy constraints)
	- NN for detecting obstructions
		- then idea lets swap edges from NN into the existing pipeline for 


## Aeolus

### Base Motion Planning.
#### RRT - planning - Rapidly exploring Random Trees
- https://www.mathworks.com/videos/autonomous-navigation-part-4-path-planning-with-a-and-rrt-1594987710455.html 
- https://mkroehnert.gitlab.io/RRT-Visualization/ 
- Steve LaValle

#### Model Predictive Control (MPC) or Receding Horizon Control.

  The key characteristics you described match MPC perfectly:
  - Optimizes over a finite prediction horizon (your "quarter second")
  - Uses a cost function with multiple objectives (speed changes, obstacle avoidance)
  - Continuously replans as new information becomes available
  - Computationally expensive optimization solved repeatedly

  In robotics/SLAM context, it's often called Receding Horizon Planning or Rolling Horizon Planning.


### U Vienna - Grasping 


### RAP - Reactive Action Planning

