

Basic PyTorch training flow
-?-
class Data, 
setup: (1) MyDataset, (2) DataLoader, (2) criterion, (3) model, (5) optimizer
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

## Examples

```
# Create dataloader object, crierion function and optimizer.
trainloader = DataLoader(dataset=data_set, batch_size=3)
criterion_rms = nn.MSELoss()
learning_rate = 2
optimizer = torch.optim.SGD(model.parameters(), lr=learning_rate)


# Train the model
def train_model(epochs):
	for epoch in range(epochs):
		for x, y in trainloader:
			yhat = model(x)
			loss = criterion_rms(yhat, y)
			optimizer.zero_grad()
			loss.backward()
			optimizer.step()
			get_surface.set_para_loss(model, loss.tolist())
		if epoch % 20 == 0:
			get_surface.plot_ps()

train_model(100)
```
## Questions

PyTorch Training Steps
-?-
- Forward step.  Compute yhat = model(input)
- Loss step.  Compute   loss = loss_fn(y, yhat)
- Gradient step. Compute  loss.backward.  # this updates  .grad for each parameter within the model
- Optimize step.  Compute new model model1 = optimize(model)  # uses gradients in model

PyTorch criterion
-?- 
The "Criteria" is the loss function used to measure how far a model's predictions are from target during training

PyTorch model
-?- 
A set of parameters and a function used to compute predictions from data point inputs, as well as a "backwards" gradient function 

PyTorch gradient
-?-
that computes gradient of each model parameter

PyTorch optimizer 
-?- 
Updates a model based on gradients computed during the forward pass



#### Criteria / Loss Functions
-?-
nn.MSELoss().  # Mean Squared Error Loss
nn.BCELoss().  # Binary Cross Entropy Loss (Used for binary classification)

#### Optimizer
-?-
