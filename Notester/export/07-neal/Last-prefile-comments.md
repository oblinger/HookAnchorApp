# 07-neal.Last-prefile-comments --

    <<>> 
    Neals TODO list (for dan) 
     
    >>> How I contributed to patent #2 
     
    The representation of context as a set of attributes and values 
    impacts patent #2.  Three icons for novice, expert, and guru might 
    be tied together as the three possible value for the 'expertise' context 
    variable.  Selecting one automatically unselects the others in the group. 
     
     
    >>> How I contributed to patent #6 
     
    * Allowing the user to modify the initial query on the fly and get 
      a continuously updating set of responses. 
    * Allowing the user to specify an ordering to the axes (see my comment below), 
      hiding/viewing axes as a control mechanism for affecting the search  
      results are all ideas that affect patent #6 
     
    (I can think harder about justifying these points if needed) 
     
     
    >>> I like the update overview flow diagram 
     
     
    >>> OPTIONAL QUESTION 
    Yes, from the point of the view of patents #3,4,5 the learner 
    view is just an embodiment, not central to the patent.  But if 
    we claim that the learner view is merely an embodiment of Patents #2 & #6 
    then what is our claim for those patents??  I guess we could try to 
    divorce the pieces of the learner view from the whole view and claim 
    that the whole view is a preferred implementation (combination) 
    of the components, but pat #2 & #6 *is* the UI. 
     
    Perhaps I am not understanding the suggestion. 
     
     
    >>> I answered the questions at the end of patents #3, #4, & #5 
        Of course most of the answers were simply to leave the button 
        in the "No" setting.  I don't understand your "rough order of magnitude" 
        request (nothing was numerical).  It probably is not important. 
     
     
    >>> QUESTION FOR DAN:  Without including a profile ... 
     
    It seems best to have a user profile for each user.  This allows the system 
    to learn context about the user from one session to the next.  Is this 
    a problem?   
     
    This profile would also allow a corp to track expenses, etc. 
     
     
    <<>> 
    <<>> 
    <<>> 
    <<>> 
    <<>> 
    <<>> 
    <<>> 
    #6  
     
    ** Dan question on system vs.user ... 
     
    If I understand the question, then the answer should be yes. 
    If you make a change to the initial query then it is as if you 
    had simply made that changed query in the first place. 
     
    (Almost.  Acutally the system would still have a history of your first 
    query and it might respond differently if it sees a pattern to your 
    sequence of queries, user getting stuck, user making many queries using 
    a set of synonyms, but missing the synonym used in the database, etc.) 
     
     
     
    <<>> 
     
    ****  COMMENTS FROM NEAL/MARK 
     
    NEAL: New Graphics Needed from Mark (plus please see the question I raised 
    in disclosure #6 about packaging of all the iconic functionalities 
    into disclosures - may override the list of new graphics below) 
    Disclosure #2 & #6 - Perhaps you want to see if there is a different 
    interface for the resource criteria ranking and weighting 
    functionality than using the bottom axis - perhaps just using the 
    checkboxes also for ranking - and modify any graphics you include in 
    #2 and #6 accordingly. 
     
    MARK: sorry, i didn't understand this before and i still might not. i 
    thought that this meant a system ranking of resources presented to the 
    user in addition to the rankings along all the other variables. in 
    such a case, another axis is the best way to present this ranking. 
      
    if we're asking that user rank and weight the variables, however, i 
    agree that another axis is inappropriate. i propose that the learner's 
    interactions with the ui be captured and used to implicitly rank and 
    weight variables, based on the amount of time spent navigating on each 
    axis. also -- if an axis is consistently hidden, that that should also 
    be pretty meaningful. in short, i think that asking the user to 
    explicitly rank and weight the variables is 1) too much to ask and 2) 
    too unreliable and that this, therefore, should be done implicitly. 
     
    i'd be happy to come up this week if you'd like. 
     
    >>> Dan's response 
     
    Yes I think I was thinking along these lines.  Perhaps we should also 
    allow the user to reorder the variables being displayed and use this 
    as an indicator of the relative importance the user is placing on that  
    var. in the lookup. 
     
