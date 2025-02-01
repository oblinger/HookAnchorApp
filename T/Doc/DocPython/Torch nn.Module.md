#dl #review

Models

Criteria / Loss Functions
-?-
- nn.CrossEntropyLoss
- nn.CrossEntropyLoss, 
- nn.MSELoss - Mean Squared Error
- nn.BCELoss - Binary Cross Entropy

Optimizers - torch.optim
- .SGD(m, lr=, )
- Adam - Adaptive Moment Estimation (AdaGrad, RMS)

```python

from torch import nn

class Foo(nn.Module):
	def __init__(self, in_dim: int, out_dim: int):
		super().__init__()
        ...
    def forward(self, x):
	    return nn.sigmoid(self.linear(x))

```