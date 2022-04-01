# abl.examples --


    DELETING STEPS

    MOVING STEPS

    REPLACING STEPS

    LEARN - EDIT - LEARN



    Reorder:

      change setting a
      if not set change setting b
      change setting b'



    ------------------------------------------------- DELETION EXAMPLE ----------------------------------------

    Author is building a 'config' script that works in several starting states.  The script is long and 
    the author forgets to set on of the parameters correctly, so they change the parameter values 
    during recording and then edit those setup steps out.  (I think this abilty to delete steps for this
    purpose really is a strong practical reason why editing is more usable that 'record-only' ...
    you don't have to be perfect.)

    This could be a continuation of the build-path setup example above.... It assumes that some parameters were already setup
    Demonstration:
      TopMenu -> Project -> Properties
      Click Build Path                  <--- this is just extra stuff that makes it so user cannot just restart recording; they must use delete later
      Click Add External Jar
      Click Open
      Select SWT.jar
      Click OK
      Click "java task tags"
       -- User realize the tag they want to show being added is already there --
      Select 'MyTag'                    <-- before this recording make sure that 'use project settings is set'
      Click 'remove'
       -- now user is ready to author the creation steps --
      Click 'new'
      Type 'MyTag'
      Click 'OK'
      Click 'OK'    

       -- After stopping this recording; the select and remove steps are deleted to get the final procedure
          this is much easier than starting the whole recording over.

    ----------------------------------------- LEARN - EDIT - LEARN   EXAMPLE  ---------------------------------------

    One of the important aspect of our system, it the abilty not just to edit, but to continue learning 
    without throwing out info from the previous recordings.

    We can continue the deletion example above with a second demonstration.  This second demonstration
    learns to make the adding of the external jar; an optional operation if the jar is already there:

    Demonstration:
      TopMenu -> Project -> Properties
      Click Build Path    
      Click "java task tags"
      Click 'new'
      Type 'MyTag'
      Click 'OK'
      Click 'OK'    

    This demonstration can only learn the conditional 'if jar file is missing then add it'
    if it has the state-action pairs from the first demonstration, before the DELETE 
    operation happened.   (Combining these two is also nice since the final procedure
    starts getting a little bigger, but it takes much less extra space in the paper to
    describe the follow on.



    ----------------------------------   RESTRUCTURING EXAMPLE  --------------------------------------


    Restructuring example:
      Demonstration 1:     A; D
      Demonstration 2:     A; B; D
      Demonstration 3:     A; B; C; D
    Resulting program:
      A
      if x then
         B
         if y then
            C
      D

    Desired program (obtained by editing)
      A
      if x then B
      if y then C
      D

    Example of this structure in eclipse:
    The scenerio is an author that is setting up a build script.  The first demonstration is on a machine
    with the appropriate JAR files.  Subsequent demonstrations show adding of jar files when appropriate.


    Action A  =   TopMenu -> project -> properties.
                  Select JavaBuildProperties

    Action B  =   If SWT.jar is not listed then   Click 'Add External Jar' ->  Click Open -> Select 'SWT.jar' -> Click ok


    Action C  =   If XML.jar is not listed then   Click 'Add External Jar' ->  Click Open -> Select 'XML.jar' -> Click ok

    Action D  =   Click "ok"     (this closes the properties window)





        
      Click Scope = "working set"
      -- 



    rom several  'install' script that works from sever


    The problem that Vittorio and I have been struggling with (and still have
    no good answers for) is *specifically* what sort of procedure and what sort
    of edits we should use to provide reasonable/believable scenarios.
    Ideally, we'd have a scenario that involves deleting some steps and a
    separate scenario that involves moving a set of steps, perhaps a third
    scenario that involves replacing a set of steps with a different set.  In
    all cases, after doing the edit, we want some additional recording that
    shows ABL works, and where we can make some arguments that with a more
    naive approach we'd get into trouble.   If you have concrete suggestions
    for plausible scenarios, that would certainly help.
