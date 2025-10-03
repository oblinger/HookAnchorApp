# Player Gallery System

The Player Gallery System is a sophisticated computer vision pipeline that automatically creates player-specific datasets from basketball game footage. The system identifies, tracks, and crops individual players to build a gallery of images organized by player identity, which can then be used to train player recognition models.

## High-Level Overview

The gallery system works by analyzing basketball game video to automatically extract and organize images of each player. The system takes an optional team roster as input but can also operate without it, discovering players through computer vision analysis. The end result is a folder structure where each player gets their own directory containing cropped images of that player throughout the game.

**Core Workflow**: Video Input → Object Detection → Player Tracking → Jersey Number OCR → Gallery Organization → Classifier Training

**Key Files**: `src/synch/mfplayergallery.py`, `src/bball_analyze/player_identification/player_gallery_utils.py`, `src/synch/mfplayermodeltrainer.py`

## System Architecture

### Two-Pass Processing Pipeline

The gallery system operates in two distinct passes:

**Pass 1 - Analysis Phase**:
- Processes the entire video to gather tracking, color, and jersey number data
- Identifies all unique players and their characteristics
- Resolves player identities by combining visual appearance with jersey numbers
- Creates a comprehensive player roster and crop locations

**Pass 2 - Extraction Phase**:
- Replays the video to extract actual player image crops
- Crops regions of interest based on locations determined in Pass 1
- Organizes crops into player-specific folders
- Generates metadata for training purposes

### ML Components and Usage

#### 1. Object Detection (YOLO/RT-DETR)
**Purpose**: Detect person bounding boxes in video frames
**Input**: Raw video frames
**Output**: Person detection bounding boxes with confidence scores
**Usage**: Initial detection of all people in the frame before tracking

**Systems**:
- **YOLO-NAS** (from SuperGradients):
  - Custom training on [`["basketball", "person", "hoop", "backboard"]`] with multiple model versions (v2, v3, v4)
- **RT-DETR** (from PaddleDetection):
  - Fine-tuned for basketball contexts with specialized post-processing
- **RF-DETR**:
  - Custom SportsVisio variant trained on basketball footage
- **YOLO Fastest XL** (from NCNN):
  - Lightweight deployment variant for basketball/hoop detection

**Adaptations**:
- **Sports-Specific Tuning**: Detection models fine-tuned on basketball game footage to handle sports-specific scenarios (fast movement, occlusions, court perspectives)
- **Multi-Scale Detection**: Optimized for detecting players at various distances from camera (close-up vs full-court views)
- **Court-Aware Filtering**: Integration with court detection to filter out spectators and focus on in-play regions

#### 2. Multi-Object Tracking (ByteTracker with Re-ID)
**Purpose**: Maintain consistent identity tracking across frames
**Components**:
- **ByteTracker**: Core tracking algorithm for temporal consistency
- **Re-ID features**: Visual appearance features to help maintain identity through occlusions
**Input**: Detection bounding boxes from object detector
**Output**: Consistent track IDs with temporal sequences
**Challenge**: Prevents identity switches when players cross paths or get occluded

**Systems**:
- **ByteTracker** (from original implementation):
  - **ByteTrackWithColor**: Custom extension integrating jersey color information into tracking associations
  - **PlayerIDByteTracker**: Player identification variant
  - **ReIDByteTracker**: Re-identification feature integration
- **Kalman Filters**:
  - Multiple variants (default, YOLOX, fast) optimized for different sports movement patterns
- **Re-ID Feature Extractors**

**Adaptations**:
- **ByteTrackWithColor**: Custom extension that integrates jersey color information into tracking associations
  - Adds color-aware association step before standard ByteTracker associations
  - Prevents association of tracklets with detections of different team colors
  - Maintains color history per tracklet for temporal consistency
  - Implements color-conflict resolution when multiple detections compete for same tracklet
- **Sports-Specific Kalman Filters**: Multiple Kalman filter variants optimized for different sports movement patterns
- **Re-ID Feature Integration**: Combines appearance features with motion prediction for robust identity maintenance during occlusions
- **Track Splitting Logic**: Custom collision detection and track splitting when players overlap spatially to maintain identity purity

#### 3. Pose Estimation
**Purpose**: Extract keypoints for better player cropping and pose analysis
**Input**: Player bounding boxes
**Output**: 17 COCO keypoints per person (shoulders, elbows, wrists, etc.)
**Usage**:
- Determines jersey region for more accurate cropping
- Identifies when players show their back (important for jersey number visibility)
- Helps distinguish players from referees based on pose characteristics

**Systems**:
- **COCO Pose Estimation Models**:
  - Jersey-focused keypoint processing for optimal number detection regions
  - Back-detection algorithm using shoulder keypoint spacing

