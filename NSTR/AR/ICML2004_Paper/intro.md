# AR.ICML2004\_Paper.intro --




    PBD IS GOOD
    - The ability to capture procedural knowledge by demonstrating instances of those procedures has great promise for many impt probs.
      - approaches are refered to as PBD.
      - procedures widespread in K based orgs.  Capture, disemination, and use of these garners a large and increasing
        fraction of these organization overall costs.  Automating portions of the process is of coorespondingly increasing importance. REF
      - Within computational context, Places where such proc knowlege capture is particularly impt:
        - idiosyncratic situations
        - spanning applications

      - However 

    NEED FOR ALIGNMENT
      - A natural approach is to use some form of adaptation or learning to obtain the procedure (program) given a
        demonstration of that procedure.
      - Learning requires multiple instances for generalization -- in this case the means obtaining multiple demonstrations
        of a step in the underlying procedure in order to learn that step.
      - But in order to be able to obtain multiple instances of a given step, some form or coorespondances must be determined
        between the steps of each of the demonstated procedures.  
      - Previous PBD work obtained these correspondances either explictly from the user REF or implicitly by
        assuming a fixed ordering among the procedure steps.
      - These approaches are inherently limited either because they require considerable effort on the authors
        part over and above the demonstration itself, or because they are limited to relatively simple procedures
        where correspondances are implicit in the order of the demonstrated steps themselves.
      - Such approachs are not amenable to procedures with many optional/alternative branches and loops.
      - Example configuring networking example ....




    OUR APPROACH
    At the highest level our approach is to integrate learning algorithm with sequence alignment algorithms in order to 
    induce the underlying procedure from a set of demonstrations of that procedure.  

    This approach accepts a set of recorded demonstrations, of a common procedure.  Each demonstration is a sequence of 
    actions and cooresponding state changes in the system as a consequence of those actions.

    Our approach is to search for an alignment of those demonstrated steps, as well as a generalized model
    of each aligned set of steps that is consistent with the demonstrations.  

    Note the circular dependance this approach implies: finding appropriate generalizations depends on the alignment of steps
    across the demonstrations, and converely determining and appropriate alignments depends on the generalizations that are 
    possible for each step given that alignment.

    Thus our approach is to simultaneously utilize the generalization and alignment constraints to obtain a model
    (procedure) that agrees with the provided demonstrations.  We refer to this approach as Simultaneous Alignment 
    and Generalization or AlignGen.

    We have incorporated these constraints into an Expectation-Maximization algorithm.  
    This algorithm presented in section ### is an extension of traditional discrete HMM learning.  

    It has been extended to utilize a heuristically determined similarity information regarding appropriate
    alignments.  Unlike many previously studied applications of HMMs this domain provides strong aprori bias 
    regarding potential alignments of the demonstarated sequences.  

    ? we will explain this bias in detail in section ###
    ? we present the formal extension to HMM leraning in section ###
    ? and empirically evaluated the effect of this additional bias in section ####


    X Once induced this procedure can be annotated (for added comprehensibility), then disseminated, and executed
    X on multiple target machines.


    BENEFITS
    - AutoAlignment: Allows passive recording of demonstation traces.
    - AutoAlignment: Minimal additional information, reduces the cost of the approach thus greatly expanding the 
                     situations where it will be cost effective to use.


    Approach find utility in several settings:
    - Serves as a cost effective method for generating instructional materials
    - Automates procedures 



    OUTLINE

    In the next section we present the AlignGen architecture in the PBD context.  We will see that
    this context provides a form simiarlity information that will drive our extension to traditional
    the traditional EM approach to HMM induction.  

    Section ### will formally present the SimIOHMM algorithm, and section ### will empirically 
    evalute this algorithm in the PBD context.  A second set of results will measure the effect
    of the bias introduced by our extension through a comparison with traditional HMM learning.

    The paper finishes with a related work section, and an analysis of benefits and drawbacks to
    of this unique approach to the PBD.

    The SimIOHMM algorithm is our implementation of the AlignGen approach.  It is an extension of IOHMMs which in turn are
    extensions of HMMs.  This algorithm is described in detail in section ##.
