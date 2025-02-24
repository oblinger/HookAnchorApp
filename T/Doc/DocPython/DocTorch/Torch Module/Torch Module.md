.[[Torch Module]].
  , [[Feed Forward Layer]],
  :
  , [[Convolution]], [[Torch Model]], 
  [[Recurrent Modules]], 

  DELS: [[Torch Model]], [[Convolution]], 



#dl #review
- [[Torch Optim]] 
- [[Torch Loss]] 

Models

Optimizers - torch.optim
- .SGD(m, lr=, )
- Adam - Adaptive Moment Estimation (AdaGrad, RMS)

```python

from torch import nn

class Model(nn.Module):
	def __init__(self, in_dim: int, out_dim: int):
		super().__init__()
        ...
    def forward(self, x):
	    return nn.sigmoid(self.linear(x))

model.train()
model.eval()

model = Model(1, 1)
model.state_dict()
list(model.parameters())

mods = nn.ModuleList(); mods.append(...)
```



Common FOUNDATIONAL Torch Modules
-?-
nn.Linear(in,out,bias=True)
nn.Conv2d <!--SR:!2025-02-22,3,250-->


Common Activation/Pooling/Regularization Torch Modules
-?-
nn.ReLU, nn.Tanh, nn.Sigmoid
nn.MaxPool2d, nn.AvgPoold2, 
nn.Dropout, nn.Dropout2D, ... <!--SR:!2025-02-22,3,250-->

Common Recurrent/Container Torch Modules
-?-
nn.RNN, nn.LTSM, nn.GRU
nn.Sequential, nn.ModuleList
nn.MultiheadAttention


How does RNN work?
Here is a brief summary of how RNNs work:

RNNs are neural networks that can process sequential data, such as text, audio, or time series data.
They contain a “hidden state” that is passed from one element in the sequence to the next, allowing the network to remember information from previous elements.
At each time step, the RNN takes in an input and the current hidden state, and produces an output and a new hidden state.
The output and new hidden state are used as input for the next time step, and this process continues until the entire sequence has been processed.
RNNs are well-suited for tasks involving variable-length sequences and maintaining state across elements.
How does LSTM work?
Here is a brief summary of how Long Short-Term Memory (LSTM) networks work:

LSTMs are a type of Recurrent Neural Network (RNN) that can better retain long-term dependencies in the data.
They have a more complex structure than regular RNNs, consisting of input, output, and forget gates that can selectively retain or discard information from the hidden state.

The input gate determines which information from the current input to store in the hidden state.
The forget gate determines which information from the previous hidden state to keep or discard.
The output gate determines which information from the hidden state to output as the final prediction.
This combination of gates allows LSTMs to retain important information from long sequences and discard irrelevant or outdated information.
LSTMs are often used for tasks involving long-term dependencies, such as language translation and language modeling.
How does GRU work?
Here is a brief summary of how Gated Recurrent Units (GRUs) work:

GRUs are a type of Recurrent Neural Network (RNN) that uses a simpler structure than LSTMs and is easier to train.
They have two gates: an update gate and a reset gate.
The update gate determines which information from the previous hidden state and current input to keep, and the reset gate determines which information to discard.
The final hidden state is a combination of the information retained by the update gate and the current input.
This combination of gates allows GRUs to retain relevant information from long sequences and discard irrelevant or outdated information.
GRUs are often used for tasks involving sequential data, such as language translation and language modeling.

Summarizing the Difference Between RNN vs LSTM vs GRU
RNNs are a type of neural network that are designed to process sequential data, such as text, audio, or time series data. They can “remember” or store information from previous inputs, which allows them to use context and dependencies between time steps.
LSTMs are a type of RNN that use special type of memory cell and gates to store and output information. The gates in an LSTM network are controlled by sigmoid activation functions. These gates allow the network to selectively store or forget information. LSTMs are effective at storing and accessing long-term dependencies. They are slower to train and run than other types of RNNs.
GRUs are simplified version of LSTMs that use single “update gate” to control the flow of information into the memory cell. GRUs are easier to train and faster to run than LSTMs, but they may not be as effective at storing and accessing long-term dependencies.
There is no one “best” type of RNN for all tasks, and the choice between LSTMs, GRUs, and other types of RNNs will depend on the specific requirements of the task at hand. It is often a good idea to try multiple types of RNNs and see which one performs best on your specific task.

Conclusion
In conclusion, the key difference between RNNs, LSTMs, and GRUs is the way that they handle memory and dependencies between time steps. RNNs, LSTMs, and GRUs are types of neural networks that process sequential data. RNNs remember information from previous inputs but may struggle with long-term dependencies. LSTMs effectively store and access long-term dependencies using a special type of memory cell and gates. GRUs, a simplified version of LSTMs, use a single “update gate” and are easier to train and run, but may not handle long-term dependencies as well. The best type of RNN depends on the task at hand. It is often useful to try multiple types and see which performs best. <!--SR:!2025-02-20,1,230-->


Batch Normalization
-?-
- nn.BatchNorm?d(num_nodes)    
- Reduces internal covariate shift across epochs and mini-batches.  
- scale and shift parameters retain model flexibility.
- No longer need dropout
- you can increase learning rate safely
- bias term is no longer needed
```python
class NetBatchNorm(nn.Module):
    
    # Constructor
    def __init__(self, in_size, n_hidden1, n_hidden2, out_size):
        super(NetBatchNorm, self).__init__()
        self.linear1 = nn.Linear(in_size, n_hidden1)
        self.linear2 = nn.Linear(n_hidden1, n_hidden2)
        self.linear3 = nn.Linear(n_hidden2, out_size)
        self.bn1 = nn.BatchNorm1d(n_hidden1)
        self.bn2 = nn.BatchNorm1d(n_hidden2)
        
    # Prediction
    def forward(self, x):
        x = self.bn1(torch.sigmoid(self.linear1(x)))
        x = self.bn2(torch.sigmoid(self.linear2(x)))
        x = self.linear3(x)
        return x
```
<!--SR:!2025-02-22,3,250-->
