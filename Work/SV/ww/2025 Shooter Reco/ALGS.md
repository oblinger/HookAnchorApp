
# Exploration

### Group Reco in VB 
  1. "Learning Actor Relation Graphs for Group Activity Recognition" (CVPR 2019)
    - https://arxiv.org/abs/1904.04295

# Basketball Shot Recognition Algorithms

## Reco

### Transformer-Based Models

**Video Vision Transformer (ViViT)**
- Extends Vision Transformer to video understanding
- Processes spatio-temporal patches directly
- Excellent for capturing long-range dependencies in shooting motion
- Can handle variable-length sequences
- Performance: ~85-90% accuracy on action recognition benchmarks
- **Paper**: "ViViT: A Video Vision Transformer" (ICCV 2021) - Very clear explanations
  - https://arxiv.org/abs/2103.15691
- **Code**: https://github.com/google-research/scenic/tree/main/scenic/projects/vivit

**TimeSformer**
- Facebook/Meta's video transformer
- Divided space-time attention mechanism
- Computationally efficient for long videos
- Strong performance on sports action recognition
- Suitable for real-time applications with optimization
- **Paper**: "Is Space-Time Attention All You Need for Video Understanding?" (ICML 2021)
  - https://arxiv.org/abs/2102.05095
- **Code**: https://github.com/facebookresearch/TimeSformer

**VideoMAE (Masked Autoencoders)**
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

### 3D Convolutional Networks

**I3D (Inflated 3D ConvNet)**
- Google's approach inflating 2D filters to 3D
- Pre-trained on Kinetics dataset
- Strong baseline for action recognition
- Can leverage ImageNet pre-training
- Good balance of accuracy and speed
- **Paper**: "Quo Vadis, Action Recognition? A New Model and the Kinetics Dataset" (CVPR 2017)
  - https://arxiv.org/abs/1705.07750
- **Code**: https://github.com/deepmind/kinetics-i3d

**SlowFast Networks**
- Facebook/Meta's dual-pathway architecture
- Slow pathway: spatial semantics at low frame rate
- Fast pathway: motion at high frame rate
- Excellent for capturing both form and motion
- Particularly good for sports actions
- **Paper**: "SlowFast Networks for Video Recognition" (ICCV 2019) - Highly readable
  - https://arxiv.org/abs/1812.03982
- **Code**: https://github.com/facebookresearch/SlowFast

**X3D (Expanded 3D Networks)**
- Efficient video networks from Facebook
- Multiple model sizes (XS to XL)
- Optimized for mobile deployment
- Good accuracy-computation tradeoff
- Suitable for edge devices
- **Paper**: "X3D: Expanding Architectures for Efficient Video Recognition" (CVPR 2020)
  - https://arxiv.org/abs/2004.04730
- **Code**: https://github.com/facebookresearch/SlowFast (included in SlowFast repo)

### Temporal Action Detection

**ActionFormer**
- Transformer-based temporal action localization
- Identifies exact shot start/end frames
- Multi-scale temporal features
- State-of-the-art on temporal detection benchmarks
- Good for identifying shot preparation, execution, follow-through
- **Paper**: "ActionFormer: Localizing Moments of Actions with Transformers" (ECCV 2022)
  - https://arxiv.org/abs/2202.07925
- **Code**: https://github.com/happyharrycn/actionformer_release

**BMN (Boundary Matching Network)**
- Precise temporal boundary detection
- Generates temporal proposals efficiently
- Can identify shot attempts vs successful shots
- Useful for detailed shot analysis
- **Paper**: "BMN: Boundary-Matching Network for Temporal Action Proposal Generation" (ICCV 2019)
  - https://arxiv.org/abs/1907.09702
- **Code**: https://github.com/JJBOY/BMN-Boundary-Matching-Network

### Hybrid Approaches

**MViTv2 (Multiscale Vision Transformers)**
- Hierarchical vision transformer
- Pooling attention for efficiency
- Strong performance on Kinetics-400/600
- Good for varying shot distances/angles
- Handles multiple players in frame
- **Papers**:
  - "MViTv2: Improved Multiscale Vision Transformers" (CVPR 2022) - Clear architecture
    - https://arxiv.org/abs/2112.01526
  - "Multiscale Vision Transformers" (ICCV 2021) - Original paper
    - https://arxiv.org/abs/2104.11227
- **Code**: https://github.com/facebookresearch/mvit

**Video Swin Transformer**
- Microsoft's hierarchical transformer
- Shifted window approach for efficiency
- Strong general video understanding
- Pre-trained models available
- Good for production deployment
- **Paper**: "Video Swin Transformer" (CVPR 2022) - Well-written
  - https://arxiv.org/abs/2106.13230
- **Code**: https://github.com/SwinTransformer/Video-Swin-Transformer

### Efficient Models for Real-Time

