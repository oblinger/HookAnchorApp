# Hack.Notester.Attachment\_Goals --

    [GOALS]  <<>>
    - can move whole notester dir?
    - can delete attachment when note is deleted?


    [SCENERIOS]
    RECEIVED FILE(S) TO FILE

    (1) DESTINATION EXISTS
        * Project exists
        TAB-G name RET               Goto to location for attachment

    (2) CREATE PROJECT
        * Files are part of new logical group
        * Expect multiple versions of file
        TAB-Q-P name RET             Create project  folder  "<<>>"
        TAB-Q-S name RET             Create sub-proj folder  "...proj/<<>>"
        TAB-Q-I name RET             (Other types of projects just have different parantage, and different dir rarely)

    (3) FILE BY DATE
        * When file is one of potentially many and no distinctive name exists for file
        TAB-Q-D name RET             Create dated folder for attachment "<<>> <<>>"   (no project)
                                     Create dated folder for attachment "...proj/yy.mm.dd <<>>"       (under project)

        JUST ATTACH
        * When many files are not expected in logical group
        TAB-A                        Create dated file "<<>> <<>>"        (no project)
                                     Create file       "...proj/<<>>"                            (under project)

    (4) MULTIPLE FILINGS
        * When file should have multiple listings
        - Reseracher's work goes into sub topic area
        TAB-G name RET TAB-L  (link to marked note, the note just created)
        This doesn't exist, and still cannot search 





    - just got a paper to read; add notes and move somewhere; still can find it later (on some todo list)
      goto todo list (which is a proj dir).  TAB-A  or TAB-Q-L TAB-A

    - Paper from reseracher
      TAB-G AUTH RET  TAB-Q-S lastname RET  TAB-Q-S paper-topic RET  TAB-A
      TAB-G topic RET  TAB-


    - Creating a child within a namespace
