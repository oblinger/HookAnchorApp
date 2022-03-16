# Notester.File\_Management --



    FILE MANAGEMENT SCENERIOS


    SITUATION:  Team collaborates on document by email.  User wants to file away versions as they come in, and 
      (1) to only keep the latest file, 
      (2) to keep the sequence of files ordered by filing date.
    APPROACH:   
    - Use email hook to pass files and descriptive email note to Notester.
    - Case #1.  Navigate to Note where file should be attached, then TAB-A to attach file (this will overwrite older verion of file w same name)
    - Case #2.  Use TAB<<>><<>> to create new project note with it own directory.  Use TAB-M to move file to project's directory.
      BUG: Tab-M should use different filenames if there is a collision?


    SITUATION:  Files already exist in filesystem.  User wants to annotation or organize them using Notester.
    APPROACH:   
    - Create an _XXXXX_.nstr file in the relevant directory.  
    - When clicked this file will open Notester to a note with that name (or prompt user to create that note)
    - Created note will point to the originating directory, and file names in that directory can be added as links
      in the associated note.

    SITUATION
