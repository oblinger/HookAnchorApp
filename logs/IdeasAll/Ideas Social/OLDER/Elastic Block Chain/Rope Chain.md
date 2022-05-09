
## === SUMMARY ===

**RC SUMMARY**
- **RC** -- **RC** stands for _**Rope Chain (RC)**_ the name of this block chain paradigm.
- **ROPE** -- Unlike a block chain which is single threaded, a rope is many threaded DAG of "fibers" with each fiber in the rope depending on many prior fibers in the rope.
- **BLOCK CHAIN** -- A second, traditional, single-threaded block chain is used to record transactions in a manner identical to existing chains like BTC or ETH chains.
- **TIE IN** -- Each block in the chain must be tied to a fiber within the rope.  The rope serve the role of Proof of Work or Proof of Stake in an RC chain.
- **POSW** -- Proof of sequential work.  In order to dodge the wasteful energy sucking, climate crushing effects of inherently parallizable proof of work we replace this with proof of SEQUENTIAL work.  This work is not possible to parralize, thus it sets up a race to run the computation as fast a physics will allow, but it is designed to preclude any kind of parallelization.  Thus limiting the cost of power consumption associated with POSW.
	- SEQUENTIAL DERIVATION W. PARALLEL VERIFICATION
- **TRUST/STAKE** -- Rope chain is not a proof of trust system.  Nonetheless, as an additional security measure rope chain nodes are assigned trust by holders of rope coin.  These trust tokens provide some accountability regarding the diversity, trustworthiness, and stabilty of the mining nodes.
- **PENALTY ESCROW** -- An optional secondary stability measure could be added requiring mining nodes can be to post a one time fixed 'penalty escrow' -- this modest sum is just enough to pay for any momentary DDOS-like disruption such a node might intentionally cause in the network.  This optional penalty is paid to any participant that submits proof of node non-compliance.

**ROPE**
- ROPE -- A rope is a multi-threaded collection of miner node outputs (called "fibers") where each fiber is derived from the "heart beat computation" within each mining node.
- ROPE THICKNESS -- The rope thickness is an natural number indicating the expected number of previous fibers each new fiber depends upon.
- DAG -- Thus the rope itself is a DAG (directed acylic graph) of fibers where each fiber lists the prior fibers it depends upon.
- MINER HEARTBEAT -- The rope is created from each miner executing a 'heart beat' operation.  A full heartbeat cycle includes:
	1.	Sorting received fibers by "weight" and select the top n fibers
	2.	Building a merkel tree of the top fiber hashes
	3.	Executing as much proof of sequential work as possible within fixed time window
	4.	Publish the created fiber to all peers in the miner network

**FIBER**
- FIBER -- A fiber is the output of the "heart beat computation" performed by a mining node will create a publish a new fiber from that node.  Each fiber depends upon many other fibers previously published by this an other mining nodes.
- FIBER LINEARITY -- No matter what, a mining node with integrity, should never fork its fiber line.  It will only produce and publish a SINGLE direct descendent for each fiber it publishes.  Publishing two direct fiber descendents for a single fiber can be proven by any receiver of these two fibers and can be used to provably:
	1. Eject the mining node from the network, and/or
	2. Take or destroy it staking tokens, and/or 
	3. Receive its penalty escrow as reward for identifying the offending miner.

THREAD
- THREAD -- A thread within the rope is a sequence of fibers that each depend upon the prior fiber.  (Notice there are an exponential number of threads since at each level each fiber depends upon many others.)
- BEAT CHAIN -- A block chain computed by a sequence of chain nodes where each block is associated with some heart beat from the node that computed a block
- PATH WEIGHT -- The sum of its fiber weights



- FIBER WEIGHT -- The 'weight' of a fiber is a combination of the:
		beat strength, trust balance, average incoming weight.
- TOP FIBERS -- This is list of CHAIN WIDTH fibers known to a node at the beginning of a heart beat with the greatest fiber weight.

THREAD COMPARISON


HEART BEAT COMPUTATION
- 
- HEART BEAT COMPUTATION -- 
	1.	Sort received fibers by weight, and select the top 'rope thickness' number of fibers
	2.	Build a merkel tree of the top fiber hashes, starting with ones own prior fiber hash
	3.	Execute as many heart beat iterations as possible within 'heartbeat length' time
	4.	Publish the created fiber to all peers within the miner network
- published heart beat received from other nodes are called a 'fibers' -- they are what is put together in the merkel tree at the beginning of this nodes heart beats.
- HEART BEAT -- Each time a node a heart beat it builds a merkel tree of its "top fibers" and uses that to start computing a new heart beat, when it is completed it publishes the result of this new heart beat computation and repeats.
- FIBER LENGTH -- The amount of time for a given heart beat.  (Rope chain aims system incentivises a periodic beat of specified duration.  e.g. 10 seconds)
- FIBER STRENGTH -- The number of heart beat parts per second 



TRUST 
- TRUST -- Block chain is run by a set of node have been explicitly given 'trust' tokens by all holders of coins on the chain.  Thus nodes are likely to be publically known and may have reputations trusted both inside and outside the chain community.
- TRUST TRANSFER -- Each RC transfer specifies a target account for the transfer, and also may specify a 'trust transfer' this is a second account which will receive the same balance amount but as a separate kind of 'trust tokens'
- TRUST BALANCE -- Node has balance of trust, this is the sum of all trust transfers to the node over time minus any trust transfers to other nodes.
- TRUST FRACTION -- A number between zero and one indicating the fraction of all trust given to a mining node relative to the total trust for all outstanding coins.
- TRUST PAYMENTS -- kickbacks???
