# Dimitri\_Patent --


    LEGEND FOR FIGURES

    <<>>

    110  Adaptive Modeling Process
    111  History?
    112  Modeling Metric
    113  Best Model
    114  
    115  Control Process
    116  Constraints
    117  Equations
    118  Object Functions
    119  
    120  Projective World Model
    121  Projected histories

    100  Learning subsystem 
    101  Observations History
    102  Adaptation process
    103  Control process
    104  User Model (current best)
    105  Generated Control Parameters
    106  The operating environment
    107  
    108  
    109  
    110  Timeline containing all observations
    111  Single Observable
    112  Example Control Parameter
    113  Example Interface Parameter
    114  Example Environmental Parameter
    115  Example Indirect Measures
    116  
    117  
    118  
    119  
    120  Induced Markov process
    121  Modeling Objective Function  -- IN TEXT:  the adaptation process selects the markov process that optimized the 'modeling objective function'
    122  
    123  
    124  
    125  
    126  
    127  
    128  
    129  
    130  Time sequence of observables
    131  Sequence projection model
    132  Projected observations sequences    (IN TEXT:  note that each projected sequence has a probability associated with that sequence)
    133  Predicted indirect measures
    134  Generated control parameter settings
    135  Objectives:  constraints and equations






    Please see a summary and attached stenotranscripat the bottom with a lawyer.
    Thanks!
    Dimitri
    -----------------

    General problem:
    There are a lot of Telematics devices and services in cars that are already available and are planning. They distract a driver  and may contribute to increase in traffic accidents. This require a smart design of use rinterfaces in cars.
    How to predict what user interface would generate the optimal driver behavior?
    Optimal behavior means - driver attention is sufficient for key things that are required to drive a car.

    Special solution:
    Create (automatically)  user interface that takes into account user biometrics. User biometrics provides some information about driver condition, satisfaction, tiredness, sleepness.
    This solution should be flexible enough to incorporate other factors that affect driver conditions like situation and environment factors (raining, speed, other cars passing this car, a baby in a car that require attention etc.).


    ----------



    SUMMARY OF THE ADAPTIVE PORTION OF THE PREDICTIVE USER INTERFACE PATENT
    -----------------------------------------------------------------------


    The predictive component of the system uses observations from the interactions between the user, the computer interface, and the surrounding
    environment to induce a model of the user's state over time.  This inferred state model in turn can be used to make predictions about 
    a number of indirect indicators of the users state (including their cognitive load, where their attention is directed, their fatigue level, 
    their sleepiness, etc.)

    This predictive time-based model can be used to adapt the a users environment in a way that satisfies or optimizes an objective function 
    specifies in terms of these indicators of the users state.  (For example, the system may dynamically update the auto's graphical displays, 
    radio audio, in a way that provides appropriate and sufficient attention for changing driving conditions.)

    We first describe the inputs and outputs of this induced time-based model and then we describe the inputs and outputs of the adaptive process
    used to induce this model.


    ----------
    USER MODEL
    ----------

    The model of the user's state (the USER MODEL) is assumed to be a Markov process with state that is initially unknown (e.g. hidden).
    The process is markovian in can be expressed using a fixed finite state representation.  This 'hidden' state
    represents the actual state of the user as it relates to the indirect measures described above (attention, etc.)
    The inputs to the model are the OBSERVANBLES, which are represented as a time-based stream of features both about the user's environment,
    as well as features about the computer generated interfaces provided to the user (which indeed are also part of the user's environment).
    So the automobiles speed, number of near-by cars, the temperature in the car, and the number of active displays, are all features
    that might be part of the OBSERVABLES.  The observables also capture historical information as appropriate, so they also 
    can capture information about the history of the interaction, so the duration of a trip, as well as an integrated assessment of 
    driver's fatigue as measured over a long time might both be part of the OBSERVABLES as well even though they really represent
    information collected over a range of time.  There are three types of OBSERVABLES, the first type are CONTROL PARAMETERS these are observables
    which the overall system has control over.  So the level of the radio's audio, the temperature in the car, and the number of displays
    active at any given time are potentially CONTROL PARAMETERS since it is possible for the overall system to have some control over these PARAMETERS.
    They number of other automobiles around the car is not a parameter since this cannot be controlled.  The second type of OBSERVABLES are
    interface parameters, these are a special case of CONTROL PARAMETERs that have to do with specifically with the computer generated displays
    available to the user.  So the number of active displays is one such INTERFACE feature.  The third category is ENVIRONMENTAL OBSERVABLES.
    These are uncontrollable features of the user's environment, like the number of cars nearby.
    (These distinctions are made because it is only the CONTROL PARAMETER type of OBSERVABLES that will be manipulated by the overall system.)

    The outputs of the USER MODEL are the INDIRECT INDICATORS of the user's state.  They each attempt to provide some indication of the
    actual (but unobservable) state of the user.  These indirect indicators are chosen so that they capture aspects of the user's state
    that are of interest to the designer of this system.  So if avoiding sleepiness is important to the designer then features
    relating to sleepiness will be included in the indirect indicators.  Thus indicators relating to attention, sleepiness, etc. are
    examples of such features.  (more is said about these 'objectives' of the designer later)

    It should be noted that the inputs and the outputs to the USER MODEL are inherently a time based set of measurements OBERVATIONS, and
    a time based set of predictions about the INDIRECT INDICATORS.  It should also be noted that the USER MODEL is a model with induced internal
    state, thus predictions about the INDIRECT INDICATORS may be a function of OBSREVATIONS made at earlier time points.  This is important
    because some indicators maybe LAGGED indicators of the user's actual state.  For example the indicator of a user's cognitive load may
    be of a nature that only after some period where the user has a high cognitive load, does the indicator actually register the cognitive load.
    More will be said about these LAGGED INDIRECT INDICATORS later, for now it is important to simply note that a USER MODEL with internal state
    is capable of "remembering" OBSERVATIONS from the past in a way that permits making predictions about indictor values in the future.


    -----------------------------
    THE ADAPTIVE MODELING PROCESS
    -----------------------------

    The adaptive process is the process used by the system to adaptively construct a user model as described above.

    A HISTORY with a given user is an input to the ADAPTIVE PROCESS.  A HISTORY include a time-based stream of OBSERVATIONS of that user's
    environment (including their GUI interface and physical environment).  It also includes a time-based stream of INDIRECT INDICATORS.

    The goal of the adaptive process is to construct a user model (a markovian function with state) which predicts the latter stream of INDICATORS
    from the former stream of OBSERVATIONS.  Both Input/Output Hidden Markov Model learning algorithms (IOHMMS) and partially observable
    markov processes (POM-DPs) are examples of such processes.

    The output of this process is a USER MODEL which is updated from time to time (or continuously) as the HISTORY with that particular user
    grows.  The ADAPTIVE PROCESS attempts to find/construct a USER MODEL which BEST predicts the INDICATORS given the observations, according to 
    some MODELING METRIC.  The MODELING METRIC is simply returns a real valued score for how well a
    set of predictions matches a set of observed INDICATOR values in the HISTORY.  This MODELING METRIC is an input to the ADAPTIVE PROCESS.

    At all times the adaptive process maintains a 'BEST' user model which represents the best USER MODEL found according the HISTORY available
    as well as the MODELLING METRIC provided.  The BEST model is an input to the control process below.


    -------------------
    THE CONTROL PROCESS
    -------------------

    The control process is a continuously running process that is parallel to the adaptive modeling process.  The control process 
    attempts to manipulate the user's environment in a way that achieves certain objectives that the designer of the entire system
    had in mind.  We enumerate the inputs and outputs and behavior of this control process:

    The objective of the control process is expressed as a set of CONSTRAINTS to be satisfied or EQUATIONS whose values are to be maximized.
    Both types of objectives are collectively call OBJECTIVE FUNCTIONS, and both are expressed in terms of the INDIRECT INDICATORS of 
    the user's state.  So minimizing a measure of user-frustration, and ensuring some minimal cut of for sleepiness is not crossed are both
    examples of OBJECTIVE FUNCTIONS for such a system, because both are expressed as a function of indirect indicators.
    The set of OBJECTIVE FUNCTIONS to be satisfied and maximized are an input to the CONTROLL PROCESS.

    The current HISTORY of the interaction with a given user is also an input to the CONTROL PROCESS.

    A time-based stream of values for the CONTROL PARAMETERS is the output of the control process.  The goal of the control process
    is to select values for the CONTROL PARAMETERS that maximized and/or satisfies the OBJECTIVE FUNCTIONS given the current
    BEST user model from the ADAPTIVE PROCESS as well as current HISTORY.

    We have several preferred embodiments for this process.  One is an exhaustive enumeration of possible control parameters and 
    testing against the OBJECTIVE FUNCTIONS, another is a gradient ascent method for selecting best parameters, (but there are many 
    more, including Monte Carlo simulations, simulated annealing, etc.)

    We also explicitly include processes that not only ensure current OBJECTIVE FUNCTION maximization/compliance, but also future
    compliance based on projections of current HISTORY.  For this, a PROJECTIVE WORLD MODEL is employed.  The projective
    world model is a model of how the *OBSERVABLES* change over time.  Note this is model has noting to do with the USER MODEL
    or with the INDIRECT INDICATOR of the user's state.  Rather this projective world model takes as input a HISTORY of OBERVABLES
    projects those values into this future it produces a predictive future 'HISTORY' for those values based on the trends
    in those values.  So the projective world model could make predictions about the number of cars near by based on current velocities
    and positions of cars currently visible.  It could make a prediction about future temperatures based on current temperature and
    settings of the heater.  Predictions could be a single extension the current HISTORY, weighted set of possible extensions, or a 
    parameterized model of those possible extensions.  

    The PROJECTIVE WROLD MODEL can either be an input to the CONTROL PROCESS, or in can be adaptive learned from the HISTORIES from many
    user's interaction with the system.  In all cases the goal of the CONTROL PROCESS is to select CONTROL PARAMTER values that not only 
    maximize/satisfy the OBJECTIVE FUNCTIONS for the current HISTORY, but also for extensions to that history generated by the 
    PROJECTIVE WORLD MODEL.  

    As a concrete example the control process may adjust the radio volume and display complexity downward because the projective world 
    model predicts traffic congestion in the near future, and based on that congestion (an ENVIRONMENT OBSERVATION) the user model 
    predicts that INDIRECT INDICATORS of user attention will drop below minimum thresholds specified in the OBJECTIVE FUNCTIONS provided 
    to the control process.  

    In contrast, predictions of future values based of the INDIRECT INDICATORS based on lower distractions from controllable parameters 
    shows INDIRECT INDICATORS which fall with in acceptable values according to the specified OBJECTIVE FUNCTIONS.  In this case
    the output from the control process would be these "reduced distraction" control parameter values.

    It is assumed that the output of these control parameters are used to drive the automobile interfaces so that the designer OBJECTIVE 
    FUNCTIONS are satisfied when possible.


    --------------------------------------------------------------------
    DISCUSSION ABOUT TIME LAGGED OBSERVABLES AND INDICATORS IN A HISTORY
    --------------------------------------------------------------------

    Both the observables and the indirect indicators may actually represent a time-lagged measurement of some actual quantity 
    that the designer cares about.  So the designer of this system may care to capture the actually complexity of a given 
    driving situation, but in some cases in may only be apparent after some period of time what that complexity actually is.
    For example a user that quickly shifts visual attention between many input sources in rapid succession (rear view mirror,
    side mirrors, and looking back) is probably in has high attention drain, but they had this high drain even before the system
    registered enough visual shifts to recognize that high attention drain.  As an example of the second type of time lag
    consider a measure of sleepiness that captures wobbling motions.  Thus user was undoubtedly sleepy before this measure could observe this
    sleepiness.  

    These time-lags in the HISTORY are accounted for in this approach by a USER MODEL that contains inferred state parameters.  These parameters
    can capture the accumulated evidence of some unobservable state in a way that it can be used to make predictions about lagged indicators
    whose values will only change in the future.

    This ability to model these lagged dependencies is one of the reasons why a USER MODEL with inferred hidden state is employed.  As
    a simplified but concrete example of such a prediction behavior, consider a user that immediately becomes confused by a particular arrangement
    of their graphical display generated by the system, but the system only recognizes this confusion at a later point in time once
    the user has exhibited sufficient manipulations of the interface and attention shifts.  A model of this user which optimally predicted
    this lagged indirect indicator of their confusion would be selected by the MODELING PROCESS.  This induced model would have to
    use its inferred 'hidden' state to keep track of the history of the interface OBSERVATIONS so that is could correctly predict 
    when the user INDIRECT INDICATORS would begin to express this confusion (it might even be at a point in time when the interface 
    was no longer complex).

     
