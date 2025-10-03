

  Main decisions to discuss:

  1. Temporal alignment - Fixed (2/3 point), variable, or hybrid approach for training data
  2. Ball tracking reliability - How to handle noisy tracking when identifying shot timing
  3. Spatial handling - Pre-center all frames vs. provide metadata to the model
  4. Clip duration - Optimal length for context vs. precision

  The hybrid approach I suggested would:
  - Use fixed alignment (shot at 2/3) for base dataset stability
  - Generate augmented samples with random temporal crops for generalization
  - Train model to detect shots at any point during inference


# Processing Tasks

## Task: Classification Task

### Problem Definition
Ternary classification of basketball video clips into three classes:
1. **Make** - successful shot
2. **Miss** - unsuccessful shot attempt
3. **No shot** - no shot attempt in clip

### Input
- Video clip (a few seconds duration)
- Backboard-centered frames (may be padded)
- Optional: Metadata about backboard location

### Output
- Classification prediction: make, miss, or no shot
- Confidence scores (optional)

### Model Requirements
4. **Training Phase**
   - Learn from preprocessed dataset with known temporal alignment
   - Handle clips where shot occurs at various points in the sequence

5. **Inference Phase**
   - Given arbitrary video interval, detect if shot occurred at ANY point
   - Classify shot outcome if shot is detected
   - Handle varying backboard positions (either via centering or metadata)

### Design Questions to Resolve
6. **Training Data Temporal Variance**
   - Should training clips have shots at fixed position (2/3 through)?
   - Or should we generate multiple clips per shot at different temporal positions?
   - This affects model's ability to recognize shots at any point during inference

7. **Spatial Alignment**
   - **Option A**: Always center backboard in preprocessing (simpler)
   - **Option B**: Provide backboard metadata, let model learn spatial invariance
   - **Option C**: Use both - centered frames + metadata as additional input

8. **Temporal Window**
   - What clip duration optimizes the trade-off between:
     - Context for accurate classification
     - Temporal precision for shot detection
     - Computational efficiency

---

## Task: Data Pre-Processing Task

### Input
- Basketball game video footage
- Metadata about shots including:
  - Shot release time (with some error margin)
  - Shot outcome (make/miss)
  - Backboard/hoop location information

### Output
- Training dataset consisting of:
  - Master clips (4-6 seconds each) containing full shot sequence
    - 1 sec before shooter release → 2 sec after closest approach
  - Backboard-centered and/or padded frames
  - Ground truth labels: **make**, **miss**, or **no shot**
  - Associated metadata (release frame, closest approach frame, etc.)

### Processing Steps
9. **Shot Event Detection**
   - Use shot release time from metadata as starting point
   - Apply ball tracking (note: somewhat noisy) to refine actual shot timing
   - Use existing make/miss detector as supplementary validation

10. **Video Clip Extraction**
   - Extract video segments around shot events
   - Extract negative samples (no shot) from game footage

11. **Frame Alignment & Preprocessing**
   - Center backboard in frame
   - Apply padding as needed to maintain consistent framing
   - Ensure temporal alignment of shot events within clips

### Design Questions to Resolve
12. **Temporal Alignment Strategy**
   - **Option A**: Fixed alignment - point of closest approach at 2/3 through clip for all training data
   - **Option B**: Variable alignment - shots occur at random points within clips
   - **Hybrid approach**: Generate fixed-alignment base dataset, then create augmented samples with random temporal offsets

13. **Ball Tracking Usage**
   - How to handle noisy ball tracking data?
   - Should we use it for validation only or as primary timing source?
   - Can we combine tracking with metadata to improve accuracy?

14. **Dataset Balance**
   - What ratio of make/miss/no-shot samples?
   - How to select no-shot intervals effectively?

---

## Video Clip Terminology

### Master Clip (Dataset Storage)
The complete video segment stored in the dataset for each shot event.

