# AR.Patent --



    DISCLOSURE NUMBER: YOR820030655 
    TITLE: Biasing Hidden Markov Model Learning With A Similarity Metric

    The invention disclosure identified above has been received by the Intellectual 
    Property Law (IPLaw) department, and must 
    be evaluated to determine the appropriate protection (if any) it should 
    receive. You have been selected to participate in that 
    evaluation. To access the evaluation form, please follow the steps below.

        1.  Click on the Evaluators tab to bring this tab to the top of the 
    Navigator.
        2.  As an evaluator, you can view and work with inventions categorized 
    by Status, Title or Number.
        3.  Select the invention you wish to evaluate by double clicking on it.
        4.  Once the invention is displayed, click the Edit Document button to 
    put in edit mode.
        5.  If you are going to be fthe only evaluator use section titled PVT II 
    in the disclosure and respond to the PVT II questions.
                6.  If there will be multiple evaluators and there will be 
    more than one evaluation form, one person will have to act as the   IDT 
    coordinator and fill out the section titled PVT II in the disclosure.  Other 
    evaluators should click on the 'Create an Evaluation' button to create an 
    Evaluation Form - their responses to the Evaluation Form questions will 
    automatically appear for all team members in the WPTS view listing for the 
    disclosure.         
            7. Click on the 'Calculate PVT Score' button and consider the 
    PVT score.
        8. After the PVT II questions on the disclosure are filled in, click on 
    'Final Evaluation'.

          Note:  If you are asked to evaluate a disclosure that was created prior 
    to October 15, 2002, the completion of the Critical Questions in the disclosure 
    and the original Patent Value Tool is sufficient to allow a 'Final Evaluation' 
    to be performed.
        
    Note: Inventions made with government funding trigger significant IBM reporting 
    obligations to the government.  If the invention was made with government 
    funding (i.e. the inventors answered critical question #6 or Patent Value Tool 
    question #10 affirmatively, or you are otherwise aware of such), 
    please expedite evaluation as it will ensure proper reporting.  Thank you for 
    your assistance in evaluating this disclosure, and your participation in the 
    protection of IBM's intellectual property.

    ===============================

    YOR820030655


     Biasing Hidden Markov Model Learning With A Similarity Metric 

    has been received by our office and assigned DISCLOSURE NO. YOR820030655.

    The patent attorney responsible for handling the disclosure has been assigned 
    to Thu A Dang/Watson/IBM.

    The Invention Development Team (IDT) member(s) assigned to this disclosure are 
    Arthur C Ciccolo/Watson/IBM;Thu A Dang/Watson/IBM.


    ---pvt


     1.  Click on the Evaluators tab to bring this tab to the top of the 
    Navigator.
     2.  As an evaluator, you can view and work with inventions categorized 
    by Status, Title or Number.
     3.  Select the invention you wish to evaluate by double clicking on it.
     4.  Once the invention is displayed, click the Edit Document button to 
    put in edit mode.
     5.  If you are going to be the only evaluator use section titled PVT II 
    in the disclosure and respond to the PVT II questions.
                6.  If there will be multiple evaluators and there will be 
    more than one evaluation form, one person will have to act as the   IDT 
    coordinator and fill out the section titled PVT II in the disclosure.  Other 
    evaluators should click on the 'Create an Evaluation' button to create an 
    Evaluation Form - their responses to the Evaluation Form questions will 
    automatically appear for all team members in the WPTS view listing for the 
    disclosure.         
            7. Click on the 'Calculate PVT Score' button and consider the 
    PVT score.
     8. After the PVT II questions on the disclosure are filled in, click on 
    'Final Evaluation'.




    ------------------------------------------------




    YOR8-2002-0579  Alignment&Generalization



    Background

    A discrete markovian process is a process that is characterized by a state machine 
    comprised of a set of n states and a transition function and an output function.
    At any given point in time the process is in one of its n states.  At each step the
    process may transition to another state (as dictated by the transition function), and
    it will output a symbol as dictated by the output function.  The important constraint
    imposed by this framework is the finite memory employed by the process.  At each step,
    both the next state, and the symbol output are only a function of the current state.
    It has cannot be affected by the history of states or symbols, nor any other information.
    Both the transition function, and the output functions are only functions of the current
    state.  Thus a markovian process is mechanism for generated a constrained stream of symbols.

    A Hidden Markov Model is a markovian process that is hidden.  All that is available is
    the sequence of symbols generated by the process.  These are provided as a set of 
    observations, and the objective is to construct a markovian model whose outputs
    accurately model the symbols generated by the original process.  Since the original
    process may be a randomized process, we best we can hope for is a process that
    has the same probability density as the original process does over all possible
    sequences of symbols.

    Traditionally an Expectation-Maximization algorithm is used to learn a Markov
    model from a set of observation sequences.  We extend this basic algorithm as
    describe below.


    Similarity Metric 

    Traditional Hidden Markov Models (HMM) assume a completely 'hidden' state, that is,
    no distributional assumptions are made regarding the transitions and outputs of the
    states of the Markov process.  Such distributions are induced from the observed sequence.
    In this disclosure we consider an additional source of information for use in
    determining the underlying Markov process.  

    We provide an additional input to the HMM learning algorithm, we provide a
    similarity metric over the set of symbols generated by the process.  This metric
    returns a similarity score for any pair of symbols generated by the Markovian process.
    The similarity function provides additional insight on the structure of the hidden
    Markov process.  The similarity metric is guaranteed to have the following relationship
    to the underlying model:

      The expected similarity of two symbols S1 and S2 generated from the
      same state in the Markov process will have a greater similarity than
      two symbols S3 and S4 generated by different states within the Markov 
      model.

    Our disclosure covers the use of this additional knowledge source as a bias in well
    known problem of searching for an accurate model of a hidden Markov process.

    The specific approach we have taken to biasing the basic algorithm is as follows:

    (1) We maintain a set of representative observations for each node in the evolving
        Markov model.

    (2) As we compute the probability of an observation matching a given node (as required by
        the Expectation Maximization algorithm) as done in the classical algorithm.  Additionally
        we include a term that is computed from the average 'similarity' between the representatives
        and the observation in question.  

    This biases the models generated to align observations 'similar' according to the metric provided.
