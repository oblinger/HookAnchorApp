#review 


# CONSIDER

- Learning Rate.  Learning Rate Scheduler
- Feature Scaling
DATA
- Feature Scaling
- Output Normalization
- Class balancing
TESTING
- Train-validation split
- Early Stopping
STABILAIZATION
- Gradient clipping
VISUALIZATION
- Monitor loss
- Visualize


Set an Appropriate Learning Rate

The learning rate controls how quickly a model updates its parameters, and it plays a crucial role in achieving optimal performance.

Moderate learning rate: Start with a moderate learning rate (e.g., 0.01) to balance the speed of learning with model stability. If the rate is too high, the model may oscillate around the optimal solution, while a rate that’s too low may slow down convergence.

Learning rate schedulers: Use learning rate schedulers like PyTorch's StepLR or ReduceLROnPlateau to adjust the learning rate dynamically during training. This helps to fine-tune the model towards the end of training and avoid getting stuck in local minima.

Data Standardization

Standardizing input features is essential for improving the performance of linear regression models.

Feature scaling: Standardize input features so they have zero mean and unit variance. This prevents features with larger scales from dominating the learning process. Use PyTorch’s torch.mean and torch.std functions or external libraries like scikit-learn for standardization.

Output normalization: If the target output is on a large scale, normalizing it (e.g., dividing by a constant or applying a transformation) may stabilize training and improve convergence.

Implement Validation Sets

A validation set helps you assess the model's performance on unseen data and prevent overfitting.

Train-validation split: Split your dataset into training and validation sets (e.g., 80/20 split). Evaluate the model's performance on the validation set periodically to ensure generalization.

Early stopping: Introduce early stopping to halt training once validation performance stops improving. This prevents overfitting and ensures the model doesn't overtrain on the training set.

Gradient Clipping for Stability

Large gradients can cause instability, especially in datasets with high variance or noisy features.

Clip gradients: Use PyTorch’s torch.nn.utils.clip_grad_norm_ to limit gradient magnitudes and avoid exploding gradients. This ensures that gradients remain within a reasonable range during backpropagation.

Stabilize training: Gradient clipping is especially important when dealing with more complex data or noisy features, as it keeps training stable.

Monitor Loss Function

Closely tracking the loss function during training helps you spot problems early and adjust accordingly.

Loss monitoring: Ensure that the loss decreases steadily as training progresses. If the loss plateaus or increases, it could signal an inappropriate learning rate or poorly prepared data.

Use visualization tools: Tools like TensorBoard or Matplotlib can help visualize loss trends over time. Monitoring both training and validation loss helps detect issues like overfitting or underfitting.



# STUFF

#### Learning Rate
- too small - learning is too slow
- too large - 

# STRATEGIES


## OVERALL STRATEGY
1. DATA - Gather data for training, dev, and test
2. QUICKLY - Quickly build first system
3. ITERATE - Improvement

EXCEPTION: If problem is exactly matched by prior experience, or body of academic work


## ORTHOGONALIZATION
- Organize your hyper parameters so the each one affect only one aspect of ML performance

### (1) TRAINING SET FIT - (If Humans are better than training error THEN: Lower/Change bias)
- Train bigger model
- Train longer or use better optimization algs:  momentum, RSMprop, Adam
- NN architecture/hyperparameter search:  (RNN, CNN)

### (2) DEV SET FIT - (IF dev error is worse than training error THEN: Lower variance)
- Get bigger training set
- Regularization: L2/Dropout regularization
- NN architecture/hyper-parameter search: 

(3) TEST SET FIT - (Fix: Bigger dev set)
- Get bigger dev set

(4) PERFORMS IN REAL WORLD - (Fix: make test more like the real world)
- Change dev set or cost function


## METRIC CHOICE

### PICK >ONE< METRIC 
- A define a single numeric "goodness" measure for optimization

IDEAS:
- For example: Combine precision and recall
- [[HarmonicMean]] - F_1 score = 2 / (1/p + 1/r) = "Harmonic Mean"
- Average over multiple sub-segments to derive a consensus or mean performance

### SATISFYING METRICS
- When possible, convert optimizing metrics To satisficing metrics 
- Try to select satisficing thresholds for as many metrics as possible 

