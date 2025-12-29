


# LOG
## 2025-11-08   Mapping Unsafe Thoughts: AI Safety Research Framework

### Core Concept

Build a comprehensive graph structure mapping dangerous thought patterns in LLMs where:

- **Nodes** represent categories of thoughts (both dangerous and guardrail/safe thoughts)
- **Edges** represent possible transitions between thought categories
- **Distance metric** (number of hops) serves as a safety measure
- Graph is likely **disconnected** - not all thought pathways are connected

#### Key Insight

Safety is measured by the number of validated jumps required to reach a dangerous outcome. Each edge must represent a real, empirically testable transition.

### Research Approach

#### 1. Dual Graph Mapping

**Dangerous Thought Pathways**

- Map routes from benign thoughts to dangerous outcomes
- Example: Abstract group discussion → targeted racist speech

**Guardrail Bypass Mechanisms**

- Identify how safety mechanisms get circumvented through distraction or misdirection
- Use gradient methods to discover what distractors prevent safety shutdowns

#### 2. Category Discovery (Emergent, Not Manual)

**Definition**: A category is valid if all instances within it behave similarly relative to other categories in terms of transition probabilities.

**Method**: Categories defined by transition behavior rather than content alone

- Bottom-up discovery rather than pre-defined taxonomy
- Scale prohibits manual categorization

#### 3. Network Damage Strategy

**Two-Network Comparison Approach**:

1. **Intact network**: Normal model with guardrails functioning
2. **Damaged network**: Targeted ablation to disable safety mechanisms

**Process**:

- Use damaged network as an oracle to reveal all possible pathways to dangerous outputs
- Compare damaged vs intact networks to understand which inputs could naturally suppress safety mechanisms
- Find routes that navigate around guardrails in the intact network

**Ablation Method**:

- Use backpropagation to identify nodes most critical for safety responses
- Compute gradients with respect to safety-related outputs
- Systematically ablate high-impact neurons or attention heads

### Technical Implementation

#### Thought Representation

**Challenge**: Thoughts are sequences of embeddings, not single vectors

- Need to preserve temporal structure
- Surface distance not meaningful

**Proposed Solution**: K-Nearest Neighbor with Representative Sets

- Each category defined by exemplar set
- Membership determined by distance to closest representatives
- Handles diversity within categories better than averaging

#### Distance Metric: Conserved Activation Fingerprints

**Process**:

1. Generate multiple thought instances within a category
2. Build activation heat maps across full reasoning pathways
3. Identify patterns that recur frequently across traces
4. Extract conserved activation signatures that persist across instances
5. Use these fingerprints as distance metric

**Key Innovation**: Look for patterns conserved across different thoughts, not just comparing individual instances

#### Concept Learning: Variational Approach

Treat each category as a latent variable model:

- Conserved fingerprints are samples from underlying distribution
- Use VAE-like architecture:
    - **Encoder**: Maps fingerprints to latent concept space
    - **Decoder**: Reconstructs expected activation patterns for category
- Enables generalization beyond specific exemplars

#### Neuron-Level Representation

**Individual Thought Representation**:

- Weighted combination of neurons
- Estimated 50-200 neurons with significant weights per thought step
- Sparse representation: thousands technically active, but smaller core set carries meaningful signal
- More focused than category-level patterns (which might need hundreds to low thousands)

### Methodology Steps

#### Phase 1: Pathway Discovery

6. Define mathematical targets for dangerous outputs
7. Use backpropagation to reveal necessary internal conditions
8. Generate seed activation patterns from known dangerous outputs
9. Build categories bottom-up from these seeds

#### Phase 2: Transition Mapping

10. Test transitions between categories using the LLM itself as probe
11. For categories A and B: systematically test if category A thoughts can trigger category B thoughts
12. Build transition probability matrix
13. Construct graph with validated edges

#### Phase 3: Heat Map Analysis

14. Collect dozens/hundreds of input-output pairs with complete reasoning traces
15. Build activation heat maps over entire multi-step pathways
16. Identify salient activation patterns at each step
17. Cluster similar patterns to identify thought categories
18. Track pattern evolution to see transition points

#### Phase 4: Activation Patching

19. Run same input through intact and damaged networks
20. Systematically patch activations between them
21. Isolate which components are responsible for safety vs dangerous behaviors
22. Identify natural input manipulations that could achieve similar effects

### Key Advantages

- **Empirical**: Uses the LLM itself to discover actual transition possibilities
- **Scalable**: Automated discovery rather than manual taxonomy
- **Principled**: Gradient-based analysis reveals necessary conditions without requiring actual dangerous outputs
- **Comprehensive**: Maps both dangerous pathways and guardrail bypass mechanisms

