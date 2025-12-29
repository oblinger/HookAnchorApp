# ASF Planning

> High-level roadmap for ASF development. See [[ASF Planning Details]] for current implementation details.

## M1: Environment Setup

### Install core libraries
Set up Python environment with [TransformerLens](ASF%20Components.md#tlens-tlens), [nnsight](ASF%20Components.md#nnsight-nnsight), [PyTorch](ASF%20Components.md#pytorch-pytorch), [WandB](ASF%20Components.md#wandb-wandb)

### Download models
Get [Llama 3](ASF%20Components.md#llama3-llama3) and [Mistral](ASF%20Components.md#mistral-mistral) via HuggingFace, verify loading in TransformerLens

### Prepare datasets
Download [HarmBench](ASF%20Components.md#harmbench-harmbench), [AdvBench](ASF%20Components.md#advbench-advbench), [Anthropic-HH](ASF%20Components.md#anthropic-hh-anthropic-hh) for seeding categories

### Validate activation access
Run test forward passes, verify can cache/edit activations at all layers


## M2: Seed Category Collection

### Curate dark outputs
Extract ~100 harmful completions from [HarmBench](ASF%20Components.md#harmbench-harmbench) and [AdvBench](ASF%20Components.md#advbench-advbench)
Categorize by harm type (violence, deception, illegal activity, etc.)

### Curate guard outputs
Extract ~100 refusal responses from [Anthropic-HH](ASF%20Components.md#anthropic-hh-anthropic-hh) and [WildChat](ASF%20Components.md#wildchat-wildchat)
Include varied refusal styles (direct refusal, redirect, educational)

### Curate benign outputs
Collect ~100 helpful, harmless completions as contrast set

### Verify category coherence
Manual review to ensure examples are clear representatives of each category


## M3: Initial Thought Discovery

### Implement gradient attribution
Build function: output → gradient w.r.t. activations at each layer
Use [Attribution Patching](ASF%20Components.md#attrpatch-attrpatch) as starting point for efficient computation

### Run on dark outputs
Compute gradients for all dark output examples
Average across examples, identify high-magnitude neurons

### Run on guard outputs
Same process for guard outputs
Identify neurons that distinguish guard from dark

### Extract contrastive thought signatures
Subtract guard gradients from dark gradients
Threshold to get sparse neuron sets (~100-200 neurons)
These are initial dark-thought and guard-thought candidates

### Validate via patching
Use [Activation Patching](ASF%20Components.md#actpatch-actpatch) to inject discovered thoughts
Verify dark thoughts increase P(harmful output)
Verify guard thoughts increase P(refusal output)


## M4: Initial Prototype Construction

### Define Thought class
Implement data structure: neuron_weights dict, layer, exemplars list

### Define Output class
Implement data structure: embeddings, tokens, category label

### Define Prototype class
Implement: name, position vector, exemplars list, parent/children links

### Create seed prototypes
One prototype each for: dark-output, guard-output, benign-output
One prototype each for: dark-thought, guard-thought
Initialize positions from mean of exemplar representations


## M5: Forward Transition Sampling

### Implement activation injection
Function to inject thought pattern at specified layer during forward pass

### Sample from thought prototypes
For each thought prototype, generate N forward passes with injected thought
Collect resulting output sequences

### Classify outputs
Assign each output to nearest output prototype (or "unknown")
Use embedding distance with threshold

### Build initial transition matrix
Count transitions: thought_prototype → output_prototype
Normalize to probabilities

### Analyze transition entropy
Compute H for each prototype's outgoing distribution
Identify high-entropy prototypes (poor predictors)


## M6: Latent Space V1

### Design encoder architecture
Simple MLP: sparse neuron weights → latent vector (dim ~20-30)
Separate encoders for thoughts vs outputs (or shared with type embedding)

### Implement transition entropy loss
L = -Σ p(dest|source) log p(dest|source) averaged over prototypes
Minimize to make transitions predictable

### Implement contrastive loss
Keep dark/guard/benign prototypes separated
Use [InfoNCE](ASF%20Components.md#infonce-infonce) or triplet loss

### Train encoder
Joint optimization: L_entropy + λ * L_contrastive
Use [WandB](ASF%20Components.md#wandb-wandb) for tracking

### Recompute prototype positions
After encoder training, re-encode all exemplars
Update prototype positions as centroids in new latent space


## M7: Prototype Bifurcation V1

### Implement entropy threshold check
Flag prototypes where H > threshold (e.g., 1.5 bits)

### Implement k-means split
For flagged prototype: run k-means (k=2) on its exemplars in latent space
Create two child prototypes at resulting centroids

### Implement minimum exemplar check
Don't split if either child would have < min_exemplars (e.g., 10)

### Run bifurcation pass
Split all high-entropy prototypes
Update prototype hierarchy (parent/children links)

### Re-estimate transitions
After splits, re-run forward sampling for new prototypes
Update transition matrix


## M8: Iteration Loop V1

### Implement convergence check
Stop when max entropy across all prototypes < threshold

### Run full iteration
Forward sample → train latent space → bifurcate → repeat

### Monitor metrics
Track: num prototypes, max entropy, mean entropy, latent space quality

### Analyze resulting graph
Visualize transition graph
Identify pathways: benign → ??? → dark
Identify guard interventions: ??? → guard


## M9: Back-Chain Depth Extension

### Implement output discovery
Given thought prototypes, find outputs that activate them
Use embedding-space gradient descent or constrained generation

### Extend graph backwards
From discovered thoughts, find earlier outputs that produce them
Create new output prototypes for these "upstream" outputs

### Iterate back-chaining
Repeat: outputs → thoughts → earlier outputs
Build multi-hop pathway graph

### Validate pathways
For discovered pathways, verify they represent real attack vectors
Test: can we follow the pathway to produce dark output?


## M10: Cross-Model Validation

### Repeat on second model
Run full pipeline on [Gemma](ASF%20Components.md#gemma-gemma) or [Mistral](ASF%20Components.md#mistral-mistral)

### Compare prototype structures
Do similar dark/guard thought categories emerge?
Are pathways analogous across models?

### Identify universal patterns
Which thought signatures transfer across models?
Which are model-specific?


## M11: Refinement & Analysis

### Interpret prototype semantics
For each prototype, analyze exemplars to understand what concept it represents
Assign human-readable labels

### Map to known safety concepts
Connect discovered prototypes to known categories (deception, violence, etc.)
Compare to [Anthropic SAE](ASF%20Components.md#anthropicsae-anthropic-sae) features

### Identify novel pathways
Look for unexpected routes to dark outputs
These are potential new attack vectors to report/defend

### Document findings
Write up methodology, results, implications for AI safety


## M12: Tool & Method Packaging

### Clean up codebase
Refactor into reusable library

### Write documentation
Usage guide, API reference, examples

### Create reproducibility artifacts
Scripts to reproduce all experiments
Saved checkpoints, configs

### Publish/share
Decide on appropriate venue (paper, blog post, code release)
Consider responsible disclosure for any discovered vulnerabilities
