
[[ABIO]]  [[ABIO Log]] 
- [ ] [[ABIO Log#2025-05-12 Brainstorm behavior generators]] 


SHOP ABIO AROUND
- [[@ARC]] - Try to connect with ARC.  



compute cosign

do some tokenization

relu
leaky relu

- how to lang chain
- high level:  experience docker (layer caching)



### Notes

Fast Path
- DAG of biomolecules
- Generator of biomolecules DAGs

Target - Energy Transport & Cell Growth 
Requires generators for:
- Chain of Cycle constructors
- Growth tree constructors
- Enzyme-controlled bioprocesses drivers for chains and trees 



### Experiments

Task #1 - Derive a functional model of molecule X given
- DAG of biomolecules w/ chemical bond strengths
- Table of molecular concentrations in many configurations over time.k
- Output 
	- induced chemistry consistent with observations
	  (scored by matching the reactant and product lists for each)
	- induce bioprocesses
	  (scored by checking answers to prediction questions)


Task #2 - Cure a disease
- Extends task #1, but is given
	- results from a healthy strain and a diseased strain of an organism
	- explain disease pathways involved
	  (scored as a list of bio-processes)
	- Return a script that intervenes to cure the disease

Damage Editor
