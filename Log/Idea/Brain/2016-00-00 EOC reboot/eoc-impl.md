

# STRAWMAN IMPLEMENTATION PROPOSAL
## OUTLINE OF LAYERS -- 
OUTLINE of the meta-meta implementation
-- BACKGROUND ENCODING LAYER          --  Defines the underlying encoding data structures  
-- BACKGROUND STANDARD TERMS LAYER    --  Defines many commonplace terms from other programming paradigms systems
-- BACKGROUND SEMANTICS LAYER         --  Defines core sematic elements of the PPP processor
-- BACKGROUND EMBODIMENT LAYER        --  Defines the components needed to embed the PPP within a world that it can sense and manipulate
-- LEARNING LAYERS

  -- prerequisite embodiment
- categories of learning
- Positioning existing ML within the larger hll
- Enumerating many HLLDs
- Requirements for bootstrapping 
  - K composibility
  - K Triangulation

- Requirements for autonomy
  
#### TL;DR for the background layers.

- The 'UNI-FORM' is a python function call   fn(val1, val2, ..., key_a=value_a, key_b=value_b)  where values
  are either recurive uni-form expressions, or is a number.
- UNI-FORM is homoiconic like LISP but it build from a subset of JSON (or LISP) that only grounds out in numeric literals.
- UNI-FORM has a templating typing system that can constrain all variables using a sophisticated recurive typing constraint system
- All parts of a UNI-FORM expressions can have weights or probabilities associated with them (even the parts of a typespec expressions for example)
- The programmable population processor is embedded into a real or simulated world by having a set of input features computed by the world, and a set of output features that control actuators on the embodiment within that world.
## BACKGROUND "HOUSE KEEPING" LAYERS
### BACKGROUND ENCODING LAYER

In order to frame later discussion it is useful to define two information types, data and knowledge.

**DATA** -- A sequence of activation sets, where each activation set is a set of feature activation levels.  A feature activation is the pairing of a floating point activation level to a feature token, where a feature token is some durable identifier used to refer to this same activation level across other activation sets.

**KNOWLEDGE** -- One or more elements from the **UNI-FORM LANGUAGE**.  The uni-form language is set of uni-form expressions -- recursive compositions of numbers and the 'uni-form' itself.  Formally, the uni-form is expressed as:      head( VALUE_1, VALUE_2, ..., key_a=VALUE_A, key_b=VALUE_B )     where head and key_a, key_b, ... are drawn from some enumerable set of identifiers, and VALUE_1, VALUE_A, VALUE_2, VALUEB, ... are either floating point numbers, or are sub-uni-form expressions. 

**EXECUTION** -- Uni-form expressions define a simple form of computation.
-- Uni-form execution is the mapping of a uni-form expression, E, and execution context, C, onto some resuting form R and an updated context C'.  Each of these parts, E, C, C', and R are elements of uni-form.
-- Execution is dictated by the 'head' identifier of the uni-form expression.  EXECUTOR(head, C) maps some subset of identifiers 'head' onto an executor functions, EXE.  Each executor function, EXE, accepts an expression, E, and context C, and returns a result R.  It may optionally modify the execution context to produce a new execution context C'
-- Execute(E, C) maps to R and updates C to C'   when   E==head(...)   and   exe,C'==EXECUTOR(head, C)   and  R==EXE(E,C), otherwise
   Execute(E, C) maps to E and C remains unchanged.
-- In the first case E is called a FUNCTIONAL FORM, and in the second case E is called a CONSTANT FORM

**IMPLICIT KNOWLEDGE** -- The information encoded in the EXE functions returned by the EXECUTOR function is call implicit knowledge.  This distinguishes this information from the explicitly encoded information that we are calling 'KNOWLEDGE' above.



--- DISCUSSION ABOUT UNI-FORM ENCODINGS ---

**RELATING UNI-FORM TO EXISTING LANGUAGES**.  Uni-form execution over JSON expressions is much like LISP evaluation except LISP operates over cons structures.     Uni-form execution is also analogous to, but more general than, lambda caluculus since it does not define any fixed function application semantics.  For example we can define a 'while' form, a uniform-expression whose head is 'while' that when executed, will in turn execute it first arguemnt and then iteratively execute is the remaining body elements repeatedly based on results returned from repeated execution of the its first argument.  

**UNI-FORM GENERALITY**.  Lambda-style function application can naturally be expressed as uni-form execution, as can every modern procedural langauge.  But uni-form expressions themselve do not fully specify any proecdural lanaguage since it does not specify the semantics employed by the head implementations, nor does it specify the variables, datastructures, etc. employed by those languages -- all of that information is only implicitly encoded within the 'context' datastructure and in the EXE implementations.  Thus, Uni-form and it execution model can be thought of as a language of 'structure' over which one can define many different representational and computational systems.  

**INTENDED USAGE OF DATA AND KNOWLEDGE FORMS** --  In the embodied intelligent agent context we use DATA in many forms to represent 'raw' data from sensors, data sent to actuators, historical 'training' data. etc.  In some contexts successive activation sets within DATA are intended to be interpreted as temporally ordered accounts of sensor/action states, in more classical inductive contexts we may use DATA to encode elements drawn iid from some distriubtion to be modelled.  KNOWLEDGE by contrast is never directly expressible outside of the execution of an embodied intelligence.  It is used in various forms to encode models learned from data, is used to control the execution of those learning processes, and is used to encode all behaviors of the embodied agent.  Knowledge is the language of structures latent in the structure of the embodied agent's processor.

### UNIFORM ENCODING OF KNOWLEDGE AND DATA

**UNI-FORM** -- The recursively defined set of all uni-form expressions.
**NUMBER** -- **NUMERIC EXPRESSION** -- An element of uni-form that is a floating point or integer number.
**STRCTURED FORM** -- An element of uni-form that is not a numeric expression -- i.e. it has at least one structured component.
**ROOT** -- The 'toplevel' outter most structured form in some structured form.  All structued expressions have one root form:  head( value_1, value_2 ..., key_a=value_a, key_b=value_b, ...)
**EXPRESSION PARTS** -- When refering to the the parts of a strctured expression, we are implicitly referring to the parts of its root form.  So a structured form, E,  will have a HEAD, it may have a first, second, ... expression, and make have expressions associated with each possible 'key' identifier.
**FUNCTIONAL FORM** -- A structed expression whose HEAD maps is mapped to a non-NULL value by the EXECUTOR function.
**CONSTANT** -- An element of uni-form that is not a functional form.
**STRUCTURED CONSTANT???** -- A constant form that is also a structured form, e.g. is has a root form, but its head function is not defined or returns itself.
**FIXED ARGUMENTS** -- the value, value_1, value_2, etc. within the root of a structured uni-form as defined above.
**KEYWORD ARGUMENTS** -- the values value_a, value_a, etc. within the root of a structured uni-form as defined above, whose keyword identifer does not begin with '^'
**META-KEYWORDS ARGUMENTS** -- values associated with keywords whose identifer begins with '^' character.  (these special keywords are used by convention to encode information that should not be processed in the same way standard keywords are processed.)



