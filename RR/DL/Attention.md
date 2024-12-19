- [[Multi-headed Attention Block]]: 
[[Self Attention]]:

#dl 

Self Attention
-?-
Self-attention weighs the importance of different words or tokens within a sequence, relative to each other. In self-attention, each token calculates its relevance to every other token, allowing the model to capture context across long distances in the sequence. This is done by creating attention scores that determine how much focus each word should have on others, enabling a model to understand dependencies and context without requiring sequential processing.
.
Attention accepts n tokens with d-dimensions and the same.
It represents the CONTEXTUALIZED importance/meaning of the word
<!--SR:!2024-12-30,12,230-->

NOTATION - QKV are all trained linear transforms of input
- Q - Query
- K - Key
- V - Value

All inputs X_i are mapped to Q_i, K_i, and V_i by multiplying by 3 matrices Wq Wk Wv
For all pairs Dot(Q_i, K_j) is computed and scaled by 1/sqrt(d_k) and multiplied by V


![[Screenshot 2024-10-29 at 10.08.52 PM.png]] 

![[Screenshot 2024-10-29 at 9.16.54 PM.png]] 



TaskData - Data rega
- [Wikipedia Example](https://en.wikipedia.org/wiki/Attention_(machine_learning)),   


MOTIVATION
- Allows long range dependencies between input tokens w/o long computed chains of tuned parameters.

## = [[RASA]]

## = RASA

