

#stat

Logit
-?-
A logit is the raw, unnormalized output of a model before it is transformed into probabilities[](https://telnyx.com/learn-ai/logits-ai)[](https://www.linkedin.com/pulse/understanding-logits-fundamental-concept-machine-aina-temiloluwa-7d3ff). 
.
Specifically:
1. In neural networks, logits are the values produced by the last layer before applying an activation function like sigmoid or softmax[](https://telnyx.com/learn-ai/logits-ai)[](https://telnyx.com/learn-ai/logits-confidence).
2. Mathematically, the logit function is defined as:
    logit(p)=log(p/(1-p)).  where p is a probability between 0 and 1[](https://en.wikipedia.org/wiki/Logit).
3. Logits map probability values from (0,1) to the real number range (-∞, +∞)[](https://en.wikipedia.org/wiki/Logit)[](https://deepai.org/machine-learning-glossary-and-terms/logit).
4. In binary classification, logits are typically transformed into probabilities using the sigmoid function, while in multi-class classification, the softmax function is used[](https://www.linkedin.com/pulse/understanding-logits-fundamental-concept-machine-aina-temiloluwa-7d3ff)[](https://telnyx.com/learn-ai/logits-confidence).
5. Logits play a crucial role in various machine learning algorithms, including logistic regression and neural networks, serving as the basis for making predictions and calculating loss functions during model training[](https://telnyx.com/learn-ai/logits-ai)[](https://www.linkedin.com/pulse/understanding-logits-fundamental-concept-machine-aina-temiloluwa-7d3ff).
6. They provide a clear separation between the modeling and decision-making processes, allowing for greater flexibility in model design and evaluation[](https://www.linkedin.com/pulse/understanding-logits-fundamental-concept-machine-aina-temiloluwa-7d3ff).
Understanding logits is essential for machine learning practitioners as they form the foundation for interpreting and working with model outputs in classification tasks.