### BACKGROUND "STANDARD TYPES" LAYER

Many useful programming idioms commonplace in modern imparative programming can be straightforwardly encoded as uni-form knowledge.  This laborious exercise is necessary in order for us to define the adaptive meta processes that operate over these forms.  Here we provide precise definitions for these intuitive and commonly used terms in the uni-form context.  The reader might skim these definitions now, and refer back to them as needed as they are used in later sections.

**EXPR** -- **UNI-FORM EXPRESSION** -- An element of uni-form.
**LIST** -- a list is a structured form whose head is the identifier 'list'.  The executor associated with 'list' will return the passed execution form as a constant will, but returns and error form if any keyword is provieded.  (meta-keywords may be provided w/o error.)
**MAP** -- a map is a structured form whose head is 'map'.  Analgous to 'list', it returns its constant value in the case that no fixed arguments are specified, else an 'error' structure is returned.

--SPECIAL STRUCTURED FORMS USED IN EXECUTION--
**SYMBOL** -- `Sym("namestring")` -- A symbol is a structured form whose head is 'Sym'.  Formally a symbol, S, is a structured form "head()" where Executor(head)==exe and exe(E, C) always returns "head()" for all E and all C.  In printed form, for textual simplicity we drop the '()' suffix for symbol forms.  As an example, `foo( alpha, beta, gamma )` is a uni-form expression with three symbols as arguments: Sym("alpha"), Sym("beta"), and Sym("gamma").  Depending on the context symbol can be used as a indicator of one of N scalar alternatives, in other contexts symbol are used as a variable placholder.
**NULL, TRUE, FALSE** -- Three symbols which map to corresponding JSON values.  In some interpreter contexts these symbols have specific semantics.
**ERROR** -- A structured constant form whose head is 'error'.  this the result for an execution that fails -- typically its arguments tell you some details about the failure.


**????ALT EXPRESSION???** -- An alt expression (weighted expression) is a structured form where numeric weights are associated with each value and each keyword.  alt( .3~111, 1.5~222, 22~foo=333 ) is an example of a weighted expression with three weights .3, 1.5, and 22 associated its values and keys.  At the JSON level this is encoded by simply having a second structured form under the '^weights' key of the first expression.  So the JSON for the example above is   alt(111, 222, foo=333, ^weights=w(.3, 1.5, foo=22)).
Semantically a weighted expression defines a non-normalized probability density funcition (where weights do not need to sum to 1.0). 

fixedargs

--FUNCTIONAL FORMS--
**N-ARY FUNCTION** -- A functional form that operates over execution contexts with no keyword aruments and exactly N fixed arguments.  (Note: N-ARY exuction contexts may have meta-keyworks, just no defined keywords).
**UNARY FUNCTION** -- An n-ary form where n=1
**BINARY FUNCTION** -- An n-ary form where n=2
**THUNK** -- An n-ary form where n=0

**NUMERIC FUNCTION** -- A functional form that always returns a numeric value.  Also called a 'metric' in certain contexts.
**SIMPLE INDICATOR FUNCTION** -- A unary numeric funtion returning either 0.0 or 1.0
**WEIGHTED INDICATOR FUNCTION** -- A unary numeric function returning a number between 0.0 and 1.0 with the intent that intermediate values


**STANDARD OPERATORS** -- The 'EXECUTOR' function maps many terms onto their expected computational embodiments. Including:  max, min, +, -, *, /, etc.  all map to their expected mathematical functions.
if, while, for, etc. all map to their expected imparative programming implementation.
(If using LISP these can simply be the LISP functions, for JSON there is not as readily availalble implementation)


**SYNTACTIC SUGAR** -- For convenience a simple parser / printer may be employed to map textual forms with embedded infix operators
into a uni-form expression.  So  c^2 == a^2 + b^2  might map to   '=='( '^'(c, 2), '+'( '^'(a, 2), '^'(b, 2) ) )



---SPECIALIZED-STUFF---


**TYPESPEC** -- A TYPESPEC is a uni-form expression which is also an indicator function, T, used to define a representational sub langauge, S, where S is some subset of the set of all UNI-FORM expressions.  Formally S == { k for all k in UNI-FORM LANGUAGE where I(k)=1.0}.  In the case that I is a weighted indicatior function, then it encodes a weighted subset of the UNI-FORM LANGAUGE where each element, k, in the sub-language, S, is associated with some weight, w>0, where w=I(k).  We refer to a typespec with a weighted indicator as a soft or weight typespec in order to indicate that its membership is not absolute.  In the case that the weighted indicator function denotes the probability of set membership, then a weighted typespec would be a pdf over uni-form expressions.  But in other cases no precise formal semantics maybe associated with the weightings.  Still the intended usage is that larger values of the indicator function indicate elements that are 'more' indicated, 'more' important, 'more' represented in the data, 'more' worthy of exploration, etc.  they are 'more' of something where that 'more-ness' is implicit in the processes that are executing over the typespec.


**TYPESPACE** -- A TYPESPACE is the set of expressions indicated by a TYPESPEC.

tspec( TYPESPEC )       // wrapper used to indicate a TYPESPEC expression
none                    // does not match an expression of any type
any                     // same as 'expr' it will match any value
int                     // matches any number without a decimal expansion
number                  // matches any number
symbol                  // matches any symbol function
list( VAL_SPEC )        // matches any uni-form with a head 'list' and only fixed arg
// matches a structured form with given HEAD, and matching arguments.  fixedargs and keyargs 
// are the specs used for other fixed or keyword arguments.
obj( HEAD_SYMBOL, SPEC1, SPEC2, ..., key_a=SPEC_A, key_b=SPEC_B, fixedargs=SPEC, keyargs=SPEC )
// matches a function with the specified signature.  'result' specifies the typespec for the function's result.
fspec( SPEC1, SPEC2, ..., key_a=SPEC_A, key_b=SPEC_B, fixedargs=SPEC, keyargs=SPEC, result=SPEC )


#### details -- the typesystem's structure defined using typespec

WRAPPER == tspec( TYPESPEC )

TYPESPEC == alt( none, any, int, number, symbol, LISTSPEC, OBJSPEC, FNSPEC )

LISTSPEC == obj( list, TYPESPEC, fixedargs=none, keyargs=none )

OBJSPEC == obj( obj, fixedargs=TYPESPEC, keyargs=TYPESPEC )

FNSPEC == obj( fn, fixedargs=TYPESPEC, keyargs=TYPESPEC )



### BACKGROUND "AI" LAYER

