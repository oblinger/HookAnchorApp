

## Deep Learning

## NOTATION

- a^[l]	Superscript-Bracket denotes the *l*-th layer
- x^(*i*)	Superscript-Paren denotes the *i*-th input example
- a_*i*		Subscript denotes the *i*-th vector element.  e. g. the *i*-th activation value
- n_H 	Subscript _H _W _C denotes the Height, Width, and num Channels for  a layer

- np		Array shape:  (example_i, axis0, axis1, ..., channel_j)

### DL2
- [[Regularization]] 


## COURSE 3

(see [[RR/DL/DL strategy]])

### --- ML improvement strategy ---
- General Strategy:
	1. **Reduce removable bias**.  Fit TRAINING set well on cost function.  
	   ==> use bigger network, use RMSprop, use Adam
	   ==> Try different NN architecture
	3. **Reduce variance**.  Fit DEV set well on cost function.  
	   ==> use regularization (L2, Dropout, data augmentation), or 
	   ==> use hyper parameter search to find better parameters
	   ==> bigger training set
	4. **Fix Dev Set**.  Fit TEST set well on cost function.   (use better dev set)
	5. **Fix Test Set or Cost Fn**.  Performs well in real word.  (change test set or cost function)


### Orthogonalization 
- Try to optimize parts of the ML solution independently
- ML improvement strategy is an example of orthogonalization


### Metrics Considerations
#### Single Number Evaluation Metric
#### Satisficing and Optimizing metrics
- Need to have n-1 satisfying metrics and only ONE optimizing metrics.
- Then one can first satisfy all satisficing metrics then work on optimizing one.

### Data Set Considerations
- Ensure dev set is a RANDOM sample when possible.
- Dev & Test set needs only be big enough to detect differences in algs being tested.  (might only be 1% of data)

### Error Analysis
- Look @ dev dataset examples to get and evaluation ideas
- Evaluate multiple ideas in parallel.  Fix miss labelling as Dog, or GreatCats, or Blurry Images
- Build system quickly, use rough performance to iteratively focus attention in improving performance

### Mismatched Test and Training

ANALYSIS OF ERRORS
- AVOIDABLE BIAS:  	Delta between   	Human-level <--> Training Set Error
- VARIANCE:			Delta between 	Training Set Error  <-->  Training-Dev Set Error
- DATA MISMATCH:	Delta between	Training-Dev Set Error  <-->  Dev or Test Set Error (or performance in the wild)

TRAINING SET:			Used for training.
TRAINING-DEV set:  	Randomly selected subset of training data to exclude from training to use as a special test (when training and dev sets are different)
DEV SET:						Used to optimize hyper parameter tuning and algo iteration.
TEST SET:					Use to assess overall performance as expected in the real world.


### REMEDIATIONS

#### Artificial Data Synthesis
- Injecting car noise

#### Transfer Learning
- When related task has MUCH more data

#### Multi-Task Learning
- Amounts of data on each task is similar (usually)
- Expect sub-concepts will transfer

#### End-to-End Learning
- Use it when you have tons of end to end training data


## COURSE 4 - CNNs

### CNN NOTATION:

- m = the number of training instances.
- *n*^[*l*] = The **Length** or **Width** of the input matrix in convolution layer *l*.
- *f*^[*l*] = The **Filter** size in convolution layer *l*.  how many cells in row/col to merge.
- *p*^[*l*] = The **padding** in convolution layer *l*. how many padding cells on each edge.
- *s*^[*l*] = The **stride** of the convolution layer *l*.  how many cells to jump in each step.
- layer_i = only count layers with weights (not pooling layer etc.)


#### Standard CNN Structure

- Conv - ... - Pooling - { Conv - ... - Pooling }* - FC - FC* - SoftMax


#### CNN Intuition
- PARAM SHARING - CNNs share parameters in the conv layers
- SPARSE PARAMS - Conv layers are local - each output depends upon small num
- Input structure is translation invariant


### Convolution Layer
- input n x n x NUM_CHAN  --->  n_shrink x n_shrink x NUM_KERNELS

#### Padding
- Valid = No padding
- Same convolution = enough padding to avoid any shrinkage.   p =  (f-1)/2

#### Strided Convolution
 - Skipping more than one position when computing the convolution.
 - output = floor( n+2p-f / s+1 )    n=input, p=padding, f=filter, s=stride

#### 3D Convolution
- Input:  height, width, channels.   
  Filter: height, width, channels.   
  Output:  height, width, 1.   (with potential shrinking)
 
#### NOTE: Mathematicians call convolution the cross-coorelation operator 
- The convolution operator is first mirrored on both horizontal and vertical.
- By first inverting the filter it makes the convolution operator associative:  
        A * (B * C) = (A * B) * C



### Pooling Layer
- Max Pooling - used most of the time
	- f = filter size (number of col/row that are merged)
	- s = stride (the jump between each computed cell)
- Average pooling - computes average instead of max.
  Occasionally used to entirely remove spatial info, so output is  1 x 1 x XXX
- Very rarely use padding; no parameters, just hyper parameters


### Fully Connected Layer
- inputs n units and outputs m units with m * n weights


### Soft Max Layer 
- Inputs n units and outputs m soft-max outputs





### OTHER IDEAS
#### Intersection Over Union
- A measure of the accuracy of a bounding box:
  Divide the intersection area by the union area.
  0% to 100%