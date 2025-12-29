

From:  https://medium.com/ubiai-nlp/5-natural-language-processing-models-you-should-know-836958303ce3


BERT 
-?-
Bidirectional Encoder Representation from Transformers
- Conditions on left and right of all context layers
- Can be fine-tuned by just adding single layer
- Still state of the art perormance in many areas



BERT STRUCTURE
-?-
BERT-base: 12 layers, 12 heads, 768 dims, 512 ctx, 110M params
BERT-large: 24 layers, 16 heads, 1024 dims, 512 ctx, 340M params
**FULL BERT BASE TRANSFORMER LAYER:**
1. ATTENTION SUBLAYER
   a. Project to Q, K, V: three (512 x 768) @ (768 x 768) matmuls
   b. Split into 12 heads — each gets Q, K, V of shape (512, 64)
   c. Attention per head: softmax(Q @ K.T / √64) → (512×512)
   d. Output per head: Attention @ V → (512, 64)
   e. Concatenate 12 heads → (512, 768)
   f. Project through W_O (768→768) → (512, 768)
   g. x = LayerNorm(x + attention_output)  # residual + normalize
2. Feed Fwd Net
   a. Expand: W1 @ x (768 → 3072) — 3072 neurons detect patterns
   b. ReLU: non-linearity, sparsifies, enables AND/conjunction
   c. Compress: W2 @ x (3072 → 768)
   d. x = LayerNorm(x + ffn_output)  # residual + normalize, send to next layer



TRANSFORMER LAYER BEHAVIOR
-?-
- **Single Attention Head** = content-addressed memory fingerprint across entire context window
- Each head projects ALL 768 input dims into its own 64-dim slice space via W_Q, W_K, W_V — a learned representational "lens"
- Q = "what am I looking for?", K = "what do I contain?", V = "what to return" — output is weighted sum over matches
- **FFN** (768→3072→768): constructs new features from attended info, ReLU sparsifies, compresses back
- **12 parallel heads** concatenated to 768, W_O remixes, then added to residual stream flowing through layers


POSITIONAL ENCODINGS: absolute vs relative
-?-
- **BERT (learned absolute):** 512 separate vectors, randomly initialized then trained. No structure. Can't extend context without retraining.
- **Sinusoidal (original Transformer):** sin/cos waves at different frequencies. PE(pos+k) is linear transform of PE(pos), theoretically helps with relative — but still struggles to generalize beyond training length.
- **Implicit relative (absolute encodings):** Model CAN learn "attend 5 tokens back" but learns it as many separate absolute patterns (pos 100→95, pos 200→195). Doesn't generalize as context grows — fixed network learns fixed position ranges.
- **Relative Attention (T5, Transformer-XL):** Attention score explicitly includes (i-j) distance term. Same parameters handle "5 back" at any position. Generalizes naturally.
- **RoPE (LLaMA, GPT-NeoX):** Rotates Q and K by position angle. Q_i · K_j naturally encodes (i-j) in the rotation. Relative position falls out of the math. Generalizes to longer sequences. <!--SR:!2025-12-15,3,250-->


TRAINING OBJECTIVES: MLM vs autoregressive
-?-
- **Masked Language Model (BERT):** Fill-in-the-blank — mask 15% of tokens, predict them. Sees full context (bidirectional).
- **Autoregressive/Causal LM (GPT):** Predict next token given previous. Can only see past (causal mask).
- **Span corruption (T5):** Mask contiguous spans, replace each with single sentinel, predict original spans.
- **BERT:** MLM + Next Sentence Prediction (NSP) — NSP now considered less important
- **GPT:** autoregressive only — causal mask hides future, but no random token masking like BERT
- **Why it matters:**
  - understanding → bidirectional visibility + denoising (MLM) objective
  - generation → causal visibility + next-token (autoregressive) objective


#pytorch

### XLNet

XLNet
-?-
- Outperforms BERT by a margin
- Generalized Autoregressive Pretrained for Language Understanding
- maximize the expected likelihood across **all permutations** of the input sequence factorization order

### RoBERTa


### ALBERT
- Much larger configuration outperforms BERT-large


### PaLM

PaLM 540B
-?-
Scaling Language Modeling with Pathways (GoogleAI)
- Largest models have great perf <!--SR:!2025-12-15,3,250-->


### GPT-3

GPT-3
-?-
Generative Pretrained Model
- Can create anything w/ a text structure (not just human language)
- Can automatically generate text summarizations (even of programming code)

#dl