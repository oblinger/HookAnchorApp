


Dropout
-?-
-  is used to avoid overfitting, by training a network to be resilient to neurons being temporarily disabled.  
- Such a network learns to redundantly encode key information thus avoid overfitting.  A Bernoulli distribution is used on each epoch to "turn off" a random set of hidden neurons, thus forcing other neurons to "cover" for the missing neurons.  
.
self.drop = nn.Dropout(0.1)    # removes 10% of nodes during training (typically .1 to .5)
x = self.drop(x)  # in forward method <!--SR:!2025-03-29,8,250-->


#dl 

