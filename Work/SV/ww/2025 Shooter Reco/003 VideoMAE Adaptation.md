# 003 VideoMAE Adaptation - Dual-Stream Architecture with Spatial Alignment

## Overview

This document describes a dual-stream architecture for basketball shooter identification that preserves pretrained VideoMAE capabilities while incorporating player tracking information through spatial alignment and cross-attention fusion.

## Architecture Components

### Stream 1: Video Processing (Pretrained VideoMAE)

**Purpose**: Extract visual features from raw video using pretrained VideoMAE without modification.

**Input**: 
- Video frames: [B, T, H, W, 3]
  - B: Batch size
  - T: Number of frames (typically 16)
  - H, W: Height and width (typically 224×224)
  - 3: RGB channels

**Processing**:
- Patchification: Divide each frame into 16×16 patches
- Result: [B, T, 14, 14, 16×16×3] where 768 = 16×16×3 flattened
- Reshape for transformer: [B, T×14×14, 768] = [B, 3136, 768]

**Output**:
- Video feature tokens: [B, 3136, 768]
- Each token retains spatial position information from original patch grid

### Stream 2: Track Processing (New Spatial Track Encoder)

**Purpose**: Encode player tracking information in spatially-aligned representation.

**Input**:
- Track heatmaps: [B, T, 14, 14, 18]
  - B: Batch size
  - T: Number of frames (matches video)
  - 14×14: Spatial grid matching video patches
  - 18: Number of information channels (each channel is float32, effectively continuous)

**Channel composition and representation**:
- **Channels 0-4**: Offensive player presence masks (O1-O5)
  - Binary values: 1.0 if player overlaps patch, 0.0 otherwise
  - Float32 representation (could be stored as 1 bit each, but kept as float for tensor operations)
  
- **Channels 5-9**: Defensive player presence masks (D1-D5)
  - Binary values: 1.0 if player overlaps patch, 0.0 otherwise
  - Float32 representation
  
- **Channel 10**: Unknown offensive player presence
  - Binary values: 1.0 if unidentified offensive player overlaps patch, 0.0 otherwise
  - Float32 representation
  
- **Channel 11**: Unknown defensive player presence
  - Binary values: 1.0 if unidentified defensive player overlaps patch, 0.0 otherwise
  - Float32 representation
  
- **Channel 12**: Non-player presence (referees, bench players, crowd)
  - Binary values: 1.0 if non-player person overlaps patch, 0.0 otherwise
  - Float32 representation
  
- **Channel 13**: Shot presence indicator
  - Binary: 1.0 if shot origin is in this patch, 0.0 otherwise
  - Only one patch in the entire 14×14 grid has value 1.0
  - Float32 representation (effectively 1 bit of information)
  
- **Channel 14**: Shot X position within patch
  - Continuous: 0.0 (left edge) to 1.0 (right edge) of 16-pixel patch
  - Only non-zero for the patch containing the shot
  - Float32 representation (utilizing full precision)
  
- **Channel 15**: Shot Y position within patch
  - Continuous: 0.0 (top edge) to 1.0 (bottom edge) of 16-pixel patch
  - Only non-zero for the patch containing the shot
  - Float32 representation (utilizing full precision)
  
- **Channel 16**: Distance to shot origin
  - Continuous: 0.0 (at shot origin) to 1.0 (≥128 pixels away)
  - ALL patches have values - provides global context
  - Computed as: min(euclidean_distance / 128.0, 1.0)
  - Float32 representation (smooth gradient)
  
- **Channel 17**: Detection confidence map
  - Continuous: 0.0 (no detection) to 1.0 (high confidence detection)
  - Per-patch maximum confidence of any detection in that patch
  - Float32 representation

**Processing Summary**:
- Each spatial location (14×14 grid) encodes what overlaps that 16×16 pixel patch
- Player presence uses binary masks (effectively 1 bit per player)
- Shot location uses mixed representation:
  - Binary indicator for which patch contains shot (1 bit)
  - Continuous sub-patch position for precision (full float32 precision)
  - Global distance field so all patches know proximity to shot
