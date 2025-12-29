# ASF Related Research


## Mechanistic Interpretability

### [Mechanistic Interpretability for AI Safety  A Review](https://arxiv.org/html/2404.14082v1)
Comprehensive review of mechanistic interpretability techniques for AI safety. Covers reverse-engineering neural networks into human-understandable algorithms and concepts. Directly relevant to our goal of understanding internal thought patterns that lead to dark or guard outputs.

### [Scaling Monosemanticity: Extracting Interpretable Features from Claude 3 Sonnet](https://transformer-circuits.pub/2024/scaling-monosemanticity/)
Anthropic's landmark paper demonstrating sparse autoencoders can extract interpretable features from production-scale models. Found features related to safety concerns including deception, sycophancy, bias, and dangerous content. Shows that safety-relevant concepts can be identified in activation space - foundational for our thought discovery approach.

### [Towards Understanding Safety Alignment: A Mechanistic Perspective from Safety Neurons](https://arxiv.org/abs/2406.14144)
Identifies "safety neurons" (~5% of neurons) that are responsible for safety behaviors in LLMs. Patching only these neurons restores >90% of safety performance. Demonstrates that safety is managed by neurons in the first several layers. Directly supports our hypothesis that dark/guard thoughts can be localized to sparse neuron sets.

### [Refusal in Language Models Is Mediated by a Single Direction](https://arxiv.org/abs/2406.11717)
Shows that refusal behavior is mediated by a one-dimensional subspace across 13 models up to 72B parameters. Erasing this direction prevents refusal; adding it elicits refusal on harmless inputs. Demonstrates the brittleness of safety fine-tuning and suggests guard thoughts may be representable as simple directions in activation space.


## Sparse Autoencoders & Feature Discovery

### [Sparse Autoencoders Find Highly Interpretable Features in Language Models](https://openreview.net/forum?id=F76bwRSLeK)
Foundational paper on using SAEs to decompose polysemantic neurons into interpretable features. SAEs address the superposition problem where single neurons represent multiple concepts. Relevant for our thought representation - could use SAEs to find interpretable basis for dark/guard thought patterns.

### [An Intuitive Explanation of Sparse Autoencoders for LLM Interpretability](https://adamkarvonen.github.io/machine_learning/2024/06/11/sae-intuitions.html)
Accessible introduction to SAEs and their role in interpretability. Explains how SAEs help break down polysemanticity. Useful background for implementing our contrastive activation clustering approach (Approach 2 in thought discovery).

### [The Engineering Challenges of Scaling Interpretability](https://www.anthropic.com/research/engineering-challenges-interpretability)
Anthropic's discussion of practical challenges in scaling interpretability methods. Addresses computational costs and engineering trade-offs. Relevant for planning our iterative safety model construction at scale.


## Activation Patching & Causal Tracing

### [How to Use and Interpret Activation Patching](https://arxiv.org/html/2404.15255v1)
Detailed guide on activation patching techniques for understanding model internals. Covers metrics, methods, and interpretation challenges. Core technique for our thought discovery validation - patching discovered thoughts to verify they produce expected outputs.

### [Attribution Patching: Activation Patching at Industrial Scale](https://www.neelnanda.io/mechanistic-interpretability/attribution-patching)
Gradient-based approximation to activation patching that scales better. Uses gradients to efficiently narrow down which components matter. Could accelerate our back-chaining thought discovery when working with large models.

### [Towards Automated Circuit Discovery for Mechanistic Interpretability](https://arxiv.org/abs/2304.14997)
ACDC algorithm for automatically finding important subgraphs in models. Systematizes the process of identifying circuits responsible for behaviors. Related to our goal of mapping pathways from benign thoughts to dark outputs.

### [Transcoders Find Interpretable LLM Feature Circuits](https://arxiv.org/abs/2406.11944)
Uses transcoders to perform circuit analysis through MLP layers. Achieves interpretable decomposition of model computations. Could complement our approach for understanding how thoughts transform into outputs through MLP layers.


## Steering Vectors & Representation Engineering

### [Steering Language Models With Activation Engineering](https://arxiv.org/abs/2308.10248)
Introduces activation addition (ActAdd) for steering model outputs by adding direction vectors. Achieves sentiment shift and detoxification without optimization. Directly relevant to our activation steering vectors approach (Approach 3) - shows steering works for safety-relevant behaviors.

### [Representation Engineering: A Top-Down Approach to AI Transparency](https://arxiv.org/abs/2310.01405)
Proposes focusing on population-level representations rather than individual neurons. Provides methods for monitoring and manipulating high-level cognitive phenomena. Demonstrates control over honesty, harmlessness, and power-seeking - exactly the behaviors we want to map in our transition graph.

### [Steering Llama 2 via Contrastive Activation Addition](https://aclanthology.org/2024/acl-long.828.pdf)
Uses contrastive pairs to compute steering vectors. Shows effectiveness across multiple behaviors. Supports our contrastive approach to distinguishing dark thoughts from guard thoughts.

### [Steering LLMs' Behavior with Concept Activation Vectors](https://www.lesswrong.com/posts/ocopJXtcRMHjZxwbm/steering-llms-behavior-with-concept-activation-vectors)
Applies Concept Activation Vectors (CAVs) to steer LLM outputs. Shows that different jailbreak methods stimulate the same activation vector corresponding to refusal. Suggests our dark thought categories may have consistent activation signatures across different attack vectors.


## Jailbreaking & Safety Alignment

