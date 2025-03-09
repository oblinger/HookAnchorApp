.[[DLF]].
  , [[DL Analyze]], [[DL Areas]], [[DL Data Loading]], [[DL Examples]]
  , [[DL Foundational Model Building]], [[DL Notation]], [[DL Optim]], 
  , [[DL Template]], [[DL Visualize]],
  , [[DL Activation]], [[DL Visualization]],
  , [[DL Initialization]],
  , [[DL Strategy]], 
  DELS: [[RR/DL/DLF/DL Strategy]], 


Deep Learning Framework

- [[DL Data Loading]] 
- [[DL Initialization]] 
- [[DL Optim]] 
- [[DL Visualize]] 
- [[DL Analyze]] 
- [[DL Template]] 
- [[DL Examples]] 


Detailed Overview
Step 1: Data Loading and Preprocessing
Step 2: Logistic Regression Model
Step 3: Model Training
Step 4: Model Optimization and Evaluation
Step 5: Visualization and Interpretation
Step 6: Model Saving and Loading
Step 7: Hyperparameter Tuning
Step 8: Feature Importance

```


#### imports

import pandas as pd
import sklearn
import torch
import torch.nn
import torch.optim
import matplotlib as plt
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScalar



#### Data Input

data = pd.read_csv('league_of_legends_data_large.csv')
y = data['win']
X = data.iloc[:,1:]   # data.drop('win', axis=1)

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=.2)

scalar = sklearn.StandardScalar()
X_train = scalar.fit_transform(X_train)
X_test = scalar.transform(X_test)

X_train = torch.tensor(X_train)
X_test = torch.tensor(X_test)
y_train = torch.tensor(y_train.values, dtype = torch.long)
y_test = torch.tensor(y_test.values, dtype = torch.long


# Create DataLoader for training and test sets
train_dataset = TensorDataset(X_train, y_train)
test_dataset = TensorDataset(X_test, y_test)

train_loader = DataLoader(train_dataset, batch_size=2, shuffle=True)
test_loader = DataLoader(test_dataset, batch_size=2, shuffle=False)



#### model

model = League(X_train.shape[1])
criterion = nn.BCELoss()
optimizer = optim.SGD(model.parameters(), lr=0.01)

```


