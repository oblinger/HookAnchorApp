n:: How to optimize a deep net

#review 

#dl 

[[DL Optim]] 


- **Mini-Batch Gradient Descent** - computes gradient and step based on "mini-batches" of the data
- **Gradient Descent** == mini-batch with n = whole dataset
- **Stocastic Gradient Descent** == mini-batch with n=1

## MINI-BATCH - Batch vs. Mini-Batch Gradient Descent
- "1 epoch" - single pass thru the whole dataset
- NOTATION:      
  m = total examples, size = mini-batch size 
  X = X^(1), ..., X^(m) = X^{1}, X^{m/size}
  Y = Y^(1), ..., Y^(m) = Y^{1}, Y^{m/size}
- Execute single step of backprop on successive mini-batches
- SIZE
  size = 1 :   Stochastic Gradient Descent  (loses advantage of vectorization)
  size = m:   Batch Gradient Descent
- CHOOSE
  m < 2K : Just use Batch gradient descent
  else :  typical mini-batch sizes:  64, 128, 256, 512,    a bit more rare: 1024   (verify that each mini-batch will fit into CPU/GPU memory)


## EXPONENTIAL MOVING AVERAGE - Exponentially Weighted (moving) Average
- Exponential Weighted Moving Averages
- V_i = V_i-1 * beta  +  theta_i * (1 - beta)
- This is kind of like averaging over the last 1 / (1 - beta) data points      (after that, weight is less than 1/e)

- BIAS CORRECTION - fixes fact that moving average begins with 0.0 from prior terms.
    V_t correction:   divide by (1-beta^t)


## MOMENTUM - Gradient descent with momentum
- Use weighted average of the last dW and db, rather then using dW and db directly.  This imparts momentum to them.
- Hyper parameters: alpha = learning rate, beta = dampening (often set as 0.9).   beta=0 turns off momentum ???
- Almost always will work better than w/o momentum
- V_dW = beta * V_dW + (1-beta) * dW
- W = W - alpha * V_dW     b = b - alpha * V_db


## RMS_propagation
- INTUITION:  As euclidean magnitude of the gradient decreases the chances of taking a step in a 'weird direction' also decreases, so one can step 
- INTUITION: when gradient in a direction is large, it is safer and still efficient to reduce learning rate in that dimension, thus /sqrt of avg of squared weights
- Compute the Exponentially Weighted Moving Average of dW and db:
  S_dW := beta * S_dW + (1-beta) * dW^2    (element wise squaring; do same for S_db)
- W := W - alpha * dW / sqrt( S_dW ) + epsilon        (epsilon=1e-8 added to ensure numerical stability in the face of )


## ADAM  =  MOMENTUM + RMS_prop  =  Adaptive Moment Estimation
- [ICLR PAPER](https://arxiv.org/pdf/1412.6980.pdf) - 
- Start with V_dW, SdW, V_db, S_db = 0
- V_dW = beta1 * V_dW + (1-beta1) * dW           (same for V_db; this is the "MOMENTUM" part)
- S_dW = beta2 * S_dW + (1-beta2) * dW^2     (same for S_db; this is the "RMS_prop" part)
- Vcorrected_dW = V_dW / (1-beta1^t)               (same for Vcorrected_db; this is the bias correction for momentum)
- Scorrected_dW = S_dW / (1-beta2^t)               (same for Scorrected_db; this is the bias correction for RMS_prop)
- W := W - alpha * Vcorrected_dW / (sqrt(Scorrected_dW) + epsilon))       (same for db)
- HYPER PARAMETERS
	- alpha needs to be tuned.
	- beta1: 0.9 seems to work well
	- beta2: 0.999 seems to work well (as suggested by the Adam)
	- epsilon:  1e-8 works well.  (alg is not very senstive to this parameter)


## LEARNING RATE DECAY
- alpha = 1 / (1 + decayRate * epochNumber) * alpha0         (Learning rate 'decays' towards zero slowly as epochNumber increases)
- alpha = 0.95 ^ epochNumber * alpha0                                (Exponentially decaying learning rate)
- alpha = k / sqrt(epochNum) * alpha0                                   (Can also use the mini-batch number t instead of epochNum)
- Stepwise decreased "staircased" learning rate
- Manual set decay over time




## HYPER PARAMETER OPTOMIZATION
STRATEGIES:
- RANDOM SAMPLING - Sample space of hyper parameters by selecting random combinations
- COARSE2FINE SAMPLING - First sample parameters coarsely over a large space, then select a subset performing well, for a denser search
- USE LOG SCALE - Many parameters make more sense to search on a log scale
  (choose log when sensitivity is exponential around some number often 0.0 or 1.0)


## BATCH NORMALIZATION - for intra layer Z or A activation values
- Normalizes the activations obtained 
- Applies a linear transform to the Z values in the layers of the net in order to control the mean and variance of the distribution of Z (activation related values) during training.

- Z_tilde^[l] = gamma^[l] * Z_norm^[l] + beta^[l]          (NOTICE:  gamma^[l] and beta^[l] are same shape at Z^[l].    So  (n_l, 1) are their shape)
  Z_norm^[l] (i) =   ( Z^[l] (i) - mean^[l] )   /   sqrt(var+epsilon)       mean^[l] = 1/m * sum Z^[l] (i)      var = 1/m * sum (Z^[l] (i) - mean^[l])^2
  Z^[l] = W^[l] * A^[l-1]     (notice: we deleted the db parameter since it will be cancelled by the Z_norm step)

- mean and var can be computed as a exponentially weighted moving average
- Can then use final mean and var at test time. 


- WHY DOES IT WORK?
	- NORMALIZES axes relative to each other
	- COVARIANT SHIFT - Reduces damaged caused by internal covariant shift in internal layers by keeping mean and variance more constant over time.
	- DECOUPLES PARAMETERS between layers
	- SLIGHT REGULARIZATION - adds mini-batch noise from noise in computed mean and variance.  (this effect is tiny, and kind of like dropout)

[[DL Optim]] 

[[RR/DL/DL strategy]] [[RR/DL/DL strategy]] 


Stocastic Gradient Descent
-?-
nn optimization, mini-batch-size == 1
.
optimizer = optim.SGD(model.parameters(), lr=0.01, weight_decay=0.01)
.
lr == Learning Rate (0.01)
weight_decay == L2 Regularization. Same as:
lambda_l2 = 0.01  # L2 regularization strength
l2_norm = sum(param.pow(2.0).sum() for param in model.parameters())
loss = criterion(outputs, targets) + lambda_l2 * l2_norm

blank line <!--SR:!2025-02-22,3,250-->

#dl 

