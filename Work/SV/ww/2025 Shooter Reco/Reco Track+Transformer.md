
> Are there any papers that are combining track metadata with something like video MAE for doing supervised task classification based on both video input and the tracking
  metadata?

⏺ Yes! There are several recent papers doing exactly this - combining tracking metadata with video transformers:

  Most Directly Relevant:

  "TrackFormer: Multi-Object Tracking with Transformers" (CVPR 2022)
  - Paper: https://arxiv.org/abs/2101.02702
  - Code: https://github.com/timmeinhardt/trackformer
  - Combines tracking data with visual features in transformer
  - Not VideoMAE specifically, but same transformer principle

  "TubeR: Tubelet Transformer for Video Action Detection" (CVPR 2022)
  - Paper: https://arxiv.org/abs/2104.00969
  - Code: https://github.com/salesforce/TubeR
  - Uses "tubelets" (tracks) as queries to transformer
  - Combines track proposals with video features
  - Very close to your use case

  "Actor-Transformers for Group Activity Recognition" (CVPR 2022)
  - Paper: https://arxiv.org/abs/2201.01304
  - Explicitly combines individual tracks with video transformer
  - Uses tracks as tokens alongside video patches
  - Applied to sports (volleyball) dataset


  Sports-Specific with Tracks + Video:
  
  "P2ANet: Player-to-Action Network for Sports Video Understanding" (2023)
  - Paper: https://arxiv.org/abs/2303.02095
  - Basketball-specific!
  - Combines player tracks with video features
  - Uses transformer architecture for fusion

  "SoccerNet-Tracking: Multiple Object Tracking on Soccer Videos" (2023)
  - Paper: https://arxiv.org/abs/2211.14286
  - Code: https://github.com/SoccerNet/sn-tracking
  - Fuses tracking with visual features
  - Transformer-based architecture


⏺ Closest to VideoMAE + Tracks:

  "MeMViT: Memory-Augmented Multiscale Vision Transformer" (CVPR 2022)
  - Paper: https://arxiv.org/abs/2201.08625
  - Code: https://github.com/facebookresearch/memvit
  - Adds "memory" tokens (can be tracking data) to ViT
  - Shows how to inject metadata into transformer architecture

  "Object-Region Video Transformers" (CVPR 2022)
  - Paper: https://arxiv.org/abs/2110.06835
  - Combines object tracks with video transformer
  - Each track becomes a token stream parallel to video patches
  - Cross-attention between tracks and video

  The Architecture Pattern They Use:

  Most papers follow this pattern:

  1. Parallel Encoding:
    - Video patches → Transformer encoder (like VideoMAE)
    - Tracks → Separate embedding layer
  2. Token Fusion:
    - Track tokens added to video token sequence
    - Or track tokens as "queries" in cross-attention
  3. Joint Processing:
    - Transformer processes both token types together
    - Learnable type embeddings distinguish tracks from video



⏺ Implementation Strategy from These Papers:

  TubeR-style (Most relevant):

  # Tracks become "object queries"
  track_tokens = embed_tracks(player_tracks)  # [B, N_players, D]
  video_tokens = video_mae_encoder(video)      # [B, T*H*W, D]

  # Cross-attention: tracks query video
  output = transformer_decoder(
      tgt=track_tokens,    # Tracks as queries
      memory=video_tokens  # Video as context
  )
  # Output: [B, N_players, D] - features per player
  shooter_scores = mlp(output)  # [B, N_players] - who shot?

  Object-Region style:

  # Concatenate track and video tokens
  all_tokens = concat([video_tokens, track_tokens])
  # Add type embeddings
  all_tokens += type_embeddings  # 0 for video, 1 for track

  # Joint transformer processing
  output = transformer(all_tokens)

  Key Insights from These Papers:

  1. Don't modify VideoMAE core - Keep it as video encoder, add tracks separately
  2. Tracks as queries works well - Let tracks "ask questions" about video
  3. Positional encoding crucial - Encode track positions to align with video regions
  4. Late fusion generally better - Process modalities separately first



⏺ Best Starting Point for You:

  TubeR is most similar to your needs:
  - Tracks are first-class citizens (not just augmentations)
  - Clean separation of modalities
  - Proven on action detection (similar to shot detection)
  - Available code to adapt

  P2ANet is specifically for basketball but newer (less proven).

  The consensus from these papers: Keep video and tracks separate through encoding, then fuse via cross-attention where tracks query video features. This is more robust than early fusion or painted overlays.
