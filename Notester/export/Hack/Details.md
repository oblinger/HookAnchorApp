# Hack.Details --

    TIMESHEET:  Prints out current activity; start time, and elapse time 
    - Input Formats: 
      [ prevTotal | prevEnd ] newCategory [ negTime | startTime ] 
      - First arg specifies time previous period ended 
      - Second arg specifies new category begin started 
      - Third arg specifies start time of new category 
        If negative then it starts in past and ends now, and activity reverts back 
        If unsigned then it starts at specified time and continues 
     
