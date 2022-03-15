# Activity.2007\_07\_10\_Meta Learning Talk --


    On behalf of Christophe Giraud-Carrier and myself, we would like to invite you to act as an invited speaker during the workshop on "Meta-Learning" 
    as part of the 2007 International Joint Conference on Neural Networks (IJCNN).  
    he workshop will take place on Thursday Aug. 16 from 7:00 PM - 10:30 PM. As an 
    invited speaker you have the option to send a paper or simply focus on your presentation. 
    A link to the conference follows:

    <<>>





    - Pietro: Library research


    Bootstrapped Learning and Deliberative Meta Learning

    There are a number of objectives that lead to different forms of Meta Learning.
    One motivation is the notion that even the adaptive (learning) processes themselves
    should be adapted based on prior experience.  The cumulative learning perspective
    is explored in an ongoing DARPA program, Bootstrapped Learning.  This program sets
    the ambitious goal of a fully autonomous learning system that must configure 100%
    of it learning parameters, as it moves between learning tasks as diverse as agent
    control, diagnosis, and planning tasks.

    while this is a clearly a form of meta leraning it is quite different and some ways
    more ambitious than current forms of meta learning.  viewed from this perspective
    meta learning becomes a much more deliberative process in two important ways:

    (1) The learning system must have a meta model of itself as a reasoning/learning
        agent.  By construction of the testing domains, we allow zero reconfiguration
        of the learnging agent as it moves from trainging task universe to training task
        universe.  Thus it must "get itself out of any jams" that occur in each domain.
        It must understand enought of its own capabilties to plan for and recover from 
        learning failures.  For example it must "get frustrated" when one sequence of 
        learning steps does not seem to be having the desired effect, and develop an 
        alternative strategy for coordinating its own, abstraction, representation, etc.
        methods in order to solve a problem that is eluding its first attempt.  
        This level of deliberative reasoning about the learning/reasoning process is 
        an interesting direction for meta learning reserach.

    (2) The second deliberative aspect of this new learning paradigm stems from the goal
        of 'bootstrapping' the learning. No intervention is 
        permitted during the learning process, and the concepts being learned
        are complex enough that they require a very strong learning bias.  Thus
        100% of the bias parameters for learning must be derived from prior learning, and
        those parameters must form a very strong bias.  We will discuss how these
        constraints also lead one in the direction of using deliberative reasoning over
        prior learned knowlege as a particular form of meta learning.

    ------------------------------


           === DELIBERATIVE META-LEARNING ===
    - 'Meta' learning (applying adapataion to the adaptation alg itself)
    - GENERALIZED TO:  Learning that is shaped by prior learning.
    - Cumulative Learning
    - Bootstrapping:  output==input

    (1) Feature-based bootstrapping
    (2) Learning using compositional building blocks
    (3a) Bias parameters <------
        - difficult to balance bias/varience tradeoff in space of biases
        - Often are in spaces that are not native for humans (so they cant help)
        ! NOT bootstrapping.  input != output
    (3b) Range of deliberative options
        - Reasoning about plausible hypotheses
        - Talk about learning



    ------------------------------------------------
    BOOTSTRAPPING   input==output

    - Discovery From Data
    - Communication from Expert
    - 
    -------------------------------------------
    INSTRUCTABLE COMPUTING   ('agent' has sought capabiilty)
    - 







