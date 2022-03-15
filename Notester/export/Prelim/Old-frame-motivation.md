# Prelim.Old-frame-motivation --

    \begin<<>>
    INTRODUCTION (summary)
    Our Goal: Encode strong bias for learning in complex domains.
    We use 3 methods:
    DECOMPOSITION -- We decompose the inductive choice of a hypothesis
        into a set of potentially independent choices (sub-induction
        problems).
    CONGRUENCE -- We provide specialized hypothesis spaces that match
        commonly occurring types of ambiguity in domain expertise.  These
        facilitate encoding domain expertise as a strong bias for
        induction.
    INTERMEDIATE OBSERVATION -- We use additional sources of training
        data.  In some cases it is possible to directly measure
        performance on one or more of the sub-induction problems.  Using
        such information is important since it allows us to decouple
        optimization of that sub-portion of the learning problem.
    \end<<>>
      + FRAME IS GENERAL.
        - Our approach assumes Uncertain K. is short hand for a set of alts
          Example: Meeting at 3:30. could be 3:35 could have exception on holidays
        - Need mechanism for selecting most approapriate for given sit. (ind).
        - No ``right'' (type) map from uncertain K to intended alternatives.
          but a small collection of types have wide applicability.
        - Thus we present a framework that generalizes the notion of K as shorthnd
        - Frame only requrires that auto procedure exists for enumerating
          and selecting among alternatives for each type of uncertain K.
      + EXPLORE TYPES OF UNCERTAINTY
        * To understand abstract framework we consider a number of types that
          can be mapped to sets of alternatives & mechanism for induction.
        * Nothing magic about these types of uncertainty, but we have found
          them to be common forms of uncertainty.  Later we will show how
          a number of these can be encoded as uncertainty types & used by PI.
        * Though nothing magic, they have wide coverage. we show this 
          by giving examples, later we consider its use in an impt & tough domain.
        * For now we will describe each, provide an intuitive example,
          and discuss how type type has been used in prior work in ML,KR,STATS.
      - TYPES OF ALT-SPACES (See examples node)
      - THE PI FRAMEWORK 
