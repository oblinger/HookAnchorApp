[[DocTorch]] #dl #review 


Tensor;;a multi-dimensional array data structure optionally with a gradient for each value

**Tensor Rank** / Tensor Order - This is the dimensions of the num Py array need to store instances of this kind of tensor.


Tensor Field - a Function mapping onto some given type of tensor
tensor
-?-
## TENSOR
```python
# TENSOR
t = torch.tensor([[1],[2],[3]], require_grad=True, dtype=torch.float32)  
  torch.int16  torch.uint8
len(t.size())==t.ndimensions();  t.numel()==reduce(mul, t.size())   reshaped = t.view(1, -1)
t[0][0]; three_values = 
t[[0,1,2]] 
t[0,0].item(); t.tolist();  t.shape==t.size();  str(t.dtype)==t.type();   new_type=t.type(torch.uint16)
np_arr = torch.from_numpy(np_arr) .numpy()
t = torch.from_numpy( pd.Series([.1, .2, .3]) .values 
t[0] = t[1]

torch.linspace(-2, 2, num=9)
torch.arange(-3, 3, 0.1)
torch.randn(size)
torch.zeros(size)

np.meshgrid(np.arange(), ...)

```
<!--SR:!2025-02-02,1,210-->

tensor math
-?-
```python
t + u - v + 2*t + 
element_wise = t+1 + 2*t + u*v;  torch.sin(t); 
dot = torch.dot(u, v)    prod = torch.mm(A, B)
result_tensor = t.mean()  t.std()  t.max() 
```

tensor grad
-?-
```python
import torch

x = torch.tensor([1, 2, 3], requires_grad = True)
z = x**2 + 2*x + 1
zz = torch.dot(z, z)
zz.backward()
x.grad
```


## Foo
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
[[null]] 

```



