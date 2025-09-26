# Jersey Number OCR System

The Jersey Number OCR System is a specialized optical character recognition pipeline designed to read jersey numbers from basketball players in video footage. The system combines text detection, character recognition, and statistical fusion to reliably identify player numbers despite challenging conditions like motion blur, occlusion, and varying lighting.

## High-Level Overview

The OCR system reads jersey numbers by first detecting text regions on player jerseys, then recognizing the individual digits. The system is specifically trained to handle the unique challenges of sports jersey recognition, including the fact that numbers can be single or double digits, may be partially occluded, and appear under various viewing angles and lighting conditions.

**Core Approach**: Player Crop → Text Detection → Digit Recognition → Statistical Fusion → Final Number
**Training Strategy**: Synthetic data generation using inpainting techniques to create large-scale training datasets

## System Architecture

### Two-Stage OCR Pipeline

#### Stage 1: Text Detection
**Purpose**: Locate jersey number regions within player crops
**Model**: PaddleOCR-based text detector (DB algorithm family)
**Input**: Player torso crop images (focused on jersey area)
**Output**: Quadrilateral bounding boxes around detected text regions
**Post-processing**: Perspective correction to convert angled text regions to rectangular crops

#### Stage 2: Digit Recognition
**Purpose**: Recognize individual digits from detected text regions
**Models**: Multiple recognition approaches supported
- **DINO v2 Double Digit Model**: Primary recognition system
- **Deep Text Recognition**: Alternative/backup recognition
- **PaddleOCR Recognition**: Traditional OCR approach

### ML Model Components

#### 1. PaddleOCR Text Detection (Paddle2PytorchDetector)
**Architecture**: DB (Differentiable Binarization) neural network
**Training Data**: General text detection datasets optimized for sports contexts
**Key Features**:
- **Batch Processing**: Optimized for processing multiple player crops simultaneously
- **GPU Acceleration**: CUDA support with optional FP16 precision
- **Configurable Post-processing**: Adjustable confidence thresholds and box extraction parameters

**Detection Parameters**:
- `det_db_unclip_ratio`: Controls tightness of detected boxes (1.3 default)
- `det_db_thresh`: Per-pixel confidence threshold (0.5 default)
- `min_height_ratio`: Aspect ratio filtering (2.5 default)

**Systems**:
- **PaddleOCR** (from PaddlePaddle):
  - Complete re-implementation with custom PyTorch wrapper for batch processing optimization
  - Jersey-specific filtering based on aspect ratios and height thresholds
- **ONNX Runtime**
- **PyTorch**

**Non-trivial Adaptations**:
- **Custom Paddle2PytorchWrapper**: Complete re-implementation of PaddleOCR inference with batch processing optimization
- **Sports-Optimized Preprocessing**: Custom normalization and resizing specifically tuned for jersey number detection
- **Efficient Memory Management**: Streaming tensor operations to handle large batch sizes without memory overflow
- **Perspective Correction Pipeline**: Custom `get_rotate_crop_image()` method that converts detected quadrilaterals to rectangular crops using perspective transforms
- **Jersey-Specific Filtering**: Multi-stage filtering based on aspect ratios, height thresholds, and scaled dimensions to eliminate false positives

#### 2. DINO v2 Double Digit Recognition Model
**Architecture**: Fine-tuned DINO v2 (Vision Transformer) with dual classification heads
**Novel Approach**: Two separate classification heads for first and second digits
**Key Innovation**: Handles both single-digit (0-9) and double-digit (10-99) numbers seamlessly

**Architecture Details**:
- **Base Model**: DINO v2 Vision Transformer (pre-trained on ImageNet)
- **Input Size**: 224x224 pixel jersey number crops
- **Output**: Two separate logit vectors (one per digit position)
- **Loss Function**: Weighted Cross-Entropy for each digit position
- **Class Weights**: Separate importance weighting for first vs second digits

**Training Features**:
- **Complex Head Option**: Multi-layer classifier with GELU activation and LayerNorm
- **Linear Head Option**: Simple linear classification for faster inference
- **Mixed Token Processing**: Combines CLS token with mean-pooled patch tokens

**Systems**:
- **DINO v2** (from Meta/FAIR):
  - Complete reimplementation with dual classification heads for simultaneous first and second digit prediction
  - Custom training pipeline using synthetic jersey number datasets with digit-specific augmentations
  - **Training**: Uses thousands of synthetic samples per number class generated via inpainting
