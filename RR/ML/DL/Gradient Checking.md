! Method used to debug NN implementations


- Compute approximate gradient by merging ALL parameters into one big theta vector, then perturbed each parameter by adding and subtracting epsilon = 1e-7
       approx_grad = ( COST(theta+epsilon) - COST(theta-epsilon) )  /  (2 * epsilon)

- Then compare this to the directly computed gradient = backprop(theta)
	delta =  || approx - grad ||2   /   ( ||approx||2 + ||grad||2 ) 

If delta < 1e-7 all is good
if delta < 1e-5 all is probably ok
if delta > 1e-3 we very likely have a problem

