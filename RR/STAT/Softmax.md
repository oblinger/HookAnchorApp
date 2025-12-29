
 [Wikipedia](https://en.wikipedia.org/wiki/Softmax_function) - 


Softmax
-?-
- Its n inputs are exponentiated, then normalized to sum to one.
	out_i = e^(in_i) / sum( e^(in_i) for all i)
- Can treat output as if it is a pdf over outputs.
- Softmax is a Generalization of the Sigmoid Function (the Logistic Function) in logistic regression where the the number of classes is 2 or greater.
- nn.Softmax(tensor_matrix_of_weighted_one_hot_multi_class_outputs) returns normalized one-hot-vector as a pdf
.
- By itself it can generate linear boundaries assuming weighted inputs <!--SR:!2025-03-23,2,230-->



#dl 
