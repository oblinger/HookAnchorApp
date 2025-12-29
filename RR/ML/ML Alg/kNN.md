


knn
-?-
k nearest neighbor

approximate knn
-?- 
- Small data (<100K): Exact KNN is fine
- Medium (100K-10M): HNSW (best quality) or Annoy (spotify - simpler)
- Large (10M+): IVF-PQ or FAISS (meta) composite indexes
- Need compression: Product Quantization

LHS - Locality Sensitive Hashing
KD-Tree
Annoy- Random Projections
.
HNSW - Hier Nav Small World (Meta FAISS)
#ml 