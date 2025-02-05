.[[Torch Module]].
  , [[Feed Forward Layer]], 



#dl #review
- [[Torch Optim]] 
- [[Torch Loss]] 

Models

Optimizers - torch.optim
- .SGD(m, lr=, )
- Adam - Adaptive Moment Estimation (AdaGrad, RMS)

```python

from torch import nn

class Model(nn.Module):
	def __init__(self, in_dim: int, out_dim: int):
		super().__init__()
        ...
    def forward(self, x):
	    return nn.sigmoid(self.linear(x))

model.train()
model.eval()

model = Model(1, 1)
model.state_dict()
list(model.parameters())

mods = nn.ModuleList(); mods.append(...)
```



Common FOUNDATIONAL Torch Modules
-?-
nn.Linear(in,out,bias=True)
nn.Conv2d
nn.BatchNorm3d


Common Activation/Pooling/Regularization Torch Modules
-?-
nn.ReLU, nn.Tanh, nn.Sigmoid
nn.MaxPool2d, nn.AvgPoold2, 
nn.Dropout, nn.Dropout2D, ...


Common Recurrent/Container Torch Modules
-?-
nn.RNN, nn.LTSM, nn.GRU
nn.Sequential, nn.ModuleList
nn.MultiheadAttention