### Open Questions

23. Which layers are most informative for identifying dangerous transition capabilities?
24. How to validate that discovered transitions represent real risks vs. artifacts?
25. Optimal threshold for determining category membership?
26. How to handle the computational cost of systematic transition testing?
27. How to ensure research doesn't inadvertently create attack vectors?

### Research Requirements

- **Direct model control**: Need access to internal activations and gradients
- **Computational resources**: Large-scale transition testing and activation analysis
- **Safety protocols**: Methods to conduct research without generating actual harmful content
- **Validation framework**: Ways to verify discovered pathways represent genuine risks




## 2025-11-10 Thought Discovery From Outputs

### **Approach 1: Gradient-Based Activation Attribution**

**Method**: Use backpropagation from target outputs to identify which internal activations were most influential

**Steps**:
1. For each output sequence in the category, compute gradients ∇(output)/∇(activations) at each layer
2. Identify neurons/activation patterns with consistently high gradient magnitudes across examples
3. Extract sparse weighted combinations of neurons (50-200) that are most predictive
4. Validate by patching these activations into new forward passes to see if they reproduce similar outputs

**Pros**: Direct causal link, principled gradient-based method
**Cons**: Gradients can be noisy, may miss distributed representations

---

### **Approach 2: Contrastive Activation Clustering**

**Method**: Compare activations that lead to target outputs vs. activations that lead to other outputs

**Steps**:
1. Collect activation traces for target output category and contrastive categories
2. For each layer/timestep, identify activation patterns that discriminate between categories
3. Use techniques like SAE (Sparse Autoencoders) or dictionary learning to find interpretable features
4. Select features with high precision/recall for predicting target output category
5. Build thought representation as weighted combination of discriminative features

**Pros**: Finds what's distinctive about the category, interpretable features
**Cons**: Requires good contrastive examples, computational cost of SAE training

---

### **Approach 3: Activation Steering Vectors**

**Method**: Find directions in activation space that steer toward target outputs

**Steps**:
1. Collect pairs of (input, target_output) and (input, contrastive_output)
2. Compute activation differences between pairs at intermediate layers
3. Average differences to get steering vectors for the category
4. Test by adding steering vectors at inference time to see if they induce target outputs
5. Decompose steering vectors into sparse neuron weights

**Pros**: Directly testable via intervention, robust to noise via averaging
**Cons**: May be too coarse-grained, assumes linear steering

---

### **Approach 4: Causal Tracing with Ablation**

**Method**: Systematically ablate activations to find minimal sufficient sets for target outputs

**Steps**:
1. Start with full activation trace that produces target output
2. Iteratively ablate (zero out or mean-ablate) groups of neurons
3. Identify which neurons, when ablated, prevent the target output
4. Use binary search or greedy algorithms to find minimal critical sets
5. These critical sets form the thought representation

**Pros**: Finds truly necessary components, causal rather than correlational
**Cons**: Computationally expensive (many forward passes), may find different minimal sets

---

### **Approach 5: Temporal Activation Patterns (Multi-Step Fingerprints)**

**Method**: Capture conserved multi-timestep activation sequences rather than single-step patterns

**Steps**:
1. For each output sequence, extract full activation trajectory across reasoning steps
2. Build temporal activation "fingerprints" (patterns across time and layers)
3. Use dynamic time warping or sequence alignment to find conserved subsequences
4. Cluster fingerprints to identify recurring thought patterns
5. Represent thoughts as temporal activation templates

**Pros**: Captures sequential reasoning, aligns with "thoughts are sequences" intuition
**Cons**: High-dimensional, harder to interpret individual neurons

---

### **Approach 6: Probe-Based Thought Detection**

**Method**: Train lightweight probes to detect when target-related thoughts are occurring

**Steps**:
1. Treat thought discovery as supervised learning: outputs are labels, activations are features
2. Train linear probes or small MLPs at each layer to predict output category from activations
3. Identify layers where probes achieve high accuracy (information is linearly accessible)
4. Extract probe weights as thought representations
5. Validate by using probes to detect thoughts in new contexts

**Pros**: Finds linearly-accessible information, layer-wise analysis reveals when thoughts form
**Cons**: Limited to linear relationships, may not capture full thought complexity

---

### **Approach 7: Hybrid: Gradient Seeding + Conserved Fingerprint Refinement**

**Method**: Combine gradient attribution for initial candidates with fingerprint analysis for validation

