

Perplexity
-?- 
Perplexity measures how well a language model predicts a sequence of words. It's essentially "how surprised" the model is by the text.
Perplexity = 2^([[CrossEntropy loss]]) 
PP(W) = P(w₁, w₂, ..., wₙ)^(-1/n)

  Where you take the inverse probability of the test set, normalized by the number of words.

  Intuition

  - Lower perplexity = better model (less "confused")
  - A perplexity of 100 means the model is as uncertain as if it were choosing uniformly among 100 words at each position
  - A perplexity of 1 would mean perfect prediction (impossible in practice)

  Typical Values

  | Model Type                  | Perplexity Range |
  |-----------------------------|------------------|
  | Good LLM on typical text    | 10-30            |
  | Older n-gram models         | 100-200          |
  | Random guessing (50k vocab) | ~50,000          |

  Limitations

  - Only measures prediction accuracy, not quality/usefulness of generations
  - Sensitive to tokenization (different tokenizers = incomparable perplexity)
  - Can be gamed by assigning high probability to common words
  - Doesn't capture factual correctness, coherence, or helpfulness

  Usage

  Primarily used for:
  - Comparing language models on the same test set
  - Tracking training progress
  - Evaluating domain adaptation (lower perplexity on in-domain text)


#dl