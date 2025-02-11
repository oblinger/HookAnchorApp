.[[Torch Module]].
  , [[Feed Forward Layer]], 
  [[Torch Model]]: 
  [[Convolution]] 



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


Batch Normalization
-?-
- nn.BatchNorm?d(num_nodes)    
- Reduces internal covariate shift across epochs and mini-batches.  
- scale and shift parameters retain model flexibility.
- No longer need dropout
- you can increase learning rate safely
- bias term is no longer needed
```python
class NetBatchNorm(nn.Module):
    
    # Constructor
    def __init__(self, in_size, n_hidden1, n_hidden2, out_size):
        super(NetBatchNorm, self).__init__()
        self.linear1 = nn.Linear(in_size, n_hidden1)
        self.linear2 = nn.Linear(n_hidden1, n_hidden2)
        self.linear3 = nn.Linear(n_hidden2, out_size)
        self.bn1 = nn.BatchNorm1d(n_hidden1)
        self.bn2 = nn.BatchNorm1d(n_hidden2)
        
    # Prediction
    def forward(self, x):
        x = self.bn1(torch.sigmoid(self.linear1(x)))
        x = self.bn2(torch.sigmoid(self.linear2(x)))
        x = self.linear3(x)
        return x
```
