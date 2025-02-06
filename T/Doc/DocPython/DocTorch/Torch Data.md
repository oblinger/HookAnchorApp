
```python
import sklearn
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from torch.utils.data import Dataset, DataLoader, TensorDataset
#
class Data(Dataset):
	def __init__(self, ...): ...
	def __getitem__(self, index): return x, y 
	def __len__(self): ...
data = Data(); data.x; data.y;
#
#

X_train, X_test, y_train, y_test = sklearn.model_selection.train_test_split(X, y, test_size=.2)

scalar = sklearn.preprocessing.StandardScaler()
X_train = scalar.fit_transform(X_train)
X_test = scalar.transform(X_test)

train_dataset, test_dataset = TensorDataset(X_train, y_train), TensorDataset(X_test, y_test)

train_loader = DataLoader(dataset=train_dataset, batch_size=10, shuffle=True)

for X,y in train_loader:

```


