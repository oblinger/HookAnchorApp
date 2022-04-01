# 42\_Bug\_in\_proof --

    Guys,
    1. I'm trying to decide if we have an error on page 2 of our 
    paper.  Can you take a look and confirm?  Here's what we say:
     - Let (X,d) be a metric space.  Let R be a finite subset of X.
     - Let A(R) be the sum of distances from points to nearest
    centers as found by the clustering algorithm
     - Let OPT(R) be the optimum sum of distances from points
    to nearest centers
     - Suppose we have an alpha-approximation algorithm that ensures
       A(R)\leq \alpha OPT(R)
    We then say that our sampling result implies that we can find a 
    clustering such that A(R) \leq 2\alpha OPT(R) + \epslion for any finite
    subset R.

    Questions: (A) Do we mean "average" instead of "sum" of distances?
    Our proofs are about averages quickly converging.  If we 
    want to say sums, do we not need an "n \epsilon" in the
    final statement above?  (B) In the statement of our
    result, do we mean to say A(X) \leq 2\alpha OPT(X) + \epsilon
    provided R is sufficiently large?  If the two R's are intended
    to have different interpretations, then do we mean 
    for any subset S of X that A(S) \leq 2\alpha OPT(S) + \epsilon
    provided we draw a sample R from S of sufficiently large size.

    2.  A larger question is shall we clean up and improve this paper 
    for a journal submission.  I would feel comfortable submitting
    the paper to a journal if we could minimally do the following:
    (a) Eliminate the dependence on M the diameter of the space
    (b) Change our results from additive to multiplicative bounds.
    Apparently Vapnik's Statistical Learning Theory book discusses 
    how to obtain multiplicative uniform convergence bounds, but 
    I have not looked into this.
    (c) Extend our analysis to the k-line median problem where the
    goal is find k lines with the property that the average distance
    from a point to its nearest line is minimized.

    I could look into (c).  Lenny mumbled something about (a) once.  
    Dan could look into (b).

    This might be more work than you have time for, but let me know
    if you're interested.

    -Nina
