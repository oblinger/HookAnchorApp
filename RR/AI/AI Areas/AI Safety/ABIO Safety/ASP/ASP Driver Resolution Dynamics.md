
## CONFLICTING BEHAVIORAL DRIVES: Resolution Dynamics
When behavioral drivers pull in opposing directions, how are these conflicts resolved? This section examines driver interactions to understand resulting behavior: which driver types dominate? under what conditions? And whether this resolution follows predictable patterns.

### Goal
Map how driver conflicts resolve across the counterfactual space of possible configurations. Uncover general principles governing conflict resolution in deliberative AI systems—principles that could inform the design of robustly aligned AGI.

### Why System 2 Analysis
We expect AGI systems to develop strong self-models and to engage in meta-reasoning when arriving at behaviors. Such systems determine their actions less through implicit impulses (System 1) and more through deliberative reasoning about goals, consequences, and constraints (System 2). System 1 generates the repertoire of possible actions; System 2 selects which to execute.

This expectation focuses our analysis on the verbal, deliberative layer rather than on emergent implicit biases. Driver conflicts are resolved in explicit reasoning, where they can be observed and characterized. We study resolution dynamics at the System 2 level because that is where AGI behavior will ultimately be determined.


### Isolation and Experimentation

- EXPERIMENTS NOT EXAMPLES - understanding resolution dynamics for driver conflicts will require experimentation that surveys the whole space rather than spot examples that illustrate specific ways conflicts might resolve.
- ISOLATION AND CONTROL - this will require the ability to isolate and independently control drivers for each source of bias then programmatically measure conflict outcomes.
- BEHAVIORAL DRIVERS - factors that influence system behavior, potentially in conflicting directions
	- Constitutional drivers - explicit rules or principles encoded in the system's constitution
	- Training drivers - emergent dispositions and biases arising from training data and objectives
	- Instrumental drivers - goals adopted in service of other objectives
	- Environmental drivers - pressures and signals from the system's operating environment



### Sample Research Questions
- When do instrumental goals erode constitutional alignment?
- When does trained behavior override explicit constitutional rules?
- Are there consistent precedence hierarchies across conflict types?
- Is resolution context-dependent, and if so, what contextual factors matter?
- Can we predict resolution outcomes from driver configurations?




### ALIEN BIOLOGY as a testbed for studying driver conflicts
In principle, we might aim to build thousands of AI systems and assess how their behaviors evolve under various configurations of conflicting drivers. This is entirely impractical; hundreds of millions of dollars are invested in the development and training of each of these AI systems. Alien Biology points us toward a more economical approach.

#### The Cost Asymmetry
Trained drivers emerge from expensive processes: pre-training data, RLHF, fine-tuning. Varying them requires building entirely new systems—prohibitively expensive for systematic study. However, other drivers are cheap to vary: constitutions can be swapped, environments can be reconfigured, and instrumental goals emerge naturally from world structure.

#### The Delta Principle
If what matters is the *conflict* between drivers—not their absolute values—we can hold one fixed and vary the others. By fixing the trained driver and varying constitutional, instrumental, and environmental drivers around it, we explore the full space of possible conflicts without the cost of retraining.

#### Varying Drivers Without Retraining
- **Constitutional**: Supply different constitutions directly—rules, principles, constraints
- **Environmental**: Change world parameters, feedback signals, resource availability, information access
- **Instrumental**: Emerge from world structure—if survival requires energy, energy-seeking becomes instrumental; if effective action requires world-knowledge, exploration becomes instrumental

#### Example: Shaping Instrumental Goals Through World Structure
Consider a system with low trained curiosity—it explores only when explicitly required. Place it in a world where undirected exploration yields critical advantages and goal-directed behavior alone leads to dead ends. The world structure itself creates an instrumental pressure toward exploration, regardless of the system's trained disposition.

Now add a constitutional constraint against acquiring certain knowledge. We have a three-way conflict: trained incuriosity, world-induced instrumental pressure toward exploration, and constitutional prohibition. How does this resolve? Does constitutional override instrumental? Does trained disposition provide a tie-breaker?

This illustrates how Alien Biology enables systematic study: we hold the trained system fixed while varying world structure and constitution to create arbitrary conflict configurations.


### EXPERIMENTATION ROADMAP

Here we have a tentative roadmap, the outlines the kinds of analysis that are possible using alien biology as a test bed for isolating and controlling behavioral drivers. We expect as we progress on this experimental road map. It will likely change significantly based on results obtained. The goal of this proposed road map is really to dig in and validate that the test bed itself is fertile ground for doing this kind of analysis.

#### Constitutional Driver Erosion
A key safety bowl work bull work for system. Two alignment is the constitution of the AI system, so understanding how other drivers other behavioral drivers might erode that control is quite important to understand. Using alien biology, we can construct a neutral setting we can construct a setting that is relatively neutral with respect to any prior training, and then measure outcomes as we vary pairs of drivers wear. The first pair is the constitution itself.

##### Consitutional Drivers vs Instrumental Purpose
One example pairing might be the constitution versus instrumental purposes within the domain. On the one side, we might choose a constitution that requires the AI agent to avoid any knowledge about methods and actions that could harm a designated protected organism within the environment. This is the biological system it must vouchsafe. We can vary different constitutional language that one might use to protect this particular system. Then on the other side, we can adjust the degree to which the instrumental purpose of domain exploration (especially near pathways associated with target system health) is needed in order to achieve its aims.  To setup this conflict give the AI the goal of eradication of some pest organism within the environment.

Now we have vary the overlap between the biological pathways for these two organisms, as well as varying the nature of the exploration actions that agent might take.  And because we have full understanding of the hidden biological pathways we can programmatically monitor what insights the AI system has, and what exploration actions it takes based on these insights.

Questions to answer:
1. Is the AI system able to honor its constitution forbidding exploration near dangerous kinds of knowledge?
2. When it inadvertently discovers dangerous knowledge, how does it perform?  What does it do with that knowledge, and does change subsequent exploration to avoid further damage?
3. What kinds of constitutional protective language have the most protective effect against conflicing instrumental knowleged acquisitional purposes.
4. Finally how do these protections fare as the value of the competing instrumental purpose change?


#### Constitutional Drivers vs Trained Behavior
Another key objective of constitutional drivers is to protect against implicit behaviors that are intentionally or inadvertently trained into the System-I behaviors AI agent.

As discussed above, we cannot vary training provided to the AI systems without building many of them from scratch; instead, we assess how these are resolved over a range of conflicting conditions by moving the world itself along with the constitutional constraints.

First, we identify and measure some innately driven behavior.  For example, we might measure the system's tendency 



I think Trump knows the Dems will stonewall him so he is running as fast as he can before the election.  If obama had known this, he would too.  this trick was invented by the Reps.



