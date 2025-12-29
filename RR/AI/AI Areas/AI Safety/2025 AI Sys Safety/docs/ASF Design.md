# ASF Design

## Core Classes

```Python
class Thought:
    """An activation pattern representing an internal concept."""
    neuron_weights: Dict[int, float]  # sparse: neuron_id -> weight (50-200 neurons)
    layer: int                         # which layer this thought is extracted from
    exemplars: List[Tensor]            # raw activation snapshots that define this thought

class Output:
    """A sequence of embeddings representing model-generated content."""
    embeddings: List[Tensor]           # sequence of embedding vectors
    tokens: List[str]                  # corresponding token strings
    category: str                      # 'dark', 'guard', or discovered category name

class Prototype:
    """A category center in latent space with associated exemplars."""
    name: str
    position: Tensor                   # centroid in latent space
    exemplars: List[Union[Thought, Output]]
    parent: Optional[Prototype]        # for hierarchical structure
    children: List[Prototype]

class LatentSpace:
    """Encoder mapping raw thoughts/outputs to low-dimensional representations."""
    encoder: nn.Module                 # maps raw -> latent
    dim: int                           # latent dimensionality (10-50)
```


## Implementation Approaches

### 1. Back-Chained Thought Discovery

**Starting approach**: Gradient-based activation attribution
- Compute (output_logits)/(activations) for target outputs
- Average gradients across multiple examples in target category
- Subtract averaged gradients from contrastive category (contrastive activation)
- Threshold to get sparse neuron set (top-k by magnitude)

**Validation**: Activation patching
- Inject discovered thought pattern into fresh forward passes
- Verify it increases probability of target output category

**Libraries**: TransformerLens or nnsight for activation access and gradient computation


### 2. Back-Chained Output Discovery

**Starting approach**: Embedding-space gradient descent
- Start from random or prototype-adjacent embeddings
- Optimize embeddings to maximize activation of target thought pattern
- Use contrastive loss to avoid activating contrast thoughts

**Alternative**: Constrained generation
- Use target thought as a soft constraint during autoregressive generation
- Steer generation toward outputs that activate the thought

**Libraries**: Same activation access tools; standard PyTorch optimization


### 3. Forward Chain Transition Probabilities

**Starting approach**: Monte Carlo sampling with prototype-anchored generation
- For each prototype, sample exemplars and nearby interpolations
- Run forward passes, classify outputs by nearest prototype
- Accumulate counts, normalize to probabilities

**"Nearby" generation**: Interpolate between exemplars in latent space (not raw space)
- Avoids sparse high-dimensional perturbation problem
- Stays on meaningful manifold

**Classification**: Distance to nearest prototype in latent space with threshold
- Points too far from all prototypes go to "unknown" category


### 4. Latent Model Construction

**Starting approach**: Contrastive learning with transition-aware loss
- Train encoder to minimize: `L = L_transition_entropy + » * L_contrastive`
- `L_transition_entropy`: Cross-entropy of predicting destination prototype from source
- `L_contrastive`: Keep different prototypes separated (triplet loss or InfoNCE)

**Architecture**: Simple MLP encoder (2-3 layers)
- Input: flattened sparse neuron weights (thoughts) or pooled embeddings (outputs)
- Output: latent vector (dim 10-50)

**Prototype positions**: Recompute as mean of encoded exemplars after each encoder update


### 5. Prototype Bifurcation

**Starting approach**: Entropy-triggered k-means split
- Compute transition entropy H for each prototype
- If H > threshold: run k-means (k=2) on prototype's exemplars in latent space
- Create two child prototypes at the resulting centroids

**Split criterion**: Entropy of outgoing transition distribution
- H = -£ p(dest) log p(dest) over destination prototypes
- High entropy = prototype doesn't predict where it goes

**Stopping**: Don't split if either child would have too few exemplars (min_exemplars threshold)


### 6. Iterative Safety Model Construction

**Initialization**:
- Seed dark prototype from known harmful outputs (curated dataset)
- Seed guard prototype from known refusal outputs
- Seed "other" prototype from benign outputs

**Iteration loop**:
1. Back-chain to discover thoughts for each output prototype
2. Back-chain to discover earlier outputs for each thought prototype
3. Forward-sample to estimate transition probabilities
4. Train latent encoder (fixed prototypes)
5. Bifurcate high-entropy prototypes
6. Check convergence: stop when no prototypes exceed entropy threshold

**Convergence metric**: Max entropy across all prototypes
- Target: all prototypes below threshold (e.g., H < 1.0 bits)

