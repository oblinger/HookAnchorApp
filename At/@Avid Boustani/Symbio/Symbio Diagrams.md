

DIAGRAMS
- Use VLM - Flamingo, Clip, LLaVA
- SAM - Segment image



  1. Geometric filtering removes 70-90% of SAM's noise immediately
  2. VLM classification is most reliable but expensive - use batching
  3. Text labels are gold when available - prioritize them
  4. Consensus from multiple sources catches errors
  5. Confidence thresholds let you tune precision vs recall
  6. Spatial coherence provides a final sanity check



  From Raw SAM Output to Semantic Regions: Five-Stage Pipeline

  Stage 1: Geometric Filtering

  Remove obvious junk based on simple measurements:
  - Too small: Masks under ~500 pixels are likely noise, shadows, or artifacts
  - Too large: Masks covering >80% of image are probably background
  - Wrong shape: Extremely thin slivers or highly irregular fragments are usually errors
  - Compactness: Very fragmented regions with high perimeter-to-area ratios are discarded

  Typical result: Reduces SAM's output from 500+ masks to 50-100 plausible regions.

  Stage 2: Hierarchical Merging

  Combine over-segmented regions that should be treated as one:
  - Nested masks: When small masks are completely inside larger ones, merge them (e.g., windows inside building footprint)
  - Adjacent similar regions: Touching masks with similar color/texture get merged (e.g., building roof + walls → complete building)
  - Spatial clustering: Groups of small nearby regions with consistent appearance become single entity

  Typical result: 50-100 regions → 15-30 meaningful features.

  Stage 3: Feature Extraction

  For each remaining region, compute properties that help with classification:
  - Geometric: area, shape, aspect ratio, orientation, location
  - Visual: average color, texture patterns, edge density
  - Contextual: position in image, distance from edges
  - Embeddings: CLIP or similar model creates semantic fingerprint

  These features feed into the classification stage.

  Stage 4: Multi-Source Semantic Classification

  Assign labels using three complementary methods, then combine:

  Method A - VLM Classification: Send cropped regions to vision-language model asking "what is this?" Most accurate but slowest/most expensive.

  Method B - Text Association: If OCR found labels, match text to nearby regions. Text connected by leader lines gets highest confidence, embedded text
  next, proximity-based last.

  Method C - Visual Heuristics: Simple rules from appearance (blue→water, green→vegetation, gray linear→road, rectangular→building).

  Consensus: Combine all three sources. When they agree, boost confidence. When they disagree, flag for human review and use highest-confidence
  prediction.

  Typical result: 70-90% of regions get confident labels, 10-30% need review.

  Stage 5: Spatial Coherence & Quality Control

  Final validation using domain knowledge:
  - Apply confidence threshold (e.g., only keep regions with >60% confidence)
  - Check spatial relationships make sense (wetlands near water bodies = good, buildings inside wetlands = suspicious)
  - Boost confidence when neighboring features support each other
  - Flag contradictions for review
  - Discard remaining low-confidence regions

  Final output: Clean set of 10-25 semantically labeled regions with confidence scores, ready for report generation.

  Why This Works

  - Geometric filtering is fast and removes 80-90% of noise immediately
  - Merging fixes SAM's tendency to over-segment
  - Multi-source classification catches errors any single method would miss
  - VLM provides semantic understanding that pure CV cannot
  - Text labels are most reliable when present (explicit human annotation)
  - Spatial coherence provides domain-specific validation

  The pipeline balances automation (VLM + heuristics) with reliability (text grounding + consensus) to transform noisy low-level masks into high-level
  semantic understanding.