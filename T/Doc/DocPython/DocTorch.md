


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