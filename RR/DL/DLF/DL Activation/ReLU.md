
- ReLU - 
- GeLU -    [vs Relu](https://www.researchgate.net/figure/Comparison-of-the-ReLu-and-GeLu-activation-functions-ReLu-is-simpler-to-compute-but_fig3_370116538) 
- Leaky ReLU

#dl
#review

Torch Activation Functions
-?-
ReLU (simple), Softmax (multi-class), Sigmoid (squash for bin-class), Leaky ReLU, GeLU (modern xformers), Tahn (zero centered, from -1 to 1) <!--SR:!2025-03-23,2,235-->


ReLU 
-?-
Rectified Linear Unit = max(0,x) <!--SR:!2025-06-18,119,290-->

Softmax
-?-
Converts a vector of values into a probability distribution, typically used in multi-class classification. <!--SR:!2025-03-23,2,235-->


Sigmoid
-?-
S-curve from -INF to +INF that squashes input values to a range between 0 and 1, often used in binary classification.
.
sig(x) = 1 / (1 - e^-x ) <!--SR:!2025-03-26,5,237-->


Tanh
-?-
Similar to sigmoid but outputs between -1 and 1, useful for zero-centered data. <!--SR:!2025-03-23,2,235-->


GeLU
-?-
Gaussian Error Linear Unit. (like a smoothed ReLU)
Multiplies input by cumulative distribution function of a Gaussian distribution
(Tends to smooth out activation as compared to ReLU, more common in recent transformers)
Uses mean=0 std=1 assuming expected layer by layer regularization (e.g. batch normalization) <!--SR:!2025-03-22,1,215-->


Leaky ReLU
-?-
Leaky Rectified Linear Unit:  F(x) = x if x>0 else alpha*x <!--SR:!2025-04-22,81,270-->
