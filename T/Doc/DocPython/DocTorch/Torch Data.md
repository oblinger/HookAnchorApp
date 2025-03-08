
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


### DataLoader - Delivers sequence of processed batches
DataLoader
-?-
Delivers sequence of processed batches
- collate_fn 

```python
BATCH_SIZE = 64
train_dataloader = DataLoader(
    split_train_, batch_size=BATCH_SIZE, shuffle=True, collate_fn=collate_batch
)
valid_dataloader = DataLoader(
    split_valid_, batch_size=BATCH_SIZE, shuffle=True, collate_fn=collate_batch
)
test_dataloader = DataLoader(
    test_dataset, batch_size=BATCH_SIZE, shuffle=True, collate_fn=collate_batch
)


from torch.nn.utils.rnn import pad_sequence

def collate_batch(batch):
    label_list, text_list = [], []
    for _label, _text in batch:
        label_list.append(label_pipeline(_label))
        text_list.append(torch.tensor(text_pipeline(_text), dtype=torch.int64))


    label_list = torch.tensor(label_list, dtype=torch.int64)
    text_list = pad_sequence(text_list, batch_first=True)


    return label_list.to(device), text_list.to(device)

```