- **HuggingFace Transformers**
- **PyTorch**

**Non-trivial Adaptations**:
- **Custom DinoV2DoubleDigitModel**: Complete reimplementation of DINO v2 architecture with dual classification heads for simultaneous first and second digit prediction
- **Weighted Loss System**: Separate class weight vectors for each digit position to handle digit frequency imbalances in jersey numbers
- **Token Fusion Strategy**: Novel combination of CLS token with mean-pooled patch tokens for better feature representation
- **Jersey-Specific Training Protocol**: Custom training pipeline using synthetic jersey number datasets with digit-specific augmentations
- **Flexible Head Architecture**: Configurable head complexity (linear vs multi-layer) based on dataset size and accuracy requirements

#### 3. Synthetic Data Generation via Inpainting

**Core Innovation**: Generate massive training datasets using inpainting techniques on real game footage

**Process**:
1. **Base Image Collection**: Gather real jersey images from game footage
2. **Number Removal**: Use inpainting models to remove existing jersey numbers, creating clean jersey templates
3. **Synthetic Number Insertion**: Overlay new numbers using various fonts, colors, and orientations
4. **Augmentation**: Apply realistic transformations (perspective, blur, lighting variations)
5. **Dataset Scaling**: Generate thousands of synthetic examples per number class

**Training Dataset Characteristics**:
- **Scale**: Generates thousands of synthetic examples per number class (0-99)
- **Realistic Backgrounds**: Uses actual jersey textures and colors from games
- **Varied Fonts**: Multiple jersey number font styles and weights
- **Lighting Conditions**: Simulated stadium lighting variations
- **Viewing Angles**: Perspective transformations for different camera angles
- **Motion Effects**: Synthetic blur and distortion effects

**Systems**:
- **Advanced Inpainting Models**:
  - Custom pipeline for jersey number region identification and clean template creation
- **Font Rendering Systems**:
  - Multiple sports-authentic font libraries with realistic styling
- **Augmentation Libraries**:
  - Physics-based transformation pipelines simulating actual game conditions
- **Quality Control Networks**:
  - Automated validation ensuring realistic appearance and proper number visibility

**Non-trivial Adaptations**:
- **Game-Footage Inpainting Pipeline**: Custom pipeline that identifies jersey number regions in real game footage and uses advanced inpainting to create clean jersey templates
- **Multi-Font Synthesis System**: Automated system for overlaying synthetic numbers using multiple authentic sports fonts with realistic styling
- **Physics-Based Augmentation**: Realistic motion blur, perspective distortion, and lighting effects that simulate actual game conditions
- **Domain-Specific Background Preservation**: Maintains authentic jersey fabric textures, wrinkles, and team-specific design elements during number replacement
- **Quality Control Filtering**: Automated validation of synthetic samples to ensure realistic appearance and proper number visibility

### Statistical Fusion System

#### Multi-Frame Aggregation
**Purpose**: Combine OCR readings from multiple frames of the same player track
**Challenge**: Individual frame OCR may be noisy due to motion, lighting, or occlusion
**Solution**: Statistical analysis across track duration to determine most likely number

#### Signal-to-Noise Analysis
**Implementation**: `aggregated_jersey_number_analysis()` function
**Method**: Dual-threshold system for robust statistical validation

**Algorithm Steps**:
1. **Digit Count Analysis**: Determine if jersey number is single or double digit
2. **Per-Digit Validation**: Analyze each digit position separately
3. **Signal Strength**: Count supporting evidence for each digit hypothesis
4. **Noise Assessment**: Measure conflicting evidence
5. **SNR Calculation**: Compute signal-to-noise ratio for confidence estimation

**Thresholds**:
- **Low Threshold**: Accept fewer samples (10) if very clean (SNR ≥ 0.5)
- **High Threshold**: Require more samples (15) for moderate quality (SNR ≥ 0.9)
- **Unknown Handling**: Mark uncertain digits as "x" for partial recognition

#### Confidence Scoring
**Multi-level Validation**:
- **Frame-level**: Individual OCR confidence per detection
- **Track-level**: Statistical confidence across multiple frames
- **Decision-level**: Final confidence in aggregated result

### Advanced OCR Features

