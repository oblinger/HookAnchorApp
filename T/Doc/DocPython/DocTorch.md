


```python

import torch
t = [[11.0, 12.0, 13.0], [21.0, 22.0, 23.0]]
a = torch.tensor(t, requires_grad=True) # rows together
a.shape == a.size() == torch.Size(2, 3)
a.dtype == torch.int64
a.numel() == 6
a.view(3, 2) # Reshape
a[row, col] // a [row:row, col]

a = torch.tensor([1,2,3])
b = a.type(torch.TensorFloat)
b.type()
b.ndimension() == 1
b.size() == torch.Size([3])  # indicies from outside to inside
a_col = a.view(3,1) or a.view(-1, 1)
c = torch.from_numpy(torch.numpy(a)) # done by reference!
j = torch.from_numpy(pandas_series.values)
lst = a.tolist()
a_number = a[0].item()
a[0] = 55
d = a[1:3]
a[1:3] = [44, 55, 66]

e = a+b
f = a * b.  # entry wise
j = torch.mm(a, b) # matrix multiply
g = torch.dot(a, b)
h = torch.mean(a) max(a) sin(a) # h is a tensor

x = torch.linspace(0, 2.*np.py, 100)
y = torch.sin(x)
import matplotlib.pyplot as plt
%matplotlib inline
plt.plot(x.numpy(), y.numpy())

xform = transforms.Compose([transforms.CenterCrop(20), transforms.ToTensor()])
dataset = dsets.MNIST(root = './data', download = True, transform = xform)

from torch.nn import Linear

class Data(Dataset):
	def __getitem__(self, idx): ...
	def __len__(self): ...

for x,y in DataLoader(dataset = Data(), batch_size = 1):

four chairs, zazie


```

## TENSOR
```
# TENSOR
t = torch.tensor([[1],[2]], require_grad=True)
t.item(); 
```

```nn
from torch.nn import Linear, Module

model = Linear(2,1)  # in_features,out_features

class MyMod(nn.Module):
  def __init__, forward, 
```

#review 

basic torch train flow
??
class Data, 
setup: (1) MyDataset, (2) criterion, (3) model, (4) loader, (5) optimizer
loop:  epoch; loop loader; yhat->loss->.zero->.backward->.step


```
# torch training
criterion = nn.MSELoss()
class MyDS(Dataset): def __init__, forward
ldr = DataLoader(dataset = data_set, batch_size = 10)
model1 = nn.Linear(2, 1)
optimizer = optim.SGD(model1.parameters(), lr = 0.1)
for epoch in range(epochs):
	for x,y in train_loader:
		yhat = model1(x)
		loss = criterion(yhat,y)
		LOSS1.append(loss.item())
		optimizer.zero_grad()
		loss.backward()
		optimizer.step()     
train_model(epochs)
Plot_2D_Plane(model1 , data_set)  
plt.plot(LOSS1)
plt.xlabel("iterations ")
plt.ylabel("Cost/total loss ")
```