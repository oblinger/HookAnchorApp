
- Each head will contribute d_h float values to the output matrix --- size(n, embed_dim) -- 
- so d_h * num_heads = embed_dim = d_k
- thus embed_dim % num_heads == 0 
