



- [[./ALGS.md]] 
- [GCP](hook://GCP) 
- [G59](https://drive.google.com/drive/folders/11P71i8hIwc-UH_Mt3-g20-gGbsHdwuSC) 
- [[Reco Approaches]] 

Plan:
- Gather data from 10, 100, 1000, 10K games.
- For each shot gather:
	- ShooterID (from shot record)
	- ShooterBoundingBox 
- Filter 
	- no shooter-id, no bounding
- Positive examples
	- Clip shooter cube (positive examples)
	- pad region around shooter bounding box
	- pad +/- 2 seconds
- Clip non-shooters detects that are stationary
- 






  Recommended approach:
  1. Start with VideoMAE pre-trained weights (already trained on Kinetics-400/600 or Something-Something)
  2. Create balanced dataset: 100K shots + 100K non-shots
  3. Fine-tune with binary classification head (shot vs non-shot)

  Why not unsupervised pre-training:
  - VideoMAE's existing weights already understand general motion patterns
  - Basketball shots are well-defined actions that existing pre-training captures
  - Your 10K games are domain-specific but not diverse enough to improve on Kinetics pre-training
  - You have 100K labeled examples - that's massive for supervised learning

  Training strategy:
  VideoMAE (pre-trained) → Freeze early layers → Add classification head → Fine-tune on 200K clips

  Key parameters:
  - Start with low learning rate (1e-4 or 1e-5)
  - Freeze backbone initially, train classifier head
  - Then unfreeze last few transformer blocks
  - Use temporal window of 16-32 frames (0.5-1 second at 30fps)
  - Center clips on shot release moment

  Data augmentation:
  - Temporal jittering (±5 frames)
  - Random crops (but keep player in frame)
  - Horizontal flips
  - Speed variations (0.9x-1.1x)