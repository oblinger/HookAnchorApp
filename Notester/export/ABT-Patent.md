# ABT-Patent --

    "From: CN=Vittorio Castelli/OU=Watson/O=IBM"
    "Subject: Here we go"
    ""
    "
    Cheers,

    - Vittorio
    <<>>
    Vittorio Castelli
    Systems Theory & Analysis Project
    Server Technology Department

    IBM T.J. Watson Research Center
    P.O. Box 218
    Yorktown Heights, NY 10598
    Phone: (914) 945 2396
    FAX:      (914) 945 2141 (Attn.: Vittorio Castelli)
    <<>>
    <<>>
    "Aliquando et insanire iucundum est"
    (L. Annaeus Seneca)

    "



    1. Background: What is the problem solved by your invention?  Describe known solutions to this problem (if any).  What are the drawbacks of such known solutions, or why is an additional solution required?  Cite any relevant technical documents or references.
    The problem this patent addresses is the problem of learning a procedure incrementally from 
    sequence of demonstrations of that procedure.  A solution to the problem will accept an initial
    partial model of the underlying procedure, and a sequence of steps executed by the underlying procedure.
    The system will then extend the input partial procedure to incorporate steps observed.  Repeated application of
    this process can be used to extend the initial (possibly) empty procedure to incorporate all of the steps and logic
    in the underlying procedure. 

    2. Summary of Invention:  Briefly describe the core idea of your invention (saving the details for questions #3 below).  Describe the advantage(s) of using your invention instead of the known solutions described above.
    We teach a solution to this problem that relies on a class of procedure transformations call 'augmentations'.
    By restricting ourselves to only using valid augmentations we ensure that our search for an updated 
    procedure to match new data, does not invalidate that procedures consistency with previously seen
    demonstrations.  Thus the solution we teach maintains a current best procedure and that procedure
    is updated by searching the valid augmentations of that procedure.  This process is guaranteed
    to transform procedures into new procedures consistent with new demonstrations, while retaining 
    the transformed procedure's consistency with any previously seen demonstrations.

    3. Description:  Describe how your invention works, and how it could be implemented, using text, diagrams and flow charts as appropriate.
    This patent critically depends on the notion of certain transformations of a procedure being
    valid augmentations of that procedure.  We define augmentation here:
    A transformation that of procedure A into procedure B, is called an AUGMENTATION of that procedure if 
    the new procedure is consistent with all of the demonstration sequences that were consistent with the original 
    procedure.  So for example, updating an original procedure with two sequential steps "A then C" into 
    a procedure with three steps "A then B then C" would not be a valid augmentation, since the original 
    procedure was consistent with a sequence of steps 'a' followed directly by 'c', but the new
    procedure only allows 'a' to be followed by 'b'.  The procedure "A optionally B then C," on the other hand,
    is a valid augmentation, since it continues to be consistent with the sequence 'a' directly followed by 'c'.
