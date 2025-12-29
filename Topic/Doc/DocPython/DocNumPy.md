

# TUTORIAL

NUMPY CREATE: from list, zeros, ones, arange, uniform
-?-
```
np.array([1, 2, 3])   -> [1, 2, 3]
np.ones((2, 1))       -> [[1.], [1.]]  # RC cola
np.zeros((1, 2))      -> [[0., 0.]]
np.arange(0, 10, 2)   -> [0, 2, 4, 6, 8]  # start, step optional
np.linspace(0, 1, 5)  -> [0., 0.25, 0.5, 0.75, 1.]
```



NUMPY INDEX: element, row, col, negative; SLICE: range, step
-?-
```
a = np.arange(12).reshape(3,4)  # 3 rows, 4 cols
# index
a[0, 1]        -> 1               # single element
a[0] == a[0,:] -> [0, 1, 2, 3]    # full row
a[:, 1]        -> [1, 5, 9]       # full column
a[-1]          -> [8, 9, 10, 11]  # last row
# slice
a[0:2, 1:3]    -> [[1,2], [5,6]]  # submatrix
a[:, ::2]      -> cols 0, 2       # step
```
<!--SR:!2025-12-14,2,230-->



NUMPY RESHAPE: reshape, flatten, view, transpose
-?-
```
a = np.arange(6)          -> [0, 1, 2, 3, 4, 5]
a.reshape(2, 3)           -> [[0,1,2], [3,4,5]]     # accepts tuple too
a.reshape(3, -1)          -> [[0,1], [2,3], [4,5]]  # -1 = infer
a.reshape(2,3).flatten()  -> [0, 1, 2, 3, 4, 5]     # copy
a.reshape(2,3).ravel()    -> [0, 1, 2, 3, 4, 5]     # mutable-ish view
a.reshape(2,3).T          -> [[0,3], [1,4], [2,5]]  # transpose
```



NUMPY AGGREGATE: sum, mean, max, argmax, by axis
-?-
```
a = np.array([[1,2,3], [4,5,6]])  # shape (2, 3)
a.sum()          -> 21            # all elements
a.sum(axis=0)    -> [5, 7, 9]     # collapse rows (down)
a.sum(axis=1)    -> [6, 15]       # collapse cols (across)
a.max()          -> 6
a.argmax()       -> 5             # flat index of max
a.argmax(axis=1) -> [2, 2]        # col index of max per row
a.mean(axis=0)   -> [2.5, 3.5, 4.5]
```



NUMPY BROADCAST: element-wise ops ( + , - , * , / , // , % , ** , < , > , = = , & , | )
-?-
```
# Rule: dims align from right, must match or be 1 (exact for @ or * )
a = np.array([[1], [2], [3]])  # shape(3, 1)
b = np.array([10, 20])         # shape(2)
a + b  -> [[11,21], [12,22], [13,23]]  # shape(3,1) + shape(2) = shape(3,2)

a = np.arange(3)               # [0, 1, 2]
a + 10    -> [10, 11, 12]      # scalar broadcasts
a * 2     -> [0, 2, 4]
a ** 2    -> [0, 1, 4]
```



NUMPY BOOLEAN: boolean indexing, where, any/all
-?-
```
a = np.array([1, 2, 3, 4, 5])
a > 2                  -> [False, False, True, True, True]
a[a > 2]               -> [3, 4, 5]         # filter
a[(a > 2) & (a < 5)]   -> [3, 4]           # combine with & | ~

np.where(a>2, a+1, 7) -> [7, 7, 4, 5, 6]  # conditional select
np.any(a > 4)          -> True
np.all(a > 0)          -> True
```
<!--SR:!2025-12-14,2,248-->



NUMPY RANDOM: reproduce, unit/uniform, int, choose, permute
-?-
```
np.random.seed(42)              # reproducibility
np.random.rand(2, 3)            # uniform [0, 1), shape (2,3)
np.random.randn(2, 3)           # standard normal, shape (2,3)
np.random.randint(0, 10, (3,))  -> [6, 3, 7]  # integers [0, 10)
np.random.choice([6,7,9], 2)    -> [9, 6]     # sample from array
np.random.permutation(5)        -> [2, 0, 4, 1, 3]  # shuffled range
```



NUMPY STACK: concatenate, vstack, hstack, split
-?-
```
a = np.array([[1, 2], [3, 4]])
b = np.array([[5, 6]])

np.vstack([a, b])      -> [[1,2], [3,4], [5,6]]  # stack vertically
np.hstack([a, b.T])    -> [[1,2,5], [3,4,6]]     # stack horizontally
np.concatenate([a, b], axis=0)  # same as vstack

c = np.arange(6)
np.split(c, 3)         -> [[0,1], [2,3], [4,5]]  # equal split
np.split(c, [2, 4])    -> [[0,1], [2,3], [4,5]]  # split at indices
```



NUMPY LINALG: dot product, matmul, @, inverse, determinate
-?-
```
# dot product (1D): sum of element-wise products -> scalar
x = np.array([1, 2, 3])
y = np.array([4, 5, 6])
np.dot(x, y)       -> 32              # 1*4 + 2*5 + 3*6

# matmul (2D): inner dims must match -> shape(m,n) @ shape(n,p) = shape(m,p)
a = np.array([[1, 2]])                # shape(1, 2)
b = np.array([[1, 2, 3], [4, 5, 6]])  # shape(2, 3)
a @ b              -> [[9, 12, 15]]   # shape(1,2) @ shape(2,3) = shape(1,3)

# inv/det require square matrix
m = np.array([[1, 2], [3, 4]])
np.linalg.inv(m)   -> [[-2, 1], [1.5, -0.5]]
np.linalg.det(m)   -> -2.0
```



NUMPY UTIL: sort, argsort, unique, cast
-?-
```
a = np.array([3, 1, 4, 1, 5])

# sort
np.sort(a)         -> [1, 1, 3, 4, 5]    # sorted copy
np.argsort(a)      -> [1, 3, 0, 2, 4]    # indices that would sort

# unique
np.unique(a)       -> [1, 3, 4, 5]       # distinct values, sorted

# cast
a.astype(float)    -> [3., 1., 4., 1., 5.]
a.astype(np.int8)  -> [3, 1, 4, 1, 5]    # smaller int type
a.dtype            -> int64              # check current type
```



#numpy
# OLDER


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


## RANDOM
-   numpy.random.rand() produces numbers in a [uniform distribution](https://raw.githubusercontent.com/jahnog/deeplearning-notes/master/Course2/images/rand.jpg).
-   and numpy.random.randn() produces numbers in a [normal distribution](https://raw.githubusercontent.com/jahnog/deeplearning-notes/master/Course2/images/randn.jpg).
- np.random.permutation(size)    X[:permutation]  (will generate a permuted matrix)
- np.random.seed(1)  -- Initializes the random number generator
-  
## CREATE   .zeros(dims)  .deepcopy  .randint  .randn
- np.zeros([x, y])
- np.deepcopy( X )
- np . random . randint(1, 11, (4,3,2))
- np.random.randn(r, c)   # matrix of [0, 1]


## SELECT X[:, 10:20]

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