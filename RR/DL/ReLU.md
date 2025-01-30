
- ReLU - 
- GeLU -    [vs Relu](https://www.researchgate.net/figure/Comparison-of-the-ReLu-and-GeLu-activation-functions-ReLu-is-simpler-to-compute-but_fig3_370116538) 
- Leaky ReLU

#dl
#review

Common Activation Functions
-?-
ReLU (simple), Softmax (multi-class), Sigmoid (squash for bin-class), Leaky ReLU, GeLU (modern xformers), Tahn (zero centered, from -1 to 1)


ReLU 
-?-
Rectified Linear Unit = max(0,x)
<!--SR:!2025-02-18,41,290-->

Softmax
-?-
Converts a vector of values into a probability distribution, typically used in multi-class classification.


Sigmoid
-?-
Squashes input values to a range between 0 and 1, often used in binary classification.


Tanh
-?-
Similar to sigmoid but outputs between -1 and 1, useful for zero-centered data.


GeLU
-?-
Gaussian Error Linear Unit
Multiplies input by cumulative distribution function of a Gaussian distribution
(Tends to smooth out activation as compared to ReLU, more common in recent transformers)
Uses mean=0 std=1 assuming expected layer by layer regularization (e.g. batch normalization)


Leaky ReLU
-?-
Leaky Rectified Linear Unit:  F(x) = x if x>0 else alpha*x
<!--SR:!2025-01-09,30,270-->