### [Jailbreaking Leading Safety-Aligned LLMs with Simple Adaptive Attacks](https://arxiv.org/abs/2404.02151)
Demonstrates 100% attack success rate on major models including GPT-4 and Claude. Shows current safety alignment is brittle. Motivates our work - we need to understand the pathways that enable these bypasses.

### [How Alignment and Jailbreak Work: Explain LLM Safety through Intermediate Hidden States](https://aclanthology.org/2024.findings-emnlp.139/)
Finds that LLMs learn ethical concepts during pre-training, and alignment associates these with emotions in middle layers. Jailbreaks disturb the transformation of early unethical classification into negative emotions. Provides mechanistic insight into how guard thoughts might be bypassed - relevant to our guardrail bypass mapping.

### [Awesome-Jailbreak-on-LLMs](https://github.com/yueliu1999/Awesome-Jailbreak-on-LLMs)
Comprehensive collection of jailbreak methods, papers, and datasets. Resource for seeding our dark output categories with known harmful output patterns.


## Prototype Networks & Few-Shot Learning

### [Prototypical Networks for Few-shot Learning](https://arxiv.org/abs/1703.05175)
Seminal paper on learning metric spaces where classification is performed by distance to class prototypes. Learns embedding where points cluster around single prototype per class. Foundational for our hierarchical prototype network approach - we adapt this for thought/output categorization.

### [Prototypical Networks (Papers With Code)](https://paperswithcode.com/paper/prototypical-networks-for-few-shot-learning)
Implementation resources and extensions. Notes that prototype approach is simpler and more efficient than complex meta-learning. Supports our choice of prototypes over more complex clustering methods.


## Latent Space & Representation Learning

### [Variational Autoencoders (Wikipedia)](https://en.wikipedia.org/wiki/Variational_autoencoder)
Overview of VAEs for learning continuous latent representations. Encoder outputs distribution over latent space, enabling smooth interpolation. Background for our latent model construction - we use similar ideas but optimize for transition entropy rather than reconstruction.

### [From Latent Dynamics to Meaningful Representations](https://arxiv.org/html/2209.00905v4)
Framework for learning latent representations that capture dynamics. Enforces latent representation to follow learnable transition dynamics. Directly relevant - we want our latent space to preserve transition structure between thought and output categories.

### [Predictable Reinforcement Learning Dynamics through Entropy Rate Minimization](https://arxiv.org/abs/2311.18703)
Uses entropy rate to quantify predictability of trajectories. Trades off optimality with predictability. Supports our use of transition entropy as the optimization target for latent space learning.


## Contrastive Learning

### [Contrastive Representation Learning (Lil'Log)](https://lilianweng.github.io/posts/2021-05-31-contrastive/)
Comprehensive overview of contrastive learning methods. Covers SimCLR, MoCo, BYOL and key techniques. Background for our contrastive activation clustering approach - we use similar principles to separate dark from guard thought representations.

### [Full Guide to Contrastive Learning (Encord)](https://encord.com/blog/guide-to-contrastive-learning/)
Practical guide covering encoders, projection networks, and loss functions. Explains how to maximize similarity within positive pairs while minimizing across negative pairs. Applicable to our latent space training where we want dark, guard, and benign categories well-separated.


## Hierarchical Clustering & Entropy-Based Splitting

### [Hierarchical Co-Clustering Based on Entropy Splitting](https://web.cs.ucla.edu/~weiwang/paper/CIKM12.pdf)
Uses entropy-based criteria for hierarchical clustering. Splits clusters to minimize within-cluster entropy. Directly relevant to our prototype bifurcation approach - we split on transition entropy rather than feature entropy.

### [Interpretable Hierarchical Clustering by Constructing an Unsupervised Decision Tree](https://www.semanticscholar.org/paper/Interpretable-hierarchical-clustering-by-an-tree-Basak-Krishnapuram/4e66ab23f0b0bf50e9265f68273d928b4e47a07c)
Combines hierarchical clustering with decision-tree interpretability. Each leaf represents a cluster, path represents a rule. Supports our goal of interpretable hierarchical prototypes where each split has semantic meaning.


## Tools & Libraries

### [TransformerLens](https://github.com/TransformerLensOrg/TransformerLens)
Library for mechanistic interpretability of GPT-style models. Exposes internal activations, supports caching and editing activations. Primary tool for implementing our thought discovery and activation patching.

### [nnsight](https://nnsight.net/)
Performant library for interpreting and manipulating model internals. Works well with larger models, wraps HuggingFace transformers. Alternative/complement to TransformerLens for larger-scale experiments.

### [Getting Started in Mechanistic Interpretability](https://transformerlensorg.github.io/TransformerLens/content/getting_started_mech_interp.html)
Tutorial and practical introduction to mech interp using TransformerLens. Includes exercises and solutions. Resource for implementation of our framework.


## Neural Network Ablation & Pruning

### [Ablation Studies in Artificial Neural Networks](https://arxiv.org/pdf/1901.08644)
Systematic approach to understanding knowledge representations through ablation. Tests network robustness to structural damage. Relevant to our Network Damage Strategy - comparing intact vs ablated networks to reveal safety pathways.

### [Hierarchical Safety Realignment](https://arxiv.org/html/2505.16104v1)
Identifies safety-critical attention heads and neurons in pruned models. Restores safety by realigning pruned safety-critical neurons. Demonstrates that safety can be localized and restored - supports feasibility of our approach to identifying dark/guard thoughts.

