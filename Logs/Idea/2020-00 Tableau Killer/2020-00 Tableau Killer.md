

## === Key Idea ===

- NEVER SAY NEVER -- The tool can "do anything" since it is designed as a deep stack of quasi-WYSIWYG declarish-layers of business logic components built in terms of other components, along with an ecosystem of alternate compoenents at each layer in the stack.
	- This means that ANY change a user might want is just key clicks away.  Though certain changes:
		- May require finding the right component
	-  (if not already covered by some alternative component as some level)

- NO CODE -- Generally one use 'no code' or at most the editing of a typically one line formula to configure any but the lowest levels of the whole system.  Indeed most formulas can be indirectly edited by WYSIWYG editing of the end result and reverse inferring the parameter change require for this update.  But power users may find it faster to simply directly type the formula (with templated type-ahead support)




## === ARCHITECTURAL PIECES ===
### --- HYPERCUBE ---

HYPERCUBE -- A hypercube is an n-dimensional set of values derived from some underlying data.
- SELECT -- sub set of data is selected for compute
- PARTITION -- data is partitioned into cube elements
- REDUCE -- compute summarization(s) of the data that are lossy wrt original data and are recurively combinable
- IRREDUCABLE -- summarization that is not reducable
- DERIVE -- derive computed values based on the reduced values
- COMPUTE -- one or more values are derived on a per-partition basis



PARAMETRIC HYPERCUBE -- is a hypercube the depends upon a set of global parameters as well as the data.

_
### --- CUBE TRIMMER ---

CUBE TRIMMER -- A cube trimmer is a hypercube transform that reduces the volume of a hypercube in some parametric way.
- REPARTITIONING -- A cube-trimmer typically re-partitions by merging cells, but leaves underlying calculation intact

The idea is that many hypercube constructions can be split into construction+trimming

_
### --- POINT SET / SPARSE SET ---

POINTSET -- A pointset is a hypercube with

_
### --- GRABBLE ---

GRABLE -- A grabble is a graph or table visualization produced from some hyper cube of data

_