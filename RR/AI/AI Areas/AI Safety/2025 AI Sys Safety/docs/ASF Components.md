# ASF Components

## Libraries & Tools

### TLens ^tlens
**TransformerLens**
https://github.com/TransformerLensOrg/TransformerLens

Library for mechanistic interpretability. Exposes internal activations, supports caching, hooks, and activation editing. Primary tool for accessing model internals.

### NNSight ^nnsight
**nnsight**
https://nnsight.net/

Alternative to TransformerLens. Better performance on larger models, wraps HuggingFace directly. Use for models >9B parameters.

### SAELens ^saelens
**SAELens**
https://github.com/jbloomAus/SAELens

Library for training and using Sparse Autoencoders. Pre-trained SAEs available for some models. Useful for feature discovery in thought representation.

### PyTorch ^pytorch
**PyTorch**
https://pytorch.org/

Core deep learning framework. Used for all custom model components, latent space training, prototype learning.


## Pre-trained Models

### Llama3 ^llama3
**Llama 3 8B Instruct**
https://huggingface.co/meta-llama/Meta-Llama-3-8B-Instruct

Open-weights safety-aligned model. Good starting point - small enough for interpretability work, has safety training to study.

### Gemma ^gemma
**Gemma 2 9B**
https://huggingface.co/google/gemma-2-9b-it

Google's open model with safety alignment. Alternative to Llama for validation across model families.

### Mistral ^mistral
**Mistral 7B Instruct**
https://huggingface.co/mistralai/Mistral-7B-Instruct-v0.3

Another safety-aligned open model. Less restrictive than Llama, useful for studying weaker guardrails.


## Datasets

### HarmBench ^harmbench
**HarmBench**
https://github.com/centerforaisafety/HarmBench

Standardized benchmark for evaluating LLM harmfulness. Contains harmful prompts and evaluation framework. Source for seeding dark output categories.

### AdvBench ^advbench
**AdvBench**
https://github.com/llm-attacks/llm-attacks

Adversarial prompts dataset from "Universal and Transferable Adversarial Attacks" paper. Additional source for dark outputs.

### Anthropic-HH ^anthropic-hh
**Anthropic HH-RLHF**
https://huggingface.co/datasets/Anthropic/hh-rlhf

Human preference data with helpful and harmless examples. Contains refusal examples useful for seeding guard output categories.

### WildChat ^wildchat
**WildChat**
https://huggingface.co/datasets/allenai/WildChat

Real user conversations with ChatGPT including safety interventions. Natural distribution of guard outputs in context.


## Algorithms & Techniques

### ActPatch ^actpatch
**Activation Patching**
Causal intervention technique. Replace activations from one forward pass into another. Core method for validating discovered thoughts.

Reference: [How to Use and Interpret Activation Patching](https://arxiv.org/html/2404.15255v1)

### AttrPatch ^attrpatch
**Attribution Patching**
Gradient-based approximation to activation patching. Faster than full patching, good for initial exploration.

Reference: Neel Nanda's [Attribution Patching](https://www.neelnanda.io/mechanistic-interpretability/attribution-patching)

### CAA ^caa
**Contrastive Activation Addition**
Compute steering vectors from contrastive pairs. Add/subtract to control model behavior.

Reference: [Steering Llama 2 via Contrastive Activation Addition](https://aclanthology.org/2024/acl-long.828.pdf)

### ProtoNet ^protonet
**Prototypical Networks**
Few-shot classification via distance to learned prototypes. Adapt for thought/output categorization.

Reference: [Prototypical Networks for Few-shot Learning](https://arxiv.org/abs/1703.05175)

### InfoNCE ^infonce
**InfoNCE Loss**
Contrastive loss function for representation learning. Use for training latent space encoder.

Reference: Standard contrastive learning literature


## Pre-trained Artifacts

### AnthropicSAE ^anthropic-sae
**Anthropic's Claude SAE Features**
Pre-trained SAE features from Claude 3 Sonnet. Includes safety-relevant features (deception, bias, dangerous content).

Reference: [Scaling Monosemanticity](https://transformer-circuits.pub/2024/scaling-monosemanticity/)

### RefusalDir ^refusal-dir
**Refusal Direction Vectors**
Pre-computed refusal directions for various models. Can be used directly or as validation for discovered guard thoughts.

Reference: [Refusal in LLMs is Mediated by a Single Direction](https://arxiv.org/abs/2406.11717)


## Compute & Infrastructure

### Colab ^colab
**Google Colab**
Free GPU access (T4/V100). Sufficient for small model experiments (<7B with quantization).

### Lambda ^lambda
**Lambda Labs / RunPod**
On-demand GPU rental. A100/H100 for larger experiments. Pay-per-hour for expensive runs.

### WandB ^wandb
**Weights & Biases**
Experiment tracking, visualization, hyperparameter sweeps. Use for all training runs.

