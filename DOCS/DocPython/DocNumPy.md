-   numpy.random.rand() produces numbers in a [uniform distribution](https://raw.githubusercontent.com/jahnog/deeplearning-notes/master/Course2/images/rand.jpg).
-   and numpy.random.randn() produces numbers in a [normal distribution](https://raw.githubusercontent.com/jahnog/deeplearning-notes/master/Course2/images/randn.jpg).

A.sum(axis=0)    # sum vertically (along axis 0);  Resulting matrix has this axis completely removed
A.reshape((1,4))

m = np . random . randint(1, 10, (4,3,2))
m.shape

## MATH

### ELEMENT WISE (with broadcast)
    .sum(axis=0, keepdims=True)
    .sum(list)  .multiply(...)  .power(x, y)   .sqrt(x)

### SPECIAL
    .dot(a, b)      (matrix multiplication)


## CREATE   .zeros(dims)  .deepcopy  .randint  .randn
- np.zeros([x, y])
- np.deepcopy( X )
- np . random . randint(1, 11, (4,3,2))
- np.random.randn(r, c)   # matrix of [0, 1]

## HOW IT WORKS
### BROADCAST
- find earliest missing
- 
### ARRAY SHAPE
- First shape index, is first indexing index, is outer most row
- Matrix = (num_rows, num_cols)




def forward_propagation(X, parameters): -> cache
def compute_cost(A2, Y):
def backward_propagation(parameters, cache, X, Y):

    grads = {"dW1": dW1,
             "db1": db1,
             "dW2": dW2,
             "db2": db2}

def update_parameters(parameters, grads, learning_rate = 1.2):