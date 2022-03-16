# Prelim.Old-outline -- Old outline (replaced 2/28/97)

    > INTRODUCTION  Prob; Impt; Approach; Benefits  *Node pi-kb-intro::
      - Increasing reliance on KB systems.  (ref of real complex stuff)    
      - Interestingly difficult is often not stating K, but stating w<<>>o over state
      -  The difficulty is not getting K, but correctly encoding it. (ref?)
        [in particular encoding the interaction between K]
      - PROBLEM: Encoding what is known w<<>>o overcommitment. [to interaction]
        Only info available is K. so strong assumptions <<>> if K is to be used,
        even though assumptions are known to be too strong.
      - Approaches:  K about uncertainty.  Rep that matches expert's uncertainty.
      - SOLUTION: Don't make predictions from K alone.
        Treat ambiguity in K. as alternatives and use as the basis for
        strongly constrained induction.
      - BENEFIT:
        + Permits reps w. that make weaker commitments, because 
          induction fill in missing information.  Allow incorp of ambig. Expert K.
        + Much of the complexity of the induced concept can be encoded in
          the experts space, leaving an small and managable space for induction.
      - OUTLINE
    > FRAMEWORK FOR PLAUSIBLE INFERENCE
                                                spaces?
      * [Approach requires that we identitify a space of possible ways in which K
        interacts, and build an inductive learner that searches this space.]
      * Not obvious what counts as an overcommitment. 
        Def of over-c, behaviour not intended by K. author.
      * No single right answer.
        Affects many things: Rep., learning efficiency, reasoning efficiency, etc.
        Appropriate action is to id general categories of over-c that make nice
        tradoffs.
      * Next sub-section enumerates several over-Cs that resolve a wide range
        Remainder of section develops the PI framwork into which they & other fit
      - TYPES OF ALT-SPACES
        + We explore two types of overcommitments found in conventional reps:
        Specific values for parameters, How K combines.
          + Discrete & continuous parameters, determinations
          + Additive, Monotonic, Functional, Overriding, Conjunctive???
        - Not a canonical or exhaustive set.  Eg:  Tree based parameters.
          Indeed such a list is not possible.
          Still has wide coverage.
    X     Unlikely to be captured in some single more general mechanism. ????
      - THE PI FRAMEWORK
        * Minimal commitment to rep.  just concerned w. how they combine
          to make hyp space & how hyps make predictions about world
        + Convetional Functional rep: DEF: vars<<>>vals  RANGE assignment, state
          fns?relations?
        + Each over-commitment category is expressed as an ALT-SPACE
          Just require enough info to do induction.
          Def: ALTERNATIVES, SELECT
        + Block Diagram of PI
         ISSUE how to resolve multiple spaces
      - SIMPLE EXAMPLE W. TWO SPACES
        ovrerrides + additive + skolem   
        Blocks world: Amt. to be pushed.
        Comfy Clothes: Flannel -> comfy, total water, saturation eq.
    > PI's RELATION TO EXISTING WORK
      Problem of intelligent action in world requires reasoning<<>>learning hybrid.
      PI's strength comes from blending strengths of reasoning & learning
      Thus can be viewed as a specific instance of each
      Expert<<>>BK sys., NON-mon, INDUCTIVE LEARNING, & EBL
    ? - Rule based systems.
      - Non-mon: Same motivation.  Viewing as induction adds:
        Restrict to computationally feasible. consider generalization consequences
      - KB induction.
        PI is induction applied to refining expert's K. 
        + FOCL, GRENDEL, FORTE, assume model is syntactically close to correct.
          PI derives bias from a non syntactic model of theories weakness.
      - EBL: EGGS, EBG
        We Generalize explanations like EBL.
        Primary weakness of EBL is extreamly strong assumtions about domain thy.
        We address this by using non-deductive explanation mechanism.
    > BIO APPLICATION
      > INTRODUCTION: Bio app. as validation of framework *Note bio-mot::
        Outline of Bio app.
        + Motivation of hidden homology detection
          * Life is proteins.  1D str. (composition) -> 3D str. (shape) -> fn.
    >   Because of its importance much work done on prob. of hidden homology
        - Substitution table methods
        - Context specific tables
        - 1D-map based approaches 
      > Expressing Biologist's understanding of protein folding as influents
        + Examples: *Note bio-k-ex::
      [related work should be scattered through last two sections]
    > IMPLEMENTATION and EXPERIMENTS
      > My implementation of the PI framework
        + Figure showing modules.  With description of the current system.
      > EXPERIMENTS
        - Hill climbing for optimal GEP <<>> GOP
          Beats identity cost function.
        - Results that suggest good intermediate terms. ????
      > ANALYSIS
        - Do combo spaces avoid over-commitment?
    > CONTRIBUTIONS & CONCLUSIONS
    > PROPOSAL SUMMARY
      - Bullet style list of work done <<>> to be done
      - Future work
      - Contributions of the work
      - Timeline
    STUFF TO ADD
    - Timeline
    - Types of combination spaces
    - Goal: Encode as experts K and certainty w/o overcommitment.
    - Thing that ties the different types of uncertainty:
      1) Can be expressed as a set of alternatives for induction.
      2) Remaining hypotheses can be efficiently computed given a set of
         examples.
      3) Induction for each type of combo-space must be independant
         from other spaces.
      (we have explored way to weaken this last constraint)
      - swallow interation into one combo-space
      - Iterative optomization of different spaces