- Total information per patch: 18 channels × 32 bits = 576 bits
  - Though only ~17 bits of actual information (13 binary + shot position + distance + confidence)

**Output**:
- Track feature tokens: [B, 3136, 256]
- Spatially aligned with video tokens (position i,j corresponds between streams)

### Fusion Module: Cross-Attention

**Purpose**: Combine video and track features while maintaining spatial correspondence.

**Input**:
- Video features: [B, 3136, 768] from VideoMAE
- Track features: [B, 3136, 256] from Track Encoder
- Both maintaining 14×14×T spatial structure

**Processing**:
```
Query: Track features (asking "what's happening at each tracked location?")
Key: Video features (providing visual context)
Value: Video features (providing visual information)

Attended_video = CrossAttention(Q=track_features, K=video_features, V=video_features)
Fused_features = track_features + attended_video  # Residual connection preserves track info
```

**Output**:
- Fused features: [B, 3136, 256] (track dimension maintained)
  - Contains both: original track information + relevant video information
  - Residual connection ensures track features aren't lost
- Attention weights: [B, Heads, 3136, 3136] (for visualization)

### Output Head: Shooter Classifier

**Purpose**: Identify which offensive player (O1-O5) is the shooter.

**Input**:
- Fused features: [B, 3136, 256] (from fusion module)
- Player masks: [B, 14, 14, 5] (from channels 0-4 of track heatmaps)

**Processing**:
1. Extract features for each offensive player using their spatial masks
2. Global pooling per player: average features from their occupied patches
3. Apply MLP classifier to each player's pooled features

**Output**:
- Shooter scores: [B, 5] 
- Each score represents likelihood that O1, O2, O3, O4, or O5 is the shooter
- Apply softmax for final prediction

## Complete Information Flow


```
Video Input ────→ Patchify ────→ VideoMAE ────→ Video Features ────→
[B,T=16,224,224,3]  [B,T×14×14,16×16×3]   (Pretrained)   [B,T×14×14,768]

Track Heatmaps ──────→ Track Encoder ──────→ Track Features ──────→
[B,T=16,14,14,18]        (CNN/ViT)             [B,T×14×14,256]

Video Features + Track Features ────→ Cross-Attention ────→ Fused Features ────→ Player Pooling ────→ Classifier ────→ Output
                                      (Q=Track, KV=Video)   [B,T×14×14,256]        (Per O1-O5)          (MLP)           [B,5]
```

## Training Phases

### Phase 1: Frozen VideoMAE Training (Epochs 1-15)

**Objective**: Learn to use tracking information without corrupting pretrained video features.

**Configuration**:
- VideoMAE: Completely frozen (no gradient updates)
- Track Encoder: Trainable (random initialization)
- Cross-Attention: Trainable (random initialization)
- Shooter Classifier: Trainable (random initialization)

**Learning rate**:
- VideoMAE: 0 (frozen)
- New components: 1e-3

**What the model learns**:
- Track encoder learns to extract meaningful spatial features from tracking heatmaps
- Cross-attention learns alignment between track positions and video patches
- Classifier learns to map fused features to shooter identity

**Training data strategy**:
- Start with clear, unoccluded examples
- Gradually introduce harder cases with multiple players near shot

### Phase 2: Partial Fine-tuning (Epochs 16-30)

**Objective**: Adapt high-level video features to basketball domain while preserving low-level feature extraction.

**Configuration**:
- VideoMAE layers 1-9: Frozen
- VideoMAE layers 10-12: Trainable (slow learning rate)
- Track Encoder: Trainable
- Cross-Attention: Trainable
- Shooter Classifier: Trainable

**Learning rate**:
- VideoMAE layers 1-9: 0 (frozen)
- VideoMAE layers 10-12: 1e-5 (very slow adaptation)
- Track Encoder: 5e-4 (reduced from phase 1)
- Cross-Attention & Classifier: 5e-4

**What the model learns**:
- High-level video features adapt to basketball-specific patterns
- Better alignment between video and track representations
- Refined shooter identification patterns