#### Perspective Correction
**Method**: `get_rotate_crop_image()` function
**Purpose**: Convert detected quadrilateral text regions to rectangular crops
**Implementation**:
- Calculates optimal crop dimensions from quadrilateral corners
- Applies perspective transformation matrix
- Uses bilinear interpolation for quality preservation

#### Aspect Ratio Filtering
**Width-to-Height Constraints**: Filters detected regions that are too wide or narrow
**Purpose**: Eliminate false positive detections (horizontal lines, decorations)
**Adaptive Scaling**: Considers original image resolution for scale-aware filtering

#### Torso-Focused Detection
**Pose Integration**: Uses pose estimation to focus on jersey/torso region
**Padding Strategy**: Extends torso bounding box to capture numbers near edges
**Background Filtering**: Minimizes false positives from non-jersey text (sponsor logos, etc.)

## Training and Data Pipeline

### Synthetic Dataset Generation
**Scale**: Thousands of synthetic jersey numbers per digit combination
**Quality Control**: Manual validation of synthetic data quality
**Diversity**:
- Multiple jersey colors and textures
- Various number fonts and sizes
- Different lighting and shadow conditions
- Realistic motion blur and distortion effects

### Model Training Process
**Base Model**: Start with DINO v2 pre-trained on general vision tasks
**Fine-tuning Strategy**:
- Freeze early transformer layers
- Train classification heads on synthetic jersey data
- Optional fine-tuning of late transformer layers
**Augmentation**: Additional training-time augmentation for robustness

### Evaluation and Validation
**Test Sets**: Held-out real game footage with manual number annotations
**Metrics**:
- **Character-level Accuracy**: Percentage of correctly recognized digits
- **Number-level Accuracy**: Percentage of completely correct jersey numbers
- **Confidence Calibration**: Reliability of confidence scores

## Integration with Gallery System

### Track-Level Processing
**Input**: Player tracks with associated image crops
**Processing**: Batch OCR processing across track duration
**Output**: Jersey number assignments with confidence scores

### Error Handling and Recovery
**Partial Recognition**: Handle cases where only one digit is confidently recognized
**Conflict Resolution**: Resolve cases where multiple numbers are detected for same player
**Roster Validation**: Cross-check OCR results against known team rosters when available

### Performance Optimization
**Batch Processing**: Process multiple player crops simultaneously
**GPU Utilization**: Efficient memory usage for large batch sizes
**Caching**: Cache model weights and processing pipelines for repeated use

## System Configuration

### Model Selection Options
```python
# Available OCR implementations
- "paddle_pytorch_detector": PaddleOCR-based detection + DINO recognition
- "torso_det_reco": Torso-focused detection and recognition
- "vit_classifier": Direct ViT-based number classification
- "sv_paddle_det_sv_dino_reco": SportsVisio optimized pipeline
```

### Quality Parameters
- **Detection Confidence**: Minimum confidence for text detection
- **Recognition Confidence**: Minimum confidence for digit recognition
- **Track Validation**: Minimum track length and OCR sample count
- **Statistical Thresholds**: SNR requirements for digit validation

### Performance Tuning
- **Batch Size**: Balance between memory usage and processing speed
- **Image Resolution**: Trade-off between accuracy and computational cost
- **Model Precision**: FP16 vs FP32 for speed vs accuracy balance

## Challenges and Solutions

### Sports-Specific Challenges
**Motion Blur**: Fast player movement causes blur
- *Solution*: Multi-frame fusion and motion-aware augmentation

**Perspective Distortion**: Numbers viewed from various angles
- *Solution*: Perspective correction and viewpoint augmentation

**Occlusion**: Players blocking each other's numbers
- *Solution*: Temporal aggregation across visible frames

**Lighting Variation**: Stadium lighting changes throughout games
- *Solution*: Synthetic lighting augmentation and robust preprocessing

### Technical Innovations
**Dual-Head Architecture**: Separate processing for each digit position
**Synthetic Training Data**: Inpainting-based dataset generation
**Statistical Fusion**: Robust aggregation across noisy individual predictions
**Sports-Optimized Pipeline**: End-to-end system designed for jersey number reading

The OCR system represents a specialized application of computer vision techniques to the unique challenges of reading jersey numbers in sports video, combining modern deep learning with domain-specific optimizations for robust performance.