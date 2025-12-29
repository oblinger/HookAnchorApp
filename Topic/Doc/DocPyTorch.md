
# Pytorch

## Pytorch Flow

PYTORCH FLOW: end-to-end pipeline
-?-
```
# 1. DATA
dataset = MyDataset(data)                    # subclass of Dataset
loader = DataLoader(dataset, batch_size=32)  # yields batches

# 2. MODEL
model = MyModel().to(device)                 # subclass of nn.Module

# 3. TRAINING SETUP
criterion = nn.CrossEntropyLoss()
optimizer = torch.optim.Adam(model.parameters(), lr=0.001)

# 4. TRAIN LOOP (per epoch)
model.train()
for x, y in loader:
    x, y = x.to(device), y.to(device)
    optimizer.zero_grad()                    # clear old grads
    pred = model(x)                          # forward
    loss = criterion(pred, y)
    loss.backward()                          # compute grads
    optimizer.step()                         # update weights

# 5. EVAL
model.eval()
with torch.no_grad():
    pred = model(x_test)
```



PYTORCH DATA: Dataset, DataLoader, transforms
-?-
```
class MyDataset(Dataset):  # images: (N, Channels, H, W), labels: (N,)
    def __init__(self, images: Tensor, labels: Tensor, transform=None):
        self.images, self.labels, self.transform = images, labels, transform
    def __len__(self) -> int:
        return len(self.images)
    def __getitem__(self, idx: int) -> tuple[Tensor, Tensor]:
        img = self.transform(self.images[idx])
        return img, self.labels[idx]

transform = transforms.Compose([         # example (for images)
    transforms.ToTensor(),               # PIL/numpy -> tensor [0,1]
    transforms.Normalize((0.5,), (0.5,)) # (mean,), (std,)
])
dataset = MyDataset(images, labels, transform=transform)

# DataLoader: batching, shuffling
loader = DataLoader(dataset, batch_size=32, shuffle=True)
```



PYTORCH MODULE: nn.Module, __init__, forward, parameters
-?-
```
class MyModel(nn.Module):
    def __init__(self, input_dim, hidden_dim, output_dim):
        super().__init__()                          # always call super
        self.fc1 = nn.Linear(input_dim, hidden_dim)
        self.fc2 = nn.Linear(hidden_dim, output_dim)
        self.relu = nn.ReLU()

    def forward(self, x):                           # defines computation
        return self.fc2(self.relu(self.fc1(x)))

model = MyModel(784, 128, 10)
model.parameters()      # iterator of all WEIGHTS (for optimizer)
model.to('cuda')        # move to GPU
model.train()           # training mode (dropout active, batchnorm updates)
model.eval()            # eval mode
```
<!--SR:!2025-12-14,2,248-->



PYTORCH LAYERS: Linear, CNN, ReLU, Normalization, Dropout, Chaining[[null]] 
-?-
```
# Fully connected
nn.Linear(in_features, out_features)      # weight: (out, in)

# Convolution
nn.Conv2d(in_channels, out_channels, kernel_size, stride=1, padding=0)

# Activations
nn.ReLU()                  # max(0, x)
nn.Sigmoid()               # 1/(1+e^-x), output [0,1]
nn.Softmax(dim=1)          # probabilities summing to 1

# Regularization
nn.BatchNorm1d(features)   # normalize across batch
nn.Dropout(p=0.5)          # randomly zero elements (training only)

# Sequential: chain layers
model = nn.Sequential(
    nn.Linear(784, 128),
    nn.ReLU(),
    nn.Linear(128, 10)
)
```



PYTORCH LOSS: CrossEntropyLoss, MSELoss, BCELoss, NLLLoss
-?-
```
# Multiclass: raw logits, NOT softmax
nn.CrossEntropyLoss()  # (N,C) logits, (N,) ints 0..C-1

# Binary
nn.BCELoss()           # (N,) probs after sigmoid, (N,) 0/1
nn.BCEWithLogitsLoss() # raw logits (more stable)

# Regression
nn.MSELoss()           # mean squared error
nn.L1Loss()            # mean absolute error

# NLLLoss: use with log_softmax output
nn.NLLLoss()           # (N,C) log probs, (N,) indices
# CrossEntropyLoss = LogSoftmax + NLLLoss
```



