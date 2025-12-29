
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