**Adaptations**:
- **Jersey-Focused Keypoint Processing**: Custom logic to identify shoulder width and torso orientation for optimal jersey number detection regions
- **Back-Detection Algorithm**: Uses shoulder keypoint spacing (left_shoulder_x vs right_shoulder_x) to determine when player is facing away from camera
- **Pose-Based Quality Filtering**: Confidence thresholds on shoulder keypoints to ensure reliable pose-based decisions
- **Non-Player Detection**: Algorithm specifically uses pose patterns to identify referees and coaches (consistent back-showing, no jersey numbers)

#### 4. Jersey Color Classification
**Purpose**: Assign team colors to players
**Input**: Cropped torso regions from pose estimation
**Output**: Color classifications (e.g., "red", "blue", "white", "stripes")
**Training**: Uses color models trained on game-specific data
**Usage**: Groups players by team and distinguishes from referees ("stripes")

**Systems**:
- **Custom Color Classification Models**:
  - Game-specific training for lighting conditions and team color variations
  - Temporal aggregation pipeline across multiple frames per track

**Adaptations**:
- **Game-Specific Color Models**: Training separate color models per game to handle lighting conditions and specific team color variations
- **Colorless Mode**: Special operating mode for challenging lighting conditions where color detection is unreliable
- **Referee Pattern Recognition**: Specific "stripes" classification to automatically identify referees
- **Temporal Color Consistency**: Aggregation of color predictions across multiple frames per track to handle lighting variations and motion blur
- **Color Conflict Resolution**: Logic to handle cases where roster information conflicts with visual color detection

#### 5. Jersey Number OCR (See OCR System.md for details)
**Purpose**: Read jersey numbers from player crops
**Components**: Text detection + digit recognition pipeline
**Input**: Player torso crops focused on jersey region
**Output**: Jersey number predictions with confidence scores
**Usage**: Primary method for distinguishing individual players

**Systems**:
- **PaddleOCR** (from PaddlePaddle):
  - Custom PyTorch wrapper with batch processing optimization for sports contexts
- **DINO v2** (from HuggingFace):
  - Fine-tuned with dual classification heads for simultaneous first and second digit prediction
- **DeepText Recognition**
- **Custom Statistical Fusion Pipeline**:
  - Multi-frame aggregation with signal-to-noise analysis across track duration
  - **Statistical Thresholds**: Min 10-15 samples with SNR 0.5-0.9 for reliable digit validation

**Adaptations**:
- **Multi-Frame Statistical Fusion**: Custom `aggregated_jersey_number_analysis()` algorithm that combines OCR readings across track duration with signal-to-noise analysis
- **Dual-Threshold Validation**: Separate thresholds for high-quality short sequences vs longer moderate-quality sequences
- **Per-Digit Analysis**: Separate validation for first and second digits to handle partial occlusions
- **Sports-Specific Training Data**: OCR models trained on synthetic jersey number datasets generated using inpainting techniques on real game footage

