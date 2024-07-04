- A time series is auto regressive iff subsequent values can be expressed as a stationary error term added to some prior term.

AR 1:    a(t) = phi * a(t-1) * error(t)


Autoregressive models generate data one element at a time, conditioning the generation of each element on previously generated elements. These models predict the probability distribution of the next element given the context of the previous elements and then sample from that distribution to generate new data. Popular examples of autoregressive models include language models like [GPT (Generative Pre-trained Transformer)](https://insights2techinfo.com/generative-pre-trained-transformer/), which can generate coherent and contextually appropriate text.