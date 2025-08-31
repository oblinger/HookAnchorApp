
#cv 

ViT
-?-
  A Vision Transformer (ViT) used as the core feature extractor in a computer vision model.
  - Applies transformer architecture (from NLP) to images
  - Splits image into patches (e.g., 16x16 pixels)
  - Treats patches like "words" in a sentence
  - Uses self-attention to understand relationships

  "Backbone":
  - The main feature extraction network
  - Like the "spine" of the model
  - Extracts representations that other parts use
  - Usually pre-trained on large datasets

  How It Works:

  Image → [Patch] [Patch] [Patch] → Transformer → Features
           16x16   16x16   16x16     Layers        ↓
                                                 Task Head
                                              (Classification,
                                               Detection, etc.)

  Common ViT Backbones:
  - ViT-B (Base): 86M parameters
  - ViT-L (Large): 307M parameters
  - ViT-H (Huge): 632M parameters
  - DeiT (Data-efficient ViT)
  - Swin Transformer (Hierarchical ViT)

  Used In:
  - CLIP - ViT backbone for vision
  - DINO/DINOv2 - Self-supervised ViT
  - SAM - Segment Anything Model
  - Detection models - DETR variants
  - Video models - TimeSformer, ViViT

  Why Popular:
  - Better than CNNs for many tasks
  - Scales well with data/compute
  - Unified architecture for multiple modalities
  - Strong transfer learning capabilities

  The ViT backbone revolutionized computer vision by showing transformers could match/beat CNNs!