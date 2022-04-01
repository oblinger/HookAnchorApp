# Old-Prj.Paper --

    1) Intro 
    2) Approach 
    3) Architecture 
    4) Cross Training 
    5) Expimental Setup and results 
    6) Analysis & conclusions 
     
    <<>> 
     
     
     
     
    >>> INTRODUCTION 
    IMPT IDEAS: 
    -  Motivate: Taht Knowledge Assets are impt to K. based company 
       - Why: expertise location; question answering; Skill gap identification 
     
    - problem 
       - ok we want skills, what is the best data source?  email. 
         why?  Ubiquitous.  Rich. 
       - Difficulty: no easily coded rules & many skill categoryies -> must learn 
       - Difficulty: email is very noisy: most msgs not about specific topic.   
                     email is not labelled. 
       - Important skill areas are typically well documented: 
         discussion groups or manuals, training material etc. 
         but this data is not tied to give person, so we cannot infer perso skill 
       - Idea:  we can train on skill text, somehow transform trained 
         system so that it applies to personal text, and use that to 
         catalog people.  (we will see thta it cannot be directly applied.) 
       - Formal def of problem. 
         Given skill data (text associated with skill) and 
               personal data (text associated with person) produce 
               Organizational skills catgalog orgnized by person and by skill 
     
    - Solution 
       - We employ several convetional text mining methods to learn 
         a document classifier over the training corpus. 
       - The document level classifier is used as a component in  
         person level classifier that predicts employee skill from a  
         collection of documents (their electronic communications) 
       - This transformed classifier is used to produce a skill catalog 
         for the organization. 
     
       - We define this approach of training on one distribution 
         and modifying the result so that they apply to a second distribution 
         as cross training. 
     
       - Our approach to cross training generalizes beyond the email-based 
         skill mining task that motivated it.   
     
       - To explore where and how this approach might apply in other cases. 
         We generalize those properties of the email based skill mining domain 
         that underpin our solution 
     
       - We show how our solution can be applied to any domain where thees two  
         generalized properties hold. 
     
       - This perspecitve of cross training gives us a new way 
         to understand prior work that generalizes classification leraning 
         in other ways.  We consider what other cross training situations 
         are addressed by existing techniques. 
     
     
    - benefits 
      - an automated skill cataloging mechanism, that relies on pre-existing data 
        sources within an orgination (email and skill related documentation) 
     
      - Because cross training generalizes beyond this domain, our solution and 
        framework can be applied in other areas. 
     
      - Practitioners applying classification learning in the practical setting 
        are frequently presented with a cross training sitution.  The  
        typical appproach is to simply ignore the systematic differences  
        between training and testing environments, and hope is violation 
        does not fatally impact performance. 
     
      - Providing a framework for cross training where systematic 
        differences between training and testing are explicitly expressed, 
        characterized, and addressed is a first and necessary step 
        in a comprehensive approach to effectively addressing this pervasive 
        shortcomming in the convetional model of induction. 
     
    ???  - Characterizing existing methods that can be applied to other cross  
        training situations is also important, since it allows us to charactize 
        when these approaches should be used in practical situations. 
     
    - outline 
      (filled in as paper is fleshed out) 
     
     
    >>> ARCHITECTURE 
     
    questions to answer: 
     
    -what preprocess steps are done (perhaps a small figure  here is worth while) 
     - bag of words  
     - clipping word counts to existance <<>> non-existance within document. 
     - Use of Info Gain 
       (Put equation in paper) 
     
     - Explain clipping word counts because of normalization problems 
       with approaches like SVMs, Baysian etc. 
     - Reference paper that says we don't loose too much precision doing this. 
     
     - Expalin that because of document length differences between train  
       and test, worse because of systematic word usage differences  
       between train and test sets we do not want to conider complex 
       decision surfaces.  We do not expect them to transfer. 
       Instead we focus on identifying imporatant features in the training  
       set and apply those in the test environemnt. 
     
     - Over the training data alone we applied both a naive bayes classifier 
       (which does not attempt to find interacting features) and a  
       SVM classifier (which can find intreactive featres) 
       We found a relatively small difference in performance XXX %. 
       Increases our confidence that are simplification to considering only  
       hypotheses that do not allow interacting featres is justified 
     
     - We tried two methods for selecting features:  MutInf, InfoGain. 
       Over our range of usage we found almost no difference between these 
       methods.  We adoped InfoGain because ###ref## identified this method as  
       being superior in selecting very small subsets of the features. 
     
       (show info gain equation) 
     
     
     - discuss the flow of the whole system.   
       - discuss the vettig by the user as a guard for privacy. 
     
     
    >>> CROSS TRAINING 
     
     
    >>>DEF OF CROSS LEARNING: 
     
    #1 Training distribution is concentrated 
       Testing distribution is diluted with many entries from a very 
       non-cohesive 'other' category. 
     
    #2 Training and testing 'example' are naturally of a different 
       grain size. 
     
    #3 Pragmatics vary greatly across domain; thus we stick to  
       word analysis.  Len of message; amount of ctx info. 
     
    Both problems skew distributions (fp fn rates) 
    Second also requires bridging gaap 
     
     
     
    >>>EXPERIMENT SECTION 
    - goals and need for virtual email 
     
    > VIRTUAL E-MAIL DATABASES 
     
    > MESSAGE LEVEL CLASSIFIER 
     
    > EXPERIMENTAL SETUP 
    - our goal is to measure performane of cross training 
      so we consider perf over diff dilution rates, fixing the grainsize 
      at rates appropriate for eMail DBs 
     
    In building these ROC curves we varied the dilution rate from 90% to 99.9% 
    and measure the performce by averageing of twenty learning/testing runs 
    for each data point. 
     
    Of the twenty ther are ten positive and ten negative. 
     
    We adopted a grainsize G of 10,000 messages per virtual personal dataset, 
    this size is consistent with real mail databases we have obtained. 
     
    > EXPERIMENT RESULTS 
    - Fig XXXX show the results of this ROC analysis 
    - It compares the performance of cross training with the performance of  
      an uncorrected leran. 
    - The dilution rate shown logrithmically on the horizontal axis  
      Average perf is shown on the vertical 
     
    - The apriori most probable class is 50% and you can see that the  
      learned classifier degrades to this point with 100% dilution (no data). 
     
    - Each data point is accurate to within a percent using 95% confidence interval 
     (see the analysis section for discussion of this point) 
     
    - We see ... 
     
    > Experiment II 
       
     
     
    >>> ANALYSIS 
     
    > Comparison to uncorrected learning 
      - The performance diff between XT performance and uncorrected 
        learning is quite large. 
      - The reader can might conclude that the comparison is somehow not 
        being fair the the conventional learning algorithm.  This is  
        completely correct from a certain perspecive. 
      - The conventional learning algorithm is simply maximizing accuracy 
        at the message level, it is not "aware" of the huge penalty for 
        false postitives in our particular application, and so it doesn't  
        attempt to avoid them 
      - But this in fact is our motivation for cross training, 
        this large discrepancy simply points to the  
     
    > Compensation for Grain Size differences 
      - We see from the results shown in Fig ### that this approach 
        is capable of compensating for a very large grain size differnce 
        between training and testing sets.   
    - In that experiment 10,000 message were combined to form a single 
    virtual person's message set.  And the system was able to compensate for these 
    differences. 
    - One might imagine an ROC curve like the the dilution curve being generated 
    for Grain Size differences.  Unfortunately varying grainsize while holding 
    dilution constant is not a meaningful operation.  If we fix dilution and vary 
    grainsize, we are in effect linearly varying the total number of "positive"  
    skill-related messages total in the group of person-related messages.  Thus  
    in the person-related group 
     
    > Limits of the approach and our analysis 
      - pos set is representative; but negative set may not  
        include sufficient varience 
     
     
    > Future work and conclusions 
     
    > Efficacy of Skill Mining 
    - Because our analysis is based on virtural person-related database rather 
    than actual  
    - What we can say regarding the posiblity of skill mining is 
      (1) when we have a jargon laden skill term 
      (2)  
     
    - Feature context 
    Because we are constructing virtual person-related datbases we 
    were unable to incorporate message context terms like mass-mailing 
    into the leraning (since such context terms are not meaniful in USENET) 
    Nontheless, we have that specializing the raw word features by these context 
    terms can improve the features specificty greatly. 
    Considering features that are in (mass mail, vs. received mail vs. sentmail) 
    - An important next step is for us to obtain sufficient email data 
    that we can test without virtual person-related datasets.  Which would then 
    permit us to systematicatically explore learning over this context-extended 
    feature space.  It would also allow us to directly measure skill mining  
    performance on actaul peoples datasets. 
     
    > Cross training kicks butt 
     