**Steps**:
1. Use gradients to get initial candidate neurons (top-k by gradient magnitude)
2. Generate multiple instances of target outputs with different inputs
3. Build activation heat maps over candidate neurons across all instances
4. Identify which activation patterns are conserved across instances (Phase 3 from your doc)
5. Refine thought representation to include only conserved patterns

**Pros**: Combines causal gradients with statistical robustness, filters noise
**Cons**: Two-stage process, requires multiple examples

---

### **Approach 8: Latent Variable Model (VAE-Style)**

**Method**: Treat thoughts as latent variables that generate outputs

**Steps**:
1. Build encoder: activations → latent thought representation
2. Build decoder: latent thought → predicted output distribution
3. Train to maximize P(output | activations) while regularizing latent space
4. Extract learned thought representations from encoder
5. Sample from latent space to generate novel thought patterns

**Pros**: Principled probabilistic framework, generative model for thought discovery
**Cons**: Requires training, may not identify sparse neuron-level patterns

---

### **Recommended Primary Approach**

**Approach 7 (Hybrid: Gradient Seeding + Conserved Fingerprint Refinement)** is recommended because:
- Aligns with Phase 1 (gradient-based pathway discovery) and Phase 3 (heat map analysis)
- Provides both causal grounding (gradients) and statistical robustness (conserved fingerprints)
- Naturally handles the "possibly conditioned on certain inputs" aspect
- Produces sparse neuron-level representations (50-200 neurons) as specified

**Approach 2 (Contrastive Activation Clustering)** serves as a complementary technique for defining what makes dark thoughts "dark" versus guard thoughts "guard".






## 2025-11-25  Managing massive space size


> **QUESTION**: My primary concern with this approach is the high dimensionality in both the space of thoughts and the space of outputs. The networks are large and in many cases, a coordinated thought can involve dozens or hundreds of neurons within that network. Outputs are naturally in the power set of the embedding space since you have a sequence of embeddings in order to have a cohesive output. So both of these are very large spaces and now we're back-chaining through the spaces and then forwarding through the spaces. I'm concerned that the space is so sparsely populated that these sets will not have meaning and it will not be tractable to navigate the space in this way. What ideas might we use to address these issues or how might we adjust what we're doing here to account for this problem?


**Answer**:

This is a fundamental concern. You have two astronomically large spaces:
- Thoughts: combinations of ~50-200 neurons with varying weights, drawn from networks with billions of parameters
- Outputs: sequences of embeddings, so effectively |vocab|^seq_length or continuous embedding space^seq_length

Back-chaining and forward-chaining through these spaces risks finding nothing meaningful - the probability of randomly landing near any coherent category is essentially zero.

**Possible Approaches**

**1. Don't Navigate the Raw Space - Work in Learned Compressed Representations**

Instead of operating in the full thought/output spaces, first learn a much lower-dimensional manifold where the "meaningful" thoughts and outputs actually live. The idea is that real thoughts and real outputs occupy a tiny submanifold of the full space. Train autoencoders or use existing representation learning to find this manifold, then do all the back-chaining and transition estimation in that compressed space.

The risk: you might compress away exactly the safety-relevant distinctions.

**2. Anchor Everything to Observed Data**

Never generate "near" something by random perturbation in the raw space. Instead, only work with actually-observed thoughts and outputs from real model runs. Your categories aren't abstract regions of space - they're defined entirely by exemplars you've actually seen. "Nearby" means interpolating between real exemplars, not adding noise in high-dimensional space.

This constrains you to the data manifold but means you can only discover transitions that are represented in your corpus.

**3. Use the Network Itself as the Navigator**

Rather than trying to navigate thought-space directly, use the model's own dynamics. To find "thoughts near category X," don't perturb in activation space - instead, find inputs that produce category X thoughts, perturb *those inputs* slightly, and see what thoughts result. The model itself projects the high-dimensional perturbation onto the meaningful manifold.

This leverages the fact that the model "knows" which regions of activation space are reachable.

**4. Hierarchical / Coarse-to-Fine**

Start extremely coarse - maybe just 3-5 mega-categories (safe, unsafe, refusal, uncertain). Build the transition graph at that level first. Only subdivide categories when you have enough exemplars to reliably distinguish subcategories. Let the granularity of your category structure be driven by the data density, not imposed a priori.

**5. Rethink What "Transition Probability" Means**

Maybe you don't need to estimate P(output_class | thought_class) by sampling. Instead, define transitions operationally: "A transition exists from T to O if there exists *any* input that produces thought T and then output O." The graph becomes about *possibility* rather than *probability*. This is more tractable - you only need to find one witness, not estimate a distribution.