### CHANGING YOUR METRIC
- When alg is executing in an undesirable way but scoring well on your metric


## DATASET CHOICES
- SAME DISTRIBUTION  -  Ensure Dev and Test sets from the SAME distribution
- DATASET SIZES - Set Dev and Test size based on the confidence required in the output.  (can be small percent as sizes grow)



## COMPARING TO HUMAN PERFORMANCE

- DATA LABELLING - Use humans to label data, and compare to alg predictions
- ERROR DIAGNOSIS - Use humans to look for patterns in prediction errors, or understand 'why' alg had a particular error
- BIAS / VARIANCE ANALYSIS - Use humans to estimate avoidable bias and variance

### STRATEGY: Reduce AVOIDABLE bias
- IF training error is worse than human performance THEN reduce bias.    (Alg as too much or bad bias)

### STRATEGY:  Reduce VARIANCE
- IF training error is matched and dev error is lower than train error THEN reduce variance


## ERROR ANALYSIS

### INSTANCE SURVEY
1. Select 100 errors at random
2. Examine instances manually looking for patterns
3. Build table with rows as instances and columns as potential pattern attribute.  
4. Fill each cell has check if row has attribute
5. During this process you may find and add new pattern attribute columns to the table
6. Compute "ceiling" for each pattern class as percent of errors they account for

Use this ceiling as inspiration for which classes of errors to try to attack.

### MISLABELLED TRAINING, DEV & TEST SETS
- Not an issue if errors are random
- Can add "incorrectly labelled" category to error analysis and can exclude them from error rate (w/o fixing dev set label)
	- Overall dev set error.  Percent errors from mis-labelling, vs percent errors from other causes
		- Fix dev set if mislabelling errors becomes larger double digit fraction
		- May just fix the error in dev & test (and not training set) and maybe not the correctly labelled examples.

==> HANDS DIRTY - Need to look at examples of errors by hand, to get feel needed to prioritize actions


## DEALING WITH TRAINING DATA FROM DIFFERENT DISTRIBUTIONS
- USE "Gold truth" data in dev and test first.  only after in the training set
- Use related but different data for training if it will give much larger dataset
- USE "TRAINING DEVSET" - Take portion of training set distribution and create second "training dev" set.
  ==> Use this to differentiate if train-dev mismatch is from variance-problem or distribution mismatch problem


| EXAMPLES        | Variance | Data Mismatch | Bias | Bias&Data |
| --------------- | -------- | ------------- | ---- | --------- |
| Human Error     | 1%       | 1%            | 0%   | 0%        |
| Train Error     | 9%       | 1.5%          | 10%  | 10%       |
| Train-dev error | 10%      | 10%           | 11%  | 11%       |
| Dev error       |          |               |      | 20%       |
|                 |          |               |      |           |

Where are the big jumps in errors?
- HUMAN   <-avoid-bias->   TRAIN   <-Variance->   TRAIN-DEV   <-DataMismatch->   DEV   <-DevSetOverfit->   TEST



MORE GENERAL FORMULATION

|                            |                |            |            |
| -------------------------- | -------------- | ---------- | ---------- |
| Human Level                | "Human level"  |            | XXXXX      |
|                            | -bais-gap-     |            |            |
| Errors on trained examples | "Train"        |            | XXXXXX     |
|                            | -variance-gap- |            |            |
| Errors on unseen examples  | "Train-Dev"    | -data-gap- | "Dev-Test" |
|                            |                |            |            |
|                            |                |            |            |

### FIXING TRAIN-TEST DISTRIBUTION GAPS
- MANUAL - Do manual error analysis to understand and fix source of error
- SYNTHETIC DEGREDATION - If dev set is damaged (car noise), try artificially doing same to training set


### TRANSFER LEARNING
- WHEN source&target tasks have same input X AND
- WHEN target task has few training examples, but source task has many, AND
- WHEN you expect low level features from source task could help on target task

1. PRE-TRAINING
   Training deep net on source tasks
2. LAST LAYER SURGERY
  Delete last layer, replace with randomly weighted layer.  (or multiple layers)
3. FINE-TUNING
   Just tune weights on last layer (if second task has little data)
4. TRANSFER
   Tune weights of all layers (if second task has lots of data)


### MULTITASK LEARNING - learning from many tasks at once