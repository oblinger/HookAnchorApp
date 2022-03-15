# Prelim.Outline -- Prelim outline

     1 INTRO          -- Induce refinement of uncertain K.
     2 FRAMEWORK      -- Formal model; uncertainty types; 2-space ex.
    -   2 space example
        uncertainty types
     1 HHM INTRO
    -1 K. USED
     2 EXPERIMENTS    -- Implementation
       ANALYSIS
    -  PROPOSED WORK
     1 CONCLUSION  S  -- Contributions<<>>Future work
    --
    10
     7 Slides + give prelim doc
    --
    17
    25 
    --
    12 extra
     1 EXAMPLES?      -- Ex of uncertain K.
    - ANALYSIS       -- Proofs  (if just proposed then put later)
    1 RELATED WORK   -- FOCL, NON-MON, STATS
    BIO SECTION OUTLINE (focl <<>> outline)
    - Intro to HHM
      Related work subst; context-table; 1d-map
      Bio K as influenets; Fig (just a portion); Explanation of pieces
    - Implementation; CM5-fig;
    - Experiments
      Use of intermediate observation.  (deltas from)
      GAP<<>>GOP graphs
      Reordering results
    % XXXX Inference relies on decomposition (pieces of K) to express
    % complex theories.  We need to use a similar approach to express complex bias
    % XXX CITE use of decomposition in KR ML & AI & CS
    >INTRODUCTION
     <<>>   Old intro
     <<>>    still good stuff here
      Prob: Encode expertise as a very strong bias for use in complex domains.
      Impt: Theoretical results place clear limits on weakly-biased induction.
            Induction can only succeed in a complex domain if it is based on
            a strong bias.
      Soln:
      - Inductive frame based on set of choices. rest follows...
            We propose PI, a framework for induction that incorporates a number
            of mechanisms that facilitate encoding bias for complex domains.
            These mechanisms are organized around two broad principles:
            congruence, and decomposition.
            Def of both.
          - In PI the expert encode K much like K eng. writes logical thy
            (This is why we call it PI.)
            * pieces of K -> pieces of choice.
            * reps for particular type of K. -> rep for par. type of ambiguity.
              when uncertain expert inserts some short hand for a set of
              possibilities.
              - Results in very strong bias, and in some cases allows
              ind. that resolves undertainties.
              - We believe that small set of types have large applicability.
          - Both the high bias, and potential for decomposition can
            greatly increase the complexity that can be handled by ind.
          - This approach is designed to facilitate encoding bias for
            complex domains.  Problems addressed and claims made are best
            tested by application of PI to a complex domain.  Thus a large
            fraction of the effort in this disertation has gone into
            applying the framework to a challenging problem in the
            biological sciences---the problem of HHM.
      Benefit:  Flexible tool for encoding domain knowledge that marrys
      best of expert and machine.  It allows specific knowledge when available,
     and allows expert to defer certain choices for inductive refinement.
      OUTLINE
    > FRAMEWORK FOR PLAUSIBLE INFERENCE
      MOTIVATING EXAMPLE
      - what is section; why this example
      - use simple example to make aspects of frame clear
      - example inspired by my current means of transport
        - SoonAfter it rains my car is broken.
        - After a hard rain my car is broken.
        - When it has gas it can locomote.
        - Generic knowledge: broken things don't locomote.
      - What hyps we want<<>>don't want to consider
      + FORMAL CHARACTERIZATION OF PI
        * We have seen types of uncertain domain K and how induction might help
          We now present a formal model of PI that uses ind to refine K.
        - BASIC IDEA:
          PI accepts uncertain domain K.
          K. is annotated w. info about experts uncertainty. eg [10can_locomote, broken->cannot, after rain
          PI makes simplfying assumptions that allows quasi-independant induction
        > Minimal commitment to PI's K. rep. just concerned with:
          - the plausible alternatives in each refinement space, 
          - the relationship between ref spaces
        + Refinement space: (function) Variables that can adopt diff vals.
          R, TYPE, RANGE    (not VAR, RANGE, TYPE, ASSIGMENT)
        + Conventional Functional rep for K
          ANTECEDENTS, CONSEQUENT, MAP
    XXXX Alt function
        + Inputs to PI
          Plausible Theory: K
          ASSIGNMENT
        ? Variables: V, RANGE(), ASSIGNMENT
        + Current Observations: O
        + Goal: G
        + Bias from pl <<>> REF-SPACE(v,k)
        + State of PI system.
          RELEVANT(v,s), RELEVANT*, PICK
          REFINE(r,sc,p,s), SCORE(k,r,s)
          INITIAL(r,p,s), BEST(r,s)
        + Block Diagram of PI
         ISSUE how to resolve multiple spaces
      > REFINMENT TYPES
      * Types we have explored & used in application later
      - Continuous threshold
      - [Monotonic function]
      - Parametric
      - Alternatives
      - Aggregate
      - Overriding
      - Implication
      > SIMPLE EXAMPLE W. TWO SPACES
        RefType: can-locomote(x) overriding-combinations
        RefType: lots-of-rain, after-delta continuous thresholds
        car(x) < can-locomote(x)
        broken(x) < car(x) ^ rain-amt(y) ^ (> y lots-of-rain)
        ~can-locomote(x) < broken(x)
    > ANALYSIS (after bio)
      Approach can always mimic conventional induction; when does is do better?
      Relys on congruence & decomposition
      1) CONGRUENCE: Ref type matches expert's uncertainty
         - Not knowable in advance.
         - You know you've done it when expert has no preference for hyps.
         - Small set of ref spaces has wide applicabilty
           We demonstrate this by using ref spaces above in bio domain
      2) DECOMPOSITON: Splitting hyp space decision into multiple quasi-indep
         decisions.  Two ways:
         1) Intermediate observations.
         2) Quasi-Independance.
    > FORMAL ANALYSIS
      - Simple P progress algorithm
      - Optimal&Terminates given progress & sub-optimal  (P progress)
      - Safety & monotonicity of all stategies
      - Optimality&Termination given no complete barriers  (P progress)
    > PI's RELATION TO EXISTING WORK
      Problem of intelligent action in world requires reasoning<<>>learning hybrid.
      PI's strength comes from blending strengths of reasoning & learning
      Thus can be viewed as a specific instance of each
      Expert<<>>BK sys., NON-mon, INDUCTIVE LEARNING, & EBL
      - Uncertainty reasoners
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
        - Substitution table methods (align algs)
        - Context specific tables.
        - 1D-map based approaches. 
      > Expressing Biologist's understanding of protein folding as influents
        + Examples: *Note bio-k-ex::
      [related work should be scattered through last two sections]
    > IMPLEMENTATION and EXPERIMENTS
      > My implementation of the PI framework
        + Figure showing modules.  With description of the current system.
      > EXPERIMENTS
        - FOCL comparison.
        - Hill climbing for optimal GEP <<>> GOP
          Domain thy for context-based substitution tables. w. combo contexts.
          Beats identity cost function.
    see how big this section is before doing 1d-maps.
        - 1D-maps results
      > ANALYSIS
        - Do combo spaces avoid over-commitment?
    > PROPOSAL SUMMARY  *Note proposed-work::
      - Bullet style list of work done <<>> to be done
      - Future work
      - Contributions of the work
      - Timeline
    > CONTRIBUTIONS & CONCLUSIONS
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
    IDEAS
    - Our approach facilitates representation of bias as pieces of knowledge.
      Impt. for the same reason that it is impt. in logic.
    - WEAKNESS
      No general way to exress K about interactions between types
      We imbed in the PICK-SPACE procedure.
      In our experients we usually forced an ordering of K.
      Could add rules paritial order info & use to generate PICK-SPACE procedure.
      that is future work.
      PICK-SCORE
    OUTTAKES
    x     Algorithm assumes that no pathalogical interactions between spaces.
    x     decomposition works. We swallow pathology in sub-space (talk about later)
    SNIPS OF TEXT TO BE ADDED TO THE PRELIM STATEMENT
     <<>>   Intro w. reference to kb approaches
     <<>>   Bio mot
     <<>>   Examples pl k. in bio.
     <<>>   Old outline (replaced 2/28/97)
     <<>>   Proposed work section