PYTORCH OPTIM: SGD, Adam, lr, weight_decay, scheduler
-?-
```
# Common optimizers
optimizer = torch.optim.SGD(model.parameters(), lr=0.01, momentum=0.9)
optimizer = torch.optim.Adam(model.parameters(), lr=0.001)
optimizer = torch.optim.AdamW(model.parameters(), lr=0.001, weight_decay=0.01)

# Key params
# lr            - learning rate
# momentum      - SGD momentum
# weight_decay  - L2 regularization

# Learning rate scheduler
scheduler = torch.optim.lr_scheduler.StepLR(optimizer, step_size=10, gamma=0.1)
# In training loop:
for epoch in range(epochs):
    train(...)
    scheduler.step()         # decay lr every 10 epochs by 0.1x
```



PYTORCH GRADIENT CLIPPING: prevent exploding gradients
-?-
```
# In training loop, after loss.backward(), before optimizer.step()
torch.nn.utils.clip_grad_norm_(model.parameters(), max_norm=1.0)

# Why: gradients can explode (especially RNNs, transformers)
# max_norm: typical values 0.5, 1.0, 5.0
# Returns: total norm of gradients (useful for monitoring)
```



PYTORCH TENSOR: creation, dtype, device, requires_grad
-?-
```
# Creation
torch.tensor([1, 2, 3])   -> [1, 2, 3]
torch.zeros(2, 3)         -> [[0., 0., 0.], [0., 0., 0.]]
torch.ones(2, 3)          -> [[1., 1., 1.], [1., 1., 1.]]
torch.randn(2, 3)         # normal distribution
torch.arange(0, 10, 2)    -> [0, 2, 4, 6, 8]

# From numpy (shares memory!)
torch.from_numpy(np_array)
tensor.numpy()                       # back to numpy

# Device
tensor.to('cuda')                    # move to GPU
tensor.to('cpu')
tensor.device                        # check current device

# Gradient tracking
x = torch.tensor([1.0], requires_grad=True)
y = x ** 2
y.backward()
x.grad                               # dy/dx = 2.0
```



PYTORCH TENSOR OPS: reshape, squeeze, cat, stack, permute
-?-
```
# PyTorch-specific (NumPy differs)
a.permute(2, 0, 1)        # NumPy: a.transpose(2, 0, 1)
torch.cat([a, b], dim=0)  # NumPy: np.concatenate(..., axis=0)

# Same in PyTorch and NumPy
a.reshape(6, 4)           # reshape (-1 to infer)
a.squeeze()               # remove dims of size 1
a[None, :]                # add dim at position 0
a.T                       # transpose (2D)
torch.stack([a, b])       # np.stack — along NEW dim
a @ b                     # matrix multiply
a * b                     # element-wise multiply
```



PYTORCH SAVE/LOAD: state_dict, checkpoints, eval mode
-?-
```
# Save model weights only (recommended)
torch.save(model.state_dict(), 'model.pth')

# Load weights
model = MyModel(...)
model.load_state_dict(torch.load('model.pth'))
model.eval()              # set to eval mode before inference!

# Save full checkpoint (for resuming training)
torch.save({
    'epoch': epoch,
    'model_state_dict': model.state_dict(),
    'optimizer_state_dict': optimizer.state_dict(),
    'loss': loss,
}, 'checkpoint.pth')

# Load checkpoint
checkpoint = torch.load('checkpoint.pth')
model.load_state_dict(checkpoint['model_state_dict'])
optimizer.load_state_dict(checkpoint['optimizer_state_dict'])
```
<!--SR:!2025-12-13,1,230-->


## Datatypes






#pytorch 
