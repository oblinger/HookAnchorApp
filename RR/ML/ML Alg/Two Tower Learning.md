
Two Tower Learning
-?-
Used in recommendations systems to connect users and items w/o modelling interaction between them
- [[Approximate Nearest Neighbor]] 
- 



  User Tower → outputs a user embedding vector (e.g., 64-dimensional)
  Item Tower → outputs an item embedding vector (e.g., 64-dimensional)


RETRIEVAL SELECTION: Use dot product
  Score = dot product of these two vectors (not matrix multiplication)


RANKING: Use deep cross-network 

