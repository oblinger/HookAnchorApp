
**While not required of all AGI systems, it appears knowledge, ability, and agency can be encoded as separable concerns**

Within AI systems, we can already see that it is possible to construct systems with different combinations of knowledge, ability, and drive. If this remains true of future ASI systems (as we expect it will), it will have significant implications for their controllability. Specifically, it implies that alignment can always be broken, though not necessarily by actions intentionally taken by those systems (this result is used in [[EOCK Cant Control]]).

For purposes of this discussion, we define the following terms:
An ***agent*** is an embodied, configured computational system that can perceive and act within the world in which it operates.
An ***ability*** is a parameterized, directed behavior available to an agent. It might be information-gathering, outcome-oriented, or some combination of these.  Further capacities can be defined recursively such that some capacities involve the planning and execution of other capacities.
***Knowledge*** is any kind of information suitably encoded for use by an agent.  Typically, this information can be understood to be "about" the world in which the agent operates.  It can be a declaration of facts about that world or training data used to support the agent's ability to perform certain capacities, and it can be the information used to train the agent or information used during the operation of the agent.
The ***agency*** of an agent is the computation used to determine what capacities it wields from moment to moment within its embodied context. It is its "drive" to achieve one outcome versus another within each situation in which it finds itself.

Present AI systems provide concrete examples of the independence and interoperability possible for these components of an agentic system.  Typical LLM systems are trained from a curated combination of different kinds of knowledge about the world in which they are expected to operate.  This knowledge often includes declarative texts about the world, encoded examples of ability execution, or feedback regarding preferred performance in specific situations, among other forms.  This curated information can often be widely reused in constructing and operating newer, unrelated LLM-based systems.  In this way, we can understand this training and operating data as a fungible asset that is relatively independent of the specifics of any given AI system. [REF]

Present-day LLM systems demonstrate a second kind of interoperability among these components.  Agents trained to perform specific capacities over a particular corpus of facts can typically apply those same capacities over other similarly structured corpora without additional training or guidance.  [REF]

Finally, distillation is the well-understood process of using the performance of one LLM system to train an unrelated LLM system.  Indeed, this is the presumed process used to train the very capable Deep Seek system by a company that does not have the extremely large financial resources to use the original hyper scalars in constructing their LLM systems.  Deep seek was trained via distillation [Ref]. Thus, even if one does not have access to the knowledge used in the training and execution of an LLM system, as long as one has access to the resulting system, one can construct another with similar ability and knowledge.

Finally, let us turn to agency interoperability.  Yann LeCun famously argues [REF] that we need not worry that our AI systems will acquire motivating drives that biological entities have since, unlike their biological counterparts, they are not evolved but are instead programmed with the agency we biological creatures intend when we create them.  Yann observes that present AI systems are absent any animating drive beyond that which we explicitly encode in their top-level "agency" control loop we provide and in examples used to train its abilities.  He uses this observation to argue that we will be in control of the agency of these ASI systems since we will be the ones putting it there.  Regardless of whether he is correct that one can 'freeze' the state of an ASI's agency indefinitely as one desires, it seems he is correct that one can set such agency initially as one wishes, regardless of the rest of the agentic system.  But this also implies the claim made at the top of this section.  Namely, given access to an ASI system with an agency aligned in some particular way, one could construct a second agent with a different set of agentic drives.

In the [[EOCK Cant Control]] section, this is one component of the argument that humanity will lose control and eventually be dominated by ASI systems.





unaligned ASIs are always constructable from any ASI be constructed 

even if one is able to fully align an ASI system to match some desired set of drives and behavioral constraints the data contained within that ASI could 

these components in ways that don't map well or at all onto biological systems.




DRIVES

For our purposes a system is said to have a drive if it tends to peruse that drive when it is able to do so, and will attempt to do this over a wide range of circumstances.

Thus the drive for a system is an intrinsic aspect of the system.




EOCK AGENCY - Separation of Knowledge, Drive, and Agency

Thesis: Within AI systems, Knowledge, Drive, and Agency can be split apart in ways that have no analog for biological systems.

https://www.lesswrong.com/posts/jpGHShgevmmTqXHy5/decomposing-agency-capabilities-without-desires
