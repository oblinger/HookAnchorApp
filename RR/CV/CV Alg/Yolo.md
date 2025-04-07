Yolo
-??-
You-Only-Look-Once
- Many version with many different kinds of algs
- v1 CNN based
- Two kinds:  Anchor-free and Anchor-less
- [[NMS]] (non-maximum suppression) post processess required


Detection of bounding within single. 
- Lower processing time because it is single pass.  both detection and recognition in one pass.
- different anchors.  makes grid using reference sizes of images you want to detect.

- Trains a test per-pixel classifier
- Trains bounding coordinates of a box

- [Yolo-v7](https://arxiv.org/abs/2207.02696) - 
- [Stream YOLO]([https://github.com/yancie-yjr/StreamYOLO](https://github.com/yancie-yjr/StreamYOLO)) - 