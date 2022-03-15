# example\_for\_lawyers --

    Below is an example that follows data through the user modelling part of the patent.
    I have attempted to use the same language I used in the original description of the user
    modelling section in this example.  This simple example attempts to give a feel for how
    these components might interact in order to control the GUI components presented to the user.




    Recall that the system models the user as a set of INDIRECT INDICATORS (like frustration level, attention load, and sleepiness) over
    time.  The induced HMM model of the user is then a parametric model of those parameters as a function of the
    past and present environment of the user.  This includes CONTROL PARAMETERS like the number and type of interfaces presented to
    the user as well as other OBSERVABLES like the temperature in the car, or the number of nearby vehicles, etc.

    This parametric user model can then be used to make predictions about the users state both now and in the future.
    For example, the user model could take the number of nearby cars, radio volume level, and whether a heads up display
    of a map is shown, and make predictions about how distracted this would make the user.
    These predictions can then be used by a CONTROL system, to decide what user interfaces to display to the user.


    As an example, let us consider one indirect indicator in the user model: "attention load" and let us consider
    three observables that relate to this indirect indicator:
      (1) Breaking lag time.  This is the time between detecting that the car in front has slowed down, and the
          time that we detect the driver of this car is breaking.
      (2) Response lag time.  This is the number of milliseconds between a question being presented verbally to the
          driver, and the time that they respond to that prompt.
      (3) User squelch.  When very overloaded a driver will "turn off" our system.  Thus this silencing operation
          itself is an indicator of a high attention load.


    LEARNING THE USER MODEL PART (1)
    Our system has an initial parametric model that related these three observables to a hidden INDIRECT INDICATOR called
    "attention load"  By looking at correlations between these three indicators we can update this parametric model
    to more accurately tie these three observables to the "attention load" level.  Concretely we are updating the 
    probability that the driver will silence the system at each different attention load level, and updating the 
    mapping between breaking lag time and attention load level, etc.

    LEARNING THE USER MODEL (2)
    Our system also has a parametric model of attention load level over time.  This user model accepts the current
    state of the environment, e.g. how many cars are near, temperature, and the status of the GUI and makes
    predictions about the future "attention load" level.  From the part of the user model learned in part (1) above
    we are able to use observed breaking time, response lag time, etc. to predict an estimate for attention load at
    each time point during the driver's activity.  Now in this part we learn a parametric model of that indirect indicator
    from data obtained in this and in previous time periods.  The learning algorithm selects parameter values in the model
    that minimizes the models prediction error against this estimated attention load.

    For simplicity sake let us assume that the parametric model is a simple sum of linear weights model.
    So attention load will be estimated as some linear function of the number of near cars, the car's temperature, and a different
    weight is added in for each possible GUI component displayed to the driver.  Thus by observing how much the drivers
    reactions are slowed we can get a quantitative measure of the impact of each possible user interface component.  Let us assume that
    historically reaction times are dramatically increased when we are providing auditory directions to the user.  This will be
    captured by the learning algorithm as a large coefficient on the attention load impact of a speaking directions interface.

    The control system can then use this parametric model to predict how much of an impact providing such auditory directions will
    have in the current environment.  If that impact will push the users attention load unacceptably high, then the control process
    will opt not to provide auditory directions to the user, until conditions on the road are lighter and thus providing those
    direction will not push the users attention beyond specified operating objectives.

    Of course in this example we assumed a simple linear model for predicting the user's indirect indicators, but the same basic 
    procedure will continue to work on more complex models where non linear effects are seen.  Models can combinatorial terms
    that allow for interactions between features, as well as non linear effects.  Perhaps in some models the users attention is not
    impacted at all by lower levels of distraction, and then is sharply impacted beyond some threshold.  Such non-linearities can be
    handled by more complex models (e.g.  polynomial curve fitting)
