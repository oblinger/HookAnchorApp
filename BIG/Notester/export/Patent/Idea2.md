# Patent.Idea2 -- Patent ideas

    1A) Status based attention mechanism 
        - AvailablityState: 
            prob distribution over discrete set of states (including unknown state) 
     
        - Rules:  Event,AvailiablityState,Confidence --> AttentionAction 
     
        - Event: 
          Control from user 
          External request for attention 
     
        - AttentionActions: 
     
    1B) Lazy availablity status determination 
        - Prompting user status only when confidence is too low 
        - AvailiablityState decay function 
          Decay(availability-state,time) --> availablty-state 
     
    1C) 
        - Adaptively learns transition probabilities 
     
    s 
    <<>> 
    PATENT:   Status-Based Attention Managment System 
     
    GOAL:  Automatically control manage user attention requests (email, phone,  
    task completion messages, voicemail, etc.) based on users current 
    attention state.   
     
    The attention AvailiablityState is expressed as a pdf over a discrete set of  
    AvailiablityStates (e.g. in meeting, watching TV).   
     
    An AttentionEvent is: 
    - an externally generated request for the users attention 
    - a users generated input (e.g. specifying a change in AvailiablityState) 
     
    Management rules specify appropriate action given an event and the 
    users current attention AvailiablityState, they also specify how that attension AvailiablityState 
    is updated either thru user action, or external sensing (e.g.  
    measuring the users location, or sound level in the vicinity of the user) 
     
    AttentionActions: 
    - Notify User of an AttentionEvent 
    - Queue event for later notification 
    - Set AttentionAvailiablityState/confidence 
    - Sense attentionAvailiablityState (include prompting user) 
     
    The system operates by applying the management rules to the  
    current AvailiablityState and incomming attention-events, as rules 
    fire the set of active attention-events is updated, and specified 
    actions are performed. 
     
     
    <<>> 
    PATENT:  Lazy Attention AvailiablityState Determination 
     
    GOAL:  Attention management minimizes user interruptions at inappropriate 
    times.  Continuously maintaining an accurate AvailiablityState reqires 
    effort that partially offsets the benefits of attentsion management. 
     
    Lazy evalution of the user's AvailiablityState minimizes the impact of 
    maintaining this AvailiablityState information, by deriving it as needed 
    and prompting the user only when insufficient confidence can be 
    obtained thru automatic derivation. 
     
     
    NOTE:  The use of the work 'lazy' refers to lazy vs. eager function evaluation. 
           The first patent assumes that the AvailiablityState is explicitly 
           maintained at all times.  We relax this assumption, and allow the 
           attention function vary with time, and to become undefined over time. 
           The system derives a high confidence specifiction of the user's 
           attention AvailiablityState only when required by incomming attention-events. 
     
    The center of the approach is an AvailiablityState decay function: 
       Decay(AvailiabilityState,time) --> AvailabltyState 
     
    The AvailabilityState 'UNKNOWN' is a special state the represents 
    the user being in any one of the other states, without the system 
    knowning that the user is in that state.   
     
     
    EXAMPLE USES OF THE SYSTEM 
     
    If STATE[InMeeting=?x] Then ACTION[SetState CannotSpeak=?x] 
    If STATE[CannotSpeak<.3] && EVENT[PhoneInterruption] Then ACTION[NotifyUser] 
    If InMeetingState>.3 && EmailInterruption  
     
     
     
    <<>> 
    PATENT:  Adaptive Attention-State Specification 
     
    GOAL:  Lazy attention management requries user to specify  
    how confidence in attention-state decays over time.  This can be 
    empirically determined by watching how the user explicitly changes 
    attention-states over time. 
     
    (This means that the system learns that the 'in-meeting' state is 
    almost always still true after 5 minutes so don't ask user again, but 
    is often not true after 5 hours so re-prompting the user for the current 
    attention state is required by most all attention events.) 
     
     
    This patent determines how the probaility of begin in a state falls off 
    as a function of time.  Different AttentionRules require differing 
    levels of confidence, so the entire profile as a function of time 
    is reuqired.  Having the user specify this is probematic. 
    Determining it from experience is thus desirable. 
     
     
