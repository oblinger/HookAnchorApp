

#review 
[[Torch Tensor]] 

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