**TASK** -- A fully specified uni-form execution that could be performed.  Formally an execution is defined by some uni-form expression E, and an execution context C.

**PROCESS** -- Task that unfolds over time (e.g. with data in and out) <<<<<<< FIX <<<<<<<<<<<<<<<<<<<<<<<<

**OBJECTIVE FN** -- A unary, numeric functional form.  Given an objective function, F, and two expressions, E1 and E2, F(E1)>F(E2) means that E1 is a more preferred result for the search task defined by F.



**KNOWLEDEGE COMPLEXITY** -- Some metric over uni-from expressions.  Intuitively larger values of this metric indicate greater complexity.  Counting the number of literals within an expression would be a measure of complexity, the minimum number of transform operators to generate expression given a set of transform operators might be another (harder to compute) measure of complexity.


**SPACE COMPLEXTTY** -- A metric over uni-form typespec.  This is a measure of the complexity of represtnational spaces.  MinimumDescriptionLength is one formualtion of space complexity. VC-dimension is another measure of space complexity.  Intuitively this measure captures some notion of the difficulty in doing operations over this TYPESPACE.


**COPMPARABLE KNOWELDGE** -- Two uni-forms, k1 and k2 are said to be comperable according to some mapping space, M, if there exists m1, m2 in M such that k2=m1(k1) and k1=m1(k2).  In the case that M is a weighted typespace then it defined a degree of comparability of k1 and k2 defined as the min( M(m1), M(m2) ).
(recall a typespec is an indicator function over uni-form expressions)





### EMBODIMENT LAYER

??? where ???

**UTILITY** -- a binary numeric function accepting a list of knowledge forms, and execution context which include some data over which the utilty is computed.

**TARGETED CAPABILITIES** -- things that are useful to learn

>>>> note: current formulation does not express other agents and there exectution explictly, but it should.
>>>> connecting 'utility' to 'objective' function as applied to a single knowledge form.



**WORLD** -- An actual or simulated universe that derives a succession of world states based past world states and on the actions of agents embedded within the world.

**AGENT** -- A actor within the world.  Actors can sense the world, and can take actions which affect this succession of world states.

**EMBODIMENTS** -- Those agents that are each controlled by an instantiation of the PPP system.  In some contexts we may refer to one of the embodiments as the STUDENT AGENT.

**PEER AGENTS** -- Agents that that exhibit targeted caps   <<<<<

**TEACHING AGENT** -- An agent that acts in a goal directed way aiming at <<<<< student agent in a goal directed way <<<<<<<<

**THINKING** -- A process outside of the world itself which an agent performs to determine its actions.  Thinking may depend on the sensed world, but can also depend on things entirely outside of the world.

**INPUT FEATURES** -- PPP FEATURES whose activation is computed at any point in time based on sensors within the WORLD

**OUTPUT FEATURES** -- PPP FEATURES whose activation at any point in time drives 

**MENTAL FEATURES** -- FEATURES that are a kind of INPUT FEATURES for the PPP, but these features sense the state of the PPP computational implementation itself rather than the sensed state of the world.  So a counter of the number of function invocations executed since birth (how much thinking had been done) could be one such sensor which could potentially allow the embodied agent to note its thinking was going in circles.




### HARNESS



## PPP

