
```python
from torch.utils.data import Dataset, DataLoader
#
class Data(Dataset):
	def __init__(self, ...): ...
	def __getitem__(self, index): return x, y 
	def __len__(self): ...
data = Data(); data.x; data.y;
#
#
for X,y in DataLoader(dataset=Data(), batch_size=10):

```


