
Torch Optimizers
-?-
```python
from torch import optim
opt = optim.SGD(model.parameters(), lr=0.01, momentum=0.9, weight_decay=, dampening=)     # Stocastic Gradient Descent

opt.step()   # update model params
opt.state_dict()

```


Torch SGD
-?-
Stocastic Gradient Descent
- nn.SGD(params=, lr=, momentum=, weight_decay=, dampening=)
	- momentum=0.9.  Like a ball's weight, it keeps velocity through flat regions
	- weight_decay=0.0001. is the L2 regularization penalty to prevent overfitting
	- dampening - reduces effect of momentum

