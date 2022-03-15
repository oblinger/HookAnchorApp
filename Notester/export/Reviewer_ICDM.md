# Reviewer\_ICDM -- The Third IEEE International Conference on Data Mining


     <<>>   <<>>

    <<>> and selecting "Program Committee,"
    "PC Information Website," and _Download Review Papers_ options.




    REVIEW  #387

    The spatial characteristics of this transformed 'transactional' representation of
    spatial data is as the authors point out sufficiently different, that specialized
    methods for learning over this data are warranted.  And the mining spatial
    data is a very relevant area, thus I lean to accept this paper.

    After reading the paper it seems that mining complex relationship types makes sense,
    and is a contribution to the field.  The authors point out in section 1.1 that
    mining these relationships will improve performance of mining 'simple' relationships
    in two specific ways.  This is a stronger contribution since it maps directly onto
    mining that has been considered in the past.  After some effort considering 
    the empirical work show in 7.3 I was disappointed that I could not convince myself
    that indeed we had shown that complex relationships boost performance over 
    previous 'simple' relationship learning.  I think I understand the results
    contained in that sections, but it doesn't seems that they directly relate to
    a comparison with previous mining of spatial data.  If there is a connection perhaps 
    there is some way to make it more apparent, if
    not then the 'contribution' section needs to be re-worded.

    Notes: Introduction spends 1/2 page convincing us that mining spatial data is different from 
    mining market basket data.  One would expect mining data with such additional structure
    could make use of that structure.  Simply stating that the additional structure admits
    approaches that advantage of these constraints.  Then later in the paper you must
    make it clear how this structure has helped.

    A Clique is a fully connected subgraph within a larger graph.  It was not clear from 
    your introduction (where you begin using the term) if your
    'cliques' are transitive and thus are fully connected.  Figure 1 seems to imply they are not,
    yet many times it seems they are.  In any case, typically the relation 'near' is not a transitive
    notion, so you need to explain your notion of 'near' used here.  If your cliques are not
    graph theoretic cliques you should use another term.

    The use of weak monotonicity is clever.  Is that a contribution of this paper?  MAke that clear.

    Proof sketch 6.1 gives some understanding of why a->b would depend on b's self co-location,
    but it would be instructive to have some intuitive understanding of why we should
    expect this simple relation to depend on B's self co-location.

    What does Figure 9 tell us about the performance of the algorithm?  What should we conclude from it?
    I think I understand it, but not its importance to understanding how well this approach performs
    relative to others (or perhaps that is not its intent)




    BestPaper? no  Jnl Special Issue? No  Relevant to ICDM? Yes

    Innovative? 5-0
    Technical Quality? 5-0
    Presentation? 5-0
    Interest to ICDM practitioners and users? y/maybe/n
    My confidence: H/m/l
    Recommendation:  Strong Accept, accept, weak accept, weak reject, reject, strong reject
    Comments:

    ------


    REVIEW 

    Short Summary of decision  #264

    I leaning to accept this paper is based on the clear formulation of an interesting datamining 
    problem of poison message detection.  This DM problem has interesting properties that 
    distinguish it and solutions to it from many other DM problems.  The authors also do
    a good job in characterizing general categories of approaches to this problem that they 
    and others have taken to the problem.

    I would be much happier if the results were also contained results from some real rather than simulated data.
    But I recognize obtaining data regarding such a rare event as a true poison message problem in a live
    network would be difficult.  The simulation testbed seems to be a well thought out model.  What little 
    detail we see of the model in the paper suggests that it does reflect the types of networks one would
    observe in real networks.

    I have two main critisicisms of the paper.  First seems to be a basic misunderstanding of
    the properties of a kNN classification algorithm.  In the first paragraph of section
    4.1 the author states that a kNN forms better estimates as the density of points increases.
    I agree.  They also say that it increases a k increases.  I disagree.  They expound on this
    in section 4.2 toward the end where the characterize the choice of 'k' as a tradeoff between
    computation and accuracy:  "... this is due to large k reduce(sic) the noise influence.
    However, large k also means more computation is needed to search the nearest neighbor."

    If a kNN works at all then it must be performing at a level above the default classifier
    which simply predicts the apriori most probable classifier, yet increasing k asymptoitically
    produces just such a classifier.  The optimal value of k is actually function of the 
    the curvature of the function being modelled, and the density of the training data.
    If k is too large it smears the function being modelled, if it is too small it is suseptible
    to noise as the authors state.  Indeed the conclusion that k=3 is a good value for this 
    domain is predicated on the assumption that as k increases further little additional improvement
    will be had prior the inveitable degradation in performance.  None of this is discussed, 
    if it is computationally infeasible to compute the remainder of the remainder of Table 3 that
    should be stated.  Otherwise an extended table should be computed in order to see what maximum
    is possible.  (In general we cannot exclude discontinuous jumps in performance)

    My second critisism is the lack of empirical analysis in this paper.  The use of a simulation
    testbed facilates statistically rigorous testing of many aspects of a system.  No confidnece intervals
    are provided for any results in the paper for example.  Many details of the system could be
    explored easily.  For example the integration in sequential decision problem requires many
    parameters to be set.  For example the time to wait before deciding that a problem has not
    propagated, the distance metric employed by kNN, the fraction of message types filtered
    at each step, etc. Very few of these interesting decisions were tested empirically, and
    none in a statically appropriate manner.


    A minor confusion (probably on my part) is the assumption that blocking non-poison message types can never
    block propagation.  The last two sentences of the first paragraph in section 2.2 say this, but it
    seems implied elsewhere.  My confusion is that the example given in the beginning involves a poison
    message that is only activated when two non-poison messages are also send in close succession afterwards.
    This seems to imply that blocking the subsequent messages would block propagation, but they are
    not called 'poison' messasges as there is assumed to be only one type of poison message at any one time.
    I assume somehow I am misunderstanding something here, but I don't know what.


    In the end, it is the characterization of the data mining problem itself as well as the clarity
    of the assumptions underlying the solution (in section 1.2) that impressed me about
    the value of the paper.  My only concern in this regard is that such a framework has already been
    outlined in some literature I am not familure with.

    BestPaper? no  Jnl Special Issue? No  Relevant to ICDM? Yes

    Innovative? 5-0  4
    Technical Quality? 5-0  3 -- only tested value of k, and two choices in distance metric.  no statstical rigor.
    Presentation? 5-0  4
    Interest to ICDM practitioners and users? y/maybe/n  yes
    My confidence: H/m/l  high (but cannot guarantee novelty of framework outside of DataMining community.)
    Recommendation:  Strong Accept, accept, weak accept, weak reject, reject, strong reject  Weak Accept
    Comments:



    ---------
    REVIEW  #316

    My rejection of this paper is primarily based on its topic not being relevant to the DataMining community.
    The primary contribution of this paper is optimum vector angle representing the best ratio of herbs
    for a dog's cardio system shown in Figure 3.  In order for this paper to be of interest to the data 
    mining community it should present a new Data Mining technique, or illumination regrding the application
    of accepted techniques to problems in general.  

    At the very least this paper should explain why many of the decisions taken in this paper were taken.
    Section 3 provides a liteny of decisions made by the author, but never once does it say *why* these
    were the right decisions for this problem.  Such an understanding might provide insight for the reader
    regarding other situations where such techniques might apply.  Better still, the author could explictly 
    provide rules of thumb regarding when these techniques are appropriate.

    The techniqes themselves also seem to have more in common with traditional statistical analysis.  Perhaps
    a Data Mining conference is not a first choice in publication venues.  It is also unclear what contribution is
    being made to either communities.  The author must highlight some technique and explain why it is a 
    novel contribution to the field, or at least provide a novel expostion regarding how known techniques
    should be applied to an interesting class of problems.  In short the author must clearly 
    explain what the contribution is to the *datamining* community is if he want to publish in that community.


    Minor Note:  The orgnization of the paper is good as is the paragraph level structure of the paper.  The
    grammar and sentence level structure has a great many problems.  I found ten errors in my first read of the
    first page alone.  I cannot imagine the challenge of writing English, if it is not ones first language,
    but these errors do detract some from ones understanding of the material.  Before final publication it
    is important to reduce the number of these errors.


    Short Summary of decision

    BestPaper? no  Jnl Special Issue? No  Relevant to ICDM? Yes

    Innovative? 5-0
    Technical Quality? 5-0
    Presentation? 5-0
    Interest to ICDM practitioners and users? y/maybe/n
    My confidence: H/m/l
    Recommendation:  Strong Accept, accept, weak accept, weak reject, reject, strong reject
    Comments:

    ------------
    REVIEW  #235

    I enjoyed my readings of this paper.  Though I found it sparsely explained in some
    parts, thus difficult to absorb.  I am recomending this paper for acceptance because
    I find the idea of clustering based on relations inherent in the data to be a
    compelling and novel one, and  because its evalutaion is also compelling.

    Indeed my greatest critism of the paper is that I feels I had to work hard to
    uncover the largest contributions of the work myself.  They were only implicit 
    in the paper.  One glaring example of this is the title itself.  There are
    many techniques for selecting an optimal number of clusters for traditional
    clustering algorithms.  The primary contribution of this paper is not a method
    for selecting 'k' as it title implies.  The contribution is a notion of cluster
    that is based on the relations inherent in the examples being clustered.
    What a great idea!  Why isn't that mentioned?!

    I recognize that these ideas are probably expounded on in the preceeding work
    (Belkhiter94 and Ganter98), but this will be unknown to a significant fraction
    of the paper's readership.  Explicating how these ideas can be applied to a
    central datamining problem (clustering) is the value of this paper.  Explaining
    the intuitions underlying these ideas and how they relate to clustering is a 
    critical but missing part of that value.  Even if these ideas are not new to
    this paper, they must be presented here.  The only sentence in the whole paper
    that discusses the motivation of this clustering metric is included on the
    first page:  "the goal of the coneptual decomposition is to find an
    economical binary relation coverage ..."  This is not sufficient.  Please
    give the reader some intution regarding the types of domains where this
    clustering might make sense.  Give us some intution regarding the difference
    between this a k-means clustering for example.

    With some effort I was able to understand the intuitions underlying underling
    the definitions in section 2, but readers with less motivation or less mathematical
    background will not fair as well.  Even a single well-chosen sentence could provide
    the intutive meaning for each of these terms.  I found definition 2.3 particularly 
    in need of an intuitive as well as mathematical defintion.  (I started at that for
    a long time before I could parse its meaning!)

    An intuitive meaning for the Rand and Inertia metrics needs to be provided.

    The testing of the algorithm was an interesting departure from the traditional
    few graphs showing performance of various alternatives as relevant parameters vary.
    The analysis gave just a hint of its performance against accepted clustering algroithms
    in a large number of contexts.  It left me with the impression that this algroithm
    was significantly different and better in some context.  Since the authors never 
    motivate why they expect this algorithm to work well, I don't feel I have any
    appreciation for where this algorithm would be expected to perform well.
    (other than some vague feeling about clusters with compact sets of relations)

    In short I like this paper alot, but I feel like it is not written in a
    way that is accessible to an important segment of its audience.
    DataMining practitioners often complain about the performance of clustering
    algorithms.  This algorithms seems to make a different set of assumptions,
    and so practitioners could be quite interested in trying it on their data.
    But they need to understand at an intutive level how this algorithm is different
    and how to apply it.  The specifiction of the algorithms seems fairly complete, 
    and very precise.  Is this algorithm availble for download?  This would 
    signficantly increase the impact that it might have on such a community.


    BestPaper? no  Jnl Special Issue? Yes  Relevant to ICDM? Yes    (but with significant expansion as described above)

    Innovative? 5-0  5
    Technical Quality? 5-0  5
    Presentation? 5-0  3
    Interest to ICDM practitioners and users? y/maybe/n  yes
    My confidence: H/m/l  m
    Recommendation:  Strong Accept, accept, weak accept, weak reject, reject, strong reject  accept
    Comments:


    REVIEW  #286

    My first read of this paper left me with the impression of a strong, well polished
    paper written by a group quite familiar with previous work in the field.  Some of 
    that initial impression still remains with me.  The organization of completeness
    of the related work section is to be admired.  Not only are a large number of
    approaches discussed, but the reader is left with some understanding of how they 
    relate.  The approach itself also seems sound and reasonaly well documented.

    My first impression was that there was considerable emprical support for the
    approach.  But when I considered some of the most basic questions
    "How well does it work?" and "when does it work?" I found that the copious
    empirical data doesn't address these.  We are shown how this approach works
    along with four different classifiers, and over a number of data sets, but
    we have no results at all comparing how it works against other imputation 
    techniques.

    To be sure, this is not an easy requirement to fulfill, it means either reimplementing
    someone elses imputation algorithm, or applying your algorithm to exactly the same
    dataset, and it the same way that others do.  Still to have no information
    on this is simply inappropriate.  One could compare against "easy" imputation
    techniques like mean or mode, and then report on how much your relative improvement
    compares with relative improvements obtained by others (even if not on the
    same dataset).  This would be better than nothing.  It seems that the authors
    believe that the relative performance of their imputation technique will vary 
    against other imputation techniques in proportion to how well baysian learning
    will perform relative to the base learning used in other techniques.  This
    is a very interesting conjecture, it could be explored via a scatter plot
    of base learner performance vs. imputation performance.

    The second question is more easily addressed.  Assuming that this technique 
    performs at least reasonably compared with other imputation techniques, then
    one would like to know where it can be used effectively?  There are many tests
    one could imagine, but the simplest is to graph performance as a function of 
    fraction of missing data.  How does this algorithm perform is the missing data
    is 10% or 50% rather than the reported 30%?  Any user of this will want to 
    have some understanding of how well it will perform on their data.

    The authors rightly note that accuracy in predicting missing features is
    not the important measure for this domain.  But they still provide these 
    numbers.  These numbers would be much more interesting if they were
    coorelated with the final accuracy of the whole system.  Indeed a simple
    scatter plot showing accuracy on predicting features against whole system
    accuracy would provide interesting insight the meaning of this intermediate
    results.  As it stands it is hard to see how those number contribute to our
    understanding of the approach.

    This paper favors tables over graphs.  This obscures trends in the data.
    It also makes comparisons more difficult.  It would be nice to compare final 
    learning algorithms across different datasets, and against 'straw man'
    approaches to feature imputation.  In the tables the entries 'original'
    and 'substitute' need to be better explained.  Original could mean
    original data in the repository.  Or it could mean the original 
    (unsubstituted) data provided to the algorithm.  The origianl/substituted
    designation could refer to the data given to the four Weka learners, or it
    could mean the data used to score the induced model.


    BestPaper? no  Jnl Special Issue? No  Relevant to ICDM? Yes

    Innovative? 5-0
    Technical Quality? 5-0
    Presentation? 5-0
    Interest to ICDM practitioners and users? y/maybe/n
    My confidence: H/m/l
    Recommendation:  Strong Accept, accept, weak accept, weak reject, reject, strong reject
    Comments:


    REVIEW  #219

    The strength of this paper is the precision applied in specifying what the method is, and
    how it is implemented.  Its weakness is in describing how it moves beyond existing
    source separation techniques, and what its performace is in practice.

    The authors clearly believe that this technique is somehow more appropriate for spatial data
    in some way.  Judging from the example provided it seems that it is designed for data
    where the parameters are related by some underlying spatial process.  If this algorithm
    is effective for some definable class of "spatial" problems that would be very interesting.
    But I was not sure what aspect of the domain needed to be spatial (the noise, the original process
    generating the data, the process generating varience, etc.)  Second, it was not explained
    how this process was well suited to spatial data.  On page two it is claimed that Factor
    Analysis and PCA are not.  

    I feel that either the paper must say "we are working on a sub-type of source separation"
    and explain why it approach is indicated 
    or it must explain why the approach is better than existing source separation techniques in 
    general.  It does neither.

    The empirical evaluation of the technique is also weak, and poorly presented.
    The scatter plots if Figure 5 and 6 do not have labels on their axes, and it is
    unclear which control points on the left are being discussed on the right (all of them?).
    It seems that the empirical evalution is not intended as a proof of the techniques soundness
    but rather serves to illustrate how it might be applied.  This would be fine, if we had an
    intuition that supported the use of this approach for specific types of problem.

    Inspite of these clear failures, I am weakly supported this papers acceptance.  The reason
    is the effort put into the techniqe itself, and because of the detail provided on the technique.
    It seems that someone quite familar with signal separation might be able to intuit where this 
    approach would be applied even though I could not.  And the bulk of the paper would provide
    ample detail to allow them to apply the technique.  





    BestPaper? no  Jnl Special Issue? No  Relevant to ICDM? Yes

    Innovative? 5-0  3
    Technical Quality? 5-0  4
    Presentation? 5-0   3
    Interest to ICDM practitioners and users? y/maybe/n  yes
    My confidence: H/m/l  low
    Recommendation:  Strong Accept, accept, weak accept, weak reject, reject, strong reject
    weak accept
    Comments:








    [LOG]

    Dear Dan,

    The ICDM '03 PC information website is now accessible from the
    conference homepage (<<>>) under
    the Program Committee link.  Your initial password is "r275Olnr".
    Please change this password and verify/update your contact and
    affiliation information at this website as soon as possible.
     
    Thanks,

    Xindong and Alex 
    ICDM '03 Program Chairs