**???DATA BINDINGS** -- Special uni-form symbols which provide access to the DATA layer within the KNOWLEDGE language.  Each data binding symbol of cooresponds to single features within the DATA layer.  The following functions are defined over features symbols in order to access and control their execution:
-- 'activation' is a numeric function that accepts a feature symbol, and returns it current activation.  
-- 'activate' accepts a feature symbol and an activation level and 'sets' the current activation of that feature to the new level.  
(NOTE: the consequence of this setting will vary depnding on the feature and upon the embodiment.  
-- IMPLICIT KNOWLEDGE FEATURE FUNCTIONS -- Other feature functions may also be defined for certain EMBODIMENTS.  For example 'history()' might provide access to some limited list of historical activation values.  or adjacent(feature1, feature2) might express IMPLICIT KNOWLEDGE about the relationship between features.  For example, the 2d-spatial layout of features encoding an embedded agent's vusual system might be encoded using the feature function adjacent(feature1, feature2)

-- 'STATE' -- an expression that records the moment to moment state of the learning.
-- 'CONFIGURATION' -- an expression that records results of learning
-- 'INPUTS' -- a list of features that are used to compute the activation of this feature
-- 'FN' -- an optional numeric function that computes the activation of a feature given its state, configuration, and the prior activation of its input features.

**FEATURE** -- The complete state of a feature.  Each can be encoded as an instance of FEATIRE.tspec

FEATURE.tspec = 
tspec( obj(
    feature, 
    pop=POPULATION,                          // the population this features belongs to
    fid=integer,                             // unique integer id for this feature 
    connectivity=list(int),
    activation=number,                       // the current activation level for this feature
    history=list(number),
    state=expr,                              // internal state of this feature  (perhaps this should simply be list(int))
    configuration=expr,                      // the result of learning
))



**POPULATION** -- A group of features that have indentical genetic endowment.  

tspec( obj(
    pop,
    activation_fn=ACTIVATION_FUNCTION        // Function used to compute the activation of any feature within the population 
    configspec=TYPESPEC                      //  Hypespec
    statespec=TYPESPEC

    
    
)


**ACTIVATION_FN** is a function of STATE1, STATE2, ...,  INPUT1, INPUT2, ..., and CONFIG
tspec( fn(fixedargs=number, state=expr, config=expr, result=number) )



=========

**SNAPSHOT** -- A snapshot is a mapping of some set of features onto an activation level for each feature.  NOTE: a snapshot is a recording of activation levels without regard to whether they are input or output features.  

**EXPERIENCE** -- is a sequence of snapshots.  In some cases these snapshots are to be interpreted as having occurred temporally in order in the world.  In other cases shapshots simply represent distinct experiences w/o any implied temporal ordering.  (Each snapshot has a boolean flag called 'chained'.  When true it signified that this shapshot is to be interpreted as having occured temporally immediately after the preceding snapshot within the EXPERIENCE.

Note: we use the term 'experience' since a key usage of experience is to encode data obtained from the embodiment of an agent.  In other caseses however the 'experience' a learning algorithm is exposed to is 'experience' internally derived from other computation.  It makes no different to the receiving learning algorithms, in both cases it represents experience presented from outside over which it is expected to process / learn.



**SNIPIT** -- a short sequence of snapshots that are chained.  A snipit is 'short' in the sense that the system does not attempt to temporally organize within the timeframe of the snipit, though it may operate on concepts that depend on the order of the snapshot chaining.

(for example 'increasing' is a concept that does not encode temporal relationships within the snapshots, but it does depend on their ordering)


## THINKING / LEARNING MODULES

C-TREE


## CATEGORIES OF LEARNING -- LEARNING DRIVES

**LEARNING**.  A Learning is an execution task which produces as list of acquired knowledge forms, A=list(k_1, k_2, ...), as a function of some data, D, where L seeks to optomize some objective function, O, over the data.  Specifically L seeks to maximize the utility of A given D.

Formally L = task( learn_alg( ...configuration..., dataspec=TYPESPEC, knowledgespec=TYPESPEC, objective=NUMERIC_FUNCTION ), ctx( data=DATA )  )   where the result of task execution R = list( k_1, k_2, ... ) such that objective( k_i, DATA ) are maximized in some way.  (could be the average is maximized, coverage of all DTATA is maximized, the maximial element is maxized, etc.)

Equivelantly a learning task is an execution that seeks to return results of high utility (see definition of utility above).


Within the learning context we define three types of learning:  Discovey learning, Emulative learning, Collaborative learning


---**DISCOVERY LEARNING**--- makes the least assumptions about the learning context.  Data is assumed to be generated by some process, learning attempts to select a element from model class which optomizes some objective function.  All of classical inductive learning, all optomization algorithms, and all serach algorithms can be encoded as a discovery learning.   (Discovery learning is where most (but not all) effort has been in machine learning over the last decades.)


---**EMULATIVE LEARNING**--- is discovery learning where it is assumed that the provided DATA results from some execution that is based in part on some target knowledge T = list( t_1, t_2, ...).  The objective of emulative learning, like discovery learning is to produce results with high utility.

Additinally it is expected that knowledge forms c_1, c_2, ... which are comparible to the targeted knowledge t_1, ..., t_n used to generate the data will turn out to have high utility.  Thus specfic forms of emulative learning directly seek to feret out targeted forms of knowledge based on data that results from them.

There is no universal emulative forms of learning.  Each eumlative form of learning must make some kind of assumption about how targeted knoweldge is employed during the execution used to produce the avaialble data.  Still in the case that those generation assumptions are valid, the we will show they can provide great constraint, allowing eumlative learning to acquire knoweldge much greater in complexity as compared to what is possible with discovery learning alone.


---**COLLABORATIVE LEARNING**--- is a form of emulative learning where both the generator function, and the learning functions are constructed in a way to yield utile output from learning execution.  Collaborative learning opens up a wide vista of paired social processes that aim to efficietly transfer knoweldge from one agent to another.

---**ATTENTIONAL COLLABORATION**--- is a simple form of collaborative learning where the generator function's guidance of the learning function is limited to attentional cue.

---**SITUATIONAL COLLABORATION**--- is a more complex form of collaborative learning where the 

---**REPRESENTATIONAL COLLABORATION**--- is an advanced form of collaborative learning where the generator and learning function seek to establish a new shared communication langauge.  Such collaboration is advanced in the sense that once the mapping of this language onto data is comparably mapped by both agents, it can be used to very effectively transmit arbirarily complex expressions between these agents.

Algebra as taught in high school is an example of representational collaboration.  it is a reresentational system known to the teacher that through representational collaboration becomes known to the learner.

Human langauge itself is an example of representational collaboration.  Mommy and daddy very explicitly endevor to expose baby to deveopmentally appropriate expressions of language in order to that baby might obtain facile use of that lanuage.  Baby is hard wired to attend to places where mommy and daddy look, is hard wired to try to reproduce behaviors (actions, speech etc.) that mommy and daddy do.  Baby is hardwired to seek acceptance of mommy and daddy.  Baby is hardwired to seek after certain thingsa and to fear or avoid others.

All of these behaviors in baby are understood my mommy and daddy and are used with great effect to arrive at a form of representational collaborative learning that results in knoweldge of complexity far far beyond the complexity that is can be learned by discovery.




So the moral of this story is to express current formulation of Machine Learning the full context of human learning.  To see to that the assumptions we typically make oabout the context of our learning algorithms are equivelant to operating with both of poor learners hands tied behind its back -- no ability to affect and experiment with the world, its feet bound around its head -- no ability to move to radically different contrexts where learning might be more asspicious.  In hard world with no benevolant mommy with who knows the targeted knowledge and uses it to guide baby.  indeed on a Desserted island where there are no agents benevolant or not that baby might emulate in his or her leraning.

The one signal we typically provide our learning algorithm is a simply 'plus' / 'minus'   'hot' / 'cold'  'pleasure' / 'pain' signal to guide it toward utile knowldege and away from garbage.

Most of machine learning today, even the most advanced forms of deep learning are equivelant to putting a caluclus problem in front of Johnny the learner, and giving him a bit of cake when his scrawls on emppty paper are correct, and a smack of the ruler in the cases where his scrawls are not correct.





#### DISCOVERING  VS  COPYING  VS  COLLABORATING

INTUITIVELY 
  discovering is like supervised induction or optomization typically 
  copying is a kind of discovery that relies assumptions about the agent generating the data
  collaborating


- Search, induction, and optomization can all be expressed as the selelection of some 'value' (a hypothesis element) from a defined set of possible values, 
  using some defined process, aiming to maximize some computable objective.
- The process and objective functions are defined in terms of some training data -- the intent is that the selected value (hypothesis) can be used to 
  make predictons about future data.
- For concreteness, at the base level, let us assume that data is expressed as a sequence of sets of feature activation (numbers between 0 and 1), and values
  (hypotheses considered) are sets of functions that compute one feature activation as a function of other activations.
- For concreteness we assume that the objective functions and the processing functions are expressed as uni-form values given some set of predefined functions.

- A 'discovering' process makes no further assumptions about the source of the data other than it is drawn "randomly" (iid) from some distribution that we 
  hope the discovering process 'learns' to emulate, by accurately predicting output features as a function of input features over the entire distribution.

- A 'copying' is a kind of discovery process seeking targeted model, M, where it is known/expected that the training data is generated by a processess based
  on M' where M and M' can be computed from each other using simple transform processes.

- A 'collaborating' process is designed to facilitate rapid collaborative copying.  Collaborative copying 


Discovery is induction of some model, M, of one variable as a fucntion of another set of variables from 'blind-data' -- data that is 
drawn from a distribution that was constructed by some process not known, or expected to be based on some form of that knowledge 'M' itself.
Learning approximate models of physical phenomenon often fits this 'discovery' model.

Copying, by contrast is induction of some model, M, in a context where it is known or expected that the process used in generated the training data
directly relied on some model, M', closely related to M.  For specific copying techniques there is generally a distance function between knowledge forms
where acceptable resulting fucnctions should be 'close' to the underlying generative model, M'

Notice given a dataset it is generally no apparent if a task is a discovery or capying inductive task -- prior knowlege about the task is generally used to 
indicate if it is a copying task, and what form of copying it might be.





---------

## HUMAN-LIKE LEARNING DRIVES

### intro
     ???? dissonance, understanding, model agency, functional modelling.   achievement drive

We have framed a wide vista showing a range of learning that humans routinely engage in, and have shown that most of today's machine learning is all bunched up in the most difficult corner of human learning.

What might it take to build unified model of machine learning that incorporated many learning types from each of these classes of machine learning?
To answer this first enumerate many of theses drives, naming them, and describing in English how each appears to work in the human case.
Later we consider a learning substrate that appears sufficient for encoding them.

It should be noted that while the bulk of ML research and practice today fits into the discovery learning category, there has been nacent work in all of the learning drives we list below in the other categories of learning.  Here we try to express a unified framework that can simultaneously leverge the range of HLLDs that humans exhibit within one autnomous seed AI.


THE HUMNAN SEED INTELLIGENCE
It appears (to the author) that the human seed intelligence is an embodied processing system driven opportunisticslly by a bundle of dozens of 'drives' each of which is trigged in certain understandable circumstances, and appears to aim towards explainable outcomes.


A COMMENT TO THE SKEPTIC -- 're-naming' many existing areas of AI/ML and calling them all 'drives' does not make them more new, nor does it make them more uniform, so what is the point.  Several:
(1) one approach constructing a seed AI is to first build a learning substrat that can be specialized to yeild each of the learning drives below.  We don't have such a substrate, but by laying them out in this way we see what this substrate must do
(2) there is a uniformity inherent across each of these:  in each case there is an insight about the world, about learning, or about societal actions which can be encoded as a constraint to drive learning.  we propose enumerating and encoding as the primary agenda of creating a seed AI
(3) we already understand humans as possessing many of these drives and as teachers of humans we leverage this knowledge in their instruction.  By virtue of this we have more of a roadmap toward building as Seed AI than we realize we have.  Simply consider the known inclinaitons we leverage in humans in inducing learning in them, and then consider how to express that inclinaion as a 'drive' with a computational algorithms that instantiates some version of the drive
(4) critially closed-loop learning requires 'all knowledge in all ways' thus we need a uniform encoding of learning such that 'orignation-independence' in achieved.


### USEFUL INNATE DRIVES

FEAR -- 
FIGHT/FLIGHT -- 
ANGER -- 

### PASSIVE DISCOVERY

These drives most closely resemble existing machine learning, in that their inputs and outputs match today's learning inputs and outputs.  Both accepts a stream of EXPERIENCE and output models of that experience that maximize of objective over a space of possible models.

But even at this most basic level, expressing our system as a set of drives provides one with a rich space of combinations which result in interesting hybrid algorithms that SIMULTANEOUSLY leverage constraints that are nearly always found in isolation.

Even at this most basic level, we see this multi-drive opportunism can provide great advantage since advancement from each of these distinct drives can be tightly interleaved (or combinded) in ways that would not occur, if expressed as separated algorithms.

NOTE:  As ML reserachers dive deeply into one doamin of application, sometimes they have invented specialized combinations that do mirror combinations here, and also mirror contstrains of their problem domain.  But the PPP is easily and naturally programmed with such combinations.

### CATEGORY DRIVES

**CATEGORY DRIVES** -- These drives all related to the creation and maintaince of categories that somehow model the system's EXPERIENCE.  Formally, each category in maps a SNAPSHOT onto a score denoting how well that snapshot matches the given category.  (In some cases this function depneds on a limited 'prior window' of SNAPSHOTS within the EXPERIENCE.


**GROUPING DRIVE** -- the drive to partition EXPERIENCE into n distinct categories where each category is as different as possilble and where experience wthin each category is a similar as possible.   (NOTE:  clustering algorithms implement this grouping drive.  NOTE: this drive is all about the "distance" between elements of experience within and across categories.)

???  distance function attends to subet of features overwhich grouping-ness is being achieved


**SEGMENTATIONAL DRIVE** -- the drive to partition SNAPSHOTS of EXPERIENCE into n categories in order to match some external signal or fitness function....  (NOTE: supervised induction implements this drive.  NOTE: this drive is all about the relationship between elemnts of experience and some external supervisory signal.)

???? external signal or fitness function.
minimizes entropy of mapping of category to signal


**TAXONOMIC DRIVE** -- The drive to create subsumption hierarchies among the created categories.  This drive can be implemented by creating new categories, or by shaping modifying existing categories.

**EXCLUSIVITY DRIVE** -- The drive to create categories (within the taxonomy) that are distinct from their peers.

**COMPLETENESS DRIVE** -- The drive to 

**STABILITY DRIVE** -- The drive for categories to remain constant over successive SNAPSHOTS of EXPERIENCE.

### MODELING DRIVES

**MODELING DRIVES** -- the drive to produce a derived representation of experience which is in some way approximately optimizes some criteria in it selection of from some representational space.   

Specifically a modeling drive, D, will produce a model, M, given some experience, E, and some fitness critera, C. In this case we say that model M, is drive, D's, REPRESENTATION of experince, E, given fitness critera, C.

XXXXXX   >>>>  (All induction, clustering, and optomization fit into this broad algorithmic characterization.  Indeed the CATIGORIZATION DRIVE listed above are a specific form of modeling drives that were described as a group since their input/output structures align in a way that they can be combined in to hybrid algorithms that arbitrarily compose their constraints into an algorithms that is simultaneiously constrained by any arbitrary cobination of their constraints.)



**ENSEMBLATION DRIVE** -- the drive to represent each snapshot, snipit, or slice within some experience as a subset of some ensemble set.

**RELATIONALIZER DRIVE** -- 
prototyper


===**NON-CATEGORIZING MODELLING DRIVES**===

**SEGMENT STATE** -- Partition experience into buckets where instances within same bucket are similar in some way
**SEGMENT CHANGE** -- 
**PREDICT "HAPPENS NEXT"** -- 

**OBJECTIFIER** -- Reimagine experience as a story about objects resulting in that experience.
**OBJ-IDENTIFIER** -- 
**OBJ-PROPERTIZER** -- 
**OBJ-CHANGE-RECOGNIZER** --

**STRUCTURE-BY**
- BY TIME
- BY PART


**ACTIONATION DRIVE** -- the 

**CAUSALIZER** -- conditional "happens next"

**CONTEXTUALIZATION DRIVE** -- the drive to group 











## ACTIVE DISCOVERY -- AUTNOMOUS DRIVES


**PLEASURE SEEKING / PAIN AVOIDING** -- 

**ANAL RETENTIVE DRIVE** --

**FRUSTRATION RESPONSE** -- 

**PLAN** -- 
  FORMULATE GOALS
  FOLLOW SUB-GOAL CHAINS


**ATTENTIONAL ACTIONS**
  - put eyes on something
  - spend time around somehting
  - think about something (try to model/predict all aspects of it)

**ATTENTIONAL STRATEGIES** --
  FOLLOW SOUND
  CURIOSITY - 
  BOREDOM -- 


## AGENT EMULATION DRIVES

**DO WHAT DADDY DOES** -- Drive to emulate

**FOLLOW THE HERD** -- 

**ATTEND TO WHAT OTHERS ATTEND TO** --


## COLLABORATIVE DRIVES

**TO SEEK APPROVAL** -- 




(this section is not current?  2015.03)
## META META JUSTIFICATION
### JUSTIFICATION
#### Which parts are cool and different?


What are the cool ideas here that make this framework worthy of exploration?

- a PPP which can mix and match constraints typcailly found is isolated algs
- multi-drive opportunism
- meta-meta auto feature seting
- rich signal feedback

#### Why does this framework have such different outcomes?

What is the 'magic sauce' that makes this framework do what others cannot?
- It is not one thing, rather it is a small combination that work in concert.
Many of these parts have previously been identified, and explored in isolation,
We argue the first seed AI will simultaenously possess all, and use them in concert to acheive liftoff.

We invent our own terms for capacities that can be achieved in multiple ways, but then immediately tie those terms to AI technologies that today provide instances of that capacity.


HUMAN COMPARABLE INTELLIGENCE -- set of capacities that many humans can achieve given sufficient time, and ideal growth context.



SEED SYSTEM -- conjunction of a seed ai, and a sufficient environment
-- SEED AI -- A system that can bootstrap to become an intelligence roughly equivelant to a human given an appropriate growth environment.
-- HUMAN COMPAIBLE SEED SYSTEM -- ensemble that results in an agent with human comparable intelligence


APARENTLY SUFFICIENT INSTANTIATION -- An set of capacities that if achieved individually, interlock to create a SEED AI, coupled with an environment that is sufficient for driving this seed to human comparable intelligence.


-- BOUNDED-UNIVERSAL IMITATION -- Effective exposure to a concept or capacity results in acquisition.

-- REDUCTIVE TRANSFER --

-- BOUNDED CONSTRUCTIVE TRANSFER -- (ugh) there exists a complexity bound underwhich all targeted concepts and capacities can be acquired thru a sequence of steps all of which are below the posited complexity bound.









-- REPRESENTATIONAL GENERATIVITY -- Capacity to tractably create new representational systems which are not constrained by underlying rep
   -- Non-parametric learning
   -- Proto-type learning
   -- Infinite VC-dimension
   -- Can any new concept class w. greater and greater fidelity w/o end

-- EMBODIED
   -- PHYSICALLY EMBODIED
   -- SOCIALLY EMBODOED

-- NON-DIMINISHING 
   -- ORIGINATION INDEPENDENCE -- 


multi-drive opportunist


An aparently sufficient adaptive processing substrate
-- non-parametric learning
-- non-diminishing 
   -- independent of starting point
-- compositional
-- triangulating

### CONCEPTS

CAPACITY -- A behavior pattern in an agent that meets some defined criteria.  
The capacity is defined by the set of environments which are auspicious for 
execution of the capacity, and a predicate specifying whether a particular 
behavior pattern exhibited by an agent in a specfic environment represents 
the execution of the capacity within the presented environment.

NATURAL CAPACITY -- MDL is much smaller than existential expression of behavior.

OBSERVED CAPACITIES --

NATURAL ANTECEDENT SETS -- A set of capacities is a natural antecedent set for some target capacity if agents generally do not possess the target capacity until after the agent possesses all of elements of one of its natural antecedent sets.

AUSPICIOUS CONTEXT -- A context where agents that do not possess a target capacity, but do have its natural antecedents will tractably acquire the target capacity.



LEARNING A CAPACITY -- An agent has learned a capacity if can exhibit the capacity
in contexts that are auspicious for exhibition of the capacity.


BOOTSTRAP COPIER -- An agent is a bootstrap copier if it is exposed to an 
environment where it has 






# Original stuff ---- NOT PART OF CURRENT DRAFT --- IGNORE
### Overview Writeup

Reach down with both hands, grab those boot laces.  Now pull hard... really, really hard.
META-META -- The framework for expressing, learning, and using the representations which we are describing here.


While the goal of most inductive learning is knowelge discovery, the goal of meta-meta is knowlege 'copying'.
Data available to the copying system is known to have been generated by an external agent using knowledge that the copying system 
is trying to recover.





### Layers Overview
OBJECTIVES
- OBJECTIVE-GENERALTIY  Bootstrap nearly everything from nearly nothing.
- OBJECTIVE-TAYLOR      Able to express hypothesis ?very? flexibly based on knowlege available 
- OBJECTIVE-ALG-EMBED   Able to embed other algs
- OBJECTIVE-K-ENG       Able to express common feature engineering in hyp
- OBJECTIVE-FAST        With constant factor slow down from most efficient implementation possible



APPROACH
- NOTE:  Search/optimization/induction can be viewed as selection of an element from a set according to constraints.
- NOTE:  Type grammars (from programming languages) are used to express very complex constraints used to define sets of structured values
- IDEA:  We can express learning as selection over values defined by a probabilistic, structured, recursive type system.
- IDEA:  We can encode range of processing as by defining specfic learning problems using this grammar + plus specfication of the selection process.
- NOTE:  This affors a uniform substrate for encoding many HLLDs
- NOTE:  This allows xfr of non-parametric 'emergent' knowledge from one HLLD for use by another
- NOTE:  This affords framwwork where meta algs can be expressed directly in same substrate.
- NOTE:  since learning alg, and meta algos for controlling them are both expressed homoiconally we can express meta learning strategies of n-th order directly 
         within framework.
- CLAIM: 

- Express induction/optimization/search as selection within some knowledge-defined type grammar.  (called a population)
- Express highly constrained optimization tasks as programmed networks of these populations  (called the programmable population processor)
- Implemnt multiple HLLDs using the PPP
- Implemnt triangulation strategies using the PPP
- program the PPP so 




(1) UNI-FORM DATA LANGUAGE
    - homo-iconic
    - probabilistic  -- (as in: ``maybe the code has an array here, or maybe that is a recurisve subfunction, or it is just a constant assignment?)
    - language of values

(3) FEATURE LANAGUAGE -- base langauge of learned connections
    - "non-parametric"   sub-symbolic
    - allows multiple primal drives to be encoded into the structure of the features so that 
      subsymbolic knowledge gleen from one can be used in another

(2) CONTROL LANGUAGE -- langauges of types + langauge of execution
    - strongly typed -- (as in: ``maybe this variable is restricted to values that yield f(x)<25 when function f is applied, or 
      maybe it is actually a list of objects from the union of these three classes'')

(4) META-LANGUAGE -- language to programmed processes that manage
   

(5) META-META LANGUAGES -- 



KEY NOUNS
- VALUES    -- JSON w. optional weights, mappable to UNI-FORM.
- CONTEXT   -- a set of values.   (forms an isa DAG. also forms containment DAG.)
               may be explicit and/or computed
- SPEC      -- an expression that is a weighted matching function against all values -- a VALUE that can be interpreted as a TypeSpec
? PREDICATE -- as an expression that is a weighted match function against all subject key object triples -- also a VALUE that can be interpreted as a TripleSpec --
- FN        -- a computable mapping y=f(x) where x in fn.domain and y in fn.range -- may output a weight set of y outputs.

DERIVED NOUNS
- DATASET   -- a context with objects with parallel keys representing experience and/or suitable for inductive learning (optionally with a way to obtain more experience)
- TYPES     -- a DAG of specs within some context
- KEYS      -- a DAG of predicates within some context
- FNS       -- a DAG of functions within some context

- TASK      -- a computation intended to achieve some objective



#### Overview Details

(1) Bootstrap nearly everything from nearly nothing
    Representation langauge is sufficiently flexible to allow one to express range of knowledge that we know humans are able to learn




### UNI-FORM

UNI-FORM -- A single recursive form that bidirectionally maps nicely onto RDF, JSON, LISP S-exprs,
and recursive map of maps.  In textual form it can be expressed succinctly as a recursive function 
invocation within some procedural language.  The python forms are:
    head(arg1, arg2, ..., key1=value1, key2=value2, ...)
    { '^':head, '^args':[arg1, arg2, ...], 'key1':value1, 'key2':value2 }    # this is also JSON form

    - head, and key1, key2, ... must be valid java identifiers (ascii that starts with a letter or underscore)
    - args and values are either a uni-form expression, a constant value:
      - a string, a number, or a dotted name.
    - A dotted name is a sequence of valid java identifiers that may have embeded '.' characters

NAMING -- If a UNI-FORM expression has a naming string, then each of its parts are also named:
    name.1, name.2, name.key1, name.key2  are the names for the sub parts shown in expression above.
    true, false, and null are special names that map to those same JSON constants.

SPECIAL KEYS -- keys with fixed special semantics are prefixed with a '^' character.

WEIGHTING -- Each sub portion of a UNI-FORM expression may be weighted -- may have a floating point
     number associated with sub-part.  the special key '^w' is used to store weights. so:
     head(arg1, arg2, key1=value1, key2=value2, ^w={'^args'=[w1, w2], 'key1':w3, 'key2':w4} )

UNIFORM COLON -- A format that allows embedding of other langauges (scripts etc.) into string values 
     while minimizing the backquoting needed


#### UNI-FORM to JSON mapping

- A uni-form expression is mapped to a JSON Object: 
  { '^':head, '^args':[arg1, arg2, ...] 'key1':value1, 'key2':value2, ... }

- dotted names are mapped to strings beginning with '.' characters
- string constant values are prefixed with (') character

#### JSON to UNI-FORM
- If a JSON object has a '^' key then it is assumed to be in UNI-FORM format and is mapped accordingly
- If not, it is assumed to be some constant JSON expression, in that case each string value
  is assumed to simply be a constant string, no (') prefix charater is expected.

#### MAP OF MAPS
- The most uniform representation of uni-form expressions is as a recurive map of string values.

STRING-VALUE-FORM -- all values in uni-form are mapped into a string value form by mapping:
  - mapping numbers to strings that contain the number's printed form
  - mapping dotted names to strings beginning with '.' followed by the dotted name
  - mapping strings to the same string with a (') prefix character

#### UNI-FORM to RDF
A uni-form expression is mapped onto RDF by creating triples for each part of the uniform expression.
  e = head(arg1, arg2, ... key1=value1, key2=value2, ...)

  e  rdf:_0  head
  e  rdf:_1  arg1
  e  rdf:_2  arg2
  e  'key1'  value1
  e  'key2'  value2

TBD mapping values to RDF constants

  

RDF-ISH
- PENT -- A subject, predicate, object triple with a context, and weight modifier.
- PSTORE -- A database of PENT
- CONTEXT -- A set of PENT -- A packet of grouped data
- TRANSFORM

MAPPING -- A set of REREP
RE-REP  -- A fn that provides values for a some relation in some context.
FACADE  -- 



FACADE -- A querable 



======================


### Data layer
#### CONTEXT API
(see Contexst.py)




#### COPS -- Context Ops

- STACK -- CtxStack( dest, ctx0, ctx1, ...)
  Defines dest_ctx as the layering of ctx0 onto the the stacking of ctx1... recursively



### Control Data Layer

VALUE -- JSON VALUE -- FORM -- a constant (scalar or structured) used by meta-meta
SCALAR VALUE -- an int, a number, string, symbol, or null
        SYMBOL VALUE -- TOKEN -- a string beginning with '.' followed by a Java identifier string
    STRUCTURED VALUE -- NODE -- a map from symbols (keys) to values.
    - Formally we write N[k]=v if node, N, maps key, k, to value v
    - NOTE: each key, k, defines an operator fn from node to value
    WEIGHTED VALUES -- any structured value may optionally provide a weighting over the set of keys.
    - the weightings is a map from symbols (keys) to weight numbers, and is accessed via the '_weights' key.
    - by default any key w/o a weight is assigned the default weight of 1.0
    UNI-FORM -- is a textual notation for expressing value
        head(arg1, arg2, ..., key1=value1, ...) 
    head(...) is shorthand use to denote any set of values for a given head


TYPE -- a type is a weighted subset of the set of VALUE. 
TYPESPEC -- SPEC -- some form (value) that can be interpreted as specifying a type.
    (NOTE: the structured typespecs below are expressed using the functional form for values as described above.)
    int, number, string, or symbol -- The 4 base typespecs, each refering to their 
        respective set of scalar values.
    all, none -- matches all and no values respectively.  null only matches the null value
    nnint, nnnumber -- non-negative integer, and non-negative number respectively
    map(T1, T2) -- $$$is the set of all nodes whose value are of type T
    node(T) == map(symbol, T)    node == node(all)
    list(T) == map(int, T)
    set(T) == map(int, T)   (like list, but order does not matter and values must be unique)
    enum(V1, V2, ...) is the type that matches exactly the set of specified values.
                      $$$ weighted enums
    obj(AT1, AT2, ..., k1=VT1, k2=VT2, ...) -- is the set of nodes whose keys satisfy the
            AT1,...VT1,... types, and whose head key, '^', matches the '^head' key's value
            '^rest' specifies the type of the rest of the positional args (set to 'null' to restrict)
            '^keys' specifies the type of the other keys
    or( T1, T2, ...) -- is the union of types T1, T2, ...
    and( T1, T2, ...) -- is the intersection of types T1, T2, ...


BINDING -- a NAME, VALUE pair 
    NAME -- a sequence of symbols 
            names can be expressed as a string with "." chars between the symbols
    BINDINGS -- CONTEXT -- CTX -- a tree of NODE that contains a set of bindings. $$$
            if a node, N, is bound to a path P, then for all keys, k, of N
                value, v=N[k], is bound to path, P.k
  

FN -- a computable mapping from an element of the domain to an element of the range.
    DOMAIN -- $$$$
    RANGE -- $$$$
    CONSTRAINED FN -- Fn types that have some constraint on domain or range.
    Each constrained function has a two letter code that can be composed to denote
    denote functions whose typing satisfies multiple constraints.
    - In -- INDICATOR FN     == fn(_returns=enum(0, 1))
    - Co -- COMPARATOR(T)    == fn(T,T, _returns=enum(-1,0,1))
    - We -- WEIGHTING(T)     == fn(T, _returns=number)
    - Sc -- SCORING(T)       == fn(T, _returns=number)
    - Di -- DISTANCE(T)      == fn(T, T, _returns=nnnumber)
    - Ge -- GENERATOR(T)     == fn(T, _returns=set(T))
    - Pa -- PARAMETRIC(T,FN) == fn(_returns=fn(T))
    - MONOTONE FN


FN TYPES
- FN TYPESPEC -- the form, f=fn(...), denotes a fn with specified domain and range.
        The domain for f is delete(f,'_returns'), and 
        The range for f is the value of f['_returns']
- FUNCTIONALLY CONSTRAINED TYPES -- each of the constrained functions above has
  a special key that is used to store the function with a node value.  
  - For example a weighting is stored in the '_w' key, so a weighted node
    would be of type:  typespec('_w'=fn(,'_returns'=nnnumber))
  - Adding a 'We-' prefix to a type is a shorthand way to denoting the type with this
    added constraint.  Thus a WeSet(T) == and(set(T), typespec('_w'=fn('_returns'=nnnumber)))
    or more compactly:  WeSet(T) == set(_rest=T,_w=fn('_returns'=nnnumber)))


DAG -- WeDAG -- 
- a DAG(T,up=fn(T,_r=T),down=fn(T,_r=set(T)),roots=set(T))
- a DAG is a set of roots from a typespace with some 'up' and 'down' relations defined 
- FOREST -- a DAG of DAGS where dependent DAGS inherit sub-portions of their parent DAGS


MACRO -- a template that can be expanded once each of its variables are bound.
- macro(head, A1, ..., _rest=Ar, k1=V1, ..., _keys=Vk, >>=
          A1(body_form(where=V1, V2=V3, value='can occur', anywhere=Vk))

  macro(comparator_fn(T(all)),
        fn(T, T, >>=enum(-1,0,1)))
  macro(para_fn(T(node), FN(fn)),
        FN(_keys=delete(FN, T._keys)))
        

???? DATASET -- DS -- Set of structured values that in some way have parallel or related keys
            
### Numeric Data Layer

FEATURE -- a single element of a population within the programmable population processor.
           each with an activiation level, an activation function, a reference name, 

POPULATION -- a set of features

FRAME -- a (usually sparse) representation of the activation levels of some features within a population.

TRACE -- a sequence of frames for a single population

DATASET -- a set of traces for some population


FEATURE FN -- FFN -- a specific kind of FN function that accepts a set of feature activation levels for a 
           fixed set of features, and a FN DOMAIN (as defined in FN) and returns an activation level.

REPSPACE -- a TYPESPEC of FFN -- 



### Rep Layer

THE MULTI-UNIVERSE -- A weighted DAG of TASK-ALIGNED UNIVERSES

UNIVERSE -- A weighted set of visible contexts

CONTEXT -- A self contained representational space
  - TYPES      -- a weighted dag of entities that can be in relation
  - RELATIONS  -- a weighted dag of available relations
  


CONTEXT 

REP -- A representational universe

- CONTEXT    -- a weighted dag of universes



SPACE -- a space 
  - Formally the spec for space is
    spec(space, seed=set(T), generator=fn(_returns=T), opt=fn(_returns=number))
  - s.seed is a set of seed values
  - a generator 


### Creation Ops
===>  DAGIFY   -- given a dataset and some predicates that apply to rows in the DS, produce DAG relations that exist between predicates
===>  CLUSTER  -- given a dataset group rows according to  

### Transform Ops

#### DAG/SPACE transforms

- CUTS -- remove roots, remove operators, remove derived nodes (and sub-nodes)
- ADDS -- add 

#### FOPS -- FN OPS
-- climb up/down domain/range types DAG
-- climb up/down ctx DAG
-- PIN ARG of fn to some specfic value


#### Climbing from A to B

##### Text Climbing
- Find subsets of text that are different    (subset creation)

- Find word group that substitutes into different 

##### FN climber
- Given a fn that is a facade, fn, from 'domain' to 'range'

-- Climb up/down context dag
-- climb up/down domain / range

### Task Definitions

TASK -- the specification of some bit of execution which could be undertaken in service of potentially addressing some objective.  A task has a:
- NAME
- PRIORITY   --  a computed or derived priority score
- ACTION     --  the root function invocation whose evaluation *IS* the task
- OBJECTIVE  --  a fn that can be executed to 
- RESULT     --  optional expression of the current 'result' of this task's execution
- STATS      --  the expected total time, time expended so far, expected time remaining on task





CONTEXT a 'structured spec'
- DAG

### Top Down

#### Heuristics

CONTEXTUAL CLIMBING

- PEER CONTAGON -- Assuming a strategy works in one context, see if it can be adapted for a peer.

- PARENT APPLICATION -- Assuming a strategy works in one context, (even better if an adaptation works for peers) theh try to adapt for parent context

- CONTEXTUALIZED SPECIALIZATION -- 


### To Figure Out

- Generated DAG
- pointer to space instance object


    - Node -- NODE(T,FN)   == typespec(..., k=FN) where 'k' is selected for the fn, so:
        WeNode(T)        == typespec(..., '_w'=fn(..., nnnumber))

    ----

    - OPERATOR FN
    - Op -- OPERATOR(T) 




$$$$ ISSUES
- lex variable
- 'fn' is both functional form typespec and fn spec ????
- PARAMETER -- a ?NAME? and a TYPESPEC of values that are valid to bind to that name.



$$$$

SPACE -- a finite type -- an explicitly enumerated set of weighted structured values.
    STRUCTURED SPACE -- 
- Specifies one or more axes that relate nodes to parent/children
  nodes within a DAG.


SPEC -- a potentially infinite set of weighted structured values.
- SPACE -- an explicitly enumerated set of values
- GENERATED SPEC -- 

NODE -- a structured value which is an element of a space or spec

### Builtin functions

fn(add, node, ...) -- accepts a node and adds (overwrites) keys in the node with 
    other specified keys.  e.g.   add(N, this=23, that=foo(25))

fn(delete, node, k1, k2, ...) -- will return a copy of node with keys, k1, k2, ... removed

fn(expand, node, bindings) -- 