### Phase 3: Full Fine-tuning (Epochs 31-45)

**Objective**: End-to-end optimization for maximum performance.

**Configuration**:
- All components trainable with differential learning rates

**Learning rate schedule**:
- VideoMAE layers 1-3: 1e-6 (extremely slow)
- VideoMAE layers 4-6: 5e-6 (very slow)
- VideoMAE layers 7-9: 1e-5 (slow)
- VideoMAE layers 10-12: 5e-5 (moderate)
- Track Encoder: 1e-4
- Cross-Attention & Classifier: 1e-4

**What the model learns**:
- Fine-grained adaptation of all features to the task
- Optimal balance between preserving pretrained knowledge and task-specific adaptation

### Phase 4: Robustness Training (Epochs 46-50)

**Objective**: Improve model robustness to tracking failures and edge cases.

**Configuration**:
- All components trainable (same as Phase 3)
- Add data augmentation and dropout

**Special training strategies**:
- Randomly drop tracking information 20% of the time (force video-only prediction)
- Randomly swap player IDs 5% of the time (force visual verification)
- Add noise to track positions (simulate detection errors)
- Include hard negatives (non-shooters near shot location)

**Learning rate**:
- All components: 5e-5 (reduced for stability)
- Cosine annealing to 0 by epoch 50

## Key Design Decisions

### Why Spatial Alignment?

Instead of abstract bounding box coordinates, we use spatially-aligned heatmaps:
- Immediate correspondence between video patches and track positions
- No need to learn coordinate-to-spatial mapping
- Natural attention alignment between streams

### Why Separate Streams?

Keeping VideoMAE and track processing separate:
- Preserves pretrained weights exactly
- Allows different processing depths (12 layers for video, 6 for tracks)
- Clean fallback to video-only if tracking fails
- Easier to debug and interpret

### Why Cross-Attention Fusion?

Using tracks as queries into video:
- Natural interpretation: "What's happening at tracked locations?"
- Efficient: Only attend to relevant video regions
- Produces interpretable attention maps
- Proven approach in multi-modal learning

## Expected Performance Characteristics

### Inference Speed
- VideoMAE forward pass: ~50ms
- Track encoder forward pass: ~20ms
- Cross-attention fusion: ~10ms
- Total: ~80ms per clip (suitable for real-time)

### Memory Requirements
- VideoMAE parameters: ~86M (ViT-Base)
- Track encoder: ~10M
- Cross-attention: ~5M
- Total: ~101M parameters

### Accuracy Expectations
- Phase 1 completion: ~80% shooter identification
- Phase 2 completion: ~87% shooter identification
- Phase 3 completion: ~92% shooter identification
- Phase 4 completion: ~94% shooter identification (robust to tracking errors)

## Dataset Staging Strategy

With 10,000 games containing ~100 shots each (1M total shots), we use a **game-diverse, shot-complete** approach rather than sampling shots within games. This preserves context consistency while maximizing environmental diversity.

### Why Complete Games Over Shot Sampling

**Advantages of using all shots per game**:
- Context consistency: Same lighting, camera angle, player uniforms, court
- Temporal relationships: Pre/post-shot player positioning patterns
- Team-specific patterns: Plays, defensive schemes, player interactions
- Pipeline simplicity: No need to curate "representative" shots within games

**Game diversity provides more value than shot density** because different games offer:
- Different arenas (lighting conditions, camera positions)
- Different teams (play styles, player types, strategic approaches)  
- Different seasons (rule changes, strategic evolution)
- Different game situations (defensive intensity varies by score differential)

### Five-Stage Scaling Plan

#### Stage 1: Proof of Concept (5 games, ~500 shots)
**Objective**: Verify architecture works and debug training pipeline

**Configuration**:
- 5 carefully selected games with cleanest tracking data
- Focus on architectural debugging and basic convergence
- Single GPU training: 2-3 days expected

**Success Criteria**:
- >70% accuracy (random baseline = 20%)
- Stable training convergence
- No major data loading or memory issues

