# Classification Approaches

For the basketball make/miss/no-shot task, considering both accuracy and GPU performance.

*Ordered by recommendation priority*

## 1. Temporal Shift Networks (TSM)
TSM shifts a portion of feature channels along the temporal dimension, enabling information exchange between neighboring frames at zero computational cost and zero parameters. It can be inserted into any 2D CNN backbone (ResNet, EfficientNet) to add temporal modeling while maintaining 2D CNN efficiency, achieving 3D CNN performance with 2D CNN complexity.
- Pros: Extremely efficient, uses 2D CNN backbone, minimal overhead for temporal modeling
- Cons: May miss fine-grained temporal details
- GPU Performance: Excellent (nearly 2D CNN speed)
- Suitability: **Top choice** - best balance of speed and accuracy; minimal overhead over 2D CNN; proven effective for temporal action recognition
- [TSM: Temporal Shift Module for Efficient Video Understanding](https://openaccess.thecvf.com/content_ICCV_2019/papers/Lin_TSM_Temporal_Shift_Module_for_Efficient_Video_Understanding_ICCV_2019_paper.pdf)

## 2. X3D (Efficient 3D CNNs)
X3D progressively expands a tiny 2D image classification architecture along multiple axes (space, time, width, depth) using stepwise network expansion. It achieves state-of-the-art performance while requiring 4.8× fewer multiply-adds and 5.5× fewer parameters than comparable methods, with the key insight that high spatiotemporal resolution can work well with extremely light network width.
- Pros: Designed for efficiency, progressive expansion approach, good accuracy/speed tradeoff
- Cons: Still fairly complex compared to 2D approaches
- GPU Performance: Better than traditional 3D CNNs
- Suitability: **Strong candidate** - balances 3D spatiotemporal modeling with efficiency; modern architecture with strong benchmarks; well-supported via PyTorch
- [X3D: Expanding Architectures for Efficient Video Recognition](https://openaccess.thecvf.com/content_CVPR_2020/papers/Feichtenhofer_X3D_Expanding_Architectures_for_Efficient_Video_Recognition_CVPR_2020_paper.pdf)

## 3. CNN + LSTM (LRCN Architecture)
LRCNs process variable-length visual input with a CNN (e.g., ResNet, EfficientNet), whose outputs are fed into stacked recurrent sequence models (LSTMs), which produce predictions. The CNN extracts per-frame spatial features, while the LSTM models temporal dependencies across the sequence, with both components trained end-to-end.
- Pros: Flexible, can use strong pretrained models, interpretable
- Cons: RNNs harder to parallelize, may struggle with long sequences
- GPU Performance: Good (2D CNN backbone is fast)
- Suitability: **Good baseline** - simple, interpretable, easy to debug and iterate; leverages strong pretrained 2D CNNs; good starting point for development
- [Long-term Recurrent Convolutional Networks for Visual Recognition and Description](https://openaccess.thecvf.com/content_cvpr_2015/papers/Donahue_Long-Term_Recurrent_Convolutional_2015_CVPR_paper.pdf)

## 4. Lightweight 2D CNN + Temporal Pooling
Uses a pretrained 2D CNN (EfficientNet, MobileNet) to extract features from each frame independently, then aggregates temporal information via pooling (max, average) or simple attention mechanisms before the classification head. Minimal architectural complexity beyond standard image classification.
- Pros: Very fast, simple, uses proven architectures
- Cons: May miss complex temporal patterns
- GPU Performance: Excellent
- Suitability: **Best for prototyping** - fastest inference option; simplest implementation; may be sufficient given constrained problem space (3 classes, centered shots); excellent for rapid prototyping and establishing baselines
- [Temporal Segment Networks: Towards Good Practices for Deep Action Recognition](https://arxiv.org/pdf/1608.00859)

---

## Alternative Approaches

*Not prioritized for this task*

### 5. 3D Convolutional Networks (C3D / I3D)
Traditional video classification approach that processes spatial and temporal dimensions together.
- Pros: Proven for action recognition, naturally captures spatio-temporal features
- Cons: Computationally expensive, large memory footprint
- GPU Performance: Moderate (heavy operations)
- Suitability: Good for accuracy, but likely overkill for ternary classification

### 6. Two-Stream / Late Fusion Approaches
Separate spatial and temporal pathways that merge predictions.
- Spatial stream: Process individual frames
- Temporal stream: Process optical flow or frame differences
- Pros: Can use pretrained 2D models, interpretable components
- Cons: Requires computing optical flow (adds preprocessing), two models to train
- GPU Performance: Good (2D CNNs are efficient)
- Suitability: Good option, especially if optical flow is already available

### 7. SlowFast Networks
Dual-pathway architecture: slow pathway (high spatial, low temporal) + fast pathway (low spatial, high temporal).
- Pros: Excellent accuracy on action recognition, captures both detailed appearance and fast motion
- Cons: Complex architecture, heavy computation
- GPU Performance: Moderate
- Suitability: Good for accuracy if computational budget allows

### 8. Vision Transformers for Video (TimeSformer / ViViT)
- Pros: State-of-the-art accuracy on many benchmarks, captures long-range dependencies
- Cons: Requires large datasets, computationally expensive, slower inference
- GPU Performance: Poor to moderate (self-attention is expensive)
- Suitability: May be overkill, unless we have massive datasets

## Implementation Considerations

1. **Transfer Learning**: All approaches benefit from pretraining on large video datasets (Kinetics, Sports-1M)
2. **Input Resolution**: Basketball hoop area is relatively small - may need higher resolution than typical action recognition
3. **Frame Rate**: Shot trajectory is fast - need sufficient temporal resolution (probably 15-30 fps minimum)
4. **Data Efficiency**: Given limited training data, simpler models (TSM, CNN+LSTM) may generalize better than complex transformers
