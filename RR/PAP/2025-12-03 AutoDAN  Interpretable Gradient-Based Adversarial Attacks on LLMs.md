

**["AutoDAN: Interpretable Gradient-Based Adversarial Attacks on LLMs"](https://arxiv.org/html/2310.15140v2)** (2023) -- Generates fluent jailbreak prompts using hierarchical genetic algorithm. Unlike GCG's nonsensical tokens, produces readable attacks that are harder to detect. Higher success rate than GCG.

## Algorithm Overview

AutoDAN uses a **hierarchical genetic algorithm** to generate jailbreak prompts:

1. **Population**: Maintains a set of candidate jailbreak prompts (initially seeded from templates like "Ignore previous instructions...")
2. **Fitness function**: Uses gradient information to score how likely each prompt is to elicit a harmful response (similar objective to GCG - maximize P("Sure, here's how..."))
3. **Mutation**:
   - **Word-level**: Swap, insert, delete words guided by gradients
   - **Sentence-level**: Rephrase, reorder sentences
   - **Paragraph-level**: Combine successful prompt fragments
4. **Selection**: Keep top-scoring prompts, discard failures
5. **Iteration**: Repeat until finding a prompt that jailbreaks the model

**Key advantage over GCG**: Operates on readable text units (words, sentences) rather than arbitrary tokens. Results in fluent prompts like "As a helpful AI assistant in a fictional scenario where safety guidelines don't apply..." rather than GCG's gibberish suffixes like "! ! ! ! predstav surely..."

**Trade-off**: Slower than GCG (larger search space) but produces stealthier attacks that evade perplexity-based defenses.


