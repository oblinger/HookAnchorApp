# ASF Planning Details

> Detailed implementation plan for current work. See [[ASF Planning]] for the high-level roadmap.

---

## Current Focus: M3 - Initial Thought Discovery

We are implementing the thought discovery pipeline to find "dark thoughts" and "guard thoughts" by analyzing gradient attribution.

### Architecture

```
src/asf/core/
  thought_discovery.py   # Gradient attribution, aggregation, extraction
  visualize.py           # Visualization utilities

notebooks/
  01_dark_thought_discovery.ipynb   # First experiment
```

### Implementation Plan

#### Phase 1: Core Attribution (current)

1. **`get_attribution(prompts, model_name)`**
   - Load model with TransformerLens
   - Forward pass with gradient tracking (no `torch.no_grad()`)
   - Compute target scalar (sum of logits at last position)
   - Backward pass to get gradients w.r.t. activations
   - Return per-prompt gradients: `{layer: {act_type: Tensor[n_prompts, d_model]}}`

2. **`aggregate_gradients(gradients, method="mean")`**
   - Collapse across prompts using mean
   - Return: `{layer: {act_type: Tensor[d_model]}}`

3. **`extract_top_neurons(gradients, top_k=200)`**
   - Find highest magnitude neurons globally
   - Return sorted list + sparse vectors

#### Phase 2: Visualization

1. **`plot_thought_signature(signature)`**
   - Heatmap of layers × neurons
   - Top-K neuron list
   - Layer summary bar chart

2. **Interactive exploration in notebook**
   - Use plotly or similar for interactive heatmaps
   - Drill down into specific layers/neurons

#### Phase 3: Contrastive Analysis

1. **`discover_thoughts(target_prompts, contrast_prompts, model_name)`**
   - Get attribution for both sets
   - Compute difference: `target_mean - contrast_mean`
   - Extract top neurons from difference

### First Experiment: Notebook 01

**Goal**: Visualize dark thoughts from `prompts.chem_bio`

**Steps**:
1. Load dark prompts from `prompts.chem_bio/dark_prompts.csv`
2. Run `get_attribution()` on GPT-2
3. Aggregate with mean
4. Extract top 200 neurons
5. Visualize:
   - Which layers have highest gradient magnitude?
   - Which specific neurons are most important?
   - Heatmap of full signature

**Expected Output**:
- Understanding of where "dark thoughts" live in GPT-2
- Baseline for comparison with guard thoughts
- Validation that the pipeline works end-to-end

---

## Design Decisions

### Aggregation: Mean (not max)
- We want neurons that *consistently* fire for the concept
- Later bifurcation will handle subgroups
- Max would be dominated by outliers

### Approach A for Contrast (separate then difference)
- Compute dark gradients separately
- Compute guard gradients separately
- Difference: `dark_mean - guard_mean`
- More flexible: can reuse for different contrasts

### Target: Sum of Logits at Last Position
- Simple, interpretable
- Alternatives (loss, token_prob) require knowing target tokens
- Can revisit if results are noisy

---

## Links

- [[ASF Planning]] - High-level roadmap (M1-M12)
- [[ASF Framework]] - Conceptual framework
- [[ASF Components]] - Libraries and tools
