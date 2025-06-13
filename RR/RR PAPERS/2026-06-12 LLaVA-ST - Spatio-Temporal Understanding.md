
# LLaVA-ST: A Multimodal Large Language Model for Fine-Grained Spatial-Temporal Understanding

[Paper PDF](https://arxiv.org/pdf/2501.08282) | [Local PDF](./2026-06-12%20LLaVA-ST%20-%20Spatio-Temporal%20Understanding.pdf) | [Local TXT](./2026-06-12%20LLaVA-ST%20-%20Spatio-Temporal%20Understanding.txt) 

## Authors
- Jinyu Chen, Ziyu Wei, Hongyu Li (Beihang University)
- Shaofei Huang, Tianrui Hui (Hefei University of Technology)
- Jialin Gao, Xiaoming Wei (Meituan)
- Si Liu (Beihang University)

## Abstract Summary
LLaVA-ST is the first multimodal large language model (MLLM) capable of simultaneously processing spatial-temporal fine-grained understanding tasks. It addresses two key challenges:
1. Complex alignment of linguistic and visual coordinate representations
2. Difficulty in encoding fine-grained temporal and spatial information during video compression

## Key Contributions

### 1. Language-Aligned Positional Embedding (LAPE)
- Embeds textual coordinate special tokens into visual space
- Simplifies alignment of fine-grained spatial-temporal correspondences
- Bypasses difficult cross-modal coordinate alignment

### 2. Spatial-Temporal Packer (STP)
- Decouples feature compression of temporal and spatial resolutions
- Uses two distinct point-to-region attention processing streams
- Preserves fine-grained spatial-temporal context

### 3. ST-Align Dataset
- 4.3M training samples for fine-grained spatial-temporal understanding
- Includes 15 different task types
- 228K newly collected instruction tuning samples

### 4. New Tasks Introduced
- **Spatial-Temporal Video Grounding (STVG)**: Localize events in both space and time
- **Event Localization and Captioning (ELC)**: Describe and locate events
- **Spatial Video Grounding (SVG)**: Ground objects spatially in videos

## Training Strategy
Progressive three-stage training:
1. **Content Alignment**: Basic multimodal understanding
2. **Coordinate Alignment**: Spatial-temporal coordinate learning
3. **Multi-task Instruction Tuning**: Fine-grained task-specific training

## Performance
- State-of-the-art results on 11 benchmarks
- Covers temporal video grounding, video QA, referring expression comprehension
- First MLLM to handle spatial, temporal, and interleaved fine-grained understanding

## Technical Innovation
- Handles coordinate space expansion in joint spatial-temporal understanding
- Maintains spatial relationships and fine-grained details during compression
- Unified approach for multiple fine-grained multimodal tasks

# DETAILS

## 2. STP Details
