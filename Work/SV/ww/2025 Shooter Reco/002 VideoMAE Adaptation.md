# 002 VideoMAE Adaptation for Basketball Shooter Identification

## Overview

This document outlines the adaptation of VideoMAE for identifying which player took a shot in basketball footage. The core idea is to maintain VideoMAE's powerful pixel-level understanding while adding basketball-specific components for tracking, shot awareness, and multi-player reasoning.

## Base Architecture: VideoMAE

VideoMAE processes video by:
1. Dividing video into spatiotemporal patches (16×16x3 pixel cubes across time)
2. Randomly masking patches during training for self-supervised learning
3. Using a Vision Transformer to encode visible patches
4. Reconstructing masked patches to learn video representations

Our adaptation keeps this pixel-level processing but adds domain-specific modifications.

## Key Modifications

### 1. Track-Aware Tokenization

**Problem**: Standard VideoMAE treats all video patches equally, but we have additional information about where players are located.

**Solution**: Augment the patch tokens with track tokens that explicitly represent player positions and identities.

**Implementation approach**:
- Create standard video patch tokens as usual (unchanged from VideoMAE)
- For each player detection in each frame, create a "track token" encoding:
  - Bounding box coordinates (x, y, width, height)
  - Player ID if known (O1-O5 for offense, D1-D5 for defense)
  - Detection confidence score
- Process tokens in separate streams (no concatenation needed)
- Video tokens go to VideoMAE, track tokens go to track transformer

**Benefit**: The transformer can now explicitly reason about player positions while still processing raw pixels for detailed visual understanding.

### 2. Hierarchical Attention Masking

**Problem**: Not all regions of the video are equally important - the shot origin and nearby players matter most.

**Solution**: Create attention masks that guide the model to focus on relevant regions.

**Three-level attention hierarchy**:
- **Level 1 (Highest)**: Region immediately around shot origin (±50 pixels)
  - Critical for seeing hand position, ball release, shooting form
  - Full attention weight (1.0)
  
- **Level 2 (Medium)**: All player bounding boxes
  - Important for understanding player actions and interactions
  - Medium attention weight (0.5)
  
- **Level 3 (Low)**: Background and court areas
  - Provides context but less critical
  - Low attention weight (0.1)

**Dynamic masking**: Attention weights change based on temporal distance from shot moment:
- At shot moment: Maximum focus on shot origin
- Before/after shot: More distributed attention to track player movements

### 3. Multi-Scale Temporal Processing

**Problem**: Different temporal resolutions are needed for different aspects - fine-grained for shooting motion, coarse for player positioning.

**Solution**: Process video at multiple temporal scales simultaneously.

**Temporal scales**:
- **Fine scale** (30 FPS, ±0.5 seconds around shot):
  - Captures detailed shooting mechanics
  - Hand position, ball release, follow-through
  - Critical for distinguishing shooter from nearby players
  
- **Medium scale** (10 FPS, ±2 seconds):
  - Player movements and positioning
  - Who's approaching the shot location
  - Defensive reactions
  