**Key Learnings**: Architecture viability, data preprocessing correctness

#### Stage 2: Initial Scaling (25 games, ~2,500 shots)
**Objective**: Test generalization across different contexts

**Configuration**:
- Diverse selection: 5 teams × 5 different opponents
- Multiple arenas to test environmental robustness
- Single GPU training: 1 week expected

**Success Criteria**:
- >80% accuracy with cross-game validation
- No severe overfitting to specific games/teams
- Consistent performance across different arenas

**Key Learnings**: Hyperparameter sensitivity, generalization capability

#### Stage 3: Medium Scale (100 games, ~10,000 shots)
**Objective**: Establish robust baseline and optimize training dynamics

**Configuration**:
- Representative sample across teams/seasons
- Multi-GPU training: 2-3 weeks expected
- Full training phase progression (frozen → partial → full fine-tuning)

**Success Criteria**:
- >85% accuracy, stable across held-out test games
- Learning curves show healthy convergence in all phases
- Attention visualizations show sensible focus patterns

**Key Learnings**: Learning rate scheduling, data augmentation effects, phase transition timing

#### Stage 4: Large Scale (500 games, ~50,000 shots)
**Objective**: High-performance model for production evaluation

**Configuration**:
- Stratified sampling across multiple seasons and teams
- Multi-GPU distributed training: 1-2 months expected
- Include robustness training (Phase 4) with tracking dropout

**Success Criteria**:
- >90% accuracy on diverse test set
- Robust to tracking failures and edge cases
- Performance stable across different game situations

**Key Learnings**: Final performance ceiling, production readiness

#### Stage 5: Full Scale (2,000+ games, 200K+ shots)
**Objective**: Production-ready model with maximum performance

**Configuration**:
- Only proceed if Stage 4 shows continued improvement trajectory
- Large-scale distributed training: 2+ months expected
- Full dataset diversity including challenging edge cases

**Success Criteria**:
- >93% accuracy on comprehensive test set
- Consistent performance across all team matchups and arenas
- Ready for production deployment

### Game Selection Strategy

**Stratified sampling ensures diversity across**:

**Teams**: Ensure multiple playing styles represented
- Fast-paced teams (Warriors, Suns) vs slow-paced (Grizzlies, Heat)
- Star-driven offenses vs ball-movement systems
- Different defensive schemes (switch-heavy vs traditional)

**Arenas**: Different camera setups and environmental conditions
- Newer arenas with advanced camera systems
- Older arenas with varying lighting quality
- Different camera heights and angles

**Seasons**: Temporal diversity to capture strategic evolution
- Recent seasons (current playing styles)
- Historical seasons (different pace, rules)
- Mix to prevent temporal overfitting

**Game Situations**: Varying defensive intensity
- Close games (high defensive effort)
- Blowout situations (relaxed defense, bench players)
- Playoff games (maximum intensity) vs regular season

### Computational Resource Planning

**Expected training times**:
- Stage 1 (500 shots): 2-3 days on single V100/A100
- Stage 2 (2,500 shots): 1 week on single GPU
- Stage 3 (10,000 shots): 2-3 weeks on 2-4 GPUs  
- Stage 4 (50,000 shots): 1-2 months on 4-8 GPUs
- Stage 5 (200K+ shots): 2+ months on 8+ GPUs

**Memory requirements**: ~12GB per GPU for batch size 8-16

### Stage Advancement Criteria

**Advance to next stage only when**:
- Current stage accuracy has plateaued (no improvement >1% over 5 epochs)
- Cross-validation shows good generalization (test accuracy within 3% of training)
- Training pipeline is stable with no divergence issues
- Architecture and hyperparameters are optimized for current scale

**Do not advance if**:
- Heavy overfitting to training games (>10% gap to validation)
- Large accuracy drops on held-out games from different contexts
- Training instability or divergence
- Major architecture changes still needed

**Early stopping**: Stop scaling if improvement per additional data point drops below cost threshold.

This staged approach maximizes learning per compute investment while building systematically toward the full 1M shot dataset.