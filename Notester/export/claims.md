# claims --



    1.  A method and apparatus for deriving a generative graph model consistent with a training set
        and a similarity metric

    2.  Where the training set is a set of sequences of observations

    3.  Where the generative model can be used to recognize whether other sequences are
        consistent with derived model.  It can also be used to generate sequences consistent
        with the derived model.

    4.  Where the generative graph model contains a set of nodes.  the nodes of the generative model
        can be placed in coorespondance with the obervations in the sequence that they generate.
     
    5.  Where the similarity metric that accepts a pair of observations
        and returns a real number representing the "similarity" of the two observations.

    6.  Where the "similarity" score of two observations is used to bias the derived model such that
        pairs of obersvation coorespond to the same nodes in the generative model have greater similarity
        that radom pairs of obervations.


    As a specialization of this we assert that the generative model is an hmm and the apparatus
    is max likely hood E-M


    As a specialization of that we assert the generative model is an IOHMM


    Do we explain how we use the sim function in the claims (talk about the representative for
    each node, etc.)  or is that simply in the preferred implementation????