- **Coarse scale** (2 FPS, ±5 seconds):
  - Overall play development
  - Player roles (who's setting screens, cutting)
  - Entry/exit from frame

**Fusion strategy**: 
- Process each scale with separate transformer layers
- Concatenate multi-scale features
- Let cross-attention learn which scale is relevant for which decision

### 4. Shot-Context Encoding

**Problem**: The model needs to know where and when the shot occurs to properly interpret the scene.

**Solution**: Encode shot information as explicit context that modulates all processing.

**Shot context components**:
- **Shot location token**: Special learnable token representing the (x, y) coordinate of shot origin
- **Temporal distance encoding**: For each frame, encode its temporal distance from shot moment
- **Spatial distance encoding**: For each player token, encode distance from shot origin

**Integration method**:
- Add shot location token as first token in sequence (like [CLS] in BERT)
- Modulate all player tokens with their distance to shot
- Use relative positional encodings based on distance from shot moment

**Benefits**:
- Model always knows where to look for shooting action
- Can learn patterns like "players far from shot origin can't be shooter"
- Temporal distance helps identify pre-shot vs post-shot movements

### 5. Player Interaction Graph Module

**Problem**: Basketball involves complex multi-player interactions (screens, cuts, defensive help) that affect who can be the shooter.

**Solution**: Add a graph neural network layer that models player relationships.

**Graph construction**:
- **Nodes**: Each player (O1-O5, D1-D5) represented by their track features
- **Edges**: Connections between all players, weighted by:
  - Spatial distance
  - Relative motion (moving toward/away from each other)
  - Team affiliation (offense/defense)

**Graph operations**:
- **Message passing**: Players exchange information about their states
- **Attention mechanism**: Learn which player relationships matter
  - High attention: Defender on shooter
  - High attention: Screener creating space
  - Low attention: Players far from action

**Learned patterns** (without explicit labels):
- Screening actions (one player blocking defender)
- Help defense (defender leaving their mark)
- Pick-and-roll coordination
- Spacing to create shooting opportunity

### 6. Compositional Output Head

**Problem**: Need to identify which specific player (O1-O5) is the shooter, not just detect shooting action.

**Solution**: Per-player classification with compositional reasoning.

**Output structure**:
- Extract features for each offensive player (O1-O5)
- Apply classifier to each player's features independently
- Output: 5 scores indicating likelihood each player is the shooter

**Compositional reasoning**:
- Combine multiple evidence sources:
  - Visual features from player's patch tokens
  - Motion pattern from track tokens
  - Interaction features from graph module
  - Proximity to shot origin
- Learn compositions like: "near shot" + "shooting motion" + "undefended" = likely shooter

**Training strategy**:
- Supervised learning with known shooter labels
- Auxiliary losses:
  - Predict whether ANY shot occurred (helps learn shooting motion)
  - Predict shot success (helps focus on release point)
  - Predict defensive pressure (helps understand interactions)

## Integration Architecture: Dual-Stream with Cross-Attention Fusion

### Architecture Overview

To preserve the pretrained VideoMAE capabilities while adding basketball-specific features, we use a dual-stream architecture with cross-attention fusion. This approach keeps the pretrained video processing intact while adding parallel processing for tracking and shot context.

### Stream Separation

**Stream 1: Video Processing (Pretrained VideoMAE)**
- Input: Only standard video patch tokens [B, T, H/16, W/16, 768]
  - Each patch: 16×16×3 = 768 dimensions (flattened RGB pixels)
  - Example: 224×224 video = 14×14 = 196 patches per frame
  - 16 frames × 196 patches = 3,136 video tokens total
- Processing: Unmodified pretrained VideoMAE encoder
- Output: Video feature representations [B, 3136, 768]
- Initially frozen to preserve pretrained knowledge

**Stream 2: Track and Context Processing (New Components)**
- Input: Track tokens + shot context tokens
  - Track tokens: [B, T, N_players, 768] where N_players ≤ 10
  - Each track token: bbox(4) + player_id(64) + confidence(1) → projected to 768 dims
  - Example: 16 frames × 10 players = 160 track tokens total
  - Shot context: 1 token of 768 dimensions
- Processing: New transformer encoder trained from scratch
- Output: Track-aware spatial-temporal features [B, 161, 768]
- Trainable from the start

### Detailed Architecture Flow

1. **Parallel Input Processing**:
   ```
   Video frames → [Patchify] → Video patches → [Pretrained VideoMAE] → Video features
   
   Player tracks → [Track Encoder] → Track tokens → [Track Transformer] → Track features
   
   Shot location → [Shot Encoder] → Shot context → ↘
                                                      [Fusion Layer]
   ```

2. **Cross-Attention Fusion**:
   - Track features serve as queries
   - Video features serve as keys and values
   - This allows tracks to "look at" relevant parts of the video
   - Shot context modulates the attention patterns

3. **Why This Architecture Works**:
   - **Preserves pretrained knowledge**: VideoMAE processes exactly what it was trained on
   - **Clean separation**: Special tokens never corrupt video processing
   - **Flexible fusion**: Cross-attention learns optimal combination
   - **Interpretable**: Can visualize what each stream contributes

### Token Compatibility Strategy

Since pretrained VideoMAE expects only video patches, we handle special tokens separately:

**Track Token Encoding**:
- Encode bounding boxes: (x, y, w, h) → 256-dim vector
- Encode player IDs: O1-O5 → learnable embeddings
- Project to VideoMAE dimension: 256 → 768
- Add positional encoding for frame location

**Shot Context Encoding**:
- Spatial location: (x, y) → 128-dim vector
- Temporal offset: frame_distance → sinusoidal encoding
- Combine into unified shot token

**Virtual Position Strategy**:
- Assign "virtual" spatial positions to special tokens
- Video patches: positions (0,0) to (13,13) for standard 14×14 grid
- Track tokens: virtual positions (14,0) to (14,9) for up to 10 players
- Shot token: virtual position (15,0)
- This allows reuse of VideoMAE's positional embeddings

### Progressive Training Strategy

**Stage 1: Frozen Backbone (Epochs 1-10)**
- Freeze entire VideoMAE encoder
- Train only: Track encoder, fusion layer, output heads
- Focus: Learn to use tracking information effectively
- Benefit: Fast training, no catastrophic forgetting

**Stage 2: Partial Fine-tuning (Epochs 11-25)**
- Unfreeze last 3 VideoMAE transformer blocks
- Continue training all new components
- Reduce learning rate for VideoMAE layers (1/10 of new components)
- Focus: Adapt video features to basketball domain

**Stage 3: Full Fine-tuning (Epochs 26-40)**
- Unfreeze entire VideoMAE
- Very low learning rate for early VideoMAE layers
- Progressive learning rates: early layers (1e-5) → late layers (1e-4) → new components (1e-3)
- Focus: End-to-end optimization

**Stage 4: Track Dropout Training (Epochs 41-50)**
- Randomly drop track information 20% of the time
- Forces model to work from video alone as fallback
- Improves robustness to tracking failures

### Fusion Layer Details

**Cross-Attention Mechanism**:
```
Query: Track features [B, N_players, 768]
Key: Video features [B, N_patches, 768]
Value: Video features [B, N_patches, 768]

Attention(Q,K,V) = softmax(QK^T/√d)V
```

**Multi-Head Design**:
- 12 attention heads
- Each head dimension: 64
- Different heads learn different relationships:
  - Head 1-3: Spatial alignment (track location ↔ video patches)
  - Head 4-6: Temporal alignment (track motion ↔ video motion)
  - Head 7-9: Action recognition (shooting motion patterns)
  - Head 10-12: Interaction patterns (screening, defending)

**Gated Fusion**:
After cross-attention, use gating to control information flow:
```
gate = sigmoid(Linear([track_features; video_features]))
fused = gate * attended_video + (1-gate) * track_features
```

This allows the model to dynamically weight video vs tracking information.

### Advantages of This Approach

1. **Maximum preservation**: Pretrained VideoMAE is never corrupted by unfamiliar tokens
2. **Graceful degradation**: Can fall back to video-only if tracking fails
3. **Interpretability**: Clear separation shows what each modality contributes
4. **Training efficiency**: Can start with frozen backbone, reducing compute
5. **Flexibility**: Easy to add/remove modalities without architectural changes

## Training Considerations

### Data Augmentation
- **Temporal jittering**: Vary exact shot moment by ±5 frames
- **Spatial jittering**: Small variations in shot location annotation
- **Track noise**: Simulate detection failures by randomly dropping tracks
- **ID swapping**: Occasionally swap player IDs to force visual verification

### Loss Functions
- **Primary**: Cross-entropy on shooter identification
- **Auxiliary**: 
  - Shot occurrence (binary)
  - Player action recognition (if available)
  - Track consistency (same player across frames)

### Curriculum Learning
1. Start with clear, unoccluded shots
2. Gradually add complex cases (multiple players, occlusions)
3. Finally include ambiguous cases (passes, fakes)

## Expected Advantages

1. **Pixel-level understanding**: Maintains VideoMAE's ability to see fine details
2. **Track integration**: Naturally incorporates detection/tracking information
3. **Shot focus**: Explicitly encodes where/when shot occurs
4. **Multi-player reasoning**: Understands screening, defensive help, etc.
5. **Interpretability**: Attention weights show what model is looking at

## Implementation Notes

- Start with pretrained VideoMAE weights for video encoding
- Track encoder and interaction modules trained from scratch
- Fine-tune end-to-end on basketball data
- Can process variable number of players (0-10) through padding/masking
- Inference time: ~100ms per clip on GPU (suitable for real-time)