

Torch Initialization
-?-
- Linear  default = { uniform in +/-  1/sqrt(layer_size) }.    Paper LeCun 2012 "Efficient Backprop"
- Xavier Initialization = uniform +/-  sprt(6) / sqrt(layer_out + layer_in). Xavier-Bengio 2010 "Understanding the difficulty of training deep feed fwd NN"
	  nn.init.xavier_uniform(linear.weight)
- HE method (used with ReLU).    nn.init.kaiming_uniform_(linear.weight, nonlinearity='relu').  He Kaiming 2015 "Delving deep into rectifiers: ..." <!--SR:!2025-02-22,3,250-->



#dl 