#### 6. Player ID Classification Model (ViT-based)
**Purpose**: Final player recognition classifier
**Model**: Vision Transformer (ViT) fine-tuned on gallery data
**Input**: Player gallery images organized by identity
**Output**: Player classification model for real-time identification
**Training Process**:
- Uses gallery images as training data
- Employs data augmentation (Gaussian noise, transformations)
- Fine-tunes pre-trained ViT models (e.g., Google's ViT-Large)
- Supports mixed precision training for efficiency

**Systems**:
- **Vision Transformer (ViT)** (from HuggingFace/Google):
  - Fine-tuned on gallery datasets with configurable head architectures (linear vs complex multi-layer)
  - **Training**: Default 1 epoch, batch size 16, found sufficient for gallery-based fine-tuning
  - **Human Activity Recognition ViT**: Alternative pre-trained variant showing better sports performance than ImageNet
- **HuggingFace Transformers**
- **Custom SvViTImageClassification**:
  - SportsVisio wrapper with referee-player-non-player three-class extension

**Adaptations**:
- **Custom ViT Head Architecture**: Multiple head options (linear vs complex multi-layer with GELU and LayerNorm)
- **Referee-Player-Non-Player Extension**: Three-class classification system that includes referees and non-players as separate categories
- **Aspect Ratio Preservation**: Optional processing mode that preserves human proportions during image preprocessing
- **Gallery-Based Training Pipeline**: Fully automated training pipeline that uses gallery output directly for model training
- **Lazy Loading Optimization**: Memory-efficient image loading for large gallery datasets
- **Class Weight Balancing**: Automatic computation of class weights to handle imbalanced player representation

### Data Processing Pipeline

#### Track Preprocessing and Validation
**Collision Detection**: Identifies when tracks overlap spatially to prevent identity mixing
**Filtering Modes**:
- **Color-based**: Only considers collisions between same-color tracks
- **All-tracks**: Considers all track overlaps
- **None**: No collision filtering

**Track Splitting**: Automatically splits tracks when players get too close to maintain identity purity

#### Jersey Number Aggregation
**Signal-to-Noise Analysis**: Combines multiple OCR readings per track using statistical analysis
**Dual Threshold System**:
- Low threshold: Accepts fewer samples if very clean (high SNR)
- High threshold: Requires more samples for moderate quality
**Digit-wise Analysis**: Analyzes single-digit vs double-digit numbers separately for robustness

#### Roster Management
**Three Operating Modes**:
1. **Injected Roster**: Uses provided team roster to validate and correct identifications
2. **Discovered Roster**: Builds roster automatically from most frequently detected player IDs
3. **Mixed Mode**: Combines roster input with automatic discovery

**Player Validation**:
- Filters invalid colors (e.g., "unknown", "stripes" for regular players)
- Validates jersey numbers against known roster
- Handles jersey number conflicts and remapping

### Gallery Organization

#### Folder Structure
```
gallery/
├── red_23/          # Player-specific folders
│   ├── left100_r45_red_23.jpg
│   ├── left150_r45_red_23.jpg
│   └── ...
├── blue_10/
│   ├── right200_51_blue_10.jpg
│   └── ...
└── stripes_r/       # Referees
    └── ...
```

#### Crop Selection Criteria
**Temporal Spacing**: Skip frames to avoid redundant similar poses (default: every 2 frames)
**Collision Avoidance**: Exclude crops where players overlap significantly
**Quality Filtering**:
- Minimum track length requirements
- Confidence thresholds for color and number detection
- Pose quality validation

#### Metadata Generation
**Raw Predictions File**: `raw_predictions.json` contains detailed information about:
- Original track IDs and remapping information
- OCR readings vs final assigned numbers
- Color classifications vs final assigned colors
- Frame-by-frame confidence scores

## Advanced Features

### Non-Player Detection
**Purpose**: Automatically identify and extract referees, coaches, or other non-players
**Method**: Analyzes tracks that:
- Consistently show back of person (wide shoulder span)
- Never have readable jersey numbers
- Appear for sufficient duration
**Assignment**: Given default color "stripes" and number "r" for referees

### Collision Resolution
**Spatial Analysis**: Uses overlap masks to detect when player bounding boxes interfere
**Filtering Options**:
- Color-aware: Only splits tracks of same team color
- All-track: Considers all overlaps regardless of team
**Track Splitting**: Automatically creates new track IDs when spatial conflicts detected

### Signal Processing for OCR
**Multi-frame Fusion**: Combines OCR readings across multiple frames per track
**Statistical Validation**: Uses signal-to-noise ratios to validate digit readings
**Confidence-based Selection**: Applies different thresholds based on reading quality and quantity

### Roster Integration
**Automatic Correction**: When using injected roster, automatically corrects:
- Wrong team colors (if jersey number is unique)
- Invalid player combinations not in roster
**Flexible Validation**: Can operate in "colorless mode" for challenging lighting conditions

## Performance Optimizations

### Memory Management
**Lazy Loading**: Optional lazy image loading to reduce memory footprint during dataset creation
**Batch Processing**: Efficient batch processing for ML model inference
**Streaming Architecture**: Processes video in streaming fashion to handle large files

### GPU Acceleration
**CUDA Support**: All ML models support GPU acceleration
**Mixed Precision**: Optional FP16 inference for faster processing
**Batch Optimization**: Dynamic batching for optimal GPU utilization

### Quality vs Speed Trade-offs
**Frame Skip Options**: Configurable frame sampling rates
**Model Size Selection**: Choice of model complexity vs accuracy
**Processing Passes**: Two-pass design allows for accuracy optimization

## Integration Points

**Input Sources**:
- Live video streams or recorded files
- Optional team rosters in structured format
- Camera calibration data for multi-view setups

**Output Integration**:
- Structured player galleries for training
- Player classification models for real-time use
- Metadata for performance analysis and debugging
- Integration with evaluation and visualization pipelines

## Configuration Parameters

Key configurable parameters include:
- **Temporal**: Frame skip rates, time windows for analysis
- **Quality**: Confidence thresholds for OCR, color, and tracking
- **Spatial**: Collision detection sensitivity, crop padding
- **Training**: Model architectures, training parameters, augmentation settings
- **Validation**: Roster validation modes, error correction strategies

The gallery system represents a comprehensive solution for automated player dataset creation, combining multiple ML techniques to solve the challenging problem of player identification in sports video.