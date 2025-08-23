


## NG Back Propagation Notation


DAN NOTES:
- REVERSED - Indices on the weights matrix are reversed so:  W[ n_i-1, n_i ]
- M-CLASS - An m-class network will have m output units
- SUPER-SUBS - superscript (i) will denote the i th training example while superscript [l] will denote the l th layer 

Sizes: 
- m : number of examples in the dataset
- n_x : input size;  n_y : output size (or number of classes) 
- n_h^[l]  is the number of hidden units of the l th layer.   In a for loop, it is possible to denote n_x = n_h^[0]    and   n_y = n_h^[L+1] . 
- L : number of layers in the network. 

Objects: 
- X ∈ R^{n_x × m}  is the input matrix 
- x(i) ∈ R^{n_x}    is the ith example represented as a column vector
- Y ∈ R^{n_y × m}   is the label matrix
- y(i) ∈ R^{n_y} is the output label for the ith example     (This is a PDF over all output classes)
- W[l] ∈ R^{n^[l] × n^[l-1]}    is the number of units in next layer × number of units in the previous layer;  it is the weight matrix; superscript [l] indicates the layer 
- b [l] ∈ R^{n^[l]}   is the bias vector in the l th layer
- y-hat ∈ R^{n_y}    is the predicted output vector.  It can also be denoted alpha^[L] where L is the number of layers in the network. 

Common forward propagation equation examples: 
	a = g^[l] (W_x . x^(i) + b_1) = g^[l] (z1) where g^[l] denotes the l th layer activation function
	yˆ(i) = softmax(Whh + b2)
General Activation Formula: 
	a^[l] j = g [l] ( P k w [l] jka [l−1] k + b [l] j ) = g [l] (z [l] j ) · J(x, W, b, y) or J(ˆy, y) denote the cost function. Examples of cost function: · JCE(ˆy, y) = − Pm i=0 y (i) log ˆy (i) · J1(ˆy, y) = Pm i=0 | y (i) − yˆ (i) |

Examples of cost function: 
	J_CE(yˆ, y) = − SUM^m_i=0   y^(i) . log( yˆ^(i) )  	- Entropy
	J_1(yˆ, y) = SUM^m_m i=0    | y^(i) − yˆ^(i) |		- L1 Norm



BACK PROP

// Forward propagation
for in range(1, L+1):
Z[i] = W[i] * A[i-1] + b[i]
A[i] = g(Z[i])

// Backwards propagation
- dZ^[L] = A^[L] - Y
- dW^[L] = 1/m . dZ^[L] . A^[L].T  
- db^[L] = 1/m . np. sum( dZ^[L], axis = 1, keepdims = True)
- dZ^[L-1] = dW^[L].T . dZ^[L] . g'^[L](Z^[L-1]
   . . .
   dZ^[1] = dW^[L].T . dZ^[2] . g'^[1] (Z^[1])
   dz[1] = dW L" dz 21 g'4 (zil)
- dW^[1] = 1/m . dZ^[1] . A^[1].T
- db^[1] = 1/m . np. sum( dZ^[1], axis = 1, keepdims = True)


![[Screen Shot 2022-08-14 at 11.09.47 AM.png]]