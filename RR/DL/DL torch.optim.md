
[[DL Optim]] 

[[DL Optimization]] [[DL optimization]] 


Stocastic Gradient Descent
-?-
nn optimization, mini-batch-size == 1
.
optimizer = optim.SGD(model.parameters(), lr=0.01, weight_decay=0.01)
.
lr == Learning Rate (0.01)
weight_decay == L2 Regularization. Same as:
lambda_l2 = 0.01  # L2 regularization strength
l2_norm = sum(param.pow(2.0).sum() for param in model.parameters())
loss = criterion(outputs, targets) + lambda_l2 * l2_norm

#dl 

