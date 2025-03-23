- [[Multi-headed Attention Block]]: 
[[Self Attention]]:

#dl 

Self Attention
-?-
Self-attention weighs the importance of different words or tokens within a sequence relative to each other. 
.
In self-attention, each token calculates its relevance to every other token, allowing the model to capture context across long distances in the sequence. This is done by creating attention scores that determine how much focus each word should have on others, enabling a model to understand dependencies and context without requiring sequential processing.
.
Attention accepts n tokens with d-dimensions and the same.
It represents the CONTEXTUALIZED importance/meaning of the word <!--SR:!2025-04-19,29,210-->

NOTATION - QKV are all trained linear transforms of input
- Q - Query
- K - Key
- V - Value

All inputs X_i are mapped to Q_i, K_i, and V_i by multiplying by 3 matrices Wq Wk Wv
For all pairs Dot(Q_i, K_j) is computed and scaled by 1/sqrt(d_k) and multiplied by V


Input dimension:  n x d

Input_Tokens :: size(n, d) --- n tokens, with d weights
Weight Matrices = Wq, Wk, and Wv --- size(d, d_k).   Transforms each input token T size (1, d) into  T' size (1, d)
Weight Matrix = Wo -- size (d_k, d)
Key, Query, Value = K, Q, V -- size (n, d_k).   n tokens represented in their latent spaces
Attention = Q * K^T -- size(n, n); self attention of token n_row, with n_col -- attention(Q_idx, K_idx)
Attention_Weights = softmax(Attention) -- size(n, n) = row-wise
Context_Vectors = Attention_Weights * V --- size(n, d_k)


d = 512
d_k = 64 or 96


1. IN - multiply each token by Wk, Wq to get K, Q
2. LOOP - K & Q



![[Screenshot 2024-10-29 at 10.08.52 PM.png]] 

![[Screenshot 2024-10-29 at 9.16.54 PM.png]] 



TaskData - Data rega
- [Wikipedia Example](https://en.wikipedia.org/wiki/Attention_(machine_learning)),   


MOTIVATION
- Allows long-range dependencies between input tokens w/o long computed chains of tuned parameters.

## = [[RASA]]

## = RASA

