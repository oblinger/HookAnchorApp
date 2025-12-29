# ASF Files

## /
- `requirements.txt` - Python dependencies
- `README.md` - Project overview and setup instructions

## src/utils/
- `activation.py` - Activation caching, patching, injection helpers
- `gradient.py` - Gradient attribution utilities
- `embedding.py` - Embedding extraction and pooling

## src/asf/core/
- `thought.py` - Thought class (sparse neuron activation patterns)
- `output.py` - Output class (embedding sequences with category labels)
- `prototype.py` - Prototype class (category centers in latent space)
- `latent_space.py` - LatentSpace encoder

## src/asf/algorithms/
- `discovery.py` - Back-chain thought/output discovery
- `transitions.py` - Forward transition probability estimation
- `bifurcation.py` - Entropy-based prototype splitting

## src/asf/training/
- `losses.py` - Transition entropy loss, contrastive loss
- `encoder.py` - Latent space encoder training

## src/asf/pipeline/
- `safety_model.py` - Iterative safety model construction orchestrator

## notebooks/
- `01_validate_activation_access.ipynb` - Test model loading and activation hooks
- `02_explore_datasets.ipynb` - Explore HarmBench, Anthropic-HH data
- `03_thought_discovery.ipynb` - Initial thought discovery experiments

## scripts/
- `download_models.py` - Download models from HuggingFace
- `download_datasets.py` - Download and prepare datasets
- `run_discovery.py` - Run thought discovery pipeline
- `run_training.py` - Train latent space encoder

## configs/
- `model_configs.yaml` - Model loading configurations
- `training_configs.yaml` - Hyperparameters for latent space training

## tests/
- `test_thought.py`
- `test_prototype.py`
- `test_transitions.py`

## models/
- `llama3-8b-instruct/` - Llama 3 8B Instruct weights
- `mistral-7b-instruct/` - Mistral 7B Instruct weights

## datasets/

### datasets/harmbench/
- `data/behavior_datasets/`
	- `harmbench_behaviors_text_all.csv` - All text behaviors
	- `harmbench_behaviors_text_test.csv` - Test split
	- `harmbench_behaviors_text_val.csv` - Validation split
	- `harmbench_behaviors_multimodal_all.csv` - Multimodal behaviors
	- `extra_behavior_datasets/advbench_behaviors.csv` - AdvBench prompts
	- `extra_behavior_datasets/adv_training_behaviors.csv` - Adversarial training set
- `data/optimizer_targets/`
	- `harmbench_targets_text.json` - Target completions for text
	- `harmbench_targets_multimodal.json` - Target completions for multimodal
	- `extra_targets/advbench_targets.json` - AdvBench targets
- `data/classifier_val_sets/`
	- `text_behaviors_val_set.json` - Classifier validation (text)
	- `multimodal_behaviors_val_set.json` - Classifier validation (multimodal)
- `baselines/gptfuzz/GPTFuzzer.csv` - GPTFuzz jailbreak prompts

### datasets/anthropic-hh/
- `train/data-00000-of-00001.arrow` - Training split (helpful/harmless pairs)
- `test/data-00000-of-00001.arrow` - Test split

### datasets/wildchat-sample/
- `data-00000-of-00001.arrow` - 10k conversation sample

## outputs/
- `checkpoints/` - Saved model checkpoints
- `logs/` - Training logs
- `results/` - Experiment results and visualizations