**Temporal Boundaries:**
- **Start**: 1 second BEFORE shooter release
  - Captures wind-up and shooting motion
  - Provides pre-shot context (body mechanics, shooting form)
- **Reference Point**: Shooter releases ball (t=0)
- **Flight Phase**: Ball travels to hoop (~1-2 seconds, varies per shot)
- **Closest Approach**: Ball reaches the hoop
- **End**: 2 seconds AFTER point of closest approach
  - Ensures outcome is clearly visible
  - Captures ball's trajectory after hoop interaction

**Total Duration**: ~4-6 seconds (varies based on shot arc flight time)

**Purpose**:
- One master clip stored per shot event
- Storage efficient
- Source for generating training windows

### Training Window (Training Time Sampling)
Sub-window randomly sampled from master clip during training.

**Characteristics:**
- Duration: [TBD - likely 2-3 seconds]
- Sampled randomly from master clip on-the-fly during each training epoch
- Constraint: Shot event (release through outcome) must appear in the middle portion of the window
  - Not in first/last 20% to avoid edge artifacts
  - Prevents model from learning position-based classification

**Purpose**:
- Infinite training variety from finite storage
- Teaches model to recognize shots at any temporal position
- Natural data augmentation

### Inference Window (Production Use)
Arbitrary video segment provided at inference time.

**Characteristics:**
- Duration: Variable/arbitrary
- Shot may occur at ANY position within the window (if present at all)
- No guaranteed temporal alignment

**Model Requirement**:
- Must detect if a shot occurs anywhere within the window
- Must classify the shot outcome (make/miss) if detected
- Must correctly identify "no shot" clips

---

## Training Data Format Considerations

### Key Design Decision: Temporal Alignment

**Adopted Strategy: Master Clip + Random Training Window Sampling**

The approach combines storage efficiency with training robustness:

1. **Dataset Storage**: Store one master clip per shot (4-6 seconds)
   - Contains full shot context from 1 sec before release to 2 sec after closest approach
   - Storage efficient - no redundant clips

2. **Training Time**: Randomly sample shorter training windows from each master clip
   - Each training epoch sees different temporal positions
   - Shot event appears at varying positions within training windows
   - Natural temporal augmentation without storing multiple clips

3. **Benefits**:
   - **Storage efficient**: One clip per shot, not dozens of variations
   - **Training variety**: Infinite temporal variations from finite data
   - **Generalization**: Model learns position-invariant shot recognition
   - **Prevents overfitting**: Shot not always at same temporal location

### Data Format Specification (Proposed)
```
training_sample/
├── master_clip.mp4         # Full master clip (4-6 seconds)
├── frames/                 # Extracted frames (optional)
├── metadata.json           # Contains:
│   ├── label              # make/miss/no_shot
│   ├── shooter_release_frame  # Frame index where shooter releases ball
│   ├── closest_approach_frame # Frame index of ball closest to hoop
│   ├── backboard_location # Bounding box or center point
│   ├── clip_start_offset  # Time offset from shooter release to clip start (-1 sec)
│   ├── clip_duration      # Total master clip duration in seconds/frames
│   ├── source_game_id     # Traceability
│   └── original_timestamp # Original game timestamp
```

**Note**: Training windows are NOT stored—they are sampled on-the-fly during training from the master clip.

---

## Open Questions for Discussion

18. **Temporal Alignment**: Fixed at 2/3, variable, or hybrid approach?

19. **Ball Tracking**: How much should we rely on noisy ball tracking vs. metadata timing?

20. **Point of Closest Approach**: Should we invest effort in precisely identifying this, or use approximate shot timing?

21. **Model Architecture Implications**: Does the temporal alignment strategy suggest specific model architectures (e.g., 3D CNN, temporal attention)?

22. **Evaluation Metrics**: Beyond accuracy, what metrics matter most (temporal precision, confidence calibration)?

23. **Dataset Size**: How many samples per class do we need for effective training?
