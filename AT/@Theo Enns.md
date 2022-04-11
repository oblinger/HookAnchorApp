
# MEETINGS
### m2022-03-08 Astro. FAANG. ML new ideas.
existing product; science early prod validation; new product

[[FAANG]].   




### m2022-03-08 Ideas About Amazon Levels
#### Dan's Role:   
- AS (Associate Scientist)
	- Pubs do matter, but so does doing research in applied setting.
	- AS science role (code but not production; )
	- strategies around stuff algo the read
- SDM (Software Development Manager)
- Staff scientists (publishing)

#### LEVELS AT AMAZON
The "essence" of a level is the amount of scope afforded company.  
- L5 own a feature, not strategy
- L6 influence across multiple groups; front line interacting w other groups
		    - strategy with their group; mentoring; hiring; growth members and team
		    - scope: influence multiple groups within a product
- L7 influence across multiple products.  over a whole product and some others
		- 40-60 under them.
		- principle engineer  (hard part of code, or sys architect)
			- large strategic impact; referees in org
		- need to align w/ corp culture from day one


#### THEO:
L6 IC->SDM (Software development manager)
   very productive team. low headcount. 

INTERVIEWING PREP
- What have you accomplished lately
- Break down problem; both in the weeds & strategic
- Amazon corp values

### m2022-03-03 - AEOLUS research summary from Theo's perspective

AEOLUS RESEARCH
- Markus Vincze lab @ TU WIEN (U of Vienna)
  V4R (Vision for robotics)
- David Fischinger (Grasping Research)
  [Height Accumulated Features (HAF) grasping](https://www.acin.tuwien.ac.at/fileadmin/acin/v4r/v4r/IROS2012_Draft.pdf) 
  Parametric Object Grasping
- George Todoran. (Base Motion Planning)
  Optimal Local Path-Planning  (using an ODE solver)
  Moving Horizon Trajectory Planner (MHTP) with an update rate of 10Hz
  Randomly-exploring random tree search  (RTT*).  
  A* over heuristically generated way-points graph


FROM THEO
- wiping research - severin, Brandon
	- Triangular mesh.  
		- Marching squares
		- Flat panes
- What algos did we use?  
	- SIFT/SURF - tracking
	- skeletal tracking
	- base motion - 
	- grasping  HAF. (prof name & )

Triangule -- Marching squares.  
- flat region (largest planes, fill with lesser planes)
	- greedy grid growth (continuity -gap -angle; curvature; )
		- first / second order discontinuities
	- flood fill (based on normal vector similarity, and physical separation and gaps)
		- -> 3D least squares plane fitting

Academic Algs impractical / 70% cheats

Entirety of use-cases.  least rescources to solve the problem.


- haptic edge wiping algorithms
