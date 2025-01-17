

#review #dl
[[Torch Tensor]] 
[[Torch Template]] 

```python

import torch

xform = transforms.Compose([transforms.CenterCrop(20), transforms.ToTensor()])
dataset = dsets.MNIST(root = './data', download = True, transform = xform)

from torch.nn import Linear

class Data(Dataset):
	def __getitem__(self, idx): ...
	def __len__(self): ...

for x,y in DataLoader(dataset = Data(), batch_size = 1):

four chairs, zazie


```


```nn
from torch.nn import Linear, Module

model = Linear(2,1)  # in_features,out_features

class MyMod(nn.Module):
  def __init__, forward, 
```


## Basics

```
z = torch.arange(-100, 100, 0.1).view(-1, 1). # builds an array of values
z = torch.tensor([[1.0]])
```


## Model / Module

```
nn.Sigmoid(), 
nn.Linear(in_dim, out_dim)
nn.Sequential(nn.Linear(2, 1), nn.Sigmoid()), 

class nn.Module(): __init__(self, inputs), forward(self, x)
```



## Visualization

```

plt.plot(z.numpy(), yhat.numpy())
plt.xlabel('z')
plt.ylabel('yhat')

```