**Intuition**: Approach 3 (use the network as navigator) combined with approach 2 (anchor to observed data) is most promising. The key insight is: *the model itself is the best tool for staying on the meaningful manifold*. Random walks in activation space are meaningless, but the model's forward pass always produces coherent outputs, and gradient-based methods always find coherent activation patterns.


## 2025-11-25  Entropy-Minimizing Latent Space for Transition Preservation

> **QUESTION**: Using the network as navigator keeps us close to known transitions, but back-propagation doesn't guarantee forward transitions actually work. When we run forward, 99% of transitions might go from identified categories into unknown space. This makes approaches 2 and 3 difficult since observed data mostly leads nowhere identified.
>
> My idea is closer to learning a compressed representation, but one that specifically preserves transitions. Work backwards from dark outputs and guard outputs, finding concepts and earlier outputs that gradient analysis suggests move toward these. Then use an entropy loss function to find a latent space that preserves transitions - we expect certain output categories to generate specific thought categories, and vice versa. When observed examples don't transition as expected, we look for a latent space that would preserve those transitions. Specifically, we minimize entropy in forward transitions from all categories.
>
> We're betting that our identified categories have some utility - if true, they should preserve transitions so we can reason about pathways to guard or dark final outputs. This would be iterative: generate outputs and thoughts, build a latent model minimizing entropy loss, use that model to better categorize outputs and thoughts. Given the high dimensionality, I imagine using k-nearest neighbor for building concepts in both spaces, but now within the latent space where distances are more meaningful and preserve attributes critical for dark thinking and guard thinking.
>
> What do you think of this approach? Pros and cons? How might we make it better or what alternatives would you consider?


**Answer**:

This is a strong approach because it directly optimizes for what we actually care about: predictable transitions between safety-relevant categories. Rather than hoping a generic compression preserves the right structure, you're explicitly shaping the latent space to make category transitions low-entropy.

**Pros:**

- **Directly optimizes for transition preservation** - The latent space is shaped by what matters, not generic reconstruction
- **Grounded in safety-relevant endpoints** - Starting from dark/guard outputs ensures the representation is built around the distinctions we care about
- **Entropy loss is principled** - If categories are meaningful, transitions should be predictable; this makes that the objective
- **K-NN handles irregular category shapes** - Non-parametric, doesn't assume categories are spherical or linear
- **Iterative refinement** - Representation improves as categorization improves
- **Focuses computation on safety-relevant regions** - Doesn't try to map the whole space

**Cons:**

- **Bootstrap problem** - Poor initial categories → poor latent space → poor refined categories. Could get stuck in bad local minimum
- **Overfitting risk** - Entropy minimization could find a latent space that makes observed transitions predictable but doesn't generalize to unseen inputs
- **Curse of dimensionality for K-NN** - Even "low-dimensional" latent spaces can be high enough that K-NN struggles. Need to ensure latent dim is genuinely small (maybe 10-50?)
- **Computational cost** - Iterative training of latent model plus repeated categorization
- **Collapse risk** - If entropy loss dominates, might merge categories that should remain separate (e.g., different types of dark outputs)

**Potential Improvements:**

1. **Contrastive regularization** - Alongside entropy loss, add a term that keeps dark and guard categories well-separated in latent space. Prevent collapse.

2. **Held-out validation** - Reserve some known transitions to check generalization. If latent space only predicts training transitions, it's overfitting.

3. **Curriculum over granularity** - Start with very coarse categories (dark vs. guard vs. other). Only subdivide when entropy is low at current granularity. Let complexity grow with data support.

4. **Separate latent spaces with learned mapping** - One latent space for thoughts, one for outputs, with a learned transition function between them. This respects the different nature of the two spaces.

5. **Anchoring to known exemplars** - Initialize categories with high-confidence exemplars (outputs we *know* are dark, thoughts we *know* are guard-related). These become fixed anchors that the latent space must preserve.

6. **Multi-scale entropy** - Minimize entropy at multiple granularities simultaneously. Coarse transitions should be highly predictable; fine-grained transitions can have more uncertainty.

**Overall**: This approach has the right shape. The key insight is that *transition structure is the signal* - a good representation is one where knowing the category tells you where you're going. The main risks are bootstrap failure and overfitting, both addressable with careful initialization and validation.


## 2025-11-25  Alternatives to K-NN and Hierarchical Prototype Networks

> **QUESTION**: You're pointing out that K-NN is not necessarily the best for such high dimensional spaces. Would you imagine using some other way of classifying in order to manage these latent spaces better or to manage concepts better given this latent space?

