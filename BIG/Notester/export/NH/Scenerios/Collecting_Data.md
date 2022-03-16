# nh.Scenerios.Collecting\_Data --


    BACKGROUND
    - These examples are about capturing information
    - In these examples we assume a project is created (with directory)



    <<>>
    WITHIN A MEETING:  TOOK SOME NOTES; SEVERAL KINDS OF INFO DERIVED FROM THE NOTES
     - Todo items are listed.
     - Items that should be stored somewhere in hierarchy are listed

    <<>>
    WITHIN OUTLOOK:  SAVE MESSAGE AND ATTACHMENTS
    - Within outlook user select 'save to notester'
      -> Notester pops up and has message saying attachments are available
    - User navigates to project or page where attachments are to be stored.
    - User types 'TAB-A' to save attachments
      -> System prompt for the name of the directory to store attachments (default name of file)
    - User enters name and hits enter
      -> System moves files into the specified subdirectory, or in the main dir 
    <<>>
    WITHIN OUTLOOK:  SINGLE FILE 'FOO.xxx' WITH EMAIL BODY AS DESCRIPTION
    Output:  
    - New sub-directory 'foo'
    - .nstr file linking back
    - Attachments as files in 'foo'


    WITHIN OUTLOOK.  UPDATE FILE 'FOO.xxx'
    Output:
    -  Moves current directory into 'originals' subdir
    -  Create 'YYYY-MM-DD' sub-directory






    OUTLOOK SCRIPT
    - Key stroke to activate script
    - Source code editable ALT-F11
    - Packaged component that can be easily (automatically)
      added to an Outlook environment.

    In hardcoded directory:
    * Current email body is written to 'note.txt'
    * Entire message (without attachments) is written to 'note.msg'
    * Each attachemnt is detached and saved in directory.
    * Named windows appliction is activated.


    -----
    INITIAL TASK SPECIFICATION

    The goal is a single keystroke action which exports the contents of the
    current email message to a prespecified directory, and then to notify
    an external application that the data is available.

    * The script is in a 'module' that can be easily added to any MS outlook 2000 or above.
    * The script should be editable (using ALT F11) from within outlook.

    * The output of the script should place all output files into '<<>>'

    * The body of the email message should be written as ASCII text in 'note.txt' in the output directory '<<>>'
      (ideally it would have would include the FROM: TO: and SUBJECT: fields at the beginning of file, but this is not necessary) 

    * Each attachment for the message should be saved into the output directory '<<>>'

    * The whole email message (without attachments) should be saved into the file 'note.msg'

    * Once all files have been written an executable 'notester-attach.exe' should be executed.

    ------
    I-Exporer





    * All strings listed above '<<>>', 'note.txt', 'note.msg',  should be hard
      coded constants at the beginning of the script (I will want to edit them later)
