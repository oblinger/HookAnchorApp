BAC


## L2 REGULARIZATION
L2 Regularization in a NN = "Weight Decay" Regularization
- Adds an additional 'cost' term to the measured cost of any given network's parameters...  the sum of the square of those parameters.
- When derivative of this cost is taken it has the effect of, At each step 'decaying' all weights towards 0 in addition to back prop signal


## DROPOUT REGULARIZATION
- Certain probability to "drop out" nodes randomly on each iteration of the back prop
- ONLY USE DURING TRAINING - 
  
- INVERTED DROPOUT - 
	- keep_prob determines which params
	- then scale up by dividing by 'keep_prob'   <--- This scaling is the 'INVERTED' part
	  (scaling helps at test time to keep expected value is the same)
  - At testing time we DONT use the dropout
  - Can't count on monotonic reduction in cost function.  (trick:  turn off dropout to verify monotonic decrease, then turn on for real run)


## DATA AUGMENTATION REGULARIZATION - increasing training set size 'for free' to reduce overfit
- Flip image left to right to double training data
- Or do random crop (rotate & scale)


## EARLY STOPPING 
- Plot devset error AND your training error.
- Stop training when dev-set error levels off (even as training error is dropping)
- Downside:  Creates bias-variance tradeoff  (when to stop)
	- kind of like L2 regularization w. different values of lambda (that forces W parameters to be small)




# NORMALIZING YOUR INPUTS

- Subtract out the mean in the input training & test features
- Divide by the variance


- IMPT:  Use the SAME mean and variance to normalize training and test data by the SAME factors!