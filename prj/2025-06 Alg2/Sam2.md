# SAM2 Usage in algorithms2 Sports Analytics Pipeline

This document outlines how SAM2 (Segment Anything Model 2) and SAM are integrated within the algorithms2 sports analytics application.

## Overview

The algorithms2 pipeline uses both SAM and SAM2 for advanced image segmentation tasks, primarily focusing on basketball court analysis. The implementation leverages EfficientVIT-based SAM models for computational efficiency while maintaining high segmentation quality.

# SAM2 Integration
## Installation and Dependencies

### Environment Configuration
- **File**: [`environment.yml:273`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/environment.yml:273)
  - SAM2 installation: `sam-2 @ git+https://github.com/facebookresearch/sam2.git@c2ec8e1`
- **File**: [`environment.yml:278`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/environment.yml:278)  
  - Original SAM: `segment-anything @ git+https://github.com/facebookresearch/segment-anything.git@6fdee8f2727f4506cfbbe553e23b895e27956588`

## Core Architecture

### SAM Framework Implementation

#### Base Classes
- **File**: [`src/bball_analyze/common/deep_learning/segmentation/base_sam.py`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/common/deep_learning/segmentation/base_sam.py:1)
  - Abstract base class `BaseSAMSegmentator` defining the segmentation interface
  - Three core methods: `predict_on_image()`, `predict_on_points()`, `predict_on_boxes()`

#### Factory Pattern
- **File**: [`src/bball_analyze/common/deep_learning/segmentation/sam_factory.py`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/common/deep_learning/segmentation/sam_factory.py:1)
  - `SAMSegmentatorFactory` for model instantiation
  - Supports model key: `"l0_efficientvit"` (EfficientVIT-based SAM)
  - Model path: `sam/l0_efficientvit.pt`

#### EfficientVIT Implementation
- **File**: [`src/bball_analyze/common/deep_learning/segmentation/sam_efficientvit.py`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/common/deep_learning/segmentation/sam_efficientvit.py:1)
  - `SAMEfficientVitSegmentator` class implementation
  - Optimized SAM variant for better performance

### External EfficientVIT Module
- **Directory**: [`external/efficientvit_repo/`](algorithms2/external/efficientvit_repo/)
  - Complete EfficientVIT implementation with SAM support
  - **Model Zoo**: [`efficientvit/sam_model_zoo.py`](algorithms2/external/efficientvit_repo/efficientvit/sam_model_zoo.py)
  - **Core Implementation**: [`efficientvit/models/efficientvit/sam.py`](algorithms2/external/efficientvit_repo/efficientvit/models/efficientvit/sam.py)
  - **Demo**: [`demo_sam_model.py`](algorithms2/external/efficientvit_repo/demo_sam_model.py)

## Configuration

### Basketball Pipeline Configuration
- **File**: [`src/synch/configs/base_basketball.json.cfg:296`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/synch/configs/base_basketball.json.cfg:296)
  - SAM model specification: `"sam_model_key": "l0_efficientvit"`

### Model Storage
- **File**: [`src/settings.py:20-21`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/settings.py:20)
  - Models stored in: `SEGMENTATION_MODELS_PATH = os.path.join(MODELS_PATH, 'object_segmentation_models')`
  - Full path: `/assets/object_segmentation_models/sam/`


## SAM Segmentation Methods

The `SAMEfficientVitSegmentator` provides three prediction methods:

1. **`predict_on_image()`**: Automatic mask generation for entire image
2. **`predict_on_points()`**: Point-prompted segmentation  
3. **`predict_on_boxes()`**: Box-prompted segmentation *(used in active hoop detection)*

## Basketball Analytics Workflow

SAM and SAM2 are integrated into two critical basketball analytics workflows:

1. **Shooter Identification**: SAM2 provides intelligent fallback when player tracking fails to identify the shooter
2. **Active Hoop Detection**: SAM performs precise backboard segmentation for court analysis

## Instance Segmentation Framework

SAM integrates with a broader instance segmentation framework:
- **Directory**: [`src/bball_analyze/common/deep_learning/instance_segmentators/`](algorithms2/src/bball_analyze/common/deep_learning/instance_segmentators/)
- Base classes for PyTorch-based segmentation models
- Standardized segmentation interface across different model types

## Summary

SAM2 and the original SAM are deeply integrated into the algorithms2 sports analytics pipeline, serving two primary basketball analysis functions:

1. **Shooter Identification**: SAM2 acts as an intelligent fallback mechanism when traditional player tracking methods cannot reliably determine who took a shot. The system recognizes tracking failures and automatically switches to advanced segmentation analysis.

2. **Hoop Detection**: SAM performs precise backboard segmentation to identify active hoops and support court-side classification for shot analysis.

The implementation uses the EfficientVIT variant of SAM for computational efficiency while maintaining segmentation quality. While the SAM2 shooter identification fallback is architecturally implemented, the core `shot_sam2_analysis()` method remains a TODO, indicating this advanced feature is planned but not yet fully implemented. The hoop detection SAM integration is fully functional and actively used in production workloads. 