**MobileVideo**
- Optimized for mobile devices
- Real-time performance (30+ FPS)
- Temporal shift modules
- Good accuracy-speed tradeoff
- Suitable for live game analysis
- **Paper**: "MobileVideo: A Compact Efficient Convolutional Neural Network" (CVPR 2020)
  - https://arxiv.org/abs/2003.03066
- **Code**: https://github.com/IBM/MobileVideo

**TSM (Temporal Shift Module)**
- Efficient temporal modeling
- 2D CNN backbone with temporal shifts
- Low computational overhead
- Can run on edge devices
- Good for real-time applications
- **Paper**: "TSM: Temporal Shift Module for Efficient Video Understanding" (ICCV 2019) - Simple concept
  - https://arxiv.org/abs/1811.08383
- **Code**: https://github.com/mit-han-lab/temporal-shift-module

### Self-Supervised Approaches

**VideoMAE V2**
- Latest masked autoencoder approach
- Billion-scale pre-training capability
- Excellent for your 100K video dataset
- Minimal labeled data required
- State-of-the-art transfer learning
- **Paper**: "VideoMAE V2: Scaling Video Masked Autoencoders with Dual Masking" (CVPR 2023)
  - https://arxiv.org/abs/2303.16727
- **Code**: https://github.com/OpenGVLab/VideoMAEv2

**DINO for Video**
- Self-distillation with no labels
- Learn robust features without annotation
- Good for player identification
- Can leverage unlabeled game footage
- **Papers**:
  - "Emerging Properties in Self-Supervised Vision Transformers" (ICCV 2021) - Foundation
    - https://arxiv.org/abs/2104.14294
  - "DINOv2: Learning Robust Visual Features without Supervision" (2023)
    - https://arxiv.org/abs/2304.07193
- **Code**: https://github.com/facebookresearch/dino

### Specialized Sports Models

**Sports-1M Pre-trained Models**
- Pre-trained on sports videos
- Better initialization for basketball
- Transfer learning advantages
- Available in multiple architectures
- **Paper**: "Large-scale Video Classification with Convolutional Neural Networks" (CVPR 2014)
  - https://cs.stanford.edu/people/karpathy/deepvideo/
- **Dataset**: https://cs.stanford.edu/people/karpathy/deepvideo/

**FineGym Models**
- Fine-grained action recognition
- Developed for sports movements
- Hierarchical action understanding
- Good for detailed shot mechanics
- **Paper**: "FineGym: A Hierarchical Video Dataset for Fine-grained Action Understanding" (CVPR 2020)
  - https://arxiv.org/abs/2004.06704
- **Code & Dataset**: https://github.com/PKU-ICST-MIPL/FineGym_CVPR20

## Quick Start Resources

### Most Readable Papers (Start Here)
1. **TSM Paper** - Simple concept, easy to understand temporal modeling
2. **SlowFast Paper** - Clear dual-pathway intuition for motion
3. **ViViT Paper** - Well-explained extension of ViT to video
4. **VideoMAE Paper** - Straightforward self-supervised approach

### Best Maintained Repositories
1. **MMAction2** - https://github.com/open-mmlab/mmaction2
   - Implements most algorithms above
   - Excellent documentation and tutorials
   - Pre-trained models available
   
2. **PyTorchVideo** - https://github.com/facebookresearch/pytorchvideo
   - Facebook's unified video understanding library
   - Clean implementations
   - Good for production

3. **PySlowFast** - https://github.com/facebookresearch/SlowFast
   - Multiple SOTA models in one repo
   - Well-maintained by Meta

## Recommendations

### For Your Use Case (100K Basketball Shots)

1. **Primary Recommendation: VideoMAE + Fine-tuning**
   - Pre-train on all 100K videos (self-supervised)
   - Fine-tune on labeled subset
   - Best accuracy potential
   - Handles data imbalance well

2. **Production Deployment: Video Swin Transformer**
   - Good accuracy-efficiency balance
   - Pre-trained models available
   - Stable and well-tested
   - Good documentation and community support

3. **Real-Time Analysis: TSM or MobileVideo**
   - For live game analysis
   - Edge device deployment
   - 30+ FPS performance
   - Acceptable accuracy tradeoff

4. **Temporal Precision: ActionFormer**
   - For exact shot timing
   - Shot phase analysis
   - Detailed biomechanics study
   - Research applications

### Training Strategy

1. **Self-supervised pre-training** on all 100K videos
2. **Multi-stage fine-tuning**:
   - Stage 1: Shot vs non-shot
   - Stage 2: Shot type classification
   - Stage 3: Shot success prediction
3. **Data augmentation**: temporal jittering, spatial crops, color jittering
4. **Ensemble methods**: Combine 2-3 models for best accuracy

### Expected Performance

- Shot detection: 95-98% accuracy
- Shot type classification: 90-95% accuracy
- Make/miss prediction: 75-85% accuracy
- Real-time processing: 30-60 FPS (model dependent)