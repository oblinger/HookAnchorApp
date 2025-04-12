

(see [[Coursera Deep Learning]] Chapter 3)

#card

DL considerations
-?-
- MODEL:
	- Structure, ActivationFn, LossFn, Dropout, Regularization, 
	- Momentum, 
- OPTIMIZER:
- SCHEDULER:

# Tricks To Play

### Gradient Accumulation 

DL strategy: Gradient accumulation
-?-
A technique that enables training with smaller batch sizes by accumulating gradients over multiple steps before applying them.
This is particularly useful when memory limitations prevent large batch sizes from being used.

Here’s how it works: 
Memory Efficiency: By accumulating gradients across several mini-batches, gradient accumulation reduces memory load, allowing models to simulate the effect of a larger batch size. 

Performance Gains: Accumulating gradients without frequent updates can stabilize training, improving model performance, especially on complex datasets. }}
__


### Mixed-Precision Training 
DL strategy: Mixed-precision training
-?-
Uses lower precision for specific calculations during training, which helps reduce memory consumption and accelerates computation without substantially affecting accuracy. 
.
Half-Precision Calculation: Calculations in half-precision (FP16) significantly reduce memory usage and speed up training times, especially effective for large models with high computational demands. 
.
Automatic Mixed Precision (AMP): Frameworks like PyTorch and TensorFlow support AMP, which automatically selects the optimal precision for each operation. 

 

### Distributed Training 
DL Strategy: Distributed Training
-?-
For training large transformers, distributed training enables the use of multiple GPUs or TPUs in parallel, reducing the overall training time. <!--SR:!2025-03-08,1,230-->  

There are two main strategies: 

Data Parallelism: Each device receives a portion of the data batch, processes it, and then synchronizes gradients across all devices.  

  This method is popular for tasks where models do not need to be split across devices. 

Model Parallelism: When a model is too large to fit on a single device, model parallelism splits the model itself across multiple devices, enabling larger models to be trained. 

 

### Efficient Optimizers 
DL Strategy: Efficient {{Optimizers}}
-?-
Selecting the right optimizer can make a difference in both speed and performance. AdamW and LAMB are popular choices for transformer training. {{AdamW}}. 
{{LAMB - Layer-wise Adaptive Moments}}

AdamW (Adam with Weight Decay): Combines Adam’s adaptive learning with weight decay, which helps in generalizing better during training. 

LAMB (Layer-wise Adaptive Moments): Used especially in large-batch training, LAMB can scale the training process, improving efficiency on extensive datasets. 

 