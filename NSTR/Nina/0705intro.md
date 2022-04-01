# Nina.0705intro --

    <<>> 
     
    Unsupervised data clustering is of central importance in a number of 
    disciplines including Machine Learning, Statistics, and Data Mining 
    [refs].  For the majority of this paper we define clustering as the 
    task of partitioning a set of points (a <<>>) into a set of 
    clusters (a <<>>) in a way that maximizes some cluster 
    quality metric (<<>>). 
     
    While this problem is known to be NP-hard [ref], polynomial-time 
    approximate clustering algorithms have been developed that generate 
    clusterings that are within a constant factor of optimal [refs].  The 
    improved polynomial performance of these approximation algorithms, 
    however, still leaves many large clustering problems computationally 
    out of reach.  The recent prominence of such very large data sources 
    (like the web) compels us to search for more dramatic approximations 
    of the clustering problem, in particular to search for approximate 
    clustering algorithms whose time complexity is sub-linear in problem 
    size (instances being clustered). 
     
    In this paper we describe the fruits of this search, we describe an 
    approach to approximate clustering whose clustering quality is 
    comparable to existing approximate clustering algorithms, yet whose 
    runtime is better than polynomial in the dataset size---it no longer 
    even <<>> on dataset size.  For the generality sake, we 
    describe this algorithm as a wrapper that fits over any existing 
    approximate (or optimal) clustering algorithm.  This extended 
    algorithm explicitly dictates the time/quality performance tradeoff 
    through a combination of sampling points from the input set, and by 
    optionally translating the sampled points into a lower dimensional 
    space before clustering. 
     
    Viewed within this space of tradeoffs, we see that existing clustering 
    algorithms sit at one extreme of the time/quality tradeoff.  Exploring 
    this tradeoff, we observe that improvement in clustering quality per 
    additional unit of processing time diminishes rapidly as dataset size 
    increases.  Surprisingly, this allows us to cluster extremely 
    (unboundedly) large datasets while incurring a minimal (arbitrarily 
    small) sacrifice in clustering quality---our approach to approximate 
    clustering has computation time polynomial in size of the permitted 
    error, and is independent of the dataset's size, and is only logarithmically  
    dependant on the datasets original dimensionality. 
     
    This new form of approximate clustering has important ramifications 
    for many practical clustering problems faced today.  Many such 
    problems can only be performed using approximations that have time 
    complexity sub-linear in dataset size.  Clustering web pages [ref], 
    click streams [ref], or other web-derivative data are all such 
    contemporary examples.  Clustering multimedia data [ref], retail chain 
    transactions, use of telecommunication phone records to detect fraud 
    [ref], etc., are other examples of important contemporary applications 
    that require sub-linear approximate clustering. 
     
    <<>> 
     
    <<>> 
     
    The next section is devoted to a formal characterization of 
    approximate clustering.  This is followed by our primary result, an 
    algorithm for approximate clustering through sampling along with 
    guarantees regarding it performance.  Four generalizations and 
    applications of this result are presented.  The first is a dimension 
    reduction procedure that can be used to trade clustering quality for a 
    logarithmic time complexity in the number of dimensions used to 
    describe each point being clustered.  Second we present an estimation 
    procedure for use when no bound is available on the range of the 
    values possible in the data being clustered.  Third we show how this 
    approach can be applied to both the discrete and continuous clustering 
    problems.  Fourth describe the application of this work to the problem 
    of property testing---determining the smallest number of clusters, 
    <<>>, required to obtain a specific clustering quality. 
     
     
     
     
     
     
    <<>> 
    In the remainder of the paper 
    - Def of the approximate clustering problem 
    - Presentation of the sampling portion of our clustering algorithm along with guarantees regarding its performance. 
    - We present four generalizations and applications of the core algorithm. 
      - The first is a dimension reduction step, this procedure trades clustering quality for complete for a logrithmic  
        time complexity in the dimension of the initial dataset. 
      - We show how the core clustering algorithm is applied when no bound is known on the range of the input point (which are sampled 
        from by the algorithm.) 
      - We argue that our approach applies to both discrete and continuous clustering 
      - We describe the application of our work to the problem of ### testing. 
     
     
     
    <<>> 
     
     Other applications include mlti data, retail chain tran, telecomm phon rec to disc fraud, etc. 
     
    Furhter such data is typically stro in sec data dev and access is costly 
     
     
     
    why does this kick ass? 
    - When 'N' is very large 
    - Clustering web pages, very large data sources. 
     
     
     
    One possible path around this prob is 
     
    Happily, we have found that the marginal clustering quality 
     
    Indeed we have found that approximating existing 
     
     Fortunately, we have found that yields dimishing returns 
     
    relative to approximations 
     
    fortunately we have found clustering the full dataset yeilds diminishing returns  returns as the set of instances to cluster increases. 
     
     <<>> 
     
     PROB: The recent prominance of very large data sources (like the web) pose, still  
     a new difficulty for these algorithms, however. 
     
     Typically the super-linear runtime performance of these clustering algorithms preclude their application to such large datasets. 
     
     An obvious approach to this problem is to perform some form of approximate clustering where computation time is traded off against the quality of the obtained clustering. 
     
    <<>> 
     
    SOLN: To this end, we formally define approximate clustering, and provide an approximate algorithm for k-medians clustering. Our algorithm employs a data-sampling wrapper around existing clustering algorithms. 
     
    This wrapper allows us to explicitly trade runtime against obtained clustering quality. 
     
     <<>> Thus, We will show 
     as a consequence we have found  find that existing clustering algorithms 
     
     \ <<>> 
     
    Thus 
     
     
     
     SOLN: To this end, we formally define approximate clustering, and provide an approximate algorithm for k-medians clustering. Our algorithm employs a data-sampling wrapper around existing clustering algorithms. 
     
    This wrapper allows us to explicitly trade runtime against obtained clustering quality. 
     
    Exploring the tradeoffs these simp we find that returns in clustering quality obtained from additional processing time diminishes rapidly as dataset size increases. 
     
     
     
    <<>>  
     1  explore whether more dramatic approximations exist that still retain sufficient clustering quality.  
    <<>> 
     (the task of partitioning a dataset of instance point according to a 
    predefined cluster quality metric)  
     
     
