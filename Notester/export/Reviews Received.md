# Reviews Received --



     <<>>

    Kezhao, 
     
    Congratulations on your acceptance into this years IC-AI conference.  Below 
    are the notes from the reviewers of your paper.  Please make the indicated  
    changes prior to your final submission.  We look forward to meeting you 
    in at IC-AI later this year. 
     
    Regards, 
     
    Ricardo Vilalta 
    Dan Oblinger 
     
     
     
     
    REVIEW #1 
    "Classification of Hard Disk Drive Component Performance  
    Using Machine Learning Methods." 
     
    Decision: ACCEPT 
     
    High Level Comments: 
     
    1) The paper provides an empirical comparison of the K-NN nearest neighbor and 
    a Neural Net (NN) on the HDD component performance domain. The NN is shown to 
    outperform the K-NN method. 
     
    2) The paper is well written and easy to follow. The value of the paper lies in proposing  
    the HDD domain as candidate for the application of machine learning techniques due to  
    the existence of feature interactions.  
     
    3) The experimental methodology is well explained and justified.  
     
    Low Level Comments: 
     
    1) Abstract, 3rd sentence, change "perception" to "perceptron". 
    2) Page 3, Section 3, 1st paragraph, change "that takes..." to "that take...". 
    3)  Section 3.1, says "wi is the square of the Euclidean...", shouldn't it 
          say "wi is the inverse of the square of the Euclidean"? 
    4) End of Section 3.2, I would say at this point the number of hidden 
    units.  
    5) Section 4, second to last paragraph, I recommend to explain what the 
    conventional simplistic method is right after mentioning it.  
     
    -------- 
    REVIEW #2 
    Classification of Hard Disk Drive Component Performance Using Machine 
    Learning Methods 
     
    Kezhao Zhang 
     
    STATUS: Marginal Accept 
     
     
    SIGNIFICANCE: 
    This paper compares two generic learning algorithms on a specific domain. 
    No generalizations or discussions about the relative performance  
    is provided.  Thus the significance of the paper is limited to this specific 
    problem. 
     
    ORIGINALITY: 
    The learning components (as opposed to the simulation components) were 
    fairly generic, the originality contained in the paper stems from their 
    comparison over a real application area. 
     
     
    TECHNICAL MERIT: 
    The work is well done, and the results are complete (e.g. confidence intervals 
    are provided on the performance figures, and scoring is done on both testing 
    and training data.) 
     
    I am not convinced that an analytical solution to the problem is impractical. 
    Such a solution would be preferable to the ML techniques employed.  The 
    author should give the reader some appreciation of the difficulty of such  
    an approach, thus motivating the use of a non-analytical solution in this  
    domain. 
     
     
    READABILITY: 
    In general the paper is clear and well polished.   
    My largest concern in this area is the lack of justification 
    for the use of learning in this domain.  The authors use a Monte Carlo 
    simulation to generate training data, but this leads the reader to wonder 
    why we don't simply use the this method to directly test each hard drive 
    design instead of utilizing learning as an intermediate step.  I believe 
    the answer is one of efficiency and because real data can be used to augment 
    the simulation data.  The first justification was never given, nor supported 
    with data on the speed of a simulation vs. a classifier application.  The 
    second justification was only given in the concluding section, both should  
    appear much earlier. 
     
     
    --------------- 
     
    Nathalie, 
     
    Congratulations on your acceptance into this years IC-AI conference.  Below 
    are the notes from the reviewers of your paper.  Please make the indicated  
    changes prior to your final submission.  We look forward to meeting you 
    in at IC-AI later this year. 
     
    Regards, 
     
    Ricardo Vilalta 
    Dan Oblinger 
     
     
     
    Review #1 
    Title of Paper: "The Class Imbalance Problem: Significance and Strategies". 
    Author: Nathalie Japkowicz 
     
    Decision: ACCEPT 
     
    High level comments: 
     
    1) Organization is good and the paper is well written. 
    2) I agree with the author that artificial domains are best to manipulate 
         parameters and discover algorithm behavior. Nevertheless, it is often 
         necessary to at least mention how the study can be extended to real- 
         world domains.     
    3) The paper is limited to describing experimental results. It would be  
         interesting to explain why high complexity, independent of training set 
         size,  makes error rate increase due to class imbalance.  
     
    Low level comments: 
     
    1) Section 1, third paragraph, when citing several authors at once use one 
    single set of parentheses and semicolon to separate them. Don't start with 
    e.g. 
    2) Also, I noticed several times a reference to an author within parentheses. 
    You should make a distinction between a direct reference :"I agree with  
    Pazzani (99) that.." with an indirect reference: "...which has been studied  
    before (Pazzani, 99)."   
    For example, end of page 3, footnote, should say "...below and by Japkowicz...(1995)". 
    Also, Section 3.1 should say "...by Schaffer(1993)." 
    3) Section 3.2 says ".. on the domain this bar represents." I don't have  
    this clear. Does it mean according to the complexity? 
    4) The author should explain what "learning by recognition" 
    means. It is not obvious from the context.  
    5) Section 6 refers to Footnote 3. This should not happen. If a footnote 
    is so important that it needs to be referenced again, it means the text 
    should be part of the main paper. Some people have suggested to  
    eliminate footnotes at all. 
     
    ---------- 
    REVIEW #2 
     
    Recommendation: ACCEPT 
     
    SIGNIFICANCE: 
    Class imbalance is an important practical and theoretical area of exploration. 
     
    ORIGINALITY: 
    The experiments and results presented in this paper are original. 
     
    TECHNICAL MERIT: 
    The experimental design was well thought out, and the approach was 
    appropriate and effective. 
     
    I was happy to see that each bar was an average of 5 runs of the algorithm. 
    This is especially important since there is so much unexplained variance in 
    the results presented.  I was disappointed that not confidence information 
    was included either on the graphs or in the text.  with this level of variance 
    it is difficult to understand what we should interpret as a trend and what 
    we should interpret as noise. 
     
    I understand that time constraints may preclude generating confidence intervals 
    for all of the data presented.  Nonetheless, at least a simple confidence 
    interval should be computed for some of the interested aspects of this data 
    and reported in the text.  this is essential for valid interpretation of the 
    data. 
     
    I noticed by eye, that class imbalance did not seem to correlated in any simple 
    way to learning error.  This is interesting, and merits some discussion. 
    Do we have too little data?  Is this noise?  It there a statistically valid 
    trend with a high enough variance to distract ones eyes, but is nonetheless 
    in the data? 
     
     
    READABILITY: 
    My primary concern was the clarity of the 
    primary results figures.  (Fig 2-5) 
     
    - The wording under figure 2 "each cluster of 5 bars" is unclear. 
      It could mean that one cluster contains all five value for that 
      attribute, or it could mean that those 5 taken tougher represent 
      the same value on that dimension. 
     
    - Both size and difficulty increase from left to right, but class 
      imbalance increases from right to left.  This is very confusing, 
      and should be corrected. 
     
    - The error bars in each graph should all be scaled on the same scale. 
      This would greatly improve our ability to compare these graphs. 
      (This should be accomplished, prior to publication even if it requires 
       adding a dummy error bar to force all graphs to be scaled to 50%) 
     
    Overall the text of the paper was quite readable. 
     