# Use Cases
## Use Case 1: Shooter Identification

### Overview
SAM2 serves as an intelligent fallback mechanism when traditional player tracking methods cannot reliably identify the shooter during basketball shot events. The system first attempts to use tracking data, pose analysis, and ball-hand contact detection, but when these methods fail or produce ambiguous results, it switches to SAM2-based segmentation analysis.

### Implementation
- **File**: [`src/bball_analyze/shooter_identification/shooter_identification.py`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/shooter_identification/shooter_identification.py:1)

#### Fallback Trigger Logic
The system recognizes when it needs SAM2 recovery in the `identify_shooter()` method:

- **Lines 155-167**: **SAM2 fallback activation**:
  ```python
  if need_recovery:
      shooter_player_id, scenario = self.shot_sam2_analysis(
          sod_frame_nbr,
          player_tracklets_data,
          player_id_data,
          ball_tracklets_data,
          scenario,
          shot_origin_reached_player,
          defensive_player,
          players_hands_status,
      )
      shooter_track_id = None  # sam2 does not operate on tracks
  ```

#### Specific Failure Scenarios
The system sets `need_recovery = True` when tracking fails in these cases:

1. **No Offensive Players Detected** - [`shooter_identification.py:227`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/shooter_identification/shooter_identification.py:227)
   ```python
   need_recovery = True
   scenario += ".no_offensive"
   ```

2. **Wrong Hand Pose Detection** - [`shooter_identification.py:253-257`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/shooter_identification/shooter_identification.py:253)
   - Player has hands down during shot when expected to have hands up

3. **Ambiguous Multiple Players** - [`shooter_identification.py:293-294`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/shooter_identification/shooter_identification.py:293)
   ```python
   need_recovery = True
   scenario += ".no_hands_indicators"
   ```

#### SAM2 Analysis Method
- **Lines 557-596**: The [`shot_sam2_analysis()` method framework](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/shooter_identification/shooter_identification.py:557):
  ```python
  def shot_sam2_analysis(self, ...):
      """Analyze shot using SAM2 method when primary methods fail."""
      # TODO: implement SAM2 analysis
      return None, scenario
  ```

### Algorithm Controller
- **File**: [`src/synch/scshooteridentifier.py:276-294`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/synch/scshooteridentifier.py:276)
  - Main shooter identification orchestration that triggers the SAM2 fallback logic

### Primary Detection Flow
1. **Shot Origin Detection (SOD)** - Identify candidates near ball at shot origin frame
2. **Team Filtering** - Select offensive team players only
3. **Ball-Hand Contact Analysis** - Check if player's hands contact the ball
4. **Hand Pose Analysis** - Verify hands-up vs hands-down pose
5. **SAM2 Fallback** - Advanced segmentation when above methods fail

## Use Case 2: Hoop Detection

### Overview
SAM is used for precise backboard segmentation in the active hoop detection algorithm, helping to identify which basketball hoop is currently active for shot analysis and court side classification.

### Implementation
- **File**: [`src/bball_analyze/active_objects/active_hoop_based_on_lines.py`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/active_objects/active_hoop_based_on_lines.py:1)

#### Key Integration Points:
- **Lines 10-12**: [Import `BaseSAMSegmentator`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/active_objects/active_hoop_based_on_lines.py:10)
- **Line 160**: [Constructor parameter `sam_segmentator: BaseSAMSegmentator`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/active_objects/active_hoop_based_on_lines.py:160)
- **Line 180**: [SAM segmentator stored as instance variable](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/active_objects/active_hoop_based_on_lines.py:180)
- **Lines 575-577**: [**Core SAM usage for backboard segmentation**](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/bball_analyze/active_objects/active_hoop_based_on_lines.py:575):
  ```python
  masks = self._sam_segmentator.predict_on_boxes(img=img, boxes=[box])
  mask = masks[0]  # only one bbox as input for SAM.
  ```

### Algorithm Controller
- **File**: [`src/synch/scactivehoop.py`](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/synch/scactivehoop.py:1)
- **Line 28**: [SAM model key configuration](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/synch/scactivehoop.py:28)
- **Lines 264-267**: [**SAM segmentator factory instantiation**](cursor://file//Users/oblinger/ob/kmr/prj/2025-06%20Alg2/algorithms2/src/synch/scactivehoop.py:264):
  ```python
  sam_segmentator = SAMSegmentatorFactory().get(
      key=config.sam_model_key,
      extra_params={'use_gpu': config.use_gpu}
  )
  ```

### Hoop Analysis Workflow
- **Backboard Analysis**: Using box-prompted segmentation to extract precise backboard masks
- **Court Side Classification**: Determining which side of the court a backboard belongs to
- **Shot Analysis**: Supporting shot detection and basketball tracking by identifying active court elements
