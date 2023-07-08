
**Objective**: Cut through the noise of modern fact reporting by allowing individuals to go on record in a way that provides them with a valid track record.




## Approach


**Participant** - A *participant* is potentially anonymous individual associated with a published public key.
**Oracle** - An *oracle* is a participant who has agreed to publish answers for one or more questions as they come to know these answers.
**Pundit** - A *pundit* is a participant who makes claims about the answers that oracles will eventually publish.


**Message** - A *message* is a block-chain published text message signed by a participant.  Questions, Answers, and Claims below are all kinds of messages.
**Question** - A *question* is a precisely worded query about some factual but unknown or un-agreed upon claim about the world, with a precisely specified range of possible answers.
**Answer** - An *answer* is one of the possible responses to a question.


**Request** - A *request* is a question that is posed to one or more or oracles for subsequent answering, possibly with a modest associated bounty payment for accepting the question.
**Commitment** - An oracle may *commit* to answering a question by publishing a precisely worded version of a question (which may different from the request) and may indicate a charge for depending upon the answer.
**Claim** - A *claim* is computable function of the latest oracle-supplied answers to one or more questions.  The claim function will return "true", "false", or "unknown".  A claim is only valid when it is tied to appropriate payments for the oracular commitments it depends upon.  Each claim specifies a list of input oracle-question pairs, as well as a level of certainty the pundit has for this claim.
**Outcome** - Each claim has a *outcome* that is true, false, or unknown based on the oracular answers supplied up to this point in time.  Typically claims will begin as unknown and then shift to either true or false depending upon oracular answers, but in it possible to new information to come to light which would cause an oracle to publish answers that override their previously supplied answer.

**Support** - A participant may express *support* for a pundit or oracle, or the claims of a pundit or the answers of an oracle.  A participant may also express non-support as well.


**Record** - The *record* for a participant is the list of published messages from that participant.  For an oracle it is the list of commitments and supplied answers, and for a pundit it is the list of claims they have made along with their up to the moment outcomes.  In general answers and support may change over time and typically it is the most recent message that is attended by the derived measures.



### LEVELS

**Level Of Certainty (LOC)** - A level of certainty is a published rubric for specifying how certain a pundit is about a give claim or level of support.  One rubric might be
- Certain - 
- Very Likely - 
- Probably - 

**Level Of Support** - 
- Supported - 
- Tentative support - 


### DERIVED MEASURES

#### RELIABILITY - Number of claims judged to be true or false

Reliability(pundit, level) = Number of claims judged to be certainly true, very likely true, probably true, probably false, very likely false, certainly false according to their chosen oracles



#### SUPPORT - Number of other participants that support your integrity

Support(participant) - Amount of support a participant has from other participants


#### BALANCE - A measure the support one receives from 
Balance(oracle) - A quality weighted measure of the support received by pundits making conflicting claims.

PREPROCESSING ???
- Compute the 'sides' for all commitments of all oracles.  (these are sets of claimants that are making incompatible claims)
- Group commitments have greatest alignment in side sets to build a full partitioning of those claims with high alignment


Ratio of support from 
- basic idea:  percentile rank of the inverse of the ratio of support the oracle receives from groups
- should it penalize dramatic cases 

#### COHERENCE - 
Coherence(oracle) - A quality weighted, certainty-weighted measure of the degree of consistency 


#### HISTORY - The number of answers provided


#### QUALITY - Combines support, balance, coherence into a unified measure

Percentile rank of quality score for all oracles
- Sometimes broken into quality bands: Gold Standard, Solid, Good, Fair to Junk

Quality(oracle)



## VISUALIZATIONS


### Oracle Quality

A number from 0 to 99 indicating percentile rank along with a color gold, silver, bronze or red


### Pundit Claim Quality

A quick visual presentation of the quality 

- Three shades of green
- A gap of color
- Three shades of red

The top band is gold standard
The lower band includes silver and gold



## GROUPS

Coalitions of pundits, and coalitions of oracles can band together to create a collective pundits and collective oracles.

These pundits and oracles have computable rules for:

- INCLUSION of new members to the group.
- EXPULSION of members from the group.
- DERIVATION of a stream of consensus messages that form the record for the group.
- UPDATES to the groups computable rules.

## ANONYMITY

To avoid sybil attacks where an single individual pretends to be many people we generally require verifiable personal identification to be associated with each participant. This may pose a significant safety threat for an individual to have their identity tied to their claims and answers.

In these cases anonymous entries can be used.

STEPS
- An identity validator creates:
	- A public/private key pair is created for the individual
	- A 12-word random pass phrase is computed for the individual
	- A hash of the person's fingerprint is computed.
	- A hash of a long pass phrase is computed.
- After two different random time delays the validator will publish two signed messages:
	- The first is a certificate of authenticity for the public key.
	- The second is a validation message containing:
	  (1) the hash of the finger print, (2) hash of the pass code, and (3) public key encrypted using the pass phrase.
	- There should not be two distinct validation messages that have the same fingerprint. A validator wont publish second validation record with the same fingerprint as one already published, in that case it will delete this record and will retract the public key if that had already been published.

- Over time the number of each type of message published by a validator must approximately match, otherwise we know they are corrupt.

 
PARTICIPANT VALIDATION - Once an anonymous participant has existed for sufficient time we know it validation message is published to it can be validated:
	- A challenge against a given public persona (public key) is posted with a price bounty to be collected by the anonymous participant when the challenge is met.
	- The challenge also lists a set of requested validators
	- The participant chooses one of the requested validators (or another validator they trust) to prove their identity.
	- That validator uses the fingerprint to look up the participant record
	- The participant enters their passcode which is used to decrypt the public key to see if it matches.
	- If it matches and there is only one record with that fingerprint the validator publishes a verification of identity and transfers the bounty to the participant
	- if it matches but there are multiple records then a verification of the existence of an individual with a matching fingerprint (so the validator did not make up a person) but the uniqueness of the person cannot be assessed.

VALIDATOR VALIDATION - A validator create spurrious 