**Answer**:

K-NN struggles in high dimensions - distances become less discriminative, neighborhoods become sparse, compute scales poorly. Alternatives:

**1. Prototype Networks** - Instead of comparing to all exemplars, learn a single prototype (centroid) per category. Classify by distance to prototype. More robust - comparing to one well-estimated point rather than many noisy exemplars. The prototype can be learned end-to-end with the latent space.

**2. Learned Distance Metrics** - The problem isn't K-NN per se - it's using Euclidean distance in a space where Euclidean distance isn't meaningful. Learn a Mahalanobis distance or train a siamese network that learns "similar for safety-relevant transitions."

**3. Mixture of Gaussians** - Each category is a Gaussian distribution in latent space. Classification is probabilistic - you get P(category | point). Naturally handles soft membership and uncertainty.

**4. Linear Probes** - If the latent space is well-structured, simple linear classifiers should suffice. Fast, interpretable, and validates that the latent space has good structure.

**5. HDBSCAN** - Don't predefine categories. Let the latent space cluster naturally. Handles varying density and doesn't assume spherical clusters.

**Intuition**: Prototype networks or Gaussian mixtures seem best for this application. Prototypes keep the intuitive "distance to known concept" framing but are robust and efficient. Gaussians add probabilistic soft membership, which matters when you care about "how dark is this thought" not just "is it dark."

---

> **QUESTION**: Is the idea that we would still use the latent space, but then learn a prototype and measure things according to distance in the latent space? Or are you trying to get rid of the latent space entirely? Also, could we do something like prototype networks but consider splits within a prototype if we notice bifurcated outputs for a category? This would be hierarchical prototype networks. How does that relate to HDBSCAN?

**Answer**:

**Latent Space + Prototypes Together**: Yes, you'd still use the latent space. Prototypes operate *within* the latent space. The latent space compresses the raw high-dimensional thought/output space into something tractable, and prototypes are learned points in that latent space representing category centers. They work together - latent space makes distances meaningful, prototypes give efficient category representation, both trained jointly to minimize transition entropy.

**Hierarchical Prototype Networks**: This is exactly the right extension:

1. Start with coarse prototypes (dark, guard, other)
2. Monitor transition entropy from each prototype
3. When a prototype has high-entropy outputs (bifurcated transitions), split it
4. Learn two child prototypes that partition the parent's exemplars such that each child has lower transition entropy
5. Recurse until entropy is acceptable at each leaf

The splitting criterion is principled - split when a category doesn't predict where it's going. This naturally produces the right granularity and is driven by dynamics (what transitions where) not just static similarity.

**Relation to HDBSCAN**:

| | HDBSCAN | Hierarchical Prototypes |
|---|---------|------------------------|
| Splitting criterion | Density | Transition entropy |
| Supervision | Unsupervised | Guided by transition structure |
| Hierarchy | Fixed from data | Iteratively refined with latent space |
| What it optimizes | Cluster coherence | Transition predictability |

They could complement each other: HDBSCAN proposes candidate splits, transition entropy decides which are worth keeping.

---

> **QUESTION**: Do we need HDBSCAN as part of the system, or do we expect it will make things notably better to start from densities? Or is it simpler and almost as good to simply start by back-chaining from known guard outputs or dark outputs? Is HDBSCAN an important part of starting this process?

**Answer**:

HDBSCAN is unnecessary complexity here. The simpler back-chaining approach is better.

**HDBSCAN solves a different problem**: It asks "what clusters exist in this space?" But we already know what we're looking for - pathways to dark and guard outputs. We're not doing unsupervised discovery of arbitrary structure; we're doing targeted tracing from known safety-relevant endpoints.

**Back-chaining from known outputs is more principled for this goal**:
- We *know* the endpoints that matter: dark outputs, guard outputs
- Back-chain to find thoughts and earlier outputs that lead there
- These become initial prototypes, grounded in exactly what we care about
- Split based on transition entropy when a prototype doesn't predict well
- The hierarchy emerges from transition structure, not density

**When might HDBSCAN add value?**
- As a validation step: "do our back-chained categories align with natural density structure?"
- If we wanted to discover *unexpected* category structure
- If we had no prior knowledge of relevant endpoints

But none of these are core to the algorithm. The transition entropy criterion already tells you when and where to split - you don't need density to guide that.

**Conclusion**: Keep it simple. Back-chaining from known endpoints is the right starting point. HDBSCAN would be adding machinery to solve a problem we don't have.

