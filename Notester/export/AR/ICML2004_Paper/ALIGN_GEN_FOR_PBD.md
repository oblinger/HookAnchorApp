# AR.ICML2004\_Paper.ALIGN\_GEN\_FOR\_PBD --

    Our extension to traditional HMM learning is motivated by sources of knowledge available in the programming 
    by demonstration context 

    THE ARCHICTURE
    The architecture is a stack instru, abstract, learn, execution

    THE INTERACTION


    =======


    Before formally describing the core leraning algorithm underlying the AlignGen approach, let us briefly lay out the 
    main components, and processes in this approach to PBD.  We have implemented and instance of this AlignGen approach called
    <<>> in order to 
    provide concrete examples for the components of the <<>> approach.

    Figure ### provides a block diagram of the {\it AlignGen


    To make this concrete we will at times refer to our 
    SheepDog system which is our implementation of the approach on the Microsoft Windows platform.












    THE INPUT
    - The training data, T, for AlignGen is a set of demonstrated executions of some underlying procedure to be induced.
    - Each demonstration is a sequence of state action pairs.
    - The action is a symbol that represents the action taken by the author.
    - The state is what the author see on the screen prior to executing the action, i.e. it is 
      the state of the GUI.

    - Formally the training data is ...


    - Each step in turn is a state action pair where the state is a vector of features
      which reprents the state of the GUI
    - A step in a demonstated procedure 
    - A demonstration of a procedure is a sequence of steps 
