

## Segment and Track Anything
https://github.com/z-x-yang/Segment-and-Track-Anything

#cv 

**Segment and Track Anything**
-??-
- open-source project,
- focuses on the segmentation and tracking of any objects in videos, 
- utilizing both automatic and interactive methods. 
- The primary algorithms utilized include
  - [**SAM** (Segment Anything Models)](https://github.com/facebookresearch/segment-anything) for automatic/interactive key-frame segmentation and 
  - the [**DeAOT** (Decoupling features in Associating Objects with Transformers)](https://github.com/yoxu515/aot-benchmark) (NeurIPS2022) for efficient multi-object tracking and propagation. 
- The SAM-Track pipeline enables dynamic and automatic detection and segmentation of new objects by SAM, while DeAOT is responsible for tracking all identified objects. <!--SR:!2025-02-22,3,172!2025-03-08,59,286-->


[**DeAOT**](https://github.com/yoxu515/aot-benchmark)
-??-
(Decoupling features in Associating Objects with Transformers).  Use in Segment and Track Anything to track objects. [**DeAOT**](https://github.com/yoxu515/aot-benchmark), [Paper](https://arxiv.org/pdf/2210.09782):
- method of hierarchical propagation for semi-supervised Video Object Segmentation (VOS).
- Based on vision transformer: Associating Objects with Transformers (AOT)
- Approach introduces hierarchical propagation into VOS.
- The hierarchical propagation can gradually propagate information from past frames to the current frame and transfer the current frame feature from object-agnostic to object-specific. 
- However, the increase of object-specific information will inevitably lead to the loss of object-agnostic visual information in deep propagation layers. To solve such a problem and further facilitate the learning of visual embeddings, this paper proposes a Decoupling Features in Hierarchical Propagation (DeAOT) approach. 
- Firstly, DeAOT decouples the hierarchical propagation of object-agnostic and object-specific embeddings by handling them in two independent branches. 
- Secondly, to compensate for the additional computation from dual-branch propagation, we propose an efficient module for constructing hierarchical propagation, i.e., Gated Propagation Module, which is carefully designed with single-head attention. <!--SR:!2025-03-07,16,212!2025-03-05,14,266--> 

Extensive experiments show that DeAOT significantly outperforms AOT in both accuracy and efficiency. On YouTube-VOS, DeAOT can
achieve 86.0% at 22.4fps and 82.0% at 53.4fps. Without test-time augmentations,
we achieve new state-of-the-art performance on four benchmarks, i.e., YouTube-
VOS (86.2%), DAVIS 2017 (86.2%), DAVIS 2016 (92.9%), and VOT 2020 (0.622).
Project page: https://github.com/z-x-yang/AOT.


<!--SR:!2000-01-01,1,250!2024-09-26,4,270-->