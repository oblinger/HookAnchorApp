# Click\_Tester\_Patent --

    DISCLOSURE NO. YOR820040661
    Your disclosure entitled:   Remote GUI control by replication of local interactions 
    has been received by our office and assigned DISCLOSURE NO. YOR820040661.

    The patent attorney responsible for handling the disclosure has been assigned 
    to Casey August<<>>Watson<<>>IBM.

    The Invention Development Team (IDT) member(s) assigned to this disclosure are 
    Lorraine M Herger<<>>Watson<<>>IBM;Casey August<<>>Watson<<>>IBM.
    -----------------------





    1. Background: What is the problem solved by your invention?  Describe known solutions to this problem (if any).  What are the drawbacks of such known solutions, or why is an additional solution required?  Cite any relevant technical documents or references.
    In this patent we address the problem of controlling multiple Graphical User Interfaces (GUIs) on remote
    machines over a network.

    Current approaches to this problem include:

    * REMOTELY OPERATING A SINGLE MACHINE.
      * SCREEN PROJECTION.  Applications like VNC or PC ANYWHERE project a computer's entire display to
        a remote computer where it can be directly manipulated.
      * OS WITH REMOTE DISPLAY CAPABILITY.  Interacting with a remote GUI using an OS facility to decouple 
        an application's host computer from the computer used to display its GUI.  X-Windows is an example of 
        such a system.
    * SCRIPING.  Authoring a script or wizard which can be distributed to remote machines in order
      to drive the remote GUIs as appropriate.  

    Limitations
    - Scripts are expensive to develop and test, and they are not suitable for non-programmers.
    - The remaining approaches have the following limits 
      (1) interacting with only a single machine at a time,
      (2) interacting at the same time that the remote machine is executing (cannot record and play back later), and
      (3) require relatively high bandwidth to effectively control the remote GUI


    2. Summary of Invention:  Briefly describe the core idea of your invention (saving the details for questions #3 below).  Describe the advantage(s) of using your invention instead of the known solutions described above.
    In this invention we describe a method for controlling remote GUIs by replicating interactions that
    occurred on a local GUI.  

    In this approach we assume that there is a 'master' application running locally, and a number of 
    'slave' applications running on remote machines, and possibly running at different points in time.

    The user interacts with the master application thru the local GUI.  This interaction is abstracted
    and recorded.  The sequence of recorded interactions may be collected and then distributed for analogous
    execution of each of the remote GUIs.  In another case individual interactions (or groups of interactions)
    may be distributed immediately for execution on the remote GUIs.

    In any of these cases a distinguishing feature of the interaction captured is that it is a
    "a small disambiguating description" of the GUI elements and GUI actions taken by the user.

    The advantages of this approach:
    (1) there is no programming required to control the remote GUI
    (2) multiple machines may be simultaneously controlled by the interactions performed against
        a single GUI.
    (3) Playback on a GUI may be delayed arbitrarily after the master GUI was controlled.
    (4) Requires less bandwidth since 'smallest' descriptions are employed.



    3. Description:  Describe how your invention works, and how it could be implemented, using text, diagrams and flow charts as appropriate.

    "A Small Disambiguating Description" is a description of the GUI actions taken by the user along with a description of those
    elements involved in the action.  Both the action and the relevant element must be distinguished from all possible other
    actions and other elements if one is to be able to later execute this interaction remotely.  


    On the other hand, a full visual representation of the GUI and the user's interaction with that GUI provides much more than
    a 'small' set of distinguishing features.  From an information theoretic perspective traditional interaction description languages
    produce a very large and redundant description of the user's interaction.

    We teach a method for constructing "small disambiguating descriptions" consisting of:
    (1) A recording process that captures actions taken by the user on the local GUI, as
        well as updates made by the application to the GUI.
    (2) Interactions are abstracted to a small disambiguating description
    (3) Captured interactions are transmitted to remote machines
    (4) Transmitted interactions are executed on remote GUIs.

    The full application will contain preferred embodiments for:
    - The capture process
    - Methods for constructing small disambiguating descriptions, and
    - Several Modes of interaction with remote machines





    ---------------------




    a 




          all informtation inherent in a GUIs 





    On the other hand, pro



    Here smallest means fewest details about the element required to 


    - remote control by 
       creating a "small disambiguating description" of GUI elements and GUI actions
       transmit and play back


    - disambiguating description means:
      - discription sufficient for disambiguating cooresponding GUI elements and actions

    - small means fewest details require to disambiguate

    - Justification <<>> Benefits
      - bandwidth efficiency 
      - robust coorespondence resolution
      - heterogeneous execution environments
      - running multiple machines from single control stream
        (feedback from remote machine not required; feedback provided on local machine)

    Usages
    - control of interactive GUI thru one way info pipe
    - 1 to n 
      efficient maitance of machine farm
    - temporally delayed playback
      - instruction; assistance; 



    VNC/PC-ANYWHERE/Xwindows
    scripts



    SUMMARY





    DESCRIPTION
