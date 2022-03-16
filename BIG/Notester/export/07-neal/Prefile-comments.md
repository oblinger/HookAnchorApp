# 07-neal.Prefile-comments --

    Neal, I have gone through all of the patents, particularly patents #3 #4 & #5. 
    Here is a set of comments<<>>additions.  Please make best use of these 
    comments as you see fit.  Apply as many (or few) as is convenient. 
     
    Cheers, 
    Dan 
     
     
    <<>> 
    THE SYSTEM VIEW 
     
    I understood and agree with the general organization of the system 
    view diagram.  I was not clear on the semantics across the entire  
    diagram.  We should be clear on whether this is a control flow diagram  
    or a data flow diagram.  If it is important to do so, we can make 
    it both a control and data flow, with different arrow notation for  
    both, but this will be a challenging figure to understand. 
     
    Also the overlapping boxes at the top, containing "2 or 3-D". 
    I don't understand the meaning of this.  Is this a detail that should 
    be explained in the text or is it appropriate for the figure? 
     
    <<>> 
    THE LEARNER VIEW 
     
    The summary was quite helpful.  Most all of it was what I had expected 
    except the way in which user contexts are selected. 
     
    In Step 1 part 2, it says that the "user uses iconic pulldowns to specify..." 
    In Patent #3 we have a diagram showing the context being defined as 
    membership in a clustering (which can be computed from current system 
    context (and prior context on that user)). 
     
    I am happy if we adopt either approach (or both approaches). 
    The advantage of the fully automated approach is that the  
    user is not burdened<<>>confused with context selection, also the system 
    administrator does not have to look at each cluster and devise an 
    iconic representation for it. 
     
    On the other hand a user that really understands the meanings of the contexts 
    can undoubtedly do a better job of assigning a current context. 
     
    Again I have no strong feeling about how this discrepancy should 
    be resolved, we just want to paint a consistent picture. 
     
    <<>> 
    SUPERVISED VS. UNSUPERVISED 
     
    The term supervised refers to whether the learning algorithm has a 
    supervisor or teacher that is telling it the "right" answers for the 
    training data.  An unsupervised algorithm is also given training data, 
    but there are no labelings on the training data from a supervisor specifying 
    the "right" answers. 
     
    So inferring user contexts is an unsupervised task, since (I was assuming) 
    the system is never told the right context for any given user<<>>query, but 
    the system was going to cluster all users<<>>queries into groups and those 
    groups were to be the contexts.  That process is unsupervised. 
     
    The process of selecting specific responses for a users query on the  
    other hand, can be supervised adaptation, since we can use feedback from 
    how the user reacts to each response as a label telling us if that  
    response was "right" or not.  In this case we do have a supervisor (the 
    human user<<>>learner.) 
     
    make sense? 
     
    <<>> 
    PATENT #3 
     
    (see my note on the Learner View for one integration issue) 
     
    - In the system diagram we have one box "Make Inference, Improve systems" 
      this implies that there is a single process at work doing these improvements. 
      But in fact we are getting three different patents for three different 
      means of improvement.  Perhaps it would be better to simply remove that 
      box, and just have the wide arrows that come from that box come directly 
      from the user feedback database, and the user feedback database comes 
      from the "capture user <<>> system feedback."   
     
      Once we make these changes then my diagram fits nicely into your  
      black subprocess box in the system view. 
     
    - in the #3 second paragraph you wrote "[others] use demographic clustering 
      in which a give learner can be a member of only one cluster" 
     
      I think that demographic clustering is clustering based on a narrow 
    set of attributes about the learner (user), but does not take into account 
      the history of interaction with the user, nor there current device, etc. 
     
      It is not the one vs. many cluster distinction. 
     
    - Differences between our work and others: 
      - Our work assumes a the context of a tutoring system where the  
        objective is to provide an ongoing training session.  Most other 
        system (including the Goetz patent), assume the shallow context 
        of a single user query stream focused on a single topic. 
        We build a profile of the user and there competence within a particular 
        subject area, and use this model to inform all of the training provided 
        on that subject.  (We probably need to make a bigger deal out of  
        an on going user model in our diagrams to make this difference stick. 
        Maybe not for the disclosures, but later.) 
      - Note that some tutoring systems also have an explicit user model, but  
        these systems do not have an adaptive question-answering component. 
     
    <<>> 
    #4 
     
    (See definition of supervised, above) 
     
    - Changes to the system view:  The User Feedback data need to go to the 
      this subprocess in the system view, the adaptive indexing needs access 
      to the resource library, and the resource lookup itself still needs to 
      have access to the query and context.  If this is too many arrows in  
      the system view, then adaptive indexing and resource lookup could be 
      merged, since both are contained in my diagram. 
     
    - In text #4.2 at end of paragraph "80% ... 20%" 
      I understand the sentiment here, but will the examiners expect that 
      we have data to backup those number?? 
     
    4.3 
    >>> "DAN, ANY BRIEF COMMENTS ... 
      None of the prior are I sent (or know about) dynamically builds inclusion 
      exclusion lists.  this will narrow our patent, but it also strengthens 
      it novelty (which we need so it is good). 
     
     
     
    <<>> 
    #5  
     
    5.1 
    >>>Define Supervised Learning --  see earlier comment 
     
    >>>"Is this true?" -- 
    - I don't know of a system that behaves like ours, but these are in common 
      usage, so I don't feel strongly that any of these components are 
      unique in themselves, rather it is the ensemble that is unique, further 
      we should use the fact that this is all happening a part of tutoring  
      system to separated us. 
     
    5.2 
    >>>"DO WE NEED REV 
    - the system diagram should propagate the context vector to this 
    sub-component. 
    - The feedback data in the systems view should flow to the set ordering  
      process as well as to the other processes.  We may adapt orderings based 
      on type of user, previous history, etc. 
    ... 
     
    5.3 
    >>> ONE SENTENCE FOR PRIOR ART 
    (See comment in patent #3) 
     
    <<>> 
    #6  
     
    >>> TRADEOFF QUESTION 
    Yes I do think the user should be able to reformulate the query after 
    the first set of responses comes back.  These subsequent queries will 
    simply be new queries with different historical context. 
     
     
